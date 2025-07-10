use std::ffi::{c_char, c_void};
use ::c2rust_bitfields;
extern "C" {
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    fn ZSTD_compressBound(srcSize: usize) -> usize;
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> usize;
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> usize;
    fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> usize;
    fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> usize;
    fn ZSTD_createCCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_CCtx;
    fn ZSTD_createCDict_advanced(
        dict: *const c_void,
        dictSize: usize,
        dictLoadMethod: ZSTD_dictLoadMethod_e,
        dictContentType: ZSTD_dictContentType_e,
        cParams: ZSTD_compressionParameters,
        customMem: ZSTD_customMem,
    ) -> *mut ZSTD_CDict;
    fn ZSTD_CCtxParams_setParameter(
        params: *mut ZSTD_CCtx_params,
        param: ZSTD_cParameter,
        value: i32,
    ) -> usize;
    fn ZSTD_getCParamsFromCCtxParams(
        CCtxParams: *const ZSTD_CCtx_params,
        srcSizeHint: u64,
        dictSize: usize,
        mode: ZSTD_CParamMode_e,
    ) -> ZSTD_compressionParameters;
    fn ZSTD_compressBegin_advanced_internal(
        cctx: *mut ZSTD_CCtx,
        dict: *const c_void,
        dictSize: usize,
        dictContentType: ZSTD_dictContentType_e,
        dtlm: ZSTD_dictTableLoadMethod_e,
        cdict: *const ZSTD_CDict,
        params: *const ZSTD_CCtx_params,
        pledgedSrcSize: u64,
    ) -> usize;
    fn ZSTD_writeLastEmptyBlock(
        dst: *mut c_void,
        dstCapacity: usize,
    ) -> usize;
    fn ZSTD_referenceExternalSequences(
        cctx: *mut ZSTD_CCtx,
        seq: *mut rawSeq,
        nbSeq: usize,
    );
    fn ZSTD_cycleLog(hashLog: u32, strat: ZSTD_strategy) -> u32;
    fn ZSTD_CCtx_trace(cctx: *mut ZSTD_CCtx, extraCSize: usize);
    fn ZSTD_compressContinue_public(
        cctx: *mut ZSTD_CCtx,
        dst: *mut c_void,
        dstCapacity: usize,
        src: *const c_void,
        srcSize: usize,
    ) -> usize;
    fn ZSTD_compressEnd_public(
        cctx: *mut ZSTD_CCtx,
        dst: *mut c_void,
        dstCapacity: usize,
        src: *const c_void,
        srcSize: usize,
    ) -> usize;
    fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx);
    fn ZSTD_XXH64_reset(
        statePtr: *mut XXH64_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const c_void,
        length: usize,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    fn POOL_create_advanced(
        numThreads: usize,
        queueSize: usize,
        customMem: ZSTD_customMem,
    ) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_resize(ctx: *mut POOL_ctx, numThreads: usize) -> i32;
    fn POOL_sizeof(ctx: *const POOL_ctx) -> usize;
    fn POOL_tryAdd(
        ctx: *mut POOL_ctx,
        function: POOL_function,
        opaque: *mut c_void,
    ) -> i32;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> i32;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> i32;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> i32;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> i32;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> i32;
    fn ZSTD_ldm_fillHashTable(
        state: *mut ldmState_t,
        ip: *const u8,
        iend: *const u8,
        params: *const ldmParams_t,
    );
    fn ZSTD_ldm_generateSequences(
        ldms: *mut ldmState_t,
        sequences: *mut RawSeqStore_t,
        params: *const ldmParams_t,
        src: *const c_void,
        srcSize: usize,
    ) -> usize;
    fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: usize) -> usize;
    fn ZSTD_ldm_adjustParameters(
        params: *mut ldmParams_t,
        cParams: *const ZSTD_compressionParameters,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: u32,
    pub __high: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
    pub __unused_initialized_1: u32,
    pub __unused_initialized_2: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4],
    pub __align: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [c_char; 4],
    pub __align: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40],
    pub __align: std::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [c_char; 48],
    pub __align: i64,
}
use crate::common::error::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: i32,
    pub bmi2: i32,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub simpleApiParams: ZSTD_CCtx_params,
    pub dictID: u32,
    pub dictContentSize: usize,
    pub workspace: ZSTD_cwksp,
    pub blockSizeMax: usize,
    pub pledgedSrcSizePlusOne: u64,
    pub consumedSrcSize: u64,
    pub producedCSize: u64,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    pub pool: *mut ZSTD_threadPool,
    pub staticSize: usize,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: i32,
    pub initialized: i32,
    pub seqStore: SeqStore_t,
    pub ldmState: ldmState_t,
    pub ldmSequences: *mut rawSeq,
    pub maxNbLdmSequences: usize,
    pub externSeqStore: RawSeqStore_t,
    pub blockState: ZSTD_blockState_t,
    pub tmpWorkspace: *mut c_void,
    pub tmpWkspSize: usize,
    pub bufferedPolicy: ZSTD_buffered_policy_e,
    pub inBuff: *mut c_char,
    pub inBuffSize: usize,
    pub inToCompress: usize,
    pub inBuffPos: usize,
    pub inBuffTarget: usize,
    pub outBuff: *mut c_char,
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
    pub offset: u32,
    pub litLength: u32,
    pub matchLength: u32,
    pub rep: u32,
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
pub type SymbolEncodingType_e = u32;
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
pub type ZSTD_longLengthType_e = u32;
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
pub type ZSTD_TraceCtx = u64;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ZSTDMT_CCtx_s {
    pub factory: *mut POOL_ctx,
    pub jobs: *mut ZSTDMT_jobDescription,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub params: ZSTD_CCtx_params,
    pub targetSectionSize: usize,
    pub targetPrefixSize: usize,
    pub jobReady: i32,
    pub inBuff: InBuff_t,
    pub roundBuff: RoundBuff_t,
    pub serial: SerialState,
    pub rsync: RSyncState_t,
    pub jobIDMask: u32,
    pub doneJobID: u32,
    pub nextJobID: u32,
    pub frameEnded: u32,
    pub allJobsCompleted: u32,
    pub frameContentSize: u64,
    pub consumed: u64,
    pub produced: u64,
    pub cMem: ZSTD_customMem,
    pub cdictLocal: *mut ZSTD_CDict,
    pub cdict: *const ZSTD_CDict,
    #[bitfield(name = "providedFactory", ty = "u32", bits = "0..=0")]
    pub providedFactory: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type ZSTD_CDict = ZSTD_CDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut c_void, *mut c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RSyncState_t {
    pub hash: u64,
    pub hitMask: u64,
    pub primePower: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SerialState {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub params: ZSTD_CCtx_params,
    pub ldmState: ldmState_t,
    pub xxhState: XXH64_state_t,
    pub nextJobID: u32,
    pub ldmWindowMutex: pthread_mutex_t,
    pub ldmWindowCond: pthread_cond_t,
    pub ldmWindow: ZSTD_window_t,
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
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_params_s {
    pub format: ZSTD_format_e,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
    pub compressionLevel: i32,
    pub forceWindow: i32,
    pub targetCBlockSize: usize,
    pub srcSizeHint: i32,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub nbWorkers: i32,
    pub jobSize: usize,
    pub overlapLog: i32,
    pub rsyncable: i32,
    pub ldmParams: ldmParams_t,
    pub enableDedicatedDictSearch: i32,
    pub inBufferMode: ZSTD_bufferMode_e,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub blockDelimiters: ZSTD_SequenceFormat_e,
    pub validateSequences: i32,
    pub postBlockSplitter: ZSTD_ParamSwitch_e,
    pub preBlockSplitter_level: i32,
    pub maxBlockSize: usize,
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,
    pub deterministicRefPrefix: i32,
    pub customMem: ZSTD_customMem,
    pub prefetchCDictTables: ZSTD_ParamSwitch_e,
    pub enableMatchFinderFallback: i32,
    pub extSeqProdState: *mut c_void,
    pub extSeqProdFunc: ZSTD_sequenceProducer_F,
    pub searchForExternalRepcodes: ZSTD_ParamSwitch_e,
}
pub type ZSTD_ParamSwitch_e = u32;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
pub type ZSTD_sequenceProducer_F = Option::<
    unsafe extern "C" fn(
        *mut c_void,
        *mut ZSTD_Sequence,
        usize,
        *const c_void,
        usize,
        *const c_void,
        usize,
        i32,
        usize,
    ) -> usize,
>;
pub type ZSTD_SequenceFormat_e = u32;
pub const ZSTD_sf_explicitBlockDelimiters: ZSTD_SequenceFormat_e = 1;
pub const ZSTD_sf_noBlockDelimiters: ZSTD_SequenceFormat_e = 0;
pub type ZSTD_bufferMode_e = u32;
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
pub type ZSTD_dictAttachPref_e = u32;
pub const ZSTD_dictForceLoad: ZSTD_dictAttachPref_e = 3;
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = 2;
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: i32,
    pub checksumFlag: i32,
    pub noDictIDFlag: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: u32,
    pub chainLog: u32,
    pub hashLog: u32,
    pub searchLog: u32,
    pub minMatch: u32,
    pub targetLength: u32,
    pub strategy: ZSTD_strategy,
}
pub type ZSTD_strategy = u32;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub type ZSTD_format_e = u32;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoundBuff_t {
    pub buffer: *mut u8,
    pub capacity: usize,
    pub pos: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InBuff_t {
    pub prefix: Range,
    pub buffer: Buffer,
    pub filled: usize,
}
pub type Buffer = buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_s {
    pub start: *mut c_void,
    pub capacity: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: *const c_void,
    pub size: usize,
}
pub type ZSTDMT_seqPool = ZSTDMT_bufferPool;
pub type ZSTDMT_bufferPool = ZSTDMT_bufferPool_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_bufferPool_s {
    pub poolMutex: pthread_mutex_t,
    pub bufferSize: usize,
    pub totalBuffers: u32,
    pub nbBuffers: u32,
    pub cMem: ZSTD_customMem,
    pub buffers: *mut Buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_CCtxPool {
    pub poolMutex: pthread_mutex_t,
    pub totalCCtx: i32,
    pub availCCtx: i32,
    pub cMem: ZSTD_customMem,
    pub cctxs: *mut *mut ZSTD_CCtx,
}
pub type ZSTD_CCtx = ZSTD_CCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_jobDescription {
    pub consumed: usize,
    pub cSize: usize,
    pub job_mutex: pthread_mutex_t,
    pub job_cond: pthread_cond_t,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub serial: *mut SerialState,
    pub dstBuff: Buffer,
    pub prefix: Range,
    pub src: Range,
    pub jobID: u32,
    pub firstJob: u32,
    pub lastJob: u32,
    pub params: ZSTD_CCtx_params,
    pub cdict: *const ZSTD_CDict,
    pub fullFrameSize: u64,
    pub dstFlushed: usize,
    pub frameChecksumNeeded: u32,
}
pub type POOL_ctx = POOL_ctx_s;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const c_void,
    pub dictSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_dictContentType_e = u32;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut c_void,
    pub dict: *const c_void,
    pub dictSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const c_void,
    pub size: usize,
    pub pos: usize,
}
pub type ZSTD_cStreamStage = u32;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = u32;
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
    pub forceNonContiguous: i32,
    pub dedicatedDictSearch: i32,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const RawSeqStore_t,
    pub prefetchCDictTables: i32,
    pub lazySkipping: i32,
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
pub struct optState_t {
    pub litFreq: *mut u32,
    pub litLengthFreq: *mut u32,
    pub matchLengthFreq: *mut u32,
    pub offCodeFreq: *mut u32,
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
pub type FSE_repeat = u32;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = u32;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = usize;
pub type ZSTD_OptPrice_e = u32;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: i32,
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
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: i32,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: usize,
    pub maxSequences: usize,
}
pub type ZSTD_threadPool = POOL_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut c_void,
    pub workspaceEnd: *mut c_void,
    pub objectEnd: *mut c_void,
    pub tableEnd: *mut c_void,
    pub tableValidEnd: *mut c_void,
    pub allocStart: *mut c_void,
    pub initOnceStart: *mut c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: i32,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}
pub type ZSTD_cwksp_static_alloc_e = u32;
pub const ZSTD_cwksp_static_alloc: ZSTD_cwksp_static_alloc_e = 1;
pub const ZSTD_cwksp_dynamic_alloc: ZSTD_cwksp_static_alloc_e = 0;
pub type ZSTD_cwksp_alloc_phase_e = u32;
pub const ZSTD_cwksp_alloc_buffers: ZSTD_cwksp_alloc_phase_e = 3;
pub const ZSTD_cwksp_alloc_aligned: ZSTD_cwksp_alloc_phase_e = 2;
pub const ZSTD_cwksp_alloc_aligned_init_once: ZSTD_cwksp_alloc_phase_e = 1;
pub const ZSTD_cwksp_alloc_objects: ZSTD_cwksp_alloc_phase_e = 0;
pub type ZSTD_compressionStage_e = u32;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type ZSTD_cParameter = u32;
pub const ZSTD_c_experimentalParam20: ZSTD_cParameter = 1017;
pub const ZSTD_c_experimentalParam19: ZSTD_cParameter = 1016;
pub const ZSTD_c_experimentalParam18: ZSTD_cParameter = 1015;
pub const ZSTD_c_experimentalParam17: ZSTD_cParameter = 1014;
pub const ZSTD_c_experimentalParam16: ZSTD_cParameter = 1013;
pub const ZSTD_c_experimentalParam15: ZSTD_cParameter = 1012;
pub const ZSTD_c_experimentalParam14: ZSTD_cParameter = 1011;
pub const ZSTD_c_experimentalParam13: ZSTD_cParameter = 1010;
pub const ZSTD_c_experimentalParam12: ZSTD_cParameter = 1009;
pub const ZSTD_c_experimentalParam11: ZSTD_cParameter = 1008;
pub const ZSTD_c_experimentalParam10: ZSTD_cParameter = 1007;
pub const ZSTD_c_experimentalParam9: ZSTD_cParameter = 1006;
pub const ZSTD_c_experimentalParam8: ZSTD_cParameter = 1005;
pub const ZSTD_c_experimentalParam7: ZSTD_cParameter = 1004;
pub const ZSTD_c_experimentalParam5: ZSTD_cParameter = 1002;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_c_targetCBlockSize: ZSTD_cParameter = 130;
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut c_void,
    pub size: usize,
    pub pos: usize,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_EndDirective = u32;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_dictLoadMethod_e = u32;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameProgression {
    pub ingested: u64,
    pub consumed: u64,
    pub produced: u64,
    pub flushed: u64,
    pub currentJobID: u32,
    pub nbActiveWorkers: u32,
}
pub type unalign32 = u32;
pub type POOL_function = Option::<unsafe extern "C" fn(*mut c_void) -> ()>;
pub type XXH_errorcode = u32;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type ZSTD_dictTableLoadMethod_e = u32;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SyncPoint {
    pub toLoad: usize,
    pub flush: i32,
}
pub type ZSTD_CParamMode_e = u32;
pub const ZSTD_cpm_unknown: ZSTD_CParamMode_e = 3;
pub const ZSTD_cpm_createCDict: ZSTD_CParamMode_e = 2;
pub const ZSTD_cpm_attachDict: ZSTD_CParamMode_e = 1;
pub const ZSTD_cpm_noAttachDict: ZSTD_CParamMode_e = 0;
pub const ZSTD_BLOCKSIZELOG_MAX: i32 = 17;
pub const ZSTD_BLOCKSIZE_MAX: i32 = (1 as i32)
    << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: u64 = (0 as u64)
    .wrapping_sub(1);
pub const ZSTD_c_forceMaxWindow: i32 = ZSTD_c_experimentalParam3
    as i32;
pub const ZSTD_c_deterministicRefPrefix: i32 = ZSTD_c_experimentalParam15
    as i32;
pub const HASH_READ_SIZE: i32 = 8;
static mut kNullRawSeqStore: RawSeqStore_t = {
    let mut init = RawSeqStore_t {
        seq: NULL_0 as *mut rawSeq,
        pos: 0,
        posInSequence: 0,
        size: 0,
        capacity: 0,
    };
    init
};
pub const ZSTD_WINDOW_START_INDEX: i32 = 2;
static mut prime8bytes: u64 = 0xcf1bbcdcb7a56463 as u64 as u64;
unsafe extern "C" fn ZSTD_ipow(mut base: u64, mut exponent: u64) -> u64 {
    let mut power: u64 = 1;
    while exponent != 0 {
        if exponent & 1_u64 != 0 {
            power = power * base;
        }
        exponent >>= 1;
        base = base * base;
    }
    return power;
}
pub const ZSTD_ROLL_HASH_CHAR_OFFSET: i32 = 10;
unsafe extern "C" fn ZSTD_rollingHash_append(
    mut hash: u64,
    mut buf: *const c_void,
    mut size: usize,
) -> u64 {
    let mut istart = buf as *const u8;
    let mut pos: usize = 0;
    pos = 0;
    while pos < size {
        hash = hash * prime8bytes;
        hash = hash
            .wrapping_add(
                (*istart.offset(pos as isize) as i32
                    + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64,
            );
        pos = pos.wrapping_add(1);
        pos;
    }
    return hash;
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_compute(
    mut buf: *const c_void,
    mut size: usize,
) -> u64 {
    return ZSTD_rollingHash_append(0, buf, size);
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_primePower(mut length: u32) -> u64 {
    return ZSTD_ipow(
        prime8bytes,
        length.wrapping_sub(1) as u64,
    );
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_rotate(
    mut hash: u64,
    mut toRemove: u8,
    mut toAdd: u8,
    mut primePower: u64,
) -> u64 {
    hash = hash
        .wrapping_sub(
            (toRemove as i32 + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64
                * primePower,
        );
    hash = hash * prime8bytes;
    hash = hash
        .wrapping_add((toAdd as i32 + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64);
    return hash;
}
#[inline]
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base) as std::ffi::c_long
        as usize;
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
#[inline]
unsafe extern "C" fn ZSTD_window_init(mut window: *mut ZSTD_window_t) {
    libc::memset(
        window as *mut c_void,
        0,
        ::core::mem::size_of::<ZSTD_window_t>() as usize,
    );
    (*window).base = b" \0" as *const u8 as *const c_char as *const u8;
    (*window).dictBase = b" \0" as *const u8 as *const c_char as *const u8;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0;
}
#[inline]
unsafe extern "C" fn ZSTD_window_update(
    mut window: *mut ZSTD_window_t,
    mut src: *const c_void,
    mut srcSize: usize,
    mut forceNonContiguous: i32,
) -> u32 {
    let ip = src as *const u8;
    let mut contiguous: u32 = 1;
    if srcSize == 0 {
        return contiguous;
    }
    if src != (*window).nextSrc as *const c_void || forceNonContiguous != 0 {
        let distanceFromBase = ((*window).nextSrc).offset_from((*window).base)
            as std::ffi::c_long as usize;
        (*window).lowLimit = (*window).dictLimit;
        (*window).dictLimit = distanceFromBase as u32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit) < HASH_READ_SIZE as u32
        {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = 0;
    }
    (*window).nextSrc = ip.offset(srcSize as isize);
    if (ip.offset(srcSize as isize)
        > ((*window).dictBase).offset((*window).lowLimit as isize)) as i32
        & (ip < ((*window).dictBase).offset((*window).dictLimit as isize))
            as i32 != 0
    {
        let highInputIdx = ip.offset(srcSize as isize).offset_from((*window).dictBase)
            as std::ffi::c_long as usize;
        let lowLimitMax = if highInputIdx > (*window).dictLimit as usize {
            (*window).dictLimit
        } else {
            highInputIdx as u32
        };
        (*window).lowLimit = lowLimitMax;
    }
    return contiguous;
}
use crate::common::mem::*;
pub const ZSTDMT_JOBSIZE_MIN: i32 = 512 as i32
    * ((1 as i32) << 10);
#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    mut size: usize,
    mut customMem: ZSTD_customMem,
) -> *mut c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
    }
    return libc::malloc(size);
}
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    mut size: usize,
    mut customMem: ZSTD_customMem,
) -> *mut c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
        libc::memset(ptr, 0, (size) as usize);
        return ptr;
    }
    return libc::calloc(1, size);
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut c_void,
    mut customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree)
                .expect("non-null function pointer")(customMem.opaque, ptr);
        } else {
            ZSTD_free!(ptr)(ZSTD_free!(ptr));
        }
    }
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const c_char,
    mut args: ...
) {}
use crate::common::bits::*;
pub const NULL: i32 = 0;
pub const NULL_0: i32 = 0;
static mut g_nullBuffer: Buffer = {
    let mut init = buffer_s {
        start: NULL_0 as *mut c_void,
        capacity: 0,
    };
    init
};
unsafe extern "C" fn ZSTDMT_freeBufferPool(mut bufPool: *mut ZSTDMT_bufferPool) {
    if bufPool.is_null() {
        return;
    }
    if !((*bufPool).buffers).is_null() {
        let mut u: u32 = 0;
        u = 0;
        while u < (*bufPool).totalBuffers {
            ZSTD_customFree(
                (*((*bufPool).buffers).offset(u as isize)).start,
                (*bufPool).cMem,
            );
            u = u.wrapping_add(1);
            u;
        }
        ZSTD_customFree((*bufPool).buffers as *mut c_void, (*bufPool).cMem);
    }
    ZSTD_pthread_mutex_destroy!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_destroy!(& bufPool -> poolMutex));
    ZSTD_customFree(bufPool as *mut c_void, (*bufPool).cMem);
}
unsafe extern "C" fn ZSTDMT_createBufferPool(
    mut maxNbBuffers: u32,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_bufferPool {
    let bufPool = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTDMT_bufferPool>(),
        cMem,
    ) as *mut ZSTDMT_bufferPool;
    if bufPool.is_null() {
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    if ZSTD_pthread_mutex_init!(& bufPool -> poolMutex, NULL) != 0 {
        ZSTD_customFree(bufPool as *mut c_void, cMem);
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    (*bufPool)
        .buffers = ZSTD_customCalloc(
        (maxNbBuffers as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Buffer>()),
        cMem,
    ) as *mut Buffer;
    if ((*bufPool).buffers).is_null() {
        ZSTDMT_freeBufferPool(bufPool);
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    (*bufPool)
        .bufferSize = (64 as i32
        * ((1 as i32) << 10)) as usize;
    (*bufPool).totalBuffers = maxNbBuffers;
    (*bufPool).nbBuffers = 0;
    (*bufPool).cMem = cMem;
    return bufPool;
}
unsafe extern "C" fn ZSTDMT_sizeof_bufferPool(
    mut bufPool: *mut ZSTDMT_bufferPool,
) -> usize {
    let poolSize = ::core::mem::size_of::<ZSTDMT_bufferPool>();
    let arraySize = ((*bufPool).totalBuffers as std::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Buffer>());
    let mut u: u32 = 0;
    let mut totalBufferSize: usize = 0;
    ZSTD_pthread_mutex_lock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& bufPool -> poolMutex));
    u = 0;
    while u < (*bufPool).totalBuffers {
        totalBufferSize = totalBufferSize
            .wrapping_add((*((*bufPool).buffers).offset(u as isize)).capacity);
        u = u.wrapping_add(1);
        u;
    }
    ZSTD_pthread_mutex_unlock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& bufPool -> poolMutex));
    return poolSize.wrapping_add(arraySize).wrapping_add(totalBufferSize);
}
unsafe extern "C" fn ZSTDMT_setBufferSize(
    bufPool: *mut ZSTDMT_bufferPool,
    bSize: usize,
) {
    ZSTD_pthread_mutex_lock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& bufPool -> poolMutex));
    (*bufPool).bufferSize = bSize;
    ZSTD_pthread_mutex_unlock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& bufPool -> poolMutex));
}
unsafe extern "C" fn ZSTDMT_expandBufferPool(
    mut srcBufPool: *mut ZSTDMT_bufferPool,
    mut maxNbBuffers: u32,
) -> *mut ZSTDMT_bufferPool {
    if srcBufPool.is_null() {
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    if (*srcBufPool).totalBuffers >= maxNbBuffers {
        return srcBufPool;
    }
    let cMem = (*srcBufPool).cMem;
    let bSize = (*srcBufPool).bufferSize;
    let mut newBufPool = std::ptr::null_mut();
    ZSTDMT_freeBufferPool(srcBufPool);
    newBufPool = ZSTDMT_createBufferPool(maxNbBuffers, cMem);
    if newBufPool.is_null() {
        return newBufPool;
    }
    ZSTDMT_setBufferSize(newBufPool, bSize);
    return newBufPool;
}
unsafe extern "C" fn ZSTDMT_getBuffer(mut bufPool: *mut ZSTDMT_bufferPool) -> Buffer {
    let bSize = (*bufPool).bufferSize;
    ZSTD_pthread_mutex_lock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& bufPool -> poolMutex));
    if (*bufPool).nbBuffers != 0 {
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_sub(1);
        let buf = *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize);
        let availBufferSize = buf.capacity;
        *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize) = g_nullBuffer;
        if (availBufferSize >= bSize) as i32
            & (availBufferSize >> 3 <= bSize) as i32 != 0
        {
            ZSTD_pthread_mutex_unlock!(
                & bufPool -> poolMutex
            )(ZSTD_pthread_mutex_unlock!(& bufPool -> poolMutex));
            return buf;
        }
        ZSTD_customFree(buf.start, (*bufPool).cMem);
    }
    ZSTD_pthread_mutex_unlock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& bufPool -> poolMutex));
    let mut buffer = buffer_s {
        start: std::ptr::null_mut(),
        capacity: 0,
    };
    let start = ZSTD_customMalloc(bSize, (*bufPool).cMem);
    buffer.start = start;
    buffer
        .capacity = if start.is_null() { 0_usize } else { bSize };
    start.is_null();
    return buffer;
}
unsafe extern "C" fn ZSTDMT_releaseBuffer(
    mut bufPool: *mut ZSTDMT_bufferPool,
    mut buf: Buffer,
) {
    if (buf.start).is_null() {
        return;
    }
    ZSTD_pthread_mutex_lock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& bufPool -> poolMutex));
    if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
        let fresh0 = (*bufPool).nbBuffers;
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_add(1);
        *((*bufPool).buffers).offset(fresh0 as isize) = buf;
        ZSTD_pthread_mutex_unlock!(
            & bufPool -> poolMutex
        )(ZSTD_pthread_mutex_unlock!(& bufPool -> poolMutex));
        return;
    }
    ZSTD_pthread_mutex_unlock!(
        & bufPool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& bufPool -> poolMutex));
    ZSTD_customFree(buf.start, (*bufPool).cMem);
}
unsafe extern "C" fn ZSTDMT_sizeof_seqPool(mut seqPool: *mut ZSTDMT_seqPool) -> usize {
    return ZSTDMT_sizeof_bufferPool(seqPool);
}
unsafe extern "C" fn bufferToSeq(mut buffer: Buffer) -> RawSeqStore_t {
    let mut seq = kNullRawSeqStore;
    seq.seq = buffer.start as *mut rawSeq;
    seq
        .capacity = (buffer.capacity)
        .wrapping_div(::core::mem::size_of::<rawSeq>());
    return seq;
}
unsafe extern "C" fn seqToBuffer(mut seq: RawSeqStore_t) -> Buffer {
    let mut buffer = buffer_s {
        start: std::ptr::null_mut(),
        capacity: 0,
    };
    buffer.start = seq.seq as *mut c_void;
    buffer
        .capacity = (seq.capacity)
        .wrapping_mul(::core::mem::size_of::<rawSeq>());
    return buffer;
}
unsafe extern "C" fn ZSTDMT_getSeq(mut seqPool: *mut ZSTDMT_seqPool) -> RawSeqStore_t {
    if (*seqPool).bufferSize == 0 {
        return kNullRawSeqStore;
    }
    return bufferToSeq(ZSTDMT_getBuffer(seqPool));
}
unsafe extern "C" fn ZSTDMT_releaseSeq(
    mut seqPool: *mut ZSTDMT_seqPool,
    mut seq: RawSeqStore_t,
) {
    ZSTDMT_releaseBuffer(seqPool, seqToBuffer(seq));
}
unsafe extern "C" fn ZSTDMT_setNbSeq(seqPool: *mut ZSTDMT_seqPool, nbSeq: usize) {
    ZSTDMT_setBufferSize(
        seqPool,
        nbSeq.wrapping_mul(::core::mem::size_of::<rawSeq>()),
    );
}
unsafe extern "C" fn ZSTDMT_createSeqPool(
    mut nbWorkers: u32,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_seqPool {
    let seqPool = ZSTDMT_createBufferPool(SEQ_POOL_MAX_NB_BUFFERS!(nbWorkers), cMem);
    if seqPool.is_null() {
        return NULL_0 as *mut ZSTDMT_seqPool;
    }
    ZSTDMT_setNbSeq(seqPool, 0);
    return seqPool;
}
unsafe extern "C" fn ZSTDMT_freeSeqPool(mut seqPool: *mut ZSTDMT_seqPool) {
    ZSTDMT_freeBufferPool(seqPool);
}
unsafe extern "C" fn ZSTDMT_expandSeqPool(
    mut pool: *mut ZSTDMT_seqPool,
    mut nbWorkers: u32,
) -> *mut ZSTDMT_seqPool {
    return ZSTDMT_expandBufferPool(pool, SEQ_POOL_MAX_NB_BUFFERS!(nbWorkers));
}
unsafe extern "C" fn ZSTDMT_freeCCtxPool(mut pool: *mut ZSTDMT_CCtxPool) {
    if pool.is_null() {
        return;
    }
    ZSTD_pthread_mutex_destroy!(
        & pool -> poolMutex
    )(ZSTD_pthread_mutex_destroy!(& pool -> poolMutex));
    if !((*pool).cctxs).is_null() {
        let mut cid: i32 = 0;
        cid = 0;
        while cid < (*pool).totalCCtx {
            ZSTD_freeCCtx(*((*pool).cctxs).offset(cid as isize));
            cid += 1;
            cid;
        }
        ZSTD_customFree((*pool).cctxs as *mut c_void, (*pool).cMem);
    }
    ZSTD_customFree(pool as *mut c_void, (*pool).cMem);
}
unsafe extern "C" fn ZSTDMT_createCCtxPool(
    mut nbWorkers: i32,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_CCtxPool {
    let cctxPool = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTDMT_CCtxPool>(),
        cMem,
    ) as *mut ZSTDMT_CCtxPool;
    if cctxPool.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    if ZSTD_pthread_mutex_init!(& cctxPool -> poolMutex, NULL) != 0 {
        ZSTD_customFree(cctxPool as *mut c_void, cMem);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).totalCCtx = nbWorkers;
    (*cctxPool)
        .cctxs = ZSTD_customCalloc(
        (nbWorkers as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>()),
        cMem,
    ) as *mut *mut ZSTD_CCtx;
    if ((*cctxPool).cctxs).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).cMem = cMem;
    let ref mut fresh1 = *((*cctxPool).cctxs).offset(0);
    *fresh1 = ZSTD_createCCtx_advanced(cMem);
    if (*((*cctxPool).cctxs).offset(0)).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).availCCtx = 1;
    return cctxPool;
}
unsafe extern "C" fn ZSTDMT_expandCCtxPool(
    mut srcPool: *mut ZSTDMT_CCtxPool,
    mut nbWorkers: i32,
) -> *mut ZSTDMT_CCtxPool {
    if srcPool.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    if nbWorkers <= (*srcPool).totalCCtx {
        return srcPool;
    }
    let cMem = (*srcPool).cMem;
    ZSTDMT_freeCCtxPool(srcPool);
    return ZSTDMT_createCCtxPool(nbWorkers, cMem);
}
unsafe extern "C" fn ZSTDMT_sizeof_CCtxPool(
    mut cctxPool: *mut ZSTDMT_CCtxPool,
) -> usize {
    ZSTD_pthread_mutex_lock!(
        & cctxPool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& cctxPool -> poolMutex));
    let nbWorkers = (*cctxPool).totalCCtx as u32;
    let poolSize = ::core::mem::size_of::<ZSTDMT_CCtxPool>();
    let arraySize = ((*cctxPool).totalCCtx as std::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>());
    let mut totalCCtxSize: usize = 0;
    let mut u: u32 = 0;
    u = 0;
    while u < nbWorkers {
        totalCCtxSize = totalCCtxSize
            .wrapping_add(ZSTD_sizeof_CCtx(*((*cctxPool).cctxs).offset(u as isize)));
        u = u.wrapping_add(1);
        u;
    }
    ZSTD_pthread_mutex_unlock!(
        & cctxPool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& cctxPool -> poolMutex));
    return poolSize.wrapping_add(arraySize).wrapping_add(totalCCtxSize);
}
unsafe extern "C" fn ZSTDMT_getCCtx(
    mut cctxPool: *mut ZSTDMT_CCtxPool,
) -> *mut ZSTD_CCtx {
    ZSTD_pthread_mutex_lock!(
        & cctxPool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& cctxPool -> poolMutex));
    if (*cctxPool).availCCtx != 0 {
        (*cctxPool).availCCtx -= 1;
        (*cctxPool).availCCtx;
        let cctx = *((*cctxPool).cctxs).offset((*cctxPool).availCCtx as isize);
        ZSTD_pthread_mutex_unlock!(
            & cctxPool -> poolMutex
        )(ZSTD_pthread_mutex_unlock!(& cctxPool -> poolMutex));
        return cctx;
    }
    ZSTD_pthread_mutex_unlock!(
        & cctxPool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& cctxPool -> poolMutex));
    return ZSTD_createCCtx_advanced((*cctxPool).cMem);
}
unsafe extern "C" fn ZSTDMT_releaseCCtx(
    mut pool: *mut ZSTDMT_CCtxPool,
    mut cctx: *mut ZSTD_CCtx,
) {
    if cctx.is_null() {
        return;
    }
    ZSTD_pthread_mutex_lock!(
        & pool -> poolMutex
    )(ZSTD_pthread_mutex_lock!(& pool -> poolMutex));
    if (*pool).availCCtx < (*pool).totalCCtx {
        let fresh2 = (*pool).availCCtx;
        (*pool).availCCtx = (*pool).availCCtx + 1;
        let ref mut fresh3 = *((*pool).cctxs).offset(fresh2 as isize);
        *fresh3 = cctx;
    } else {
        ZSTD_freeCCtx(cctx);
    }
    ZSTD_pthread_mutex_unlock!(
        & pool -> poolMutex
    )(ZSTD_pthread_mutex_unlock!(& pool -> poolMutex));
}
unsafe extern "C" fn ZSTDMT_serialState_reset(
    mut serialState: *mut SerialState,
    mut seqPool: *mut ZSTDMT_seqPool,
    mut params: ZSTD_CCtx_params,
    mut jobSize: usize,
    mut dict: *const c_void,
    dictSize: usize,
    mut dictContentType: ZSTD_dictContentType_e,
) -> i32 {
    if params.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &mut params.cParams);
    } else {
        libc::memset(
            &mut params.ldmParams as *mut ldmParams_t as *mut c_void,
            0,
            ::core::mem::size_of::<ldmParams_t>() as usize,
        );
    }
    (*serialState).nextJobID = 0;
    if params.fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(
            &mut (*serialState).xxhState,
            0,
        );
    }
    if params.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        let mut cMem = params.customMem;
        let hashLog = params.ldmParams.hashLog;
        let hashSize = (1_usize << hashLog)
            .wrapping_mul(::core::mem::size_of::<ldmEntry_t>());
        let bucketLog = (params.ldmParams.hashLog)
            .wrapping_sub(params.ldmParams.bucketSizeLog);
        let prevBucketLog = ((*serialState).params.ldmParams.hashLog)
            .wrapping_sub((*serialState).params.ldmParams.bucketSizeLog);
        let numBuckets = 1_usize << bucketLog;
        ZSTDMT_setNbSeq(seqPool, ZSTD_ldm_getMaxNbSeq(params.ldmParams, jobSize));
        ZSTD_window_init(&mut (*serialState).ldmState.window);
        if ((*serialState).ldmState.hashTable).is_null()
            || (*serialState).params.ldmParams.hashLog < hashLog
        {
            ZSTD_customFree(
                (*serialState).ldmState.hashTable as *mut c_void,
                cMem,
            );
            (*serialState)
                .ldmState
                .hashTable = ZSTD_customMalloc(hashSize, cMem) as *mut ldmEntry_t;
        }
        if ((*serialState).ldmState.bucketOffsets).is_null() || prevBucketLog < bucketLog
        {
            ZSTD_customFree(
                (*serialState).ldmState.bucketOffsets as *mut c_void,
                cMem,
            );
            (*serialState)
                .ldmState
                .bucketOffsets = ZSTD_customMalloc(numBuckets, cMem) as *mut u8;
        }
        if ((*serialState).ldmState.hashTable).is_null()
            || ((*serialState).ldmState.bucketOffsets).is_null()
        {
            return 1;
        }
        libc::memset((*serialState).ldmState.hashTable, 0, (hashSize) as usize);
        libc::memset((*serialState).ldmState.bucketOffsets, 0, (numBuckets) as usize);
        (*serialState).ldmState.loadedDictEnd = 0;
        if dictSize > 0 {
            if dictContentType as u32
                == ZSTD_dct_rawContent as i32 as u32
            {
                let dictEnd = (dict as *const u8).offset(dictSize as isize);
                ZSTD_window_update(
                    &mut (*serialState).ldmState.window,
                    dict,
                    dictSize,
                    0,
                );
                ZSTD_ldm_fillHashTable(
                    &mut (*serialState).ldmState,
                    dict as *const u8,
                    dictEnd,
                    &mut params.ldmParams,
                );
                (*serialState)
                    .ldmState
                    .loadedDictEnd = if params.forceWindow != 0 {
                    0_u32
                } else {
                    dictEnd.offset_from((*serialState).ldmState.window.base)
                        as std::ffi::c_long as u32
                };
            }
        }
        (*serialState).ldmWindow = (*serialState).ldmState.window;
    }
    (*serialState).params = params;
    (*serialState).params.jobSize = jobSize as u32 as usize;
    return 0;
}
unsafe extern "C" fn ZSTDMT_serialState_init(
    mut serialState: *mut SerialState,
) -> i32 {
    let mut initError: i32 = 0;
    libc::memset(
        serialState as *mut c_void,
        0,
        ::core::mem::size_of::<SerialState>() as usize,
    );
    initError |= ZSTD_pthread_mutex_init!(& serialState -> mutex, NULL);
    initError |= ZSTD_pthread_cond_init!(& serialState -> cond, NULL);
    initError |= ZSTD_pthread_mutex_init!(& serialState -> ldmWindowMutex, NULL);
    initError |= ZSTD_pthread_cond_init!(& serialState -> ldmWindowCond, NULL);
    return initError;
}
unsafe extern "C" fn ZSTDMT_serialState_free(mut serialState: *mut SerialState) {
    let mut cMem = (*serialState).params.customMem;
    ZSTD_pthread_mutex_destroy!(
        & serialState -> mutex
    )(ZSTD_pthread_mutex_destroy!(& serialState -> mutex));
    ZSTD_pthread_cond_destroy!(
        & serialState -> cond
    )(ZSTD_pthread_cond_destroy!(& serialState -> cond));
    ZSTD_pthread_mutex_destroy!(
        & serialState -> ldmWindowMutex
    )(ZSTD_pthread_mutex_destroy!(& serialState -> ldmWindowMutex));
    ZSTD_pthread_cond_destroy!(
        & serialState -> ldmWindowCond
    )(ZSTD_pthread_cond_destroy!(& serialState -> ldmWindowCond));
    ZSTD_customFree((*serialState).ldmState.hashTable as *mut c_void, cMem);
    ZSTD_customFree(
        (*serialState).ldmState.bucketOffsets as *mut c_void,
        cMem,
    );
}
unsafe extern "C" fn ZSTDMT_serialState_genSequences(
    mut serialState: *mut SerialState,
    mut seqStore: *mut RawSeqStore_t,
    mut src: Range,
    mut jobID: u32,
) {
    ZSTD_PTHREAD_MUTEX_LOCK!(
        & serialState -> mutex
    )(ZSTD_PTHREAD_MUTEX_LOCK!(& serialState -> mutex));
    while (*serialState).nextJobID < jobID {
        ZSTD_pthread_cond_wait!(
            & serialState -> cond, & serialState -> mutex
        )(
            ZSTD_pthread_cond_wait!(& serialState -> cond, & serialState -> mutex),
            ZSTD_pthread_cond_wait!(& serialState -> cond, & serialState -> mutex),
        );
    }
    if (*serialState).nextJobID == jobID {
        if (*serialState).params.ldmParams.enableLdm as u32
            == ZSTD_ps_enable as i32 as u32
        {
            let mut error: usize = 0;
            ZSTD_window_update(
                &mut (*serialState).ldmState.window,
                src.start,
                src.size,
                0,
            );
            error = ZSTD_ldm_generateSequences(
                &mut (*serialState).ldmState,
                seqStore,
                &mut (*serialState).params.ldmParams,
                src.start,
                src.size,
            );
            ZSTD_PTHREAD_MUTEX_LOCK!(
                & serialState -> ldmWindowMutex
            )(ZSTD_PTHREAD_MUTEX_LOCK!(& serialState -> ldmWindowMutex));
            (*serialState).ldmWindow = (*serialState).ldmState.window;
            ZSTD_pthread_cond_signal!(
                & serialState -> ldmWindowCond
            )(ZSTD_pthread_cond_signal!(& serialState -> ldmWindowCond));
            ZSTD_pthread_mutex_unlock!(
                & serialState -> ldmWindowMutex
            )(ZSTD_pthread_mutex_unlock!(& serialState -> ldmWindowMutex));
        }
        if (*serialState).params.fParams.checksumFlag != 0
            && src.size > 0
        {
            ZSTD_XXH64_update(&mut (*serialState).xxhState, src.start, src.size);
        }
    }
    (*serialState).nextJobID = ((*serialState).nextJobID).wrapping_add(1);
    (*serialState).nextJobID;
    ZSTD_pthread_cond_broadcast!(
        & serialState -> cond
    )(ZSTD_pthread_cond_broadcast!(& serialState -> cond));
    ZSTD_pthread_mutex_unlock!(
        & serialState -> mutex
    )(ZSTD_pthread_mutex_unlock!(& serialState -> mutex));
}
unsafe extern "C" fn ZSTDMT_serialState_applySequences(
    mut serialState: *const SerialState,
    mut jobCCtx: *mut ZSTD_CCtx,
    mut seqStore: *const RawSeqStore_t,
) {
    if (*seqStore).size > 0 {
        ZSTD_referenceExternalSequences(jobCCtx, (*seqStore).seq, (*seqStore).size);
    }
}
unsafe extern "C" fn ZSTDMT_serialState_ensureFinished(
    mut serialState: *mut SerialState,
    mut jobID: u32,
    mut cSize: usize,
) {
    ZSTD_PTHREAD_MUTEX_LOCK!(
        & serialState -> mutex
    )(ZSTD_PTHREAD_MUTEX_LOCK!(& serialState -> mutex));
    if (*serialState).nextJobID <= jobID {
        (*serialState)
            .nextJobID = jobID.wrapping_add(1);
        ZSTD_pthread_cond_broadcast!(
            & serialState -> cond
        )(ZSTD_pthread_cond_broadcast!(& serialState -> cond));
        ZSTD_PTHREAD_MUTEX_LOCK!(
            & serialState -> ldmWindowMutex
        )(ZSTD_PTHREAD_MUTEX_LOCK!(& serialState -> ldmWindowMutex));
        ZSTD_window_clear(&mut (*serialState).ldmWindow);
        ZSTD_pthread_cond_signal!(
            & serialState -> ldmWindowCond
        )(ZSTD_pthread_cond_signal!(& serialState -> ldmWindowCond));
        ZSTD_pthread_mutex_unlock!(
            & serialState -> ldmWindowMutex
        )(ZSTD_pthread_mutex_unlock!(& serialState -> ldmWindowMutex));
    }
    ZSTD_pthread_mutex_unlock!(
        & serialState -> mutex
    )(ZSTD_pthread_mutex_unlock!(& serialState -> mutex));
}
static mut kNullRange: Range = {
    let mut init = Range {
        start: NULL_0 as *const c_void,
        size: 0,
    };
    init
};
unsafe extern "C" fn ZSTDMT_compressionJob(mut jobDescription: *mut c_void) {
    let mut current_block: u64;
    let job = jobDescription as *mut ZSTDMT_jobDescription;
    let mut jobParams = (*job).params;
    let cctx = ZSTDMT_getCCtx((*job).cctxPool);
    let mut rawSeqStore = ZSTDMT_getSeq((*job).seqPool);
    let mut dstBuff = (*job).dstBuff;
    let mut lastCBlockSize: usize = 0;
    if cctx.is_null() {
        pthread_mutex_lock(&mut (*job).job_mutex);
        (*job).cSize = ERROR(ZSTD_error_memory_allocation);
        pthread_mutex_unlock(&mut (*job).job_mutex);
    } else {
        if (dstBuff.start).is_null() {
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if (dstBuff.start).is_null() {
                pthread_mutex_lock(&mut (*job).job_mutex);
                (*job)
                    .cSize = -(ZSTD_error_memory_allocation as i32)
                    as usize;
                pthread_mutex_unlock(&mut (*job).job_mutex);
                current_block = 12352469457211969742;
            } else {
                (*job).dstBuff = dstBuff;
                current_block = 7976072742316086414;
            }
        } else {
            current_block = 7976072742316086414;
        }
        match current_block {
            12352469457211969742 => {}
            _ => {
                if jobParams.ldmParams.enableLdm as u32
                    == ZSTD_ps_enable as i32 as u32
                    && (rawSeqStore.seq).is_null()
                {
                    pthread_mutex_lock(&mut (*job).job_mutex);
                    (*job)
                        .cSize = -(ZSTD_error_memory_allocation as i32)
                        as usize;
                    pthread_mutex_unlock(&mut (*job).job_mutex);
                } else {
                    if (*job).jobID != 0 {
                        jobParams.fParams.checksumFlag = 0;
                    }
                    jobParams.ldmParams.enableLdm = ZSTD_ps_disable;
                    jobParams.nbWorkers = 0;
                    ZSTDMT_serialState_genSequences(
                        (*job).serial,
                        &mut rawSeqStore,
                        (*job).src,
                        (*job).jobID,
                    );
                    if !((*job).cdict).is_null() {
                        let initError = ZSTD_compressBegin_advanced_internal(
                            cctx,
                            NULL_0 as *const c_void,
                            0,
                            ZSTD_dct_auto,
                            ZSTD_dtlm_fast,
                            (*job).cdict,
                            &mut jobParams,
                            (*job).fullFrameSize,
                        );
                        if ERR_isError(initError) {
                            pthread_mutex_lock(&mut (*job).job_mutex);
                            let ref mut fresh4 = JOB_ERROR(ZSTD_error_initError);
                            *fresh4 = initError;
                            pthread_mutex_unlock(&mut (*job).job_mutex);
                            current_block = 12352469457211969742;
                        } else {
                            current_block = 16738040538446813684;
                        }
                    } else {
                        let pledgedSrcSize = (if (*job).firstJob != 0 {
                            (*job).fullFrameSize
                        } else {
                            (*job).src.size as u64
                        }) as u64;
                        let forceWindowError = ZSTD_CCtxParams_setParameter(
                            &mut jobParams,
                            ZSTD_c_forceMaxWindow as ZSTD_cParameter,
                            ((*job).firstJob == 0) as i32,
                        );
                        if ERR_isError(forceWindowError) {
                            pthread_mutex_lock(&mut (*job).job_mutex);
                            let ref mut fresh5 = JOB_ERROR(ZSTD_error_forceWindowError);
                            *fresh5 = forceWindowError;
                            pthread_mutex_unlock(&mut (*job).job_mutex);
                            current_block = 12352469457211969742;
                        } else {
                            if (*job).firstJob == 0 {
                                let err = ZSTD_CCtxParams_setParameter(
                                    &mut jobParams,
                                    ZSTD_c_deterministicRefPrefix as ZSTD_cParameter,
                                    0,
                                );
                                if ERR_isError(err) {
                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                    let ref mut fresh6 = JOB_ERROR(ZSTD_error_err);
                                    *fresh6 = err;
                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                    current_block = 12352469457211969742;
                                } else {
                                    current_block = 2543120759711851213;
                                }
                            } else {
                                current_block = 2543120759711851213;
                            }
                            match current_block {
                                12352469457211969742 => {}
                                _ => {
                                    let initError_0 = ZSTD_compressBegin_advanced_internal(
                                        cctx,
                                        (*job).prefix.start,
                                        (*job).prefix.size,
                                        ZSTD_dct_rawContent,
                                        ZSTD_dtlm_fast,
                                        NULL_0 as *const ZSTD_CDict,
                                        &mut jobParams,
                                        pledgedSrcSize as u64,
                                    );
                                    if ERR_isError(initError_0) {
                                        pthread_mutex_lock(&mut (*job).job_mutex);
                                        let ref mut fresh7 = JOB_ERROR(ZSTD_error_initError);
                                        *fresh7 = initError_0;
                                        pthread_mutex_unlock(&mut (*job).job_mutex);
                                        current_block = 12352469457211969742;
                                    } else {
                                        current_block = 16738040538446813684;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        12352469457211969742 => {}
                        _ => {
                            ZSTDMT_serialState_applySequences(
                                (*job).serial,
                                cctx,
                                &mut rawSeqStore,
                            );
                            if (*job).firstJob == 0 {
                                let hSize = ZSTD_compressContinue_public(
                                    cctx,
                                    dstBuff.start,
                                    dstBuff.capacity,
                                    (*job).src.start,
                                    0,
                                );
                                if ERR_isError(hSize) {
                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                    let ref mut fresh8 = JOB_ERROR(ZSTD_error_hSize);
                                    *fresh8 = hSize;
                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                    current_block = 12352469457211969742;
                                } else {
                                    ZSTD_invalidateRepCodes(cctx);
                                    current_block = 6560072651652764009;
                                }
                            } else {
                                current_block = 6560072651652764009;
                            }
                            match current_block {
                                12352469457211969742 => {}
                                _ => {
                                    let chunkSize = (4 as i32 * ZSTD_BLOCKSIZE_MAX)
                                        as usize;
                                    let nbChunks = (((*job).src.size)
                                        .wrapping_add(
                                            chunkSize.wrapping_sub(1),
                                        ) / chunkSize) as i32;
                                    let mut ip = (*job).src.start as *const u8;
                                    let ostart = dstBuff.start as *mut u8;
                                    let mut op = ostart;
                                    let mut oend = op.offset(dstBuff.capacity as isize);
                                    let mut chunkNb: i32 = 0;
                                    ::core::mem::size_of::<usize>()
                                        > ::core::mem::size_of::<i32>()
                                            as std::ffi::c_ulong;
                                    chunkNb = 1;
                                    loop {
                                        if !(chunkNb < nbChunks) {
                                            current_block = 851619935621435220;
                                            break;
                                        }
                                        let cSize = ZSTD_compressContinue_public(
                                            cctx,
                                            op as *mut c_void,
                                            oend.offset_from(op) as std::ffi::c_long as usize,
                                            ip as *const c_void,
                                            chunkSize,
                                        );
                                        if ERR_isError(cSize) {
                                            pthread_mutex_lock(&mut (*job).job_mutex);
                                            let ref mut fresh9 = JOB_ERROR(ZSTD_error_cSize);
                                            *fresh9 = cSize;
                                            pthread_mutex_unlock(&mut (*job).job_mutex);
                                            current_block = 12352469457211969742;
                                            break;
                                        } else {
                                            ip = ip.offset(chunkSize as isize);
                                            op = op.offset(cSize as isize);
                                            ZSTD_PTHREAD_MUTEX_LOCK!(
                                                & job -> job_mutex
                                            )(ZSTD_PTHREAD_MUTEX_LOCK!(& job -> job_mutex));
                                            (*job).cSize = ((*job).cSize).wrapping_add(cSize);
                                            (*job).consumed = chunkSize * chunkNb as usize;
                                            ZSTD_pthread_cond_signal!(
                                                & job -> job_cond
                                            )(ZSTD_pthread_cond_signal!(& job -> job_cond));
                                            ZSTD_pthread_mutex_unlock!(
                                                & job -> job_mutex
                                            )(ZSTD_pthread_mutex_unlock!(& job -> job_mutex));
                                            chunkNb += 1;
                                            chunkNb;
                                        }
                                    }
                                    match current_block {
                                        12352469457211969742 => {}
                                        _ => {
                                            if (nbChunks > 0) as i32
                                                as u32 | (*job).lastJob != 0
                                            {
                                                let lastBlockSize1 = (*job).src.size
                                                    & chunkSize.wrapping_sub(1);
                                                let lastBlockSize = if (lastBlockSize1
                                                    == 0) as i32
                                                    & ((*job).src.size >= chunkSize) as i32 != 0
                                                {
                                                    chunkSize
                                                } else {
                                                    lastBlockSize1
                                                };
                                                let cSize_0 = if (*job).lastJob != 0 {
                                                    ZSTD_compressEnd_public(
                                                        cctx,
                                                        op as *mut c_void,
                                                        oend.offset_from(op) as std::ffi::c_long as usize,
                                                        ip as *const c_void,
                                                        lastBlockSize,
                                                    )
                                                } else {
                                                    ZSTD_compressContinue_public(
                                                        cctx,
                                                        op as *mut c_void,
                                                        oend.offset_from(op) as std::ffi::c_long as usize,
                                                        ip as *const c_void,
                                                        lastBlockSize,
                                                    )
                                                };
                                                if ERR_isError(cSize_0) {
                                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                                    let ref mut fresh10 = JOB_ERROR(ZSTD_error_cSize);
                                                    *fresh10 = cSize_0;
                                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                                    current_block = 12352469457211969742;
                                                } else {
                                                    lastCBlockSize = cSize_0;
                                                    current_block = 200744462051969938;
                                                }
                                            } else {
                                                current_block = 200744462051969938;
                                            }
                                            match current_block {
                                                12352469457211969742 => {}
                                                _ => {
                                                    (*job).firstJob == 0;
                                                    ZSTD_CCtx_trace(cctx, 0);
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
    }
    ZSTDMT_serialState_ensureFinished((*job).serial, (*job).jobID, (*job).cSize);
    (*job).prefix.size > 0;
    ZSTDMT_releaseSeq((*job).seqPool, rawSeqStore);
    ZSTDMT_releaseCCtx((*job).cctxPool, cctx);
    ZSTD_PTHREAD_MUTEX_LOCK!(
        & job -> job_mutex
    )(ZSTD_PTHREAD_MUTEX_LOCK!(& job -> job_mutex));
    ERR_isError((*job).cSize) != 0;
    (*job).cSize = ((*job).cSize).wrapping_add(lastCBlockSize);
    (*job).consumed = (*job).src.size;
    ZSTD_pthread_cond_signal!(
        & job -> job_cond
    )(ZSTD_pthread_cond_signal!(& job -> job_cond));
    ZSTD_pthread_mutex_unlock!(
        & job -> job_mutex
    )(ZSTD_pthread_mutex_unlock!(& job -> job_mutex));
}
static mut kNullRoundBuff: RoundBuff_t = {
    let mut init = RoundBuff_t {
        buffer: NULL_0 as *mut u8,
        capacity: 0,
        pos: 0,
    };
    init
};
pub const RSYNC_LENGTH: i32 = 32;
pub const RSYNC_MIN_BLOCK_LOG: i32 = ZSTD_BLOCKSIZELOG_MAX;
pub const RSYNC_MIN_BLOCK_SIZE: i32 = (1 as i32)
    << RSYNC_MIN_BLOCK_LOG;
unsafe extern "C" fn ZSTDMT_freeJobsTable(
    mut jobTable: *mut ZSTDMT_jobDescription,
    mut nbJobs: u32,
    mut cMem: ZSTD_customMem,
) {
    let mut jobNb: u32 = 0;
    if jobTable.is_null() {
        return;
    }
    jobNb = 0;
    while jobNb < nbJobs {
        ZSTD_pthread_mutex_destroy!(
            & jobTable[jobNb].job_mutex
        )(ZSTD_pthread_mutex_destroy!(& jobTable[jobNb].job_mutex));
        ZSTD_pthread_cond_destroy!(
            & jobTable[jobNb].job_cond
        )(ZSTD_pthread_cond_destroy!(& jobTable[jobNb].job_cond));
        jobNb = jobNb.wrapping_add(1);
        jobNb;
    }
    ZSTD_customFree(jobTable as *mut c_void, cMem);
}
unsafe extern "C" fn ZSTDMT_createJobsTable(
    mut nbJobsPtr: *mut u32,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_jobDescription {
    let nbJobsLog2 = (ZSTD_highbit32(*nbJobsPtr))
        .wrapping_add(1);
    let nbJobs = ((1 as i32) << nbJobsLog2) as u32;
    let mut jobNb: u32 = 0;
    let jobTable = ZSTD_customCalloc(
        (nbJobs as std::ffi::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<ZSTDMT_jobDescription>(),
            ),
        cMem,
    ) as *mut ZSTDMT_jobDescription;
    let mut initError: i32 = 0;
    if jobTable.is_null() {
        return NULL_0 as *mut ZSTDMT_jobDescription;
    }
    *nbJobsPtr = nbJobs;
    jobNb = 0;
    while jobNb < nbJobs {
        initError |= ZSTD_pthread_mutex_init!(& jobTable[jobNb].job_mutex, NULL);
        initError |= ZSTD_pthread_cond_init!(& jobTable[jobNb].job_cond, NULL);
        jobNb = jobNb.wrapping_add(1);
        jobNb;
    }
    if initError != 0 {
        ZSTDMT_freeJobsTable(jobTable, nbJobs, cMem);
        return NULL_0 as *mut ZSTDMT_jobDescription;
    }
    return jobTable;
}
unsafe extern "C" fn ZSTDMT_expandJobsTable(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut nbWorkers: u32,
) -> usize {
    let mut nbJobs = nbWorkers.wrapping_add(2);
    if nbJobs
        > ((*mtctx).jobIDMask).wrapping_add(1)
    {
        ZSTDMT_freeJobsTable(
            (*mtctx).jobs,
            ((*mtctx).jobIDMask).wrapping_add(1),
            (*mtctx).cMem,
        );
        (*mtctx).jobIDMask = 0;
        (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, (*mtctx).cMem);
        RETURN_ERROR_IF!(((*mtctx).jobs).is_null(), ZSTD_error_memory_allocation);
        (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    }
    return 0;
}
unsafe extern "C" fn ZSTDMT_CCtxParam_setNbWorkers(
    mut params: *mut ZSTD_CCtx_params,
    mut nbWorkers: u32,
) -> usize {
    return ZSTD_CCtxParams_setParameter(
        params,
        ZSTD_c_nbWorkers,
        nbWorkers as i32,
    );
}
#[inline]
unsafe extern "C" fn ZSTDMT_createCCtx_advanced_internal(
    mut nbWorkers: u32,
    mut cMem: ZSTD_customMem,
    mut pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    let mut mtctx = std::ptr::null_mut();
    let mut nbJobs = nbWorkers.wrapping_add(2);
    let mut initError: i32 = 0;
    if nbWorkers < 1 {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    nbWorkers = std::cmp::min(nbWorkers, ZSTDMT_NBWORKERS_MAX);
    if (cMem.customAlloc).is_some() as i32
        ^ (cMem.customFree).is_some() as i32 != 0
    {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    mtctx = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTDMT_CCtx>(),
        cMem,
    ) as *mut ZSTDMT_CCtx;
    if mtctx.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    (*mtctx).cMem = cMem;
    (*mtctx).allJobsCompleted = 1;
    if !pool.is_null() {
        (*mtctx).factory = pool;
        (*mtctx).set_providedFactory(1 as u32);
    } else {
        (*mtctx)
            .factory = POOL_create_advanced(
            nbWorkers as usize,
            0,
            cMem,
        );
        (*mtctx).set_providedFactory(0 as u32);
    }
    (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, cMem);
    (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    (*mtctx)
        .bufPool = ZSTDMT_createBufferPool(BUF_POOL_MAX_NB_BUFFERS!(nbWorkers), cMem);
    (*mtctx).cctxPool = ZSTDMT_createCCtxPool(nbWorkers as i32, cMem);
    (*mtctx).seqPool = ZSTDMT_createSeqPool(nbWorkers, cMem);
    initError = ZSTDMT_serialState_init(&mut (*mtctx).serial);
    (*mtctx).roundBuff = kNullRoundBuff;
    if ((*mtctx).factory).is_null() as i32
        | ((*mtctx).jobs).is_null() as i32
        | ((*mtctx).bufPool).is_null() as i32
        | ((*mtctx).cctxPool).is_null() as i32
        | ((*mtctx).seqPool).is_null() as i32 | initError != 0
    {
        ZSTDMT_freeCCtx(mtctx);
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    return mtctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_createCCtx_advanced(
    mut nbWorkers: u32,
    mut cMem: ZSTD_customMem,
    mut pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    return ZSTDMT_createCCtx_advanced_internal(nbWorkers, cMem, pool);
}
unsafe extern "C" fn ZSTDMT_releaseAllJobResources(mut mtctx: *mut ZSTDMT_CCtx) {
    let mut jobID: u32 = 0;
    jobID = 0;
    while jobID <= (*mtctx).jobIDMask {
        let mutex = (*((*mtctx).jobs).offset(jobID as isize)).job_mutex;
        let cond = (*((*mtctx).jobs).offset(jobID as isize)).job_cond;
        ZSTDMT_releaseBuffer(
            (*mtctx).bufPool,
            (*((*mtctx).jobs).offset(jobID as isize)).dstBuff,
        );
        libc::memset(
            &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription
                as *mut c_void,
            0,
            ::core::mem::size_of::<ZSTDMT_jobDescription>()
                as usize,
        );
        (*((*mtctx).jobs).offset(jobID as isize)).job_mutex = mutex;
        (*((*mtctx).jobs).offset(jobID as isize)).job_cond = cond;
        jobID = jobID.wrapping_add(1);
        jobID;
    }
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0;
    (*mtctx).allJobsCompleted = 1;
}
unsafe extern "C" fn ZSTDMT_waitForAllJobsCompleted(mut mtctx: *mut ZSTDMT_CCtx) {
    while (*mtctx).doneJobID < (*mtctx).nextJobID {
        let jobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
        ZSTD_PTHREAD_MUTEX_LOCK!(
            & mtctx -> jobs[jobID].job_mutex
        )(ZSTD_PTHREAD_MUTEX_LOCK!(& mtctx -> jobs[jobID].job_mutex));
        while (*((*mtctx).jobs).offset(jobID as isize)).consumed
            < (*((*mtctx).jobs).offset(jobID as isize)).src.size
        {
            ZSTD_pthread_cond_wait!(
                & mtctx -> jobs[jobID].job_cond, & mtctx -> jobs[jobID].job_mutex
            )(
                ZSTD_pthread_cond_wait!(
                    & mtctx -> jobs[jobID].job_cond, & mtctx -> jobs[jobID].job_mutex
                ),
                ZSTD_pthread_cond_wait!(
                    & mtctx -> jobs[jobID].job_cond, & mtctx -> jobs[jobID].job_mutex
                ),
            );
        }
        ZSTD_pthread_mutex_unlock!(
            & mtctx -> jobs[jobID].job_mutex
        )(ZSTD_pthread_mutex_unlock!(& mtctx -> jobs[jobID].job_mutex));
        (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
        (*mtctx).doneJobID;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_freeCCtx(mut mtctx: *mut ZSTDMT_CCtx) -> usize {
    if mtctx.is_null() {
        return 0;
    }
    if (*mtctx).providedFactory() == 0 {
        POOL_free((*mtctx).factory);
    }
    ZSTDMT_releaseAllJobResources(mtctx);
    ZSTDMT_freeJobsTable(
        (*mtctx).jobs,
        ((*mtctx).jobIDMask).wrapping_add(1),
        (*mtctx).cMem,
    );
    ZSTDMT_freeBufferPool((*mtctx).bufPool);
    ZSTDMT_freeCCtxPool((*mtctx).cctxPool);
    ZSTDMT_freeSeqPool((*mtctx).seqPool);
    ZSTDMT_serialState_free(&mut (*mtctx).serial);
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !((*mtctx).roundBuff.buffer).is_null() {
        ZSTD_customFree(
            (*mtctx).roundBuff.buffer as *mut c_void,
            (*mtctx).cMem,
        );
    }
    ZSTD_customFree(mtctx as *mut c_void, (*mtctx).cMem);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_sizeof_CCtx(mut mtctx: *mut ZSTDMT_CCtx) -> usize {
    if mtctx.is_null() {
        return 0;
    }
    return (::core::mem::size_of::<ZSTDMT_CCtx>())
        .wrapping_add(POOL_sizeof((*mtctx).factory))
        .wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool))
        .wrapping_add(
            (((*mtctx).jobIDMask).wrapping_add(1)
                as std::ffi::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<ZSTDMT_jobDescription>(),
                ),
        )
        .wrapping_add(ZSTDMT_sizeof_CCtxPool((*mtctx).cctxPool))
        .wrapping_add(ZSTDMT_sizeof_seqPool((*mtctx).seqPool))
        .wrapping_add(ZSTD_sizeof_CDict((*mtctx).cdictLocal))
        .wrapping_add((*mtctx).roundBuff.capacity);
}
unsafe extern "C" fn ZSTDMT_resize(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut nbWorkers: u32,
) -> usize {
    RETURN_ERROR_IF!(POOL_resize((*mtctx).factory, nbWorkers as usize) != 0, ZSTD_error_memory_allocation);
    FORWARD_IF_ERROR!(ZSTDMT_expandJobsTable(mtctx, nbWorkers), "");
    (*mtctx)
        .bufPool = ZSTDMT_expandBufferPool(
        (*mtctx).bufPool,
        BUF_POOL_MAX_NB_BUFFERS!(nbWorkers),
    );
    RETURN_ERROR_IF!(((*mtctx).bufPool).is_null(), ZSTD_error_memory_allocation);
    (*mtctx)
        .cctxPool = ZSTDMT_expandCCtxPool(
        (*mtctx).cctxPool,
        nbWorkers as i32,
    );
    RETURN_ERROR_IF!(((*mtctx).cctxPool).is_null(), ZSTD_error_memory_allocation);
    (*mtctx).seqPool = ZSTDMT_expandSeqPool((*mtctx).seqPool, nbWorkers);
    RETURN_ERROR_IF!(((*mtctx).seqPool).is_null(), ZSTD_error_memory_allocation);
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_updateCParams_whileCompressing(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut cctxParams: *const ZSTD_CCtx_params,
) {
    let saved_wlog = (*mtctx).params.cParams.windowLog;
    let compressionLevel = (*cctxParams).compressionLevel;
    (*mtctx).params.compressionLevel = compressionLevel;
    let mut cParams = ZSTD_getCParamsFromCCtxParams(
        cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        0,
        ZSTD_cpm_noAttachDict,
    );
    cParams.windowLog = saved_wlog;
    (*mtctx).params.cParams = cParams;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_getFrameProgression(
    mut mtctx: *mut ZSTDMT_CCtx,
) -> ZSTD_frameProgression {
    let mut fps = ZSTD_frameProgression {
        ingested: 0,
        consumed: 0,
        produced: 0,
        flushed: 0,
        currentJobID: 0,
        nbActiveWorkers: 0,
    };
    fps
        .ingested = ((*mtctx).consumed)
        .wrapping_add((*mtctx).inBuff.filled as u64);
    fps.consumed = (*mtctx).consumed;
    fps.flushed = (*mtctx).produced;
    fps.produced = fps.flushed;
    fps.currentJobID = (*mtctx).nextJobID;
    fps.nbActiveWorkers = 0;
    let mut jobNb: u32 = 0;
    let mut lastJobNb = ((*mtctx).nextJobID)
        .wrapping_add((*mtctx).jobReady as u32);
    jobNb = (*mtctx).doneJobID;
    while jobNb < lastJobNb {
        let wJobID = jobNb & (*mtctx).jobIDMask;
        let mut jobPtr: *mut ZSTDMT_jobDescription = &mut *((*mtctx).jobs)
            .offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
        ZSTD_pthread_mutex_lock!(
            & jobPtr -> job_mutex
        )(ZSTD_pthread_mutex_lock!(& jobPtr -> job_mutex));
        let cResult = (*jobPtr).cSize;
        let produced = if ERR_isError(cResult) {
            0_usize
        } else {
            cResult
        };
        let flushed = if ERR_isError(cResult) {
            0_usize
        } else {
            (*jobPtr).dstFlushed
        };
        fps
            .ingested = (fps.ingested)
            .wrapping_add((*jobPtr).src.size as u64);
        fps
            .consumed = (fps.consumed)
            .wrapping_add((*jobPtr).consumed as u64);
        fps.produced = (fps.produced).wrapping_add(produced as u64);
        fps.flushed = (fps.flushed).wrapping_add(flushed as u64);
        fps
            .nbActiveWorkers = (fps.nbActiveWorkers)
            .wrapping_add(
                ((*jobPtr).consumed < (*jobPtr).src.size) as i32
                    as u32,
            );
        ZSTD_pthread_mutex_unlock!(
            & mtctx -> jobs[wJobID].job_mutex
        )(ZSTD_pthread_mutex_unlock!(& mtctx -> jobs[wJobID].job_mutex));
        jobNb = jobNb.wrapping_add(1);
        jobNb;
    }
    return fps;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_toFlushNow(mut mtctx: *mut ZSTDMT_CCtx) -> usize {
    let mut toFlush: usize = 0;
    let jobID = (*mtctx).doneJobID;
    if jobID == (*mtctx).nextJobID {
        return 0;
    }
    let wJobID = jobID & (*mtctx).jobIDMask;
    let jobPtr: *mut ZSTDMT_jobDescription = &mut *((*mtctx).jobs)
        .offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
    ZSTD_pthread_mutex_lock!(
        & jobPtr -> job_mutex
    )(ZSTD_pthread_mutex_lock!(& jobPtr -> job_mutex));
    let cResult = (*jobPtr).cSize;
    let produced = if ERR_isError(cResult) {
        0_usize
    } else {
        cResult
    };
    let flushed = if ERR_isError(cResult) {
        0_usize
    } else {
        (*jobPtr).dstFlushed
    };
    toFlush = produced.wrapping_sub(flushed);
    toFlush == 0;
    ZSTD_pthread_mutex_unlock!(
        & mtctx -> jobs[wJobID].job_mutex
    )(ZSTD_pthread_mutex_unlock!(& mtctx -> jobs[wJobID].job_mutex));
    return toFlush;
}

const ZSTDMT_JOBLOG_MAX: u32 = if usize::BITS == 32 {
    29
} else {
    30
};

unsafe extern "C" fn ZSTDMT_computeTargetJobLog(
    mut params: *const ZSTD_CCtx_params,
) -> u32 {
    let mut jobLog: u32 = 0;
    if (*params).ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        jobLog = std::cmp::max(
            21, ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy) + 3
        );
    } else {
        jobLog = std::cmp::max(20, (*params).cParams.windowLog + 2);
    }
    return std::cmp::min(jobLog, ZSTDMT_JOBLOG_MAX);
}
unsafe extern "C" fn ZSTDMT_overlapLog_default(
    mut strat: ZSTD_strategy,
) -> i32 {
    match strat as u32 {
        9 => return 9,
        8 | 7 => return 8,
        6 | 5 => return 7,
        4 | 3 | 2 | 1 | _ => {}
    }
    return 6;
}
unsafe extern "C" fn ZSTDMT_overlapLog(
    mut ovlog: i32,
    mut strat: ZSTD_strategy,
) -> i32 {
    if ovlog == 0 {
        return ZSTDMT_overlapLog_default(strat);
    }
    return ovlog;
}
unsafe extern "C" fn ZSTDMT_computeOverlapSize(
    mut params: *const ZSTD_CCtx_params,
) -> usize {
    let overlapRLog = 9 as i32
        - ZSTDMT_overlapLog((*params).overlapLog, (*params).cParams.strategy);
    let mut ovLog = (if overlapRLog >= 8 {
        0 as u32
    } else {
        ((*params).cParams.windowLog).wrapping_sub(overlapRLog as u32)
    }) as i32;
    if (*params).ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        ovLog = std::cmp::min((*params).cParams.windowLog, ZSTDMT_computeTargetJobLog(params) - 2)
            .wrapping_sub(overlapRLog as u32) as i32;
    }
    return if ovLog == 0 {
        0_usize
    } else {
        1_usize << ovLog
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream_internal(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut dict: *const c_void,
    mut dictSize: usize,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
) -> usize {
    if params.nbWorkers != (*mtctx).params.nbWorkers {
        FORWARD_IF_ERROR!(
            ZSTDMT_resize(mtctx, (unsigned) params.nbWorkers), ""
        );
    }
    if params.jobSize != 0
        && params.jobSize < ZSTDMT_JOBSIZE_MIN as usize
    {
        params.jobSize = ZSTDMT_JOBSIZE_MIN as usize;
    }
    if params.jobSize
        > (if MEM_32bits {
            512 as i32 * ((1 as i32) << 20)
        } else {
            1024 as i32 * ((1 as i32) << 20)
        }) as usize
    {
        params
            .jobSize = (if MEM_32bits {
            512 as i32 * ((1 as i32) << 20)
        } else {
            1024 as i32 * ((1 as i32) << 20)
        }) as usize;
    }
    if (*mtctx).allJobsCompleted == 0 {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        (*mtctx).allJobsCompleted = 1;
    }
    (*mtctx).params = params;
    (*mtctx).frameContentSize = pledgedSrcSize;
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !dict.is_null() {
        (*mtctx)
            .cdictLocal = ZSTD_createCDict_advanced(
            dict,
            dictSize,
            ZSTD_dlm_byCopy,
            dictContentType,
            params.cParams,
            (*mtctx).cMem,
        );
        (*mtctx).cdict = (*mtctx).cdictLocal;
        RETURN_ERROR_IF!(((*mtctx).cdictLocal).is_null(), ZSTD_error_memory_allocation);
    } else {
        (*mtctx).cdictLocal = NULL_0 as *mut ZSTD_CDict;
        (*mtctx).cdict = cdict;
    }
    (*mtctx).targetPrefixSize = ZSTDMT_computeOverlapSize(&mut params);
    (*mtctx).targetSectionSize = params.jobSize;
    if (*mtctx).targetSectionSize == 0 {
        (*mtctx)
            .targetSectionSize = ((1 as u64)
            << ZSTDMT_computeTargetJobLog(&mut params)) as usize;
    }
    if params.rsyncable != 0 {
        let jobSizeKB = ((*mtctx).targetSectionSize >> 10) as u32;
        let rsyncBits = (ZSTD_highbit32(jobSizeKB))
            .wrapping_add(10);
        (*mtctx).rsync.hash = 0;
        (*mtctx)
            .rsync
            .hitMask = ((1 as u64) << rsyncBits)
            .wrapping_sub(1) as u64;
        (*mtctx).rsync.primePower = ZSTD_rollingHash_primePower(RSYNC_LENGTH as u32);
    }
    if (*mtctx).targetSectionSize < (*mtctx).targetPrefixSize {
        (*mtctx).targetSectionSize = (*mtctx).targetPrefixSize;
    }
    ZSTDMT_setBufferSize(
        (*mtctx).bufPool,
        ZSTD_compressBound((*mtctx).targetSectionSize),
    );
    let windowSize = (if (*mtctx).params.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        (1 as u32) << (*mtctx).params.cParams.windowLog
    } else {
        0 as u32
    }) as usize;
    let nbSlackBuffers = (2 as i32
        + ((*mtctx).targetPrefixSize > 0)
            as i32) as usize;
    let slackSize = (*mtctx).targetSectionSize * nbSlackBuffers;
    let nbWorkers = std::cmp::max((*mtctx).params.nbWorkers, 1);
    let sectionsSize = (*mtctx).targetSectionSize * nbWorkers;
    let capacity = std::cmp::max(windowSize, sectionsSize).wrapping_add(slackSize);
    if (*mtctx).roundBuff.capacity < capacity {
        if !((*mtctx).roundBuff.buffer).is_null() {
            ZSTD_customFree(
                (*mtctx).roundBuff.buffer as *mut c_void,
                (*mtctx).cMem,
            );
        }
        (*mtctx)
            .roundBuff
            .buffer = ZSTD_customMalloc(capacity, (*mtctx).cMem) as *mut u8;
        if ((*mtctx).roundBuff.buffer).is_null() {
            (*mtctx).roundBuff.capacity = 0;
            return ERROR(ZSTD_error_memory_allocation);
        }
        (*mtctx).roundBuff.capacity = capacity;
    }
    (*mtctx).roundBuff.pos = 0;
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0;
    (*mtctx).inBuff.prefix = kNullRange;
    (*mtctx).doneJobID = 0;
    (*mtctx).nextJobID = 0;
    (*mtctx).frameEnded = 0;
    (*mtctx).allJobsCompleted = 0;
    (*mtctx).consumed = 0;
    (*mtctx).produced = 0;
    ZSTD_freeCDict((*mtctx).cdictLocal);
    (*mtctx).cdictLocal = NULL_0 as *mut ZSTD_CDict;
    (*mtctx).cdict = NULL_0 as *const ZSTD_CDict;
    if !dict.is_null() {
        if dictContentType as u32
            == ZSTD_dct_rawContent as i32 as u32
        {
            (*mtctx)
                .inBuff
                .prefix
                .start = dict as *const u8 as *const c_void;
            (*mtctx).inBuff.prefix.size = dictSize;
        } else {
            (*mtctx)
                .cdictLocal = ZSTD_createCDict_advanced(
                dict,
                dictSize,
                ZSTD_dlm_byRef,
                dictContentType,
                params.cParams,
                (*mtctx).cMem,
            );
            (*mtctx).cdict = (*mtctx).cdictLocal;
            RETURN_ERROR_IF!(((*mtctx).cdictLocal).is_null(), ZSTD_error_memory_allocation);
        }
    } else {
        (*mtctx).cdict = cdict;
    }
    RETURN_ERROR_IF!(ZSTDMT_serialState_reset(
        &mut (*mtctx).serial,
        (*mtctx).seqPool,
        params,
        (*mtctx).targetSectionSize,
        dict,
        dictSize,
        dictContentType,
    ) != 0, ZSTD_error_memory_allocation);
    return 0;
}
unsafe extern "C" fn ZSTDMT_writeLastEmptyBlock(mut job: *mut ZSTDMT_jobDescription) {
    (*job).dstBuff = ZSTDMT_getBuffer((*job).bufPool);
    if ((*job).dstBuff.start).is_null() {
        (*job).cSize = ERROR(ZSTD_error_memory_allocation);
        return;
    }
    (*job).src = kNullRange;
    (*job)
        .cSize = ZSTD_writeLastEmptyBlock((*job).dstBuff.start, (*job).dstBuff.capacity);
}
unsafe extern "C" fn ZSTDMT_createCompressionJob(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut srcSize: usize,
    mut endOp: ZSTD_EndDirective,
) -> usize {
    let jobID = (*mtctx).nextJobID & (*mtctx).jobIDMask;
    let endFrame = (endOp as u32
        == ZSTD_e_end as i32 as u32) as i32;
    if (*mtctx).nextJobID > ((*mtctx).doneJobID).wrapping_add((*mtctx).jobIDMask) {
        return 0;
    }
    if (*mtctx).jobReady == 0 {
        let mut src = (*mtctx).inBuff.buffer.start as *const u8;
        let ref mut fresh11 = (*((*mtctx).jobs).offset(jobID as isize)).src.start;
        *fresh11 = src as *const c_void;
        (*((*mtctx).jobs).offset(jobID as isize)).src.size = srcSize;
        (*((*mtctx).jobs).offset(jobID as isize)).prefix = (*mtctx).inBuff.prefix;
        (*((*mtctx).jobs).offset(jobID as isize))
            .consumed = 0;
        (*((*mtctx).jobs).offset(jobID as isize)).cSize = 0;
        (*((*mtctx).jobs).offset(jobID as isize)).params = (*mtctx).params;
        let ref mut fresh12 = (*((*mtctx).jobs).offset(jobID as isize)).cdict;
        *fresh12 = if (*mtctx).nextJobID == 0 {
            (*mtctx).cdict
        } else {
            NULL_0 as *const ZSTD_CDict
        };
        (*((*mtctx).jobs).offset(jobID as isize))
            .fullFrameSize = (*mtctx).frameContentSize;
        (*((*mtctx).jobs).offset(jobID as isize)).dstBuff = g_nullBuffer;
        let ref mut fresh13 = (*((*mtctx).jobs).offset(jobID as isize)).cctxPool;
        *fresh13 = (*mtctx).cctxPool;
        let ref mut fresh14 = (*((*mtctx).jobs).offset(jobID as isize)).bufPool;
        *fresh14 = (*mtctx).bufPool;
        let ref mut fresh15 = (*((*mtctx).jobs).offset(jobID as isize)).seqPool;
        *fresh15 = (*mtctx).seqPool;
        let ref mut fresh16 = (*((*mtctx).jobs).offset(jobID as isize)).serial;
        *fresh16 = &mut (*mtctx).serial;
        (*((*mtctx).jobs).offset(jobID as isize)).jobID = (*mtctx).nextJobID;
        (*((*mtctx).jobs).offset(jobID as isize))
            .firstJob = ((*mtctx).nextJobID == 0)
            as i32 as u32;
        (*((*mtctx).jobs).offset(jobID as isize)).lastJob = endFrame as u32;
        (*((*mtctx).jobs).offset(jobID as isize))
            .frameChecksumNeeded = ((*mtctx).params.fParams.checksumFlag != 0
            && endFrame != 0
            && (*mtctx).nextJobID > 0)
            as i32 as u32;
        (*((*mtctx).jobs).offset(jobID as isize))
            .dstFlushed = 0;
        (*mtctx).roundBuff.pos = ((*mtctx).roundBuff.pos).wrapping_add(srcSize);
        (*mtctx).inBuff.buffer = g_nullBuffer;
        (*mtctx).inBuff.filled = 0;
        if endFrame == 0 {
            let newPrefixSize = std::cmp::min(srcSize, (*mtctx).targetPrefixSize);
            (*mtctx)
                .inBuff
                .prefix
                .start = src.offset(srcSize as isize).offset(-(newPrefixSize as isize))
                as *const c_void;
            (*mtctx).inBuff.prefix.size = newPrefixSize;
        } else {
            (*mtctx).inBuff.prefix = kNullRange;
            (*mtctx).frameEnded = endFrame as u32;
            if (*mtctx).nextJobID == 0 {
                (*mtctx).params.fParams.checksumFlag = 0;
            }
        }
        if srcSize == 0
            && (*mtctx).nextJobID > 0
        {
            ZSTDMT_writeLastEmptyBlock(((*mtctx).jobs).offset(jobID as isize));
            (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
            (*mtctx).nextJobID;
            return 0;
        }
    }
    if POOL_tryAdd(
        (*mtctx).factory,
        Some(ZSTDMT_compressionJob as unsafe extern "C" fn(*mut c_void) -> ()),
        &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription
            as *mut c_void,
    ) != 0
    {
        (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
        (*mtctx).nextJobID;
        (*mtctx).jobReady = 0;
    } else {
        (*mtctx).jobReady = 1;
    }
    return 0;
}
/** ZSTDMT_flushProduced() :
 *  flush whatever data has been produced but not yet flushed in current job.
 *  move to next job if current one is fully flushed.
 * `output` : `pos` will be updated with amount of data flushed .
 * `blockToFlush` : if >0, the function will block and wait if there is no data available to flush .
 * @return : amount of data remaining within internal buffer, 0 if no more, 1 if unknown but > 0, or an error code */
unsafe extern "C" fn ZSTDMT_flushProduced(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut blockToFlush: u32,
    mut end: ZSTD_EndDirective,
) -> usize {
    let wJobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
    ZSTD_PTHREAD_MUTEX_LOCK!(
        & mtctx -> jobs[wJobID].job_mutex
    )(ZSTD_PTHREAD_MUTEX_LOCK!(& mtctx -> jobs[wJobID].job_mutex));
    if blockToFlush != 0 && (*mtctx).doneJobID < (*mtctx).nextJobID {
        while (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed
            == (*((*mtctx).jobs).offset(wJobID as isize)).cSize /* nothing to flush */
        {
            if (*((*mtctx).jobs).offset(wJobID as isize)).consumed
                == (*((*mtctx).jobs).offset(wJobID as isize)).src.size
            {
                break;
            }
            ZSTD_pthread_cond_wait!(
                & mtctx -> jobs[wJobID].job_cond, & mtctx -> jobs[wJobID].job_mutex
            )(
                ZSTD_pthread_cond_wait!(
                    & mtctx -> jobs[wJobID].job_cond, & mtctx -> jobs[wJobID].job_mutex
                ),
                ZSTD_pthread_cond_wait!(
                    & mtctx -> jobs[wJobID].job_cond, & mtctx -> jobs[wJobID].job_mutex
                ),
            );
        }
    }
    /* try to flush something */
    let mut cSize = (*((*mtctx).jobs).offset(wJobID as isize)).cSize; /* shared */
    let srcConsumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed; /* shared */
    let srcSize = (*((*mtctx).jobs).offset(wJobID as isize)).src.size; /* read-only, could be done after mutex lock, but no-declaration-after-statement */
    ZSTD_pthread_mutex_unlock!(
        & mtctx -> jobs[wJobID].job_mutex
    )(ZSTD_pthread_mutex_unlock!(& mtctx -> jobs[wJobID].job_mutex));
    if ERR_isError(cSize) {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize;
    }
    /* add frame checksum if necessary (can only happen once) */
    if srcConsumed == srcSize /* job completed -> worker no longer active */
        && (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded != 0
    {
        let checksum = ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as u32;
        MEM_writeLE32(
            ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start
                as *mut c_char)
                .offset((*((*mtctx).jobs).offset(wJobID as isize)).cSize as isize)
                as *mut c_void,
            checksum,
        );
        cSize = cSize.wrapping_add(4); /* can write this shared value, as worker is no longer active */
        let ref mut fresh17 = (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
        *fresh17 = (*fresh17).wrapping_add(4);
        (*((*mtctx).jobs).offset(wJobID as isize))
            .frameChecksumNeeded = 0;
    }
    if cSize > 0 { /* compression is ongoing or completed */
        let toFlush = std::cmp::min(
            cSize - (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed, (*output).size - (*output).pos
        );
        if toFlush > 0 {
            libc::memcpy((*output).dst.byte_add((*output).pos),
            (*(*mtctx).jobs.offset(wJobID as isize)).dstBuff.start.byte_add((*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed), toFlush);
        }
        (*output).pos = ((*output).pos).wrapping_add(toFlush);
        let ref mut fresh18 = (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed;
        *fresh18 = (*fresh18).wrapping_add(toFlush);
        if srcConsumed == srcSize
            && (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed == cSize
        {
            ZSTDMT_releaseBuffer(
                (*mtctx).bufPool,
                (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff,
            );
            (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff = g_nullBuffer;
            (*((*mtctx).jobs).offset(wJobID as isize))
                .cSize = 0;
            (*mtctx)
                .consumed = ((*mtctx).consumed)
                .wrapping_add(srcSize as u64);
            (*mtctx)
                .produced = ((*mtctx).produced)
                .wrapping_add(cSize as u64);
            (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
            (*mtctx).doneJobID;
        }
    }
    if cSize > (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed {
        return cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed);
    }
    if srcSize > srcConsumed {
        return 1;
    }
    if (*mtctx).doneJobID < (*mtctx).nextJobID {
        return 1;
    }
    if (*mtctx).jobReady != 0 {
        return 1;
    }
    if (*mtctx).inBuff.filled > 0 {
        return 1;
    }
    (*mtctx).allJobsCompleted = (*mtctx).frameEnded;
    if end as u32 == ZSTD_e_end as i32 as u32 {
        return ((*mtctx).frameEnded == 0) as i32 as usize;
    }
    return 0;
}
unsafe extern "C" fn ZSTDMT_getInputDataInUse(mut mtctx: *mut ZSTDMT_CCtx) -> Range {
    let firstJobID = (*mtctx).doneJobID;
    let lastJobID = (*mtctx).nextJobID;
    let mut jobID: u32 = 0;
    let mut roundBuffCapacity = (*mtctx).roundBuff.capacity;
    let mut nbJobs1stRoundMin = roundBuffCapacity / (*mtctx).targetSectionSize;
    if (lastJobID as usize) < nbJobs1stRoundMin {
        return kNullRange;
    }
    jobID = firstJobID;
    while jobID < lastJobID {
        let wJobID = jobID & (*mtctx).jobIDMask;
        let mut consumed: usize = 0;
        ZSTD_PTHREAD_MUTEX_LOCK!(
            & mtctx -> jobs[wJobID].job_mutex
        )(ZSTD_PTHREAD_MUTEX_LOCK!(& mtctx -> jobs[wJobID].job_mutex));
        consumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
        ZSTD_pthread_mutex_unlock!(
            & mtctx -> jobs[wJobID].job_mutex
        )(ZSTD_pthread_mutex_unlock!(& mtctx -> jobs[wJobID].job_mutex));
        if consumed < (*((*mtctx).jobs).offset(wJobID as isize)).src.size {
            let mut range = (*((*mtctx).jobs).offset(wJobID as isize)).prefix;
            if range.size == 0 {
                range = (*((*mtctx).jobs).offset(wJobID as isize)).src;
            }
            return range;
        }
        jobID = jobID.wrapping_add(1);
        jobID;
    }
    return kNullRange;
}
unsafe extern "C" fn ZSTDMT_isOverlapped(
    mut buffer: Buffer,
    mut range: Range,
) -> i32 {
    let bufferStart = buffer.start as *const u8;
    let rangeStart = range.start as *const u8;
    if rangeStart.is_null() || bufferStart.is_null() {
        return 0;
    }
    let bufferEnd = bufferStart.offset(buffer.capacity as isize);
    let rangeEnd = rangeStart.offset(range.size as isize);
    if bufferStart == bufferEnd || rangeStart == rangeEnd {
        return 0;
    }
    return (bufferStart < rangeEnd && rangeStart < bufferEnd) as i32;
}
unsafe extern "C" fn ZSTDMT_doesOverlapWindow(
    mut buffer: Buffer,
    mut window: ZSTD_window_t,
) -> i32 {
    let mut extDict = Range {
        start: std::ptr::null(),
        size: 0,
    };
    let mut prefix = Range {
        start: std::ptr::null(),
        size: 0,
    };
    extDict
        .start = (window.dictBase).offset(window.lowLimit as isize)
        as *const c_void;
    extDict.size = (window.dictLimit).wrapping_sub(window.lowLimit) as usize;
    prefix
        .start = (window.base).offset(window.dictLimit as isize)
        as *const c_void;
    prefix
        .size = (window.nextSrc)
        .offset_from((window.base).offset(window.dictLimit as isize)) as std::ffi::c_long
        as usize;
    return (ZSTDMT_isOverlapped(buffer, extDict) != 0
        || ZSTDMT_isOverlapped(buffer, prefix) != 0) as i32;
}
unsafe extern "C" fn ZSTDMT_waitForLdmComplete(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut buffer: Buffer,
) {
    if (*mtctx).params.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        let mut mutex: *mut pthread_mutex_t = &mut (*mtctx).serial.ldmWindowMutex;
        ZSTD_PTHREAD_MUTEX_LOCK!(mutex)(ZSTD_PTHREAD_MUTEX_LOCK!(mutex));
        while ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow) != 0 {
            ZSTD_pthread_cond_wait!(
                & mtctx -> serial.ldmWindowCond, mutex
            )(
                ZSTD_pthread_cond_wait!(& mtctx -> serial.ldmWindowCond, mutex),
                ZSTD_pthread_cond_wait!(& mtctx -> serial.ldmWindowCond, mutex),
            );
        }
        ZSTD_pthread_mutex_unlock!(mutex)(ZSTD_pthread_mutex_unlock!(mutex));
    }
}
unsafe extern "C" fn ZSTDMT_tryGetInputRange(
    mut mtctx: *mut ZSTDMT_CCtx,
) -> i32 {
    let inUse = ZSTDMT_getInputDataInUse(mtctx);
    let spaceLeft = ((*mtctx).roundBuff.capacity).wrapping_sub((*mtctx).roundBuff.pos);
    let spaceNeeded = (*mtctx).targetSectionSize;
    let mut buffer = buffer_s {
        start: std::ptr::null_mut(),
        capacity: 0,
    };
    if spaceLeft < spaceNeeded {
        let start = (*mtctx).roundBuff.buffer;
        let prefixSize = (*mtctx).inBuff.prefix.size;
        buffer.start = start as *mut c_void;
        buffer.capacity = prefixSize;
        if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
            return 0;
        }
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        libc::memmove(start, (*mtctx).inBuff.prefix.start, (prefixSize) as usize);
        (*mtctx).inBuff.prefix.start = start as *const c_void;
        (*mtctx).roundBuff.pos = prefixSize;
    }
    buffer
        .start = ((*mtctx).roundBuff.buffer).offset((*mtctx).roundBuff.pos as isize)
        as *mut c_void;
    buffer.capacity = spaceNeeded;
    if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
        return 0;
    }
    ZSTDMT_waitForLdmComplete(mtctx, buffer);
    (*mtctx).inBuff.buffer = buffer;
    (*mtctx).inBuff.filled = 0;
    return 1;
}
unsafe extern "C" fn findSynchronizationPoint(
    mut mtctx: *const ZSTDMT_CCtx,
    input: ZSTD_inBuffer,
) -> SyncPoint {
    let istart = (input.src as *const u8).offset(input.pos as isize);
    let primePower = (*mtctx).rsync.primePower;
    let hitMask = (*mtctx).rsync.hitMask;
    let mut syncPoint = SyncPoint { toLoad: 0, flush: 0 };
    let mut hash: u64 = 0;
    let mut prev = std::ptr::null();
    let mut pos: usize = 0;
    syncPoint
        .toLoad = std::cmp::min(
        input.size - input.pos, (*mtctx).targetSectionSize - (*mtctx).inBuff.filled
    );
    syncPoint.flush = 0;
    if (*mtctx).params.rsyncable == 0 {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(input.size).wrapping_sub(input.pos)
        < RSYNC_MIN_BLOCK_SIZE as usize
    {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad) < RSYNC_LENGTH as usize {
        return syncPoint;
    }
    if (*mtctx).inBuff.filled < RSYNC_MIN_BLOCK_SIZE as usize {
        pos = (RSYNC_MIN_BLOCK_SIZE as usize).wrapping_sub((*mtctx).inBuff.filled);
        if pos >= RSYNC_LENGTH as usize {
            prev = istart.offset(pos as isize).offset(-(RSYNC_LENGTH as isize));
            hash = ZSTD_rollingHash_compute(
                prev as *const c_void,
                RSYNC_LENGTH as usize,
            );
        } else {
            prev = ((*mtctx).inBuff.buffer.start as *const u8)
                .offset((*mtctx).inBuff.filled as isize)
                .offset(-(RSYNC_LENGTH as isize));
            hash = ZSTD_rollingHash_compute(
                prev.offset(pos as isize) as *const c_void,
                (RSYNC_LENGTH as usize).wrapping_sub(pos),
            );
            hash = ZSTD_rollingHash_append(hash, istart as *const c_void, pos);
        }
    } else {
        pos = 0;
        prev = ((*mtctx).inBuff.buffer.start as *const u8)
            .offset((*mtctx).inBuff.filled as isize)
            .offset(-(RSYNC_LENGTH as isize));
        hash = ZSTD_rollingHash_compute(
            prev as *const c_void,
            RSYNC_LENGTH as usize,
        );
        if hash & hitMask == hitMask {
            syncPoint.toLoad = 0;
            syncPoint.flush = 1;
            return syncPoint;
        }
    }
    while pos < syncPoint.toLoad {
        let toRemove = (if pos < RSYNC_LENGTH as usize {
            *prev.offset(pos as isize) as i32
        } else {
            *istart.offset(pos.wrapping_sub(RSYNC_LENGTH as usize) as isize)
                as i32
        }) as u8;
        hash = ZSTD_rollingHash_rotate(
            hash,
            toRemove,
            *istart.offset(pos as isize),
            primePower,
        );
        if hash & hitMask == hitMask {
            syncPoint.toLoad = pos.wrapping_add(1);
            syncPoint.flush = 1;
            pos = pos.wrapping_add(1);
            pos;
            break;
        } else {
            pos = pos.wrapping_add(1);
            pos;
        }
    }
    return syncPoint;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_nextInputSizeHint(
    mut mtctx: *const ZSTDMT_CCtx,
) -> usize {
    let mut hintInSize = ((*mtctx).targetSectionSize)
        .wrapping_sub((*mtctx).inBuff.filled);
    if hintInSize == 0 {
        hintInSize = (*mtctx).targetSectionSize;
    }
    return hintInSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compressStream_generic(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> usize {
    let mut forwardInputProgress: u32 = 0;
    RETURN_ERROR_IF!((*mtctx).frameEnded != 0
        && endOp as u32
            == ZSTD_e_continue as i32 as u32, ZSTD_error_stage_wrong);
    if (*mtctx).jobReady == 0 && (*input).size > (*input).pos {
        if ((*mtctx).inBuff.buffer.start).is_null() {
            ZSTDMT_tryGetInputRange(mtctx) == 0;
        }
        if !((*mtctx).inBuff.buffer.start).is_null() {
            let syncPoint = findSynchronizationPoint(mtctx, *input);
            if syncPoint.flush != 0
                && endOp as u32
                    == ZSTD_e_continue as i32 as u32
            {
                endOp = ZSTD_e_flush;
            }
            libc::memcpy((*mtctx).inBuff.buffer.start.byte_add((*mtctx).inBuff.filled), (*input).src.byte_add((*input).pos), (syncPoint.toLoad) as usize);
            (*input).pos = ((*input).pos).wrapping_add(syncPoint.toLoad);
            (*mtctx)
                .inBuff
                .filled = ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad);
            forwardInputProgress = (syncPoint.toLoad > 0)
                as i32 as u32;
        }
    }
    if (*input).pos < (*input).size
        && endOp as u32 == ZSTD_e_end as i32 as u32
    {
        endOp = ZSTD_e_flush;
    }
    if (*mtctx).jobReady != 0 || (*mtctx).inBuff.filled >= (*mtctx).targetSectionSize
        || endOp as u32
            != ZSTD_e_continue as i32 as u32
            && (*mtctx).inBuff.filled > 0
        || endOp as u32 == ZSTD_e_end as i32 as u32
            && (*mtctx).frameEnded == 0
    {
        let jobSize = (*mtctx).inBuff.filled;
        FORWARD_IF_ERROR!(
            ZSTDMT_createCompressionJob(mtctx, jobSize, endOp), ""
        );
    }
    let remainingToFlush = ZSTDMT_flushProduced(
        mtctx,
        output,
        (forwardInputProgress == 0) as i32 as u32,
        endOp,
    );
    if (*input).pos < (*input).size {
        return std::cmp::max(remainingToFlush, 1);
    }
    return remainingToFlush;
}
