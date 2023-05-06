use ::libc;
extern "C" {
    pub type ZSTDMT_CCtx_s;
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    fn HUF_estimateCompressedSize(
        CTable: *const HUF_CElt,
        count: *const libc::c_uint,
        maxSymbolValue: libc::c_uint,
    ) -> libc::size_t;
    fn HUF_compress1X_usingCTable(
        dst: *mut libc::c_void,
        dstSize: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        CTable: *const HUF_CElt,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn HUF_compress4X_usingCTable(
        dst: *mut libc::c_void,
        dstSize: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        CTable: *const HUF_CElt,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn HIST_count_wksp(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        workSpace: *mut libc::c_void,
        workSpaceSize: libc::size_t,
    ) -> libc::size_t;
    fn HIST_countFast_wksp(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        workSpace: *mut libc::c_void,
        workSpaceSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_buildBlockEntropyStats(
        seqStorePtr: *const seqStore_t,
        prevEntropy: *const ZSTD_entropyCTables_t,
        nextEntropy: *mut ZSTD_entropyCTables_t,
        cctxParams: *const ZSTD_CCtx_params,
        entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
        workspace: *mut libc::c_void,
        wkspSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_encodeSequences(
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        CTable_MatchLength: *const FSE_CTable,
        mlCodeTable: *const u8,
        CTable_OffsetBits: *const FSE_CTable,
        ofCodeTable: *const u8,
        CTable_LitLength: *const FSE_CTable,
        llCodeTable: *const u8,
        sequences: *const seqDef,
        nbSeq: libc::size_t,
        longOffsets: libc::c_int,
        bmi2: libc::c_int,
    ) -> libc::size_t;
    fn ZSTD_fseBitCost(
        ctable: *const FSE_CTable,
        count: *const libc::c_uint,
        max: libc::c_uint,
    ) -> libc::size_t;
    fn ZSTD_crossEntropyCost(
        norm: *const libc::c_short,
        accuracyLog: libc::c_uint,
        count: *const libc::c_uint,
        max: libc::c_uint,
    ) -> libc::size_t;
    fn ZSTD_noCompressLiterals(
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_compressRleLiteralsBlock(
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: libc::c_int,
    pub bmi2: libc::c_int,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub simpleApiParams: ZSTD_CCtx_params,
    pub dictID: u32,
    pub dictContentSize: libc::size_t,
    pub workspace: ZSTD_cwksp,
    pub blockSize: libc::size_t,
    pub pledgedSrcSizePlusOne: libc::c_ulonglong,
    pub consumedSrcSize: libc::c_ulonglong,
    pub producedCSize: libc::c_ulonglong,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    pub pool: *mut ZSTD_threadPool,
    pub staticSize: libc::size_t,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: libc::c_int,
    pub initialized: libc::c_int,
    pub seqStore: seqStore_t,
    pub ldmState: ldmState_t,
    pub ldmSequences: *mut rawSeq,
    pub maxNbLdmSequences: libc::size_t,
    pub externSeqStore: rawSeqStore_t,
    pub blockState: ZSTD_blockState_t,
    pub entropyWorkspace: *mut u32,
    pub bufferedPolicy: ZSTD_buffered_policy_e,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: libc::size_t,
    pub inToCompress: libc::size_t,
    pub inBuffPos: libc::size_t,
    pub inBuffTarget: libc::size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: libc::size_t,
    pub outBuffContentSize: libc::size_t,
    pub outBuffFlushedSize: libc::size_t,
    pub streamStage: ZSTD_cStreamStage,
    pub frameEnded: u32,
    pub expectedInBuffer: ZSTD_inBuffer,
    pub stableIn_notConsumed: libc::size_t,
    pub expectedOutBufferSize: libc::size_t,
    pub localDict: ZSTD_localDict,
    pub cdict: *const ZSTD_CDict,
    pub prefixDict: ZSTD_prefixDict,
    pub mtctx: *mut ZSTDMT_CCtx,
    pub traceCtx: ZSTD_TraceCtx,
    pub blockSplitCtx: ZSTD_blockSplitCtx,
    pub externalMatchCtx: ZSTD_externalMatchCtx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_externalMatchCtx {
    pub mState: *mut libc::c_void,
    pub mFinder: Option<ZSTD_sequenceProducer_F>,
    pub seqBuffer: *mut ZSTD_Sequence,
    pub seqBufferCapacity: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Sequence {
    pub offset: libc::c_uint,
    pub litLength: libc::c_uint,
    pub matchLength: libc::c_uint,
    pub rep: libc::c_uint,
}
pub type ZSTD_sequenceProducer_F = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut ZSTD_Sequence,
    libc::size_t,
    *const libc::c_void,
    libc::size_t,
    *const libc::c_void,
    libc::size_t,
    libc::c_int,
    libc::size_t,
) -> libc::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: seqStore_t,
    pub firstHalfSeqStore: seqStore_t,
    pub secondHalfSeqStore: seqStore_t,
    pub currSeqStore: seqStore_t,
    pub nextSeqStore: seqStore_t,
    pub partitions: [u32; 196],
    pub entropyMetadata: ZSTD_entropyCTablesMetadata_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTablesMetadata_t {
    pub hufMetadata: ZSTD_hufCTablesMetadata_t,
    pub fseMetadata: ZSTD_fseCTablesMetadata_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTablesMetadata_t {
    pub llType: symbolEncodingType_e,
    pub ofType: symbolEncodingType_e,
    pub mlType: symbolEncodingType_e,
    pub fseTablesBuffer: [u8; 133],
    pub fseTablesSize: libc::size_t,
    pub lastCountSize: libc::size_t,
}
pub type symbolEncodingType_e = libc::c_uint;
pub const set_repeat: symbolEncodingType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub const set_rle: symbolEncodingType_e = 1;
pub const set_basic: symbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTablesMetadata_t {
    pub hType: symbolEncodingType_e,
    pub hufDesBuffer: [u8; 128],
    pub hufDesSize: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut u8,
    pub lit: *mut u8,
    pub llCode: *mut u8,
    pub mlCode: *mut u8,
    pub ofCode: *mut u8,
    pub maxNbSeq: libc::size_t,
    pub maxNbLit: libc::size_t,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: u32,
}
pub type ZSTD_longLengthType_e = libc::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type seqDef = seqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqDef_s {
    pub offBase: u32,
    pub litLength: u16,
    pub mlBase: u16,
}
pub type ZSTD_TraceCtx = libc::c_ulonglong;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const libc::c_void,
    pub dictSize: libc::size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_dictContentType_e = libc::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_CDict = ZSTD_CDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut libc::c_void,
    pub dict: *const libc::c_void,
    pub dictSize: libc::size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: libc::size_t,
    pub pos: libc::size_t,
}
pub type ZSTD_cStreamStage = libc::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = libc::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_matchState_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_matchState_t {
    pub window: ZSTD_window_t,
    pub loadedDictEnd: u32,
    pub nextToUpdate: u32,
    pub hashLog3: u32,
    pub rowHashLog: u32,
    pub tagTable: *mut u8,
    pub hashCache: [u32; 8],
    pub hashSalt: u64,
    pub hashSaltEntropy: u32,
    pub hashTable: *mut u32,
    pub hashTable3: *mut u32,
    pub chainTable: *mut u32,
    pub forceNonContiguous: u32,
    pub dedicatedDictSearch: libc::c_int,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_matchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const rawSeqStore_t,
    pub prefetchCDictTables: libc::c_int,
    pub lazySkipping: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: libc::size_t,
    pub posInSequence: libc::size_t,
    pub size: libc::size_t,
    pub capacity: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: u32,
    pub litLength: u32,
    pub matchLength: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub minMatch: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optState_t {
    pub litFreq: *mut libc::c_uint,
    pub litLengthFreq: *mut libc::c_uint,
    pub matchLengthFreq: *mut libc::c_uint,
    pub offCodeFreq: *mut libc::c_uint,
    pub matchTable: *mut ZSTD_match_t,
    pub priceTable: *mut ZSTD_optimal_t,
    pub litSum: u32,
    pub litLengthSum: u32,
    pub matchLengthSum: u32,
    pub offCodeSum: u32,
    pub litSumBasePrice: u32,
    pub litLengthSumBasePrice: u32,
    pub matchLengthSumBasePrice: u32,
    pub offCodeSumBasePrice: u32,
    pub priceType: ZSTD_OptPrice_e,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
}
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
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
pub type FSE_repeat = libc::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = libc::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = libc::size_t;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: u32,
    pub mlen: u32,
    pub litlen: u32,
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: u32,
    pub len: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: u32,
    pub lowLimit: u32,
    pub nbOverflowCorrections: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub loadedDictEnd: u32,
    pub bucketOffsets: *mut u8,
    pub splitIndices: [libc::size_t; 64],
    pub matchCandidates: [ldmMatchCandidate_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmMatchCandidate_t {
    pub split: *const u8,
    pub hash: u32,
    pub checksum: u32,
    pub bucket: *mut ldmEntry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: u32,
    pub checksum: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: libc::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: libc::size_t,
    pub maxSequences: libc::size_t,
}
pub type ZSTD_threadPool = POOL_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(*mut libc::c_void, libc::size_t) -> *mut libc::c_void>;
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut libc::c_void,
    pub workspaceEnd: *mut libc::c_void,
    pub objectEnd: *mut libc::c_void,
    pub tableEnd: *mut libc::c_void,
    pub tableValidEnd: *mut libc::c_void,
    pub allocStart: *mut libc::c_void,
    pub initOnceStart: *mut libc::c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: libc::c_int,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}
pub type ZSTD_cwksp_static_alloc_e = libc::c_uint;
pub const ZSTD_cwksp_static_alloc: ZSTD_cwksp_static_alloc_e = 1;
pub const ZSTD_cwksp_dynamic_alloc: ZSTD_cwksp_static_alloc_e = 0;
pub type ZSTD_cwksp_alloc_phase_e = libc::c_uint;
pub const ZSTD_cwksp_alloc_buffers: ZSTD_cwksp_alloc_phase_e = 3;
pub const ZSTD_cwksp_alloc_aligned: ZSTD_cwksp_alloc_phase_e = 2;
pub const ZSTD_cwksp_alloc_aligned_init_once: ZSTD_cwksp_alloc_phase_e = 1;
pub const ZSTD_cwksp_alloc_objects: ZSTD_cwksp_alloc_phase_e = 0;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_params_s {
    pub format: ZSTD_format_e,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
    pub compressionLevel: libc::c_int,
    pub forceWindow: libc::c_int,
    pub targetCBlockSize: libc::size_t,
    pub srcSizeHint: libc::c_int,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
    pub nbWorkers: libc::c_int,
    pub jobSize: libc::size_t,
    pub overlapLog: libc::c_int,
    pub rsyncable: libc::c_int,
    pub ldmParams: ldmParams_t,
    pub enableDedicatedDictSearch: libc::c_int,
    pub inBufferMode: ZSTD_bufferMode_e,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub blockDelimiters: ZSTD_sequenceFormat_e,
    pub validateSequences: libc::c_int,
    pub useBlockSplitter: ZSTD_paramSwitch_e,
    pub useRowMatchFinder: ZSTD_paramSwitch_e,
    pub deterministicRefPrefix: libc::c_int,
    pub customMem: ZSTD_customMem,
    pub prefetchCDictTables: ZSTD_paramSwitch_e,
    pub enableMatchFinderFallback: libc::c_int,
    pub useSequenceProducer: libc::c_int,
    pub maxBlockSize: libc::size_t,
    pub searchForExternalRepcodes: ZSTD_paramSwitch_e,
}
pub type ZSTD_sequenceFormat_e = libc::c_uint;
pub const ZSTD_sf_explicitBlockDelimiters: ZSTD_sequenceFormat_e = 1;
pub const ZSTD_sf_noBlockDelimiters: ZSTD_sequenceFormat_e = 0;
pub type ZSTD_bufferMode_e = libc::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: ZSTD_paramSwitch_e,
    pub hashLog: u32,
    pub bucketSizeLog: u32,
    pub minMatchLength: u32,
    pub hashRateLog: u32,
    pub windowLog: u32,
}
pub type ZSTD_dictAttachPref_e = libc::c_uint;
pub const ZSTD_dictForceLoad: ZSTD_dictAttachPref_e = 3;
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = 2;
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_int,
    pub checksumFlag: libc::c_int,
    pub noDictIDFlag: libc::c_int,
}
pub type ZSTD_format_e = libc::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub type ZSTD_compressionStage_e = libc::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type repcodes_t = repcodes_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_sequenceLength {
    pub litLength: u32,
    pub matchLength: u32,
}
pub const bt_raw: C2RustUnnamed_1 = 0;
pub type unalign16 = u16;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
pub const bt_compressed: C2RustUnnamed_1 = 2;
pub type unalign32 = u32;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed = 10;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const bt_reserved: C2RustUnnamed_1 = 3;
pub const bt_rle: C2RustUnnamed_1 = 1;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const MaxOff: libc::c_int = 31 as libc::c_int;
static mut OF_defaultNorm: [i16; 29] = [
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
];
pub const OF_DEFAULTNORMLOG: libc::c_int = 5 as libc::c_int;
static mut OF_defaultNormLog: u32 = OF_DEFAULTNORMLOG as u32;
pub const DefaultMaxOff: libc::c_int = 28 as libc::c_int;
static mut LL_bits: [u8; 36] = [
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    2 as libc::c_int as u8,
    2 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    4 as libc::c_int as u8,
    6 as libc::c_int as u8,
    7 as libc::c_int as u8,
    8 as libc::c_int as u8,
    9 as libc::c_int as u8,
    10 as libc::c_int as u8,
    11 as libc::c_int as u8,
    12 as libc::c_int as u8,
    13 as libc::c_int as u8,
    14 as libc::c_int as u8,
    15 as libc::c_int as u8,
    16 as libc::c_int as u8,
];
static mut LL_defaultNorm: [i16; 36] = [
    4 as libc::c_int as i16,
    3 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    3 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
];
pub const LL_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
static mut LL_defaultNormLog: u32 = LL_DEFAULTNORMLOG as u32;
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const ZSTD_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
static mut ML_bits: [u8; 53] = [
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    2 as libc::c_int as u8,
    2 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    5 as libc::c_int as u8,
    7 as libc::c_int as u8,
    8 as libc::c_int as u8,
    9 as libc::c_int as u8,
    10 as libc::c_int as u8,
    11 as libc::c_int as u8,
    12 as libc::c_int as u8,
    13 as libc::c_int as u8,
    14 as libc::c_int as u8,
    15 as libc::c_int as u8,
    16 as libc::c_int as u8,
];
static mut ML_defaultNorm: [i16; 53] = [
    1 as libc::c_int as i16,
    4 as libc::c_int as i16,
    3 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
];
static mut ML_defaultNormLog: u32 = ML_DEFAULTNORMLOG as u32;
pub const ML_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
pub const MaxML: libc::c_int = 52 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
pub const LONGNBSEQ: libc::c_int = 0x7f00 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>() == 4) as libc::c_int as libc::c_uint;
}
pub const STREAM_ACCUMULATOR_MIN_32: libc::c_int = 25 as libc::c_int;
pub const STREAM_ACCUMULATOR_MIN_64: libc::c_int = 57 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const libc::c_char, mut args: ...) {}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: u16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut u8;
        *p.offset(0) = val as u8;
        *p.offset(1 as libc::c_int as isize) = (val as libc::c_int >> 8 as libc::c_int) as u8;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut libc::c_void, mut val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8).offset(2) = (val >> 16 as libc::c_int) as u8;
}
pub const ZSTD_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
static mut ZSTD_blockHeaderSize: libc::size_t = ZSTD_BLOCKHEADERSIZE as libc::size_t;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
pub const MINMATCH: libc::c_int = 3 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_getSequenceLength(
    mut seqStore: *const seqStore_t,
    mut seq: *const seqDef,
) -> ZSTD_sequenceLength {
    let mut seqLen = ZSTD_sequenceLength {
        litLength: 0,
        matchLength: 0,
    };
    seqLen.litLength = (*seq).litLength as u32;
    seqLen.matchLength = ((*seq).mlBase as libc::c_int + MINMATCH) as u32;
    if (*seqStore).longLengthPos
        == seq.offset_from((*seqStore).sequencesStart) as libc::c_long as u32
    {
        if (*seqStore).longLengthType as libc::c_uint
            == ZSTD_llt_literalLength as libc::c_int as libc::c_uint
        {
            seqLen.litLength = (seqLen.litLength as libc::c_uint)
                .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
        }
        if (*seqStore).longLengthType as libc::c_uint
            == ZSTD_llt_matchLength as libc::c_int as libc::c_uint
        {
            seqLen.matchLength = (seqLen.matchLength as libc::c_uint)
                .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
        }
    }
    return seqLen;
}
pub const HUF_WORKSPACE_SIZE: libc::c_int = ((8) << 10 as libc::c_int) + 512;
#[inline]
unsafe extern "C" fn ZSTD_noCompressBlock(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut lastBlock: u32,
) -> libc::size_t {
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_raw as libc::c_int as u32) << 1 as libc::c_int)
        .wrapping_add((srcSize << 3 as libc::c_int) as u32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    MEM_writeLE24(dst, cBlockHeader24);
    libc::memcpy(
        (dst as *mut u8).offset(ZSTD_blockHeaderSize as isize) as *mut libc::c_void,
        src,
        srcSize as libc::size_t,
    );
    return ZSTD_blockHeaderSize.wrapping_add(srcSize);
}
#[inline]
unsafe extern "C" fn ZSTD_updateRep(mut rep: *mut u32, offBase: u32, ll0: u32) {
    if offBase > ZSTD_REP_NUM as libc::c_uint {
        *rep.offset(2) = *rep.offset(1);
        *rep.offset(1) = *rep.offset(0);
        debug_assert!(offBase > 3);
        *rep.offset(0 as libc::c_int as isize) = offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_uint);
    } else {
        debug_assert!(1 as libc::c_int as libc::c_uint <= offBase && offBase <= 3);
        let repCode = offBase.wrapping_sub(1).wrapping_add(ll0);
        if repCode > 0 {
            let currentOffset = if repCode == ZSTD_REP_NUM as libc::c_uint {
                (*rep.offset(0)).wrapping_sub(1)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep.offset(2 as libc::c_int as isize) = if repCode >= 2 {
                *rep.offset(1)
            } else {
                *rep.offset(2)
            };
            *rep.offset(1 as libc::c_int as isize) = *rep.offset(0);
            *rep.offset(0) = currentOffset;
        }
    };
}
unsafe extern "C" fn ZSTD_compressSubBlock_literal(
    mut hufTable: *const HUF_CElt,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut literals: *const u8,
    mut litSize: libc::size_t,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    bmi2: libc::c_int,
    mut writeEntropy: libc::c_int,
    mut entropyWritten: *mut libc::c_int,
) -> libc::size_t {
    let header = (if writeEntropy != 0 {
        200 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::size_t;
    let lhSize = (3 as libc::c_int
        + (litSize
            >= ((1 as libc::c_int * ((1) << 10 as libc::c_int)) as libc::c_ulong)
                .wrapping_sub(header)) as libc::c_int
        + (litSize
            >= ((16 as libc::c_int * ((1) << 10 as libc::c_int)) as libc::c_ulong)
                .wrapping_sub(header)) as libc::c_int) as libc::size_t;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart.offset(lhSize as isize);
    let singleStream = (lhSize == 3) as libc::c_int as u32;
    let mut hType = (if writeEntropy != 0 {
        (*hufMetadata).hType as libc::c_uint
    } else {
        set_repeat as libc::c_int as libc::c_uint
    }) as symbolEncodingType_e;
    let mut cLitSize = 0 as libc::c_int as libc::size_t;
    *entropyWritten = 0 as libc::c_int;
    if litSize == 0
        || (*hufMetadata).hType as libc::c_uint == set_basic as libc::c_int as libc::c_uint
    {
        return ZSTD_noCompressLiterals(dst, dstSize, literals as *const libc::c_void, litSize);
    } else {
        if (*hufMetadata).hType as libc::c_uint == set_rle as libc::c_int as libc::c_uint {
            return ZSTD_compressRleLiteralsBlock(
                dst,
                dstSize,
                literals as *const libc::c_void,
                litSize,
            );
        }
    }
    debug_assert!(litSize > 0);
    debug_assert!(
        (*hufMetadata).hType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
            || (*hufMetadata).hType as libc::c_uint == set_repeat as libc::c_int as libc::c_uint
    );
    if writeEntropy != 0
        && (*hufMetadata).hType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
    {
        libc::memcpy(
            op as *mut libc::c_void,
            ((*hufMetadata).hufDesBuffer).as_ptr() as *const libc::c_void,
            (*hufMetadata).hufDesSize as libc::size_t,
        );
        op = op.offset((*hufMetadata).hufDesSize as isize);
        cLitSize = (cLitSize as libc::c_ulong).wrapping_add((*hufMetadata).hufDesSize);
    }
    let flags = if bmi2 != 0 {
        HUF_flags_bmi2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let cSize = if singleStream != 0 {
        HUF_compress1X_usingCTable(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as libc::size_t,
            literals as *const libc::c_void,
            litSize,
            hufTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as libc::size_t,
            literals as *const libc::c_void,
            litSize,
            hufTable,
            flags,
        )
    };
    op = op.offset(cSize as isize);
    cLitSize = (cLitSize as libc::c_ulong).wrapping_add(cSize);
    if cSize == 0 || ERR_isError(cSize) != 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    if writeEntropy == 0 && cLitSize >= litSize {
        return ZSTD_noCompressLiterals(dst, dstSize, literals as *const libc::c_void, litSize);
    }
    if lhSize
        < (3 as libc::c_int
            + (cLitSize >= (1 as libc::c_int * ((1) << 10 as libc::c_int)) as libc::c_ulong)
                as libc::c_int
            + (cLitSize >= (16 as libc::c_int * ((1) << 10 as libc::c_int)) as libc::c_ulong)
                as libc::c_int) as libc::size_t
    {
        debug_assert!(cLitSize > litSize);
        return ZSTD_noCompressLiterals(dst, dstSize, literals as *const libc::c_void, litSize);
    }
    match lhSize {
        3 => {
            let lhc = (hType as libc::c_uint)
                .wrapping_add(
                    (((singleStream == 0) as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                )
                .wrapping_add((litSize as u32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as u32) << 14 as libc::c_int);
            MEM_writeLE24(ostart as *mut libc::c_void, lhc);
        }
        4 => {
            let lhc_0 = (hType as libc::c_uint)
                .wrapping_add(((2) << 2 as libc::c_int) as libc::c_uint)
                .wrapping_add((litSize as u32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as u32) << 18 as libc::c_int);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_0);
        }
        5 => {
            let lhc_1 = (hType as libc::c_uint)
                .wrapping_add(((3) << 2 as libc::c_int) as libc::c_uint)
                .wrapping_add((litSize as u32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as u32) << 22 as libc::c_int);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_1);
            *ostart.offset(4 as libc::c_int as isize) = (cLitSize >> 10 as libc::c_int) as u8;
        }
        _ => {
            debug_assert!(false);
        }
    }
    *entropyWritten = 1 as libc::c_int;
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_seqDecompressedSize(
    mut seqStore: *const seqStore_t,
    mut sequences: *const seqDef,
    mut nbSeq: libc::size_t,
    mut litSize: libc::size_t,
    mut lastSequence: libc::c_int,
) -> libc::size_t {
    let sstart = sequences;
    let send = sequences.offset(nbSeq as isize);
    let mut sp = sstart;
    let mut matchLengthSum = 0 as libc::c_int as libc::size_t;
    let mut litLengthSum = 0 as libc::c_int as libc::size_t;
    while send.offset_from(sp) as libc::c_long > 0 {
        let seqLen = ZSTD_getSequenceLength(seqStore, sp);
        litLengthSum =
            (litLengthSum as libc::c_ulong).wrapping_add(seqLen.litLength as libc::c_ulong);
        matchLengthSum =
            (matchLengthSum as libc::c_ulong).wrapping_add(seqLen.matchLength as libc::c_ulong);
        sp = sp.offset(1);
    }
    debug_assert!(litLengthSum <= litSize);
    if lastSequence == 0 {
        debug_assert!(litLengthSum == litSize);
    }
    return matchLengthSum.wrapping_add(litSize);
}
unsafe extern "C" fn ZSTD_compressSubBlock_sequences(
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut sequences: *const seqDef,
    mut nbSeq: libc::size_t,
    mut llCode: *const u8,
    mut mlCode: *const u8,
    mut ofCode: *const u8,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    bmi2: libc::c_int,
    mut writeEntropy: libc::c_int,
    mut entropyWritten: *mut libc::c_int,
) -> libc::size_t {
    let longOffsets = ((*cctxParams).cParams.windowLog
        > (if MEM_32bits() != 0 {
            STREAM_ACCUMULATOR_MIN_32
        } else {
            STREAM_ACCUMULATOR_MIN_64
        }) as u32) as libc::c_int;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut seqHead = 0 as *mut u8;
    *entropyWritten = 0 as libc::c_int;
    if (oend.offset_from(op) as libc::c_long) < (3 as libc::c_int + 1) as libc::c_long {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if nbSeq < 0x7f as libc::c_int as libc::c_ulong {
        let fresh0 = op;
        op = op.offset(1);
        *fresh0 = nbSeq as u8;
    } else if nbSeq < LONGNBSEQ as libc::c_ulong {
        *op.offset(0 as libc::c_int as isize) =
            (nbSeq >> 8 as libc::c_int).wrapping_add(0x80 as libc::c_int as libc::c_ulong) as u8;
        *op.offset(1) = nbSeq as u8;
        op = op.offset(2);
    } else {
        *op.offset(0) = 0xff as libc::c_int as u8;
        MEM_writeLE16(
            op.offset(1) as *mut libc::c_void,
            nbSeq.wrapping_sub(LONGNBSEQ as libc::c_ulong) as u16,
        );
        op = op.offset(3);
    }
    if nbSeq == 0 {
        return op.offset_from(ostart) as libc::c_long as libc::size_t;
    }
    let fresh1 = op;
    op = op.offset(1);
    seqHead = fresh1;
    if writeEntropy != 0 {
        let LLtype = (*fseMetadata).llType as u32;
        let Offtype = (*fseMetadata).ofType as u32;
        let MLtype = (*fseMetadata).mlType as u32;
        *seqHead = (LLtype << 6 as libc::c_int)
            .wrapping_add(Offtype << 4 as libc::c_int)
            .wrapping_add(MLtype << 2 as libc::c_int) as u8;
        libc::memcpy(
            op as *mut libc::c_void,
            ((*fseMetadata).fseTablesBuffer).as_ptr() as *const libc::c_void,
            (*fseMetadata).fseTablesSize as libc::size_t,
        );
        op = op.offset((*fseMetadata).fseTablesSize as isize);
    } else {
        let repeat = set_repeat as libc::c_int as u32;
        *seqHead = (repeat << 6 as libc::c_int)
            .wrapping_add(repeat << 4 as libc::c_int)
            .wrapping_add(repeat << 2 as libc::c_int) as u8;
    }
    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        ((*fseTables).matchlengthCTable).as_ptr(),
        mlCode,
        ((*fseTables).offcodeCTable).as_ptr(),
        ofCode,
        ((*fseTables).litlengthCTable).as_ptr(),
        llCode,
        sequences,
        nbSeq,
        longOffsets,
        bmi2,
    );
    let err_code = bitstreamSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    op = op.offset(bitstreamSize as isize);
    if writeEntropy != 0
        && (*fseMetadata).lastCountSize != 0
        && ((*fseMetadata).lastCountSize).wrapping_add(bitstreamSize) < 4
    {
        debug_assert!(((*fseMetadata).lastCountSize).wrapping_add(bitstreamSize) == 3);
        return 0 as libc::c_int as libc::size_t;
    }
    if (op.offset_from(seqHead) as libc::c_long) < 4 {
        return 0 as libc::c_int as libc::size_t;
    }
    *entropyWritten = 1 as libc::c_int;
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_compressSubBlock(
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut sequences: *const seqDef,
    mut nbSeq: libc::size_t,
    mut literals: *const u8,
    mut litSize: libc::size_t,
    mut llCode: *const u8,
    mut mlCode: *const u8,
    mut ofCode: *const u8,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    bmi2: libc::c_int,
    mut writeLitEntropy: libc::c_int,
    mut writeSeqEntropy: libc::c_int,
    mut litEntropyWritten: *mut libc::c_int,
    mut seqEntropyWritten: *mut libc::c_int,
    mut lastBlock: u32,
) -> libc::size_t {
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart.offset(ZSTD_blockHeaderSize as isize);
    let mut cLitSize = ZSTD_compressSubBlock_literal(
        ((*entropy).huf.CTable).as_ptr(),
        &(*entropyMetadata).hufMetadata,
        literals,
        litSize,
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        bmi2,
        writeLitEntropy,
        litEntropyWritten,
    );
    let err_code = cLitSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if cLitSize == 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(cLitSize as isize);
    let mut cSeqSize = ZSTD_compressSubBlock_sequences(
        &(*entropy).fse,
        &(*entropyMetadata).fseMetadata,
        sequences,
        nbSeq,
        llCode,
        mlCode,
        ofCode,
        cctxParams,
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        bmi2,
        writeSeqEntropy,
        seqEntropyWritten,
    );
    let err_code_0 = cSeqSize;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if cSeqSize == 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(cSeqSize as isize);
    let mut cSize = (op.offset_from(ostart) as libc::c_long as libc::c_ulong)
        .wrapping_sub(ZSTD_blockHeaderSize);
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_compressed as libc::c_int as u32) << 1 as libc::c_int)
        .wrapping_add((cSize << 3 as libc::c_int) as u32);
    MEM_writeLE24(ostart as *mut libc::c_void, cBlockHeader24);
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_literal(
    mut literals: *const u8,
    mut litSize: libc::size_t,
    mut huf: *const ZSTD_hufCTables_t,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut writeEntropy: libc::c_int,
) -> libc::size_t {
    let countWksp = workspace as *mut libc::c_uint;
    let mut maxSymbolValue = 255 as libc::c_int as libc::c_uint;
    let mut literalSectionHeaderSize = 3 as libc::c_int as libc::size_t;
    if (*hufMetadata).hType as libc::c_uint == set_basic as libc::c_int as libc::c_uint {
        return litSize;
    } else {
        if (*hufMetadata).hType as libc::c_uint == set_rle as libc::c_int as libc::c_uint {
            return 1 as libc::c_int as libc::size_t;
        } else {
            if (*hufMetadata).hType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
                || (*hufMetadata).hType as libc::c_uint == set_repeat as libc::c_int as libc::c_uint
            {
                let largest = HIST_count_wksp(
                    countWksp,
                    &mut maxSymbolValue,
                    literals as *const libc::c_void,
                    litSize,
                    workspace,
                    wkspSize,
                );
                if ERR_isError(largest) != 0 {
                    return litSize;
                }
                let mut cLitSizeEstimate =
                    HUF_estimateCompressedSize(((*huf).CTable).as_ptr(), countWksp, maxSymbolValue);
                if writeEntropy != 0 {
                    cLitSizeEstimate =
                        (cLitSizeEstimate as libc::c_ulong).wrapping_add((*hufMetadata).hufDesSize);
                }
                return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
            }
        }
    }
    debug_assert!(false);
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_symbolType(
    mut type_0: symbolEncodingType_e,
    mut codeTable: *const u8,
    mut maxCode: libc::c_uint,
    mut nbSeq: libc::size_t,
    mut fseCTable: *const FSE_CTable,
    mut additionalBits: *const u8,
    mut defaultNorm: *const libc::c_short,
    mut defaultNormLog: u32,
    mut defaultMax: u32,
    mut workspace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
) -> libc::size_t {
    let countWksp = workspace as *mut libc::c_uint;
    let mut ctp = codeTable;
    let ctStart = ctp;
    let ctEnd = ctStart.offset(nbSeq as isize);
    let mut cSymbolTypeSizeEstimateInBits = 0 as libc::c_int as libc::size_t;
    let mut max = maxCode;
    HIST_countFast_wksp(
        countWksp,
        &mut max,
        codeTable as *const libc::c_void,
        nbSeq,
        workspace,
        wkspSize,
    );
    if type_0 as libc::c_uint == set_basic as libc::c_int as libc::c_uint {
        debug_assert!(max <= defaultMax);
        cSymbolTypeSizeEstimateInBits = if max <= defaultMax {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, countWksp, max)
        } else {
            -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t
        };
    } else if type_0 as libc::c_uint == set_rle as libc::c_int as libc::c_uint {
        cSymbolTypeSizeEstimateInBits = 0 as libc::c_int as libc::size_t;
    } else if type_0 as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == set_repeat as libc::c_int as libc::c_uint
    {
        cSymbolTypeSizeEstimateInBits = ZSTD_fseBitCost(fseCTable, countWksp, max);
    }
    if ERR_isError(cSymbolTypeSizeEstimateInBits) != 0 {
        return nbSeq.wrapping_mul(10);
    }
    while ctp < ctEnd {
        if !additionalBits.is_null() {
            cSymbolTypeSizeEstimateInBits = (cSymbolTypeSizeEstimateInBits as libc::c_ulong)
                .wrapping_add(*additionalBits.offset(*ctp as isize) as libc::c_ulong);
        } else {
            cSymbolTypeSizeEstimateInBits = (cSymbolTypeSizeEstimateInBits as libc::c_ulong)
                .wrapping_add(*ctp as libc::c_ulong);
        }
        ctp = ctp.offset(1);
    }
    return cSymbolTypeSizeEstimateInBits.wrapping_div(8);
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_sequences(
    mut ofCodeTable: *const u8,
    mut llCodeTable: *const u8,
    mut mlCodeTable: *const u8,
    mut nbSeq: libc::size_t,
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut writeEntropy: libc::c_int,
) -> libc::size_t {
    let sequencesSectionHeaderSize = 3 as libc::c_int as libc::size_t;
    let mut cSeqSizeEstimate = 0 as libc::c_int as libc::size_t;
    if nbSeq == 0 {
        return sequencesSectionHeaderSize;
    }
    cSeqSizeEstimate =
        (cSeqSizeEstimate as libc::c_ulong).wrapping_add(ZSTD_estimateSubBlockSize_symbolType(
            (*fseMetadata).ofType,
            ofCodeTable,
            MaxOff as libc::c_uint,
            nbSeq,
            ((*fseTables).offcodeCTable).as_ptr(),
            NULL as *const u8,
            OF_defaultNorm.as_ptr(),
            OF_defaultNormLog,
            DefaultMaxOff as u32,
            workspace,
            wkspSize,
        ));
    cSeqSizeEstimate =
        (cSeqSizeEstimate as libc::c_ulong).wrapping_add(ZSTD_estimateSubBlockSize_symbolType(
            (*fseMetadata).llType,
            llCodeTable,
            MaxLL as libc::c_uint,
            nbSeq,
            ((*fseTables).litlengthCTable).as_ptr(),
            LL_bits.as_ptr(),
            LL_defaultNorm.as_ptr(),
            LL_defaultNormLog,
            MaxLL as u32,
            workspace,
            wkspSize,
        ));
    cSeqSizeEstimate =
        (cSeqSizeEstimate as libc::c_ulong).wrapping_add(ZSTD_estimateSubBlockSize_symbolType(
            (*fseMetadata).mlType,
            mlCodeTable,
            MaxML as libc::c_uint,
            nbSeq,
            ((*fseTables).matchlengthCTable).as_ptr(),
            ML_bits.as_ptr(),
            ML_defaultNorm.as_ptr(),
            ML_defaultNormLog,
            MaxML as u32,
            workspace,
            wkspSize,
        ));
    if writeEntropy != 0 {
        cSeqSizeEstimate =
            (cSeqSizeEstimate as libc::c_ulong).wrapping_add((*fseMetadata).fseTablesSize);
    }
    return cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize);
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize(
    mut literals: *const u8,
    mut litSize: libc::size_t,
    mut ofCodeTable: *const u8,
    mut llCodeTable: *const u8,
    mut mlCodeTable: *const u8,
    mut nbSeq: libc::size_t,
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut writeLitEntropy: libc::c_int,
    mut writeSeqEntropy: libc::c_int,
) -> libc::size_t {
    let mut cSizeEstimate = 0 as libc::c_int as libc::size_t;
    cSizeEstimate =
        (cSizeEstimate as libc::c_ulong).wrapping_add(ZSTD_estimateSubBlockSize_literal(
            literals,
            litSize,
            &(*entropy).huf,
            &(*entropyMetadata).hufMetadata,
            workspace,
            wkspSize,
            writeLitEntropy,
        ));
    cSizeEstimate =
        (cSizeEstimate as libc::c_ulong).wrapping_add(ZSTD_estimateSubBlockSize_sequences(
            ofCodeTable,
            llCodeTable,
            mlCodeTable,
            nbSeq,
            &(*entropy).fse,
            &(*entropyMetadata).fseMetadata,
            workspace,
            wkspSize,
            writeSeqEntropy,
        ));
    return cSizeEstimate.wrapping_add(ZSTD_blockHeaderSize);
}
unsafe extern "C" fn ZSTD_needSequenceEntropyTables(
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
) -> libc::c_int {
    if (*fseMetadata).llType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
        || (*fseMetadata).llType as libc::c_uint == set_rle as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if (*fseMetadata).mlType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
        || (*fseMetadata).mlType as libc::c_uint == set_rle as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if (*fseMetadata).ofType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
        || (*fseMetadata).ofType as libc::c_uint == set_rle as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ZSTD_compressSubBlock_multi(
    mut seqStorePtr: *const seqStore_t,
    mut prevCBlock: *const ZSTD_compressedBlockState_t,
    mut nextCBlock: *mut ZSTD_compressedBlockState_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    bmi2: libc::c_int,
    mut lastBlock: u32,
    mut workspace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
) -> libc::size_t {
    let sstart: *const seqDef = (*seqStorePtr).sequencesStart;
    let send: *const seqDef = (*seqStorePtr).sequences;
    let mut sp = sstart;
    let lstart: *const u8 = (*seqStorePtr).litStart;
    let lend: *const u8 = (*seqStorePtr).lit;
    let mut lp = lstart;
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut llCodePtr: *const u8 = (*seqStorePtr).llCode;
    let mut mlCodePtr: *const u8 = (*seqStorePtr).mlCode;
    let mut ofCodePtr: *const u8 = (*seqStorePtr).ofCode;
    let mut targetCBlockSize = (*cctxParams).targetCBlockSize;
    let mut litSize: libc::size_t = 0;
    let mut seqCount: libc::size_t = 0;
    let mut writeLitEntropy = ((*entropyMetadata).hufMetadata.hType as libc::c_uint
        == set_compressed as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut writeSeqEntropy = 1 as libc::c_int;
    let mut lastSequence = 0 as libc::c_int;
    litSize = 0 as libc::c_int as libc::size_t;
    seqCount = 0 as libc::c_int as libc::size_t;
    loop {
        let mut cBlockSizeEstimate = 0 as libc::c_int as libc::size_t;
        if sstart == send {
            lastSequence = 1 as libc::c_int;
        } else {
            let sequence = sp.offset(seqCount as isize);
            lastSequence = (sequence == send.offset(-(1))) as libc::c_int;
            litSize = (litSize as libc::c_ulong).wrapping_add(
                (ZSTD_getSequenceLength(seqStorePtr, sequence)).litLength as libc::c_ulong,
            );
            seqCount = seqCount.wrapping_add(1);
        }
        if lastSequence != 0 {
            debug_assert!(lp <= lend);
            debug_assert!(litSize <= lend.offset_from(lp) as libc::c_long as libc::size_t);
            litSize = lend.offset_from(lp) as libc::c_long as libc::size_t;
        }
        cBlockSizeEstimate = ZSTD_estimateSubBlockSize(
            lp,
            litSize,
            ofCodePtr,
            llCodePtr,
            mlCodePtr,
            seqCount,
            &mut (*nextCBlock).entropy,
            entropyMetadata,
            workspace,
            wkspSize,
            writeLitEntropy,
            writeSeqEntropy,
        );
        if cBlockSizeEstimate > targetCBlockSize || lastSequence != 0 {
            let mut litEntropyWritten = 0 as libc::c_int;
            let mut seqEntropyWritten = 0 as libc::c_int;
            let decompressedSize =
                ZSTD_seqDecompressedSize(seqStorePtr, sp, seqCount, litSize, lastSequence);
            let cSize = ZSTD_compressSubBlock(
                &mut (*nextCBlock).entropy,
                entropyMetadata,
                sp,
                seqCount,
                lp,
                litSize,
                llCodePtr,
                mlCodePtr,
                ofCodePtr,
                cctxParams,
                op as *mut libc::c_void,
                oend.offset_from(op) as libc::c_long as libc::size_t,
                bmi2,
                writeLitEntropy,
                writeSeqEntropy,
                &mut litEntropyWritten,
                &mut seqEntropyWritten,
                (lastBlock != 0 && lastSequence != 0) as libc::c_int as u32,
            );
            let err_code = cSize;
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if cSize > 0 && cSize < decompressedSize {
                debug_assert!(ip.offset(decompressedSize as isize) <= iend);
                ip = ip.offset(decompressedSize as isize);
                sp = sp.offset(seqCount as isize);
                lp = lp.offset(litSize as isize);
                op = op.offset(cSize as isize);
                llCodePtr = llCodePtr.offset(seqCount as isize);
                mlCodePtr = mlCodePtr.offset(seqCount as isize);
                ofCodePtr = ofCodePtr.offset(seqCount as isize);
                litSize = 0 as libc::c_int as libc::size_t;
                seqCount = 0 as libc::c_int as libc::size_t;
                if litEntropyWritten != 0 {
                    writeLitEntropy = 0 as libc::c_int;
                }
                if seqEntropyWritten != 0 {
                    writeSeqEntropy = 0 as libc::c_int;
                }
            }
        }
        if !(lastSequence == 0) {
            break;
        }
    }
    if writeLitEntropy != 0 {
        libc::memcpy(
            &mut (*nextCBlock).entropy.huf as *mut ZSTD_hufCTables_t as *mut libc::c_void,
            &(*prevCBlock).entropy.huf as *const ZSTD_hufCTables_t as *const libc::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::size_t,
        );
    }
    if writeSeqEntropy != 0 && ZSTD_needSequenceEntropyTables(&(*entropyMetadata).fseMetadata) != 0
    {
        return 0 as libc::c_int as libc::size_t;
    }
    if ip < iend {
        let cSize_0 = ZSTD_noCompressBlock(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as libc::size_t,
            ip as *const libc::c_void,
            iend.offset_from(ip) as libc::c_long as libc::size_t,
            lastBlock,
        );
        let err_code_0 = cSize_0;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        debug_assert!(cSize_0 != 0 as libc::c_int as libc::c_ulong);
        op = op.offset(cSize_0 as isize);
        if sp < send {
            let mut seq = 0 as *const seqDef;
            let mut rep = repcodes_t { rep: [0; 3] };
            libc::memcpy(
                &mut rep as *mut repcodes_t as *mut libc::c_void,
                ((*prevCBlock).rep).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<repcodes_t>() as libc::size_t,
            );
            seq = sstart;
            while seq < sp {
                ZSTD_updateRep(
                    (rep.rep).as_mut_ptr(),
                    (*seq).offBase,
                    ((ZSTD_getSequenceLength(seqStorePtr, seq)).litLength == 0) as libc::c_int
                        as u32,
                );
                seq = seq.offset(1);
            }
            libc::memcpy(
                ((*nextCBlock).rep).as_mut_ptr() as *mut libc::c_void,
                &mut rep as *mut repcodes_t as *const libc::c_void,
                ::core::mem::size_of::<repcodes_t>() as libc::size_t,
            );
        }
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressSuperBlock(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut lastBlock: libc::c_uint,
) -> libc::size_t {
    let mut entropyMetadata = ZSTD_entropyCTablesMetadata_t {
        hufMetadata: ZSTD_hufCTablesMetadata_t {
            hType: set_basic,
            hufDesBuffer: [0; 128],
            hufDesSize: 0,
        },
        fseMetadata: ZSTD_fseCTablesMetadata_t {
            llType: set_basic,
            ofType: set_basic,
            mlType: set_basic,
            fseTablesBuffer: [0; 133],
            fseTablesSize: 0,
            lastCountSize: 0,
        },
    };
    let err_code = ZSTD_buildBlockEntropyStats(
        &mut (*zc).seqStore,
        &mut (*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &mut (*zc).appliedParams,
        &mut entropyMetadata,
        (*zc).entropyWorkspace as *mut libc::c_void,
        ((((8) << 10 as libc::c_int) + 512) as libc::c_ulong).wrapping_add(
            (::core::mem::size_of::<libc::c_uint>()).wrapping_mul(
                ((if 35 as libc::c_int > 52 {
                    35 as libc::c_int
                } else {
                    52 as libc::c_int
                }) + 2) as libc::c_ulong,
            ),
        ),
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return ZSTD_compressSubBlock_multi(
        &mut (*zc).seqStore,
        (*zc).blockState.prevCBlock,
        (*zc).blockState.nextCBlock,
        &mut entropyMetadata,
        &mut (*zc).appliedParams,
        dst,
        dstCapacity,
        src,
        srcSize,
        (*zc).bmi2,
        lastBlock,
        (*zc).entropyWorkspace as *mut libc::c_void,
        (HUF_WORKSPACE_SIZE as libc::c_ulong).wrapping_add(
            (::core::mem::size_of::<libc::c_uint>()).wrapping_mul(
                ((if 35 as libc::c_int > 52 {
                    35 as libc::c_int
                } else {
                    52 as libc::c_int
                }) + 2) as libc::c_ulong,
            ),
        ),
    );
}
