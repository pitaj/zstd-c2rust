use ::libc;
extern "C" {
    pub type ZSTDMT_CCtx_s;
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    fn HUF_estimateCompressedSize(
        CTable: *const HUF_CElt,
        count: *const libc::c_uint,
        maxSymbolValue: libc::c_uint,
    ) -> size_t;
    fn HUF_compress1X_usingCTable(
        dst: *mut libc::c_void,
        dstSize: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
        CTable: *const HUF_CElt,
        flags: libc::c_int,
    ) -> size_t;
    fn HUF_compress4X_usingCTable(
        dst: *mut libc::c_void,
        dstSize: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
        CTable: *const HUF_CElt,
        flags: libc::c_int,
    ) -> size_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn HIST_count_wksp(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: size_t,
        workSpace: *mut libc::c_void,
        workSpaceSize: size_t,
    ) -> size_t;
    fn HIST_countFast_wksp(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: size_t,
        workSpace: *mut libc::c_void,
        workSpaceSize: size_t,
    ) -> size_t;
    fn ZSTD_buildBlockEntropyStats(
        seqStorePtr: *const seqStore_t,
        prevEntropy: *const ZSTD_entropyCTables_t,
        nextEntropy: *mut ZSTD_entropyCTables_t,
        cctxParams: *const ZSTD_CCtx_params,
        entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
        workspace: *mut libc::c_void,
        wkspSize: size_t,
    ) -> size_t;
    fn ZSTD_encodeSequences(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        CTable_MatchLength: *const FSE_CTable,
        mlCodeTable: *const BYTE,
        CTable_OffsetBits: *const FSE_CTable,
        ofCodeTable: *const BYTE,
        CTable_LitLength: *const FSE_CTable,
        llCodeTable: *const BYTE,
        sequences: *const seqDef,
        nbSeq: size_t,
        longOffsets: libc::c_int,
        bmi2: libc::c_int,
    ) -> size_t;
    fn ZSTD_fseBitCost(
        ctable: *const FSE_CTable,
        count: *const libc::c_uint,
        max: libc::c_uint,
    ) -> size_t;
    fn ZSTD_crossEntropyCost(
        norm: *const libc::c_short,
        accuracyLog: libc::c_uint,
        count: *const libc::c_uint,
        max: libc::c_uint,
    ) -> size_t;
    fn ZSTD_noCompressLiterals(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressRleLiteralsBlock(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: libc::c_int,
    pub bmi2: libc::c_int,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub simpleApiParams: ZSTD_CCtx_params,
    pub dictID: U32,
    pub dictContentSize: size_t,
    pub workspace: ZSTD_cwksp,
    pub blockSize: size_t,
    pub pledgedSrcSizePlusOne: libc::c_ulonglong,
    pub consumedSrcSize: libc::c_ulonglong,
    pub producedCSize: libc::c_ulonglong,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    pub pool: *mut ZSTD_threadPool,
    pub staticSize: size_t,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: libc::c_int,
    pub initialized: libc::c_int,
    pub seqStore: seqStore_t,
    pub ldmState: ldmState_t,
    pub ldmSequences: *mut rawSeq,
    pub maxNbLdmSequences: size_t,
    pub externSeqStore: rawSeqStore_t,
    pub blockState: ZSTD_blockState_t,
    pub entropyWorkspace: *mut U32,
    pub bufferedPolicy: ZSTD_buffered_policy_e,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: size_t,
    pub inToCompress: size_t,
    pub inBuffPos: size_t,
    pub inBuffTarget: size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: size_t,
    pub outBuffContentSize: size_t,
    pub outBuffFlushedSize: size_t,
    pub streamStage: ZSTD_cStreamStage,
    pub frameEnded: U32,
    pub expectedInBuffer: ZSTD_inBuffer,
    pub stableIn_notConsumed: size_t,
    pub expectedOutBufferSize: size_t,
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
    pub mFinder: Option::<ZSTD_sequenceProducer_F>,
    pub seqBuffer: *mut ZSTD_Sequence,
    pub seqBufferCapacity: size_t,
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
    size_t,
    *const libc::c_void,
    size_t,
    *const libc::c_void,
    size_t,
    libc::c_int,
    size_t,
) -> size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: seqStore_t,
    pub firstHalfSeqStore: seqStore_t,
    pub secondHalfSeqStore: seqStore_t,
    pub currSeqStore: seqStore_t,
    pub nextSeqStore: seqStore_t,
    pub partitions: [U32; 196],
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
    pub fseTablesBuffer: [BYTE; 133],
    pub fseTablesSize: size_t,
    pub lastCountSize: size_t,
}
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type symbolEncodingType_e = libc::c_uint;
pub const set_repeat: symbolEncodingType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub const set_rle: symbolEncodingType_e = 1;
pub const set_basic: symbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTablesMetadata_t {
    pub hType: symbolEncodingType_e,
    pub hufDesBuffer: [BYTE; 128],
    pub hufDesSize: size_t,
}
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut BYTE,
    pub lit: *mut BYTE,
    pub llCode: *mut BYTE,
    pub mlCode: *mut BYTE,
    pub ofCode: *mut BYTE,
    pub maxNbSeq: size_t,
    pub maxNbLit: size_t,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: U32,
}
pub type ZSTD_longLengthType_e = libc::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type seqDef = seqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqDef_s {
    pub offBase: U32,
    pub litLength: U16,
    pub mlBase: U16,
}
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type ZSTD_TraceCtx = libc::c_ulonglong;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const libc::c_void,
    pub dictSize: size_t,
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
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
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
    pub loadedDictEnd: U32,
    pub nextToUpdate: U32,
    pub hashLog3: U32,
    pub rowHashLog: U32,
    pub tagTable: *mut BYTE,
    pub hashCache: [U32; 8],
    pub hashSalt: U64,
    pub hashSaltEntropy: U32,
    pub hashTable: *mut U32,
    pub hashTable3: *mut U32,
    pub chainTable: *mut U32,
    pub forceNonContiguous: U32,
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
    pub pos: size_t,
    pub posInSequence: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
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
    pub litSum: U32,
    pub litLengthSum: U32,
    pub matchLengthSum: U32,
    pub offCodeSum: U32,
    pub litSumBasePrice: U32,
    pub litLengthSumBasePrice: U32,
    pub matchLengthSumBasePrice: U32,
    pub offCodeSumBasePrice: U32,
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
pub type HUF_CElt = size_t;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
    pub nbOverflowCorrections: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [U32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub loadedDictEnd: U32,
    pub bucketOffsets: *mut BYTE,
    pub splitIndices: [size_t; 64],
    pub matchCandidates: [ldmMatchCandidate_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmMatchCandidate_t {
    pub split: *const BYTE,
    pub hash: U32,
    pub checksum: U32,
    pub bucket: *mut ldmEntry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: U32,
    pub checksum: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: libc::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: size_t,
    pub maxSequences: size_t,
}
pub type ZSTD_threadPool = POOL_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
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
pub type XXH64_hash_t = uint64_t;
pub type XXH32_hash_t = uint32_t;
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
    pub allocFailed: BYTE,
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
    pub targetCBlockSize: size_t,
    pub srcSizeHint: libc::c_int,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
    pub nbWorkers: libc::c_int,
    pub jobSize: size_t,
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
    pub maxBlockSize: size_t,
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
    pub hashLog: U32,
    pub bucketSizeLog: U32,
    pub minMatchLength: U32,
    pub hashRateLog: U32,
    pub windowLog: U32,
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
    pub rep: [U32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_sequenceLength {
    pub litLength: U32,
    pub matchLength: U32,
}
pub const bt_raw: C2RustUnnamed_1 = 0;
pub type unalign16 = U16;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
pub const bt_compressed: C2RustUnnamed_1 = 2;
pub type unalign32 = U32;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type S16 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub type U8 = uint8_t;
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
static mut OF_defaultNorm: [S16; 29] = [
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
];
pub const OF_DEFAULTNORMLOG: libc::c_int = 5 as libc::c_int;
static mut OF_defaultNormLog: U32 = OF_DEFAULTNORMLOG as U32;
pub const DefaultMaxOff: libc::c_int = 28 as libc::c_int;
static mut LL_bits: [U8; 36] = [
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    2 as libc::c_int as U8,
    2 as libc::c_int as U8,
    3 as libc::c_int as U8,
    3 as libc::c_int as U8,
    4 as libc::c_int as U8,
    6 as libc::c_int as U8,
    7 as libc::c_int as U8,
    8 as libc::c_int as U8,
    9 as libc::c_int as U8,
    10 as libc::c_int as U8,
    11 as libc::c_int as U8,
    12 as libc::c_int as U8,
    13 as libc::c_int as U8,
    14 as libc::c_int as U8,
    15 as libc::c_int as U8,
    16 as libc::c_int as U8,
];
static mut LL_defaultNorm: [S16; 36] = [
    4 as libc::c_int as S16,
    3 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    3 as libc::c_int as S16,
    2 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
];
pub const LL_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
static mut LL_defaultNormLog: U32 = LL_DEFAULTNORMLOG as U32;
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
static mut ML_bits: [U8; 53] = [
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    2 as libc::c_int as U8,
    2 as libc::c_int as U8,
    3 as libc::c_int as U8,
    3 as libc::c_int as U8,
    4 as libc::c_int as U8,
    4 as libc::c_int as U8,
    5 as libc::c_int as U8,
    7 as libc::c_int as U8,
    8 as libc::c_int as U8,
    9 as libc::c_int as U8,
    10 as libc::c_int as U8,
    11 as libc::c_int as U8,
    12 as libc::c_int as U8,
    13 as libc::c_int as U8,
    14 as libc::c_int as U8,
    15 as libc::c_int as U8,
    16 as libc::c_int as U8,
];
static mut ML_defaultNorm: [S16; 53] = [
    1 as libc::c_int as S16,
    4 as libc::c_int as S16,
    3 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    2 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    1 as libc::c_int as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
    -(1 as libc::c_int) as S16,
];
static mut ML_defaultNormLog: U32 = ML_DEFAULTNORMLOG as U32;
pub const ML_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
pub const MaxML: libc::c_int = 52 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: U32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: U32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
pub const LONGNBSEQ: libc::c_int = 0x7f00 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
pub const STREAM_ACCUMULATOR_MIN_32: libc::c_int = 25 as libc::c_int;
pub const STREAM_ACCUMULATOR_MIN_64: libc::c_int = 57 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: U16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: U16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut BYTE;
        *p.offset(0 as libc::c_int as isize) = val as BYTE;
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (val as libc::c_int >> 8 as libc::c_int) as BYTE;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut libc::c_void, mut val: U32) {
    MEM_writeLE16(memPtr, val as U16);
    *(memPtr as *mut BYTE)
        .offset(2 as libc::c_int as isize) = (val >> 16 as libc::c_int) as BYTE;
}
pub const ZSTD_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
static mut ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
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
    seqLen.litLength = (*seq).litLength as U32;
    seqLen.matchLength = ((*seq).mlBase as libc::c_int + MINMATCH) as U32;
    if (*seqStore).longLengthPos
        == seq.offset_from((*seqStore).sequencesStart) as libc::c_long as U32
    {
        if (*seqStore).longLengthType as libc::c_uint
            == ZSTD_llt_literalLength as libc::c_int as libc::c_uint
        {
            seqLen
                .litLength = (seqLen.litLength as libc::c_uint)
                .wrapping_add(0x10000 as libc::c_int as libc::c_uint) as U32 as U32;
        }
        if (*seqStore).longLengthType as libc::c_uint
            == ZSTD_llt_matchLength as libc::c_int as libc::c_uint
        {
            seqLen
                .matchLength = (seqLen.matchLength as libc::c_uint)
                .wrapping_add(0x10000 as libc::c_int as libc::c_uint) as U32 as U32;
        }
    }
    return seqLen;
}
pub const HUF_WORKSPACE_SIZE: libc::c_int = ((8 as libc::c_int) << 10 as libc::c_int)
    + 512 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_noCompressBlock(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut lastBlock: U32,
) -> size_t {
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_raw as libc::c_int as U32) << 1 as libc::c_int)
        .wrapping_add((srcSize << 3 as libc::c_int) as U32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    MEM_writeLE24(dst, cBlockHeader24);
    libc::memcpy(
        (dst as *mut BYTE).offset(ZSTD_blockHeaderSize as isize) as *mut libc::c_void,
        src,
        srcSize as libc::size_t,
    );
    return ZSTD_blockHeaderSize.wrapping_add(srcSize);
}
#[inline]
unsafe extern "C" fn ZSTD_updateRep(mut rep: *mut U32, offBase: U32, ll0: U32) {
    if offBase > ZSTD_REP_NUM as libc::c_uint {
        *rep.offset(2 as libc::c_int as isize) = *rep.offset(1 as libc::c_int as isize);
        *rep.offset(1 as libc::c_int as isize) = *rep.offset(0 as libc::c_int as isize);
        if offBase > 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                723 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void ZSTD_updateRep(U32 *, const U32, const U32)\0"))
                    .as_ptr(),
            );
        }
        *rep
            .offset(
                0 as libc::c_int as isize,
            ) = offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_uint);
    } else {
        if 1 as libc::c_int as libc::c_uint <= offBase
            && offBase <= 3 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"OFFBASE_IS_REPCODE(offBase)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                725 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void ZSTD_updateRep(U32 *, const U32, const U32)\0"))
                    .as_ptr(),
            );
        }
        let repCode = offBase
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_add(ll0);
        if repCode > 0 as libc::c_int as libc::c_uint {
            let currentOffset = if repCode == ZSTD_REP_NUM as libc::c_uint {
                (*rep.offset(0 as libc::c_int as isize))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep
                .offset(
                    2 as libc::c_int as isize,
                ) = if repCode >= 2 as libc::c_int as libc::c_uint {
                *rep.offset(1 as libc::c_int as isize)
            } else {
                *rep.offset(2 as libc::c_int as isize)
            };
            *rep
                .offset(
                    1 as libc::c_int as isize,
                ) = *rep.offset(0 as libc::c_int as isize);
            *rep.offset(0 as libc::c_int as isize) = currentOffset;
        }
    };
}
unsafe extern "C" fn ZSTD_compressSubBlock_literal(
    mut hufTable: *const HUF_CElt,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut literals: *const BYTE,
    mut litSize: size_t,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    bmi2: libc::c_int,
    mut writeEntropy: libc::c_int,
    mut entropyWritten: *mut libc::c_int,
) -> size_t {
    let header = (if writeEntropy != 0 { 200 as libc::c_int } else { 0 as libc::c_int })
        as size_t;
    let lhSize = (3 as libc::c_int
        + (litSize
            >= ((1 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong)
                .wrapping_sub(header)) as libc::c_int
        + (litSize
            >= ((16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong)
                .wrapping_sub(header)) as libc::c_int) as size_t;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart.offset(lhSize as isize);
    let singleStream = (lhSize == 3 as libc::c_int as libc::c_ulong) as libc::c_int
        as U32;
    let mut hType = (if writeEntropy != 0 {
        (*hufMetadata).hType as libc::c_uint
    } else {
        set_repeat as libc::c_int as libc::c_uint
    }) as symbolEncodingType_e;
    let mut cLitSize = 0 as libc::c_int as size_t;
    *entropyWritten = 0 as libc::c_int;
    if litSize == 0 as libc::c_int as libc::c_ulong
        || (*hufMetadata).hType as libc::c_uint
            == set_basic as libc::c_int as libc::c_uint
    {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const libc::c_void,
            litSize,
        )
    } else {
        if (*hufMetadata).hType as libc::c_uint == set_rle as libc::c_int as libc::c_uint
        {
            return ZSTD_compressRleLiteralsBlock(
                dst,
                dstSize,
                literals as *const libc::c_void,
                litSize,
            );
        }
    }
    if litSize > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"litSize > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 151],
                &[libc::c_char; 151],
            >(
                b"size_t ZSTD_compressSubBlock_literal(const HUF_CElt *, const ZSTD_hufCTablesMetadata_t *, const BYTE *, size_t, void *, size_t, const int, int, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*hufMetadata).hType as libc::c_uint
        == set_compressed as libc::c_int as libc::c_uint
        || (*hufMetadata).hType as libc::c_uint
            == set_repeat as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"hufMetadata->hType == set_compressed || hufMetadata->hType == set_repeat\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 151],
                &[libc::c_char; 151],
            >(
                b"size_t ZSTD_compressSubBlock_literal(const HUF_CElt *, const ZSTD_hufCTablesMetadata_t *, const BYTE *, size_t, void *, size_t, const int, int, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if writeEntropy != 0
        && (*hufMetadata).hType as libc::c_uint
            == set_compressed as libc::c_int as libc::c_uint
    {
        libc::memcpy(
            op as *mut libc::c_void,
            ((*hufMetadata).hufDesBuffer).as_ptr() as *const libc::c_void,
            (*hufMetadata).hufDesSize as libc::size_t,
        );
        op = op.offset((*hufMetadata).hufDesSize as isize);
        cLitSize = (cLitSize as libc::c_ulong).wrapping_add((*hufMetadata).hufDesSize)
            as size_t as size_t;
    }
    let flags = if bmi2 != 0 { HUF_flags_bmi2 as libc::c_int } else { 0 as libc::c_int };
    let cSize = if singleStream != 0 {
        HUF_compress1X_usingCTable(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as size_t,
            literals as *const libc::c_void,
            litSize,
            hufTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as size_t,
            literals as *const libc::c_void,
            litSize,
            hufTable,
            flags,
        )
    };
    op = op.offset(cSize as isize);
    cLitSize = (cLitSize as libc::c_ulong).wrapping_add(cSize) as size_t as size_t;
    if cSize == 0 as libc::c_int as libc::c_ulong || ERR_isError(cSize) != 0 {
        return 0 as libc::c_int as size_t;
    }
    if writeEntropy == 0 && cLitSize >= litSize {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const libc::c_void,
            litSize,
        );
    }
    if lhSize
        < (3 as libc::c_int
            + (cLitSize
                >= (1 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                    as libc::c_ulong) as libc::c_int
            + (cLitSize
                >= (16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                    as libc::c_ulong) as libc::c_int) as size_t
    {
        if cLitSize > litSize {} else {
            __assert_fail(
                b"cLitSize > litSize\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                    as *const u8 as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 151],
                    &[libc::c_char; 151],
                >(
                    b"size_t ZSTD_compressSubBlock_literal(const HUF_CElt *, const ZSTD_hufCTablesMetadata_t *, const BYTE *, size_t, void *, size_t, const int, int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const libc::c_void,
            litSize,
        );
    }
    match lhSize {
        3 => {
            let lhc = (hType as libc::c_uint)
                .wrapping_add(
                    (((singleStream == 0) as libc::c_int) << 2 as libc::c_int)
                        as libc::c_uint,
                )
                .wrapping_add((litSize as U32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as U32) << 14 as libc::c_int);
            MEM_writeLE24(ostart as *mut libc::c_void, lhc);
        }
        4 => {
            let lhc_0 = (hType as libc::c_uint)
                .wrapping_add(((2 as libc::c_int) << 2 as libc::c_int) as libc::c_uint)
                .wrapping_add((litSize as U32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as U32) << 18 as libc::c_int);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_0);
        }
        5 => {
            let lhc_1 = (hType as libc::c_uint)
                .wrapping_add(((3 as libc::c_int) << 2 as libc::c_int) as libc::c_uint)
                .wrapping_add((litSize as U32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as U32) << 22 as libc::c_int);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_1);
            *ostart
                .offset(
                    4 as libc::c_int as isize,
                ) = (cLitSize >> 10 as libc::c_int) as BYTE;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                    as *const u8 as *const libc::c_char,
                121 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 151],
                    &[libc::c_char; 151],
                >(
                    b"size_t ZSTD_compressSubBlock_literal(const HUF_CElt *, const ZSTD_hufCTablesMetadata_t *, const BYTE *, size_t, void *, size_t, const int, int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    *entropyWritten = 1 as libc::c_int;
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_seqDecompressedSize(
    mut seqStore: *const seqStore_t,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut litSize: size_t,
    mut lastSequence: libc::c_int,
) -> size_t {
    let sstart = sequences;
    let send = sequences.offset(nbSeq as isize);
    let mut sp = sstart;
    let mut matchLengthSum = 0 as libc::c_int as size_t;
    let mut litLengthSum = 0 as libc::c_int as size_t;
    while send.offset_from(sp) as libc::c_long > 0 as libc::c_int as libc::c_long {
        let seqLen = ZSTD_getSequenceLength(seqStore, sp);
        litLengthSum = (litLengthSum as libc::c_ulong)
            .wrapping_add(seqLen.litLength as libc::c_ulong) as size_t as size_t;
        matchLengthSum = (matchLengthSum as libc::c_ulong)
            .wrapping_add(seqLen.matchLength as libc::c_ulong) as size_t as size_t;
        sp = sp.offset(1);
    }
    if litLengthSum <= litSize {} else {
        __assert_fail(
            b"litLengthSum <= litSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"size_t ZSTD_seqDecompressedSize(const seqStore_t *, const seqDef *, size_t, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    if lastSequence == 0 {
        if litLengthSum == litSize {} else {
            __assert_fail(
                b"litLengthSum == litSize\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                    as *const u8 as *const libc::c_char,
                147 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"size_t ZSTD_seqDecompressedSize(const seqStore_t *, const seqDef *, size_t, size_t, int)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    return matchLengthSum.wrapping_add(litSize);
}
unsafe extern "C" fn ZSTD_compressSubBlock_sequences(
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut llCode: *const BYTE,
    mut mlCode: *const BYTE,
    mut ofCode: *const BYTE,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    bmi2: libc::c_int,
    mut writeEntropy: libc::c_int,
    mut entropyWritten: *mut libc::c_int,
) -> size_t {
    let longOffsets = ((*cctxParams).cParams.windowLog
        > (if MEM_32bits() != 0 {
            STREAM_ACCUMULATOR_MIN_32
        } else {
            STREAM_ACCUMULATOR_MIN_64
        }) as U32) as libc::c_int;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut seqHead = 0 as *mut BYTE;
    *entropyWritten = 0 as libc::c_int;
    if (oend.offset_from(op) as libc::c_long)
        < (3 as libc::c_int + 1 as libc::c_int) as libc::c_long
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if nbSeq < 0x7f as libc::c_int as libc::c_ulong {
        let fresh0 = op;
        op = op.offset(1);
        *fresh0 = nbSeq as BYTE;
    } else if nbSeq < LONGNBSEQ as libc::c_ulong {
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = (nbSeq >> 8 as libc::c_int)
            .wrapping_add(0x80 as libc::c_int as libc::c_ulong) as BYTE;
        *op.offset(1 as libc::c_int as isize) = nbSeq as BYTE;
        op = op.offset(2 as libc::c_int as isize);
    } else {
        *op.offset(0 as libc::c_int as isize) = 0xff as libc::c_int as BYTE;
        MEM_writeLE16(
            op.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            nbSeq.wrapping_sub(LONGNBSEQ as libc::c_ulong) as U16,
        );
        op = op.offset(3 as libc::c_int as isize);
    }
    if nbSeq == 0 as libc::c_int as libc::c_ulong {
        return op.offset_from(ostart) as libc::c_long as size_t;
    }
    let fresh1 = op;
    op = op.offset(1);
    seqHead = fresh1;
    if writeEntropy != 0 {
        let LLtype = (*fseMetadata).llType as U32;
        let Offtype = (*fseMetadata).ofType as U32;
        let MLtype = (*fseMetadata).mlType as U32;
        *seqHead = (LLtype << 6 as libc::c_int)
            .wrapping_add(Offtype << 4 as libc::c_int)
            .wrapping_add(MLtype << 2 as libc::c_int) as BYTE;
        libc::memcpy(
            op as *mut libc::c_void,
            ((*fseMetadata).fseTablesBuffer).as_ptr() as *const libc::c_void,
            (*fseMetadata).fseTablesSize as libc::size_t,
        );
        op = op.offset((*fseMetadata).fseTablesSize as isize);
    } else {
        let repeat = set_repeat as libc::c_int as U32;
        *seqHead = (repeat << 6 as libc::c_int)
            .wrapping_add(repeat << 4 as libc::c_int)
            .wrapping_add(repeat << 2 as libc::c_int) as BYTE;
    }
    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
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
    if writeEntropy != 0 && (*fseMetadata).lastCountSize != 0
        && ((*fseMetadata).lastCountSize).wrapping_add(bitstreamSize)
            < 4 as libc::c_int as libc::c_ulong
    {
        if ((*fseMetadata).lastCountSize).wrapping_add(bitstreamSize)
            == 3 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"fseMetadata->lastCountSize + bitstreamSize == 3\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                    as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 232],
                    &[libc::c_char; 232],
                >(
                    b"size_t ZSTD_compressSubBlock_sequences(const ZSTD_fseCTables_t *, const ZSTD_fseCTablesMetadata_t *, const seqDef *, size_t, const BYTE *, const BYTE *, const BYTE *, const ZSTD_CCtx_params *, void *, size_t, const int, int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        return 0 as libc::c_int as size_t;
    }
    if (op.offset_from(seqHead) as libc::c_long) < 4 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as size_t;
    }
    *entropyWritten = 1 as libc::c_int;
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_compressSubBlock(
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut literals: *const BYTE,
    mut litSize: size_t,
    mut llCode: *const BYTE,
    mut mlCode: *const BYTE,
    mut ofCode: *const BYTE,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    bmi2: libc::c_int,
    mut writeLitEntropy: libc::c_int,
    mut writeSeqEntropy: libc::c_int,
    mut litEntropyWritten: *mut libc::c_int,
    mut seqEntropyWritten: *mut libc::c_int,
    mut lastBlock: U32,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart.offset(ZSTD_blockHeaderSize as isize);
    let mut cLitSize = ZSTD_compressSubBlock_literal(
        ((*entropy).huf.CTable).as_ptr(),
        &(*entropyMetadata).hufMetadata,
        literals,
        litSize,
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        bmi2,
        writeLitEntropy,
        litEntropyWritten,
    );
    let err_code = cLitSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if cLitSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
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
        oend.offset_from(op) as libc::c_long as size_t,
        bmi2,
        writeSeqEntropy,
        seqEntropyWritten,
    );
    let err_code_0 = cSeqSize;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if cSeqSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    op = op.offset(cSeqSize as isize);
    let mut cSize = (op.offset_from(ostart) as libc::c_long as libc::c_ulong)
        .wrapping_sub(ZSTD_blockHeaderSize);
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_compressed as libc::c_int as U32) << 1 as libc::c_int)
        .wrapping_add((cSize << 3 as libc::c_int) as U32);
    MEM_writeLE24(ostart as *mut libc::c_void, cBlockHeader24);
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_literal(
    mut literals: *const BYTE,
    mut litSize: size_t,
    mut huf: *const ZSTD_hufCTables_t,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut writeEntropy: libc::c_int,
) -> size_t {
    let countWksp = workspace as *mut libc::c_uint;
    let mut maxSymbolValue = 255 as libc::c_int as libc::c_uint;
    let mut literalSectionHeaderSize = 3 as libc::c_int as size_t;
    if (*hufMetadata).hType as libc::c_uint == set_basic as libc::c_int as libc::c_uint {
        return litSize
    } else {
        if (*hufMetadata).hType as libc::c_uint == set_rle as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int as size_t
        } else {
            if (*hufMetadata).hType as libc::c_uint
                == set_compressed as libc::c_int as libc::c_uint
                || (*hufMetadata).hType as libc::c_uint
                    == set_repeat as libc::c_int as libc::c_uint
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
                let mut cLitSizeEstimate = HUF_estimateCompressedSize(
                    ((*huf).CTable).as_ptr(),
                    countWksp,
                    maxSymbolValue,
                );
                if writeEntropy != 0 {
                    cLitSizeEstimate = (cLitSizeEstimate as libc::c_ulong)
                        .wrapping_add((*hufMetadata).hufDesSize) as size_t as size_t;
                }
                return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
            }
        }
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
            as *const u8 as *const libc::c_char,
        325 as libc::c_int as libc::c_uint,
        (*::core::mem::transmute::<
            &[u8; 146],
            &[libc::c_char; 146],
        >(
            b"size_t ZSTD_estimateSubBlockSize_literal(const BYTE *, size_t, const ZSTD_hufCTables_t *, const ZSTD_hufCTablesMetadata_t *, void *, size_t, int)\0",
        ))
            .as_ptr(),
    );
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_symbolType(
    mut type_0: symbolEncodingType_e,
    mut codeTable: *const BYTE,
    mut maxCode: libc::c_uint,
    mut nbSeq: size_t,
    mut fseCTable: *const FSE_CTable,
    mut additionalBits: *const U8,
    mut defaultNorm: *const libc::c_short,
    mut defaultNormLog: U32,
    mut defaultMax: U32,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let countWksp = workspace as *mut libc::c_uint;
    let mut ctp = codeTable;
    let ctStart = ctp;
    let ctEnd = ctStart.offset(nbSeq as isize);
    let mut cSymbolTypeSizeEstimateInBits = 0 as libc::c_int as size_t;
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
        if max <= defaultMax {} else {
            __assert_fail(
                b"max <= defaultMax\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                    as *const u8 as *const libc::c_char,
                346 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 175],
                    &[libc::c_char; 175],
                >(
                    b"size_t ZSTD_estimateSubBlockSize_symbolType(symbolEncodingType_e, const BYTE *, unsigned int, size_t, const FSE_CTable *, const U8 *, const short *, U32, U32, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        cSymbolTypeSizeEstimateInBits = if max <= defaultMax {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, countWksp, max)
        } else {
            -(ZSTD_error_GENERIC as libc::c_int) as size_t
        };
    } else if type_0 as libc::c_uint == set_rle as libc::c_int as libc::c_uint {
        cSymbolTypeSizeEstimateInBits = 0 as libc::c_int as size_t;
    } else if type_0 as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == set_repeat as libc::c_int as libc::c_uint
    {
        cSymbolTypeSizeEstimateInBits = ZSTD_fseBitCost(fseCTable, countWksp, max);
    }
    if ERR_isError(cSymbolTypeSizeEstimateInBits) != 0 {
        return nbSeq.wrapping_mul(10 as libc::c_int as libc::c_ulong);
    }
    while ctp < ctEnd {
        if !additionalBits.is_null() {
            cSymbolTypeSizeEstimateInBits = (cSymbolTypeSizeEstimateInBits
                as libc::c_ulong)
                .wrapping_add(*additionalBits.offset(*ctp as isize) as libc::c_ulong)
                as size_t as size_t;
        } else {
            cSymbolTypeSizeEstimateInBits = (cSymbolTypeSizeEstimateInBits
                as libc::c_ulong)
                .wrapping_add(*ctp as libc::c_ulong) as size_t as size_t;
        }
        ctp = ctp.offset(1);
    }
    return cSymbolTypeSizeEstimateInBits.wrapping_div(8 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_sequences(
    mut ofCodeTable: *const BYTE,
    mut llCodeTable: *const BYTE,
    mut mlCodeTable: *const BYTE,
    mut nbSeq: size_t,
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut writeEntropy: libc::c_int,
) -> size_t {
    let sequencesSectionHeaderSize = 3 as libc::c_int as size_t;
    let mut cSeqSizeEstimate = 0 as libc::c_int as size_t;
    if nbSeq == 0 as libc::c_int as libc::c_ulong {
        return sequencesSectionHeaderSize;
    }
    cSeqSizeEstimate = (cSeqSizeEstimate as libc::c_ulong)
        .wrapping_add(
            ZSTD_estimateSubBlockSize_symbolType(
                (*fseMetadata).ofType,
                ofCodeTable,
                MaxOff as libc::c_uint,
                nbSeq,
                ((*fseTables).offcodeCTable).as_ptr(),
                NULL as *const U8,
                OF_defaultNorm.as_ptr(),
                OF_defaultNormLog,
                DefaultMaxOff as U32,
                workspace,
                wkspSize,
            ),
        ) as size_t as size_t;
    cSeqSizeEstimate = (cSeqSizeEstimate as libc::c_ulong)
        .wrapping_add(
            ZSTD_estimateSubBlockSize_symbolType(
                (*fseMetadata).llType,
                llCodeTable,
                MaxLL as libc::c_uint,
                nbSeq,
                ((*fseTables).litlengthCTable).as_ptr(),
                LL_bits.as_ptr(),
                LL_defaultNorm.as_ptr(),
                LL_defaultNormLog,
                MaxLL as U32,
                workspace,
                wkspSize,
            ),
        ) as size_t as size_t;
    cSeqSizeEstimate = (cSeqSizeEstimate as libc::c_ulong)
        .wrapping_add(
            ZSTD_estimateSubBlockSize_symbolType(
                (*fseMetadata).mlType,
                mlCodeTable,
                MaxML as libc::c_uint,
                nbSeq,
                ((*fseTables).matchlengthCTable).as_ptr(),
                ML_bits.as_ptr(),
                ML_defaultNorm.as_ptr(),
                ML_defaultNormLog,
                MaxML as U32,
                workspace,
                wkspSize,
            ),
        ) as size_t as size_t;
    if writeEntropy != 0 {
        cSeqSizeEstimate = (cSeqSizeEstimate as libc::c_ulong)
            .wrapping_add((*fseMetadata).fseTablesSize) as size_t as size_t;
    }
    return cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize);
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize(
    mut literals: *const BYTE,
    mut litSize: size_t,
    mut ofCodeTable: *const BYTE,
    mut llCodeTable: *const BYTE,
    mut mlCodeTable: *const BYTE,
    mut nbSeq: size_t,
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut writeLitEntropy: libc::c_int,
    mut writeSeqEntropy: libc::c_int,
) -> size_t {
    let mut cSizeEstimate = 0 as libc::c_int as size_t;
    cSizeEstimate = (cSizeEstimate as libc::c_ulong)
        .wrapping_add(
            ZSTD_estimateSubBlockSize_literal(
                literals,
                litSize,
                &(*entropy).huf,
                &(*entropyMetadata).hufMetadata,
                workspace,
                wkspSize,
                writeLitEntropy,
            ),
        ) as size_t as size_t;
    cSizeEstimate = (cSizeEstimate as libc::c_ulong)
        .wrapping_add(
            ZSTD_estimateSubBlockSize_sequences(
                ofCodeTable,
                llCodeTable,
                mlCodeTable,
                nbSeq,
                &(*entropy).fse,
                &(*entropyMetadata).fseMetadata,
                workspace,
                wkspSize,
                writeSeqEntropy,
            ),
        ) as size_t as size_t;
    return cSizeEstimate.wrapping_add(ZSTD_blockHeaderSize);
}
unsafe extern "C" fn ZSTD_needSequenceEntropyTables(
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
) -> libc::c_int {
    if (*fseMetadata).llType as libc::c_uint
        == set_compressed as libc::c_int as libc::c_uint
        || (*fseMetadata).llType as libc::c_uint
            == set_rle as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if (*fseMetadata).mlType as libc::c_uint
        == set_compressed as libc::c_int as libc::c_uint
        || (*fseMetadata).mlType as libc::c_uint
            == set_rle as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if (*fseMetadata).ofType as libc::c_uint
        == set_compressed as libc::c_int as libc::c_uint
        || (*fseMetadata).ofType as libc::c_uint
            == set_rle as libc::c_int as libc::c_uint
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
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    bmi2: libc::c_int,
    mut lastBlock: U32,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let sstart: *const seqDef = (*seqStorePtr).sequencesStart;
    let send: *const seqDef = (*seqStorePtr).sequences;
    let mut sp = sstart;
    let lstart: *const BYTE = (*seqStorePtr).litStart;
    let lend: *const BYTE = (*seqStorePtr).lit;
    let mut lp = lstart;
    let mut ip = src as *const BYTE;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut llCodePtr: *const BYTE = (*seqStorePtr).llCode;
    let mut mlCodePtr: *const BYTE = (*seqStorePtr).mlCode;
    let mut ofCodePtr: *const BYTE = (*seqStorePtr).ofCode;
    let mut targetCBlockSize = (*cctxParams).targetCBlockSize;
    let mut litSize: size_t = 0;
    let mut seqCount: size_t = 0;
    let mut writeLitEntropy = ((*entropyMetadata).hufMetadata.hType as libc::c_uint
        == set_compressed as libc::c_int as libc::c_uint) as libc::c_int;
    let mut writeSeqEntropy = 1 as libc::c_int;
    let mut lastSequence = 0 as libc::c_int;
    litSize = 0 as libc::c_int as size_t;
    seqCount = 0 as libc::c_int as size_t;
    loop {
        let mut cBlockSizeEstimate = 0 as libc::c_int as size_t;
        if sstart == send {
            lastSequence = 1 as libc::c_int;
        } else {
            let sequence = sp.offset(seqCount as isize);
            lastSequence = (sequence == send.offset(-(1 as libc::c_int as isize)))
                as libc::c_int;
            litSize = (litSize as libc::c_ulong)
                .wrapping_add(
                    (ZSTD_getSequenceLength(seqStorePtr, sequence)).litLength
                        as libc::c_ulong,
                ) as size_t as size_t;
            seqCount = seqCount.wrapping_add(1);
        }
        if lastSequence != 0 {
            if lp <= lend {} else {
                __assert_fail(
                    b"lp <= lend\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                        as *const u8 as *const libc::c_char,
                    475 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 258],
                        &[libc::c_char; 258],
                    >(
                        b"size_t ZSTD_compressSubBlock_multi(const seqStore_t *, const ZSTD_compressedBlockState_t *, ZSTD_compressedBlockState_t *, const ZSTD_entropyCTablesMetadata_t *, const ZSTD_CCtx_params *, void *, size_t, const void *, size_t, const int, U32, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            if litSize <= lend.offset_from(lp) as libc::c_long as size_t {} else {
                __assert_fail(
                    b"litSize <= (size_t)(lend - lp)\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                        as *const u8 as *const libc::c_char,
                    476 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 258],
                        &[libc::c_char; 258],
                    >(
                        b"size_t ZSTD_compressSubBlock_multi(const seqStore_t *, const ZSTD_compressedBlockState_t *, ZSTD_compressedBlockState_t *, const ZSTD_entropyCTablesMetadata_t *, const ZSTD_CCtx_params *, void *, size_t, const void *, size_t, const int, U32, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            litSize = lend.offset_from(lp) as libc::c_long as size_t;
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
            let decompressedSize = ZSTD_seqDecompressedSize(
                seqStorePtr,
                sp,
                seqCount,
                litSize,
                lastSequence,
            );
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
                oend.offset_from(op) as libc::c_long as size_t,
                bmi2,
                writeLitEntropy,
                writeSeqEntropy,
                &mut litEntropyWritten,
                &mut seqEntropyWritten,
                (lastBlock != 0 && lastSequence != 0) as libc::c_int as U32,
            );
            let err_code = cSize;
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if cSize > 0 as libc::c_int as libc::c_ulong && cSize < decompressedSize {
                if ip.offset(decompressedSize as isize) <= iend {} else {
                    __assert_fail(
                        b"ip + decompressedSize <= iend\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                            as *const u8 as *const libc::c_char,
                        503 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 258],
                            &[libc::c_char; 258],
                        >(
                            b"size_t ZSTD_compressSubBlock_multi(const seqStore_t *, const ZSTD_compressedBlockState_t *, ZSTD_compressedBlockState_t *, const ZSTD_entropyCTablesMetadata_t *, const ZSTD_CCtx_params *, void *, size_t, const void *, size_t, const int, U32, void *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ip = ip.offset(decompressedSize as isize);
                sp = sp.offset(seqCount as isize);
                lp = lp.offset(litSize as isize);
                op = op.offset(cSize as isize);
                llCodePtr = llCodePtr.offset(seqCount as isize);
                mlCodePtr = mlCodePtr.offset(seqCount as isize);
                ofCodePtr = ofCodePtr.offset(seqCount as isize);
                litSize = 0 as libc::c_int as size_t;
                seqCount = 0 as libc::c_int as size_t;
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
            &mut (*nextCBlock).entropy.huf as *mut ZSTD_hufCTables_t
                as *mut libc::c_void,
            &(*prevCBlock).entropy.huf as *const ZSTD_hufCTables_t
                as *const libc::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong as libc::size_t,
        );
    }
    if writeSeqEntropy != 0
        && ZSTD_needSequenceEntropyTables(&(*entropyMetadata).fseMetadata) != 0
    {
        return 0 as libc::c_int as size_t;
    }
    if ip < iend {
        let cSize_0 = ZSTD_noCompressBlock(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as size_t,
            ip as *const libc::c_void,
            iend.offset_from(ip) as libc::c_long as size_t,
            lastBlock,
        );
        let err_code_0 = cSize_0;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        if cSize_0 != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cSize != 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_superblock.c\0"
                    as *const u8 as *const libc::c_char,
                538 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 258],
                    &[libc::c_char; 258],
                >(
                    b"size_t ZSTD_compressSubBlock_multi(const seqStore_t *, const ZSTD_compressedBlockState_t *, ZSTD_compressedBlockState_t *, const ZSTD_entropyCTablesMetadata_t *, const ZSTD_CCtx_params *, void *, size_t, const void *, size_t, const int, U32, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        op = op.offset(cSize_0 as isize);
        if sp < send {
            let mut seq = 0 as *const seqDef;
            let mut rep = repcodes_t { rep: [0; 3] };
            libc::memcpy(
                &mut rep as *mut repcodes_t as *mut libc::c_void,
                ((*prevCBlock).rep).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
            );
            seq = sstart;
            while seq < sp {
                ZSTD_updateRep(
                    (rep.rep).as_mut_ptr(),
                    (*seq).offBase,
                    ((ZSTD_getSequenceLength(seqStorePtr, seq)).litLength
                        == 0 as libc::c_int as libc::c_uint) as libc::c_int as U32,
                );
                seq = seq.offset(1);
            }
            libc::memcpy(
                ((*nextCBlock).rep).as_mut_ptr() as *mut libc::c_void,
                &mut rep as *mut repcodes_t as *const libc::c_void,
                ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
            );
        }
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressSuperBlock(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut lastBlock: libc::c_uint,
) -> size_t {
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
        ((((8 as libc::c_int) << 10 as libc::c_int) + 512 as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(
                        ((if 35 as libc::c_int > 52 as libc::c_int {
                            35 as libc::c_int
                        } else {
                            52 as libc::c_int
                        }) + 2 as libc::c_int) as libc::c_ulong,
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
        (HUF_WORKSPACE_SIZE as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(
                        ((if 35 as libc::c_int > 52 as libc::c_int {
                            35 as libc::c_int
                        } else {
                            52 as libc::c_int
                        }) + 2 as libc::c_int) as libc::c_ulong,
                    ),
            ),
    );
}
