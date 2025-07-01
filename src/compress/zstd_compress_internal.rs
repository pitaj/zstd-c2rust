use crate::common::bits::*;
use crate::common::mem::*;
use crate::common::error::*;
use crate::zstd_h::*;
use crate::common::zstd_internal_h::*;
use crate::common::huf_h::*;
use crate::common::fse_h::*;
use crate::common::bitstream_h::*;
use crate::compress::zstd_preSplit::*;
use crate::compress::zstd_cwksp_h::*;

use core::mem::size_of;

/*-*************************************
*  Constants
***************************************/
pub const kSearchStrength: u32 = 8;
pub const HASH_READ_SIZE: u32 = 8;
pub const ZSTD_DUBT_UNSORTED_MARK: u32 = 1; /* For btlazy2 strategy, index ZSTD_DUBT_UNSORTED_MARK==1 means "unsorted".
                                       It could be confused for a real successor at index "1", if sorted as larger than its predecessor.
                                       It's not a big deal though : candidate will just be sorted again.
                                       Additionally, candidate position 1 will be lost.
                                       But candidate 1 cannot hide a large tree of candidates, so it's a minimal loss.
                                       The benefit is that ZSTD_DUBT_UNSORTED_MARK cannot be mishandled after table reuse with a different strategy.
                                       This constant is required by ZSTD_compressBlock_btlazy2() and ZSTD_reduceTable_internal() */


/*-*************************************
*  Context memory management
***************************************/
pub type ZSTD_compressionStage_e = std::ffi::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;

pub type ZSTD_cStreamStage = std::ffi::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;

pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const std::ffi::c_void,
    pub dictSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut std::ffi::c_void,
    pub dict: *const std::ffi::c_void,
    pub dictSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}

/***********************************************
*  Sequences *
***********************************************/

pub type SeqDef = SeqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqDef_s {
    pub offBase: u32, /* offBase == Offset + ZSTD_REP_NUM, or repcode 1,2,3 */
    pub litLength: u16,
    pub mlBase: u16, /* mlBase == matchLength - MINMATCH */
}

/* Controls whether seqStore has a single "long" litLength or matchLength. See SeqStore_t. */
pub type ZSTD_longLengthType_e = std::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2; /* represents a long match */
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1; /* represents a long literal */
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0; /* no longLengthType */


#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqStore_t {
    pub sequencesStart: *mut SeqDef,
    pub sequences: *mut SeqDef, /* ptr to end of sequences */
    pub litStart: *mut u8,
    pub lit: *mut u8, /* ptr to end of literals */
    pub llCode: *mut u8,
    pub mlCode: *mut u8,
    pub ofCode: *mut u8,
    pub maxNbSeq: usize,
    pub maxNbLit: usize,

    /* longLengthPos and longLengthType to allow us to represent either a single litLength or matchLength
     * in the seqStore that has a value larger than U16 (if it exists). To do so, we increment
     * the existing value of the litLength or matchLength by 0x10000.
     */
    pub longLengthType: ZSTD_longLengthType_e,
    /* Index of the sequence to apply long length modification to */
    pub longLengthPos: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_SequenceLength {
    pub litLength: u32,
    pub matchLength: u32,
}

/**
 * Returns the ZSTD_SequenceLength for the given sequences. It handles the decoding of long sequences
 * indicated by longLengthPos and longLengthType, and adds MINMATCH back to matchLength.
 */
#[inline]
pub unsafe fn ZSTD_getSequenceLength(
    mut seqStore: *const SeqStore_t,
    mut seq: *const SeqDef,
) -> ZSTD_SequenceLength {
    let mut seqLen = ZSTD_SequenceLength {
        litLength: 0,
        matchLength: 0,
    };
    seqLen.litLength = (*seq).litLength as u32;
    seqLen.matchLength = (*seq).mlBase as u32 + MINMATCH;
    if (*seqStore).longLengthPos
        == seq.offset_from((*seqStore).sequencesStart) as u32
    {
        if (*seqStore).longLengthType
            == ZSTD_llt_literalLength
        {
            seqLen
                .litLength = (seqLen.litLength)
                .wrapping_add(0x10000);
        }
        if (*seqStore).longLengthType
            == ZSTD_llt_matchLength
        {
            seqLen
                .matchLength = (seqLen.matchLength)
                .wrapping_add(0x10000);
        }
    }
    return seqLen;
}

/* compress & dictBuilder */
pub unsafe fn ZSTD_getSeqStore(
    mut ctx: *const ZSTD_CCtx,
) -> *const SeqStore_t {
    return &(*ctx).seqStore;
}
/* compress, dictBuilder, decodeCorpus (shouldn't get its definition from here) */
pub unsafe fn ZSTD_seqToCodes(
    mut seqStorePtr: *const SeqStore_t,
) -> std::ffi::c_int {
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable = (*seqStorePtr).llCode;
    let ofCodeTable = (*seqStorePtr).ofCode;
    let mlCodeTable = (*seqStorePtr).mlCode;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as std::ffi::c_long as u32;
    let mut u: u32 = 0;
    let mut longOffsets: std::ffi::c_int = 0;
    u = 0;
    while u < nbSeq {
        let llv = (*sequences.offset(u as isize)).litLength as u32;
        let ofCode = ZSTD_highbit32((*sequences.offset(u as isize)).offBase);
        let mlv = (*sequences.offset(u as isize)).mlBase as u32;
        *llCodeTable.offset(u as isize) = ZSTD_LLcode(llv) as u8;
        *ofCodeTable.offset(u as isize) = ofCode as u8;
        *mlCodeTable.offset(u as isize) = ZSTD_MLcode(mlv) as u8;
        if MEM_32bits
            && ofCode
                >= (if MEM_32bits {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as u32
        {
            longOffsets = 1;
        }
        u = u.wrapping_add(1);
        u;
    }
    if (*seqStorePtr).longLengthType as std::ffi::c_uint
        == ZSTD_llt_literalLength as std::ffi::c_int as std::ffi::c_uint
    {
        *llCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxLL as u8;
    }
    if (*seqStorePtr).longLengthType as std::ffi::c_uint
        == ZSTD_llt_matchLength as std::ffi::c_int as std::ffi::c_uint
    {
        *mlCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxML as u8;
    }
    return longOffsets;
}


/***********************************************
*  Entropy buffer statistics structs and funcs *
***********************************************/
/** ZSTD_hufCTablesMetadata_t :
 *  Stores Literals Block Type for a super-block in hType, and
 *  huffman tree description in hufDesBuffer.
 *  hufDesSize refers to the size of huffman tree description in bytes.
 *  This metadata is populated in ZSTD_buildBlockEntropyStats_literals() */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTablesMetadata_t {
    pub hType: SymbolEncodingType_e,
    pub hufDesBuffer: [u8; 128],
    pub hufDesSize: usize,
}

/** ZSTD_fseCTablesMetadata_t :
 *  Stores symbol compression modes for a super-block in {ll, ol, ml}Type, and
 *  fse tables in fseTablesBuffer.
 *  fseTablesSize refers to the size of fse tables in bytes.
 *  This metadata is populated in ZSTD_buildBlockEntropyStats_sequences() */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTablesMetadata_t {
    pub llType: SymbolEncodingType_e,
    pub ofType: SymbolEncodingType_e,
    pub mlType: SymbolEncodingType_e,
    pub fseTablesBuffer: [u8; 133],
    pub fseTablesSize: usize,
    pub lastCountSize: usize, /* This is to account for bug in 1.3.4. More detail in ZSTD_entropyCompressSeqStore_internal() */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTablesMetadata_t {
    pub hufMetadata: ZSTD_hufCTablesMetadata_t,
    pub fseMetadata: ZSTD_fseCTablesMetadata_t,
}

/** ZSTD_buildBlockEntropyStats() :
 *  Builds entropy for the block.
 *  @return : 0 on success or error code */
pub use crate::compress::zstd_compress::ZSTD_buildBlockEntropyStats_literals;

/*********************************
*  Compression internals structs *
*********************************/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: u32, /* Offset sumtype code for the match, using ZSTD_storeSeq() format */
    pub len: u32, /* Raw length of match */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: u32, /* Offset of sequence */
    pub litLength: u32, /* Length of literals prior to match */
    pub matchLength: u32, /* Raw length of match */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawSeqStore_t {
    pub seq: *mut rawSeq, /* The start of the sequences */
    pub pos: usize, /* The index in seq where reading stopped. pos <= size. */
    pub posInSequence: usize, /* The position within the sequence at seq[pos] where reading
                        stopped. posInSequence <= seq[pos].litLength + seq[pos].matchLength */
    pub size: usize, /* The number of sequences. <= capacity. */
    pub capacity: usize, /* The capacity starting from `seq` pointer */
}

const kNullRawSeqStore: RawSeqStore_t = RawSeqStore_t {
    seq: std::ptr::null_mut(),
    pos: 0,
    posInSequence: 0,
    size: 0,
    capacity: 0,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: std::ffi::c_int, /* price from beginning of segment to this position */
    pub off: u32, /* offset of previous match */
    pub mlen: u32, /* length of previous match */
    pub litlen: u32, /* nb of literals since previous match */
    pub rep: [u32; 3], /* offset history after previous match */
}

pub type ZSTD_OptPrice_e = std::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;

pub const ZSTD_OPT_SIZE: u32 = ZSTD_OPT_NUM + 3;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct optState_t {
    /* All tables are allocated inside cctx->workspace by ZSTD_resetCCtx_internal() */
    pub litFreq: *mut std::ffi::c_uint, /* table of literals statistics, of size 256 */
    pub litLengthFreq: *mut std::ffi::c_uint, /* table of litLength statistics, of size (MaxLL+1) */
    pub matchLengthFreq: *mut std::ffi::c_uint, /* table of matchLength statistics, of size (MaxML+1) */
    pub offCodeFreq: *mut std::ffi::c_uint, /* table of offCode statistics, of size (MaxOff+1) */
    pub matchTable: *mut ZSTD_match_t, /* list of found matches, of size ZSTD_OPT_SIZE */
    pub priceTable: *mut ZSTD_optimal_t, /* All positions tracked by optimal parser, of size ZSTD_OPT_SIZE */
    
    pub litSum: u32, /* nb of literals */
    pub litLengthSum: u32, /* nb of litLength codes */
    pub matchLengthSum: u32, /* nb of matchLength codes */
    pub offCodeSum: u32, /* nb of offset codes */
    pub litSumBasePrice: u32, /* to compare to log2(litfreq) */
    pub litLengthSumBasePrice: u32, /* to compare to log2(llfreq)  */
    pub matchLengthSumBasePrice: u32, /* to compare to log2(mlfreq)  */
    pub offCodeSumBasePrice: u32, /* to compare to log2(offreq)  */
    pub priceType: ZSTD_OptPrice_e, /* prices can be determined dynamically, or follow a pre-defined cost structure */
    pub symbolCosts: *const ZSTD_entropyCTables_t, /* pre-calculated dictionary statistics */
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const u8, /* next block here to continue on current prefix */
    pub base: *const u8, /* All regular indexes relative to this position */
    pub dictBase: *const u8, /* extDict indexes relative to this position */
    pub dictLimit: u32, /* below that point, need extDict */
    pub lowLimit: u32, /* below that point, no more valid data */
    pub nbOverflowCorrections: u32, /* Number of times overflow correction has run since
                            * ZSTD_window_init(). Useful for debugging coredumps
                            * and for ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY. */
}

pub const ZSTD_WINDOW_START_INDEX: u32 = 2;
/* Size of prefetching hash cache for row-based matchfinder */
pub const ZSTD_ROW_HASH_CACHE_SIZE: usize = 8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_MatchState_t {
    pub window: ZSTD_window_t, /* State for window round buffer management */
    pub loadedDictEnd: u32, /* index of end of dictionary, within context's referential.
                            * When loadedDictEnd != 0, a dictionary is in use, and still valid.
                            * This relies on a mechanism to set loadedDictEnd=0 when dictionary is no longer within distance.
                            * Such mechanism is provided within ZSTD_window_enforceMaxDist() and ZSTD_checkDictValidity().
                            * When dict referential is copied into active context (i.e. not attached),
                            * loadedDictEnd == dictSize, since referential starts from zero.
                            */
    pub nextToUpdate: u32, /* index from which to continue table update */
    pub hashLog3: u32, /* dispatch table for matches of len==3 : larger == faster, more memory */
    
    pub rowHashLog: u32, /* For row-based matchfinder: Hashlog based on nb of rows in the hashTable.*/
    pub tagTable: *mut u8, /* For row-based matchFinder: A row-based table containing the hashes and head index. */
    pub hashCache: [u32; ZSTD_ROW_HASH_CACHE_SIZE], /* For row-based matchFinder: a cache of hashes to improve speed */
    pub hashSalt: u64, /* For row-based matchFinder: salts the hash for reuse of tag table */
    pub hashSaltEntropy: u32, /* For row-based matchFinder: collects entropy for salt generation */
    
    pub hashTable: *mut u32,
    pub hashTable3: *mut u32,
    pub chainTable: *mut u32,
    
    pub forceNonContiguous: std::ffi::c_int, /* Non-zero if we should force non-contiguous load for the next window update. */
    pub dedicatedDictSearch: std::ffi::c_int, /* Indicates whether this matchState is using the
                            * dedicated dictionary search structure.
                            */
    pub opt: optState_t, /* optimal parser state */
    pub dictMatchState: *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const RawSeqStore_t,

    /* Controls prefetching in some dictMatchState matchfinders.
    * This behavior is controlled from the cctx ms.
    * This parameter has no effect in the cdict ms. */
    pub prefetchCDictTables: std::ffi::c_int,

    /* When == 0, lazy match finders insert every position.
    * When != 0, lazy match finders only insert positions they search.
    * This allows them to skip much faster over incompressible data,
    * at a small cost to compression ratio.
    */
    pub lazySkipping: std::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_MatchState_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: u32,
    pub checksum: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmMatchCandidate_t {
    pub split: *const u8,
    pub hash: u32,
    pub checksum: u32,
    pub bucket: *mut ldmEntry_t,
}

pub const LDM_BATCH_SIZE: usize = 64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t, /* State for the window round buffer management */
    pub hashTable: *mut ldmEntry_t,
    pub loadedDictEnd: u32,
    pub bucketOffsets: *mut u8, /* Next position in bucket to insert entry */
    pub splitIndices: [usize; LDM_BATCH_SIZE],
    pub matchCandidates: [ldmMatchCandidate_t; LDM_BATCH_SIZE],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: ZSTD_ParamSwitch_e, /* ZSTD_ps_enable to enable LDM. ZSTD_ps_auto by default */
    pub hashLog: u32, /* Log size of hashTable */
    pub bucketSizeLog: u32, /* Log bucket size for collision resolution, at most 8 */
    pub minMatchLength: u32, /* Minimum match length */
    pub hashRateLog: u32, /* Log number of entries to skip */
    pub windowLog: u32, /* Window log for the LDM */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: std::ffi::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: usize,
    pub maxSequences: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_params_s {
    pub format: ZSTD_format_e,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,

    pub compressionLevel: std::ffi::c_int,
    pub forceWindow: std::ffi::c_int, /* force back-references to respect limit of
                                * 1<<wLog, even for dictionary */
    pub targetCBlockSize: usize, /* Tries to fit compressed block size to be around targetCBlockSize.
                                * No target when targetCBlockSize == 0.
                                * There is no guarantee on compressed block size */
    pub srcSizeHint: std::ffi::c_int, /* User's best guess of source size.
                                * Hint is not valid when srcSizeHint == 0.
                                * There is no guarantee that hint is close to actual source size */

    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,

    /* Multithreading: used to pass parameters to mtctx */
    pub nbWorkers: std::ffi::c_int,
    pub jobSize: usize,
    pub overlapLog: std::ffi::c_int,
    pub rsyncable: std::ffi::c_int,

    /* Long distance matching parameters */
    pub ldmParams: ldmParams_t,

    /* Dedicated dict search algorithm trigger */
    pub enableDedicatedDictSearch: std::ffi::c_int,

    /* Input/output buffer modes */
    pub inBufferMode: ZSTD_bufferMode_e,
    pub outBufferMode: ZSTD_bufferMode_e,

    /* Sequence compression API */
    pub blockDelimiters: ZSTD_SequenceFormat_e,
    pub validateSequences: std::ffi::c_int,

    /* Block splitting
     * @postBlockSplitter executes split analysis after sequences are produced,
     * it's more accurate but consumes more resources.
     * @preBlockSplitter_level splits before knowing sequences,
     * it's more approximative but also cheaper.
     * Valid @preBlockSplitter_level values range from 0 to 6 (included).
     * 0 means auto, 1 means do not split,
     * then levels are sorted in increasing cpu budget, from 2 (fastest) to 6 (slowest).
     * Highest @preBlockSplitter_level combines well with @postBlockSplitter.
     */
    pub postBlockSplitter: ZSTD_ParamSwitch_e,
    pub preBlockSplitter_level: std::ffi::c_int,

    /* Adjust the max block size*/
    pub maxBlockSize: usize,

    /* Param for deciding whether to use row-based matchfinder */
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,

    /* Always load a dictionary in ext-dict mode (not prefix mode)? */
    pub deterministicRefPrefix: std::ffi::c_int,

    /* Internal use, for createCCtxParams() and freeCCtxParams() only */
    pub customMem: ZSTD_customMem,

    /* Controls prefetching in some dictMatchState matchfinders */
    pub prefetchCDictTables: ZSTD_ParamSwitch_e,

    /* Controls whether zstd will fall back to an internal matchfinder
     * if the external matchfinder returns an error code. */
    pub enableMatchFinderFallback: std::ffi::c_int,
    
    /* Parameters for the external sequence producer API.
     * Users set these parameters through ZSTD_registerSequenceProducer().
     * It is not possible to set these parameters individually through the public API. */
    pub extSeqProdState: *mut std::ffi::c_void,
    pub extSeqProdFunc: ZSTD_sequenceProducer_F,

    /* Controls repcode search in external sequence parsing */
    pub searchForExternalRepcodes: ZSTD_ParamSwitch_e,
}

pub const COMPRESS_SEQUENCES_WORKSPACE_SIZE: usize = std::mem::size_of::<std::ffi::c_uint>() * (MaxSeq as usize + 2);
pub const ENTROPY_WORKSPACE_SIZE: usize = HUF_WORKSPACE_SIZE + COMPRESS_SEQUENCES_WORKSPACE_SIZE;
pub const TMP_WORKSPACE_SIZE: usize = std::cmp::max(ENTROPY_WORKSPACE_SIZE, ZSTD_SLIPBLOCK_WORKSPACESIZE);

/**
 * Indicates whether this compression proceeds directly from user-provided
 * source buffer to user-provided destination buffer (ZSTDb_not_buffered), or
 * whether the context needs to buffer the input/output (ZSTDb_buffered).
 */
pub type ZSTD_buffered_policy_e = std::ffi::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;

/**
 * Struct that contains all elements of block splitter that should be allocated
 * in a wksp.
 */
pub const ZSTD_MAX_NB_BLOCK_SPLITS: usize = 196;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: SeqStore_t,
    pub firstHalfSeqStore: SeqStore_t,
    pub secondHalfSeqStore: SeqStore_t,
    pub currSeqStore: SeqStore_t,
    pub nextSeqStore: SeqStore_t,
    pub partitions: [u32; ZSTD_MAX_NB_BLOCK_SPLITS],
    pub entropyMetadata: ZSTD_entropyCTablesMetadata_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: std::ffi::c_int, /* == 1 if cParams(except wlog) or compression level are changed in requestedParams. Triggers transmission of new params to ZSTDMT (if available) then reset to 0. */
    pub bmi2: std::ffi::c_int, /* == 1 if the CPU supports BMI2 and 0 otherwise. CPU support is determined dynamically once per context lifetime. */
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub simpleApiParams: ZSTD_CCtx_params, /* Param storage used by the simple API - not sticky. Must only be used in top-level simple API functions for storage. */
    pub dictID: u32,
    pub dictContentSize: usize,

    pub workspace: ZSTD_cwksp, /* manages buffer for dynamic allocations */
    pub blockSizeMax: usize,
    pub pledgedSrcSizePlusOne: std::ffi::c_ulonglong, /* this way, 0 (default) == unknown */
    pub consumedSrcSize: std::ffi::c_ulonglong,
    pub producedCSize: std::ffi::c_ulonglong,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    // pub pool: *mut ZSTD_threadPool,
    pub staticSize: usize,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: std::ffi::c_int,
    pub initialized: std::ffi::c_int,

    pub seqStore: SeqStore_t, /* sequences storage ptrs */
    pub ldmState: ldmState_t, /* long distance matching state */
    pub ldmSequences: *mut rawSeq, /* Storage for the ldm output sequences */
    pub maxNbLdmSequences: usize,
    pub externSeqStore: RawSeqStore_t, /* Mutable reference to external sequences */
    pub blockState: ZSTD_blockState_t,
    pub tmpWorkspace: *mut std::ffi::c_void,
    pub tmpWkspSize: usize, /* used as substitute of stack space - must be aligned for S64 type */

    /* Whether we are streaming or not */
    pub bufferedPolicy: ZSTD_buffered_policy_e,

    /* streaming */
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: usize,
    pub inToCompress: usize,
    pub inBuffPos: usize,
    pub inBuffTarget: usize,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: usize,
    pub outBuffContentSize: usize,
    pub outBuffFlushedSize: usize,
    pub streamStage: ZSTD_cStreamStage,
    pub frameEnded: u32,

    /* Stable in/out buffer verification */
    pub expectedInBuffer: ZSTD_inBuffer,
    pub stableIn_notConsumed: usize, /* nb bytes within stable input buffer that are said to be consumed but are not */
    pub expectedOutBufferSize: usize,

    /* Dictionary */
    pub localDict: ZSTD_localDict,
    pub cdict: *const ZSTD_CDict,
    pub prefixDict: ZSTD_prefixDict, /* single-usage dictionary */

    // /* Multi-threading */
    // // TODO #[cfg(feature = "multithread")]
    // pub mtctx: *mut ZSTDMT_CCtx,

    // /* Tracing */
    // // TODO #[cfg(feature = "trace")]
    // pub traceCtx: ZSTD_TraceCtx,

    /* Workspace for block splitter */
    pub blockSplitCtx: ZSTD_blockSplitCtx,

    /* Buffer for output from external sequence producer */
    pub extSeqBuf: *mut ZSTD_Sequence,
    pub extSeqBufCapacity: usize,
}

pub type ZSTD_dictTableLoadMethod_e = std::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;

pub type ZSTD_tableFillPurpose_e = std::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;

pub type ZSTD_dictMode_e = std::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;

pub type ZSTD_CParamMode_e = std::ffi::c_uint;
/* ZSTD_getCParams, ZSTD_getParams, ZSTD_adjustParams.
* We don't know what these parameters are for. We default to the legacy
* behavior of taking both the source size and the dict size into account
* when selecting and adjusting parameters.
*/
pub const ZSTD_cpm_unknown: ZSTD_CParamMode_e = 3;
/* Creating a CDict.
* In this mode we take both the source size and the dictionary size
* into account when selecting and adjusting the parameters.
*/
pub const ZSTD_cpm_createCDict: ZSTD_CParamMode_e = 2;
/* Compression with ZSTD_dictMatchState or ZSTD_dedicatedDictSearch.
* In this mode we only take the srcSize into account when selecting
* and adjusting parameters.
*/
pub const ZSTD_cpm_attachDict: ZSTD_CParamMode_e = 1; 
/* Compression with ZSTD_noDict or ZSTD_extDict.
* In this mode we use both the srcSize and the dictSize
* when selecting and adjusting parameters.
*/
pub const ZSTD_cpm_noAttachDict: ZSTD_CParamMode_e = 0;

pub type ZSTD_BlockCompressor_f = Option::<
    unsafe extern "C" fn(
        *mut ZSTD_MatchState_t,
        *mut SeqStore_t,
        *mut u32,
        *const std::ffi::c_void,
        usize,
    ) -> usize
>;
pub use crate::compress::zstd_compress::ZSTD_selectBlockCompressor;

#[inline]
#[rustfmt::skip]
pub fn ZSTD_LLcode(litLength: u32) -> u32 {
    static LL_Code: [u8; 64] = [       0,  1,  2,  3,  4,  5,  6,  7,
                                       8,  9, 10, 11, 12, 13, 14, 15,
                                      16, 16, 17, 17, 18, 18, 19, 19,
                                      20, 20, 20, 20, 21, 21, 21, 21,
                                      22, 22, 22, 22, 22, 22, 22, 22,
                                      23, 23, 23, 23, 23, 23, 23, 23,
                                      24, 24, 24, 24, 24, 24, 24, 24,
                                      24, 24, 24, 24, 24, 24, 24, 24
    ];
    const LL_deltaCode: u32 = 19;
    if litLength > 63 {
        ZSTD_highbit32(litLength).wrapping_add(LL_deltaCode)
    } else {
        u32::from(LL_Code[litLength as usize])
    }
}

/* ZSTD_MLcode() :
 * note : mlBase = matchLength - MINMATCH;
 *        because it's the format it's stored in seqStore->sequences */
pub fn ZSTD_MLcode(mut mlBase: u32) -> u32 {
    static ML_Code: [u8; 128] = [      0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                                      16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                      32, 32, 33, 33, 34, 34, 35, 35, 36, 36, 36, 36, 37, 37, 37, 37,
                                      38, 38, 38, 38, 38, 38, 38, 38, 39, 39, 39, 39, 39, 39, 39, 39,
                                      40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
                                      41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41,
                                      42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                                      42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42
    ];
    const ML_deltaCode: u32 = 36;
    if mlBase > 127 {
        ZSTD_highbit32(mlBase).wrapping_add(ML_deltaCode)
    } else {
        u32::from(ML_Code[mlBase as usize])
    }
}

/* ZSTD_cParam_withinBounds:
 * @return 1 if value is within cParam bounds,
 * 0 otherwise */
#[inline]
pub fn ZSTD_cParam_withinBounds(
    cParam: ZSTD_cParameter,
    value: std::ffi::c_int,
) -> bool {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) {
        return false;
    }
    if value < bounds.lowerBound {
        return false;
    }
    if value > bounds.upperBound {
        return false;
    }
    
    true
}

/* ZSTD_selectAddr:
 * @return index >= lowLimit ? candidate : backup,
 * tries to force branchless codegen. */
 #[inline]
pub unsafe fn ZSTD_selectAddr(
    mut index: u32,
    mut lowLimit: u32,
    mut candidate: *const u8,
    mut backup: *const u8,
) -> *const u8 {
    #[cfg(target_arch = "x86_64")]
    {
        std::arch::asm!(
            "cmp {1}, {2}\ncmova {3}, {0}\n", inlateout(reg) candidate, inlateout(reg) index
            => _, inlateout(reg) lowLimit => _, inlateout(reg) backup => _,
            options(preserves_flags, pure, readonly, att_syntax)
        );
        return candidate;
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        if index >= lowLimit {
            candidate
        } else {
            backup
        }
    }
}

/* ZSTD_noCompressBlock() :
 * Writes uncompressed block to dst buffer from given src.
 * Returns the size of the block */
#[inline]
pub unsafe fn ZSTD_noCompressBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastBlock: u32,
) -> usize {
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_raw as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    RETURN_ERROR_IF!(srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity, ZSTD_error_dstSize_tooSmall);
    MEM_writeLE24(dst, cBlockHeader24);
    libc::memcpy(dst.byte_add(ZSTD_blockHeaderSize), src, srcSize);
    return ZSTD_blockHeaderSize.wrapping_add(srcSize);
}

#[inline]
pub unsafe fn ZSTD_rleCompressBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: u8,
    mut srcSize: usize,
    mut lastBlock: u32,
) -> usize {
    let op = dst as *mut u8;
    let cBlockHeader = lastBlock
        .wrapping_add((bt_rle as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    RETURN_ERROR_IF!(dstCapacity < 4, ZSTD_error_dstSize_tooSmall);
    MEM_writeLE24(op as *mut std::ffi::c_void, cBlockHeader);
    *op.offset(3) = src;
    return 4;
}

/* ZSTD_minGain() :
 * minimum compression required
 * to generate a compress block or a compressed literals section.
 * note : use same formula for both situations */
 #[inline]
pub unsafe fn ZSTD_minGain(
    srcSize: usize,
    strat: ZSTD_strategy,
) -> usize {
    let minlog = if strat >= ZSTD_btultra {
        (strat as u32) - 1
    } else {
        6_u32
    };
    (srcSize >> minlog).wrapping_add(2)
}

#[inline]
pub unsafe fn ZSTD_literalsCompressionIsDisabled(
    cctxParams: *const ZSTD_CCtx_params,
) -> bool {
    match (*cctxParams).literalCompressionMode {
        ZSTD_ps_enable => false,
        ZSTD_ps_disable => true,
        x @ (ZSTD_ps_auto | _) => {
            if x != ZSTD_ps_auto {
                debug_assert!(false, "impossible: pre-validated");
            }
            (*cctxParams).cParams.strategy == ZSTD_fast
                && (*cctxParams).cParams.targetLength > 0
        }
    }
}

/** ZSTD_safecopyLiterals() :
 *  memcpy() function that won't read beyond more than WILDCOPY_OVERLENGTH bytes past ilimit_w.
 *  Only called when the sequence ends past ilimit_w, so it only needs to be optimized for single
 *  large copies.
 */
pub unsafe fn ZSTD_safecopyLiterals(
    mut op: *mut u8,
    mut ip: *const u8,
    iend: *const u8,
    mut ilimit_w: *const u8,
) {
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            ilimit_w.offset_from(ip) as usize,
            ZSTD_no_overlap,
        );
        op = op.offset(ilimit_w.offset_from(ip) as isize);
        ip = ilimit_w;
    }
    while ip < iend {
        let fresh0 = ip;
        ip = ip.offset(1);
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = *fresh0;
    }
}

/* accepts IDs 1,2,3 */
pub const fn REPCODE_TO_OFFBASE(r: u32) -> u32 {
    debug_assert!(r >= 1);
    debug_assert!(r <= ZSTD_REP_NUM);
    r
}

pub const fn OFFSET_TO_OFFBASE(o: u32) -> u32 {
    debug_assert!(o > 0);
    o + ZSTD_REP_NUM
}

pub const fn OFFBASE_IS_OFFSET(o: u32) -> bool {
    o > ZSTD_REP_NUM
}

pub const fn OFFBASE_IS_REPCODE(o: u32) -> bool {
    1 <= o && o <= ZSTD_REP_NUM
}

pub const fn OFFBASE_TO_OFFSET(o: u32) -> u32 {
    debug_assert!(OFFBASE_IS_OFFSET(o));
    o - ZSTD_REP_NUM
}

/* returns ID 1,2,3 */
pub const fn OFFBASE_TO_REPCODE(o: u32) -> u32 {
    debug_assert!(OFFBASE_IS_REPCODE(o));
    o
}

pub const REPCODE1_TO_OFFBASE: u32 = REPCODE_TO_OFFBASE(1);
pub const REPCODE2_TO_OFFBASE: u32 = REPCODE_TO_OFFBASE(2);
pub const REPCODE3_TO_OFFBASE: u32 = REPCODE_TO_OFFBASE(3);

/** ZSTD_storeSeqOnly() :
 *  Store a sequence (litlen, litPtr, offBase and matchLength) into SeqStore_t.
 *  Literals themselves are not copied, but @litPtr is updated.
 *  @offBase : Users should employ macros REPCODE_TO_OFFBASE() and OFFSET_TO_OFFBASE().
 *  @matchLength : must be >= MINMATCH
*/
#[inline(always)]
pub unsafe fn ZSTD_storeSeqOnly(
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: usize,
    mut offBase: u32,
    mut matchLength: usize,
) {
    debug_assert!((*seqStorePtr).sequences.offset_from((*seqStorePtr).sequencesStart) < ((*seqStorePtr).maxNbSeq) as isize);

    /* literal Length */
    debug_assert!(litLength <= ZSTD_BLOCKSIZE_MAX);
    if UNLIKELY!(litLength > 0xFFFF) {
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength; /* there can only be a single long length */
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as u32;
    }
    (*((*seqStorePtr).sequences).offset(0))
        .litLength = litLength as u16;

    /* match offset */
    (*((*seqStorePtr).sequences).offset(0))
        .offBase = offBase;

    /* match Length */
    debug_assert!(matchLength <= ZSTD_BLOCKSIZE_MAX);
    debug_assert!(matchLength >= MINMATCH as usize);
    let mlBase = matchLength.wrapping_sub(MINMATCH as usize);
    if UNLIKELY!(mlBase > 0xFFFF) {
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as u32;
    }
    (*((*seqStorePtr).sequences).offset(0))
        .mlBase = mlBase as u16;

    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
}

/** ZSTD_storeSeq() :
 *  Store a sequence (litlen, litPtr, offBase and matchLength) into SeqStore_t.
 *  @offBase : Users should employ macros REPCODE_TO_OFFBASE() and OFFSET_TO_OFFBASE().
 *  @matchLength : must be >= MINMATCH
 *  Allowed to over-read literals up to litLimit.
*/
#[inline(always)]
pub unsafe fn ZSTD_storeSeq(
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: usize,
    mut literals: *const u8,
    mut litLimit: *const u8,
    mut offBase: u32,
    mut matchLength: usize,
) {
    let litLimit_w = litLimit.sub(WILDCOPY_OVERLENGTH);
    let litEnd = literals.add(litLength);

    // TODO
    // #if defined(DEBUGLEVEL) && (DEBUGLEVEL >= 6)
    //     static const BYTE* g_start = NULL;
    //     if (g_start==NULL) g_start = (const BYTE*)literals;  /* note : index only works for compression within a single segment */
    //     {   U32 const pos = (U32)((const BYTE*)literals - g_start);
    //         DEBUGLOG(6, "Cpos%7u :%3u literals, match%4u bytes at offBase%7u",
    //                pos, (U32)litLength, (U32)matchLength, (U32)offBase);
    //     }
    // #endif

    debug_assert!((*seqStorePtr).sequences.offset_from((*seqStorePtr).sequencesStart) < ((*seqStorePtr).maxNbSeq) as isize);
    /* copy Literals */
    debug_assert!((*seqStorePtr).maxNbSeq <= KB(128));
    debug_assert!((*seqStorePtr).lit.add(litLength) <= (*seqStorePtr).litStart.add((*seqStorePtr).maxNbLit));
    if litEnd <= litLimit_w {
        /* Common case we can use wildcopy.
        * First copy 16 bytes, because literals are likely short.
        */
        const _: () = assert!(WILDCOPY_OVERLENGTH >= 16);
        ZSTD_copy16(
            (*seqStorePtr).lit as *mut std::ffi::c_void,
            literals as *const std::ffi::c_void,
        );
        if litLength > 16 {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16)
                    as *mut std::ffi::c_void,
                literals.offset(16)
                    as *const std::ffi::c_void,
                litLength.wrapping_sub(16),
                ZSTD_no_overlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).add(litLength);
    ZSTD_storeSeqOnly(seqStorePtr, litLength, offBase, matchLength);
}

/* ZSTD_updateRep() :
 * updates in-place @rep (array of repeat offsets)
 * @offBase : sum-type, using numeric representation of ZSTD_storeSeq()
 */
#[inline]
pub unsafe fn ZSTD_updateRep(mut rep: *mut u32, offBase: u32, ll0: u32) {
    if OFFBASE_IS_OFFSET(offBase) { /* full offset */
        *rep
            .offset(
                2,
            ) = *rep.offset(1);
        *rep
            .offset(
                1,
            ) = *rep.offset(0);
        *rep.offset(0) = OFFBASE_TO_OFFSET(offBase);
    } else { /* repcode */
        let repCode = OFFBASE_TO_REPCODE(offBase)
            .wrapping_sub(1)
            .wrapping_add(ll0);
        if repCode > 0 { /* note : if repCode==0, no change */
            let currentOffset = if repCode == ZSTD_REP_NUM {
                (*rep.offset(0)).wrapping_sub(1)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep
                .offset(
                    2,
                ) = if repCode >= 2 {
                *rep.offset(1)
            } else {
                *rep.offset(2)
            };
            *rep
                .offset(
                    1,
                ) = *rep.offset(0);
            *rep.offset(0) = currentOffset;
        } else { /* repCode == 0 */
            /* nothing to do */
        }
    };
}

pub type Repcodes_t = repcodes_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [u32; 3],
}

#[inline]
pub unsafe fn ZSTD_newRep(
    mut rep: *const u32, // rep[ZSTD_REP_NUM]
    offBase: u32,
    ll0: u32,
) -> Repcodes_t {
    let mut newReps = repcodes_s { rep: [0; 3] };
    libc::memcpy(
        &mut newReps as *mut Repcodes_t as *mut std::ffi::c_void,
        rep as *const std::ffi::c_void,
        size_of::<Repcodes_t>(),
    );
    ZSTD_updateRep((newReps.rep).as_mut_ptr(), offBase, ll0);
    newReps
}

/*-*************************************
*  Match length counter
***************************************/
#[inline]
pub unsafe fn ZSTD_count(
    mut pIn: *const u8,
    mut pMatch: *const u8,
    pInLimit: *const u8,
) -> usize {
    let pStart = pIn;
    let pInLoopLimit = pInLimit
        .sub(size_of::<usize>() - 1);
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch.cast())
            ^ MEM_readST(pIn.cast());
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as usize;
        }
        pIn = pIn.add(size_of::<usize>());
        pMatch = pMatch
            .add(size_of::<usize>());
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch.cast())
                ^ MEM_readST(pIn.cast());
            if diff_0 == 0 {
                pIn = pIn
                    .add(
                        size_of::<usize>()
                    );
                pMatch = pMatch
                    .add(
                        size_of::<usize>()
                    );
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as usize;
            }
        }
    }
    if MEM_64bits && pIn < pInLimit.offset(-3_isize)
        && MEM_read32(pMatch.cast())
            == MEM_read32(pIn.cast())
    {
        pIn = pIn.offset(4);
        pMatch = pMatch.offset(4);
    }
    if pIn < pInLimit.offset(-1_isize)
        && MEM_read16(pMatch.cast())
            == MEM_read16(pIn.cast())
    {
        pIn = pIn.offset(2);
        pMatch = pMatch.offset(2);
    }
    if pIn < pInLimit && *pMatch == *pIn {
        pIn = pIn.offset(1);
    }
    return pIn.offset_from(pStart) as usize;
}

/** ZSTD_count_2segments() :
 *  can count match length with `ip` & `match` in 2 different segments.
 *  convention : on reaching mEnd, match count continue starting from iStart
 */
#[inline]
pub unsafe fn ZSTD_count_2segments(
    mut ip: *const u8,
    mut match_0: *const u8,
    mut iEnd: *const u8,
    mut mEnd: *const u8,
    mut iStart: *const u8,
) -> usize {
    let vEnd = std::cmp::min(ip.offset(mEnd.offset_from(match_0)), iEnd);
    let matchLength = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength;
    }

    // TODO
    // DEBUGLOG(7, "ZSTD_count_2segments: found a 2-parts match (current length==%zu)", matchLength);
    // DEBUGLOG(7, "distance from match beginning to end dictionary = %i", (int)(mEnd - match));
    // DEBUGLOG(7, "distance from current pos to end buffer = %i", (int)(iEnd - ip));
    // DEBUGLOG(7, "next byte : ip==%02X, istart==%02X", ip[matchLength], *iStart);
    // DEBUGLOG(7, "final match length = %zu", matchLength + ZSTD_count(ip+matchLength, iStart, iEnd));
    return matchLength
        .wrapping_add(ZSTD_count(ip.add(matchLength), iStart, iEnd));
}

/*-*************************************
 *  Hashes
 ***************************************/
pub const prime3bytes: u32 = 506832829;
pub unsafe fn ZSTD_hash3(u: u32, h: u32, s: u32) -> u32 {
    (((u << (32-24)).wrapping_mul(prime3bytes)) ^ s) >> 32_u32.wrapping_sub(h)
}
pub unsafe fn ZSTD_hash3Ptr( /* only in zstd_opt.h */
    ptr: *const std::ffi::c_void,
    h: u32,
) -> usize {
    ZSTD_hash3(MEM_readLE32(ptr), h, 0) as usize
}
pub unsafe fn ZSTD_hash3PtrS(
    ptr: *const std::ffi::c_void,
    h: u32,
    s: u32,
) -> usize {
    ZSTD_hash3(MEM_readLE32(ptr), h, 0) as usize
}

pub const prime4bytes: u32 = 2654435761;
pub unsafe fn ZSTD_hash4(u: u32, h: u32, s: u32) -> u32 {
    (u.wrapping_mul(prime4bytes) ^ s) >> 32_u32.wrapping_sub(h)
}
pub unsafe fn ZSTD_hash4Ptr(
    ptr: *const std::ffi::c_void,
    h: u32,
) -> usize {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0) as usize
}
pub unsafe fn ZSTD_hash4PtrS(
    ptr: *const std::ffi::c_void,
    h: u32,
    s: u32,
) -> usize {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0) as usize
}

pub const prime5bytes: u64 = 889523592379;
pub unsafe fn ZSTD_hash5(u: u64, h: u32, s: u64) -> usize {
    (((u << 64 - 40).wrapping_mul(prime5bytes) ^ s) >> 64_u32.wrapping_sub(h)) as usize
}
pub unsafe fn ZSTD_hash5Ptr(
    p: *const std::ffi::c_void,
    h: u32,
) -> usize {
    ZSTD_hash5(MEM_readLE64(p), h, 0)
}
pub unsafe fn ZSTD_hash5PtrS(
    p: *const std::ffi::c_void,
    h: u32,
    s: u64,
) -> usize {
    ZSTD_hash5(MEM_readLE64(p), h, 0)
}

pub const prime6bytes: u64 = 227718039650203;
pub fn ZSTD_hash6(u: u64, h: u32, s: u64) -> usize {
    (((u << 64 - 48).wrapping_mul(prime6bytes) ^ s) >> 64_u32.wrapping_sub(h)) as usize
}
pub unsafe fn ZSTD_hash6Ptr(
    p: *const std::ffi::c_void,
    h: u32,
) -> usize {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}
pub unsafe fn ZSTD_hash6PtrS(
    p: *const std::ffi::c_void,
    h: u32,
    s: u64,
) -> usize {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}

pub const prime7bytes: u64 = 58295818150454627;
pub unsafe fn ZSTD_hash7(u: u64, h: u32, s: u64) -> usize {
    (((u << 64 - 56).wrapping_mul(prime7bytes) ^ s) >> 64_u32.wrapping_sub(h)) as usize
}
pub unsafe fn ZSTD_hash7Ptr(
    p: *const std::ffi::c_void,
    h: u32,
) -> usize {
    ZSTD_hash7(MEM_readLE64(p), h, 0)
}
pub unsafe fn ZSTD_hash7PtrS(
    p: *const std::ffi::c_void,
    h: u32,
    s: u64,
) -> usize {
    ZSTD_hash7(MEM_readLE64(p), h, 0)
}

pub const prime8bytes: u64 = 0xcf1bbcdcb7a56463;
pub fn ZSTD_hash8(u: u64, h: u32, s: u64) -> usize {
    ((u.wrapping_mul(prime8bytes) ^ s) >> 64_u32.wrapping_sub(h)) as usize
}
pub unsafe fn ZSTD_hash8Ptr(
    p: *const std::ffi::c_void,
    h: u32,
) -> usize {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}
pub unsafe fn ZSTD_hash8PtrS(
    p: *const std::ffi::c_void,
    h: u32,
    s: u64,
) -> usize {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}

#[inline(always)]
pub unsafe fn ZSTD_hashPtr(
    p: *const std::ffi::c_void,
    hBits: u32,
    mls: u32,
) -> usize {
    /* Although some of these hashes do support hBits up to 64, some do not.
     * To be on the safe side, always avoid hBits > 32. */
    debug_assert!(hBits <= 32);

    match mls {
        5 => ZSTD_hash5Ptr(p, hBits),
        6 => ZSTD_hash6Ptr(p, hBits),
        7 => ZSTD_hash7Ptr(p, hBits),
        8 => ZSTD_hash8Ptr(p, hBits),
        4 | _ => ZSTD_hash4Ptr(p, hBits),
    }
}

#[inline(always)]
pub unsafe fn ZSTD_hashPtrS(
    p: *const std::ffi::c_void,
    hBits: u32,
    mls: u32,
    hashSalt: u64,
) -> usize {
    /* Although some of these hashes do support hBits up to 64, some do not.
     * To be on the safe side, always avoid hBits > 32. */
    debug_assert!(hBits <= 32);

    match mls {
        5 => ZSTD_hash5PtrS(p, hBits, hashSalt),
        6 => ZSTD_hash6PtrS(p, hBits, hashSalt),
        7 => ZSTD_hash7PtrS(p, hBits, hashSalt),
        8 => ZSTD_hash8PtrS(p, hBits, hashSalt),
        4 | _ => ZSTD_hash4PtrS(p, hBits, hashSalt as u32),
    }
}

/** ZSTD_ipow() :
 * Return base^exponent.
 */
pub unsafe fn ZSTD_ipow(base: u64, exponent: u64) -> u64 {
    base.pow(exponent as u32)
}

pub const ZSTD_ROLL_HASH_CHAR_OFFSET: u64 = 10;

/** ZSTD_rollingHash_append() :
 * Add the buffer to the hash value.
 */
pub unsafe fn ZSTD_rollingHash_append(
    mut hash: u64,
    buf: *const std::ffi::c_void,
    size: usize,
) -> u64 {
    let mut istart = buf as *const u8;
    let mut pos: usize = 0;
    while pos < size {
        hash = hash.wrapping_mul(prime8bytes);
        hash = hash
            .wrapping_add(
                u64::from(*istart.add(pos)) + ZSTD_ROLL_HASH_CHAR_OFFSET,
            );
        pos = pos.wrapping_add(1);
    }
    hash
}

/** ZSTD_rollingHash_compute() :
 * Compute the rolling hash value of the buffer.
 */
#[inline]
pub unsafe fn ZSTD_rollingHash_compute(
    buf: *const std::ffi::c_void,
    size: usize,
) -> u64 {
    ZSTD_rollingHash_append(0, buf, size)
}

/** ZSTD_rollingHash_primePower() :
 * Compute the primePower to be passed to ZSTD_rollingHash_rotate() for a hash
 * over a window of length bytes.
 */
#[inline]
pub unsafe fn ZSTD_rollingHash_primePower(length: u32) -> u64 {
    ZSTD_ipow(
        prime8bytes,
        length.wrapping_sub(1) as u64,
    )
}

/** ZSTD_rollingHash_rotate() :
 * Rotate the rolling hash by one byte.
 */
#[inline]
pub unsafe fn ZSTD_rollingHash_rotate(
    mut hash: u64,
    toRemove: u8,
    toAdd: u8,
    primePower: u64,
) -> u64 {
    hash = hash
        .wrapping_sub(
            (toRemove as u64 + ZSTD_ROLL_HASH_CHAR_OFFSET).wrapping_mul(primePower)
        );
    hash = hash.wrapping_mul(prime8bytes);
    hash = hash
        .wrapping_add(toAdd as u64 + ZSTD_ROLL_HASH_CHAR_OFFSET);
    hash
}

/*-*************************************
*  Round buffer management
***************************************/
/* Max @current value allowed:
 * In 32-bit mode: we want to avoid crossing the 2 GB limit,
 *                 reducing risks of side effects in case of signed operations on indexes.
 * In 64-bit mode: we want to ensure that adding the maximum job size (512 MB)
 *                 doesn't overflow U32 index capacity (4 GB) */
pub const ZSTD_CURRENT_MAX: usize = if MEM_64bits {
    MB(3500)
} else {
    MB(2000)
};

/* Maximum chunk size before overflow correction needs to be called again */
pub const ZSTD_CHUNKSIZE_MAX: usize =
    (u32::MAX as usize) /* Maximum ending current index */
    - ZSTD_CURRENT_MAX; /* Maximum beginning lowLimit */

/**
 * ZSTD_window_clear():
 * Clears the window containing the history by simply setting it to empty.
 */
#[inline]
pub unsafe fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base);
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}

pub unsafe fn ZSTD_window_isEmpty(window: ZSTD_window_t) -> bool {
    window.dictLimit == ZSTD_WINDOW_START_INDEX &&
    window.lowLimit == ZSTD_WINDOW_START_INDEX &&
    window.nextSrc.offset_from(window.base) == ZSTD_WINDOW_START_INDEX as isize
}

/**
 * ZSTD_window_hasExtDict():
 * Returns non-zero if the window has a non-empty extDict.
 */
pub fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> bool {
    window.lowLimit < window.dictLimit
}

/**
 * ZSTD_matchState_dictMode():
 * Inspects the provided matchState and figures out what dictMode should be
 * passed to the compressor.
 */
#[inline]
pub unsafe fn ZSTD_matchState_dictMode(
    mut ms: *const ZSTD_MatchState_t,
) -> ZSTD_dictMode_e {
    if ZSTD_window_hasExtDict((*ms).window) {
        ZSTD_extDict
    } else if !((*ms).dictMatchState).is_null() {
        if (*(*ms).dictMatchState).dedicatedDictSearch != 0 {
            ZSTD_dedicatedDictSearch
        } else {
            ZSTD_dictMatchState
        }
    } else {
        ZSTD_noDict
    }
}

/* Defining this macro to non-zero tells zstd to run the overflow correction
 * code much more frequently. This is very inefficient, and should only be
 * used for tests and fuzzers.
 */
pub const ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY: std::ffi::c_int = 0; // TODO configurable?

/**
 * ZSTD_window_canOverflowCorrect():
 * Returns non-zero if the indices are large enough for overflow correction
 * to work correctly without impacting compression ratio.
 */
#[inline]
pub unsafe fn ZSTD_window_canOverflowCorrect(
    window: ZSTD_window_t,
    mut cycleLog: u32,
    mut maxDist: u32,
    mut loadedDictEnd: u32,
    mut src: *const std::ffi::c_void,
) -> bool {
    let cycleSize = 1_u32 << cycleLog;
    let curr = (src as *const u8).offset_from(window.base) as u32;
    let minIndexToOverflowCorrect = cycleSize
        .wrapping_add(std::cmp::max(maxDist, cycleSize))
        .wrapping_add(ZSTD_WINDOW_START_INDEX);

    /* Adjust the min index to backoff the overflow correction frequency,
     * so we don't waste too much CPU in overflow correction. If this
     * computation overflows we don't really care, we just need to make
     * sure it is at least minIndexToOverflowCorrect.
     */
    let adjustment = window.nbOverflowCorrections.wrapping_add(1);
    let adjustedIndex = std::cmp::max(
        minIndexToOverflowCorrect * adjustment, minIndexToOverflowCorrect
    );
    let indexLargeEnough = curr > adjustedIndex;

    /* Only overflow correct early if the dictionary is invalidated already,
     * so we don't hurt compression ratio.
     */
    let dictionaryInvalidated = curr > maxDist.wrapping_add(loadedDictEnd);

    indexLargeEnough && dictionaryInvalidated
}

/**
 * ZSTD_window_needOverflowCorrection():
 * Returns non-zero if the indices are getting too large and need overflow
 * protection.
 */
#[inline]
pub unsafe fn ZSTD_window_needOverflowCorrection(
    window: ZSTD_window_t,
    mut cycleLog: u32,
    mut maxDist: u32,
    mut loadedDictEnd: u32,
    mut src: *const std::ffi::c_void,
    mut srcEnd: *const std::ffi::c_void,
) -> bool {
    let curr = (srcEnd as *const u8).offset_from(window.base) as usize;
    if ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY > 0 {
        ZSTD_window_canOverflowCorrect(window, cycleLog, maxDist, loadedDictEnd, src)
    } else {
        curr > ZSTD_CURRENT_MAX
    }
}

/**
 * ZSTD_window_correctOverflow():
 * Reduces the indices to protect from index overflow.
 * Returns the correction made to the indices, which must be applied to every
 * stored index.
 *
 * The least significant cycleLog bits of the indices must remain the same,
 * which may be 0. Every index up to maxDist in the past must be valid.
 */
#[inline]
pub unsafe fn ZSTD_window_correctOverflow(
    mut window: *mut ZSTD_window_t,
    mut cycleLog: u32,
    mut maxDist: u32,
    mut src: *const std::ffi::c_void,
) -> u32 {
    /* preemptive overflow correction:
     * 1. correction is large enough:
     *    lowLimit > (3<<29) ==> current > 3<<29 + 1<<windowLog
     *    1<<windowLog <= newCurrent < 1<<chainLog + 1<<windowLog
     *
     *    current - newCurrent
     *    > (3<<29 + 1<<windowLog) - (1<<windowLog + 1<<chainLog)
     *    > (3<<29) - (1<<chainLog)
     *    > (3<<29) - (1<<30)             (NOTE: chainLog <= 30)
     *    > 1<<29
     *
     * 2. (ip+ZSTD_CHUNKSIZE_MAX - cctx->base) doesn't overflow:
     *    After correction, current is less than (1<<chainLog + 1<<windowLog).
     *    In 64-bit mode we are safe, because we have 64-bit ptrdiff_t.
     *    In 32-bit mode we are safe, because (chainLog <= 29), so
     *    ip+ZSTD_CHUNKSIZE_MAX - cctx->base < 1<<32.
     * 3. (cctx->lowLimit + 1<<windowLog) < 1<<32:
     *    windowLog <= 31 ==> 3<<29 + 1<<windowLog < 7<<29 < 1<<32.
     */
    let cycleSize = 1_u32 << cycleLog;
    let cycleMask = cycleSize.wrapping_sub(1);
    let curr = src.byte_offset_from((*window).base.cast()) as u32;
    let currentCycle = curr & cycleMask;

    /* Ensure newCurrent - maxDist >= ZSTD_WINDOW_START_INDEX. */
    let currentCycleCorrection = if currentCycle < ZSTD_WINDOW_START_INDEX {
        std::cmp::max(cycleSize, ZSTD_WINDOW_START_INDEX)
    } else {
        0
    };
    let newCurrent = currentCycle
        .wrapping_add(currentCycleCorrection)
        .wrapping_add(std::cmp::max(maxDist, cycleSize));
    let correction = curr.wrapping_sub(newCurrent);

    /* maxDist must be a power of two so that:
     *   (newCurrent & cycleMask) == (curr & cycleMask)
     * This is required to not corrupt the chains / binary tree.
     */
    debug_assert!(maxDist & (maxDist - 1) == 0);
    debug_assert!((curr & cycleMask) == (newCurrent & cycleMask));
    debug_assert!(curr > newCurrent);
    if ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY == 0 {
        /* Loose bound, should be around 1<<29 (see above) */
        debug_assert!(correction > 1_u32 << 28);
    }

    (*window).base = ((*window).base).offset(correction as isize);
    (*window).dictBase = ((*window).dictBase).offset(correction as isize);
    if (*window).lowLimit < correction.wrapping_add(ZSTD_WINDOW_START_INDEX) {
        (*window).lowLimit = ZSTD_WINDOW_START_INDEX;
    } else {
        (*window).lowLimit = ((*window).lowLimit).wrapping_sub(correction);
    }
    if (*window).dictLimit < correction.wrapping_add(ZSTD_WINDOW_START_INDEX) {
        (*window).dictLimit = ZSTD_WINDOW_START_INDEX;
    } else {
        (*window).dictLimit = ((*window).dictLimit).wrapping_sub(correction);
    }

    /* Ensure we can still reference the full window. */
    debug_assert!(newCurrent >= maxDist);
    debug_assert!(newCurrent - maxDist >= ZSTD_WINDOW_START_INDEX);
    /* Ensure that lowLimit and dictLimit didn't underflow. */
    debug_assert!((*window).lowLimit <= newCurrent);
    debug_assert!((*window).dictLimit <= newCurrent);

    (*window).nbOverflowCorrections = ((*window).nbOverflowCorrections).wrapping_add(1);


    DEBUGLOG!(4, "Correction of 0x%x bytes to lowLimit=0x%x", correction, (*window).lowLimit);

    correction
}

/**
 * ZSTD_window_enforceMaxDist():
 * Updates lowLimit so that:
 *    (srcEnd - base) - lowLimit == maxDist + loadedDictEnd
 *
 * It ensures index is valid as long as index >= lowLimit.
 * This must be called before a block compression call.
 *
 * loadedDictEnd is only defined if a dictionary is in use for current compression.
 * As the name implies, loadedDictEnd represents the index at end of dictionary.
 * The value lies within context's referential, it can be directly compared to blockEndIdx.
 *
 * If loadedDictEndPtr is NULL, no dictionary is in use, and we use loadedDictEnd == 0.
 * If loadedDictEndPtr is not NULL, we set it to zero after updating lowLimit.
 * This is because dictionaries are allowed to be referenced fully
 * as long as the last byte of the dictionary is in the window.
 * Once input has progressed beyond window size, dictionary cannot be referenced anymore.
 *
 * In normal dict mode, the dictionary lies between lowLimit and dictLimit.
 * In dictMatchState mode, lowLimit and dictLimit are the same,
 * and the dictionary is below them.
 * forceWindow and dictMatchState are therefore incompatible.
 */
#[inline]
pub unsafe fn ZSTD_window_enforceMaxDist(
    mut window: *mut ZSTD_window_t,
    mut blockEnd: *const std::ffi::c_void,
    mut maxDist: u32,
    mut loadedDictEndPtr: *mut u32,
    mut dictMatchStatePtr: *mut *const ZSTD_MatchState_t,
) {
    let blockEndIdx = blockEnd.byte_offset_from((*window).base.cast()) as u32;
    let loadedDictEnd = if !loadedDictEndPtr.is_null() {
        *loadedDictEndPtr
    } else {
        0
    };
    DEBUGLOG!(5, "ZSTD_window_enforceMaxDist: blockEndIdx=%u, maxDist=%u, loadedDictEnd=%u",
                blockEndIdx, maxDist, loadedDictEnd);

    /* - When there is no dictionary : loadedDictEnd == 0.
         In which case, the test (blockEndIdx > maxDist) is merely to avoid
         overflowing next operation `newLowLimit = blockEndIdx - maxDist`.
       - When there is a standard dictionary :
         Index referential is copied from the dictionary,
         which means it starts from 0.
         In which case, loadedDictEnd == dictSize,
         and it makes sense to compare `blockEndIdx > maxDist + dictSize`
         since `blockEndIdx` also starts from zero.
       - When there is an attached dictionary :
         loadedDictEnd is expressed within the referential of the context,
         so it can be directly compared against blockEndIdx.
    */
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit = blockEndIdx.wrapping_sub(maxDist);
        if (*window).lowLimit < newLowLimit {
            (*window).lowLimit = newLowLimit;
        }
        if (*window).dictLimit < (*window).lowLimit {
            DEBUGLOG!(5, "Update dictLimit to match lowLimit, from %u to %u",
                        (*window).dictLimit, (*window).lowLimit);
            (*window).dictLimit = (*window).lowLimit;
        }
        /* On reaching window size, dictionaries are invalidated */
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr = 0;
        }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = std::ptr::null();
        }
    }
}

/* Similar to ZSTD_window_enforceMaxDist(),
 * but only invalidates dictionary
 * when input progresses beyond window size.
 * assumption : loadedDictEndPtr and dictMatchStatePtr are valid (non NULL)
 *              loadedDictEnd uses same referential as window->base
 *              maxDist is the window size */
#[inline]
pub unsafe fn ZSTD_checkDictValidity(
    mut window: *const ZSTD_window_t,
    mut blockEnd: *const std::ffi::c_void,
    mut maxDist: u32,
    mut loadedDictEndPtr: *mut u32,
    mut dictMatchStatePtr: *mut *const ZSTD_MatchState_t,
) {
    debug_assert!(!loadedDictEndPtr.is_null());
    debug_assert!(!dictMatchStatePtr.is_null());

    let blockEndIdx = blockEnd.byte_offset_from((*window).base.cast()) as u32;
    let loadedDictEnd = *loadedDictEndPtr;
    if blockEndIdx > loadedDictEnd.wrapping_add(maxDist)
        || loadedDictEnd != (*window).dictLimit
    {
        /* On reaching window size, dictionaries are invalidated.
             * For simplification, if window size is reached anywhere within next block,
             * the dictionary is invalidated for the full block.
             *
             * We also have to invalidate the dictionary if ZSTD_window_update() has detected
             * non-contiguous segments, which means that loadedDictEnd != window->dictLimit.
             * loadedDictEnd may be 0, if forceWindow is true, but in that case we never use
             * dictMatchState, so setting it to NULL is not a problem.
             */
        DEBUGLOG!(6, "invalidating dictionary for current block (distance > windowSize)");
        *loadedDictEndPtr = 0;
        *dictMatchStatePtr = std::ptr::null();
    } else {
        if *loadedDictEndPtr != 0 {
            DEBUGLOG!(6, "dictionary considered valid for current block");
        }
    };
}

#[inline]
pub unsafe fn ZSTD_window_init(mut window: *mut ZSTD_window_t) {
    libc::memset(
        window as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_window_t>(),
    );
    (*window).base = b" \0" as *const u8;
    (*window).dictBase = b" \0" as *const u8;
    const _: () = assert!(ZSTD_DUBT_UNSORTED_MARK < ZSTD_WINDOW_START_INDEX); /* Start above ZSTD_DUBT_UNSORTED_MARK */
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX; /* start from >0, so that 1st position is valid */
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX; /* it ensures first and later CCtx usages compress the same */
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize); /* see issue #1241 */
    (*window).nbOverflowCorrections = 0;
}

/**
 * ZSTD_window_update():
 * Updates the window by appending [src, src + srcSize) to the window.
 * If it is not contiguous, the current prefix becomes the extDict, and we
 * forget about the extDict. Handles overlap of the prefix and extDict.
 * Returns non-zero if the segment is contiguous.
 */
#[inline]
pub unsafe fn ZSTD_window_update(
    mut window: *mut ZSTD_window_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut forceNonContiguous: bool,
) -> bool {
    let ip = src as *const u8;
    let mut contiguous: bool = true;
    DEBUGLOG!(5, "ZSTD_window_update");
    if srcSize == 0 {
        return contiguous;
    }
    debug_assert!(!(*window).base.is_null());
    debug_assert!(!(*window).dictBase.is_null());

    /* Check if blocks follow each other */
    if ip != (*window).nextSrc || forceNonContiguous {
        /* not contiguous */
        let distanceFromBase = ((*window).nextSrc).offset_from((*window).base);
        DEBUGLOG!(5, "Non contiguous blocks, new segment starts at %u", (*window).dictLimit);
        (*window).lowLimit = (*window).dictLimit;
        u32::try_from(distanceFromBase).expect("should never overflow"); /* should never overflow */
        (*window).dictLimit = distanceFromBase as u32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-distanceFromBase);
        /* ms->nextToUpdate = window->dictLimit; */
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit) < HASH_READ_SIZE {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = false;
    }
    (*window).nextSrc = ip.add(srcSize);

    /* if input and dictionary overlap : reduce dictionary (area presumed modified by input) */
    if (ip.add(srcSize)
        > ((*window).dictBase).offset((*window).lowLimit as isize))
        & (ip < ((*window).dictBase).offset((*window).dictLimit as isize))
    {
        let highInputIdx = ip.offset(srcSize as isize).offset_from((*window).dictBase);
        let lowLimitMax = if highInputIdx > (*window).dictLimit as isize {
            (*window).dictLimit
        } else {
            highInputIdx as u32
        };
        (*window).lowLimit = lowLimitMax;
    }
    contiguous
}

/**
 * Returns the lowest allowed match index. It may either be in the ext-dict or the prefix.
 */
#[inline]
pub unsafe fn ZSTD_getLowestMatchIndex(
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut windowLog: std::ffi::c_uint,
) -> u32 {
    let maxDistance = 1_u32 << windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = (*ms).loadedDictEnd != 0;
    /* When using a dictionary the entire dictionary is valid if a single byte of the dictionary
     * is within the window. We invalidate the dictionary (and set loadedDictEnd to 0) when it isn't
     * valid for the entire block. So this check is sufficient to find the lowest valid match index.
     */
    let matchLowest = if isDictionary { lowestValid } else { withinWindow };
    matchLowest
}

/**
 * Returns the lowest allowed match index in the prefix.
 */
#[inline]
pub unsafe fn ZSTD_getLowestPrefixIndex(
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut windowLog: std::ffi::c_uint,
) -> u32 {
    let maxDistance = 1_u32 << windowLog;
    let lowestValid = (*ms).window.dictLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    /* When computing the lowest prefix index we need to take the dictionary into account to handle
     * the edge case where the dictionary and the source are contiguous in memory.
     */
    let isDictionary = (*ms).loadedDictEnd != 0;
    let matchLowest = if isDictionary { lowestValid } else { withinWindow };
    matchLowest
}

/* index_safety_check:
 * intentional underflow : ensure repIndex isn't overlapping dict + prefix
 * @return 1 if values are not overlapping,
 * 0 otherwise */
pub unsafe fn ZSTD_index_overlap_check(
    prefixLowestIndex: u32,
    repIndex: u32,
) -> bool {
    prefixLowestIndex
        .wrapping_sub(1)
        .wrapping_sub(repIndex) >= 3
}


/* debug functions */
// TODO
// #if (DEBUGLEVEL>=2)
//
// MEM_STATIC double ZSTD_fWeight(U32 rawStat)
// {
//     U32 const fp_accuracy = 8;
//     U32 const fp_multiplier = (1 << fp_accuracy);
//     U32 const newStat = rawStat + 1;
//     U32 const hb = ZSTD_highbit32(newStat);
//     U32 const BWeight = hb * fp_multiplier;
//     U32 const FWeight = (newStat << fp_accuracy) >> hb;
//     U32 const weight = BWeight + FWeight;
//     assert(hb + fp_accuracy < 31);
//     return (double)weight / fp_multiplier;
// }
//
// /* display a table content,
//  * listing each element, its frequency, and its predicted bit cost */
// MEM_STATIC void ZSTD_debugTable(const U32* table, U32 max)
// {
//     unsigned u, sum;
//     for (u=0, sum=0; u<=max; u++) sum += table[u];
//     DEBUGLOG(2, "total nb elts: %u", sum);
//     for (u=0; u<=max; u++) {
//         DEBUGLOG(2, "%2u: %5u  (%.2f)",
//                 u, table[u], ZSTD_fWeight(sum) - ZSTD_fWeight(table[u]) );
//     }
// }
//
// #endif

/* Short Cache */

/* Normally, zstd matchfinders follow this flow:
 *     1. Compute hash at ip
 *     2. Load index from hashTable[hash]
 *     3. Check if *ip == *(base + index)
 * In dictionary compression, loading *(base + index) is often an L2 or even L3 miss.
 *
 * Short cache is an optimization which allows us to avoid step 3 most of the time
 * when the data doesn't actually match. With short cache, the flow becomes:
 *     1. Compute (hash, currentTag) at ip. currentTag is an 8-bit independent hash at ip.
 *     2. Load (index, matchTag) from hashTable[hash]. See ZSTD_writeTaggedIndex to understand how this works.
 *     3. Only if currentTag == matchTag, check *ip == *(base + index). Otherwise, continue.
 *
 * Currently, short cache is only implemented in CDict hashtables. Thus, its use is limited to
 * dictMatchState matchfinders.
 */
pub const ZSTD_SHORT_CACHE_TAG_BITS: u32 = 8;
pub const ZSTD_SHORT_CACHE_TAG_MASK: usize = (1_usize << ZSTD_SHORT_CACHE_TAG_BITS) - 1;

/* Helper function for ZSTD_fillHashTable and ZSTD_fillDoubleHashTable.
 * Unpacks hashAndTag into (hash, tag), then packs (index, tag) into hashTable[hash]. */
#[inline]
pub unsafe fn ZSTD_writeTaggedIndex(
    hashTable: *mut u32,
    hashAndTag: usize,
    index: u32,
) {
    let hash = hashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
    let tag = (hashAndTag & ZSTD_SHORT_CACHE_TAG_MASK) as u32;
    *hashTable.offset(hash as isize) = index << ZSTD_SHORT_CACHE_TAG_BITS | tag;
}

/* Helper function for short cache matchfinders.
 * Unpacks tag1 and tag2 from lower bits of packedTag1 and packedTag2, then checks if the tags match. */
#[inline]
pub unsafe fn ZSTD_comparePackedTags(
    packedTag1: usize,
    packedTag2: usize,
) -> bool {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK) as u32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK) as u32;
    tag1 == tag2
}

/* ===============================================================
 * Shared internal declarations
 * These prototypes may be called from sources not in lib/compress
 * =============================================================== */

/* ZSTD_loadCEntropy() :
 * dict : must point at beginning of a valid zstd dictionary.
 * return : size of dictionary header (size of magic number + dict ID + entropy tables)
 * assumptions : magic number supposed already checked
 *               and dictSize >= 8 */
pub use crate::compress::zstd_compress::ZSTD_loadCEntropy;

pub use crate::compress::zstd_compress::ZSTD_reset_compressedBlockState;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_SequencePosition {
    pub idx: u32, /* Index in array of ZSTD_Sequence */
    pub posInSequence: u32, /* Position within sequence at idx */
    pub posInSrc: usize, /* Number of bytes given by sequences provided so far */
}

/* for benchmark */
pub use crate::compress::zstd_compress::ZSTD_convertBlockSequences;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSummary {
    pub nbSequences: usize,
    pub blockSize: usize,
    pub litSize: usize,
}

pub use crate::compress::zstd_compress::ZSTD_get1BlockSummary;

/* ==============================================================
 * Private declarations
 * These prototypes shall only be called from within lib/compress
 * ============================================================== */

/* ZSTD_getCParamsFromCCtxParams() :
 * cParams are built depending on compressionLevel, src size hints,
 * LDM and manually set compression parameters.
 * Note: srcSizeHint == 0 means 0!
 */
pub use crate::compress::zstd_compress::ZSTD_getCParamsFromCCtxParams;

/** ZSTD_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
pub use crate::compress::zstd_compress::ZSTD_initCStream_internal;

pub use crate::compress::zstd_compress::ZSTD_resetSeqStore;

/** ZSTD_getCParamsFromCDict() :
 *  as the name implies */
pub use crate::compress::zstd_compress::ZSTD_getCParamsFromCDict;

/* ZSTD_compressBegin_advanced_internal() :
 * Private use only. To be called from zstdmt_compress.c. */
pub use crate::compress::zstd_compress::ZSTD_compressBegin_advanced_internal;

/* ZSTD_compress_advanced_internal() :
 * Private use only. To be called from zstdmt_compress.c. */
pub use crate::compress::zstd_compress::ZSTD_compress_advanced_internal;


/* ZSTD_writeLastEmptyBlock() :
 * output an empty Block with end-of-frame mark to complete a frame
 * @return : size of data written into `dst` (== ZSTD_blockHeaderSize (defined in zstd_internal.h))
 *           or an error code if `dstCapacity` is too small (<ZSTD_blockHeaderSize)
 */
pub use crate::compress::zstd_compress::ZSTD_writeLastEmptyBlock;


/* ZSTD_referenceExternalSequences() :
 * Must be called before starting a compression operation.
 * seqs must parse a prefix of the source.
 * This cannot be used when long range matching is enabled.
 * Zstd will use these sequences, and pass the literals to a secondary block
 * compressor.
 * NOTE: seqs are not verified! Invalid sequences can cause out-of-bounds memory
 * access and data corruption.
 */
pub use crate::compress::zstd_compress::ZSTD_referenceExternalSequences;

/** ZSTD_cycleLog() :
 *  condition for correct operation : hashLog > 1 */
pub use crate::compress::zstd_compress::ZSTD_cycleLog;

/** ZSTD_CCtx_trace() :
 *  Trace the end of a compression call.
 */
pub use crate::compress::zstd_compress::ZSTD_CCtx_trace;

/* Returns 1 if an external sequence producer is registered, otherwise returns 0. */
#[inline]
pub unsafe fn ZSTD_hasExtSeqProd(
    mut params: *const ZSTD_CCtx_params,
) -> bool {
    ((*params).extSeqProdFunc).is_some()
}

/* ===============================================================
 * Deprecated definitions that are still used internally to avoid
 * deprecation warnings. These functions are exactly equivalent to
 * their public variants, but avoid the deprecation warnings.
 * =============================================================== */

#[deprecated]
pub use crate::compress::zstd_compress::ZSTD_compressBegin_usingCDict_deprecated;

#[deprecated]
pub use crate::compress::zstd_compress::ZSTD_compressContinue_public;

#[deprecated]
pub use crate::compress::zstd_compress::ZSTD_compressEnd_public;

#[deprecated]
pub use crate::compress::zstd_compress::ZSTD_compressBlock_deprecated;
