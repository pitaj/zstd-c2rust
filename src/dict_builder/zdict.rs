use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTDMT_CCtx_s;
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock() -> clock_t;
    fn FSE_normalizeCount(
        normalizedCounter: *mut libc::c_short,
        tableLog: libc::c_uint,
        count: *const libc::c_uint,
        srcSize: libc::size_t,
        maxSymbolValue: libc::c_uint,
        useLowProbCount: libc::c_uint,
    ) -> libc::size_t;
    fn FSE_writeNCount(
        buffer: *mut libc::c_void,
        bufferSize: libc::size_t,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
    ) -> libc::size_t;
    fn HUF_writeCTable_wksp(
        dst: *mut libc::c_void,
        maxDstSize: libc::size_t,
        CTable: *const HUF_CElt,
        maxSymbolValue: libc::c_uint,
        huffLog: libc::c_uint,
        workspace: *mut libc::c_void,
        workspaceSize: libc::size_t,
    ) -> libc::size_t;
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    fn HUF_buildCTable_wksp(
        tree: *mut HUF_CElt,
        count: *const libc::c_uint,
        maxSymbolValue: u32,
        maxNbBits: u32,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_seqToCodes(seqStorePtr: *const seqStore_t) -> libc::c_int;
    fn ZSTD_getSeqStore(ctx: *const ZSTD_CCtx) -> *const seqStore_t;
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> libc::size_t;
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> libc::size_t;
    fn ZSTD_createCDict_advanced(
        dict: *const libc::c_void,
        dictSize: libc::size_t,
        dictLoadMethod: ZSTD_dictLoadMethod_e,
        dictContentType: ZSTD_dictContentType_e,
        cParams: ZSTD_compressionParameters,
        customMem: ZSTD_customMem,
    ) -> *mut ZSTD_CDict;
    fn ZSTD_getParams(
        compressionLevel: libc::c_int,
        estimatedSrcSize: libc::c_ulonglong,
        dictSize: libc::size_t,
    ) -> ZSTD_parameters;
    fn ZSTD_XXH64(
        input: *const libc::c_void,
        length: libc::size_t,
        seed: XXH64_hash_t,
    ) -> XXH64_hash_t;
    fn ZSTD_loadCEntropy(
        bs: *mut ZSTD_compressedBlockState_t,
        workspace: *mut libc::c_void,
        dict: *const libc::c_void,
        dictSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_reset_compressedBlockState(bs: *mut ZSTD_compressedBlockState_t);
    fn ZSTD_compressBegin_usingCDict_deprecated(
        cctx: *mut ZSTD_CCtx,
        cdict: *const ZSTD_CDict,
    ) -> libc::size_t;
    fn ZSTD_compressBlock_deprecated(
        cctx: *mut ZSTD_CCtx,
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
    fn ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: libc::size_t,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const libc::size_t,
        nbSamples: libc::c_uint,
        parameters: *mut ZDICT_fastCover_params_t,
    ) -> libc::size_t;
    fn divsufsort(
        T: *const libc::c_uchar,
        SA: *mut libc::c_int,
        n: libc::c_int,
        openMP: libc::c_int,
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: libc::size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
pub type unalignArch = libc::size_t;
pub type FSE_CTable = libc::c_uint;
pub type ZSTD_ErrorCode = libc::c_uint;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_externalSequences_invalid: ZSTD_ErrorCode = 107;
pub const ZSTD_error_sequenceProducer_failed: ZSTD_ErrorCode = 106;
pub const ZSTD_error_srcBuffer_wrong: ZSTD_ErrorCode = 105;
pub const ZSTD_error_dstBuffer_wrong: ZSTD_ErrorCode = 104;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ZSTD_ErrorCode = 82;
pub const ZSTD_error_noForwardProgress_destFull: ZSTD_ErrorCode = 80;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ZSTD_ErrorCode = 50;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_combination_unsupported: ZSTD_ErrorCode = 41;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_literals_headerWrong: ZSTD_ErrorCode = 24;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type ERR_enum = ZSTD_ErrorCode;
pub type FSE_repeat = libc::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type HUF_CElt = libc::size_t;
pub type HUF_repeat = libc::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub f: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: libc::c_double,
    pub accel: libc::c_uint,
    pub shrinkDict: libc::c_uint,
    pub shrinkDictMaxRegression: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EStats_ress_t {
    pub dict: *mut ZSTD_CDict,
    pub zc: *mut ZSTD_CCtx,
    pub workPlace: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct offsetCount_t {
    pub offset: u32,
    pub count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictItem {
    pub pos: u32,
    pub length: u32,
    pub savings: u32,
}
pub const DEBUGLEVEL: libc::c_int = 1 as libc::c_int;
pub const MINRATIO: libc::c_int = 4 as libc::c_int;
pub const ZDICT_MAX_SAMPLES_SIZE: libc::c_uint = (2000) << 20 as libc::c_int;
pub const ZDICT_MIN_SAMPLES_SIZE: libc::c_int = ZDICT_CONTENTSIZE_MIN * MINRATIO;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const CLOCKS_PER_SEC: libc::c_int = 1000000 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>() == 8) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> u16 {
    return *(ptr as *const unalign16);
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> u32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> u64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_readST(mut ptr: *const libc::c_void) -> libc::size_t {
    return *(ptr as *const unalignArch);
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> u32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr);
    } else {
        return MEM_swap32(MEM_read32(memPtr));
    };
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
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0);
    return val.trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0);
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: u64) -> libc::c_uint {
    debug_assert!(val != 0);
    return (val as libc::c_ulonglong).trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros64(mut val: u64) -> libc::c_uint {
    debug_assert!(val != 0);
    return (val as libc::c_ulonglong).leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: libc::size_t) -> libc::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            return ZSTD_countTrailingZeros64(val) >> 3 as libc::c_int;
        } else {
            return ZSTD_countTrailingZeros32(val as u32) >> 3 as libc::c_int;
        }
    } else if MEM_64bits() != 0 {
        return ZSTD_countLeadingZeros64(val) >> 3 as libc::c_int;
    } else {
        return ZSTD_countLeadingZeros32(val as u32) >> 3 as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0);
    return (31).wrapping_sub(ZSTD_countLeadingZeros32(val));
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const libc::c_char, mut args: ...) {}
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
pub const HUF_WORKSPACE_SIZE: libc::c_int = ((8) << 10 as libc::c_int) + 512;
unsafe extern "C" fn ERR_getErrorCode(mut code: libc::size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    return (0).wrapping_sub(code) as ERR_enum;
}
unsafe extern "C" fn ERR_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
pub const ZSTD_MAGIC_DICTIONARY: libc::c_uint = 0xec30a437 as libc::c_uint;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
pub const HUF_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
pub const OffFSELog: libc::c_int = 8 as libc::c_int;
pub const MaxML: libc::c_int = 52 as libc::c_int;
pub const MLFSELog: libc::c_int = 9 as libc::c_int;
pub const FSE_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const LLFSELog: libc::c_int = 9 as libc::c_int;
pub const ZSTD_CLEVEL_DEFAULT: libc::c_int = 3 as libc::c_int;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
pub const ZSTD_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
static mut ZSTD_defaultCMem: ZSTD_customMem = unsafe {
    {
        let mut init = ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTD_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTD_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut libc::c_void,
        };
        init
    }
};
static mut repStartValue: [u32; 3] = [
    1 as libc::c_int as u32,
    4 as libc::c_int as u32,
    8 as libc::c_int as u32,
];
pub const ZDICT_DICTSIZE_MIN: libc::c_int = 256 as libc::c_int;
pub const ZDICT_CONTENTSIZE_MIN: libc::c_int = 128 as libc::c_int;
pub const NOISELENGTH: libc::c_int = 32 as libc::c_int;
static mut g_selectivity_default: u32 = 9 as libc::c_int as u32;
unsafe extern "C" fn ZDICT_clockSpan(mut nPrevious: clock_t) -> clock_t {
    return clock() - nPrevious;
}
unsafe extern "C" fn ZDICT_printHex(mut ptr: *const libc::c_void, mut length: libc::size_t) {
    let b = ptr as *const u8;
    let mut u: libc::size_t = 0;
    u = 0 as libc::c_int as libc::size_t;
    while u < length {
        let mut c = *b.offset(u as isize);
        if (c as libc::c_int) < 32 || c as libc::c_int > 126 {
            c = '.' as i32 as u8;
        }
        fprintf(
            stderr,
            b"%c\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
        );
        fflush(stderr);
        u = u.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_isError(mut errorCode: libc::size_t) -> libc::c_uint {
    return ERR_isError(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getErrorName(mut errorCode: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorName(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getDictID(
    mut dictBuffer: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::c_uint {
    if dictSize < 8 {
        return 0 as libc::c_int as libc::c_uint;
    }
    if MEM_readLE32(dictBuffer) != ZSTD_MAGIC_DICTIONARY {
        return 0 as libc::c_int as libc::c_uint;
    }
    return MEM_readLE32((dictBuffer as *const libc::c_char).offset(4) as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getDictHeaderSize(
    mut dictBuffer: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::size_t {
    let mut headerSize: libc::size_t = 0;
    if dictSize <= 8 || MEM_readLE32(dictBuffer) != ZSTD_MAGIC_DICTIONARY {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    let mut bs = malloc(::core::mem::size_of::<ZSTD_compressedBlockState_t>())
        as *mut ZSTD_compressedBlockState_t;
    let mut wksp = malloc(HUF_WORKSPACE_SIZE as libc::c_ulong) as *mut u32;
    if bs.is_null() || wksp.is_null() {
        headerSize = -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    } else {
        ZSTD_reset_compressedBlockState(bs);
        headerSize = ZSTD_loadCEntropy(bs, wksp as *mut libc::c_void, dictBuffer, dictSize);
    }
    free(bs as *mut libc::c_void);
    free(wksp as *mut libc::c_void);
    return headerSize;
}
unsafe extern "C" fn ZDICT_count(
    mut pIn: *const libc::c_void,
    mut pMatch: *const libc::c_void,
) -> libc::size_t {
    let pStart = pIn as *const libc::c_char;
    loop {
        let diff = MEM_readST(pMatch) ^ MEM_readST(pIn);
        if diff == 0 {
            pIn = (pIn as *const libc::c_char)
                .offset(::core::mem::size_of::<libc::size_t>() as isize)
                as *const libc::c_void;
            pMatch = (pMatch as *const libc::c_char)
                .offset(::core::mem::size_of::<libc::size_t>() as isize)
                as *const libc::c_void;
        } else {
            pIn = (pIn as *const libc::c_char).offset(ZSTD_NbCommonBytes(diff) as isize)
                as *const libc::c_void;
            return (pIn as *const libc::c_char).offset_from(pStart) as libc::c_long
                as libc::size_t;
        }
    }
}
unsafe extern "C" fn ZDICT_initDictItem(mut d: *mut dictItem) {
    (*d).pos = 1 as libc::c_int as u32;
    (*d).length = 0 as libc::c_int as u32;
    (*d).savings = -(1) as u32;
}
pub const LLIMIT: libc::c_int = 64 as libc::c_int;
pub const MINMATCHLENGTH: libc::c_int = 7 as libc::c_int;
unsafe extern "C" fn ZDICT_analyzePos(
    mut doneMarks: *mut u8,
    mut suffix: *const libc::c_int,
    mut start: u32,
    mut buffer: *const libc::c_void,
    mut minRatio: u32,
    mut notificationLevel: u32,
) -> dictItem {
    let mut lengthList: [u32; 64] = [
        0 as libc::c_int as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut cumulLength: [u32; 64] = [
        0 as libc::c_int as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut savings: [u32; 64] = [
        0 as libc::c_int as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut b = buffer as *const u8;
    let mut maxLength = LLIMIT as libc::size_t;
    let mut pos = *suffix.offset(start as isize) as libc::size_t;
    let mut end = start;
    let mut solution = dictItem {
        pos: 0,
        length: 0,
        savings: 0,
    };
    memset(
        &mut solution as *mut dictItem as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<dictItem>(),
    );
    *doneMarks.offset(pos as isize) = 1 as libc::c_int as u8;
    if MEM_read16(b.offset(pos as isize).offset(0) as *const libc::c_void) as libc::c_int
        == MEM_read16(b.offset(pos as isize).offset(2) as *const libc::c_void) as libc::c_int
        || MEM_read16(b.offset(pos as isize).offset(1) as *const libc::c_void) as libc::c_int
            == MEM_read16(b.offset(pos as isize).offset(3) as *const libc::c_void) as libc::c_int
        || MEM_read16(b.offset(pos as isize).offset(2) as *const libc::c_void) as libc::c_int
            == MEM_read16(b.offset(pos as isize).offset(4) as *const libc::c_void) as libc::c_int
    {
        let pattern16 = MEM_read16(b.offset(pos as isize).offset(4) as *const libc::c_void);
        let mut u: u32 = 0;
        let mut patternEnd = 6 as libc::c_int as u32;
        while MEM_read16(b.offset(pos as isize).offset(patternEnd as isize) as *const libc::c_void)
            as libc::c_int
            == pattern16 as libc::c_int
        {
            patternEnd = (patternEnd as libc::c_uint).wrapping_add(2);
        }
        if *b.offset(pos.wrapping_add(patternEnd as libc::c_ulong) as isize) as libc::c_int
            == *b.offset(
                pos.wrapping_add(patternEnd as libc::c_ulong)
                    .wrapping_sub(1) as isize,
            ) as libc::c_int
        {
            patternEnd = patternEnd.wrapping_add(1);
        }
        u = 1 as libc::c_int as u32;
        while u < patternEnd {
            *doneMarks.offset(pos.wrapping_add(u as libc::c_ulong) as isize) =
                1 as libc::c_int as u8;
            u = u.wrapping_add(1);
        }
        return solution;
    }
    let mut length: libc::size_t = 0;
    loop {
        end = end.wrapping_add(1);
        length = ZDICT_count(
            b.offset(pos as isize) as *const libc::c_void,
            b.offset(*suffix.offset(end as isize) as isize) as *const libc::c_void,
        );
        if !(length >= MINMATCHLENGTH as libc::c_ulong) {
            break;
        }
    }
    let mut length_0: libc::size_t = 0;
    loop {
        length_0 = ZDICT_count(
            b.offset(pos as isize) as *const libc::c_void,
            b.offset(*suffix.offset(start as isize).offset(-(1)) as isize) as *const libc::c_void,
        );
        if length_0 >= MINMATCHLENGTH as libc::c_ulong {
            start = start.wrapping_sub(1);
        }
        if !(length_0 >= MINMATCHLENGTH as libc::c_ulong) {
            break;
        }
    }
    if end.wrapping_sub(start) < minRatio {
        let mut idx: u32 = 0;
        idx = start;
        while idx < end {
            *doneMarks.offset(*suffix.offset(idx as isize) as isize) = 1 as libc::c_int as u8;
            idx = idx.wrapping_add(1);
        }
        return solution;
    }
    let mut i: libc::c_int = 0;
    let mut mml: u32 = 0;
    let mut refinedStart = start;
    let mut refinedEnd = end;
    if notificationLevel >= 4 {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    if notificationLevel >= 4 {
        fprintf(
            stderr,
            b"found %3u matches of length >= %i at pos %7u  \0" as *const u8 as *const libc::c_char,
            end.wrapping_sub(start),
            7 as libc::c_int,
            pos as libc::c_uint,
        );
        fflush(stderr);
    }
    if notificationLevel >= 4 {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    mml = MINMATCHLENGTH as u32;
    loop {
        let mut currentChar = 0 as libc::c_int as u8;
        let mut currentCount = 0 as libc::c_int as u32;
        let mut currentID = refinedStart;
        let mut id: u32 = 0;
        let mut selectedCount = 0 as libc::c_int as u32;
        let mut selectedID = currentID;
        id = refinedStart;
        while id < refinedEnd {
            if *b.offset((*suffix.offset(id as isize) as libc::c_uint).wrapping_add(mml) as isize)
                as libc::c_int
                != currentChar as libc::c_int
            {
                if currentCount > selectedCount {
                    selectedCount = currentCount;
                    selectedID = currentID;
                }
                currentID = id;
                currentChar = *b.offset(
                    (*suffix.offset(id as isize) as libc::c_uint).wrapping_add(mml) as isize,
                );
                currentCount = 0 as libc::c_int as u32;
            }
            currentCount = currentCount.wrapping_add(1);
            id = id.wrapping_add(1);
        }
        if currentCount > selectedCount {
            selectedCount = currentCount;
            selectedID = currentID;
        }
        if selectedCount < minRatio {
            break;
        }
        refinedStart = selectedID;
        refinedEnd = refinedStart.wrapping_add(selectedCount);
        mml = mml.wrapping_add(1);
    }
    start = refinedStart;
    pos = *suffix.offset(refinedStart as isize) as libc::size_t;
    end = start;
    memset(
        lengthList.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[u32; 64]>(),
    );
    let mut length_1: libc::size_t = 0;
    loop {
        end = end.wrapping_add(1);
        length_1 = ZDICT_count(
            b.offset(pos as isize) as *const libc::c_void,
            b.offset(*suffix.offset(end as isize) as isize) as *const libc::c_void,
        );
        if length_1 >= LLIMIT as libc::c_ulong {
            length_1 = (LLIMIT - 1 as libc::c_int) as libc::size_t;
        }
        lengthList[length_1 as usize] = (lengthList[length_1 as usize]).wrapping_add(1);
        if !(length_1 >= MINMATCHLENGTH as libc::c_ulong) {
            break;
        }
    }
    let mut length_2 = MINMATCHLENGTH as libc::size_t;
    while (length_2 >= MINMATCHLENGTH as libc::c_ulong) as libc::c_int & (start > 0) as libc::c_int
        != 0
    {
        length_2 = ZDICT_count(
            b.offset(pos as isize) as *const libc::c_void,
            b.offset(*suffix.offset(start.wrapping_sub(1) as isize) as isize)
                as *const libc::c_void,
        );
        if length_2 >= LLIMIT as libc::c_ulong {
            length_2 = (LLIMIT - 1 as libc::c_int) as libc::size_t;
        }
        lengthList[length_2 as usize] = (lengthList[length_2 as usize]).wrapping_add(1);
        if length_2 >= MINMATCHLENGTH as libc::c_ulong {
            start = start.wrapping_sub(1);
        }
    }
    memset(
        cumulLength.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[u32; 64]>(),
    );
    cumulLength[maxLength.wrapping_sub(1) as usize] =
        lengthList[maxLength.wrapping_sub(1) as usize];
    i = maxLength.wrapping_sub(2) as libc::c_int;
    while i >= 0 {
        cumulLength[i as usize] =
            (cumulLength[(i + 1) as usize]).wrapping_add(lengthList[i as usize]);
        i -= 1;
    }
    i = LLIMIT - 1 as libc::c_int;
    while i >= MINMATCHLENGTH {
        if cumulLength[i as usize] >= minRatio {
            break;
        }
        i -= 1;
    }
    maxLength = i as libc::size_t;
    let mut l = maxLength as u32;
    let c = *b.offset(pos.wrapping_add(maxLength).wrapping_sub(1) as isize);
    while *b.offset(pos.wrapping_add(l as libc::c_ulong).wrapping_sub(2) as isize) as libc::c_int
        == c as libc::c_int
    {
        l = l.wrapping_sub(1);
    }
    maxLength = l as libc::size_t;
    if maxLength < MINMATCHLENGTH as libc::c_ulong {
        return solution;
    }
    savings[5 as libc::c_int as usize] = 0 as libc::c_int as u32;
    i = MINMATCHLENGTH;
    while i <= maxLength as libc::c_int {
        savings[i as usize] = (savings[(i - 1 as libc::c_int) as usize]).wrapping_add(
            (lengthList[i as usize]).wrapping_mul((i - 3 as libc::c_int) as libc::c_uint),
        );
        i += 1;
    }
    if notificationLevel >= 4 {
        fprintf(
            stderr,
            b"Selected dict at position %u, of length %u : saves %u (ratio: %.2f)  \n\0"
                as *const u8 as *const libc::c_char,
            pos as libc::c_uint,
            maxLength as libc::c_uint,
            savings[maxLength as usize],
            savings[maxLength as usize] as libc::c_double / maxLength as libc::c_double,
        );
        fflush(stderr);
    }
    solution.pos = pos as u32;
    solution.length = maxLength as u32;
    solution.savings = savings[maxLength as usize];
    let mut id_0: u32 = 0;
    id_0 = start;
    while id_0 < end {
        let mut p: u32 = 0;
        let mut pEnd: u32 = 0;
        let mut length_3: u32 = 0;
        let testedPos = *suffix.offset(id_0 as isize) as u32;
        if testedPos as libc::c_ulong == pos {
            length_3 = solution.length;
        } else {
            length_3 = ZDICT_count(
                b.offset(pos as isize) as *const libc::c_void,
                b.offset(testedPos as isize) as *const libc::c_void,
            ) as u32;
            if length_3 > solution.length {
                length_3 = solution.length;
            }
        }
        pEnd = testedPos.wrapping_add(length_3);
        p = testedPos;
        while p < pEnd {
            *doneMarks.offset(p as isize) = 1 as libc::c_int as u8;
            p = p.wrapping_add(1);
        }
        id_0 = id_0.wrapping_add(1);
    }
    return solution;
}
unsafe extern "C" fn isIncluded(
    mut in_0: *const libc::c_void,
    mut container: *const libc::c_void,
    mut length: libc::size_t,
) -> libc::c_int {
    let ip = in_0 as *const libc::c_char;
    let into = container as *const libc::c_char;
    let mut u: libc::size_t = 0;
    u = 0 as libc::c_int as libc::size_t;
    while u < length {
        if *ip.offset(u as isize) as libc::c_int != *into.offset(u as isize) as libc::c_int {
            break;
        }
        u = u.wrapping_add(1);
    }
    return (u == length) as libc::c_int;
}
unsafe extern "C" fn ZDICT_tryMerge(
    mut table: *mut dictItem,
    mut elt: dictItem,
    mut eltNbToSkip: u32,
    mut buffer: *const libc::c_void,
) -> u32 {
    let tableSize = (*table).pos;
    let eltEnd = (elt.pos).wrapping_add(elt.length);
    let buf = buffer as *const libc::c_char;
    let mut u: u32 = 0;
    u = 1 as libc::c_int as u32;
    while u < tableSize {
        if !(u == eltNbToSkip) {
            if (*table.offset(u as isize)).pos > elt.pos
                && (*table.offset(u as isize)).pos <= eltEnd
            {
                let addedLength = ((*table.offset(u as isize)).pos).wrapping_sub(elt.pos);
                let ref mut fresh0 = (*table.offset(u as isize)).length;
                *fresh0 = (*fresh0 as libc::c_uint).wrapping_add(addedLength) as u32 as u32;
                (*table.offset(u as isize)).pos = elt.pos;
                let ref mut fresh1 = (*table.offset(u as isize)).savings;
                *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(
                    (elt.savings)
                        .wrapping_mul(addedLength)
                        .wrapping_div(elt.length),
                );
                let ref mut fresh2 = (*table.offset(u as isize)).savings;
                *fresh2 = (*fresh2 as libc::c_uint).wrapping_add((elt.length).wrapping_div(8));
                elt = *table.offset(u as isize);
                while u > 1 && (*table.offset(u.wrapping_sub(1) as isize)).savings < elt.savings {
                    *table.offset(u as isize) = *table.offset(u.wrapping_sub(1) as isize);
                    u = u.wrapping_sub(1);
                }
                *table.offset(u as isize) = elt;
                return u;
            }
        }
        u = u.wrapping_add(1);
    }
    u = 1 as libc::c_int as u32;
    while u < tableSize {
        if !(u == eltNbToSkip) {
            if ((*table.offset(u as isize)).pos).wrapping_add((*table.offset(u as isize)).length)
                >= elt.pos
                && (*table.offset(u as isize)).pos < elt.pos
            {
                let addedLength_0 = eltEnd as libc::c_int
                    - ((*table.offset(u as isize)).pos)
                        .wrapping_add((*table.offset(u as isize)).length)
                        as libc::c_int;
                let ref mut fresh3 = (*table.offset(u as isize)).savings;
                *fresh3 = (*fresh3 as libc::c_uint).wrapping_add((elt.length).wrapping_div(8));
                if addedLength_0 > 0 {
                    let ref mut fresh4 = (*table.offset(u as isize)).length;
                    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(addedLength_0 as libc::c_uint);
                    let ref mut fresh5 = (*table.offset(u as isize)).savings;
                    *fresh5 = (*fresh5 as libc::c_uint).wrapping_add(
                        (elt.savings)
                            .wrapping_mul(addedLength_0 as libc::c_uint)
                            .wrapping_div(elt.length),
                    );
                }
                elt = *table.offset(u as isize);
                while u > 1 && (*table.offset(u.wrapping_sub(1) as isize)).savings < elt.savings {
                    *table.offset(u as isize) = *table.offset(u.wrapping_sub(1) as isize);
                    u = u.wrapping_sub(1);
                }
                *table.offset(u as isize) = elt;
                return u;
            }
            if MEM_read64(
                buf.offset((*table.offset(u as isize)).pos as isize) as *const libc::c_void
            ) == MEM_read64(buf.offset(elt.pos as isize).offset(1) as *const libc::c_void)
            {
                if isIncluded(
                    buf.offset((*table.offset(u as isize)).pos as isize) as *const libc::c_void,
                    buf.offset(elt.pos as isize).offset(1) as *const libc::c_void,
                    (*table.offset(u as isize)).length as libc::size_t,
                ) != 0
                {
                    let addedLength_1 = (if elt.length as libc::c_int
                        - (*table.offset(u as isize)).length as libc::c_int
                        > 1
                    {
                        elt.length as libc::c_int
                            - (*table.offset(u as isize)).length as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::size_t;
                    (*table.offset(u as isize)).pos = elt.pos;
                    let ref mut fresh6 = (*table.offset(u as isize)).savings;
                    *fresh6 = (*fresh6 as libc::c_uint).wrapping_add(
                        (elt.savings as libc::c_ulong)
                            .wrapping_mul(addedLength_1)
                            .wrapping_div(elt.length as libc::c_ulong)
                            as u32,
                    );
                    (*table.offset(u as isize)).length =
                        if elt.length < ((*table.offset(u as isize)).length).wrapping_add(1) {
                            elt.length
                        } else {
                            ((*table.offset(u as isize)).length).wrapping_add(1)
                        };
                    return u;
                }
            }
        }
        u = u.wrapping_add(1);
    }
    return 0 as libc::c_int as u32;
}
unsafe extern "C" fn ZDICT_removeDictItem(mut table: *mut dictItem, mut id: u32) {
    let max = (*table.offset(0)).pos;
    let mut u: u32 = 0;
    if id == 0 {
        return;
    }
    u = id;
    while u < max.wrapping_sub(1) {
        *table.offset(u as isize) = *table.offset(u.wrapping_add(1) as isize);
        u = u.wrapping_add(1);
    }
    (*table).pos = ((*table).pos).wrapping_sub(1);
}
unsafe extern "C" fn ZDICT_insertDictItem(
    mut table: *mut dictItem,
    mut maxSize: u32,
    mut elt: dictItem,
    mut buffer: *const libc::c_void,
) {
    let mut mergeId = ZDICT_tryMerge(table, elt, 0 as libc::c_int as u32, buffer);
    if mergeId != 0 {
        let mut newMerge = 1 as libc::c_int as u32;
        while newMerge != 0 {
            newMerge = ZDICT_tryMerge(table, *table.offset(mergeId as isize), mergeId, buffer);
            if newMerge != 0 {
                ZDICT_removeDictItem(table, mergeId);
            }
            mergeId = newMerge;
        }
        return;
    }
    let mut current: u32 = 0;
    let mut nextElt = (*table).pos;
    if nextElt >= maxSize {
        nextElt = maxSize.wrapping_sub(1);
    }
    current = nextElt.wrapping_sub(1);
    while (*table.offset(current as isize)).savings < elt.savings {
        *table.offset(current.wrapping_add(1) as isize) = *table.offset(current as isize);
        current = current.wrapping_sub(1);
    }
    *table.offset(current.wrapping_add(1) as isize) = elt;
    (*table).pos = nextElt.wrapping_add(1);
}
unsafe extern "C" fn ZDICT_dictSize(mut dictList: *const dictItem) -> u32 {
    let mut u: u32 = 0;
    let mut dictSize = 0 as libc::c_int as u32;
    u = 1 as libc::c_int as u32;
    while u < (*dictList.offset(0)).pos {
        dictSize = (dictSize as libc::c_uint).wrapping_add((*dictList.offset(u as isize)).length);
        u = u.wrapping_add(1);
    }
    return dictSize;
}
unsafe extern "C" fn ZDICT_trainBuffer_legacy(
    mut dictList: *mut dictItem,
    mut dictListSize: u32,
    buffer: *const libc::c_void,
    mut bufferSize: libc::size_t,
    mut fileSizes: *const libc::size_t,
    mut nbFiles: libc::c_uint,
    mut minRatio: libc::c_uint,
    mut notificationLevel: u32,
) -> libc::size_t {
    let suffix0 = malloc(
        bufferSize
            .wrapping_add(2)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>()),
    ) as *mut libc::c_int;
    let suffix = suffix0.offset(1);
    let mut reverseSuffix =
        malloc(bufferSize.wrapping_mul(::core::mem::size_of::<u32>())) as *mut u32;
    let mut doneMarks = malloc(
        bufferSize
            .wrapping_add(16)
            .wrapping_mul(::core::mem::size_of::<u8>()),
    ) as *mut u8;
    let mut filePos =
        malloc((nbFiles as libc::c_ulong).wrapping_mul(::core::mem::size_of::<u32>())) as *mut u32;
    let mut result = 0 as libc::c_int as libc::size_t;
    let mut displayClock = 0 as libc::c_int as clock_t;
    let refreshRate = CLOCKS_PER_SEC as __clock_t * 3 / 10 as libc::c_int as libc::c_long;
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"\r%70s\r\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    if suffix0.is_null() || reverseSuffix.is_null() || doneMarks.is_null() || filePos.is_null() {
        result = -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    } else {
        if minRatio < MINRATIO as libc::c_uint {
            minRatio = MINRATIO as libc::c_uint;
        }
        memset(
            doneMarks as *mut libc::c_void,
            0 as libc::c_int,
            bufferSize.wrapping_add(16),
        );
        if bufferSize > ZDICT_MAX_SAMPLES_SIZE as libc::c_ulong {
            if notificationLevel >= 3 {
                fprintf(
                    stderr,
                    b"sample set too large : reduced to %u MB ...\n\0" as *const u8
                        as *const libc::c_char,
                    (2000) << 20 as libc::c_int >> 20 as libc::c_int,
                );
                fflush(stderr);
            }
        }
        while bufferSize > ZDICT_MAX_SAMPLES_SIZE as libc::c_ulong {
            nbFiles = nbFiles.wrapping_sub(1);
            bufferSize =
                (bufferSize as libc::c_ulong).wrapping_sub(*fileSizes.offset(nbFiles as isize));
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"sorting %u files of total size %u MB ...\n\0" as *const u8 as *const libc::c_char,
                nbFiles,
                (bufferSize >> 20 as libc::c_int) as libc::c_uint,
            );
            fflush(stderr);
        }
        let divSuftSortResult = divsufsort(
            buffer as *const libc::c_uchar,
            suffix,
            bufferSize as libc::c_int,
            0 as libc::c_int,
        );
        if divSuftSortResult != 0 {
            result = -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
        } else {
            *suffix.offset(bufferSize as isize) = bufferSize as libc::c_int;
            *suffix0.offset(0) = bufferSize as libc::c_int;
            let mut pos: libc::size_t = 0;
            pos = 0 as libc::c_int as libc::size_t;
            while pos < bufferSize {
                *reverseSuffix.offset(*suffix.offset(pos as isize) as isize) = pos as u32;
                pos = pos.wrapping_add(1);
            }
            *filePos.offset(0) = 0 as libc::c_int as u32;
            pos = 1 as libc::c_int as libc::size_t;
            while pos < nbFiles as libc::c_ulong {
                *filePos.offset(pos as isize) = (*filePos.offset(pos.wrapping_sub(1) as isize)
                    as libc::c_ulong)
                    .wrapping_add(*fileSizes.offset(pos.wrapping_sub(1) as isize))
                    as u32;
                pos = pos.wrapping_add(1);
            }
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"finding patterns ... \n\0" as *const u8 as *const libc::c_char,
                );
                fflush(stderr);
            }
            if notificationLevel >= 3 {
                fprintf(
                    stderr,
                    b"minimum ratio : %u \n\0" as *const u8 as *const libc::c_char,
                    minRatio,
                );
                fflush(stderr);
            }
            let mut cursor: u32 = 0;
            cursor = 0 as libc::c_int as u32;
            while (cursor as libc::c_ulong) < bufferSize {
                let mut solution = dictItem {
                    pos: 0,
                    length: 0,
                    savings: 0,
                };
                if *doneMarks.offset(cursor as isize) != 0 {
                    cursor = cursor.wrapping_add(1);
                } else {
                    solution = ZDICT_analyzePos(
                        doneMarks,
                        suffix,
                        *reverseSuffix.offset(cursor as isize),
                        buffer,
                        minRatio,
                        notificationLevel,
                    );
                    if solution.length == 0 {
                        cursor = cursor.wrapping_add(1);
                    } else {
                        ZDICT_insertDictItem(dictList, dictListSize, solution, buffer);
                        cursor = (cursor as libc::c_uint).wrapping_add(solution.length);
                        if notificationLevel >= 2 {
                            if ZDICT_clockSpan(displayClock) > refreshRate {
                                displayClock = clock();
                                fprintf(
                                    stderr,
                                    b"\r%4.2f %% \r\0" as *const u8 as *const libc::c_char,
                                    cursor as libc::c_double / bufferSize as libc::c_double
                                        * 100.0f64,
                                );
                                fflush(stderr);
                                if notificationLevel >= 4 {
                                    fflush(stderr);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(suffix0 as *mut libc::c_void);
    free(reverseSuffix as *mut libc::c_void);
    free(doneMarks as *mut libc::c_void);
    free(filePos as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn ZDICT_fillNoise(mut buffer: *mut libc::c_void, mut length: libc::size_t) {
    let prime1 = 2654435761 as libc::c_uint;
    let prime2 = 2246822519 as libc::c_uint;
    let mut acc = prime1;
    let mut p = 0 as libc::c_int as libc::size_t;
    p = 0 as libc::c_int as libc::size_t;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut libc::c_uchar).offset(p as isize) =
            (acc >> 21 as libc::c_int) as libc::c_uchar;
        p = p.wrapping_add(1);
    }
}
pub const MAXREPOFFSET: libc::c_int = 1024 as libc::c_int;
unsafe extern "C" fn ZDICT_countEStats(
    mut esr: EStats_ress_t,
    mut params: *const ZSTD_parameters,
    mut countLit: *mut libc::c_uint,
    mut offsetcodeCount: *mut libc::c_uint,
    mut matchlengthCount: *mut libc::c_uint,
    mut litlengthCount: *mut libc::c_uint,
    mut repOffsets: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut notificationLevel: u32,
) {
    let blockSizeMax = (if ((1) << 17 as libc::c_int) < (1) << (*params).cParams.windowLog {
        (1) << 17 as libc::c_int
    } else {
        (1) << (*params).cParams.windowLog
    }) as libc::size_t;
    let mut cSize: libc::size_t = 0;
    if srcSize > blockSizeMax {
        srcSize = blockSizeMax;
    }
    let errorCode = ZSTD_compressBegin_usingCDict_deprecated(esr.zc, esr.dict);
    if ERR_isError(errorCode) != 0 {
        if notificationLevel >= 1 {
            fprintf(
                stderr,
                b"warning : ZSTD_compressBegin_usingCDict failed \n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
        return;
    }
    cSize = ZSTD_compressBlock_deprecated(
        esr.zc,
        esr.workPlace,
        ZSTD_BLOCKSIZE_MAX as libc::size_t,
        src,
        srcSize,
    );
    if ERR_isError(cSize) != 0 {
        if notificationLevel >= 3 {
            fprintf(
                stderr,
                b"warning : could not compress sample size %u \n\0" as *const u8
                    as *const libc::c_char,
                srcSize as libc::c_uint,
            );
            fflush(stderr);
        }
        return;
    }
    if cSize != 0 {
        let seqStorePtr = ZSTD_getSeqStore(esr.zc);
        let mut bytePtr = 0 as *const u8;
        bytePtr = (*seqStorePtr).litStart;
        while bytePtr < (*seqStorePtr).lit as *const u8 {
            let ref mut fresh7 = *countLit.offset(*bytePtr as isize);
            *fresh7 = (*fresh7).wrapping_add(1);
            bytePtr = bytePtr.offset(1);
        }
        let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
            as libc::c_long as u32;
        ZSTD_seqToCodes(seqStorePtr);
        let mut codePtr: *const u8 = (*seqStorePtr).ofCode;
        let mut u: u32 = 0;
        u = 0 as libc::c_int as u32;
        while u < nbSeq {
            let ref mut fresh8 = *offsetcodeCount.offset(*codePtr.offset(u as isize) as isize);
            *fresh8 = (*fresh8).wrapping_add(1);
            u = u.wrapping_add(1);
        }
        let mut codePtr_0: *const u8 = (*seqStorePtr).mlCode;
        let mut u_0: u32 = 0;
        u_0 = 0 as libc::c_int as u32;
        while u_0 < nbSeq {
            let ref mut fresh9 = *matchlengthCount.offset(*codePtr_0.offset(u_0 as isize) as isize);
            *fresh9 = (*fresh9).wrapping_add(1);
            u_0 = u_0.wrapping_add(1);
        }
        let mut codePtr_1: *const u8 = (*seqStorePtr).llCode;
        let mut u_1: u32 = 0;
        u_1 = 0 as libc::c_int as u32;
        while u_1 < nbSeq {
            let ref mut fresh10 = *litlengthCount.offset(*codePtr_1.offset(u_1 as isize) as isize);
            *fresh10 = (*fresh10).wrapping_add(1);
            u_1 = u_1.wrapping_add(1);
        }
        if nbSeq >= 2 {
            let seq: *const seqDef = (*seqStorePtr).sequencesStart;
            let mut offset1 = ((*seq.offset(0)).offBase).wrapping_sub(ZSTD_REP_NUM as libc::c_uint);
            let mut offset2 = ((*seq.offset(1)).offBase).wrapping_sub(ZSTD_REP_NUM as libc::c_uint);
            if offset1 >= MAXREPOFFSET as libc::c_uint {
                offset1 = 0 as libc::c_int as u32;
            }
            if offset2 >= MAXREPOFFSET as libc::c_uint {
                offset2 = 0 as libc::c_int as u32;
            }
            let ref mut fresh11 = *repOffsets.offset(offset1 as isize);
            *fresh11 = (*fresh11 as libc::c_uint).wrapping_add(3);
            let ref mut fresh12 = *repOffsets.offset(offset2 as isize);
            *fresh12 = (*fresh12 as libc::c_uint).wrapping_add(1);
        }
    }
}
unsafe extern "C" fn ZDICT_totalSampleSize(
    mut fileSizes: *const libc::size_t,
    mut nbFiles: libc::c_uint,
) -> libc::size_t {
    let mut total = 0 as libc::c_int as libc::size_t;
    let mut u: libc::c_uint = 0;
    u = 0 as libc::c_int as libc::c_uint;
    while u < nbFiles {
        total = (total as libc::c_ulong).wrapping_add(*fileSizes.offset(u as isize));
        u = u.wrapping_add(1);
    }
    return total;
}
unsafe extern "C" fn ZDICT_insertSortCount(
    mut table: *mut offsetCount_t,
    mut val: u32,
    mut count: u32,
) {
    let mut u: u32 = 0;
    (*table.offset(ZSTD_REP_NUM as isize)).offset = val;
    (*table.offset(ZSTD_REP_NUM as isize)).count = count;
    u = ZSTD_REP_NUM as u32;
    while u > 0 {
        let mut tmp = offsetCount_t {
            offset: 0,
            count: 0,
        };
        if (*table.offset(u.wrapping_sub(1) as isize)).count >= (*table.offset(u as isize)).count {
            break;
        }
        tmp = *table.offset(u.wrapping_sub(1) as isize);
        *table.offset(u.wrapping_sub(1) as isize) = *table.offset(u as isize);
        *table.offset(u as isize) = tmp;
        u = u.wrapping_sub(1);
    }
}
unsafe extern "C" fn ZDICT_flatLit(mut countLit: *mut libc::c_uint) {
    let mut u: libc::c_int = 0;
    u = 1 as libc::c_int;
    while u < 256 {
        *countLit.offset(u as isize) = 2 as libc::c_int as libc::c_uint;
        u += 1;
    }
    *countLit.offset(0) = 4 as libc::c_int as libc::c_uint;
    *countLit.offset(253) = 1 as libc::c_int as libc::c_uint;
    *countLit.offset(254) = 1 as libc::c_int as libc::c_uint;
}
pub const OFFCODE_MAX: libc::c_int = 30 as libc::c_int;
unsafe extern "C" fn ZDICT_analyzeEntropy(
    mut dstBuffer: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut compressionLevel: libc::c_int,
    mut srcBuffer: *const libc::c_void,
    mut fileSizes: *const libc::size_t,
    mut nbFiles: libc::c_uint,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: libc::size_t,
    mut notificationLevel: libc::c_uint,
) -> libc::size_t {
    let mut countLit: [libc::c_uint; 256] = [0; 256];
    let mut hufTable: [HUF_CElt; 257] = [0; 257];
    let mut offcodeCount: [libc::c_uint; 31] = [0; 31];
    let mut offcodeNCount: [libc::c_short; 31] = [0; 31];
    let mut offcodeMax = ZSTD_highbit32(
        dictBufferSize
            .wrapping_add((128 as libc::c_int * ((1) << 10 as libc::c_int)) as libc::c_ulong)
            as u32,
    );
    let mut matchLengthCount: [libc::c_uint; 53] = [0; 53];
    let mut matchLengthNCount: [libc::c_short; 53] = [0; 53];
    let mut litLengthCount: [libc::c_uint; 36] = [0; 36];
    let mut litLengthNCount: [libc::c_short; 36] = [0; 36];
    let mut repOffset: [u32; 1024] = [0; 1024];
    let mut bestRepOffset: [offsetCount_t; 4] = [offsetCount_t {
        offset: 0,
        count: 0,
    }; 4];
    let mut esr = {
        let mut init = EStats_ress_t {
            dict: NULL as *mut ZSTD_CDict,
            zc: NULL as *mut ZSTD_CCtx,
            workPlace: NULL as *mut libc::c_void,
        };
        init
    };
    let mut params = ZSTD_parameters {
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0 as ZSTD_strategy,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
    };
    let mut u: u32 = 0;
    let mut huffLog = 11 as libc::c_int as u32;
    let mut Offlog = OffFSELog as u32;
    let mut mlLog = MLFSELog as u32;
    let mut llLog = LLFSELog as u32;
    let mut total: u32 = 0;
    let mut pos = 0 as libc::c_int as libc::size_t;
    let mut errorCode: libc::size_t = 0;
    let mut eSize = 0 as libc::c_int as libc::size_t;
    let totalSrcSize = ZDICT_totalSampleSize(fileSizes, nbFiles);
    let averageSampleSize = totalSrcSize.wrapping_div(
        nbFiles.wrapping_add((nbFiles == 0) as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    let mut dstPtr = dstBuffer as *mut u8;
    let mut wksp: [u32; 1216] = [0; 1216];
    if offcodeMax > OFFCODE_MAX as libc::c_uint {
        eSize = -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as libc::size_t;
    } else {
        u = 0 as libc::c_int as u32;
        while u < 256 {
            countLit[u as usize] = 1 as libc::c_int as libc::c_uint;
            u = u.wrapping_add(1);
        }
        u = 0 as libc::c_int as u32;
        while u <= offcodeMax {
            offcodeCount[u as usize] = 1 as libc::c_int as libc::c_uint;
            u = u.wrapping_add(1);
        }
        u = 0 as libc::c_int as u32;
        while u <= MaxML as libc::c_uint {
            matchLengthCount[u as usize] = 1 as libc::c_int as libc::c_uint;
            u = u.wrapping_add(1);
        }
        u = 0 as libc::c_int as u32;
        while u <= MaxLL as libc::c_uint {
            litLengthCount[u as usize] = 1 as libc::c_int as libc::c_uint;
            u = u.wrapping_add(1);
        }
        memset(
            repOffset.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[u32; 1024]>(),
        );
        repOffset[8 as libc::c_int as usize] = 1 as libc::c_int as u32;
        repOffset[4 as libc::c_int as usize] = repOffset[8 as libc::c_int as usize];
        repOffset[1 as libc::c_int as usize] = repOffset[4 as libc::c_int as usize];
        memset(
            bestRepOffset.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[offsetCount_t; 4]>(),
        );
        if compressionLevel == 0 {
            compressionLevel = ZSTD_CLEVEL_DEFAULT;
        }
        params = ZSTD_getParams(
            compressionLevel,
            averageSampleSize as libc::c_ulonglong,
            dictBufferSize,
        );
        esr.dict = ZSTD_createCDict_advanced(
            dictBuffer,
            dictBufferSize,
            ZSTD_dlm_byRef,
            ZSTD_dct_rawContent,
            params.cParams,
            ZSTD_defaultCMem,
        );
        esr.zc = ZSTD_createCCtx();
        esr.workPlace = malloc(ZSTD_BLOCKSIZE_MAX as libc::c_ulong);
        if (esr.dict).is_null() || (esr.zc).is_null() || (esr.workPlace).is_null() {
            eSize = -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
            if notificationLevel >= 1 {
                fprintf(
                    stderr,
                    b"Not enough memory \n\0" as *const u8 as *const libc::c_char,
                );
                fflush(stderr);
            }
        } else {
            u = 0 as libc::c_int as u32;
            while u < nbFiles {
                ZDICT_countEStats(
                    esr,
                    &mut params,
                    countLit.as_mut_ptr(),
                    offcodeCount.as_mut_ptr(),
                    matchLengthCount.as_mut_ptr(),
                    litLengthCount.as_mut_ptr(),
                    repOffset.as_mut_ptr(),
                    (srcBuffer as *const libc::c_char).offset(pos as isize) as *const libc::c_void,
                    *fileSizes.offset(u as isize),
                    notificationLevel,
                );
                pos = (pos as libc::c_ulong).wrapping_add(*fileSizes.offset(u as isize));
                u = u.wrapping_add(1);
            }
            if notificationLevel >= 4 {
                if notificationLevel >= 4 {
                    fprintf(
                        stderr,
                        b"Offset Code Frequencies : \n\0" as *const u8 as *const libc::c_char,
                    );
                    fflush(stderr);
                }
                u = 0 as libc::c_int as u32;
                while u <= offcodeMax {
                    if notificationLevel >= 4 {
                        fprintf(
                            stderr,
                            b"%2u :%7u \n\0" as *const u8 as *const libc::c_char,
                            u,
                            offcodeCount[u as usize],
                        );
                        fflush(stderr);
                    }
                    u = u.wrapping_add(1);
                }
            }
            let mut maxNbBits = HUF_buildCTable_wksp(
                hufTable.as_mut_ptr(),
                countLit.as_mut_ptr(),
                255 as libc::c_int as u32,
                huffLog,
                wksp.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[u32; 1216]>(),
            );
            if ERR_isError(maxNbBits) != 0 {
                eSize = maxNbBits;
                if notificationLevel >= 1 {
                    fprintf(
                        stderr,
                        b" HUF_buildCTable error \n\0" as *const u8 as *const libc::c_char,
                    );
                    fflush(stderr);
                }
            } else {
                if maxNbBits == 8 {
                    if notificationLevel >= 2 {
                        fprintf(
                            stderr,
                            b"warning : pathological dataset : literals are not compressible : samples are noisy or too regular \n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        fflush(stderr);
                    }
                    ZDICT_flatLit(countLit.as_mut_ptr());
                    maxNbBits = HUF_buildCTable_wksp(
                        hufTable.as_mut_ptr(),
                        countLit.as_mut_ptr(),
                        255 as libc::c_int as u32,
                        huffLog,
                        wksp.as_mut_ptr() as *mut libc::c_void,
                        ::core::mem::size_of::<[u32; 1216]>(),
                    );
                    debug_assert!(maxNbBits == 9);
                }
                huffLog = maxNbBits as u32;
                let mut offset: u32 = 0;
                offset = 1 as libc::c_int as u32;
                while offset < MAXREPOFFSET as libc::c_uint {
                    ZDICT_insertSortCount(
                        bestRepOffset.as_mut_ptr(),
                        offset,
                        repOffset[offset as usize],
                    );
                    offset = offset.wrapping_add(1);
                }
                total = 0 as libc::c_int as u32;
                u = 0 as libc::c_int as u32;
                while u <= offcodeMax {
                    total = (total as libc::c_uint).wrapping_add(offcodeCount[u as usize]);
                    u = u.wrapping_add(1);
                }
                errorCode = FSE_normalizeCount(
                    offcodeNCount.as_mut_ptr(),
                    Offlog,
                    offcodeCount.as_mut_ptr(),
                    total as libc::size_t,
                    offcodeMax,
                    1 as libc::c_int as libc::c_uint,
                );
                if ERR_isError(errorCode) != 0 {
                    eSize = errorCode;
                    if notificationLevel >= 1 {
                        fprintf(
                            stderr,
                            b"FSE_normalizeCount error with offcodeCount \n\0" as *const u8
                                as *const libc::c_char,
                        );
                        fflush(stderr);
                    }
                } else {
                    Offlog = errorCode as u32;
                    total = 0 as libc::c_int as u32;
                    u = 0 as libc::c_int as u32;
                    while u <= MaxML as libc::c_uint {
                        total = (total as libc::c_uint).wrapping_add(matchLengthCount[u as usize]);
                        u = u.wrapping_add(1);
                    }
                    errorCode = FSE_normalizeCount(
                        matchLengthNCount.as_mut_ptr(),
                        mlLog,
                        matchLengthCount.as_mut_ptr(),
                        total as libc::size_t,
                        MaxML as libc::c_uint,
                        1 as libc::c_int as libc::c_uint,
                    );
                    if ERR_isError(errorCode) != 0 {
                        eSize = errorCode;
                        if notificationLevel >= 1 {
                            fprintf(
                                stderr,
                                b"FSE_normalizeCount error with matchLengthCount \n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fflush(stderr);
                        }
                    } else {
                        mlLog = errorCode as u32;
                        total = 0 as libc::c_int as u32;
                        u = 0 as libc::c_int as u32;
                        while u <= MaxLL as libc::c_uint {
                            total =
                                (total as libc::c_uint).wrapping_add(litLengthCount[u as usize]);
                            u = u.wrapping_add(1);
                        }
                        errorCode = FSE_normalizeCount(
                            litLengthNCount.as_mut_ptr(),
                            llLog,
                            litLengthCount.as_mut_ptr(),
                            total as libc::size_t,
                            MaxLL as libc::c_uint,
                            1 as libc::c_int as libc::c_uint,
                        );
                        if ERR_isError(errorCode) != 0 {
                            eSize = errorCode;
                            if notificationLevel >= 1 {
                                fprintf(
                                    stderr,
                                    b"FSE_normalizeCount error with litLengthCount \n\0"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                fflush(stderr);
                            }
                        } else {
                            llLog = errorCode as u32;
                            let hhSize = HUF_writeCTable_wksp(
                                dstPtr as *mut libc::c_void,
                                maxDstSize,
                                hufTable.as_mut_ptr(),
                                255 as libc::c_int as libc::c_uint,
                                huffLog,
                                wksp.as_mut_ptr() as *mut libc::c_void,
                                ::core::mem::size_of::<[u32; 1216]>(),
                            );
                            if ERR_isError(hhSize) != 0 {
                                eSize = hhSize;
                                if notificationLevel >= 1 {
                                    fprintf(
                                        stderr,
                                        b"HUF_writeCTable error \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    fflush(stderr);
                                }
                            } else {
                                dstPtr = dstPtr.offset(hhSize as isize);
                                maxDstSize = (maxDstSize as libc::c_ulong).wrapping_sub(hhSize);
                                eSize = (eSize as libc::c_ulong).wrapping_add(hhSize);
                                let ohSize = FSE_writeNCount(
                                    dstPtr as *mut libc::c_void,
                                    maxDstSize,
                                    offcodeNCount.as_mut_ptr(),
                                    OFFCODE_MAX as libc::c_uint,
                                    Offlog,
                                );
                                if ERR_isError(ohSize) != 0 {
                                    eSize = ohSize;
                                    if notificationLevel >= 1 {
                                        fprintf(
                                            stderr,
                                            b"FSE_writeNCount error with offcodeNCount \n\0"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        fflush(stderr);
                                    }
                                } else {
                                    dstPtr = dstPtr.offset(ohSize as isize);
                                    maxDstSize = (maxDstSize as libc::c_ulong).wrapping_sub(ohSize);
                                    eSize = (eSize as libc::c_ulong).wrapping_add(ohSize);
                                    let mhSize = FSE_writeNCount(
                                        dstPtr as *mut libc::c_void,
                                        maxDstSize,
                                        matchLengthNCount.as_mut_ptr(),
                                        MaxML as libc::c_uint,
                                        mlLog,
                                    );
                                    if ERR_isError(mhSize) != 0 {
                                        eSize = mhSize;
                                        if notificationLevel >= 1 {
                                            fprintf(
                                                stderr,
                                                b"FSE_writeNCount error with matchLengthNCount \n\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                            );
                                            fflush(stderr);
                                        }
                                    } else {
                                        dstPtr = dstPtr.offset(mhSize as isize);
                                        maxDstSize =
                                            (maxDstSize as libc::c_ulong).wrapping_sub(mhSize);
                                        eSize = (eSize as libc::c_ulong).wrapping_add(mhSize);
                                        let lhSize = FSE_writeNCount(
                                            dstPtr as *mut libc::c_void,
                                            maxDstSize,
                                            litLengthNCount.as_mut_ptr(),
                                            MaxLL as libc::c_uint,
                                            llLog,
                                        );
                                        if ERR_isError(lhSize) != 0 {
                                            eSize = lhSize;
                                            if notificationLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"FSE_writeNCount error with litlengthNCount \n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                                fflush(stderr);
                                            }
                                        } else {
                                            dstPtr = dstPtr.offset(lhSize as isize);
                                            maxDstSize =
                                                (maxDstSize as libc::c_ulong).wrapping_sub(lhSize);
                                            eSize = (eSize as libc::c_ulong).wrapping_add(lhSize);
                                            if maxDstSize < 12 {
                                                eSize = -(ZSTD_error_dstSize_tooSmall
                                                    as libc::c_int)
                                                    as libc::size_t;
                                                if notificationLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"not enough space to write RepOffsets \n\0"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    fflush(stderr);
                                                }
                                            } else {
                                                MEM_writeLE32(
                                                    dstPtr.offset(0) as *mut libc::c_void,
                                                    repStartValue[0 as libc::c_int as usize],
                                                );
                                                MEM_writeLE32(
                                                    dstPtr.offset(4) as *mut libc::c_void,
                                                    repStartValue[1 as libc::c_int as usize],
                                                );
                                                MEM_writeLE32(
                                                    dstPtr.offset(8) as *mut libc::c_void,
                                                    repStartValue[2 as libc::c_int as usize],
                                                );
                                                eSize = (eSize as libc::c_ulong).wrapping_add(12)
                                                    as libc::size_t
                                                    as libc::size_t;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ZSTD_freeCDict(esr.dict);
    ZSTD_freeCCtx(esr.zc);
    free(esr.workPlace);
    return eSize;
}
unsafe extern "C" fn ZDICT_maxRep(mut reps: *const u32) -> u32 {
    let mut maxRep = *reps.offset(0);
    let mut r: libc::c_int = 0;
    r = 1 as libc::c_int;
    while r < ZSTD_REP_NUM {
        maxRep = if maxRep > *reps.offset(r as isize) {
            maxRep
        } else {
            *reps.offset(r as isize)
        };
        r += 1;
    }
    return maxRep;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_finalizeDictionary(
    mut dictBuffer: *mut libc::c_void,
    mut dictBufferCapacity: libc::size_t,
    mut customDictContent: *const libc::c_void,
    mut dictContentSize: libc::size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const libc::size_t,
    mut nbSamples: libc::c_uint,
    mut params: ZDICT_params_t,
) -> libc::size_t {
    let mut hSize: libc::size_t = 0;
    let mut header: [u8; 256] = [0; 256];
    let compressionLevel = if params.compressionLevel == 0 {
        ZSTD_CLEVEL_DEFAULT
    } else {
        params.compressionLevel
    };
    let notificationLevel = params.notificationLevel;
    let minContentSize = ZDICT_maxRep(repStartValue.as_ptr()) as libc::size_t;
    let mut paddingSize: libc::size_t = 0;
    if dictBufferCapacity < dictContentSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    MEM_writeLE32(
        header.as_mut_ptr() as *mut libc::c_void,
        ZSTD_MAGIC_DICTIONARY,
    );
    let randomID = ZSTD_XXH64(
        customDictContent,
        dictContentSize,
        0 as libc::c_int as XXH64_hash_t,
    );
    let compliantID = randomID
        .wrapping_rem(((1) << 31 as libc::c_int).wrapping_sub(32768) as libc::c_ulong)
        .wrapping_add(32768) as u32;
    let dictID = if params.dictID != 0 {
        params.dictID
    } else {
        compliantID
    };
    MEM_writeLE32(header.as_mut_ptr().offset(4) as *mut libc::c_void, dictID);
    hSize = 8 as libc::c_int as libc::size_t;
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"\r%70s\r\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"statistics ... \n\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    let eSize = ZDICT_analyzeEntropy(
        header.as_mut_ptr().offset(hSize as isize) as *mut libc::c_void,
        (HBUFFSIZE as libc::c_ulong).wrapping_sub(hSize),
        compressionLevel,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        customDictContent,
        dictContentSize,
        notificationLevel,
    );
    if ZDICT_isError(eSize) != 0 {
        return eSize;
    }
    hSize = (hSize as libc::c_ulong).wrapping_add(eSize);
    if hSize.wrapping_add(dictContentSize) > dictBufferCapacity {
        dictContentSize = dictBufferCapacity.wrapping_sub(hSize);
    }
    if dictContentSize < minContentSize {
        if hSize.wrapping_add(minContentSize) > dictBufferCapacity {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
        }
        paddingSize = minContentSize.wrapping_sub(dictContentSize);
    } else {
        paddingSize = 0 as libc::c_int as libc::size_t;
    }
    let dictSize = hSize
        .wrapping_add(paddingSize)
        .wrapping_add(dictContentSize);
    let outDictHeader = dictBuffer as *mut u8;
    let outDictPadding = outDictHeader.offset(hSize as isize);
    let outDictContent = outDictPadding.offset(paddingSize as isize);
    debug_assert!(dictSize <= dictBufferCapacity);
    debug_assert!(
        outDictContent.offset(dictContentSize as isize)
            == (dictBuffer as *mut u8).offset(dictSize as isize)
    );
    memmove(
        outDictContent as *mut libc::c_void,
        customDictContent,
        dictContentSize,
    );
    memcpy(
        outDictHeader as *mut libc::c_void,
        header.as_mut_ptr() as *const libc::c_void,
        hSize,
    );
    memset(
        outDictPadding as *mut libc::c_void,
        0 as libc::c_int,
        paddingSize,
    );
    return dictSize;
}
pub const HBUFFSIZE: libc::c_int = 256 as libc::c_int;
unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer_advanced(
    mut dictBuffer: *mut libc::c_void,
    mut dictContentSize: libc::size_t,
    mut dictBufferCapacity: libc::size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const libc::size_t,
    mut nbSamples: libc::c_uint,
    mut params: ZDICT_params_t,
) -> libc::size_t {
    let compressionLevel = if params.compressionLevel == 0 {
        ZSTD_CLEVEL_DEFAULT
    } else {
        params.compressionLevel
    };
    let notificationLevel = params.notificationLevel;
    let mut hSize = 8 as libc::c_int as libc::size_t;
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"\r%70s\r\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"statistics ... \n\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    let eSize = ZDICT_analyzeEntropy(
        (dictBuffer as *mut libc::c_char).offset(hSize as isize) as *mut libc::c_void,
        dictBufferCapacity.wrapping_sub(hSize),
        compressionLevel,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        (dictBuffer as *mut libc::c_char)
            .offset(dictBufferCapacity as isize)
            .offset(-(dictContentSize as isize)) as *const libc::c_void,
        dictContentSize,
        notificationLevel,
    );
    if ZDICT_isError(eSize) != 0 {
        return eSize;
    }
    hSize = (hSize as libc::c_ulong).wrapping_add(eSize);
    MEM_writeLE32(dictBuffer, ZSTD_MAGIC_DICTIONARY);
    let randomID = ZSTD_XXH64(
        (dictBuffer as *mut libc::c_char)
            .offset(dictBufferCapacity as isize)
            .offset(-(dictContentSize as isize)) as *const libc::c_void,
        dictContentSize,
        0 as libc::c_int as XXH64_hash_t,
    );
    let compliantID = randomID
        .wrapping_rem(((1) << 31 as libc::c_int).wrapping_sub(32768) as libc::c_ulong)
        .wrapping_add(32768) as u32;
    let dictID = if params.dictID != 0 {
        params.dictID
    } else {
        compliantID
    };
    MEM_writeLE32(
        (dictBuffer as *mut libc::c_char).offset(4) as *mut libc::c_void,
        dictID,
    );
    if hSize.wrapping_add(dictContentSize) < dictBufferCapacity {
        memmove(
            (dictBuffer as *mut libc::c_char).offset(hSize as isize) as *mut libc::c_void,
            (dictBuffer as *mut libc::c_char)
                .offset(dictBufferCapacity as isize)
                .offset(-(dictContentSize as isize)) as *const libc::c_void,
            dictContentSize,
        );
    }
    return if dictBufferCapacity < hSize.wrapping_add(dictContentSize) {
        dictBufferCapacity
    } else {
        hSize.wrapping_add(dictContentSize)
    };
}
unsafe extern "C" fn ZDICT_trainFromBuffer_unsafe_legacy(
    mut dictBuffer: *mut libc::c_void,
    mut maxDictSize: libc::size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const libc::size_t,
    mut nbSamples: libc::c_uint,
    mut params: ZDICT_legacy_params_t,
) -> libc::size_t {
    let dictListSize = if (if 10000 as libc::c_int as libc::c_uint > nbSamples {
        10000 as libc::c_int as libc::c_uint
    } else {
        nbSamples
    }) > maxDictSize.wrapping_div(16) as u32
    {
        if 10000 as libc::c_int as libc::c_uint > nbSamples {
            10000 as libc::c_int as libc::c_uint
        } else {
            nbSamples
        }
    } else {
        maxDictSize.wrapping_div(16) as u32
    };
    let dictList =
        malloc((dictListSize as libc::c_ulong).wrapping_mul(::core::mem::size_of::<dictItem>()))
            as *mut dictItem;
    let selectivity = if params.selectivityLevel == 0 {
        g_selectivity_default
    } else {
        params.selectivityLevel
    };
    let minRep = if selectivity > 30 {
        MINRATIO as libc::c_uint
    } else {
        nbSamples >> selectivity
    };
    let targetDictSize = maxDictSize;
    let samplesBuffSize = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    let mut dictSize = 0 as libc::c_int as libc::size_t;
    let notificationLevel = params.zParams.notificationLevel;
    if dictList.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    if maxDictSize < ZDICT_DICTSIZE_MIN as libc::c_ulong {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if samplesBuffSize < ZDICT_MIN_SAMPLES_SIZE as libc::c_ulong {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as libc::size_t;
    }
    ZDICT_initDictItem(dictList);
    ZDICT_trainBuffer_legacy(
        dictList,
        dictListSize,
        samplesBuffer,
        samplesBuffSize,
        samplesSizes,
        nbSamples,
        minRep,
        notificationLevel,
    );
    if params.zParams.notificationLevel >= 3 {
        let nb = if (25) < (*dictList.offset(0)).pos {
            25 as libc::c_int as libc::c_uint
        } else {
            (*dictList.offset(0)).pos
        };
        let dictContentSize = ZDICT_dictSize(dictList);
        let mut u: libc::c_uint = 0;
        if notificationLevel >= 3 {
            fprintf(
                stderr,
                b"\n %u segments found, of total size %u \n\0" as *const u8 as *const libc::c_char,
                ((*dictList.offset(0)).pos).wrapping_sub(1),
                dictContentSize,
            );
            fflush(stderr);
        }
        if notificationLevel >= 3 {
            fprintf(
                stderr,
                b"list %u best segments \n\0" as *const u8 as *const libc::c_char,
                nb.wrapping_sub(1),
            );
            fflush(stderr);
        }
        u = 1 as libc::c_int as libc::c_uint;
        while u < nb {
            let pos = (*dictList.offset(u as isize)).pos;
            let length = (*dictList.offset(u as isize)).length;
            let printedLength = if (40) < length {
                40 as libc::c_int as libc::c_uint
            } else {
                length
            };
            if pos as libc::c_ulong > samplesBuffSize
                || pos.wrapping_add(length) as libc::c_ulong > samplesBuffSize
            {
                free(dictList as *mut libc::c_void);
                return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
            }
            if notificationLevel >= 3 {
                fprintf(
                    stderr,
                    b"%3u:%3u bytes at pos %8u, savings %7u bytes |\0" as *const u8
                        as *const libc::c_char,
                    u,
                    length,
                    pos,
                    (*dictList.offset(u as isize)).savings,
                );
                fflush(stderr);
            }
            ZDICT_printHex(
                (samplesBuffer as *const libc::c_char).offset(pos as isize) as *const libc::c_void,
                printedLength as libc::size_t,
            );
            if notificationLevel >= 3 {
                fprintf(stderr, b"| \n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
            }
            u = u.wrapping_add(1);
        }
    }
    let mut dictContentSize_0 = ZDICT_dictSize(dictList);
    if dictContentSize_0 < ZDICT_CONTENTSIZE_MIN as libc::c_uint {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as libc::size_t;
    }
    if (dictContentSize_0 as libc::c_ulong) < targetDictSize.wrapping_div(4) {
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  warning : selected content significantly smaller than requested (%u < %u) \n\0"
                    as *const u8 as *const libc::c_char,
                dictContentSize_0,
                maxDictSize as libc::c_uint,
            );
            fflush(stderr);
        }
        if samplesBuffSize < (10).wrapping_mul(targetDictSize) {
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"!  consider increasing the number of samples (total size : %u MB)\n\0"
                        as *const u8 as *const libc::c_char,
                    (samplesBuffSize >> 20 as libc::c_int) as libc::c_uint,
                );
                fflush(stderr);
            }
        }
        if minRep > MINRATIO as libc::c_uint {
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"!  consider increasing selectivity to produce larger dictionary (-s%u) \n\0"
                        as *const u8 as *const libc::c_char,
                    selectivity.wrapping_add(1),
                );
                fflush(stderr);
            }
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"!  note : larger dictionaries are not necessarily better, test its efficiency on samples \n\0"
                        as *const u8 as *const libc::c_char,
                );
                fflush(stderr);
            }
        }
    }
    if dictContentSize_0 as libc::c_ulong > targetDictSize.wrapping_mul(3)
        && nbSamples > (2 as libc::c_int * MINRATIO) as libc::c_uint
        && selectivity > 1
    {
        let mut proposedSelectivity = selectivity.wrapping_sub(1);
        while nbSamples >> proposedSelectivity <= MINRATIO as libc::c_uint {
            proposedSelectivity = proposedSelectivity.wrapping_sub(1);
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  note : calculated dictionary significantly larger than requested (%u > %u) \n\0"
                    as *const u8 as *const libc::c_char,
                dictContentSize_0,
                maxDictSize as libc::c_uint,
            );
            fflush(stderr);
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  consider increasing dictionary size, or produce denser dictionary (-s%u) \n\0"
                    as *const u8 as *const libc::c_char,
                proposedSelectivity,
            );
            fflush(stderr);
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  always test dictionary efficiency on real samples \n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
    }
    let max = (*dictList).pos;
    let mut currentSize = 0 as libc::c_int as u32;
    let mut n: u32 = 0;
    n = 1 as libc::c_int as u32;
    while n < max {
        currentSize =
            (currentSize as libc::c_uint).wrapping_add((*dictList.offset(n as isize)).length);
        if currentSize as libc::c_ulong > targetDictSize {
            currentSize =
                (currentSize as libc::c_uint).wrapping_sub((*dictList.offset(n as isize)).length);
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    (*dictList).pos = n;
    dictContentSize_0 = currentSize;
    let mut u_0: u32 = 0;
    let mut ptr = (dictBuffer as *mut u8).offset(maxDictSize as isize);
    u_0 = 1 as libc::c_int as u32;
    while u_0 < (*dictList).pos {
        let mut l = (*dictList.offset(u_0 as isize)).length;
        ptr = ptr.offset(-(l as isize));
        if ptr < dictBuffer as *mut u8 {
            free(dictList as *mut libc::c_void);
            return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
        }
        memcpy(
            ptr as *mut libc::c_void,
            (samplesBuffer as *const libc::c_char)
                .offset((*dictList.offset(u_0 as isize)).pos as isize)
                as *const libc::c_void,
            l as libc::c_ulong,
        );
        u_0 = u_0.wrapping_add(1);
    }
    dictSize = ZDICT_addEntropyTablesFromBuffer_advanced(
        dictBuffer,
        dictContentSize_0 as libc::size_t,
        maxDictSize,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        params.zParams,
    );
    free(dictList as *mut libc::c_void);
    return dictSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_legacy(
    mut dictBuffer: *mut libc::c_void,
    mut dictBufferCapacity: libc::size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const libc::size_t,
    mut nbSamples: libc::c_uint,
    mut params: ZDICT_legacy_params_t,
) -> libc::size_t {
    let mut result: libc::size_t = 0;
    let mut newBuff = 0 as *mut libc::c_void;
    let sBuffSize = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    if sBuffSize < ZDICT_MIN_SAMPLES_SIZE as libc::c_ulong {
        return 0 as libc::c_int as libc::size_t;
    }
    newBuff = malloc(sBuffSize.wrapping_add(NOISELENGTH as libc::c_ulong));
    if newBuff.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    memcpy(newBuff, samplesBuffer, sBuffSize);
    ZDICT_fillNoise(
        (newBuff as *mut libc::c_char).offset(sBuffSize as isize) as *mut libc::c_void,
        NOISELENGTH as libc::size_t,
    );
    result = ZDICT_trainFromBuffer_unsafe_legacy(
        dictBuffer,
        dictBufferCapacity,
        newBuff,
        samplesSizes,
        nbSamples,
        params,
    );
    free(newBuff);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer(
    mut dictBuffer: *mut libc::c_void,
    mut dictBufferCapacity: libc::size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const libc::size_t,
    mut nbSamples: libc::c_uint,
) -> libc::size_t {
    let mut params = ZDICT_fastCover_params_t {
        k: 0,
        d: 0,
        f: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        accel: 0,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    memset(
        &mut params as *mut ZDICT_fastCover_params_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZDICT_fastCover_params_t>(),
    );
    params.d = 8 as libc::c_int as libc::c_uint;
    params.steps = 4 as libc::c_int as libc::c_uint;
    params.zParams.compressionLevel = ZSTD_CLEVEL_DEFAULT;
    params.zParams.notificationLevel = DEBUGLEVEL as libc::c_uint;
    return ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer,
        dictBufferCapacity,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        &mut params,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer(
    mut dictBuffer: *mut libc::c_void,
    mut dictContentSize: libc::size_t,
    mut dictBufferCapacity: libc::size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const libc::size_t,
    mut nbSamples: libc::c_uint,
) -> libc::size_t {
    let mut params = ZDICT_params_t {
        compressionLevel: 0,
        notificationLevel: 0,
        dictID: 0,
    };
    memset(
        &mut params as *mut ZDICT_params_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZDICT_params_t>(),
    );
    return ZDICT_addEntropyTablesFromBuffer_advanced(
        dictBuffer,
        dictContentSize,
        dictBufferCapacity,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        params,
    );
}
