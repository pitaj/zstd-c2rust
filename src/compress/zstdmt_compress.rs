use ::libc;
use ::c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ZSTD_compressBound(srcSize: libc::size_t) -> libc::size_t;
    fn ZSTD_CCtxParams_setParameter(
        params: *mut ZSTD_CCtx_params,
        param: ZSTD_cParameter,
        value: libc::c_int,
    ) -> libc::size_t;
    fn ZSTD_checkCParams(params: ZSTD_compressionParameters) -> libc::size_t;
    fn ZSTD_createCDict_advanced(
        dict: *const libc::c_void,
        dictSize: libc::size_t,
        dictLoadMethod: ZSTD_dictLoadMethod_e,
        dictContentType: ZSTD_dictContentType_e,
        cParams: ZSTD_compressionParameters,
        customMem: ZSTD_customMem,
    ) -> *mut ZSTD_CDict;
    fn ZSTD_createCCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_CCtx;
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> libc::size_t;
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> libc::size_t;
    fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> libc::size_t;
    fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> libc::size_t;
    fn POOL_create_advanced(
        numThreads: libc::size_t,
        queueSize: libc::size_t,
        customMem: ZSTD_customMem,
    ) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_resize(ctx: *mut POOL_ctx, numThreads: libc::size_t) -> libc::c_int;
    fn POOL_sizeof(ctx: *const POOL_ctx) -> libc::size_t;
    fn POOL_tryAdd(
        ctx: *mut POOL_ctx,
        function: POOL_function,
        opaque: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn ZSTD_pthread_cond_destroy(cond: *mut *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn ZSTD_pthread_cond_init(
        cond: *mut *mut pthread_cond_t,
        attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn ZSTD_pthread_mutex_destroy(mutex: *mut *mut pthread_mutex_t) -> libc::c_int;
    fn ZSTD_pthread_mutex_init(
        mutex: *mut *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx);
    fn ZSTD_cycleLog(hashLog: u32, strat: ZSTD_strategy) -> u32;
    fn ZSTD_getCParamsFromCCtxParams(
        CCtxParams: *const ZSTD_CCtx_params,
        srcSizeHint: u64,
        dictSize: libc::size_t,
        mode: ZSTD_cParamMode_e,
    ) -> ZSTD_compressionParameters;
    fn ZSTD_writeLastEmptyBlock(dst: *mut libc::c_void, dstCapacity: libc::size_t) -> libc::size_t;
    fn ZSTD_CCtx_trace(cctx: *mut ZSTD_CCtx, extraCSize: libc::size_t);
    fn ZSTD_compressContinue_public(
        cctx: *mut ZSTD_CCtx,
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_compressEnd_public(
        cctx: *mut ZSTD_CCtx,
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_referenceExternalSequences(
        cctx: *mut ZSTD_CCtx,
        seq: *mut rawSeq,
        nbSeq: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_compressBegin_advanced_internal(
        cctx: *mut ZSTD_CCtx,
        dict: *const libc::c_void,
        dictSize: libc::size_t,
        dictContentType: ZSTD_dictContentType_e,
        dtlm: ZSTD_dictTableLoadMethod_e,
        cdict: *const ZSTD_CDict,
        params: *const ZSTD_CCtx_params,
        pledgedSrcSize: libc::c_ulonglong,
    ) -> libc::size_t;
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    fn ZSTD_XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const libc::c_void,
        length: libc::size_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_reset(
        statePtr: *mut XXH64_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
    fn ZSTD_ldm_fillHashTable(
        state: *mut ldmState_t,
        ip: *const u8,
        iend: *const u8,
        params: *const ldmParams_t,
    );
    fn ZSTD_ldm_generateSequences(
        ldms: *mut ldmState_t,
        sequences: *mut rawSeqStore_t,
        params: *const ldmParams_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
    fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: libc::size_t) -> libc::size_t;
    fn ZSTD_ldm_adjustParameters(
        params: *mut ldmParams_t,
        cParams: *const ZSTD_compressionParameters,
    );
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
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
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type unalign32 = u32;
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
    pub mFinder: Option::<ZSTD_sequenceProducer_F>,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ZSTDMT_CCtx_s {
    pub factory: *mut POOL_ctx,
    pub jobs: *mut ZSTDMT_jobDescription,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub params: ZSTD_CCtx_params,
    pub targetSectionSize: libc::size_t,
    pub targetPrefixSize: libc::size_t,
    pub jobReady: libc::c_int,
    pub inBuff: inBuff_t,
    pub roundBuff: roundBuff_t,
    pub serial: serialState_t,
    pub rsync: rsyncState_t,
    pub jobIDMask: libc::c_uint,
    pub doneJobID: libc::c_uint,
    pub nextJobID: libc::c_uint,
    pub frameEnded: libc::c_uint,
    pub allJobsCompleted: libc::c_uint,
    pub frameContentSize: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
    pub cMem: ZSTD_customMem,
    pub cdictLocal: *mut ZSTD_CDict,
    pub cdict: *const ZSTD_CDict,
    #[bitfield(name = "providedFactory", ty = "libc::c_uint", bits = "0..=0")]
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
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::size_t) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsyncState_t {
    pub hash: u64,
    pub hitMask: u64,
    pub primePower: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct serialState_t {
    pub mutex: *mut pthread_mutex_t,
    pub cond: *mut pthread_cond_t,
    pub params: ZSTD_CCtx_params,
    pub ldmState: ldmState_t,
    pub xxhState: XXH64_state_t,
    pub nextJobID: libc::c_uint,
    pub ldmWindowMutex: *mut pthread_mutex_t,
    pub ldmWindowCond: *mut pthread_cond_t,
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
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
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
pub type ZSTD_format_e = libc::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct roundBuff_t {
    pub buffer: *mut u8,
    pub capacity: libc::size_t,
    pub pos: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inBuff_t {
    pub prefix: range_t,
    pub buffer: buffer_t,
    pub filled: libc::size_t,
}
pub type buffer_t = buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_s {
    pub start: *mut libc::c_void,
    pub capacity: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_t {
    pub start: *const libc::c_void,
    pub size: libc::size_t,
}
pub type ZSTDMT_seqPool = ZSTDMT_bufferPool;
pub type ZSTDMT_bufferPool = ZSTDMT_bufferPool_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_bufferPool_s {
    pub poolMutex: *mut pthread_mutex_t,
    pub bufferSize: libc::size_t,
    pub totalBuffers: libc::c_uint,
    pub nbBuffers: libc::c_uint,
    pub cMem: ZSTD_customMem,
    pub bTable: [buffer_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_CCtxPool {
    pub poolMutex: *mut pthread_mutex_t,
    pub totalCCtx: libc::c_int,
    pub availCCtx: libc::c_int,
    pub cMem: ZSTD_customMem,
    pub cctx: [*mut ZSTD_CCtx; 1],
}
pub type ZSTD_CCtx = ZSTD_CCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_jobDescription {
    pub consumed: libc::size_t,
    pub cSize: libc::size_t,
    pub job_mutex: *mut pthread_mutex_t,
    pub job_cond: *mut pthread_cond_t,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub serial: *mut serialState_t,
    pub dstBuff: buffer_t,
    pub prefix: range_t,
    pub src: range_t,
    pub jobID: libc::c_uint,
    pub firstJob: libc::c_uint,
    pub lastJob: libc::c_uint,
    pub params: ZSTD_CCtx_params,
    pub cdict: *const ZSTD_CDict,
    pub fullFrameSize: libc::c_ulonglong,
    pub dstFlushed: libc::size_t,
    pub frameChecksumNeeded: libc::c_uint,
}
pub type POOL_ctx = POOL_ctx_s;
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
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
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
pub type ZSTD_compressionStage_e = libc::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type ZSTD_cParameter = libc::c_uint;
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
pub const ZSTD_c_experimentalParam6: ZSTD_cParameter = 1003;
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
    pub dst: *mut libc::c_void,
    pub size: libc::size_t,
    pub pos: libc::size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_EndDirective = libc::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameProgression {
    pub ingested: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
    pub flushed: libc::c_ulonglong,
    pub currentJobID: libc::c_uint,
    pub nbActiveWorkers: libc::c_uint,
}
pub type POOL_function = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed_0 = 120;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed_0 = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed_0 = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed_0 = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed_0 = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed_0 = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed_0 = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed_0 = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed_0 = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed_0 = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed_0 = 72;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed_0 = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed_0 = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed_0 = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed_0 = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed_0 = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed_0 = 50;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed_0 = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed_0 = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed_0 = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed_0 = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed_0 = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed_0 = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed_0 = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed_0 = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed_0 = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed_0 = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed_0 = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed_0 = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed_0 = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed_0 = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed_0 = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed_0 = 10;
pub const ZSTD_error_GENERIC: C2RustUnnamed_0 = 1;
pub const ZSTD_error_no_error: C2RustUnnamed_0 = 0;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syncPoint_t {
    pub toLoad: libc::size_t,
    pub flush: libc::c_int,
}
pub type ZSTD_cParamMode_e = libc::c_uint;
pub const ZSTD_cpm_unknown: ZSTD_cParamMode_e = 3;
pub const ZSTD_cpm_createCDict: ZSTD_cParamMode_e = 2;
pub const ZSTD_cpm_attachDict: ZSTD_cParamMode_e = 1;
pub const ZSTD_cpm_noAttachDict: ZSTD_cParamMode_e = 0;
pub const ZSTD_CONTENTSIZE_UNKNOWN: libc::c_ulonglong = (0)
    .wrapping_sub(1);
pub const ZSTD_c_forceMaxWindow: libc::c_int = ZSTD_c_experimentalParam3 as libc::c_int;
pub const ZSTD_c_deterministicRefPrefix: libc::c_int = ZSTD_c_experimentalParam15
    as libc::c_int;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
pub const NULL_0: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>()
        == 4) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
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
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut libc::c_void,
    mut customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree)
                .expect("non-null function pointer")(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    mut size: libc::size_t,
    mut customMem: ZSTD_customMem,
) -> *mut libc::c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
        libc::memset(ptr, 0 as libc::c_int, size as libc::size_t);
        return ptr;
    }
    return calloc(1 as libc::c_int as libc::c_ulong, size);
}
#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    mut size: libc::size_t,
    mut customMem: ZSTD_customMem,
) -> *mut libc::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
    }
    return malloc(size);
}
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return (31)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
static mut ZSTD_blockHeaderSize: libc::size_t = ZSTD_BLOCKHEADERSIZE as libc::size_t;
pub const ZSTD_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
pub const ZSTD_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
#[inline]
unsafe extern "C" fn ZSTD_window_update(
    mut window: *mut ZSTD_window_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut forceNonContiguous: libc::c_int,
) -> u32 {
    let ip = src as *const u8;
    let mut contiguous = 1 as libc::c_int as u32;
    if srcSize == 0 {
        return contiguous;
    }
    debug_assert!(!((*window).base).is_null());
    debug_assert!(!((*window).dictBase).is_null());
    if src != (*window).nextSrc as *const libc::c_void || forceNonContiguous != 0 {
        let distanceFromBase = ((*window).nextSrc).offset_from((*window).base)
            as libc::c_long as libc::size_t;
        (*window).lowLimit = (*window).dictLimit;
        debug_assert!(distanceFromBase == distanceFromBase as u32 as libc::size_t);
        (*window).dictLimit = distanceFromBase as u32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit)
            < HASH_READ_SIZE as libc::c_uint
        {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = 0 as libc::c_int as u32;
    }
    (*window).nextSrc = ip.offset(srcSize as isize);
    if (ip.offset(srcSize as isize)
        > ((*window).dictBase).offset((*window).lowLimit as isize)) as libc::c_int
        & (ip < ((*window).dictBase).offset((*window).dictLimit as isize)) as libc::c_int
        != 0
    {
        let highInputIdx = ip.offset(srcSize as isize).offset_from((*window).dictBase)
            as libc::c_long;
        let lowLimitMax = if highInputIdx > (*window).dictLimit as ptrdiff_t {
            (*window).dictLimit
        } else {
            highInputIdx as u32
        };
        (*window).lowLimit = lowLimitMax;
    }
    return contiguous;
}
pub const HASH_READ_SIZE: libc::c_int = 8 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_init(mut window: *mut ZSTD_window_t) {
    libc::memset(
        window as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_window_t>() as libc::size_t,
    );
    (*window).base = b" \0" as *const u8 as *const libc::c_char as *const u8;
    (*window).dictBase = b" \0" as *const u8 as *const libc::c_char as *const u8;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0 as libc::c_int as u32;
}
pub const ZSTD_WINDOW_START_INDEX: libc::c_int = 2 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_primePower(mut length: u32) -> u64 {
    return ZSTD_ipow(
        prime8bytes,
        length.wrapping_sub(1) as u64,
    );
}
static mut prime8bytes: u64 = 0xcf1bbcdcb7a56463 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_ipow(mut base: u64, mut exponent: u64) -> u64 {
    let mut power = 1 as libc::c_int as u64;
    while exponent != 0 {
        if exponent & 1 != 0 {
            power = (power as libc::c_ulong).wrapping_mul(base) ;
        }
        exponent >>= 1 as libc::c_int;
        base = (base as libc::c_ulong).wrapping_mul(base) ;
    }
    return power;
}
pub const ZSTDMT_JOBSIZE_MIN: libc::c_int = 512 as libc::c_int
    * ((1) << 10 as libc::c_int);
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_rotate(
    mut hash: u64,
    mut toRemove: u8,
    mut toAdd: u8,
    mut primePower: u64,
) -> u64 {
    hash = (hash as libc::c_ulong)
        .wrapping_sub(
            ((toRemove as libc::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as libc::c_ulong)
                .wrapping_mul(primePower),
        ) ;
    hash = (hash as libc::c_ulong).wrapping_mul(prime8bytes) ;
    hash = (hash as libc::c_ulong)
        .wrapping_add(
            (toAdd as libc::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as libc::c_ulong,
        ) ;
    return hash;
}
pub const ZSTD_ROLL_HASH_CHAR_OFFSET: libc::c_int = 10 as libc::c_int;
static mut kNullRawSeqStore: rawSeqStore_t = {
    let mut init = rawSeqStore_t {
        seq: NULL_0 as *mut rawSeq,
        pos: 0 as libc::c_int as libc::size_t,
        posInSequence: 0 as libc::c_int as libc::size_t,
        size: 0 as libc::c_int as libc::size_t,
        capacity: 0 as libc::c_int as libc::size_t,
    };
    init
};
unsafe extern "C" fn ZSTD_rollingHash_append(
    mut hash: u64,
    mut buf: *const libc::c_void,
    mut size: libc::size_t,
) -> u64 {
    let mut istart = buf as *const u8;
    let mut pos: libc::size_t = 0;
    pos = 0 as libc::c_int as libc::size_t;
    while pos < size {
        hash = (hash as libc::c_ulong).wrapping_mul(prime8bytes) ;
        hash = (hash as libc::c_ulong)
            .wrapping_add(
                (*istart.offset(pos as isize) as libc::c_int
                    + ZSTD_ROLL_HASH_CHAR_OFFSET) as libc::c_ulong,
            ) ;
        pos = pos.wrapping_add(1);
    }
    return hash;
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_compute(
    mut buf: *const libc::c_void,
    mut size: libc::size_t,
) -> u64 {
    return ZSTD_rollingHash_append(0 as libc::c_int as u64, buf, size);
}
#[inline]
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base) as libc::c_long as libc::size_t;
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
#[inline]
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> u32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as u32;
}
static mut g_nullBuffer: buffer_t = {
    let mut init = buffer_s {
        start: NULL_0 as *mut libc::c_void,
        capacity: 0 as libc::c_int as libc::size_t,
    };
    init
};
unsafe extern "C" fn ZSTDMT_createBufferPool(
    mut maxNbBuffers: libc::c_uint,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_bufferPool {
    let bufPool = ZSTD_customCalloc(
        (::core::mem::size_of::<ZSTDMT_bufferPool>())
            .wrapping_add(
                (maxNbBuffers.wrapping_sub(1)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<buffer_t>()),
            ),
        cMem,
    ) as *mut ZSTDMT_bufferPool;
    if bufPool.is_null() {
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    if ZSTD_pthread_mutex_init(
        &mut (*bufPool).poolMutex,
        NULL_0 as *const pthread_mutexattr_t,
    ) != 0
    {
        ZSTD_customFree(bufPool as *mut libc::c_void, cMem);
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    (*bufPool)
        .bufferSize = (64 as libc::c_int * ((1) << 10 as libc::c_int))
        as libc::size_t;
    (*bufPool).totalBuffers = maxNbBuffers;
    (*bufPool).nbBuffers = 0 as libc::c_int as libc::c_uint;
    (*bufPool).cMem = cMem;
    return bufPool;
}
unsafe extern "C" fn ZSTDMT_freeBufferPool(mut bufPool: *mut ZSTDMT_bufferPool) {
    let mut u: libc::c_uint = 0;
    if bufPool.is_null() {
        return;
    }
    u = 0 as libc::c_int as libc::c_uint;
    while u < (*bufPool).totalBuffers {
        ZSTD_customFree(
            (*((*bufPool).bTable).as_mut_ptr().offset(u as isize)).start,
            (*bufPool).cMem,
        );
        u = u.wrapping_add(1);
    }
    ZSTD_pthread_mutex_destroy(&mut (*bufPool).poolMutex);
    ZSTD_customFree(bufPool as *mut libc::c_void, (*bufPool).cMem);
}
unsafe extern "C" fn ZSTDMT_sizeof_bufferPool(
    mut bufPool: *mut ZSTDMT_bufferPool,
) -> libc::size_t {
    let poolSize = (::core::mem::size_of::<ZSTDMT_bufferPool>())
        .wrapping_add(
            (((*bufPool).totalBuffers).wrapping_sub(1)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<buffer_t>()),
        );
    let mut u: libc::c_uint = 0;
    let mut totalBufferSize = 0 as libc::c_int as libc::size_t;
    pthread_mutex_lock((*bufPool).poolMutex);
    u = 0 as libc::c_int as libc::c_uint;
    while u < (*bufPool).totalBuffers {
        totalBufferSize = (totalBufferSize as libc::c_ulong)
            .wrapping_add(
                (*((*bufPool).bTable).as_mut_ptr().offset(u as isize)).capacity,
            ) ;
        u = u.wrapping_add(1);
    }
    pthread_mutex_unlock((*bufPool).poolMutex);
    return poolSize.wrapping_add(totalBufferSize);
}
unsafe extern "C" fn ZSTDMT_setBufferSize(
    bufPool: *mut ZSTDMT_bufferPool,
    bSize: libc::size_t,
) {
    pthread_mutex_lock((*bufPool).poolMutex);
    (*bufPool).bufferSize = bSize;
    pthread_mutex_unlock((*bufPool).poolMutex);
}
unsafe extern "C" fn ZSTDMT_expandBufferPool(
    mut srcBufPool: *mut ZSTDMT_bufferPool,
    mut maxNbBuffers: libc::c_uint,
) -> *mut ZSTDMT_bufferPool {
    if srcBufPool.is_null() {
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    if (*srcBufPool).totalBuffers >= maxNbBuffers {
        return srcBufPool;
    }
    let cMem = (*srcBufPool).cMem;
    let bSize = (*srcBufPool).bufferSize;
    let mut newBufPool = 0 as *mut ZSTDMT_bufferPool;
    ZSTDMT_freeBufferPool(srcBufPool);
    newBufPool = ZSTDMT_createBufferPool(maxNbBuffers, cMem);
    if newBufPool.is_null() {
        return newBufPool;
    }
    ZSTDMT_setBufferSize(newBufPool, bSize);
    return newBufPool;
}
unsafe extern "C" fn ZSTDMT_getBuffer(mut bufPool: *mut ZSTDMT_bufferPool) -> buffer_t {
    let bSize = (*bufPool).bufferSize;
    pthread_mutex_lock((*bufPool).poolMutex);
    if (*bufPool).nbBuffers != 0 {
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_sub(1);
        let buf = *((*bufPool).bTable)
            .as_mut_ptr()
            .offset((*bufPool).nbBuffers as isize);
        let availBufferSize = buf.capacity;
        *((*bufPool).bTable)
            .as_mut_ptr()
            .offset((*bufPool).nbBuffers as isize) = g_nullBuffer;
        if (availBufferSize >= bSize) as libc::c_int
            & (availBufferSize >> 3 as libc::c_int <= bSize) as libc::c_int != 0
        {
            pthread_mutex_unlock((*bufPool).poolMutex);
            return buf;
        }
        ZSTD_customFree(buf.start, (*bufPool).cMem);
    }
    pthread_mutex_unlock((*bufPool).poolMutex);
    let mut buffer = buffer_t {
        start: 0 as *mut libc::c_void,
        capacity: 0,
    };
    let start = ZSTD_customMalloc(bSize, (*bufPool).cMem);
    buffer.start = start;
    buffer
        .capacity = if start.is_null() {
        0 as libc::c_int as libc::c_ulong
    } else {
        bSize
    };
    start.is_null();
    return buffer;
}
unsafe extern "C" fn ZSTDMT_releaseBuffer(
    mut bufPool: *mut ZSTDMT_bufferPool,
    mut buf: buffer_t,
) {
    if (buf.start).is_null() {
        return;
    }
    pthread_mutex_lock((*bufPool).poolMutex);
    if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
        let fresh0 = (*bufPool).nbBuffers;
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_add(1);
        *((*bufPool).bTable).as_mut_ptr().offset(fresh0 as isize) = buf;
        pthread_mutex_unlock((*bufPool).poolMutex);
        return;
    }
    pthread_mutex_unlock((*bufPool).poolMutex);
    ZSTD_customFree(buf.start, (*bufPool).cMem);
}
unsafe extern "C" fn ZSTDMT_sizeof_seqPool(mut seqPool: *mut ZSTDMT_seqPool) -> libc::size_t {
    return ZSTDMT_sizeof_bufferPool(seqPool);
}
unsafe extern "C" fn bufferToSeq(mut buffer: buffer_t) -> rawSeqStore_t {
    let mut seq = kNullRawSeqStore;
    seq.seq = buffer.start as *mut rawSeq;
    seq
        .capacity = (buffer.capacity)
        .wrapping_div(::core::mem::size_of::<rawSeq>());
    return seq;
}
unsafe extern "C" fn seqToBuffer(mut seq: rawSeqStore_t) -> buffer_t {
    let mut buffer = buffer_t {
        start: 0 as *mut libc::c_void,
        capacity: 0,
    };
    buffer.start = seq.seq as *mut libc::c_void;
    buffer
        .capacity = (seq.capacity)
        .wrapping_mul(::core::mem::size_of::<rawSeq>());
    return buffer;
}
unsafe extern "C" fn ZSTDMT_getSeq(mut seqPool: *mut ZSTDMT_seqPool) -> rawSeqStore_t {
    if (*seqPool).bufferSize == 0 {
        return kNullRawSeqStore;
    }
    return bufferToSeq(ZSTDMT_getBuffer(seqPool));
}
unsafe extern "C" fn ZSTDMT_releaseSeq(
    mut seqPool: *mut ZSTDMT_seqPool,
    mut seq: rawSeqStore_t,
) {
    ZSTDMT_releaseBuffer(seqPool, seqToBuffer(seq));
}
unsafe extern "C" fn ZSTDMT_setNbSeq(seqPool: *mut ZSTDMT_seqPool, nbSeq: libc::size_t) {
    ZSTDMT_setBufferSize(
        seqPool,
        nbSeq.wrapping_mul(::core::mem::size_of::<rawSeq>()),
    );
}
unsafe extern "C" fn ZSTDMT_createSeqPool(
    mut nbWorkers: libc::c_uint,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_seqPool {
    let seqPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
    if seqPool.is_null() {
        return NULL_0 as *mut ZSTDMT_seqPool;
    }
    ZSTDMT_setNbSeq(seqPool, 0 as libc::c_int as libc::size_t);
    return seqPool;
}
unsafe extern "C" fn ZSTDMT_freeSeqPool(mut seqPool: *mut ZSTDMT_seqPool) {
    ZSTDMT_freeBufferPool(seqPool);
}
unsafe extern "C" fn ZSTDMT_expandSeqPool(
    mut pool: *mut ZSTDMT_seqPool,
    mut nbWorkers: u32,
) -> *mut ZSTDMT_seqPool {
    return ZSTDMT_expandBufferPool(pool, nbWorkers);
}
unsafe extern "C" fn ZSTDMT_freeCCtxPool(mut pool: *mut ZSTDMT_CCtxPool) {
    let mut cid: libc::c_int = 0;
    cid = 0 as libc::c_int;
    while cid < (*pool).totalCCtx {
        ZSTD_freeCCtx(*((*pool).cctx).as_mut_ptr().offset(cid as isize));
        cid += 1;
    }
    ZSTD_pthread_mutex_destroy(&mut (*pool).poolMutex);
    ZSTD_customFree(pool as *mut libc::c_void, (*pool).cMem);
}
unsafe extern "C" fn ZSTDMT_createCCtxPool(
    mut nbWorkers: libc::c_int,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_CCtxPool {
    let cctxPool = ZSTD_customCalloc(
        (::core::mem::size_of::<ZSTDMT_CCtxPool>())
            .wrapping_add(
                ((nbWorkers - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut ZSTD_CCtx>(),
                    ),
            ),
        cMem,
    ) as *mut ZSTDMT_CCtxPool;
    debug_assert!(nbWorkers > 0);
    if cctxPool.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    if ZSTD_pthread_mutex_init(
        &mut (*cctxPool).poolMutex,
        NULL_0 as *const pthread_mutexattr_t,
    ) != 0
    {
        ZSTD_customFree(cctxPool as *mut libc::c_void, cMem);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).cMem = cMem;
    (*cctxPool).totalCCtx = nbWorkers;
    (*cctxPool).availCCtx = 1 as libc::c_int;
    let ref mut fresh1 = *((*cctxPool).cctx)
        .as_mut_ptr()
        .offset(0);
    *fresh1 = ZSTD_createCCtx_advanced(cMem);
    if (*((*cctxPool).cctx).as_mut_ptr().offset(0)).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    return cctxPool;
}
unsafe extern "C" fn ZSTDMT_expandCCtxPool(
    mut srcPool: *mut ZSTDMT_CCtxPool,
    mut nbWorkers: libc::c_int,
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
) -> libc::size_t {
    pthread_mutex_lock((*cctxPool).poolMutex);
    let nbWorkers = (*cctxPool).totalCCtx as libc::c_uint;
    let poolSize = (::core::mem::size_of::<ZSTDMT_CCtxPool>())
        .wrapping_add(
            (nbWorkers.wrapping_sub(1) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>()),
        );
    let mut u: libc::c_uint = 0;
    let mut totalCCtxSize = 0 as libc::c_int as libc::size_t;
    u = 0 as libc::c_int as libc::c_uint;
    while u < nbWorkers {
        totalCCtxSize = (totalCCtxSize as libc::c_ulong)
            .wrapping_add(
                ZSTD_sizeof_CCtx(*((*cctxPool).cctx).as_mut_ptr().offset(u as isize)),
            ) ;
        u = u.wrapping_add(1);
    }
    pthread_mutex_unlock((*cctxPool).poolMutex);
    debug_assert!(nbWorkers > 0);
    return poolSize.wrapping_add(totalCCtxSize);
}
unsafe extern "C" fn ZSTDMT_getCCtx(
    mut cctxPool: *mut ZSTDMT_CCtxPool,
) -> *mut ZSTD_CCtx {
    pthread_mutex_lock((*cctxPool).poolMutex);
    if (*cctxPool).availCCtx != 0 {
        (*cctxPool).availCCtx -= 1;
        let cctx = *((*cctxPool).cctx)
            .as_mut_ptr()
            .offset((*cctxPool).availCCtx as isize);
        pthread_mutex_unlock((*cctxPool).poolMutex);
        return cctx;
    }
    pthread_mutex_unlock((*cctxPool).poolMutex);
    return ZSTD_createCCtx_advanced((*cctxPool).cMem);
}
unsafe extern "C" fn ZSTDMT_releaseCCtx(
    mut pool: *mut ZSTDMT_CCtxPool,
    mut cctx: *mut ZSTD_CCtx,
) {
    if cctx.is_null() {
        return;
    }
    pthread_mutex_lock((*pool).poolMutex);
    if (*pool).availCCtx < (*pool).totalCCtx {
        let fresh2 = (*pool).availCCtx;
        (*pool).availCCtx = (*pool).availCCtx + 1;
        let ref mut fresh3 = *((*pool).cctx).as_mut_ptr().offset(fresh2 as isize);
        *fresh3 = cctx;
    } else {
        ZSTD_freeCCtx(cctx);
    }
    pthread_mutex_unlock((*pool).poolMutex);
}
unsafe extern "C" fn ZSTDMT_serialState_reset(
    mut serialState: *mut serialState_t,
    mut seqPool: *mut ZSTDMT_seqPool,
    mut params: ZSTD_CCtx_params,
    mut jobSize: libc::size_t,
    mut dict: *const libc::c_void,
    dictSize: libc::size_t,
    mut dictContentType: ZSTD_dictContentType_e,
) -> libc::c_int {
    if params.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &mut params.cParams);
        debug_assert!(params.ldmParams.hashLog >= params.ldmParams.bucketSizeLog);
        debug_assert!(params.ldmParams.hashRateLog < 32);
    } else {
        libc::memset(
            &mut params.ldmParams as *mut ldmParams_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ldmParams_t>() as libc::size_t,
        );
    }
    (*serialState).nextJobID = 0 as libc::c_int as libc::c_uint;
    if params.fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut (*serialState).xxhState, 0 as libc::c_int as XXH64_hash_t);
    }
    if params.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        let mut cMem = params.customMem;
        let hashLog = params.ldmParams.hashLog;
        let hashSize = ((1) << hashLog)
            .wrapping_mul(::core::mem::size_of::<ldmEntry_t>());
        let bucketLog = (params.ldmParams.hashLog)
            .wrapping_sub(params.ldmParams.bucketSizeLog);
        let prevBucketLog = ((*serialState).params.ldmParams.hashLog)
            .wrapping_sub((*serialState).params.ldmParams.bucketSizeLog);
        let numBuckets = (1) << bucketLog;
        ZSTDMT_setNbSeq(seqPool, ZSTD_ldm_getMaxNbSeq(params.ldmParams, jobSize));
        ZSTD_window_init(&mut (*serialState).ldmState.window);
        if ((*serialState).ldmState.hashTable).is_null()
            || (*serialState).params.ldmParams.hashLog < hashLog
        {
            ZSTD_customFree(
                (*serialState).ldmState.hashTable as *mut libc::c_void,
                cMem,
            );
            (*serialState)
                .ldmState
                .hashTable = ZSTD_customMalloc(hashSize, cMem) as *mut ldmEntry_t;
        }
        if ((*serialState).ldmState.bucketOffsets).is_null() || prevBucketLog < bucketLog
        {
            ZSTD_customFree(
                (*serialState).ldmState.bucketOffsets as *mut libc::c_void,
                cMem,
            );
            (*serialState)
                .ldmState
                .bucketOffsets = ZSTD_customMalloc(numBuckets, cMem) as *mut u8;
        }
        if ((*serialState).ldmState.hashTable).is_null()
            || ((*serialState).ldmState.bucketOffsets).is_null()
        {
            return 1 as libc::c_int;
        }
        libc::memset(
            (*serialState).ldmState.hashTable as *mut libc::c_void,
            0 as libc::c_int,
            hashSize as libc::size_t,
        );
        libc::memset(
            (*serialState).ldmState.bucketOffsets as *mut libc::c_void,
            0 as libc::c_int,
            numBuckets as libc::size_t,
        );
        (*serialState).ldmState.loadedDictEnd = 0 as libc::c_int as u32;
        if dictSize > 0 {
            if dictContentType as libc::c_uint
                == ZSTD_dct_rawContent as libc::c_int as libc::c_uint
            {
                let dictEnd = (dict as *const u8).offset(dictSize as isize);
                ZSTD_window_update(
                    &mut (*serialState).ldmState.window,
                    dict,
                    dictSize,
                    0 as libc::c_int,
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
                    0 as libc::c_int as libc::c_uint
                } else {
                    dictEnd.offset_from((*serialState).ldmState.window.base)
                        as libc::c_long as u32
                };
            }
        }
        (*serialState).ldmWindow = (*serialState).ldmState.window;
    }
    (*serialState).params = params;
    (*serialState).params.jobSize = jobSize as u32 as libc::size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ZSTDMT_serialState_init(
    mut serialState: *mut serialState_t,
) -> libc::c_int {
    let mut initError = 0 as libc::c_int;
    libc::memset(
        serialState as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<serialState_t>() as libc::size_t,
    );
    initError
        |= ZSTD_pthread_mutex_init(
            &mut (*serialState).mutex,
            NULL_0 as *const pthread_mutexattr_t,
        );
    initError
        |= ZSTD_pthread_cond_init(
            &mut (*serialState).cond,
            NULL_0 as *const pthread_condattr_t,
        );
    initError
        |= ZSTD_pthread_mutex_init(
            &mut (*serialState).ldmWindowMutex,
            NULL_0 as *const pthread_mutexattr_t,
        );
    initError
        |= ZSTD_pthread_cond_init(
            &mut (*serialState).ldmWindowCond,
            NULL_0 as *const pthread_condattr_t,
        );
    return initError;
}
unsafe extern "C" fn ZSTDMT_serialState_free(mut serialState: *mut serialState_t) {
    let mut cMem = (*serialState).params.customMem;
    ZSTD_pthread_mutex_destroy(&mut (*serialState).mutex);
    ZSTD_pthread_cond_destroy(&mut (*serialState).cond);
    ZSTD_pthread_mutex_destroy(&mut (*serialState).ldmWindowMutex);
    ZSTD_pthread_cond_destroy(&mut (*serialState).ldmWindowCond);
    ZSTD_customFree((*serialState).ldmState.hashTable as *mut libc::c_void, cMem);
    ZSTD_customFree((*serialState).ldmState.bucketOffsets as *mut libc::c_void, cMem);
}
unsafe extern "C" fn ZSTDMT_serialState_update(
    mut serialState: *mut serialState_t,
    mut jobCCtx: *mut ZSTD_CCtx,
    mut seqStore: rawSeqStore_t,
    mut src: range_t,
    mut jobID: libc::c_uint,
) {
    pthread_mutex_lock((*serialState).mutex);
    while (*serialState).nextJobID < jobID {
        pthread_cond_wait((*serialState).cond, (*serialState).mutex);
    }
    if (*serialState).nextJobID == jobID {
        if (*serialState).params.ldmParams.enableLdm as libc::c_uint
            == ZSTD_ps_enable as libc::c_int as libc::c_uint
        {
            let mut error: libc::size_t = 0;
            debug_assert!(!(seqStore.seq).is_null()
                && seqStore.pos == 0
                && seqStore.size == 0
                && seqStore.capacity > 0);
            debug_assert!(src.size <= (*serialState).params.jobSize);
            ZSTD_window_update(
                &mut (*serialState).ldmState.window,
                src.start,
                src.size,
                0 as libc::c_int,
            );
            error = ZSTD_ldm_generateSequences(
                &mut (*serialState).ldmState,
                &mut seqStore,
                &mut (*serialState).params.ldmParams,
                src.start,
                src.size,
            );
            debug_assert!(ERR_isError(error) == 0);
            pthread_mutex_lock((*serialState).ldmWindowMutex);
            (*serialState).ldmWindow = (*serialState).ldmState.window;
            pthread_cond_signal((*serialState).ldmWindowCond);
            pthread_mutex_unlock((*serialState).ldmWindowMutex);
        }
        if (*serialState).params.fParams.checksumFlag != 0
            && src.size > 0
        {
            ZSTD_XXH64_update(&mut (*serialState).xxhState, src.start, src.size);
        }
    }
    (*serialState).nextJobID = ((*serialState).nextJobID).wrapping_add(1);
    pthread_cond_broadcast((*serialState).cond);
    pthread_mutex_unlock((*serialState).mutex);
    if seqStore.size > 0 {
        let err = ZSTD_referenceExternalSequences(jobCCtx, seqStore.seq, seqStore.size);
        debug_assert!((*serialState).params.ldmParams.enableLdm as libc::c_uint
            == ZSTD_ps_enable as libc::c_int as libc::c_uint);
        debug_assert!(ERR_isError(err) == 0);
    }
}
unsafe extern "C" fn ZSTDMT_serialState_ensureFinished(
    mut serialState: *mut serialState_t,
    mut jobID: libc::c_uint,
    mut cSize: libc::size_t,
) {
    pthread_mutex_lock((*serialState).mutex);
    if (*serialState).nextJobID <= jobID {
        debug_assert!(ERR_isError(cSize) != 0);
        (*serialState).nextJobID = jobID.wrapping_add(1);
        pthread_cond_broadcast((*serialState).cond);
        pthread_mutex_lock((*serialState).ldmWindowMutex);
        ZSTD_window_clear(&mut (*serialState).ldmWindow);
        pthread_cond_signal((*serialState).ldmWindowCond);
        pthread_mutex_unlock((*serialState).ldmWindowMutex);
    }
    pthread_mutex_unlock((*serialState).mutex);
}
static mut kNullRange: range_t = {
    let mut init = range_t {
        start: NULL_0 as *const libc::c_void,
        size: 0 as libc::c_int as libc::size_t,
    };
    init
};
unsafe extern "C" fn ZSTDMT_compressionJob(mut jobDescription: *mut libc::c_void) {
    let mut current_block: u64;
    let job = jobDescription as *mut ZSTDMT_jobDescription;
    let mut jobParams = (*job).params;
    let cctx = ZSTDMT_getCCtx((*job).cctxPool);
    let mut rawSeqStore = ZSTDMT_getSeq((*job).seqPool);
    let mut dstBuff = (*job).dstBuff;
    let mut lastCBlockSize = 0 as libc::c_int as libc::size_t;
    if cctx.is_null() {
        pthread_mutex_lock((*job).job_mutex);
        (*job).cSize = -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
        pthread_mutex_unlock((*job).job_mutex);
    } else {
        if (dstBuff.start).is_null() {
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if (dstBuff.start).is_null() {
                pthread_mutex_lock((*job).job_mutex);
                (*job).cSize = -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
                pthread_mutex_unlock((*job).job_mutex);
                current_block = 15223880703280672966;
            } else {
                (*job).dstBuff = dstBuff;
                current_block = 1841672684692190573;
            }
        } else {
            current_block = 1841672684692190573;
        }
        match current_block {
            15223880703280672966 => {}
            _ => {
                if jobParams.ldmParams.enableLdm as libc::c_uint
                    == ZSTD_ps_enable as libc::c_int as libc::c_uint
                    && (rawSeqStore.seq).is_null()
                {
                    pthread_mutex_lock((*job).job_mutex);
                    (*job)
                        .cSize = -(ZSTD_error_memory_allocation as libc::c_int)
                        as libc::size_t;
                    pthread_mutex_unlock((*job).job_mutex);
                } else {
                    if (*job).jobID != 0 as libc::c_int as libc::c_uint {
                        jobParams.fParams.checksumFlag = 0 as libc::c_int;
                    }
                    jobParams.ldmParams.enableLdm = ZSTD_ps_disable;
                    jobParams.nbWorkers = 0 as libc::c_int;
                    if !((*job).cdict).is_null() {
                        let initError = ZSTD_compressBegin_advanced_internal(
                            cctx,
                            NULL_0 as *const libc::c_void,
                            0 as libc::c_int as libc::size_t,
                            ZSTD_dct_auto,
                            ZSTD_dtlm_fast,
                            (*job).cdict,
                            &mut jobParams,
                            (*job).fullFrameSize,
                        );
                        debug_assert!((*job).firstJob != 0);
                        if ERR_isError(initError) != 0 {
                            pthread_mutex_lock((*job).job_mutex);
                            (*job).cSize = initError;
                            pthread_mutex_unlock((*job).job_mutex);
                            current_block = 15223880703280672966;
                        } else {
                            current_block = 18435049525520518667;
                        }
                    } else {
                        let pledgedSrcSize = (if (*job).firstJob != 0 {
                            (*job).fullFrameSize
                        } else {
                            (*job).src.size as libc::c_ulonglong
                        }) as u64;
                        let forceWindowError = ZSTD_CCtxParams_setParameter(
                            &mut jobParams,
                            ZSTD_c_forceMaxWindow as ZSTD_cParameter,
                            ((*job).firstJob == 0) as libc::c_int,
                        );
                        if ERR_isError(forceWindowError) != 0 {
                            pthread_mutex_lock((*job).job_mutex);
                            (*job).cSize = forceWindowError;
                            pthread_mutex_unlock((*job).job_mutex);
                            current_block = 15223880703280672966;
                        } else {
                            if (*job).firstJob == 0 {
                                let err = ZSTD_CCtxParams_setParameter(
                                    &mut jobParams,
                                    ZSTD_c_deterministicRefPrefix as ZSTD_cParameter,
                                    0 as libc::c_int,
                                );
                                if ERR_isError(err) != 0 {
                                    pthread_mutex_lock((*job).job_mutex);
                                    (*job).cSize = err;
                                    pthread_mutex_unlock((*job).job_mutex);
                                    current_block = 15223880703280672966;
                                } else {
                                    current_block = 10692455896603418738;
                                }
                            } else {
                                current_block = 10692455896603418738;
                            }
                            match current_block {
                                15223880703280672966 => {}
                                _ => {
                                    let initError_0 = ZSTD_compressBegin_advanced_internal(
                                        cctx,
                                        (*job).prefix.start,
                                        (*job).prefix.size,
                                        ZSTD_dct_rawContent,
                                        ZSTD_dtlm_fast,
                                        NULL_0 as *const ZSTD_CDict,
                                        &mut jobParams,
                                        pledgedSrcSize as libc::c_ulonglong,
                                    );
                                    if ERR_isError(initError_0) != 0 {
                                        pthread_mutex_lock((*job).job_mutex);
                                        (*job).cSize = initError_0;
                                        pthread_mutex_unlock((*job).job_mutex);
                                        current_block = 15223880703280672966;
                                    } else {
                                        current_block = 18435049525520518667;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        15223880703280672966 => {}
                        _ => {
                            ZSTDMT_serialState_update(
                                (*job).serial,
                                cctx,
                                rawSeqStore,
                                (*job).src,
                                (*job).jobID,
                            );
                            if (*job).firstJob == 0 {
                                let hSize = ZSTD_compressContinue_public(
                                    cctx,
                                    dstBuff.start,
                                    dstBuff.capacity,
                                    (*job).src.start,
                                    0 as libc::c_int as libc::size_t,
                                );
                                if ERR_isError(hSize) != 0 {
                                    pthread_mutex_lock((*job).job_mutex);
                                    (*job).cSize = hSize;
                                    pthread_mutex_unlock((*job).job_mutex);
                                    current_block = 15223880703280672966;
                                } else {
                                    ZSTD_invalidateRepCodes(cctx);
                                    current_block = 13678349939556791712;
                                }
                            } else {
                                current_block = 13678349939556791712;
                            }
                            match current_block {
                                15223880703280672966 => {}
                                _ => {
                                    let chunkSize = (4 as libc::c_int * ZSTD_BLOCKSIZE_MAX)
                                        as libc::size_t;
                                    let nbChunks = ((*job).src.size)
                                        .wrapping_add(
                                            chunkSize.wrapping_sub(1),
                                        )
                                        .wrapping_div(chunkSize) as libc::c_int;
                                    let mut ip = (*job).src.start as *const u8;
                                    let ostart = dstBuff.start as *mut u8;
                                    let mut op = ostart;
                                    let mut oend = op.offset(dstBuff.capacity as isize);
                                    let mut chunkNb: libc::c_int = 0;
                                    if ::core::mem::size_of::<libc::size_t>()
                                        > ::core::mem::size_of::<libc::c_int>()
                                    {
                                        debug_assert!((*job).src.size
                                            < (2147483647)
                                                .wrapping_mul(chunkSize));
                                    }
                                    debug_assert!((*job).cSize == 0);
                                    chunkNb = 1 as libc::c_int;
                                    loop {
                                        if !(chunkNb < nbChunks) {
                                            current_block = 7189308829251266000;
                                            break;
                                        }
                                        let cSize = ZSTD_compressContinue_public(
                                            cctx,
                                            op as *mut libc::c_void,
                                            oend.offset_from(op) as libc::c_long as libc::size_t,
                                            ip as *const libc::c_void,
                                            chunkSize,
                                        );
                                        if ERR_isError(cSize) != 0 {
                                            pthread_mutex_lock((*job).job_mutex);
                                            (*job).cSize = cSize;
                                            pthread_mutex_unlock((*job).job_mutex);
                                            current_block = 15223880703280672966;
                                            break;
                                        } else {
                                            ip = ip.offset(chunkSize as isize);
                                            op = op.offset(cSize as isize);
                                            debug_assert!(op < oend);
                                            pthread_mutex_lock((*job).job_mutex);
                                            (*job)
                                                .cSize = ((*job).cSize as libc::c_ulong).wrapping_add(cSize)
                                                ;
                                            (*job)
                                                .consumed = chunkSize
                                                .wrapping_mul(chunkNb as libc::c_ulong);
                                            pthread_cond_signal((*job).job_cond);
                                            pthread_mutex_unlock((*job).job_mutex);
                                            chunkNb += 1;
                                        }
                                    }
                                    match current_block {
                                        15223880703280672966 => {}
                                        _ => {
                                            debug_assert!(chunkSize > 0);
                                            debug_assert!(chunkSize
                                                & chunkSize.wrapping_sub(1)
                                                == 0);
                                            if (nbChunks > 0) as libc::c_int
                                                as libc::c_uint | (*job).lastJob != 0
                                            {
                                                let lastBlockSize1 = (*job).src.size
                                                    & chunkSize.wrapping_sub(1);
                                                let lastBlockSize = if (lastBlockSize1
                                                    == 0) as libc::c_int
                                                    & ((*job).src.size >= chunkSize) as libc::c_int != 0
                                                {
                                                    chunkSize
                                                } else {
                                                    lastBlockSize1
                                                };
                                                let cSize_0 = if (*job).lastJob != 0 {
                                                    ZSTD_compressEnd_public(
                                                        cctx,
                                                        op as *mut libc::c_void,
                                                        oend.offset_from(op) as libc::c_long as libc::size_t,
                                                        ip as *const libc::c_void,
                                                        lastBlockSize,
                                                    )
                                                } else {
                                                    ZSTD_compressContinue_public(
                                                        cctx,
                                                        op as *mut libc::c_void,
                                                        oend.offset_from(op) as libc::c_long as libc::size_t,
                                                        ip as *const libc::c_void,
                                                        lastBlockSize,
                                                    )
                                                };
                                                if ERR_isError(cSize_0) != 0 {
                                                    pthread_mutex_lock((*job).job_mutex);
                                                    (*job).cSize = cSize_0;
                                                    pthread_mutex_unlock((*job).job_mutex);
                                                    current_block = 15223880703280672966;
                                                } else {
                                                    lastCBlockSize = cSize_0;
                                                    current_block = 15514718523126015390;
                                                }
                                            } else {
                                                current_block = 15514718523126015390;
                                            }
                                            match current_block {
                                                15223880703280672966 => {}
                                                _ => {
                                                    if (*job).firstJob == 0 {
                                                        debug_assert!(ZSTD_window_hasExtDict(
                                                            (*cctx).blockState.matchState.window,
                                                        ) == 0);
                                                    }
                                                    ZSTD_CCtx_trace(cctx, 0 as libc::c_int as libc::size_t);
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
    pthread_mutex_lock((*job).job_mutex);
    if ERR_isError((*job).cSize) != 0 {
        debug_assert!(lastCBlockSize == 0);
    }
    (*job)
        .cSize = ((*job).cSize as libc::c_ulong).wrapping_add(lastCBlockSize) as libc::size_t
        as libc::size_t;
    (*job).consumed = (*job).src.size;
    pthread_cond_signal((*job).job_cond);
    pthread_mutex_unlock((*job).job_mutex);
}
static mut kNullRoundBuff: roundBuff_t = {
    let mut init = roundBuff_t {
        buffer: NULL_0 as *mut u8,
        capacity: 0 as libc::c_int as libc::size_t,
        pos: 0 as libc::c_int as libc::size_t,
    };
    init
};
pub const RSYNC_LENGTH: libc::c_int = 32 as libc::c_int;
pub const RSYNC_MIN_BLOCK_LOG: libc::c_int = ZSTD_BLOCKSIZELOG_MAX;
pub const RSYNC_MIN_BLOCK_SIZE: libc::c_int = (1) << RSYNC_MIN_BLOCK_LOG;
unsafe extern "C" fn ZSTDMT_freeJobsTable(
    mut jobTable: *mut ZSTDMT_jobDescription,
    mut nbJobs: u32,
    mut cMem: ZSTD_customMem,
) {
    let mut jobNb: u32 = 0;
    if jobTable.is_null() {
        return;
    }
    jobNb = 0 as libc::c_int as u32;
    while jobNb < nbJobs {
        ZSTD_pthread_mutex_destroy(&mut (*jobTable.offset(jobNb as isize)).job_mutex);
        ZSTD_pthread_cond_destroy(&mut (*jobTable.offset(jobNb as isize)).job_cond);
        jobNb = jobNb.wrapping_add(1);
    }
    ZSTD_customFree(jobTable as *mut libc::c_void, cMem);
}
unsafe extern "C" fn ZSTDMT_createJobsTable(
    mut nbJobsPtr: *mut u32,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_jobDescription {
    let nbJobsLog2 = (ZSTD_highbit32(*nbJobsPtr))
        .wrapping_add(1);
    let nbJobs = ((1) << nbJobsLog2) as u32;
    let mut jobNb: u32 = 0;
    let jobTable = ZSTD_customCalloc(
        (nbJobs as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<ZSTDMT_jobDescription>(),
            ),
        cMem,
    ) as *mut ZSTDMT_jobDescription;
    let mut initError = 0 as libc::c_int;
    if jobTable.is_null() {
        return NULL_0 as *mut ZSTDMT_jobDescription;
    }
    *nbJobsPtr = nbJobs;
    jobNb = 0 as libc::c_int as u32;
    while jobNb < nbJobs {
        initError
            |= ZSTD_pthread_mutex_init(
                &mut (*jobTable.offset(jobNb as isize)).job_mutex,
                NULL_0 as *const pthread_mutexattr_t,
            );
        initError
            |= ZSTD_pthread_cond_init(
                &mut (*jobTable.offset(jobNb as isize)).job_cond,
                NULL_0 as *const pthread_condattr_t,
            );
        jobNb = jobNb.wrapping_add(1);
    }
    if initError != 0 as libc::c_int {
        ZSTDMT_freeJobsTable(jobTable, nbJobs, cMem);
        return NULL_0 as *mut ZSTDMT_jobDescription;
    }
    return jobTable;
}
unsafe extern "C" fn ZSTDMT_expandJobsTable(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut nbWorkers: u32,
) -> libc::size_t {
    let mut nbJobs = nbWorkers.wrapping_add(2);
    if nbJobs > ((*mtctx).jobIDMask).wrapping_add(1) {
        ZSTDMT_freeJobsTable(
            (*mtctx).jobs,
            ((*mtctx).jobIDMask).wrapping_add(1),
            (*mtctx).cMem,
        );
        (*mtctx).jobIDMask = 0 as libc::c_int as libc::c_uint;
        (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, (*mtctx).cMem);
        if ((*mtctx).jobs).is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
        }
        debug_assert!(nbJobs != 0 as libc::c_int as libc::c_uint
            && nbJobs & nbJobs.wrapping_sub(1)
                == 0);
        (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    }
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDMT_CCtxParam_setNbWorkers(
    mut params: *mut ZSTD_CCtx_params,
    mut nbWorkers: libc::c_uint,
) -> libc::size_t {
    return ZSTD_CCtxParams_setParameter(
        params,
        ZSTD_c_nbWorkers,
        nbWorkers as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn ZSTDMT_createCCtx_advanced_internal(
    mut nbWorkers: libc::c_uint,
    mut cMem: ZSTD_customMem,
    mut pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    let mut mtctx = 0 as *mut ZSTDMT_CCtx;
    let mut nbJobs = nbWorkers.wrapping_add(2);
    let mut initError: libc::c_int = 0;
    if nbWorkers < 1 {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    nbWorkers = if nbWorkers
        < (if ::core::mem::size_of::<*mut libc::c_void>()
            == 4
        {
            64 as libc::c_int
        } else {
            256 as libc::c_int
        }) as libc::c_uint
    {
        nbWorkers
    } else {
        (if ::core::mem::size_of::<*mut libc::c_void>()
            == 4
        {
            64 as libc::c_int
        } else {
            256 as libc::c_int
        }) as libc::c_uint
    };
    if (cMem.customAlloc).is_some() as libc::c_int
        ^ (cMem.customFree).is_some() as libc::c_int != 0
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
    (*mtctx).allJobsCompleted = 1 as libc::c_int as libc::c_uint;
    if !pool.is_null() {
        (*mtctx).factory = pool;
        (*mtctx).set_providedFactory(1);
    } else {
        (*mtctx)
            .factory = POOL_create_advanced(
            nbWorkers as libc::size_t,
            0 as libc::c_int as libc::size_t,
            cMem,
        );
        (*mtctx).set_providedFactory(0);
    }
    (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, cMem);
    debug_assert!(nbJobs > 0);
    debug_assert!(nbJobs & nbJobs.wrapping_sub(1)
        == 0);
    (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    (*mtctx)
        .bufPool = ZSTDMT_createBufferPool(
        (2)
            .wrapping_mul(nbWorkers)
            .wrapping_add(3),
        cMem,
    );
    (*mtctx).cctxPool = ZSTDMT_createCCtxPool(nbWorkers as libc::c_int, cMem);
    (*mtctx).seqPool = ZSTDMT_createSeqPool(nbWorkers, cMem);
    initError = ZSTDMT_serialState_init(&mut (*mtctx).serial);
    (*mtctx).roundBuff = kNullRoundBuff;
    if ((*mtctx).factory).is_null() as libc::c_int
        | ((*mtctx).jobs).is_null() as libc::c_int
        | ((*mtctx).bufPool).is_null() as libc::c_int
        | ((*mtctx).cctxPool).is_null() as libc::c_int
        | ((*mtctx).seqPool).is_null() as libc::c_int | initError != 0
    {
        ZSTDMT_freeCCtx(mtctx);
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    return mtctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_createCCtx_advanced(
    mut nbWorkers: libc::c_uint,
    mut cMem: ZSTD_customMem,
    mut pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    return ZSTDMT_createCCtx_advanced_internal(nbWorkers, cMem, pool);
}
unsafe extern "C" fn ZSTDMT_releaseAllJobResources(mut mtctx: *mut ZSTDMT_CCtx) {
    let mut jobID: libc::c_uint = 0;
    jobID = 0 as libc::c_int as libc::c_uint;
    while jobID <= (*mtctx).jobIDMask {
        let mutex = (*((*mtctx).jobs).offset(jobID as isize)).job_mutex;
        let cond = (*((*mtctx).jobs).offset(jobID as isize)).job_cond;
        ZSTDMT_releaseBuffer(
            (*mtctx).bufPool,
            (*((*mtctx).jobs).offset(jobID as isize)).dstBuff,
        );
        libc::memset(
            &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ZSTDMT_jobDescription>()
                as libc::size_t,
        );
        let ref mut fresh4 = (*((*mtctx).jobs).offset(jobID as isize)).job_mutex;
        *fresh4 = mutex;
        let ref mut fresh5 = (*((*mtctx).jobs).offset(jobID as isize)).job_cond;
        *fresh5 = cond;
        jobID = jobID.wrapping_add(1);
    }
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0 as libc::c_int as libc::size_t;
    (*mtctx).allJobsCompleted = 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ZSTDMT_waitForAllJobsCompleted(mut mtctx: *mut ZSTDMT_CCtx) {
    while (*mtctx).doneJobID < (*mtctx).nextJobID {
        let jobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
        pthread_mutex_lock((*((*mtctx).jobs).offset(jobID as isize)).job_mutex);
        while (*((*mtctx).jobs).offset(jobID as isize)).consumed
            < (*((*mtctx).jobs).offset(jobID as isize)).src.size
        {
            pthread_cond_wait(
                (*((*mtctx).jobs).offset(jobID as isize)).job_cond,
                (*((*mtctx).jobs).offset(jobID as isize)).job_mutex,
            );
        }
        pthread_mutex_unlock((*((*mtctx).jobs).offset(jobID as isize)).job_mutex);
        (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_freeCCtx(mut mtctx: *mut ZSTDMT_CCtx) -> libc::size_t {
    if mtctx.is_null() {
        return 0 as libc::c_int as libc::size_t;
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
        ZSTD_customFree((*mtctx).roundBuff.buffer as *mut libc::c_void, (*mtctx).cMem);
    }
    ZSTD_customFree(mtctx as *mut libc::c_void, (*mtctx).cMem);
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_sizeof_CCtx(mut mtctx: *mut ZSTDMT_CCtx) -> libc::size_t {
    if mtctx.is_null() {
        return 0 as libc::c_int as libc::size_t;
    }
    return (::core::mem::size_of::<ZSTDMT_CCtx>())
        .wrapping_add(POOL_sizeof((*mtctx).factory))
        .wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool))
        .wrapping_add(
            (((*mtctx).jobIDMask).wrapping_add(1)
                as libc::c_ulong)
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
    mut nbWorkers: libc::c_uint,
) -> libc::size_t {
    if POOL_resize((*mtctx).factory, nbWorkers as libc::size_t) != 0 {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    let err_code = ZSTDMT_expandJobsTable(mtctx, nbWorkers);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    (*mtctx)
        .bufPool = ZSTDMT_expandBufferPool(
        (*mtctx).bufPool,
        (2)
            .wrapping_mul(nbWorkers)
            .wrapping_add(3),
    );
    if ((*mtctx).bufPool).is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    (*mtctx)
        .cctxPool = ZSTDMT_expandCCtxPool((*mtctx).cctxPool, nbWorkers as libc::c_int);
    if ((*mtctx).cctxPool).is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    (*mtctx).seqPool = ZSTDMT_expandSeqPool((*mtctx).seqPool, nbWorkers);
    if ((*mtctx).seqPool).is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    return 0 as libc::c_int as libc::size_t;
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
        0 as libc::c_int as libc::size_t,
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
        .wrapping_add((*mtctx).inBuff.filled as libc::c_ulonglong);
    fps.consumed = (*mtctx).consumed;
    fps.flushed = (*mtctx).produced;
    fps.produced = fps.flushed;
    fps.currentJobID = (*mtctx).nextJobID;
    fps.nbActiveWorkers = 0 as libc::c_int as libc::c_uint;
    let mut jobNb: libc::c_uint = 0;
    let mut lastJobNb = ((*mtctx).nextJobID)
        .wrapping_add((*mtctx).jobReady as libc::c_uint);
    debug_assert!((*mtctx).jobReady <= 1);
    jobNb = (*mtctx).doneJobID;
    while jobNb < lastJobNb {
        let wJobID = jobNb & (*mtctx).jobIDMask;
        let mut jobPtr: *mut ZSTDMT_jobDescription = &mut *((*mtctx).jobs)
            .offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
        pthread_mutex_lock((*jobPtr).job_mutex);
        let cResult = (*jobPtr).cSize;
        let produced = if ERR_isError(cResult) != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            cResult
        };
        let flushed = if ERR_isError(cResult) != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (*jobPtr).dstFlushed
        };
        debug_assert!(flushed <= produced);
        fps
            .ingested = (fps.ingested)
            .wrapping_add((*jobPtr).src.size as libc::c_ulonglong);
        fps
            .consumed = (fps.consumed)
            .wrapping_add((*jobPtr).consumed as libc::c_ulonglong);
        fps.produced = (fps.produced).wrapping_add(produced as libc::c_ulonglong);
        fps.flushed = (fps.flushed).wrapping_add(flushed as libc::c_ulonglong);
        fps
            .nbActiveWorkers = (fps.nbActiveWorkers)
            .wrapping_add(
                ((*jobPtr).consumed < (*jobPtr).src.size) as libc::c_int as libc::c_uint,
            );
        pthread_mutex_unlock((*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
        jobNb = jobNb.wrapping_add(1);
    }
    return fps;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_toFlushNow(mut mtctx: *mut ZSTDMT_CCtx) -> libc::size_t {
    let mut toFlush: libc::size_t = 0;
    let jobID = (*mtctx).doneJobID;
    debug_assert!(jobID <= (*mtctx).nextJobID);
    if jobID == (*mtctx).nextJobID {
        return 0 as libc::c_int as libc::size_t;
    }
    let wJobID = jobID & (*mtctx).jobIDMask;
    let jobPtr: *mut ZSTDMT_jobDescription = &mut *((*mtctx).jobs)
        .offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
    pthread_mutex_lock((*jobPtr).job_mutex);
    let cResult = (*jobPtr).cSize;
    let produced = if ERR_isError(cResult) != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        cResult
    };
    let flushed = if ERR_isError(cResult) != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        (*jobPtr).dstFlushed
    };
    debug_assert!(flushed <= produced);
    debug_assert!((*jobPtr).consumed <= (*jobPtr).src.size);
    toFlush = produced.wrapping_sub(flushed);
    if toFlush == 0 {
        debug_assert!((*jobPtr).consumed < (*jobPtr).src.size);
    }
    pthread_mutex_unlock((*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
    return toFlush;
}
unsafe extern "C" fn ZSTDMT_computeTargetJobLog(
    mut params: *const ZSTD_CCtx_params,
) -> libc::c_uint {
    let mut jobLog: libc::c_uint = 0;
    if (*params).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        jobLog = if 21 as libc::c_int as libc::c_uint
            > (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy))
                .wrapping_add(3)
        {
            21 as libc::c_int as libc::c_uint
        } else {
            (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy))
                .wrapping_add(3)
        };
    } else {
        jobLog = if 20 as libc::c_int as libc::c_uint
            > ((*params).cParams.windowLog)
                .wrapping_add(2)
        {
            20 as libc::c_int as libc::c_uint
        } else {
            ((*params).cParams.windowLog).wrapping_add(2)
        };
    }
    return if jobLog
        < (if MEM_32bits() != 0 { 29 as libc::c_int } else { 30 as libc::c_int })
            as libc::c_uint
    {
        jobLog
    } else {
        (if MEM_32bits() != 0 { 29 as libc::c_int } else { 30 as libc::c_int })
            as libc::c_uint
    };
}
unsafe extern "C" fn ZSTDMT_overlapLog_default(mut strat: ZSTD_strategy) -> libc::c_int {
    match strat as libc::c_uint {
        9 => return 9 as libc::c_int,
        8 | 7 => return 8 as libc::c_int,
        6 | 5 => return 7 as libc::c_int,
        4 | 3 | 2 | 1 | _ => {}
    }
    return 6 as libc::c_int;
}
unsafe extern "C" fn ZSTDMT_overlapLog(
    mut ovlog: libc::c_int,
    mut strat: ZSTD_strategy,
) -> libc::c_int {
    debug_assert!(0 as libc::c_int <= ovlog && ovlog <= 9);
    if ovlog == 0 {
        return ZSTDMT_overlapLog_default(strat);
    }
    return ovlog;
}
unsafe extern "C" fn ZSTDMT_computeOverlapSize(
    mut params: *const ZSTD_CCtx_params,
) -> libc::size_t {
    let overlapRLog = 9 as libc::c_int
        - ZSTDMT_overlapLog((*params).overlapLog, (*params).cParams.strategy);
    let mut ovLog = (if overlapRLog >= 8 {
        0 as libc::c_int as libc::c_uint
    } else {
        ((*params).cParams.windowLog).wrapping_sub(overlapRLog as libc::c_uint)
    }) as libc::c_int;
    debug_assert!(0 as libc::c_int <= overlapRLog && overlapRLog <= 8);
    if (*params).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        ovLog = (if (*params).cParams.windowLog
            < (ZSTDMT_computeTargetJobLog(params))
                .wrapping_sub(2)
        {
            (*params).cParams.windowLog
        } else {
            (ZSTDMT_computeTargetJobLog(params))
                .wrapping_sub(2)
        })
            .wrapping_sub(overlapRLog as libc::c_uint) as libc::c_int;
    }
    debug_assert!(0 as libc::c_int <= ovLog
        && ovLog
            <= (if ::core::mem::size_of::<libc::size_t>()
                == 4
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            }));
    return if ovLog == 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        (1) << ovLog
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream_internal(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> libc::size_t {
    debug_assert!(ERR_isError(ZSTD_checkCParams(params.cParams)) == 0);
    debug_assert!(!(!dict.is_null() && !cdict.is_null()));
    if params.nbWorkers != (*mtctx).params.nbWorkers {
        let err_code = ZSTDMT_resize(mtctx, params.nbWorkers as libc::c_uint);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
    }
    if params.jobSize != 0 as libc::c_int as libc::c_ulong
        && params.jobSize < ZSTDMT_JOBSIZE_MIN as libc::c_ulong
    {
        params.jobSize = ZSTDMT_JOBSIZE_MIN as libc::size_t;
    }
    if params.jobSize
        > (if MEM_32bits() != 0 {
            512 as libc::c_int * ((1) << 20 as libc::c_int)
        } else {
            1024 as libc::c_int * ((1) << 20 as libc::c_int)
        }) as libc::size_t
    {
        params
            .jobSize = (if MEM_32bits() != 0 {
            512 as libc::c_int * ((1) << 20 as libc::c_int)
        } else {
            1024 as libc::c_int * ((1) << 20 as libc::c_int)
        }) as libc::size_t;
    }
    if (*mtctx).allJobsCompleted == 0 {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        (*mtctx).allJobsCompleted = 1 as libc::c_int as libc::c_uint;
    }
    (*mtctx).params = params;
    (*mtctx).frameContentSize = pledgedSrcSize;
    if !dict.is_null() {
        ZSTD_freeCDict((*mtctx).cdictLocal);
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
        if ((*mtctx).cdictLocal).is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
        }
    } else {
        ZSTD_freeCDict((*mtctx).cdictLocal);
        (*mtctx).cdictLocal = NULL_0 as *mut ZSTD_CDict;
        (*mtctx).cdict = cdict;
    }
    (*mtctx).targetPrefixSize = ZSTDMT_computeOverlapSize(&mut params);
    (*mtctx).targetSectionSize = params.jobSize;
    if (*mtctx).targetSectionSize == 0 {
        (*mtctx)
            .targetSectionSize = ((1)
            << ZSTDMT_computeTargetJobLog(&mut params)) as libc::size_t;
    }
    debug_assert!((*mtctx).targetSectionSize
        <= (if MEM_32bits() != 0 {
            512 as libc::c_int * ((1) << 20 as libc::c_int)
        } else {
            1024 as libc::c_int * ((1) << 20 as libc::c_int)
        }) as libc::size_t);
    if params.rsyncable != 0 {
        let jobSizeKB = ((*mtctx).targetSectionSize >> 10 as libc::c_int) as u32;
        debug_assert!(jobSizeKB >= 1);
        let rsyncBits = (ZSTD_highbit32(jobSizeKB))
            .wrapping_add(10);
        debug_assert!(rsyncBits >= (17 as libc::c_int + 2) as libc::c_uint);
        (*mtctx).rsync.hash = 0 as libc::c_int as u64;
        (*mtctx)
            .rsync
            .hitMask = ((1) << rsyncBits)
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
    let windowSize = (if (*mtctx).params.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        (1) << (*mtctx).params.cParams.windowLog
    } else {
        0 as libc::c_int as libc::c_uint
    }) as libc::size_t;
    let nbSlackBuffers = (2 as libc::c_int
        + ((*mtctx).targetPrefixSize > 0) as libc::c_int)
        as libc::size_t;
    let slackSize = ((*mtctx).targetSectionSize).wrapping_mul(nbSlackBuffers);
    let nbWorkers = (if (*mtctx).params.nbWorkers > 1 {
        (*mtctx).params.nbWorkers
    } else {
        1 as libc::c_int
    }) as libc::size_t;
    let sectionsSize = ((*mtctx).targetSectionSize).wrapping_mul(nbWorkers);
    let capacity = (if windowSize > sectionsSize { windowSize } else { sectionsSize })
        .wrapping_add(slackSize);
    if (*mtctx).roundBuff.capacity < capacity {
        if !((*mtctx).roundBuff.buffer).is_null() {
            ZSTD_customFree(
                (*mtctx).roundBuff.buffer as *mut libc::c_void,
                (*mtctx).cMem,
            );
        }
        (*mtctx)
            .roundBuff
            .buffer = ZSTD_customMalloc(capacity, (*mtctx).cMem) as *mut u8;
        if ((*mtctx).roundBuff.buffer).is_null() {
            (*mtctx).roundBuff.capacity = 0 as libc::c_int as libc::size_t;
            return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
        }
        (*mtctx).roundBuff.capacity = capacity;
    }
    (*mtctx).roundBuff.pos = 0 as libc::c_int as libc::size_t;
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0 as libc::c_int as libc::size_t;
    (*mtctx).inBuff.prefix = kNullRange;
    (*mtctx).doneJobID = 0 as libc::c_int as libc::c_uint;
    (*mtctx).nextJobID = 0 as libc::c_int as libc::c_uint;
    (*mtctx).frameEnded = 0 as libc::c_int as libc::c_uint;
    (*mtctx).allJobsCompleted = 0 as libc::c_int as libc::c_uint;
    (*mtctx).consumed = 0 as libc::c_int as libc::c_ulonglong;
    (*mtctx).produced = 0 as libc::c_int as libc::c_ulonglong;
    if ZSTDMT_serialState_reset(
        &mut (*mtctx).serial,
        (*mtctx).seqPool,
        params,
        (*mtctx).targetSectionSize,
        dict,
        dictSize,
        dictContentType,
    ) != 0
    {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDMT_writeLastEmptyBlock(mut job: *mut ZSTDMT_jobDescription) {
    debug_assert!((*job).lastJob == 1);
    debug_assert!((*job).src.size == 0);
    debug_assert!((*job).firstJob == 0);
    debug_assert!(((*job).dstBuff.start).is_null());
    (*job).dstBuff = ZSTDMT_getBuffer((*job).bufPool);
    if ((*job).dstBuff.start).is_null() {
        (*job).cSize = -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
        return;
    }
    debug_assert!((*job).dstBuff.capacity >= ZSTD_blockHeaderSize);
    (*job).src = kNullRange;
    (*job)
        .cSize = ZSTD_writeLastEmptyBlock((*job).dstBuff.start, (*job).dstBuff.capacity);
    debug_assert!(ERR_isError((*job).cSize) == 0);
    debug_assert!((*job).consumed == 0);;
}
unsafe extern "C" fn ZSTDMT_createCompressionJob(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut srcSize: libc::size_t,
    mut endOp: ZSTD_EndDirective,
) -> libc::size_t {
    let jobID = (*mtctx).nextJobID & (*mtctx).jobIDMask;
    let endFrame = (endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint)
        as libc::c_int;
    if (*mtctx).nextJobID > ((*mtctx).doneJobID).wrapping_add((*mtctx).jobIDMask) {
        debug_assert!((*mtctx).nextJobID & (*mtctx).jobIDMask
            == (*mtctx).doneJobID & (*mtctx).jobIDMask);
        return 0 as libc::c_int as libc::size_t;
    }
    if (*mtctx).jobReady == 0 {
        let mut src = (*mtctx).inBuff.buffer.start as *const u8;
        let ref mut fresh6 = (*((*mtctx).jobs).offset(jobID as isize)).src.start;
        *fresh6 = src as *const libc::c_void;
        (*((*mtctx).jobs).offset(jobID as isize)).src.size = srcSize;
        debug_assert!((*mtctx).inBuff.filled >= srcSize);
        (*((*mtctx).jobs).offset(jobID as isize)).prefix = (*mtctx).inBuff.prefix;
        (*((*mtctx).jobs).offset(jobID as isize)).consumed = 0 as libc::c_int as libc::size_t;
        (*((*mtctx).jobs).offset(jobID as isize)).cSize = 0 as libc::c_int as libc::size_t;
        (*((*mtctx).jobs).offset(jobID as isize)).params = (*mtctx).params;
        let ref mut fresh7 = (*((*mtctx).jobs).offset(jobID as isize)).cdict;
        *fresh7 = if (*mtctx).nextJobID == 0 {
            (*mtctx).cdict
        } else {
            NULL_0 as *const ZSTD_CDict
        };
        (*((*mtctx).jobs).offset(jobID as isize))
            .fullFrameSize = (*mtctx).frameContentSize;
        (*((*mtctx).jobs).offset(jobID as isize)).dstBuff = g_nullBuffer;
        let ref mut fresh8 = (*((*mtctx).jobs).offset(jobID as isize)).cctxPool;
        *fresh8 = (*mtctx).cctxPool;
        let ref mut fresh9 = (*((*mtctx).jobs).offset(jobID as isize)).bufPool;
        *fresh9 = (*mtctx).bufPool;
        let ref mut fresh10 = (*((*mtctx).jobs).offset(jobID as isize)).seqPool;
        *fresh10 = (*mtctx).seqPool;
        let ref mut fresh11 = (*((*mtctx).jobs).offset(jobID as isize)).serial;
        *fresh11 = &mut (*mtctx).serial;
        (*((*mtctx).jobs).offset(jobID as isize)).jobID = (*mtctx).nextJobID;
        (*((*mtctx).jobs).offset(jobID as isize))
            .firstJob = ((*mtctx).nextJobID == 0)
            as libc::c_int as libc::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).lastJob = endFrame as libc::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize))
            .frameChecksumNeeded = ((*mtctx).params.fParams.checksumFlag != 0
            && endFrame != 0 && (*mtctx).nextJobID > 0)
            as libc::c_int as libc::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize))
            .dstFlushed = 0 as libc::c_int as libc::size_t;
        (*mtctx)
            .roundBuff
            .pos = ((*mtctx).roundBuff.pos as libc::c_ulong).wrapping_add(srcSize)
            ;
        (*mtctx).inBuff.buffer = g_nullBuffer;
        (*mtctx).inBuff.filled = 0 as libc::c_int as libc::size_t;
        if endFrame == 0 {
            let newPrefixSize = if srcSize < (*mtctx).targetPrefixSize {
                srcSize
            } else {
                (*mtctx).targetPrefixSize
            };
            (*mtctx)
                .inBuff
                .prefix
                .start = src.offset(srcSize as isize).offset(-(newPrefixSize as isize))
                as *const libc::c_void;
            (*mtctx).inBuff.prefix.size = newPrefixSize;
        } else {
            (*mtctx).inBuff.prefix = kNullRange;
            (*mtctx).frameEnded = endFrame as libc::c_uint;
            if (*mtctx).nextJobID == 0 {
                (*mtctx).params.fParams.checksumFlag = 0 as libc::c_int;
            }
        }
        if srcSize == 0
            && (*mtctx).nextJobID > 0
        {
            debug_assert!(endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint);
            ZSTDMT_writeLastEmptyBlock(((*mtctx).jobs).offset(jobID as isize));
            (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
            return 0 as libc::c_int as libc::size_t;
        }
    }
    if POOL_tryAdd(
        (*mtctx).factory,
        Some(ZSTDMT_compressionJob as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription
            as *mut libc::c_void,
    ) != 0
    {
        (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
        (*mtctx).jobReady = 0 as libc::c_int;
    } else {
        (*mtctx).jobReady = 1 as libc::c_int;
    }
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDMT_flushProduced(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut blockToFlush: libc::c_uint,
    mut end: ZSTD_EndDirective,
) -> libc::size_t {
    let wJobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
    debug_assert!((*output).size >= (*output).pos);
    pthread_mutex_lock((*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
    if blockToFlush != 0 && (*mtctx).doneJobID < (*mtctx).nextJobID {
        debug_assert!((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed
            <= (*((*mtctx).jobs).offset(wJobID as isize)).cSize);
        while (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed
            == (*((*mtctx).jobs).offset(wJobID as isize)).cSize
        {
            if (*((*mtctx).jobs).offset(wJobID as isize)).consumed
                == (*((*mtctx).jobs).offset(wJobID as isize)).src.size
            {
                break;
            }
            pthread_cond_wait(
                (*((*mtctx).jobs).offset(wJobID as isize)).job_cond,
                (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex,
            );
        }
    }
    let mut cSize = (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
    let srcConsumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
    let srcSize = (*((*mtctx).jobs).offset(wJobID as isize)).src.size;
    pthread_mutex_unlock((*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
    if ERR_isError(cSize) != 0 {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize;
    }
    debug_assert!(srcConsumed <= srcSize);
    if srcConsumed == srcSize
        && (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded != 0
    {
        let checksum = ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as u32;
        MEM_writeLE32(
            ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start
                as *mut libc::c_char)
                .offset((*((*mtctx).jobs).offset(wJobID as isize)).cSize as isize)
                as *mut libc::c_void,
            checksum,
        );
        cSize = (cSize as libc::c_ulong).wrapping_add(4)
            ;
        let ref mut fresh12 = (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
        *fresh12 = (*fresh12 as libc::c_ulong)
            .wrapping_add(4) ;
        (*((*mtctx).jobs).offset(wJobID as isize))
            .frameChecksumNeeded = 0 as libc::c_int as libc::c_uint;
    }
    if cSize > 0 {
        let toFlush = if cSize
            .wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
            < ((*output).size).wrapping_sub((*output).pos)
        {
            cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
        } else {
            ((*output).size).wrapping_sub((*output).pos)
        };
        debug_assert!((*mtctx).doneJobID < (*mtctx).nextJobID);
        debug_assert!(cSize >= (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed);
        debug_assert!(!((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start).is_null());
        if toFlush > 0 {
            libc::memcpy(
                ((*output).dst as *mut libc::c_char).offset((*output).pos as isize)
                    as *mut libc::c_void,
                ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start
                    as *const libc::c_char)
                    .offset(
                        (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed as isize,
                    ) as *const libc::c_void,
                toFlush as libc::size_t,
            );
        }
        (*output)
            .pos = ((*output).pos as libc::c_ulong).wrapping_add(toFlush) as libc::size_t
            as libc::size_t;
        let ref mut fresh13 = (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed;
        *fresh13 = (*fresh13 as libc::c_ulong).wrapping_add(toFlush) ;
        if srcConsumed == srcSize
            && (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed == cSize
        {
            ZSTDMT_releaseBuffer(
                (*mtctx).bufPool,
                (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff,
            );
            (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff = g_nullBuffer;
            (*((*mtctx).jobs).offset(wJobID as isize))
                .cSize = 0 as libc::c_int as libc::size_t;
            (*mtctx)
                .consumed = ((*mtctx).consumed)
                .wrapping_add(srcSize as libc::c_ulonglong);
            (*mtctx)
                .produced = ((*mtctx).produced).wrapping_add(cSize as libc::c_ulonglong);
            (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
        }
    }
    if cSize > (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed {
        return cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed);
    }
    if srcSize > srcConsumed {
        return 1 as libc::c_int as libc::size_t;
    }
    if (*mtctx).doneJobID < (*mtctx).nextJobID {
        return 1 as libc::c_int as libc::size_t;
    }
    if (*mtctx).jobReady != 0 {
        return 1 as libc::c_int as libc::size_t;
    }
    if (*mtctx).inBuff.filled > 0 {
        return 1 as libc::c_int as libc::size_t;
    }
    (*mtctx).allJobsCompleted = (*mtctx).frameEnded;
    if end as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint {
        return ((*mtctx).frameEnded == 0) as libc::c_int as libc::size_t;
    }
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDMT_getInputDataInUse(mut mtctx: *mut ZSTDMT_CCtx) -> range_t {
    let firstJobID = (*mtctx).doneJobID;
    let lastJobID = (*mtctx).nextJobID;
    let mut jobID: libc::c_uint = 0;
    jobID = firstJobID;
    while jobID < lastJobID {
        let wJobID = jobID & (*mtctx).jobIDMask;
        let mut consumed: libc::size_t = 0;
        pthread_mutex_lock((*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
        consumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
        pthread_mutex_unlock((*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
        if consumed < (*((*mtctx).jobs).offset(wJobID as isize)).src.size {
            let mut range = (*((*mtctx).jobs).offset(wJobID as isize)).prefix;
            if range.size == 0 {
                range = (*((*mtctx).jobs).offset(wJobID as isize)).src;
            }
            debug_assert!(range.start <= (*((*mtctx).jobs).offset(wJobID as isize)).src.start);
            return range;
        }
        jobID = jobID.wrapping_add(1);
    }
    return kNullRange;
}
unsafe extern "C" fn ZSTDMT_isOverlapped(
    mut buffer: buffer_t,
    mut range: range_t,
) -> libc::c_int {
    let bufferStart = buffer.start as *const u8;
    let rangeStart = range.start as *const u8;
    if rangeStart.is_null() || bufferStart.is_null() {
        return 0 as libc::c_int;
    }
    let bufferEnd = bufferStart.offset(buffer.capacity as isize);
    let rangeEnd = rangeStart.offset(range.size as isize);
    if bufferStart == bufferEnd || rangeStart == rangeEnd {
        return 0 as libc::c_int;
    }
    return (bufferStart < rangeEnd && rangeStart < bufferEnd) as libc::c_int;
}
unsafe extern "C" fn ZSTDMT_doesOverlapWindow(
    mut buffer: buffer_t,
    mut window: ZSTD_window_t,
) -> libc::c_int {
    let mut extDict = range_t {
        start: 0 as *const libc::c_void,
        size: 0,
    };
    let mut prefix = range_t {
        start: 0 as *const libc::c_void,
        size: 0,
    };
    extDict
        .start = (window.dictBase).offset(window.lowLimit as isize)
        as *const libc::c_void;
    extDict.size = (window.dictLimit).wrapping_sub(window.lowLimit) as libc::size_t;
    prefix
        .start = (window.base).offset(window.dictLimit as isize) as *const libc::c_void;
    prefix
        .size = (window.nextSrc)
        .offset_from((window.base).offset(window.dictLimit as isize)) as libc::c_long
        as libc::size_t;
    return (ZSTDMT_isOverlapped(buffer, extDict) != 0
        || ZSTDMT_isOverlapped(buffer, prefix) != 0) as libc::c_int;
}
unsafe extern "C" fn ZSTDMT_waitForLdmComplete(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut buffer: buffer_t,
) {
    if (*mtctx).params.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        let mut mutex: *mut *mut pthread_mutex_t = &mut (*mtctx).serial.ldmWindowMutex;
        pthread_mutex_lock(*mutex);
        while ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow) != 0 {
            pthread_cond_wait((*mtctx).serial.ldmWindowCond, *mutex);
        }
        pthread_mutex_unlock(*mutex);
    }
}
unsafe extern "C" fn ZSTDMT_tryGetInputRange(
    mut mtctx: *mut ZSTDMT_CCtx,
) -> libc::c_int {
    let inUse = ZSTDMT_getInputDataInUse(mtctx);
    let spaceLeft = ((*mtctx).roundBuff.capacity).wrapping_sub((*mtctx).roundBuff.pos);
    let target = (*mtctx).targetSectionSize;
    let mut buffer = buffer_t {
        start: 0 as *mut libc::c_void,
        capacity: 0,
    };
    debug_assert!(((*mtctx).inBuff.buffer.start).is_null());
    debug_assert!((*mtctx).roundBuff.capacity >= target);
    if spaceLeft < target {
        let start = (*mtctx).roundBuff.buffer;
        let prefixSize = (*mtctx).inBuff.prefix.size;
        buffer.start = start as *mut libc::c_void;
        buffer.capacity = prefixSize;
        if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
            return 0 as libc::c_int;
        }
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        libc::memmove(
            start as *mut libc::c_void,
            (*mtctx).inBuff.prefix.start,
            prefixSize as libc::size_t,
        );
        (*mtctx).inBuff.prefix.start = start as *const libc::c_void;
        (*mtctx).roundBuff.pos = prefixSize;
    }
    buffer
        .start = ((*mtctx).roundBuff.buffer).offset((*mtctx).roundBuff.pos as isize)
        as *mut libc::c_void;
    buffer.capacity = target;
    if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
        return 0 as libc::c_int;
    }
    debug_assert!(ZSTDMT_isOverlapped(buffer, (*mtctx).inBuff.prefix) == 0);
    ZSTDMT_waitForLdmComplete(mtctx, buffer);
    (*mtctx).inBuff.buffer = buffer;
    (*mtctx).inBuff.filled = 0 as libc::c_int as libc::size_t;
    debug_assert!(((*mtctx).roundBuff.pos).wrapping_add(buffer.capacity)
        <= (*mtctx).roundBuff.capacity);
    return 1 as libc::c_int;
}
unsafe extern "C" fn findSynchronizationPoint(
    mut mtctx: *const ZSTDMT_CCtx,
    input: ZSTD_inBuffer,
) -> syncPoint_t {
    let istart = (input.src as *const u8).offset(input.pos as isize);
    let primePower = (*mtctx).rsync.primePower;
    let hitMask = (*mtctx).rsync.hitMask;
    let mut syncPoint = syncPoint_t { toLoad: 0, flush: 0 };
    let mut hash: u64 = 0;
    let mut prev = 0 as *const u8;
    let mut pos: libc::size_t = 0;
    syncPoint
        .toLoad = if (input.size).wrapping_sub(input.pos)
        < ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled)
    {
        (input.size).wrapping_sub(input.pos)
    } else {
        ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled)
    };
    syncPoint.flush = 0 as libc::c_int;
    if (*mtctx).params.rsyncable == 0 {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(input.size).wrapping_sub(input.pos)
        < RSYNC_MIN_BLOCK_SIZE as libc::c_ulong
    {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad)
        < RSYNC_LENGTH as libc::c_ulong
    {
        return syncPoint;
    }
    if (*mtctx).inBuff.filled < RSYNC_MIN_BLOCK_SIZE as libc::c_ulong {
        pos = (RSYNC_MIN_BLOCK_SIZE as libc::c_ulong)
            .wrapping_sub((*mtctx).inBuff.filled);
        if pos >= RSYNC_LENGTH as libc::c_ulong {
            prev = istart.offset(pos as isize).offset(-(RSYNC_LENGTH as isize));
            hash = ZSTD_rollingHash_compute(
                prev as *const libc::c_void,
                RSYNC_LENGTH as libc::size_t,
            );
        } else {
            debug_assert!((*mtctx).inBuff.filled >= 32);
            prev = ((*mtctx).inBuff.buffer.start as *const u8)
                .offset((*mtctx).inBuff.filled as isize)
                .offset(-(RSYNC_LENGTH as isize));
            hash = ZSTD_rollingHash_compute(
                prev.offset(pos as isize) as *const libc::c_void,
                (RSYNC_LENGTH as libc::c_ulong).wrapping_sub(pos),
            );
            hash = ZSTD_rollingHash_append(hash, istart as *const libc::c_void, pos);
        }
    } else {
        debug_assert!((*mtctx).inBuff.filled
            >= ((1) << 17 as libc::c_int) as libc::c_ulong);
        debug_assert!((1) << 17 as libc::c_int >= 32);
        pos = 0 as libc::c_int as libc::size_t;
        prev = ((*mtctx).inBuff.buffer.start as *const u8)
            .offset((*mtctx).inBuff.filled as isize)
            .offset(-(RSYNC_LENGTH as isize));
        hash = ZSTD_rollingHash_compute(
            prev as *const libc::c_void,
            RSYNC_LENGTH as libc::size_t,
        );
        if hash & hitMask == hitMask {
            syncPoint.toLoad = 0 as libc::c_int as libc::size_t;
            syncPoint.flush = 1 as libc::c_int;
            return syncPoint;
        }
    }
    debug_assert!(pos < 32
        || ZSTD_rollingHash_compute(
            istart.offset(pos as isize).offset(-(32))
                as *const libc::c_void,
            32 as libc::c_int as libc::size_t,
        ) == hash);
    while pos < syncPoint.toLoad {
        let toRemove = (if pos < RSYNC_LENGTH as libc::c_ulong {
            *prev.offset(pos as isize) as libc::c_int
        } else {
            *istart.offset(pos.wrapping_sub(RSYNC_LENGTH as libc::c_ulong) as isize)
                as libc::c_int
        }) as u8;
        hash = ZSTD_rollingHash_rotate(
            hash,
            toRemove,
            *istart.offset(pos as isize),
            primePower,
        );
        debug_assert!(((*mtctx).inBuff.filled).wrapping_add(pos)
            >= ((1) << 17 as libc::c_int) as libc::c_ulong);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = pos.wrapping_add(1);
            syncPoint.flush = 1 as libc::c_int;
            pos = pos.wrapping_add(1);
            break;
        } else {
            pos = pos.wrapping_add(1);
        }
    }
    debug_assert!(pos < 32
        || ZSTD_rollingHash_compute(
            istart.offset(pos as isize).offset(-(32))
                as *const libc::c_void,
            32 as libc::c_int as libc::size_t,
        ) == hash);
    return syncPoint;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_nextInputSizeHint(
    mut mtctx: *const ZSTDMT_CCtx,
) -> libc::size_t {
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
) -> libc::size_t {
    let mut forwardInputProgress = 0 as libc::c_int as libc::c_uint;
    debug_assert!((*output).pos <= (*output).size);
    debug_assert!((*input).pos <= (*input).size);
    if (*mtctx).frameEnded != 0
        && endOp as libc::c_uint == ZSTD_e_continue as libc::c_int as libc::c_uint
    {
        return -(ZSTD_error_stage_wrong as libc::c_int) as libc::size_t;
    }
    if (*mtctx).jobReady == 0 && (*input).size > (*input).pos {
        if ((*mtctx).inBuff.buffer.start).is_null() {
            debug_assert!((*mtctx).inBuff.filled == 0);
            if ZSTDMT_tryGetInputRange(mtctx) == 0 {
                debug_assert!((*mtctx).doneJobID != (*mtctx).nextJobID);
            }
        }
        if !((*mtctx).inBuff.buffer.start).is_null() {
            let syncPoint = findSynchronizationPoint(mtctx, *input);
            if syncPoint.flush != 0
                && endOp as libc::c_uint
                    == ZSTD_e_continue as libc::c_int as libc::c_uint
            {
                endOp = ZSTD_e_flush;
            }
            debug_assert!((*mtctx).inBuff.buffer.capacity >= (*mtctx).targetSectionSize);
            libc::memcpy(
                ((*mtctx).inBuff.buffer.start as *mut libc::c_char)
                    .offset((*mtctx).inBuff.filled as isize) as *mut libc::c_void,
                ((*input).src as *const libc::c_char).offset((*input).pos as isize)
                    as *const libc::c_void,
                syncPoint.toLoad as libc::size_t,
            );
            (*input)
                .pos = ((*input).pos as libc::c_ulong).wrapping_add(syncPoint.toLoad)
                ;
            (*mtctx)
                .inBuff
                .filled = ((*mtctx).inBuff.filled as libc::c_ulong)
                .wrapping_add(syncPoint.toLoad) ;
            forwardInputProgress = (syncPoint.toLoad > 0)
                as libc::c_int as libc::c_uint;
        }
    }
    if (*input).pos < (*input).size
        && endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
    {
        debug_assert!((*mtctx).inBuff.filled == 0
            || (*mtctx).inBuff.filled == (*mtctx).targetSectionSize
            || (*mtctx).params.rsyncable != 0);
        endOp = ZSTD_e_flush;
    }
    if (*mtctx).jobReady != 0 || (*mtctx).inBuff.filled >= (*mtctx).targetSectionSize
        || endOp as libc::c_uint != ZSTD_e_continue as libc::c_int as libc::c_uint
            && (*mtctx).inBuff.filled > 0
        || endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
            && (*mtctx).frameEnded == 0
    {
        let jobSize = (*mtctx).inBuff.filled;
        debug_assert!((*mtctx).inBuff.filled <= (*mtctx).targetSectionSize);
        let err_code = ZSTDMT_createCompressionJob(mtctx, jobSize, endOp);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
    }
    let remainingToFlush = ZSTDMT_flushProduced(
        mtctx,
        output,
        (forwardInputProgress == 0) as libc::c_int as libc::c_uint,
        endOp,
    );
    if (*input).pos < (*input).size {
        return if remainingToFlush > 1 {
            remainingToFlush
        } else {
            1 as libc::c_int as libc::c_ulong
        };
    }
    return remainingToFlush;
}
