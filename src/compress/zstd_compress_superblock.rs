use ::libc;
extern "C" {
    pub type ZSTDMT_CCtx_s;
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    fn ZSTD_buildBlockEntropyStats(
        seqStorePtr: *const SeqStore_t,
        prevEntropy: *const ZSTD_entropyCTables_t,
        nextEntropy: *mut ZSTD_entropyCTables_t,
        cctxParams: *const ZSTD_CCtx_params,
        entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
        workspace: *mut std::ffi::c_void,
        wkspSize: usize,
    ) -> usize;
    fn HUF_compress4X_usingCTable(
        dst: *mut std::ffi::c_void,
        dstSize: usize,
        src: *const std::ffi::c_void,
        srcSize: usize,
        CTable: *const HUF_CElt,
        flags: std::ffi::c_int,
    ) -> usize;
    fn HUF_estimateCompressedSize(
        CTable: *const HUF_CElt,
        count: *const std::ffi::c_uint,
        maxSymbolValue: std::ffi::c_uint,
    ) -> usize;
    fn HUF_compress1X_usingCTable(
        dst: *mut std::ffi::c_void,
        dstSize: usize,
        src: *const std::ffi::c_void,
        srcSize: usize,
        CTable: *const HUF_CElt,
        flags: std::ffi::c_int,
    ) -> usize;
    fn HIST_count_wksp(
        count: *mut std::ffi::c_uint,
        maxSymbolValuePtr: *mut std::ffi::c_uint,
        src: *const std::ffi::c_void,
        srcSize: usize,
        workSpace: *mut std::ffi::c_void,
        workSpaceSize: usize,
    ) -> usize;
    fn HIST_countFast_wksp(
        count: *mut std::ffi::c_uint,
        maxSymbolValuePtr: *mut std::ffi::c_uint,
        src: *const std::ffi::c_void,
        srcSize: usize,
        workSpace: *mut std::ffi::c_void,
        workSpaceSize: usize,
    ) -> usize;
    fn ZSTD_encodeSequences(
        dst: *mut std::ffi::c_void,
        dstCapacity: usize,
        CTable_MatchLength: *const FSE_CTable,
        mlCodeTable: *const u8,
        CTable_OffsetBits: *const FSE_CTable,
        ofCodeTable: *const u8,
        CTable_LitLength: *const FSE_CTable,
        llCodeTable: *const u8,
        sequences: *const SeqDef,
        nbSeq: usize,
        longOffsets: std::ffi::c_int,
        bmi2: std::ffi::c_int,
    ) -> usize;
    fn ZSTD_fseBitCost(
        ctable: *const FSE_CTable,
        count: *const std::ffi::c_uint,
        max: std::ffi::c_uint,
    ) -> usize;
    fn ZSTD_crossEntropyCost(
        norm: *const std::ffi::c_short,
        accuracyLog: std::ffi::c_uint,
        count: *const std::ffi::c_uint,
        max: std::ffi::c_uint,
    ) -> usize;
    fn ZSTD_noCompressLiterals(
        dst: *mut std::ffi::c_void,
        dstCapacity: usize,
        src: *const std::ffi::c_void,
        srcSize: usize,
    ) -> usize;
    fn ZSTD_compressRleLiteralsBlock(
        dst: *mut std::ffi::c_void,
        dstCapacity: usize,
        src: *const std::ffi::c_void,
        srcSize: usize,
    ) -> usize;
}
pub type C2RustUnnamed = std::ffi::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
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
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed = 49;
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
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: std::ffi::c_int,
    pub bmi2: std::ffi::c_int,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub simpleApiParams: ZSTD_CCtx_params,
    pub dictID: u32,
    pub dictContentSize: usize,
    pub workspace: ZSTD_cwksp,
    pub blockSizeMax: usize,
    pub pledgedSrcSizePlusOne: std::ffi::c_ulonglong,
    pub consumedSrcSize: std::ffi::c_ulonglong,
    pub producedCSize: std::ffi::c_ulonglong,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    pub pool: *mut ZSTD_threadPool,
    pub staticSize: usize,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: std::ffi::c_int,
    pub initialized: std::ffi::c_int,
    pub seqStore: SeqStore_t,
    pub ldmState: ldmState_t,
    pub ldmSequences: *mut rawSeq,
    pub maxNbLdmSequences: usize,
    pub externSeqStore: RawSeqStore_t,
    pub blockState: ZSTD_blockState_t,
    pub tmpWorkspace: *mut std::ffi::c_void,
    pub tmpWkspSize: usize,
    pub bufferedPolicy: ZSTD_buffered_policy_e,
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
    pub expectedInBuffer: ZSTD_inBuffer,
    pub stableIn_notConsumed: usize,
    pub expectedOutBufferSize: usize,
    pub localDict: ZSTD_localDict,
    pub cdict: *const ZSTD_CDict,
    pub prefixDict: ZSTD_prefixDict,
    pub mtctx: *mut ZSTDMT_CCtx,
    pub traceCtx: ZSTD_TraceCtx,
    pub blockSplitCtx: ZSTD_blockSplitCtx,
    pub extSeqBuf: *mut ZSTD_Sequence,
    pub extSeqBufCapacity: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Sequence {
    pub offset: std::ffi::c_uint,
    pub litLength: std::ffi::c_uint,
    pub matchLength: std::ffi::c_uint,
    pub rep: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: SeqStore_t,
    pub firstHalfSeqStore: SeqStore_t,
    pub secondHalfSeqStore: SeqStore_t,
    pub currSeqStore: SeqStore_t,
    pub nextSeqStore: SeqStore_t,
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
    pub llType: SymbolEncodingType_e,
    pub ofType: SymbolEncodingType_e,
    pub mlType: SymbolEncodingType_e,
    pub fseTablesBuffer: [u8; 133],
    pub fseTablesSize: usize,
    pub lastCountSize: usize,
}
pub type SymbolEncodingType_e = std::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTablesMetadata_t {
    pub hType: SymbolEncodingType_e,
    pub hufDesBuffer: [u8; 128],
    pub hufDesSize: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqStore_t {
    pub sequencesStart: *mut SeqDef,
    pub sequences: *mut SeqDef,
    pub litStart: *mut u8,
    pub lit: *mut u8,
    pub llCode: *mut u8,
    pub mlCode: *mut u8,
    pub ofCode: *mut u8,
    pub maxNbSeq: usize,
    pub maxNbLit: usize,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: u32,
}
pub type ZSTD_longLengthType_e = std::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type SeqDef = SeqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqDef_s {
    pub offBase: u32,
    pub litLength: u16,
    pub mlBase: u16,
}
pub type ZSTD_TraceCtx = std::ffi::c_ulonglong;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const std::ffi::c_void,
    pub dictSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_dictContentType_e = std::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_CDict = ZSTD_CDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut std::ffi::c_void,
    pub dict: *const std::ffi::c_void,
    pub dictSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const std::ffi::c_void,
    pub size: usize,
    pub pos: usize,
}
pub type ZSTD_cStreamStage = std::ffi::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = std::ffi::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_MatchState_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_MatchState_t {
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
    pub forceNonContiguous: std::ffi::c_int,
    pub dedicatedDictSearch: std::ffi::c_int,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const RawSeqStore_t,
    pub prefetchCDictTables: std::ffi::c_int,
    pub lazySkipping: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: usize,
    pub posInSequence: usize,
    pub size: usize,
    pub capacity: usize,
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
    pub windowLog: std::ffi::c_uint,
    pub chainLog: std::ffi::c_uint,
    pub hashLog: std::ffi::c_uint,
    pub searchLog: std::ffi::c_uint,
    pub minMatch: std::ffi::c_uint,
    pub targetLength: std::ffi::c_uint,
    pub strategy: ZSTD_strategy,
}
pub type ZSTD_strategy = std::ffi::c_uint;
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
    pub litFreq: *mut std::ffi::c_uint,
    pub litLengthFreq: *mut std::ffi::c_uint,
    pub matchLengthFreq: *mut std::ffi::c_uint,
    pub offCodeFreq: *mut std::ffi::c_uint,
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
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
}
pub type ZSTD_ParamSwitch_e = std::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
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
pub type FSE_repeat = std::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = std::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = usize;
pub type ZSTD_OptPrice_e = std::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: std::ffi::c_int,
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
    pub splitIndices: [usize; 64],
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
    pub collectSequences: std::ffi::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: usize,
    pub maxSequences: usize,
}
pub type ZSTD_threadPool = POOL_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut std::ffi::c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void,
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
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut std::ffi::c_void,
    pub workspaceEnd: *mut std::ffi::c_void,
    pub objectEnd: *mut std::ffi::c_void,
    pub tableEnd: *mut std::ffi::c_void,
    pub tableValidEnd: *mut std::ffi::c_void,
    pub allocStart: *mut std::ffi::c_void,
    pub initOnceStart: *mut std::ffi::c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: std::ffi::c_int,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}
pub type ZSTD_cwksp_static_alloc_e = std::ffi::c_uint;
pub const ZSTD_cwksp_static_alloc: ZSTD_cwksp_static_alloc_e = 1;
pub const ZSTD_cwksp_dynamic_alloc: ZSTD_cwksp_static_alloc_e = 0;
pub type ZSTD_cwksp_alloc_phase_e = std::ffi::c_uint;
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
    pub compressionLevel: std::ffi::c_int,
    pub forceWindow: std::ffi::c_int,
    pub targetCBlockSize: usize,
    pub srcSizeHint: std::ffi::c_int,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub nbWorkers: std::ffi::c_int,
    pub jobSize: usize,
    pub overlapLog: std::ffi::c_int,
    pub rsyncable: std::ffi::c_int,
    pub ldmParams: ldmParams_t,
    pub enableDedicatedDictSearch: std::ffi::c_int,
    pub inBufferMode: ZSTD_bufferMode_e,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub blockDelimiters: ZSTD_SequenceFormat_e,
    pub validateSequences: std::ffi::c_int,
    pub postBlockSplitter: ZSTD_ParamSwitch_e,
    pub preBlockSplitter_level: std::ffi::c_int,
    pub maxBlockSize: usize,
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,
    pub deterministicRefPrefix: std::ffi::c_int,
    pub customMem: ZSTD_customMem,
    pub prefetchCDictTables: ZSTD_ParamSwitch_e,
    pub enableMatchFinderFallback: std::ffi::c_int,
    pub extSeqProdState: *mut std::ffi::c_void,
    pub extSeqProdFunc: ZSTD_sequenceProducer_F,
    pub searchForExternalRepcodes: ZSTD_ParamSwitch_e,
}
pub type ZSTD_sequenceProducer_F = Option::<
    unsafe extern "C" fn(
        *mut std::ffi::c_void,
        *mut ZSTD_Sequence,
        usize,
        *const std::ffi::c_void,
        usize,
        *const std::ffi::c_void,
        usize,
        std::ffi::c_int,
        usize,
    ) -> usize,
>;
pub type ZSTD_SequenceFormat_e = std::ffi::c_uint;
pub const ZSTD_sf_explicitBlockDelimiters: ZSTD_SequenceFormat_e = 1;
pub const ZSTD_sf_noBlockDelimiters: ZSTD_SequenceFormat_e = 0;
pub type ZSTD_bufferMode_e = std::ffi::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: ZSTD_ParamSwitch_e,
    pub hashLog: u32,
    pub bucketSizeLog: u32,
    pub minMatchLength: u32,
    pub hashRateLog: u32,
    pub windowLog: u32,
}
pub type ZSTD_dictAttachPref_e = std::ffi::c_uint;
pub const ZSTD_dictForceLoad: ZSTD_dictAttachPref_e = 3;
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = 2;
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: std::ffi::c_int,
    pub checksumFlag: std::ffi::c_int,
    pub noDictIDFlag: std::ffi::c_int,
}
pub type ZSTD_format_e = std::ffi::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub type ZSTD_compressionStage_e = std::ffi::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type Repcodes_t = repcodes_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_SequenceLength {
    pub litLength: u32,
    pub matchLength: u32,
}
pub const bt_raw: C2RustUnnamed_1 = 0;
pub type unalign16 = u16;
pub const bt_compressed: C2RustUnnamed_1 = 2;
pub type unalign32 = u32;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EstimatedBlockSize {
    pub estLitSize: usize,
    pub estBlockSize: usize,
}
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_1 = std::ffi::c_uint;
pub const bt_reserved: C2RustUnnamed_1 = 3;
pub const bt_rle: C2RustUnnamed_1 = 1;
pub const ZSTD_TARGETCBLOCKSIZE_MIN: std::ffi::c_int = 1340;
#[inline]
unsafe extern "C" fn ZSTD_getSequenceLength(
    mut seqStore: *const SeqStore_t,
    mut seq: *const SeqDef,
) -> ZSTD_SequenceLength {
    let mut seqLen = ZSTD_SequenceLength {
        litLength: 0,
        matchLength: 0,
    };
    seqLen.litLength = (*seq).litLength as u32;
    seqLen.matchLength = ((*seq).mlBase as std::ffi::c_int + MINMATCH) as u32;
    if (*seqStore).longLengthPos
        == seq.offset_from((*seqStore).sequencesStart) as std::ffi::c_long as u32
    {
        if (*seqStore).longLengthType as std::ffi::c_uint
            == ZSTD_llt_literalLength as std::ffi::c_int as std::ffi::c_uint
        {
            seqLen
                .litLength = (seqLen.litLength)
                .wrapping_add(0x10000 as std::ffi::c_int as u32);
        }
        if (*seqStore).longLengthType as std::ffi::c_uint
            == ZSTD_llt_matchLength as std::ffi::c_int as std::ffi::c_uint
        {
            seqLen
                .matchLength = (seqLen.matchLength)
                .wrapping_add(0x10000 as std::ffi::c_int as u32);
        }
    }
    return seqLen;
}
#[inline]
unsafe extern "C" fn ZSTD_noCompressBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastBlock: u32,
) -> usize {
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_raw as std::ffi::c_int as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as usize;
    }
    MEM_writeLE24(dst, cBlockHeader24);
    libc::memcpy(
        ZSTD_memcpy!((u8 *) dst + ZSTD_blockHeaderSize, src, srcSize),
        ZSTD_memcpy!((u8 *) dst + ZSTD_blockHeaderSize, src, srcSize),
        ZSTD_memcpy!((u8 *) dst + ZSTD_blockHeaderSize, src, srcSize) as usize,
    );
    return ZSTD_blockHeaderSize.wrapping_add(srcSize);
}
#[inline]
unsafe extern "C" fn ZSTD_updateRep(mut rep: *mut u32, offBase: u32, ll0: u32) {
    if OFFBASE_IS_OFFSET!(offBase) != 0 {
        *rep
            .offset(
                2 as std::ffi::c_int as isize,
            ) = *rep.offset(1);
        *rep
            .offset(
                1 as std::ffi::c_int as isize,
            ) = *rep.offset(0);
        *rep.offset(0) = OFFBASE_TO_OFFSET!(offBase);
    } else {
        let repCode = OFFBASE_TO_REPCODE!(offBase)
            .wrapping_sub(1)
            .wrapping_add(ll0);
        if repCode > 0 as std::ffi::c_int as u32 {
            let currentOffset = if repCode == ZSTD_REP_NUM as u32 {
                (*rep.offset(0))
                    .wrapping_sub(1)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep
                .offset(
                    2 as std::ffi::c_int as isize,
                ) = if repCode >= 2 as std::ffi::c_int as u32 {
                *rep.offset(1)
            } else {
                *rep.offset(2)
            };
            *rep
                .offset(
                    1 as std::ffi::c_int as isize,
                ) = *rep.offset(0);
            *rep.offset(0) = currentOffset;
        }
    };
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    return (::core::mem::size_of::<usize>()
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int
        as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    return 1 as std::ffi::c_int as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut std::ffi::c_void, mut value: u16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut std::ffi::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut std::ffi::c_void, mut val: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut u8;
        *p.offset(0) = val as u8;
        *p
            .offset(
                1 as std::ffi::c_int as isize,
            ) = (val as std::ffi::c_int >> 8) as u8;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut std::ffi::c_void, mut val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8)
        .offset(2) = (val >> 16) as u8;
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut std::ffi::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
pub const ZSTD_isError: unsafe extern "C" fn(usize) -> std::ffi::c_uint = ERR_isError;
pub const ZSTD_REP_NUM: std::ffi::c_int = 3;
pub const ZSTD_BLOCKHEADERSIZE: std::ffi::c_int = 3;
static mut ZSTD_blockHeaderSize: usize = ZSTD_BLOCKHEADERSIZE as usize;
pub const LONGNBSEQ: std::ffi::c_int = 0x7f00 as std::ffi::c_int;
pub const MINMATCH: std::ffi::c_int = 3;
pub const MaxML: std::ffi::c_int = 52;
pub const MaxLL: std::ffi::c_int = 35;
pub const DefaultMaxOff: std::ffi::c_int = 28;
pub const MaxOff: std::ffi::c_int = 31;
static mut LL_bits: [u8; 36] = [
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    4 as std::ffi::c_int as u8,
    6 as std::ffi::c_int as u8,
    7 as std::ffi::c_int as u8,
    8 as std::ffi::c_int as u8,
    9 as std::ffi::c_int as u8,
    10 as std::ffi::c_int as u8,
    11 as std::ffi::c_int as u8,
    12 as std::ffi::c_int as u8,
    13 as std::ffi::c_int as u8,
    14 as std::ffi::c_int as u8,
    15 as std::ffi::c_int as u8,
    16 as std::ffi::c_int as u8,
];
static mut LL_defaultNorm: [i16; 36] = [
    4 as std::ffi::c_int as i16,
    3 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    3 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
];
pub const LL_DEFAULTNORMLOG: std::ffi::c_int = 6;
static mut LL_defaultNormLog: u32 = LL_DEFAULTNORMLOG as u32;
static mut ML_bits: [u8; 53] = [
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    4 as std::ffi::c_int as u8,
    4 as std::ffi::c_int as u8,
    5 as std::ffi::c_int as u8,
    7 as std::ffi::c_int as u8,
    8 as std::ffi::c_int as u8,
    9 as std::ffi::c_int as u8,
    10 as std::ffi::c_int as u8,
    11 as std::ffi::c_int as u8,
    12 as std::ffi::c_int as u8,
    13 as std::ffi::c_int as u8,
    14 as std::ffi::c_int as u8,
    15 as std::ffi::c_int as u8,
    16 as std::ffi::c_int as u8,
];
static mut ML_defaultNorm: [i16; 53] = [
    1 as std::ffi::c_int as i16,
    4 as std::ffi::c_int as i16,
    3 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
];
pub const ML_DEFAULTNORMLOG: std::ffi::c_int = 6;
static mut ML_defaultNormLog: u32 = ML_DEFAULTNORMLOG as u32;
static mut OF_defaultNorm: [i16; 29] = [
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    2 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    1 as std::ffi::c_int as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
    -(1 as std::ffi::c_int) as i16,
];
pub const OF_DEFAULTNORMLOG: std::ffi::c_int = 5;
static mut OF_defaultNormLog: u32 = OF_DEFAULTNORMLOG as u32;
unsafe extern "C" fn ERR_isError(mut code: usize) -> std::ffi::c_uint {
    return (code > ERROR!(maxCode)) as std::ffi::c_int as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const std::ffi::c_char,
    mut args: ...
) {}
pub const STREAM_ACCUMULATOR_MIN_32: std::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: std::ffi::c_int = 57;
pub const NULL: std::ffi::c_int = 0;
unsafe extern "C" fn ZSTD_compressSubBlock_literal(
    mut hufTable: *const HUF_CElt,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut literals: *const u8,
    mut litSize: usize,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    bmi2: std::ffi::c_int,
    mut writeEntropy: std::ffi::c_int,
    mut entropyWritten: *mut std::ffi::c_int,
) -> usize {
    let header = (if writeEntropy != 0 {
        200 as std::ffi::c_int
    } else {
        0 as std::ffi::c_int
    }) as usize;
    let lhSize = (3 as std::ffi::c_int
        + (litSize
            >= ((1 as std::ffi::c_int
                * ((1 as std::ffi::c_int) << 10)) as usize)
                .wrapping_sub(header)) as std::ffi::c_int
        + (litSize
            >= ((16 as std::ffi::c_int
                * ((1 as std::ffi::c_int) << 10)) as usize)
                .wrapping_sub(header)) as std::ffi::c_int) as usize;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart.offset(lhSize as isize);
    let singleStream = (lhSize == 3 as std::ffi::c_int as usize) as std::ffi::c_int
        as u32;
    let mut hType = (if writeEntropy != 0 {
        (*hufMetadata).hType as std::ffi::c_uint
    } else {
        set_repeat as std::ffi::c_int as std::ffi::c_uint
    }) as SymbolEncodingType_e;
    let mut cLitSize: usize = 0;
    *entropyWritten = 0;
    if litSize == 0 as std::ffi::c_int as usize
        || (*hufMetadata).hType as std::ffi::c_uint
            == set_basic as std::ffi::c_int as std::ffi::c_uint
    {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const std::ffi::c_void,
            litSize,
        )
    } else if (*hufMetadata).hType as std::ffi::c_uint
        == set_rle as std::ffi::c_int as std::ffi::c_uint
    {
        return ZSTD_compressRleLiteralsBlock(
            dst,
            dstSize,
            literals as *const std::ffi::c_void,
            litSize,
        )
    }
    if writeEntropy != 0
        && (*hufMetadata).hType as std::ffi::c_uint
            == set_compressed as std::ffi::c_int as std::ffi::c_uint
    {
        libc::memcpy(
            ZSTD_memcpy!(op, hufMetadata -> hufDesBuffer, hufMetadata -> hufDesSize),
            ZSTD_memcpy!(op, hufMetadata -> hufDesBuffer, hufMetadata -> hufDesSize),
            ZSTD_memcpy!(op, hufMetadata -> hufDesBuffer, hufMetadata -> hufDesSize)
                as usize,
        );
        op = op.offset((*hufMetadata).hufDesSize as isize);
        cLitSize = cLitSize.wrapping_add((*hufMetadata).hufDesSize);
    }
    let flags = if bmi2 != 0 {
        HUF_flags_bmi2 as std::ffi::c_int
    } else {
        0 as std::ffi::c_int
    };
    let cSize = if singleStream != 0 {
        HUF_compress1X_usingCTable(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as usize,
            literals as *const std::ffi::c_void,
            litSize,
            hufTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as usize,
            literals as *const std::ffi::c_void,
            litSize,
            hufTable,
            flags,
        )
    };
    op = op.offset(cSize as isize);
    cLitSize = cLitSize.wrapping_add(cSize);
    if cSize == 0 as std::ffi::c_int as usize || ERR_isError(cSize) != 0 {
        return 0 as std::ffi::c_int as usize;
    }
    if writeEntropy == 0 && cLitSize >= litSize {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const std::ffi::c_void,
            litSize,
        );
    }
    if lhSize
        < (3 as std::ffi::c_int
            + (cLitSize
                >= (1 as std::ffi::c_int
                    * ((1 as std::ffi::c_int) << 10)) as usize)
                as std::ffi::c_int
            + (cLitSize
                >= (16 as std::ffi::c_int
                    * ((1 as std::ffi::c_int) << 10)) as usize)
                as std::ffi::c_int) as usize
    {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const std::ffi::c_void,
            litSize,
        );
    }
    match lhSize {
        3 => {
            let lhc = (hType as std::ffi::c_uint)
                .wrapping_add(
                    ((singleStream == 0) as std::ffi::c_int as u32)
                        << 2,
                )
                .wrapping_add((litSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 14);
            MEM_writeLE24(ostart as *mut std::ffi::c_void, lhc);
        }
        4 => {
            let lhc_0 = (hType as std::ffi::c_uint)
                .wrapping_add(
                    ((2 as std::ffi::c_int) << 2) as std::ffi::c_uint,
                )
                .wrapping_add((litSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 18);
            MEM_writeLE32(ostart as *mut std::ffi::c_void, lhc_0);
        }
        5 => {
            let lhc_1 = (hType as std::ffi::c_uint)
                .wrapping_add(
                    ((3 as std::ffi::c_int) << 2) as std::ffi::c_uint,
                )
                .wrapping_add((litSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 22);
            MEM_writeLE32(ostart as *mut std::ffi::c_void, lhc_1);
            *ostart
                .offset(
                    4 as std::ffi::c_int as isize,
                ) = (cLitSize >> 10) as u8;
        }
        _ => {}
    }
    *entropyWritten = 1;
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
unsafe extern "C" fn ZSTD_seqDecompressedSize(
    mut seqStore: *const SeqStore_t,
    mut sequences: *const SeqDef,
    mut nbSeqs: usize,
    mut litSize: usize,
    mut lastSubBlock: std::ffi::c_int,
) -> usize {
    let mut matchLengthSum: usize = 0;
    let mut litLengthSum: usize = 0;
    let mut n: usize = 0;
    n = 0;
    while n < nbSeqs {
        let seqLen = ZSTD_getSequenceLength(seqStore, sequences.offset(n as isize));
        litLengthSum = litLengthSum.wrapping_add(seqLen.litLength as usize);
        matchLengthSum = matchLengthSum.wrapping_add(seqLen.matchLength as usize);
        n = n.wrapping_add(1);
        n;
    }
    lastSubBlock == 0;
    return matchLengthSum.wrapping_add(litSize);
}
unsafe extern "C" fn ZSTD_compressSubBlock_sequences(
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut sequences: *const SeqDef,
    mut nbSeq: usize,
    mut llCode: *const u8,
    mut mlCode: *const u8,
    mut ofCode: *const u8,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    bmi2: std::ffi::c_int,
    mut writeEntropy: std::ffi::c_int,
    mut entropyWritten: *mut std::ffi::c_int,
) -> usize {
    let longOffsets = ((*cctxParams).cParams.windowLog
        > (if MEM_32bits() != 0 {
            STREAM_ACCUMULATOR_MIN_32
        } else {
            STREAM_ACCUMULATOR_MIN_64
        }) as u32) as std::ffi::c_int;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut seqHead = 0 as *mut u8;
    *entropyWritten = 0;
    if (oend.offset_from(op) as std::ffi::c_long)
        < (3 as std::ffi::c_int + 1 as std::ffi::c_int) as std::ffi::c_long
    {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as usize;
    }
    if nbSeq < 128 as std::ffi::c_int as usize {
        let fresh0 = op;
        op = op.offset(1);
        *fresh0 = nbSeq as u8;
    } else if nbSeq < LONGNBSEQ as usize {
        *op
            .offset(
                0 as std::ffi::c_int as isize,
            ) = (nbSeq >> 8)
            .wrapping_add(0x80 as std::ffi::c_int as usize) as u8;
        *op.offset(1) = nbSeq as u8;
        op = op.offset(2);
    } else {
        *op.offset(0) = 0xff as std::ffi::c_int as u8;
        MEM_writeLE16(
            op.offset(1) as *mut std::ffi::c_void,
            nbSeq.wrapping_sub(LONGNBSEQ as usize) as u16,
        );
        op = op.offset(3);
    }
    if nbSeq == 0 as std::ffi::c_int as usize {
        return op.offset_from(ostart) as std::ffi::c_long as usize;
    }
    let fresh1 = op;
    op = op.offset(1);
    seqHead = fresh1;
    if writeEntropy != 0 {
        let LLtype = (*fseMetadata).llType as u32;
        let Offtype = (*fseMetadata).ofType as u32;
        let MLtype = (*fseMetadata).mlType as u32;
        *seqHead = (LLtype << 6)
            .wrapping_add(Offtype << 4)
            .wrapping_add(MLtype << 2) as u8;
        libc::memcpy(
            ZSTD_memcpy!(
                op, fseMetadata -> fseTablesBuffer, fseMetadata -> fseTablesSize
            ),
            ZSTD_memcpy!(
                op, fseMetadata -> fseTablesBuffer, fseMetadata -> fseTablesSize
            ),
            ZSTD_memcpy!(
                op, fseMetadata -> fseTablesBuffer, fseMetadata -> fseTablesSize
            ) as usize,
        );
        op = op.offset((*fseMetadata).fseTablesSize as isize);
    } else {
        let repeat = set_repeat as std::ffi::c_int as u32;
        *seqHead = (repeat << 6)
            .wrapping_add(repeat << 4)
            .wrapping_add(repeat << 2) as u8;
    }
    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
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
    let err_code = FORWARD_IF_ERROR!(bitstreamSize, "ZSTD_encodeSequences failed");
    if FORWARD_IF_ERROR!(bitstreamSize, "ZSTD_encodeSequences failed") != 0 {
        return FORWARD_IF_ERROR!(bitstreamSize, "ZSTD_encodeSequences failed");
    }
    op = op.offset(bitstreamSize as isize);
    if writeEntropy != 0 && (*fseMetadata).lastCountSize != 0
        && ((*fseMetadata).lastCountSize).wrapping_add(bitstreamSize)
            < 4 as std::ffi::c_int as usize
    {
        return 0 as std::ffi::c_int as usize;
    }
    if (op.offset_from(seqHead) as std::ffi::c_long)
        < 4 as std::ffi::c_int as std::ffi::c_long
    {
        return 0 as std::ffi::c_int as usize;
    }
    *entropyWritten = 1;
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
unsafe extern "C" fn ZSTD_compressSubBlock(
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut sequences: *const SeqDef,
    mut nbSeq: usize,
    mut literals: *const u8,
    mut litSize: usize,
    mut llCode: *const u8,
    mut mlCode: *const u8,
    mut ofCode: *const u8,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    bmi2: std::ffi::c_int,
    mut writeLitEntropy: std::ffi::c_int,
    mut writeSeqEntropy: std::ffi::c_int,
    mut litEntropyWritten: *mut std::ffi::c_int,
    mut seqEntropyWritten: *mut std::ffi::c_int,
    mut lastBlock: u32,
) -> usize {
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart.offset(ZSTD_blockHeaderSize as isize);
    let mut cLitSize = ZSTD_compressSubBlock_literal(
        ((*entropy).huf.CTable).as_ptr(),
        &(*entropyMetadata).hufMetadata,
        literals,
        litSize,
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        bmi2,
        writeLitEntropy,
        litEntropyWritten,
    );
    let err_code = FORWARD_IF_ERROR!(cLitSize, "ZSTD_compressSubBlock_literal failed");
    if FORWARD_IF_ERROR!(cLitSize, "ZSTD_compressSubBlock_literal failed") != 0 {
        return FORWARD_IF_ERROR!(cLitSize, "ZSTD_compressSubBlock_literal failed");
    }
    if cLitSize == 0 as std::ffi::c_int as usize {
        return 0 as std::ffi::c_int as usize;
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
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        bmi2,
        writeSeqEntropy,
        seqEntropyWritten,
    );
    let err_code_0 = FORWARD_IF_ERROR!(
        cSeqSize, "ZSTD_compressSubBlock_sequences failed"
    );
    if FORWARD_IF_ERROR!(cSeqSize, "ZSTD_compressSubBlock_sequences failed") != 0 {
        return FORWARD_IF_ERROR!(cSeqSize, "ZSTD_compressSubBlock_sequences failed");
    }
    if cSeqSize == 0 as std::ffi::c_int as usize {
        return 0 as std::ffi::c_int as usize;
    }
    op = op.offset(cSeqSize as isize);
    let mut cSize = (op.offset_from(ostart) as std::ffi::c_long as usize)
        .wrapping_sub(ZSTD_blockHeaderSize);
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_compressed as std::ffi::c_int as u32) << 1)
        .wrapping_add((cSize << 3) as u32);
    MEM_writeLE24(ostart as *mut std::ffi::c_void, cBlockHeader24);
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_literal(
    mut literals: *const u8,
    mut litSize: usize,
    mut huf: *const ZSTD_hufCTables_t,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut writeEntropy: std::ffi::c_int,
) -> usize {
    let countWksp = workspace as *mut std::ffi::c_uint;
    let mut maxSymbolValue: std::ffi::c_uint = 255;
    let mut literalSectionHeaderSize: usize = 3;
    if (*hufMetadata).hType as std::ffi::c_uint
        == set_basic as std::ffi::c_int as std::ffi::c_uint
    {
        return litSize
    } else if (*hufMetadata).hType as std::ffi::c_uint
        == set_rle as std::ffi::c_int as std::ffi::c_uint
    {
        return 1 as std::ffi::c_int as usize
    } else if (*hufMetadata).hType as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint
        || (*hufMetadata).hType as std::ffi::c_uint
            == set_repeat as std::ffi::c_int as std::ffi::c_uint
    {
        let largest = HIST_count_wksp(
            countWksp,
            &mut maxSymbolValue,
            literals as *const std::ffi::c_void,
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
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add((*hufMetadata).hufDesSize);
        }
        return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
    }
    return 0 as std::ffi::c_int as usize;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_symbolType(
    mut type_0: SymbolEncodingType_e,
    mut codeTable: *const u8,
    mut maxCode: std::ffi::c_uint,
    mut nbSeq: usize,
    mut fseCTable: *const FSE_CTable,
    mut additionalBits: *const u8,
    mut defaultNorm: *const std::ffi::c_short,
    mut defaultNormLog: u32,
    mut defaultMax: u32,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let countWksp = workspace as *mut std::ffi::c_uint;
    let mut ctp = codeTable;
    let ctStart = ctp;
    let ctEnd = ctStart.offset(nbSeq as isize);
    let mut cSymbolTypeSizeEstimateInBits: usize = 0;
    let mut max = maxCode;
    HIST_countFast_wksp(
        countWksp,
        &mut max,
        codeTable as *const std::ffi::c_void,
        nbSeq,
        workspace,
        wkspSize,
    );
    if type_0 as std::ffi::c_uint == set_basic as std::ffi::c_int as std::ffi::c_uint {
        cSymbolTypeSizeEstimateInBits = if max <= defaultMax {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, countWksp, max)
        } else {
            ERROR!(GENERIC)
        };
    } else if type_0 as std::ffi::c_uint
        == set_rle as std::ffi::c_int as std::ffi::c_uint
    {
        cSymbolTypeSizeEstimateInBits = 0;
    } else if type_0 as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint
        || type_0 as std::ffi::c_uint
            == set_repeat as std::ffi::c_int as std::ffi::c_uint
    {
        cSymbolTypeSizeEstimateInBits = ZSTD_fseBitCost(fseCTable, countWksp, max);
    }
    if ERR_isError(cSymbolTypeSizeEstimateInBits) != 0 {
        return nbSeq * 10 as std::ffi::c_int as usize;
    }
    while ctp < ctEnd {
        if !additionalBits.is_null() {
            cSymbolTypeSizeEstimateInBits = cSymbolTypeSizeEstimateInBits
                .wrapping_add(*additionalBits.offset(*ctp as isize) as usize);
        } else {
            cSymbolTypeSizeEstimateInBits = cSymbolTypeSizeEstimateInBits
                .wrapping_add(*ctp as usize);
        }
        ctp = ctp.offset(1);
        ctp;
    }
    return cSymbolTypeSizeEstimateInBits / 8 as std::ffi::c_int as usize;
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize_sequences(
    mut ofCodeTable: *const u8,
    mut llCodeTable: *const u8,
    mut mlCodeTable: *const u8,
    mut nbSeq: usize,
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut writeEntropy: std::ffi::c_int,
) -> usize {
    let sequencesSectionHeaderSize = 3;
    let mut cSeqSizeEstimate: usize = 0;
    if nbSeq == 0 as std::ffi::c_int as usize {
        return sequencesSectionHeaderSize;
    }
    cSeqSizeEstimate = cSeqSizeEstimate
        .wrapping_add(
            ZSTD_estimateSubBlockSize_symbolType(
                (*fseMetadata).ofType,
                ofCodeTable,
                MaxOff as std::ffi::c_uint,
                nbSeq,
                ((*fseTables).offcodeCTable).as_ptr(),
                NULL as *const u8,
                OF_defaultNorm.as_ptr(),
                OF_defaultNormLog,
                DefaultMaxOff as u32,
                workspace,
                wkspSize,
            ),
        );
    cSeqSizeEstimate = cSeqSizeEstimate
        .wrapping_add(
            ZSTD_estimateSubBlockSize_symbolType(
                (*fseMetadata).llType,
                llCodeTable,
                MaxLL as std::ffi::c_uint,
                nbSeq,
                ((*fseTables).litlengthCTable).as_ptr(),
                LL_bits.as_ptr(),
                LL_defaultNorm.as_ptr(),
                LL_defaultNormLog,
                MaxLL as u32,
                workspace,
                wkspSize,
            ),
        );
    cSeqSizeEstimate = cSeqSizeEstimate
        .wrapping_add(
            ZSTD_estimateSubBlockSize_symbolType(
                (*fseMetadata).mlType,
                mlCodeTable,
                MaxML as std::ffi::c_uint,
                nbSeq,
                ((*fseTables).matchlengthCTable).as_ptr(),
                ML_bits.as_ptr(),
                ML_defaultNorm.as_ptr(),
                ML_defaultNormLog,
                MaxML as u32,
                workspace,
                wkspSize,
            ),
        );
    if writeEntropy != 0 {
        cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add((*fseMetadata).fseTablesSize);
    }
    return cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize);
}
unsafe extern "C" fn ZSTD_estimateSubBlockSize(
    mut literals: *const u8,
    mut litSize: usize,
    mut ofCodeTable: *const u8,
    mut llCodeTable: *const u8,
    mut mlCodeTable: *const u8,
    mut nbSeq: usize,
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut writeLitEntropy: std::ffi::c_int,
    mut writeSeqEntropy: std::ffi::c_int,
) -> EstimatedBlockSize {
    let mut ebs = EstimatedBlockSize {
        estLitSize: 0,
        estBlockSize: 0,
    };
    ebs
        .estLitSize = ZSTD_estimateSubBlockSize_literal(
        literals,
        litSize,
        &(*entropy).huf,
        &(*entropyMetadata).hufMetadata,
        workspace,
        wkspSize,
        writeLitEntropy,
    );
    ebs
        .estBlockSize = ZSTD_estimateSubBlockSize_sequences(
        ofCodeTable,
        llCodeTable,
        mlCodeTable,
        nbSeq,
        &(*entropy).fse,
        &(*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
        writeSeqEntropy,
    );
    ebs
        .estBlockSize = (ebs.estBlockSize)
        .wrapping_add((ebs.estLitSize).wrapping_add(ZSTD_blockHeaderSize));
    return ebs;
}
unsafe extern "C" fn ZSTD_needSequenceEntropyTables(
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
) -> std::ffi::c_int {
    if (*fseMetadata).llType as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint
        || (*fseMetadata).llType as std::ffi::c_uint
            == set_rle as std::ffi::c_int as std::ffi::c_uint
    {
        return 1 as std::ffi::c_int;
    }
    if (*fseMetadata).mlType as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint
        || (*fseMetadata).mlType as std::ffi::c_uint
            == set_rle as std::ffi::c_int as std::ffi::c_uint
    {
        return 1 as std::ffi::c_int;
    }
    if (*fseMetadata).ofType as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint
        || (*fseMetadata).ofType as std::ffi::c_uint
            == set_rle as std::ffi::c_int as std::ffi::c_uint
    {
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn countLiterals(
    mut seqStore: *const SeqStore_t,
    mut sp: *const SeqDef,
    mut seqCount: usize,
) -> usize {
    let mut n: usize = 0;
    let mut total: usize = 0;
    n = 0;
    while n < seqCount {
        total = total
            .wrapping_add(
                (ZSTD_getSequenceLength(seqStore, sp.offset(n as isize))).litLength
                    as usize,
            );
        n = n.wrapping_add(1);
        n;
    }
    return total;
}
pub const BYTESCALE: std::ffi::c_int = 256;
unsafe extern "C" fn sizeBlockSequences(
    mut sp: *const SeqDef,
    mut nbSeqs: usize,
    mut targetBudget: usize,
    mut avgLitCost: usize,
    mut avgSeqCost: usize,
    mut firstSubBlock: std::ffi::c_int,
) -> usize {
    let mut n: usize = 0;
    let mut budget: usize = 0;
    let mut inSize: usize = 0;
    let headerSize = firstSubBlock as usize * 120 as std::ffi::c_int as usize
        * BYTESCALE as usize;
    budget = budget.wrapping_add(headerSize);
    budget = budget
        .wrapping_add(
            ((*sp.offset(0)).litLength as usize
                * avgLitCost)
                .wrapping_add(avgSeqCost),
        );
    if budget > targetBudget {
        return 1 as std::ffi::c_int as usize;
    }
    inSize = ((*sp.offset(0)).litLength as std::ffi::c_int
        + ((*sp.offset(0)).mlBase as std::ffi::c_int
            + MINMATCH)) as usize;
    n = 1;
    while n < nbSeqs {
        let mut currentCost = ((*sp.offset(n as isize)).litLength as usize * avgLitCost)
            .wrapping_add(avgSeqCost);
        budget = budget.wrapping_add(currentCost);
        inSize = inSize
            .wrapping_add(
                ((*sp.offset(n as isize)).litLength as std::ffi::c_int
                    + ((*sp.offset(n as isize)).mlBase as std::ffi::c_int + MINMATCH))
                    as usize,
            );
        if budget > targetBudget && budget < inSize * BYTESCALE as usize {
            break;
        }
        n = n.wrapping_add(1);
        n;
    }
    return n;
}
unsafe extern "C" fn ZSTD_compressSubBlock_multi(
    mut seqStorePtr: *const SeqStore_t,
    mut prevCBlock: *const ZSTD_compressedBlockState_t,
    mut nextCBlock: *mut ZSTD_compressedBlockState_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    bmi2: std::ffi::c_int,
    mut lastBlock: u32,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let sstart: *const SeqDef = (*seqStorePtr).sequencesStart;
    let send: *const SeqDef = (*seqStorePtr).sequences;
    let mut sp = sstart;
    let nbSeqs = send.offset_from(sstart) as std::ffi::c_long as usize;
    let lstart: *const u8 = (*seqStorePtr).litStart;
    let lend: *const u8 = (*seqStorePtr).lit;
    let mut lp = lstart;
    let nbLiterals = lend.offset_from(lstart) as std::ffi::c_long as usize;
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut llCodePtr: *const u8 = (*seqStorePtr).llCode;
    let mut mlCodePtr: *const u8 = (*seqStorePtr).mlCode;
    let mut ofCodePtr: *const u8 = (*seqStorePtr).ofCode;
    let minTarget = ZSTD_TARGETCBLOCKSIZE_MIN as usize;
    let targetCBlockSize = MAX!(minTarget, cctxParams -> targetCBlockSize);
    let mut writeLitEntropy = ((*entropyMetadata).hufMetadata.hType as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
    let mut writeSeqEntropy: std::ffi::c_int = 1;
    if nbSeqs > 0 as std::ffi::c_int as usize {
        let ebs = ZSTD_estimateSubBlockSize(
            lp,
            nbLiterals,
            ofCodePtr,
            llCodePtr,
            mlCodePtr,
            nbSeqs,
            &mut (*nextCBlock).entropy,
            entropyMetadata,
            workspace,
            wkspSize,
            writeLitEntropy,
            writeSeqEntropy,
        );
        let avgLitCost = if nbLiterals != 0 {
            ebs.estLitSize * BYTESCALE as usize / nbLiterals
        } else {
            BYTESCALE as usize
        };
        let avgSeqCost = (ebs.estBlockSize).wrapping_sub(ebs.estLitSize)
            * BYTESCALE as usize / nbSeqs;
        let nbSubBlocks = MAX!(
            (ebs.estBlockSize + (targetCBlockSize / 2)) / targetCBlockSize, 1
        );
        let mut n: usize = 0;
        let mut avgBlockBudget: usize = 0;
        let mut blockBudgetSupp: usize = 0;
        avgBlockBudget = ebs.estBlockSize * BYTESCALE as usize / nbSubBlocks;
        if ebs.estBlockSize > srcSize {
            return 0 as std::ffi::c_int as usize;
        }
        n = 0;
        while n < nbSubBlocks.wrapping_sub(1) {
            let seqCount = sizeBlockSequences(
                sp,
                send.offset_from(sp) as std::ffi::c_long as usize,
                avgBlockBudget.wrapping_add(blockBudgetSupp),
                avgLitCost,
                avgSeqCost,
                (n == 0 as std::ffi::c_int as usize) as std::ffi::c_int,
            );
            if sp.offset(seqCount as isize) == send {
                break;
            }
            let mut litEntropyWritten: std::ffi::c_int = 0;
            let mut seqEntropyWritten: std::ffi::c_int = 0;
            let mut litSize = countLiterals(seqStorePtr, sp, seqCount);
            let decompressedSize = ZSTD_seqDecompressedSize(
                seqStorePtr,
                sp,
                seqCount,
                litSize,
                0 as std::ffi::c_int,
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
                op as *mut std::ffi::c_void,
                oend.offset_from(op) as std::ffi::c_long as usize,
                bmi2,
                writeLitEntropy,
                writeSeqEntropy,
                &mut litEntropyWritten,
                &mut seqEntropyWritten,
                0 as std::ffi::c_int as u32,
            );
            let err_code = FORWARD_IF_ERROR!(cSize, "ZSTD_compressSubBlock failed");
            if FORWARD_IF_ERROR!(cSize, "ZSTD_compressSubBlock failed") != 0 {
                return FORWARD_IF_ERROR!(cSize, "ZSTD_compressSubBlock failed");
            }
            if cSize > 0 as std::ffi::c_int as usize && cSize < decompressedSize {
                ip = ip.offset(decompressedSize as isize);
                lp = lp.offset(litSize as isize);
                op = op.offset(cSize as isize);
                llCodePtr = llCodePtr.offset(seqCount as isize);
                mlCodePtr = mlCodePtr.offset(seqCount as isize);
                ofCodePtr = ofCodePtr.offset(seqCount as isize);
                if litEntropyWritten != 0 {
                    writeLitEntropy = 0;
                }
                if seqEntropyWritten != 0 {
                    writeSeqEntropy = 0;
                }
                sp = sp.offset(seqCount as isize);
                blockBudgetSupp = 0;
            }
            n = n.wrapping_add(1);
            n;
        }
    }
    let mut litEntropyWritten_0: std::ffi::c_int = 0;
    let mut seqEntropyWritten_0: std::ffi::c_int = 0;
    let mut litSize_0 = lend.offset_from(lp) as std::ffi::c_long as usize;
    let mut seqCount_0 = send.offset_from(sp) as std::ffi::c_long as usize;
    let decompressedSize_0 = ZSTD_seqDecompressedSize(
        seqStorePtr,
        sp,
        seqCount_0,
        litSize_0,
        1 as std::ffi::c_int,
    );
    let cSize_0 = ZSTD_compressSubBlock(
        &mut (*nextCBlock).entropy,
        entropyMetadata,
        sp,
        seqCount_0,
        lp,
        litSize_0,
        llCodePtr,
        mlCodePtr,
        ofCodePtr,
        cctxParams,
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        bmi2,
        writeLitEntropy,
        writeSeqEntropy,
        &mut litEntropyWritten_0,
        &mut seqEntropyWritten_0,
        lastBlock,
    );
    let err_code_0 = FORWARD_IF_ERROR!(cSize, "ZSTD_compressSubBlock failed");
    if FORWARD_IF_ERROR!(cSize, "ZSTD_compressSubBlock failed") != 0 {
        return FORWARD_IF_ERROR!(cSize, "ZSTD_compressSubBlock failed");
    }
    if cSize_0 > 0 as std::ffi::c_int as usize && cSize_0 < decompressedSize_0 {
        ip = ip.offset(decompressedSize_0 as isize);
        lp = lp.offset(litSize_0 as isize);
        op = op.offset(cSize_0 as isize);
        llCodePtr = llCodePtr.offset(seqCount_0 as isize);
        mlCodePtr = mlCodePtr.offset(seqCount_0 as isize);
        ofCodePtr = ofCodePtr.offset(seqCount_0 as isize);
        if litEntropyWritten_0 != 0 {
            writeLitEntropy = 0;
        }
        if seqEntropyWritten_0 != 0 {
            writeSeqEntropy = 0;
        }
        sp = sp.offset(seqCount_0 as isize);
    }
    if writeLitEntropy != 0 {
        libc::memcpy(
            &mut (*nextCBlock).entropy.huf as *mut ZSTD_hufCTables_t
                as *mut std::ffi::c_void,
            &(*prevCBlock).entropy.huf as *const ZSTD_hufCTables_t
                as *const std::ffi::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>()
                as usize,
        );
    }
    if writeSeqEntropy != 0
        && ZSTD_needSequenceEntropyTables(&(*entropyMetadata).fseMetadata) != 0
    {
        return 0 as std::ffi::c_int as usize;
    }
    if ip < iend {
        let rSize = iend.offset_from(ip) as std::ffi::c_long as usize;
        let cSize_1 = ZSTD_noCompressBlock(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as usize,
            ip as *const std::ffi::c_void,
            rSize,
            lastBlock,
        );
        let err_code_1 = FORWARD_IF_ERROR!(cSize, "ZSTD_noCompressBlock failed");
        if FORWARD_IF_ERROR!(cSize, "ZSTD_noCompressBlock failed") != 0 {
            return FORWARD_IF_ERROR!(cSize, "ZSTD_noCompressBlock failed");
        }
        op = op.offset(cSize_1 as isize);
        if sp < send {
            let mut seq = 0 as *const SeqDef;
            let mut rep = repcodes_s { rep: [0; 3] };
            libc::memcpy(
                &mut rep as *mut Repcodes_t as *mut std::ffi::c_void,
                ((*prevCBlock).rep).as_ptr() as *const std::ffi::c_void,
                ::core::mem::size_of::<Repcodes_t>() as usize,
            );
            seq = sstart;
            while seq < sp {
                ZSTD_updateRep(
                    (rep.rep).as_mut_ptr(),
                    (*seq).offBase,
                    ((ZSTD_getSequenceLength(seqStorePtr, seq)).litLength
                        == 0 as std::ffi::c_int as u32) as std::ffi::c_int as u32,
                );
                seq = seq.offset(1);
                seq;
            }
            libc::memcpy(
                ((*nextCBlock).rep).as_mut_ptr() as *mut std::ffi::c_void,
                &mut rep as *mut Repcodes_t as *const std::ffi::c_void,
                ::core::mem::size_of::<Repcodes_t>() as usize,
            );
        }
    }
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressSuperBlock(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastBlock: std::ffi::c_uint,
) -> usize {
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
    let err_code = FORWARD_IF_ERROR!(
        ZSTD_buildBlockEntropyStats(& zc -> seqStore, & zc -> blockState.prevCBlock ->
        entropy, & zc -> blockState.nextCBlock -> entropy, & zc -> appliedParams, &
        entropyMetadata, zc -> tmpWorkspace, zc -> tmpWkspSize), ""
    );
    if FORWARD_IF_ERROR!(
        ZSTD_buildBlockEntropyStats(& zc -> seqStore, & zc -> blockState.prevCBlock ->
        entropy, & zc -> blockState.nextCBlock -> entropy, & zc -> appliedParams, &
        entropyMetadata, zc -> tmpWorkspace, zc -> tmpWkspSize), ""
    ) != 0
    {
        return FORWARD_IF_ERROR!(
            ZSTD_buildBlockEntropyStats(& zc -> seqStore, & zc -> blockState.prevCBlock
            -> entropy, & zc -> blockState.nextCBlock -> entropy, & zc -> appliedParams,
            & entropyMetadata, zc -> tmpWorkspace, zc -> tmpWkspSize), ""
        );
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
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
    );
}
