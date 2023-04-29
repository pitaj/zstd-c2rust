use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use core::arch::asm;
extern "C" {
    pub type ZSTDMT_CCtx_s;
    pub type POOL_ctx_s;
    pub type ZSTD_DCtx_s;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn FSE_readNCount(
        normalizedCounter: *mut libc::c_short,
        maxSymbolValuePtr: *mut libc::c_uint,
        tableLogPtr: *mut libc::c_uint,
        rBuffer: *const libc::c_void,
        rBuffSize: size_t,
    ) -> size_t;
    fn FSE_buildCTable_wksp(
        ct: *mut FSE_CTable,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
    ) -> size_t;
    fn HUF_optimalTableLog(
        maxTableLog: libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
        table: *mut HUF_CElt,
        count: *const libc::c_uint,
        flags: libc::c_int,
    ) -> libc::c_uint;
    fn HUF_writeCTable_wksp(
        dst: *mut libc::c_void,
        maxDstSize: size_t,
        CTable: *const HUF_CElt,
        maxSymbolValue: libc::c_uint,
        huffLog: libc::c_uint,
        workspace: *mut libc::c_void,
        workspaceSize: size_t,
    ) -> size_t;
    fn HUF_estimateCompressedSize(
        CTable: *const HUF_CElt,
        count: *const libc::c_uint,
        maxSymbolValue: libc::c_uint,
    ) -> size_t;
    fn HUF_validateCTable(
        CTable: *const HUF_CElt,
        count: *const libc::c_uint,
        maxSymbolValue: libc::c_uint,
    ) -> libc::c_int;
    fn HUF_buildCTable_wksp(
        tree: *mut HUF_CElt,
        count: *const libc::c_uint,
        maxSymbolValue: U32,
        maxNbBits: U32,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
    ) -> size_t;
    fn HUF_readCTable(
        CTable: *mut HUF_CElt,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: size_t,
        hasZeroWeights: *mut libc::c_uint,
    ) -> size_t;
    fn ZSTD_XXH64_reset(
        statePtr: *mut XXH64_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const libc::c_void,
        length: size_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    fn ZSTD_trace_compress_begin(cctx: *const ZSTD_CCtx_s) -> ZSTD_TraceCtx;
    fn ZSTD_trace_compress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace);
    fn ZSTDMT_compressStream_generic(
        mtctx: *mut ZSTDMT_CCtx,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
        endOp: ZSTD_EndDirective,
    ) -> size_t;
    fn ZSTDMT_updateCParams_whileCompressing(
        mtctx: *mut ZSTDMT_CCtx,
        cctxParams: *const ZSTD_CCtx_params,
    );
    fn ZSTDMT_initCStream_internal(
        mtctx: *mut ZSTDMT_CCtx,
        dict: *const libc::c_void,
        dictSize: size_t,
        dictContentType: ZSTD_dictContentType_e,
        cdict: *const ZSTD_CDict,
        params: ZSTD_CCtx_params,
        pledgedSrcSize: libc::c_ulonglong,
    ) -> size_t;
    fn ZSTDMT_createCCtx_advanced(
        nbWorkers: libc::c_uint,
        cMem: ZSTD_customMem,
        pool: *mut ZSTD_threadPool,
    ) -> *mut ZSTDMT_CCtx;
    fn ZSTDMT_nextInputSizeHint(mtctx: *const ZSTDMT_CCtx) -> size_t;
    fn ZSTDMT_freeCCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t;
    fn ZSTDMT_sizeof_CCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t;
    fn ZSTDMT_getFrameProgression(mtctx: *mut ZSTDMT_CCtx) -> ZSTD_frameProgression;
    fn ZSTDMT_toFlushNow(mtctx: *mut ZSTDMT_CCtx) -> size_t;
    fn ZSTD_selectEncodingType(
        repeatMode: *mut FSE_repeat,
        count: *const libc::c_uint,
        max: libc::c_uint,
        mostFrequent: size_t,
        nbSeq: size_t,
        FSELog: libc::c_uint,
        prevCTable: *const FSE_CTable,
        defaultNorm: *const libc::c_short,
        defaultNormLog: U32,
        isDefaultAllowed: ZSTD_defaultPolicy_e,
        strategy: ZSTD_strategy,
    ) -> symbolEncodingType_e;
    fn ZSTD_buildCTable(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        nextCTable: *mut FSE_CTable,
        FSELog: U32,
        type_0: symbolEncodingType_e,
        count: *mut libc::c_uint,
        max: U32,
        codeTable: *const BYTE,
        nbSeq: size_t,
        defaultNorm: *const S16,
        defaultNormLog: U32,
        defaultMax: U32,
        prevCTable: *const FSE_CTable,
        prevCTableSize: size_t,
        entropyWorkspace: *mut libc::c_void,
        entropyWorkspaceSize: size_t,
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
    fn ZSTD_compressLiterals(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
        entropyWorkspace: *mut libc::c_void,
        entropyWorkspaceSize: size_t,
        prevHuf: *const ZSTD_hufCTables_t,
        nextHuf: *mut ZSTD_hufCTables_t,
        strategy: ZSTD_strategy,
        disableLiteralCompression: libc::c_int,
        suspectUncompressible: libc::c_int,
        bmi2: libc::c_int,
    ) -> size_t;
    fn ZSTD_fillHashTable(
        ms: *mut ZSTD_matchState_t,
        end: *const libc::c_void,
        dtlm: ZSTD_dictTableLoadMethod_e,
        tfp: ZSTD_tableFillPurpose_e,
    );
    fn ZSTD_compressBlock_fast(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_fast_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_fast_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_fillDoubleHashTable(
        ms: *mut ZSTD_matchState_t,
        end: *const libc::c_void,
        dtlm: ZSTD_dictTableLoadMethod_e,
        tfp: ZSTD_tableFillPurpose_e,
    );
    fn ZSTD_compressBlock_doubleFast(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_doubleFast_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_doubleFast_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_insertAndFindFirstIndex(ms: *mut ZSTD_matchState_t, ip: *const BYTE) -> U32;
    fn ZSTD_row_update(ms: *mut ZSTD_matchState_t, ip: *const BYTE);
    fn ZSTD_dedicatedDictSearch_lazy_loadDictionary(
        ms: *mut ZSTD_matchState_t,
        ip: *const BYTE,
    );
    fn ZSTD_compressBlock_btlazy2(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btlazy2_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_dictMatchState_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_dictMatchState_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_dictMatchState_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_dedicatedDictSearch(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_dedicatedDictSearch(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_dedicatedDictSearch(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_dedicatedDictSearch_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_dedicatedDictSearch_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_dedicatedDictSearch_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_greedy_extDict_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy_extDict_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_lazy2_extDict_row(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btlazy2_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_updateTree(ms: *mut ZSTD_matchState_t, ip: *const BYTE, iend: *const BYTE);
    fn ZSTD_compressBlock_btopt(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btultra(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btultra2(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btopt_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btultra_dictMatchState(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btopt_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_compressBlock_btultra_extDict(
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_ldm_fillHashTable(
        state: *mut ldmState_t,
        ip: *const BYTE,
        iend: *const BYTE,
        params: *const ldmParams_t,
    );
    fn ZSTD_ldm_generateSequences(
        ldms: *mut ldmState_t,
        sequences: *mut rawSeqStore_t,
        params: *const ldmParams_t,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_ldm_blockCompress(
        rawSeqStore: *mut rawSeqStore_t,
        ms: *mut ZSTD_matchState_t,
        seqStore: *mut seqStore_t,
        rep: *mut U32,
        useRowMatchFinder: ZSTD_paramSwitch_e,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_ldm_skipSequences(
        rawSeqStore: *mut rawSeqStore_t,
        srcSize: size_t,
        minMatch: U32,
    );
    fn ZSTD_ldm_skipRawSeqStoreBytes(rawSeqStore: *mut rawSeqStore_t, nbBytes: size_t);
    fn ZSTD_ldm_getTableSize(params: ldmParams_t) -> size_t;
    fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: size_t) -> size_t;
    fn ZSTD_ldm_adjustParameters(
        params: *mut ldmParams_t,
        cParams: *const ZSTD_compressionParameters,
    );
    fn ZSTD_compressSuperBlock(
        zc: *mut ZSTD_CCtx,
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
        lastBlock: libc::c_uint,
    ) -> size_t;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __loadu_si128 {
    pub __v: __m128i_u,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si128 {
    pub __v: __m128i_u,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U8 = uint8_t;
pub type U16 = uint16_t;
pub type S16 = int16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type unalign64 = U64;
pub type unalignArch = size_t;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
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
pub struct ZSTD_CDict_s {
    pub dictContent: *const libc::c_void,
    pub dictContentSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub entropyWorkspace: *mut U32,
    pub workspace: ZSTD_cwksp,
    pub matchState: ZSTD_matchState_t,
    pub cBlockState: ZSTD_compressedBlockState_t,
    pub customMem: ZSTD_customMem,
    pub dictID: U32,
    pub compressionLevel: libc::c_int,
    pub useRowMatchFinder: ZSTD_paramSwitch_e,
}
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [U32; 3],
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
pub type HUF_CElt = size_t;
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
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_symbolEncodingTypeStats_t {
    pub LLtype: U32,
    pub Offtype: U32,
    pub MLtype: U32,
    pub size: size_t,
    pub lastCountSize: size_t,
    pub longOffsets: libc::c_int,
}
pub type ZSTD_defaultPolicy_e = libc::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_defaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_defaultPolicy_e = 0;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
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
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: libc::c_int,
    pub upperBound: libc::c_int,
}
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
pub type repcodes_t = repcodes_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [U32; 3],
}
pub const ZSTDbss_noCompress: C2RustUnnamed_2 = 1;
pub const ZSTDbss_compress: C2RustUnnamed_2 = 0;
pub type ZSTD_blockCompressor = Option::<
    unsafe extern "C" fn(
        *mut ZSTD_matchState_t,
        *mut seqStore_t,
        *mut U32,
        *const libc::c_void,
        size_t,
    ) -> size_t,
>;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_sequencePosition {
    pub idx: U32,
    pub posInSequence: U32,
    pub posInSrc: size_t,
}
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
pub type ZSTD_overlap_e = libc::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
pub const bt_compressed: C2RustUnnamed_1 = 2;
pub const bt_rle: C2RustUnnamed_1 = 1;
pub const bt_raw: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqStoreSplits {
    pub splitLocations: *mut U32,
    pub idx: size_t,
}
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Trace {
    pub version: libc::c_uint,
    pub streaming: libc::c_uint,
    pub dictionaryID: libc::c_uint,
    pub dictionaryIsCold: libc::c_uint,
    pub dictionarySize: size_t,
    pub uncompressedSize: size_t,
    pub compressedSize: size_t,
    pub params: *const ZSTD_CCtx_params_s,
    pub cctx: *const ZSTD_CCtx_s,
    pub dctx: *const ZSTD_DCtx_s,
}
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = libc::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
pub type ZSTD_compResetPolicy_e = libc::c_uint;
pub const ZSTDcrp_leaveDirty: ZSTD_compResetPolicy_e = 1;
pub const ZSTDcrp_makeClean: ZSTD_compResetPolicy_e = 0;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub type ZSTD_resetTarget_e = libc::c_uint;
pub const ZSTD_resetTarget_CCtx: ZSTD_resetTarget_e = 1;
pub const ZSTD_resetTarget_CDict: ZSTD_resetTarget_e = 0;
pub type ZSTD_indexResetPolicy_e = libc::c_uint;
pub const ZSTDirp_reset: ZSTD_indexResetPolicy_e = 1;
pub const ZSTDirp_continue: ZSTD_indexResetPolicy_e = 0;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
pub type ZSTD_cParamMode_e = libc::c_uint;
pub const ZSTD_cpm_unknown: ZSTD_cParamMode_e = 3;
pub const ZSTD_cpm_createCDict: ZSTD_cParamMode_e = 2;
pub const ZSTD_cpm_attachDict: ZSTD_cParamMode_e = 1;
pub const ZSTD_cpm_noAttachDict: ZSTD_cParamMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub type ZSTD_ResetDirective = libc::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: U32,
    pub f1d: U32,
    pub f7b: U32,
    pub f7c: U32,
}
pub type ZSTD_EndDirective = libc::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_CStream = ZSTD_CCtx;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub type ZSTD_sequenceCopier = Option::<
    unsafe extern "C" fn(
        *mut ZSTD_CCtx,
        *mut ZSTD_sequencePosition,
        *const ZSTD_Sequence,
        size_t,
        *const libc::c_void,
        size_t,
        ZSTD_paramSwitch_e,
    ) -> size_t,
>;
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
pub type C2RustUnnamed = libc::c_uint;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
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
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const bt_reserved: C2RustUnnamed_1 = 3;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ZSTD_MAGIC_DICTIONARY: libc::c_uint = 0xec30a437 as libc::c_uint;
pub const ZSTD_CONTENTSIZE_UNKNOWN: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    mut size: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut libc::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
    }
    return malloc(size);
}
pub const ZSTD_CLEVEL_DEFAULT: libc::c_int = 3 as libc::c_int;
static mut ZSTD_defaultCMem: ZSTD_customMem = unsafe {
    {
        let mut init = ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTD_allocFunction,
            >(NULL as libc::intptr_t),
            customFree: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTD_freeFunction,
            >(NULL as libc::intptr_t),
            opaque: NULL as *mut libc::c_void,
        };
        init
    }
};
pub const ZSTD_VERSION_NUMBER: libc::c_int = ZSTD_VERSION_MAJOR * 100 as libc::c_int
    * 100 as libc::c_int + ZSTD_VERSION_MINOR * 100 as libc::c_int
    + ZSTD_VERSION_RELEASE;
pub const ZSTD_VERSION_MAJOR: libc::c_int = 1 as libc::c_int;
pub const ZSTD_VERSION_MINOR: libc::c_int = 5 as libc::c_int;
pub const ZSTD_VERSION_RELEASE: libc::c_int = 5 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return *(ptr as *const unalign16);
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_readST(mut ptr: *const libc::c_void) -> size_t {
    return *(ptr as *const unalignArch);
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: U16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: U32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr)
    } else {
        return MEM_swap32(MEM_read32(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0.swap_bytes();
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
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: U32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void, mut val64: U64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
pub const ZSTD_MAGICNUMBER: libc::c_uint = 0xfd2fb528 as libc::c_uint;
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT: libc::c_int = 27 as libc::c_int;
pub const ZSTD_SKIPPABLEHEADERSIZE: libc::c_int = 8 as libc::c_int;
pub const ZSTD_MAGIC_SKIPPABLE_START: libc::c_int = 0x184d2a50 as libc::c_int;
pub const ZSTD_c_searchForExternalRepcodes: libc::c_int = 1016;
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1 as libc::c_int) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_BLOCKSIZE_MAX_MIN: libc::c_int = (1 as libc::c_int) << 10 as libc::c_int;
pub const ZSTD_c_maxBlockSize: libc::c_int = 1015;
pub const ZSTD_c_enableSeqProducerFallback: libc::c_int = 1014;
pub const ZSTD_c_prefetchCDictTables: libc::c_int = 1013;
pub const ZSTD_c_deterministicRefPrefix: libc::c_int = 1012;
pub const ZSTD_c_useRowMatchFinder: libc::c_int = 1011;
pub const ZSTD_c_useBlockSplitter: libc::c_int = 1010;
pub const ZSTD_c_validateSequences: libc::c_int = 1009;
pub const ZSTD_c_blockDelimiters: libc::c_int = 1008;
pub const ZSTD_c_stableOutBuffer: libc::c_int = 1007;
pub const ZSTD_c_stableInBuffer: libc::c_int = 1006;
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const ZSTD_SRCSIZEHINT_MAX: libc::c_int = INT_MAX;
pub const ZSTD_SRCSIZEHINT_MIN: libc::c_int = 0 as libc::c_int;
pub const ZSTD_c_srcSizeHint: libc::c_int = 1004;
pub const ZSTD_TARGETCBLOCKSIZE_MAX: libc::c_int = ZSTD_BLOCKSIZE_MAX;
pub const ZSTD_TARGETCBLOCKSIZE_MIN: libc::c_int = 64 as libc::c_int;
pub const ZSTD_c_targetCBlockSize: libc::c_int = 1003;
pub const ZSTD_c_literalCompressionMode: libc::c_int = 1002;
pub const ZSTD_c_forceAttachDict: libc::c_int = 1001;
pub const ZSTD_c_format: libc::c_int = 10;
pub const ZSTD_c_forceMaxWindow: libc::c_int = 1000;
pub const ZSTD_c_rsyncable: libc::c_int = 500;
pub const ZSTD_HASHLOG_MIN: libc::c_int = 6 as libc::c_int;
pub const ZSTD_WINDOWLOG_MAX_64: libc::c_int = 31 as libc::c_int;
pub const ZSTD_WINDOWLOG_MAX_32: libc::c_int = 30 as libc::c_int;
pub const ZSTD_LDM_HASHRATELOG_MIN: libc::c_int = 0 as libc::c_int;
pub const ZSTD_LDM_BUCKETSIZELOG_MAX: libc::c_int = 8 as libc::c_int;
pub const ZSTD_LDM_BUCKETSIZELOG_MIN: libc::c_int = 1 as libc::c_int;
pub const ZSTD_LDM_MINMATCH_MAX: libc::c_int = 4096 as libc::c_int;
pub const ZSTD_LDM_MINMATCH_MIN: libc::c_int = 4 as libc::c_int;
pub const ZSTD_LDM_HASHLOG_MIN: libc::c_int = ZSTD_HASHLOG_MIN;
pub const ZSTD_c_enableDedicatedDictSearch: libc::c_int = 1005;
pub const ZSTD_OVERLAPLOG_MAX: libc::c_int = 9 as libc::c_int;
pub const ZSTD_OVERLAPLOG_MIN: libc::c_int = 0 as libc::c_int;
pub const ZSTD_STRATEGY_MAX: libc::c_int = ZSTD_btultra2 as libc::c_int;
pub const ZSTD_STRATEGY_MIN: libc::c_int = ZSTD_fast as libc::c_int;
pub const ZSTD_TARGETLENGTH_MAX: libc::c_int = ZSTD_BLOCKSIZE_MAX;
pub const ZSTD_TARGETLENGTH_MIN: libc::c_int = 0 as libc::c_int;
pub const ZSTD_MINMATCH_MAX: libc::c_int = 7 as libc::c_int;
pub const ZSTD_MINMATCH_MIN: libc::c_int = 3 as libc::c_int;
pub const ZSTD_SEARCHLOG_MIN: libc::c_int = 1 as libc::c_int;
pub const ZSTD_CHAINLOG_MAX_64: libc::c_int = 30 as libc::c_int;
pub const ZSTD_CHAINLOG_MAX_32: libc::c_int = 29 as libc::c_int;
pub const ZSTD_CHAINLOG_MIN: libc::c_int = ZSTD_HASHLOG_MIN;
pub const ZSTD_WINDOWLOG_MIN: libc::c_int = 10 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    mut size: size_t,
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"unsigned int ZSTD_highbit32(U32)\0"))
                .as_ptr(),
        );
    }
    return (31 as libc::c_int as libc::c_uint)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
pub const STREAM_ACCUMULATOR_MIN_32: libc::c_int = 25 as libc::c_int;
pub const STREAM_ACCUMULATOR_MIN_64: libc::c_int = 57 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U64(value: U64, mut count: U32) -> U64 {
    if count < 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"count < 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"U64 ZSTD_rotateRight_U64(const U64, U32)\0"))
                .as_ptr(),
        );
    }
    count &= 0x3f as libc::c_int as libc::c_uint;
    return value >> count
        | value
            << ((0 as libc::c_uint).wrapping_sub(count)
                & 0x3f as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: U64) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"unsigned int ZSTD_countTrailingZeros64(U64)\0"))
                .as_ptr(),
        );
    }
    return (val as libc::c_ulonglong).trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"unsigned int ZSTD_countTrailingZeros32(U32)\0"))
                .as_ptr(),
        );
    }
    return val.trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros64(mut val: U64) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"unsigned int ZSTD_countLeadingZeros64(U64)\0"))
                .as_ptr(),
        );
    }
    return (val as libc::c_ulonglong).leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"unsigned int ZSTD_countLeadingZeros32(U32)\0"))
                .as_ptr(),
        );
    }
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: size_t) -> libc::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            return ZSTD_countTrailingZeros64(val) >> 3 as libc::c_int
        } else {
            return ZSTD_countTrailingZeros32(val as U32) >> 3 as libc::c_int
        }
    } else if MEM_64bits() != 0 {
        return ZSTD_countLeadingZeros64(val) >> 3 as libc::c_int
    } else {
        return ZSTD_countLeadingZeros32(val as U32) >> 3 as libc::c_int
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
pub const HUF_WORKSPACE_SIZE: libc::c_int = ((8 as libc::c_int) << 10 as libc::c_int)
    + 512 as libc::c_int;
pub const HUF_SYMBOLVALUE_MAX: libc::c_int = 255 as libc::c_int;
pub const HUF_OPTIMAL_DEPTH_THRESHOLD: libc::c_int = ZSTD_btultra as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_init(mut window: *mut ZSTD_window_t) {
    libc::memset(
        window as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_window_t>() as libc::c_ulong as libc::size_t,
    );
    (*window).base = b" \0" as *const u8 as *const libc::c_char as *const BYTE;
    (*window).dictBase = b" \0" as *const u8 as *const libc::c_char as *const BYTE;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as U32;
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as U32;
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0 as libc::c_int as U32;
}
#[inline]
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base) as libc::c_long as size_t;
    let end = endT as U32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
pub const MINMATCH: libc::c_int = 3 as libc::c_int;
pub const ZSTD_WORKSPACETOOLARGE_MAXDURATION: libc::c_int = 128 as libc::c_int;
pub const ZSTD_WORKSPACETOOLARGE_FACTOR: libc::c_int = 3 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_cpuSupportsBmi2() -> libc::c_int {
    let mut cpuid = ZSTD_cpuid();
    return (ZSTD_cpuid_bmi1(cpuid) != 0 && ZSTD_cpuid_bmi2(cpuid) != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cpuid() -> ZSTD_cpuid_t {
    let mut f1c = 0 as libc::c_int as U32;
    let mut f1d = 0 as libc::c_int as U32;
    let mut f7b = 0 as libc::c_int as U32;
    let mut f7c = 0 as libc::c_int as U32;
    let mut n: U32 = 0;
    asm!(
        "cpuid", inlateout("ax") 0 as libc::c_int => n, out("ecx")   _, out("edx")   _,
        options(preserves_flags, pure, readonly, att_syntax)
    );
    if n >= 1 as libc::c_int as libc::c_uint {
        let mut f1a: U32 = 0;
        asm!(
            "cpuid", inlateout("ax") 1 as libc::c_int => f1a, lateout("cx") f1c,
            lateout("dx") f1d, options(preserves_flags, pure, readonly, att_syntax)
        );
    }
    if n >= 7 as libc::c_int as libc::c_uint {
        let mut f7a: U32 = 0;
        asm!(
            "cpuid\nmov {restmp0:x}, %bx", restmp0 = lateout(reg) f7b, inlateout("ax") 7
            as libc::c_int => f7a, inlateout("cx") 0 as libc::c_int => f7c, out("edx")  
            _, options(preserves_flags, pure, readonly, att_syntax)
        );
    }
    let mut cpuid = ZSTD_cpuid_t {
        f1c: 0,
        f1d: 0,
        f7b: 0,
        f7c: 0,
    };
    cpuid.f1c = f1c;
    cpuid.f1d = f1d;
    cpuid.f7b = f7b;
    cpuid.f7c = f7c;
    return cpuid;
}
#[inline]
unsafe extern "C" fn ZSTD_cpuid_bmi2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & (1 as libc::c_uint) << 8 as libc::c_int
        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cpuid_bmi1(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & (1 as libc::c_uint) << 3 as libc::c_int
        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: libc::c_int = 10 as libc::c_int;
pub const HASH_READ_SIZE: libc::c_int = 8 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_update(
    mut window: *mut ZSTD_window_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut forceNonContiguous: libc::c_int,
) -> U32 {
    let ip = src as *const BYTE;
    let mut contiguous = 1 as libc::c_int as U32;
    if srcSize == 0 as libc::c_int as libc::c_ulong {
        return contiguous;
    }
    if !((*window).base).is_null() {} else {
        __assert_fail(
            b"window->base != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1258 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"U32 ZSTD_window_update(ZSTD_window_t *, const void *, size_t, int)\0"))
                .as_ptr(),
        );
    }
    if !((*window).dictBase).is_null() {} else {
        __assert_fail(
            b"window->dictBase != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1259 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"U32 ZSTD_window_update(ZSTD_window_t *, const void *, size_t, int)\0"))
                .as_ptr(),
        );
    }
    if src != (*window).nextSrc as *const libc::c_void || forceNonContiguous != 0 {
        let distanceFromBase = ((*window).nextSrc).offset_from((*window).base)
            as libc::c_long as size_t;
        (*window).lowLimit = (*window).dictLimit;
        if distanceFromBase == distanceFromBase as U32 as size_t {} else {
            __assert_fail(
                b"distanceFromBase == (size_t)(U32)distanceFromBase\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                1266 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"U32 ZSTD_window_update(ZSTD_window_t *, const void *, size_t, int)\0",
                ))
                    .as_ptr(),
            );
        }
        (*window).dictLimit = distanceFromBase as U32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit)
            < HASH_READ_SIZE as libc::c_uint
        {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = 0 as libc::c_int as U32;
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
            highInputIdx as U32
        };
        (*window).lowLimit = lowLimitMax;
    }
    return contiguous;
}
pub const ZSTDMT_JOBSIZE_MIN: libc::c_int = 512 as libc::c_int
    * ((1 as libc::c_int) << 10 as libc::c_int);
#[inline]
unsafe extern "C" fn ZSTD_window_canOverflowCorrect(
    window: ZSTD_window_t,
    mut cycleLog: U32,
    mut maxDist: U32,
    mut loadedDictEnd: U32,
    mut src: *const libc::c_void,
) -> U32 {
    let cycleSize = (1 as libc::c_uint) << cycleLog;
    let curr = (src as *const BYTE).offset_from(window.base) as libc::c_long as U32;
    let minIndexToOverflowCorrect = cycleSize
        .wrapping_add((if maxDist > cycleSize { maxDist } else { cycleSize }))
        .wrapping_add(ZSTD_WINDOW_START_INDEX as libc::c_uint);
    let adjustment = (window.nbOverflowCorrections)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let adjustedIndex = if minIndexToOverflowCorrect.wrapping_mul(adjustment)
        > minIndexToOverflowCorrect
    {
        minIndexToOverflowCorrect.wrapping_mul(adjustment)
    } else {
        minIndexToOverflowCorrect
    };
    let indexLargeEnough = (curr > adjustedIndex) as libc::c_int as U32;
    let dictionaryInvalidated = (curr > maxDist.wrapping_add(loadedDictEnd))
        as libc::c_int as U32;
    return (indexLargeEnough != 0 && dictionaryInvalidated != 0) as libc::c_int as U32;
}
#[inline]
unsafe extern "C" fn ZSTD_window_needOverflowCorrection(
    window: ZSTD_window_t,
    mut cycleLog: U32,
    mut maxDist: U32,
    mut loadedDictEnd: U32,
    mut src: *const libc::c_void,
    mut srcEnd: *const libc::c_void,
) -> U32 {
    let curr = (srcEnd as *const BYTE).offset_from(window.base) as libc::c_long as U32;
    return (curr
        > ((3 as libc::c_uint) << 29 as libc::c_int)
            .wrapping_add(
                (1 as libc::c_uint)
                    << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                        == 4 as libc::c_int as libc::c_ulong
                    {
                        ZSTD_WINDOWLOG_MAX_32
                    } else {
                        ZSTD_WINDOWLOG_MAX_64
                    }),
            )) as libc::c_int as U32;
}
#[inline]
unsafe extern "C" fn ZSTD_limitCopy(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let length = if dstCapacity < srcSize { dstCapacity } else { srcSize };
    if length > 0 as libc::c_int as libc::c_ulong {
        libc::memcpy(dst, src, length as libc::size_t);
    }
    return length;
}
pub const ZSTD_DUBT_UNSORTED_MARK: libc::c_int = 1 as libc::c_int;
pub const ZSTD_CWKSP_ALIGNMENT_BYTES: libc::c_int = 64 as libc::c_int;
pub const ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY: libc::c_int = 0 as libc::c_int;
pub const ZSTD_WINDOW_START_INDEX: libc::c_int = 2 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_correctOverflow(
    mut window: *mut ZSTD_window_t,
    mut cycleLog: U32,
    mut maxDist: U32,
    mut src: *const libc::c_void,
) -> U32 {
    let cycleSize = (1 as libc::c_uint) << cycleLog;
    let cycleMask = cycleSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let curr = (src as *const BYTE).offset_from((*window).base) as libc::c_long as U32;
    let currentCycle = curr & cycleMask;
    let currentCycleCorrection = if currentCycle
        < ZSTD_WINDOW_START_INDEX as libc::c_uint
    {
        if cycleSize > 2 as libc::c_int as libc::c_uint {
            cycleSize
        } else {
            2 as libc::c_int as libc::c_uint
        }
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let newCurrent = currentCycle
        .wrapping_add(currentCycleCorrection)
        .wrapping_add((if maxDist > cycleSize { maxDist } else { cycleSize }));
    let correction = curr.wrapping_sub(newCurrent);
    if maxDist & maxDist.wrapping_sub(1 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"(maxDist & (maxDist - 1)) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1094 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    if curr & cycleMask == newCurrent & cycleMask {} else {
        __assert_fail(
            b"(curr & cycleMask) == (newCurrent & cycleMask)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1095 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    if curr > newCurrent {} else {
        __assert_fail(
            b"curr > newCurrent\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1096 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    if ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY == 0 {
        if correction > ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint
        {} else {
            __assert_fail(
                b"correction > 1<<28\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                1099 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    (*window).base = ((*window).base).offset(correction as isize);
    (*window).dictBase = ((*window).dictBase).offset(correction as isize);
    if (*window).lowLimit
        < correction.wrapping_add(ZSTD_WINDOW_START_INDEX as libc::c_uint)
    {
        (*window).lowLimit = ZSTD_WINDOW_START_INDEX as U32;
    } else {
        (*window)
            .lowLimit = ((*window).lowLimit as libc::c_uint).wrapping_sub(correction)
            as U32 as U32;
    }
    if (*window).dictLimit
        < correction.wrapping_add(ZSTD_WINDOW_START_INDEX as libc::c_uint)
    {
        (*window).dictLimit = ZSTD_WINDOW_START_INDEX as U32;
    } else {
        (*window)
            .dictLimit = ((*window).dictLimit as libc::c_uint).wrapping_sub(correction)
            as U32 as U32;
    }
    if newCurrent >= maxDist {} else {
        __assert_fail(
            b"newCurrent >= maxDist\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1116 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    if newCurrent.wrapping_sub(maxDist) >= 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"newCurrent - maxDist >= ZSTD_WINDOW_START_INDEX\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1117 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*window).lowLimit <= newCurrent {} else {
        __assert_fail(
            b"window->lowLimit <= newCurrent\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1119 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*window).dictLimit <= newCurrent {} else {
        __assert_fail(
            b"window->dictLimit <= newCurrent\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1120 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"U32 ZSTD_window_correctOverflow(ZSTD_window_t *, U32, U32, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    (*window).nbOverflowCorrections = ((*window).nbOverflowCorrections).wrapping_add(1);
    return correction;
}
#[inline]
unsafe extern "C" fn ZSTD_checkDictValidity(
    mut window: *const ZSTD_window_t,
    mut blockEnd: *const libc::c_void,
    mut maxDist: U32,
    mut loadedDictEndPtr: *mut U32,
    mut dictMatchStatePtr: *mut *const ZSTD_matchState_t,
) {
    if !loadedDictEndPtr.is_null() {} else {
        __assert_fail(
            b"loadedDictEndPtr != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1204 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"void ZSTD_checkDictValidity(const ZSTD_window_t *, const void *, U32, U32 *, const ZSTD_matchState_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if !dictMatchStatePtr.is_null() {} else {
        __assert_fail(
            b"dictMatchStatePtr != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1205 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"void ZSTD_checkDictValidity(const ZSTD_window_t *, const void *, U32, U32 *, const ZSTD_matchState_t **)\0",
            ))
                .as_ptr(),
        );
    }
    let blockEndIdx = (blockEnd as *const BYTE).offset_from((*window).base)
        as libc::c_long as U32;
    let loadedDictEnd = *loadedDictEndPtr;
    if blockEndIdx >= loadedDictEnd {} else {
        __assert_fail(
            b"blockEndIdx >= loadedDictEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1210 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"void ZSTD_checkDictValidity(const ZSTD_window_t *, const void *, U32, U32 *, const ZSTD_matchState_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if blockEndIdx > loadedDictEnd.wrapping_add(maxDist)
        || loadedDictEnd != (*window).dictLimit
    {
        *loadedDictEndPtr = 0 as libc::c_int as U32;
        *dictMatchStatePtr = NULL as *const ZSTD_matchState_t;
    } else {
        *loadedDictEndPtr != 0 as libc::c_int as libc::c_uint;
    };
}
#[inline]
unsafe extern "C" fn ZSTD_window_enforceMaxDist(
    mut window: *mut ZSTD_window_t,
    mut blockEnd: *const libc::c_void,
    mut maxDist: U32,
    mut loadedDictEndPtr: *mut U32,
    mut dictMatchStatePtr: *mut *const ZSTD_matchState_t,
) {
    let blockEndIdx = (blockEnd as *const BYTE).offset_from((*window).base)
        as libc::c_long as U32;
    let loadedDictEnd = if !loadedDictEndPtr.is_null() {
        *loadedDictEndPtr
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit = blockEndIdx.wrapping_sub(maxDist);
        if (*window).lowLimit < newLowLimit {
            (*window).lowLimit = newLowLimit;
        }
        if (*window).dictLimit < (*window).lowLimit {
            (*window).dictLimit = (*window).lowLimit;
        }
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr = 0 as libc::c_int as U32;
        }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = NULL as *const ZSTD_matchState_t;
        }
    }
}
pub const ZSTD_MAX_NB_BLOCK_SPLITS: libc::c_int = 196 as libc::c_int;
pub const LitHufLog: libc::c_int = 11 as libc::c_int;
pub const ZSTD_OPT_NUM: libc::c_int = (1 as libc::c_int) << 12 as libc::c_int;
pub const Litbits: libc::c_int = 8 as libc::c_int;
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
#[inline]
unsafe extern "C" fn ZSTD_rleCompressBlock(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: BYTE,
    mut srcSize: size_t,
    mut lastBlock: U32,
) -> size_t {
    let op = dst as *mut BYTE;
    let cBlockHeader = lastBlock
        .wrapping_add((bt_rle as libc::c_int as U32) << 1 as libc::c_int)
        .wrapping_add((srcSize << 3 as libc::c_int) as U32);
    if dstCapacity < 4 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    MEM_writeLE24(op as *mut libc::c_void, cBlockHeader);
    *op.offset(3 as libc::c_int as isize) = src;
    return 4 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_count(
    mut pIn: *const BYTE,
    mut pMatch: *const BYTE,
    pInLimit: *const BYTE,
) -> size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit
        .offset(
            -((::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
        );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const libc::c_void)
            ^ MEM_readST(pIn as *const libc::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
        pMatch = pMatch
            .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const libc::c_void)
                ^ MEM_readST(pIn as *const libc::c_void);
            if diff_0 == 0 {
                pIn = pIn
                    .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
                pMatch = pMatch
                    .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as libc::c_long as size_t;
            }
        }
    }
    if MEM_64bits() != 0 && pIn < pInLimit.offset(-(3 as libc::c_int as isize))
        && MEM_read32(pMatch as *const libc::c_void)
            == MEM_read32(pIn as *const libc::c_void)
    {
        pIn = pIn.offset(4 as libc::c_int as isize);
        pMatch = pMatch.offset(4 as libc::c_int as isize);
    }
    if pIn < pInLimit.offset(-(1 as libc::c_int as isize))
        && MEM_read16(pMatch as *const libc::c_void) as libc::c_int
            == MEM_read16(pIn as *const libc::c_void) as libc::c_int
    {
        pIn = pIn.offset(2 as libc::c_int as isize);
        pMatch = pMatch.offset(2 as libc::c_int as isize);
    }
    if pIn < pInLimit && *pMatch as libc::c_int == *pIn as libc::c_int {
        pIn = pIn.offset(1);
    }
    return pIn.offset_from(pStart) as libc::c_long as size_t;
}
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
pub const MIN_CBLOCK_SIZE: libc::c_int = 1 as libc::c_int + 1 as libc::c_int;
pub const ZSTD_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
static mut ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
static mut kNullRawSeqStore: rawSeqStore_t = {
    let mut init = rawSeqStore_t {
        seq: NULL as *mut rawSeq,
        pos: 0 as libc::c_int as size_t,
        posInSequence: 0 as libc::c_int as size_t,
        size: 0 as libc::c_int as size_t,
        capacity: 0 as libc::c_int as size_t,
    };
    init
};
pub const MaxML: libc::c_int = 52 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_isEmpty(window: ZSTD_window_t) -> U32 {
    return (window.dictLimit == ZSTD_WINDOW_START_INDEX as libc::c_uint
        && window.lowLimit == ZSTD_WINDOW_START_INDEX as libc::c_uint
        && (window.nextSrc).offset_from(window.base) as libc::c_long
            == ZSTD_WINDOW_START_INDEX as libc::c_long) as libc::c_int as U32;
}
pub const ZSTD_SHORT_CACHE_TAG_BITS: libc::c_int = 8 as libc::c_int;
static mut repStartValue: [U32; 3] = [
    1 as libc::c_int as U32,
    4 as libc::c_int as U32,
    8 as libc::c_int as U32,
];
static mut ML_defaultNormLog: U32 = ML_DEFAULTNORMLOG as U32;
pub const WILDCOPY_VECLEN: libc::c_int = 16 as libc::c_int;
unsafe extern "C" fn ZSTD_copy8(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    libc::memcpy(dst, src, 8 as libc::c_int as libc::c_ulong as libc::size_t);
}
unsafe extern "C" fn ZSTD_copy16(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
#[inline(always)]
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut length: ptrdiff_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut BYTE).offset_from(src as *const BYTE) as libc::c_long;
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let oend = op.offset(length as isize);
    if ovtype as libc::c_uint
        == ZSTD_overlap_src_before_dst as libc::c_int as libc::c_uint
        && diff < WILDCOPY_VECLEN as libc::c_long
    {
        loop {
            ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(8 as libc::c_int as isize);
            ip = ip.offset(8 as libc::c_int as isize);
            if !(op < oend) {
                break;
            }
        }
    } else {
        if diff >= 16 as libc::c_int as libc::c_long
            || diff <= -(16 as libc::c_int) as libc::c_long
        {} else {
            __assert_fail(
                b"diff >= WILDCOPY_VECLEN || diff <= -WILDCOPY_VECLEN\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/zstd_internal.h\0"
                    as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void ZSTD_wildcopy(void *, const void *, ptrdiff_t, const ZSTD_overlap_e)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
        if 16 as libc::c_int as libc::c_long >= length {
            return;
        }
        op = op.offset(16 as libc::c_int as isize);
        ip = ip.offset(16 as libc::c_int as isize);
        loop {
            ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(16 as libc::c_int as isize);
            ip = ip.offset(16 as libc::c_int as isize);
            ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(16 as libc::c_int as isize);
            ip = ip.offset(16 as libc::c_int as isize);
            if !(op < oend) {
                break;
            }
        }
    };
}
unsafe extern "C" fn ZSTD_safecopyLiterals(
    mut op: *mut BYTE,
    mut ip: *const BYTE,
    iend: *const BYTE,
    mut ilimit_w: *const BYTE,
) {
    if iend > ilimit_w {} else {
        __assert_fail(
            b"iend > ilimit_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            627 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_safecopyLiterals(BYTE *, const BYTE *, const BYTE *const, const BYTE *)\0",
            ))
                .as_ptr(),
        );
    }
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            ip as *const libc::c_void,
            ilimit_w.offset_from(ip) as libc::c_long,
            ZSTD_no_overlap,
        );
        op = op.offset(ilimit_w.offset_from(ip) as libc::c_long as isize);
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
pub const WILDCOPY_OVERLENGTH: libc::c_int = 32 as libc::c_int;
#[inline(always)]
unsafe extern "C" fn ZSTD_storeSeq(
    mut seqStorePtr: *mut seqStore_t,
    mut litLength: size_t,
    mut literals: *const BYTE,
    mut litLimit: *const BYTE,
    mut offBase: U32,
    mut matchLength: size_t,
) {
    let litLimit_w = litLimit.offset(-(WILDCOPY_OVERLENGTH as isize));
    let litEnd = literals.offset(litLength as isize);
    if (((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as libc::c_long as size_t) < (*seqStorePtr).maxNbSeq
    {} else {
        __assert_fail(
            b"(size_t)(seqStorePtr->sequences - seqStorePtr->sequencesStart) < seqStorePtr->maxNbSeq\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            669 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*seqStorePtr).maxNbLit
        <= (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulong
    {} else {
        __assert_fail(
            b"seqStorePtr->maxNbLit <= 128 KB\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            671 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*seqStorePtr).lit).offset(litLength as isize)
        <= ((*seqStorePtr).litStart).offset((*seqStorePtr).maxNbLit as isize)
    {} else {
        __assert_fail(
            b"seqStorePtr->lit + litLength <= seqStorePtr->litStart + seqStorePtr->maxNbLit\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            672 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if literals.offset(litLength as isize) <= litLimit {} else {
        __assert_fail(
            b"literals + litLength <= litLimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            673 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if litEnd <= litLimit_w {
        ZSTD_copy16(
            (*seqStorePtr).lit as *mut libc::c_void,
            literals as *const libc::c_void,
        );
        if litLength > 16 as libc::c_int as libc::c_ulong {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16 as libc::c_int as isize)
                    as *mut libc::c_void,
                literals.offset(16 as libc::c_int as isize) as *const libc::c_void,
                litLength as ptrdiff_t - 16 as libc::c_int as libc::c_long,
                ZSTD_no_overlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(litLength as isize);
    if litLength > 0xffff as libc::c_int as libc::c_ulong {
        if (*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"seqStorePtr->longLengthType == ZSTD_llt_none\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                690 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as U32;
    }
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize))
        .litLength = litLength as U16;
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize)).offBase = offBase;
    if matchLength >= 3 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"matchLength >= MINMATCH\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            700 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mlBase = matchLength.wrapping_sub(MINMATCH as libc::c_ulong);
    if mlBase > 0xffff as libc::c_int as libc::c_ulong {
        if (*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"seqStorePtr->longLengthType == ZSTD_llt_none\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                703 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as U32;
    }
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize))
        .mlBase = mlBase as U16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
}
pub const ML_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
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
pub const MLFSELog: libc::c_int = 9 as libc::c_int;
pub const DefaultMaxOff: libc::c_int = 28 as libc::c_int;
static mut OF_defaultNormLog: U32 = OF_DEFAULTNORMLOG as U32;
pub const OF_DEFAULTNORMLOG: libc::c_int = 5 as libc::c_int;
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
pub const MaxOff: libc::c_int = 31 as libc::c_int;
pub const OffFSELog: libc::c_int = 8 as libc::c_int;
pub const MaxLL: libc::c_int = 35 as libc::c_int;
static mut LL_defaultNormLog: U32 = LL_DEFAULTNORMLOG as U32;
pub const LL_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
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
pub const LLFSELog: libc::c_int = 9 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: U32) -> U32 {
    static mut ML_Code: [BYTE; 128] = [
        0 as libc::c_int as BYTE,
        1 as libc::c_int as BYTE,
        2 as libc::c_int as BYTE,
        3 as libc::c_int as BYTE,
        4 as libc::c_int as BYTE,
        5 as libc::c_int as BYTE,
        6 as libc::c_int as BYTE,
        7 as libc::c_int as BYTE,
        8 as libc::c_int as BYTE,
        9 as libc::c_int as BYTE,
        10 as libc::c_int as BYTE,
        11 as libc::c_int as BYTE,
        12 as libc::c_int as BYTE,
        13 as libc::c_int as BYTE,
        14 as libc::c_int as BYTE,
        15 as libc::c_int as BYTE,
        16 as libc::c_int as BYTE,
        17 as libc::c_int as BYTE,
        18 as libc::c_int as BYTE,
        19 as libc::c_int as BYTE,
        20 as libc::c_int as BYTE,
        21 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        25 as libc::c_int as BYTE,
        26 as libc::c_int as BYTE,
        27 as libc::c_int as BYTE,
        28 as libc::c_int as BYTE,
        29 as libc::c_int as BYTE,
        30 as libc::c_int as BYTE,
        31 as libc::c_int as BYTE,
        32 as libc::c_int as BYTE,
        32 as libc::c_int as BYTE,
        33 as libc::c_int as BYTE,
        33 as libc::c_int as BYTE,
        34 as libc::c_int as BYTE,
        34 as libc::c_int as BYTE,
        35 as libc::c_int as BYTE,
        35 as libc::c_int as BYTE,
        36 as libc::c_int as BYTE,
        36 as libc::c_int as BYTE,
        36 as libc::c_int as BYTE,
        36 as libc::c_int as BYTE,
        37 as libc::c_int as BYTE,
        37 as libc::c_int as BYTE,
        37 as libc::c_int as BYTE,
        37 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        38 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        39 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        40 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        41 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
        42 as libc::c_int as BYTE,
    ];
    static mut ML_deltaCode: U32 = 36 as libc::c_int as U32;
    return if mlBase > 127 as libc::c_int as libc::c_uint {
        (ZSTD_highbit32(mlBase)).wrapping_add(ML_deltaCode)
    } else {
        ML_Code[mlBase as usize] as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn ZSTD_LLcode(mut litLength: U32) -> U32 {
    static mut LL_Code: [BYTE; 64] = [
        0 as libc::c_int as BYTE,
        1 as libc::c_int as BYTE,
        2 as libc::c_int as BYTE,
        3 as libc::c_int as BYTE,
        4 as libc::c_int as BYTE,
        5 as libc::c_int as BYTE,
        6 as libc::c_int as BYTE,
        7 as libc::c_int as BYTE,
        8 as libc::c_int as BYTE,
        9 as libc::c_int as BYTE,
        10 as libc::c_int as BYTE,
        11 as libc::c_int as BYTE,
        12 as libc::c_int as BYTE,
        13 as libc::c_int as BYTE,
        14 as libc::c_int as BYTE,
        15 as libc::c_int as BYTE,
        16 as libc::c_int as BYTE,
        16 as libc::c_int as BYTE,
        17 as libc::c_int as BYTE,
        17 as libc::c_int as BYTE,
        18 as libc::c_int as BYTE,
        18 as libc::c_int as BYTE,
        19 as libc::c_int as BYTE,
        19 as libc::c_int as BYTE,
        20 as libc::c_int as BYTE,
        20 as libc::c_int as BYTE,
        20 as libc::c_int as BYTE,
        20 as libc::c_int as BYTE,
        21 as libc::c_int as BYTE,
        21 as libc::c_int as BYTE,
        21 as libc::c_int as BYTE,
        21 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        22 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        23 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
        24 as libc::c_int as BYTE,
    ];
    static mut LL_deltaCode: U32 = 19 as libc::c_int as U32;
    return if litLength > 63 as libc::c_int as libc::c_uint {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        LL_Code[litLength as usize] as libc::c_uint
    };
}
pub const LONGNBSEQ: libc::c_int = 0x7f00 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_literalsCompressionIsDisabled(
    mut cctxParams: *const ZSTD_CCtx_params,
) -> libc::c_int {
    match (*cctxParams).literalCompressionMode as libc::c_uint {
        1 => return 0 as libc::c_int,
        2 => return 1 as libc::c_int,
        0 => {}
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                612 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"int ZSTD_literalsCompressionIsDisabled(const ZSTD_CCtx_params *)\0"))
                    .as_ptr(),
            );
        }
    }
    return ((*cctxParams).cParams.strategy as libc::c_uint
        == ZSTD_fast as libc::c_int as libc::c_uint
        && (*cctxParams).cParams.targetLength > 0 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_minGain(
    mut srcSize: size_t,
    mut strat: ZSTD_strategy,
) -> size_t {
    let minlog = if strat as libc::c_uint >= ZSTD_btultra as libc::c_int as libc::c_uint
    {
        (strat as U32).wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        6 as libc::c_int as libc::c_uint
    };
    if ZSTD_cParam_withinBounds(ZSTD_c_strategy, strat as libc::c_int) != 0 {} else {
        __assert_fail(
            b"ZSTD_cParam_withinBounds(ZSTD_c_strategy, (int)strat)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            600 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"size_t ZSTD_minGain(size_t, ZSTD_strategy)\0"))
                .as_ptr(),
        );
    }
    return (srcSize >> minlog).wrapping_add(2 as libc::c_int as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn ZSTD_cParam_withinBounds(
    mut cParam: ZSTD_cParameter,
    mut value: libc::c_int,
) -> libc::c_int {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) != 0 {
        return 0 as libc::c_int;
    }
    if value < bounds.lowerBound {
        return 0 as libc::c_int;
    }
    if value > bounds.upperBound {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
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
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> U32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as U32;
}
#[inline]
unsafe extern "C" fn ZSTD_matchState_dictMode(
    mut ms: *const ZSTD_matchState_t,
) -> ZSTD_dictMode_e {
    return (if ZSTD_window_hasExtDict((*ms).window) != 0 {
        ZSTD_extDict as libc::c_int
    } else if !((*ms).dictMatchState).is_null() {
        if (*(*ms).dictMatchState).dedicatedDictSearch != 0 {
            ZSTD_dedicatedDictSearch as libc::c_int
        } else {
            ZSTD_dictMatchState as libc::c_int
        }
    } else {
        ZSTD_noDict as libc::c_int
    }) as ZSTD_dictMode_e;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_move(
    mut dst: *mut ZSTD_cwksp,
    mut src: *mut ZSTD_cwksp,
) {
    *dst = *src;
    libc::memset(
        src as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_cwksp>() as libc::c_ulong as libc::size_t,
    );
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_buffer(
    mut ws: *mut ZSTD_cwksp,
    mut bytes: size_t,
) -> *mut BYTE {
    return ZSTD_cwksp_reserve_internal(ws, bytes, ZSTD_cwksp_alloc_buffers) as *mut BYTE;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_aligned(
    mut ws: *mut ZSTD_cwksp,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let mut ptr = ZSTD_cwksp_reserve_internal(
        ws,
        ZSTD_cwksp_align(bytes, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t),
        ZSTD_cwksp_alloc_aligned,
    );
    if ptr as size_t & (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"((size_t)ptr & (ZSTD_CWKSP_ALIGNMENT_BYTES-1))== 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            431 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void *ZSTD_cwksp_reserve_aligned(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_slack_space_required() -> size_t {
    let slackSpace = (ZSTD_CWKSP_ALIGNMENT_BYTES * 2 as libc::c_int) as size_t;
    return slackSpace;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_aligned_init_once(
    mut ws: *mut ZSTD_cwksp,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let alignedBytes = ZSTD_cwksp_align(bytes, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t);
    let mut ptr = ZSTD_cwksp_reserve_internal(
        ws,
        alignedBytes,
        ZSTD_cwksp_alloc_aligned_init_once,
    );
    if ptr as size_t & (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"((size_t)ptr & (ZSTD_CWKSP_ALIGNMENT_BYTES-1))== 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            407 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void *ZSTD_cwksp_reserve_aligned_init_once(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    if !ptr.is_null() && ptr < (*ws).initOnceStart {
        libc::memset(
            ptr,
            0 as libc::c_int,
            (if (((*ws).initOnceStart as *mut U8).offset_from(ptr as *mut U8)
                as libc::c_long as size_t) < alignedBytes
            {
                ((*ws).initOnceStart as *mut U8).offset_from(ptr as *mut U8)
                    as libc::c_long as size_t
            } else {
                alignedBytes
            }) as libc::size_t,
        );
        (*ws).initOnceStart = ptr;
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_clean_tables(mut ws: *mut ZSTD_cwksp) {
    if (*ws).tableValidEnd >= (*ws).objectEnd {} else {
        __assert_fail(
            b"ws->tableValidEnd >= ws->objectEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            568 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void ZSTD_cwksp_clean_tables(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableValidEnd <= (*ws).allocStart {} else {
        __assert_fail(
            b"ws->tableValidEnd <= ws->allocStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            569 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void ZSTD_cwksp_clean_tables(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableValidEnd < (*ws).tableEnd {
        libc::memset(
            (*ws).tableValidEnd,
            0 as libc::c_int,
            ((*ws).tableEnd as *mut BYTE).offset_from((*ws).tableValidEnd as *mut BYTE)
                as libc::c_long as size_t as libc::size_t,
        );
    }
    ZSTD_cwksp_mark_tables_clean(ws);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_table(
    mut ws: *mut ZSTD_cwksp,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let phase = ZSTD_cwksp_alloc_aligned_init_once;
    let mut alloc = 0 as *mut libc::c_void;
    let mut end = 0 as *mut libc::c_void;
    let mut top = 0 as *mut libc::c_void;
    if ((*ws).phase as libc::c_uint) < phase as libc::c_uint {
        if ERR_isError(ZSTD_cwksp_internal_advance_phase(ws, phase)) != 0 {
            return NULL as *mut libc::c_void;
        }
    }
    alloc = (*ws).tableEnd;
    end = (alloc as *mut BYTE).offset(bytes as isize) as *mut libc::c_void;
    top = (*ws).allocStart;
    if bytes
        & (::core::mem::size_of::<U32>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(bytes & (sizeof(U32)-1)) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            460 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *ZSTD_cwksp_reserve_table(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
    if end <= top {} else {
        __assert_fail(
            b"end <= top\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            462 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *ZSTD_cwksp_reserve_table(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    if end > top {
        (*ws).allocFailed = 1 as libc::c_int as BYTE;
        return NULL as *mut libc::c_void;
    }
    (*ws).tableEnd = end;
    if bytes & (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(bytes & (ZSTD_CWKSP_ALIGNMENT_BYTES-1)) == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            476 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *ZSTD_cwksp_reserve_table(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    if alloc as size_t & (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"((size_t)alloc & (ZSTD_CWKSP_ALIGNMENT_BYTES-1))== 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            477 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *ZSTD_cwksp_reserve_table(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    return alloc;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_clear_tables(mut ws: *mut ZSTD_cwksp) {
    (*ws).tableEnd = (*ws).objectEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_clear(mut ws: *mut ZSTD_cwksp) {
    (*ws).tableEnd = (*ws).objectEnd;
    (*ws).allocStart = ZSTD_cwksp_initialAllocStart(ws);
    (*ws).allocFailed = 0 as libc::c_int as BYTE;
    if (*ws).phase as libc::c_uint
        > ZSTD_cwksp_alloc_aligned_init_once as libc::c_int as libc::c_uint
    {
        (*ws).phase = ZSTD_cwksp_alloc_aligned_init_once;
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_object(
    mut ws: *mut ZSTD_cwksp,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let roundedBytes = ZSTD_cwksp_align(
        bytes,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    let mut alloc = (*ws).objectEnd;
    let mut end = (alloc as *mut BYTE).offset(roundedBytes as isize)
        as *mut libc::c_void;
    if (alloc as size_t)
        .wrapping_rem(::core::mem::align_of::<*mut libc::c_void>() as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(size_t)alloc % ZSTD_ALIGNOF(void*) == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            499 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void *ZSTD_cwksp_reserve_object(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    if bytes.wrapping_rem(::core::mem::align_of::<*mut libc::c_void>() as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"bytes % ZSTD_ALIGNOF(void*) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            500 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void *ZSTD_cwksp_reserve_object(ZSTD_cwksp *, size_t)\0"))
                .as_ptr(),
        );
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
    if (*ws).phase as libc::c_uint
        != ZSTD_cwksp_alloc_objects as libc::c_int as libc::c_uint
        || end > (*ws).workspaceEnd
    {
        (*ws).allocFailed = 1 as libc::c_int as BYTE;
        return NULL as *mut libc::c_void;
    }
    (*ws).objectEnd = end;
    (*ws).tableEnd = end;
    (*ws).tableValidEnd = end;
    return alloc;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_owns_buffer(
    mut ws: *const ZSTD_cwksp,
    mut ptr: *const libc::c_void,
) -> libc::c_int {
    return (!ptr.is_null() && (*ws).workspace <= ptr as *mut libc::c_void
        && ptr < (*ws).workspaceEnd as *const libc::c_void) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_alloc_size(mut size: size_t) -> size_t {
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    return size;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_align(mut size: size_t, align: size_t) -> size_t {
    let mask = align.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if align & mask == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"(align & mask) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            211 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"size_t ZSTD_cwksp_align(size_t, const size_t)\0"))
                .as_ptr(),
        );
    }
    return size.wrapping_add(mask) & !mask;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_create(
    mut ws: *mut ZSTD_cwksp,
    mut size: size_t,
    mut customMem: ZSTD_customMem,
) -> size_t {
    let mut workspace = ZSTD_customMalloc(size, customMem);
    if workspace.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    ZSTD_cwksp_init(ws, workspace, size, ZSTD_cwksp_dynamic_alloc);
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_init(
    mut ws: *mut ZSTD_cwksp,
    mut start: *mut libc::c_void,
    mut size: size_t,
    mut isStatic: ZSTD_cwksp_static_alloc_e,
) {
    if start as size_t
        & (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"((size_t)start & (sizeof(void*)-1)) == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            646 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void ZSTD_cwksp_init(ZSTD_cwksp *, void *, size_t, ZSTD_cwksp_static_alloc_e)\0",
            ))
                .as_ptr(),
        );
    }
    (*ws).workspace = start;
    (*ws).workspaceEnd = (start as *mut BYTE).offset(size as isize) as *mut libc::c_void;
    (*ws).objectEnd = (*ws).workspace;
    (*ws).tableValidEnd = (*ws).objectEnd;
    (*ws).initOnceStart = ZSTD_cwksp_initialAllocStart(ws);
    (*ws).phase = ZSTD_cwksp_alloc_objects;
    (*ws).isStatic = isStatic;
    ZSTD_cwksp_clear(ws);
    (*ws).workspaceOversizedDuration = 0 as libc::c_int;
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_aligned_alloc_size(mut size: size_t) -> size_t {
    return ZSTD_cwksp_alloc_size(
        ZSTD_cwksp_align(size, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t),
    );
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_internal(
    mut ws: *mut ZSTD_cwksp,
    mut bytes: size_t,
    mut phase: ZSTD_cwksp_alloc_phase_e,
) -> *mut libc::c_void {
    let mut alloc = 0 as *mut libc::c_void;
    if ERR_isError(ZSTD_cwksp_internal_advance_phase(ws, phase)) != 0
        || bytes == 0 as libc::c_int as libc::c_ulong
    {
        return NULL as *mut libc::c_void;
    }
    alloc = ZSTD_cwksp_reserve_internal_buffer_space(ws, bytes);
    return alloc;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_free(
    mut ws: *mut ZSTD_cwksp,
    mut customMem: ZSTD_customMem,
) {
    let mut ptr = (*ws).workspace;
    libc::memset(
        ws as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_cwksp>() as libc::c_ulong as libc::size_t,
    );
    ZSTD_customFree(ptr, customMem);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_internal_buffer_space(
    mut ws: *mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut libc::c_void {
    let alloc = ((*ws).allocStart as *mut BYTE).offset(-(bytes as isize))
        as *mut libc::c_void;
    let bottom = (*ws).tableEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
    if alloc >= bottom {} else {
        __assert_fail(
            b"alloc >= bottom\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void *ZSTD_cwksp_reserve_internal_buffer_space(ZSTD_cwksp *, const size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if alloc < bottom {
        (*ws).allocFailed = 1 as libc::c_int as BYTE;
        return NULL as *mut libc::c_void;
    }
    if alloc < (*ws).tableValidEnd {
        (*ws).tableValidEnd = alloc;
    }
    (*ws).allocStart = alloc;
    return alloc;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_mark_tables_clean(mut ws: *mut ZSTD_cwksp) {
    if (*ws).tableValidEnd >= (*ws).objectEnd {} else {
        __assert_fail(
            b"ws->tableValidEnd >= ws->objectEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            555 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ZSTD_cwksp_mark_tables_clean(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableValidEnd <= (*ws).allocStart {} else {
        __assert_fail(
            b"ws->tableValidEnd <= ws->allocStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            556 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ZSTD_cwksp_mark_tables_clean(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableValidEnd < (*ws).tableEnd {
        (*ws).tableValidEnd = (*ws).tableEnd;
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_assert_internal_consistency(mut ws: *mut ZSTD_cwksp) {
    if (*ws).workspace <= (*ws).objectEnd {} else {
        __assert_fail(
            b"ws->workspace <= ws->objectEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).objectEnd <= (*ws).tableEnd {} else {
        __assert_fail(
            b"ws->objectEnd <= ws->tableEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).objectEnd <= (*ws).tableValidEnd {} else {
        __assert_fail(
            b"ws->objectEnd <= ws->tableValidEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            185 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableEnd <= (*ws).allocStart {} else {
        __assert_fail(
            b"ws->tableEnd <= ws->allocStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableValidEnd <= (*ws).allocStart {} else {
        __assert_fail(
            b"ws->tableValidEnd <= ws->allocStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            187 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).allocStart <= (*ws).workspaceEnd {} else {
        __assert_fail(
            b"ws->allocStart <= ws->workspaceEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            188 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).initOnceStart <= ZSTD_cwksp_initialAllocStart(ws) {} else {
        __assert_fail(
            b"ws->initOnceStart <= ZSTD_cwksp_initialAllocStart(ws)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).workspace <= (*ws).initOnceStart {} else {
        __assert_fail(
            b"ws->workspace <= ws->initOnceStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            190 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void ZSTD_cwksp_assert_internal_consistency(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    };
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_bytes_to_align_ptr(
    mut ptr: *mut libc::c_void,
    alignBytes: size_t,
) -> size_t {
    let alignBytesMask = alignBytes.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let bytes = alignBytes.wrapping_sub(ptr as size_t & alignBytesMask) & alignBytesMask;
    if alignBytes & alignBytesMask == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"(alignBytes & alignBytesMask) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            265 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"size_t ZSTD_cwksp_bytes_to_align_ptr(void *, const size_t)\0"))
                .as_ptr(),
        );
    }
    if bytes < alignBytes {} else {
        __assert_fail(
            b"bytes < alignBytes\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            266 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"size_t ZSTD_cwksp_bytes_to_align_ptr(void *, const size_t)\0"))
                .as_ptr(),
        );
    }
    return bytes;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_internal_advance_phase(
    mut ws: *mut ZSTD_cwksp,
    mut phase: ZSTD_cwksp_alloc_phase_e,
) -> size_t {
    if phase as libc::c_uint >= (*ws).phase as libc::c_uint {} else {
        __assert_fail(
            b"phase >= ws->phase\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            316 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"size_t ZSTD_cwksp_internal_advance_phase(ZSTD_cwksp *, ZSTD_cwksp_alloc_phase_e)\0",
            ))
                .as_ptr(),
        );
    }
    if phase as libc::c_uint > (*ws).phase as libc::c_uint {
        if ((*ws).phase as libc::c_uint)
            < ZSTD_cwksp_alloc_aligned_init_once as libc::c_int as libc::c_uint
            && phase as libc::c_uint
                >= ZSTD_cwksp_alloc_aligned_init_once as libc::c_int as libc::c_uint
        {
            (*ws).tableValidEnd = (*ws).objectEnd;
            (*ws).initOnceStart = ZSTD_cwksp_initialAllocStart(ws);
            let alloc = (*ws).objectEnd;
            let bytesToAlign = ZSTD_cwksp_bytes_to_align_ptr(
                alloc,
                ZSTD_CWKSP_ALIGNMENT_BYTES as size_t,
            );
            let objectEnd = (alloc as *mut BYTE).offset(bytesToAlign as isize)
                as *mut libc::c_void;
            if objectEnd > (*ws).workspaceEnd {
                return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
            }
            (*ws).objectEnd = objectEnd;
            (*ws).tableEnd = objectEnd;
            if (*ws).tableValidEnd < (*ws).tableEnd {
                (*ws).tableValidEnd = (*ws).tableEnd;
            }
        }
        (*ws).phase = phase;
        ZSTD_cwksp_assert_internal_consistency(ws);
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_mark_tables_dirty(mut ws: *mut ZSTD_cwksp) {
    if (*ws).tableValidEnd >= (*ws).objectEnd {} else {
        __assert_fail(
            b"ws->tableValidEnd >= ws->objectEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            547 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ZSTD_cwksp_mark_tables_dirty(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    if (*ws).tableValidEnd <= (*ws).allocStart {} else {
        __assert_fail(
            b"ws->tableValidEnd <= ws->allocStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_cwksp.h\0" as *const u8
                as *const libc::c_char,
            548 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ZSTD_cwksp_mark_tables_dirty(ZSTD_cwksp *)\0"))
                .as_ptr(),
        );
    }
    (*ws).tableValidEnd = (*ws).objectEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_initialAllocStart(
    mut ws: *mut ZSTD_cwksp,
) -> *mut libc::c_void {
    return ((*ws).workspaceEnd as size_t
        & !(ZSTD_CWKSP_ALIGNMENT_BYTES - 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_check_too_large(
    mut ws: *mut ZSTD_cwksp,
    mut additionalNeededSpace: size_t,
) -> libc::c_int {
    return ZSTD_cwksp_check_available(
        ws,
        additionalNeededSpace
            .wrapping_mul(ZSTD_WORKSPACETOOLARGE_FACTOR as libc::c_ulong),
    );
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_used(mut ws: *const ZSTD_cwksp) -> size_t {
    return (((*ws).tableEnd as *mut BYTE).offset_from((*ws).workspace as *mut BYTE)
        as libc::c_long as size_t)
        .wrapping_add(
            ((*ws).workspaceEnd as *mut BYTE).offset_from((*ws).allocStart as *mut BYTE)
                as libc::c_long as size_t,
        );
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_estimated_space_within_bounds(
    ws: *const ZSTD_cwksp,
    estimatedSpace: size_t,
) -> libc::c_int {
    return (estimatedSpace.wrapping_sub(ZSTD_cwksp_slack_space_required())
        <= ZSTD_cwksp_used(ws) && ZSTD_cwksp_used(ws) <= estimatedSpace) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_reserve_failed(
    mut ws: *const ZSTD_cwksp,
) -> libc::c_int {
    return (*ws).allocFailed as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_check_available(
    mut ws: *mut ZSTD_cwksp,
    mut additionalNeededSpace: size_t,
) -> libc::c_int {
    return (ZSTD_cwksp_available_space(ws) >= additionalNeededSpace) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_sizeof(mut ws: *const ZSTD_cwksp) -> size_t {
    return ((*ws).workspaceEnd as *mut BYTE).offset_from((*ws).workspace as *mut BYTE)
        as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_available_space(mut ws: *mut ZSTD_cwksp) -> size_t {
    return ((*ws).allocStart as *mut BYTE).offset_from((*ws).tableEnd as *mut BYTE)
        as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_check_wasteful(
    mut ws: *mut ZSTD_cwksp,
    mut additionalNeededSpace: size_t,
) -> libc::c_int {
    return (ZSTD_cwksp_check_too_large(ws, additionalNeededSpace) != 0
        && (*ws).workspaceOversizedDuration > ZSTD_WORKSPACETOOLARGE_MAXDURATION)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_cwksp_bump_oversized_duration(
    mut ws: *mut ZSTD_cwksp,
    mut additionalNeededSpace: size_t,
) {
    if ZSTD_cwksp_check_too_large(ws, additionalNeededSpace) != 0 {
        (*ws).workspaceOversizedDuration += 1;
    } else {
        (*ws).workspaceOversizedDuration = 0 as libc::c_int;
    };
}
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: libc::c_int = 2 as libc::c_int;
pub const ZSTD_ROW_HASH_TAG_BITS: libc::c_int = 8 as libc::c_int;
pub const ZSTD_LDM_DEFAULT_WINDOW_LOG: libc::c_int = 27 as libc::c_int;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBound(mut srcSize: size_t) -> size_t {
    let r = if srcSize as libc::c_ulonglong
        >= (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
        {
            0xff00ff00ff00ff00 as libc::c_ulonglong
        } else {
            0xff00ff00 as libc::c_uint as libc::c_ulonglong
        })
    {
        0 as libc::c_int as libc::c_ulong
    } else {
        srcSize
            .wrapping_add(srcSize >> 8 as libc::c_int)
            .wrapping_add(
                (if srcSize
                    < ((128 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong
                {
                    (((128 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong)
                        .wrapping_sub(srcSize) >> 11 as libc::c_int
                } else {
                    0 as libc::c_int as libc::c_ulong
                }),
            )
    };
    if r == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCCtx() -> *mut ZSTD_CCtx {
    return ZSTD_createCCtx_advanced(ZSTD_defaultCMem);
}
unsafe extern "C" fn ZSTD_initCCtx(
    mut cctx: *mut ZSTD_CCtx,
    mut memManager: ZSTD_customMem,
) {
    if !cctx.is_null() {} else {
        __assert_fail(
            b"cctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ZSTD_initCCtx(ZSTD_CCtx *, ZSTD_customMem)\0"))
                .as_ptr(),
        );
    }
    libc::memset(
        cctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong as libc::size_t,
    );
    (*cctx).customMem = memManager;
    (*cctx).bmi2 = ZSTD_cpuSupportsBmi2();
    let err = ZSTD_CCtx_reset(cctx, ZSTD_reset_parameters);
    if ERR_isError(err) == 0 {} else {
        __assert_fail(
            b"!ZSTD_isError(err)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ZSTD_initCCtx(ZSTD_CCtx *, ZSTD_customMem)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCCtx_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CCtx {
    if (customMem.customAlloc).is_none() as libc::c_int
        ^ (customMem.customFree).is_none() as libc::c_int != 0
    {
        return NULL as *mut ZSTD_CCtx;
    }
    let cctx = ZSTD_customMalloc(
        ::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong,
        customMem,
    ) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return NULL as *mut ZSTD_CCtx;
    }
    ZSTD_initCCtx(cctx, customMem);
    return cctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCCtx(
    mut workspace: *mut libc::c_void,
    mut workspaceSize: size_t,
) -> *mut ZSTD_CCtx {
    let mut ws = ZSTD_cwksp {
        workspace: 0 as *mut libc::c_void,
        workspaceEnd: 0 as *mut libc::c_void,
        objectEnd: 0 as *mut libc::c_void,
        tableEnd: 0 as *mut libc::c_void,
        tableValidEnd: 0 as *mut libc::c_void,
        allocStart: 0 as *mut libc::c_void,
        initOnceStart: 0 as *mut libc::c_void,
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    let mut cctx = 0 as *mut ZSTD_CCtx;
    if workspaceSize <= ::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong {
        return NULL as *mut ZSTD_CCtx;
    }
    if workspace as size_t & 7 as libc::c_int as libc::c_ulong != 0 {
        return NULL as *mut ZSTD_CCtx;
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_static_alloc);
    cctx = ZSTD_cwksp_reserve_object(
        &mut ws,
        ::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong,
    ) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return NULL as *mut ZSTD_CCtx;
    }
    libc::memset(
        cctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong as libc::size_t,
    );
    ZSTD_cwksp_move(&mut (*cctx).workspace, &mut ws);
    (*cctx).staticSize = workspaceSize;
    if ZSTD_cwksp_check_available(
        &mut (*cctx).workspace,
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
            )
            .wrapping_add(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<ZSTD_compressedBlockState_t>()
                            as libc::c_ulong,
                    ),
            ),
    ) == 0
    {
        return NULL as *mut ZSTD_CCtx;
    }
    (*cctx)
        .blockState
        .prevCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong,
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx)
        .blockState
        .nextCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong,
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx)
        .entropyWorkspace = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
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
    ) as *mut U32;
    (*cctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
    return cctx;
}
unsafe extern "C" fn ZSTD_clearAllDicts(mut cctx: *mut ZSTD_CCtx) {
    ZSTD_customFree((*cctx).localDict.dictBuffer, (*cctx).customMem);
    ZSTD_freeCDict((*cctx).localDict.cdict);
    libc::memset(
        &mut (*cctx).localDict as *mut ZSTD_localDict as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_localDict>() as libc::c_ulong as libc::size_t,
    );
    libc::memset(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_prefixDict>() as libc::c_ulong as libc::size_t,
    );
    (*cctx).cdict = NULL as *const ZSTD_CDict;
}
unsafe extern "C" fn ZSTD_sizeof_localDict(mut dict: ZSTD_localDict) -> size_t {
    let bufferSize = if !(dict.dictBuffer).is_null() {
        dict.dictSize
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let cdictSize = ZSTD_sizeof_CDict(dict.cdict);
    return bufferSize.wrapping_add(cdictSize);
}
unsafe extern "C" fn ZSTD_freeCCtxContent(mut cctx: *mut ZSTD_CCtx) {
    if !cctx.is_null() {} else {
        __assert_fail(
            b"cctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void ZSTD_freeCCtxContent(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    if (*cctx).staticSize == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cctx->staticSize == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void ZSTD_freeCCtxContent(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    ZSTD_clearAllDicts(cctx);
    ZSTDMT_freeCCtx((*cctx).mtctx);
    (*cctx).mtctx = NULL as *mut ZSTDMT_CCtx;
    ZSTD_cwksp_free(&mut (*cctx).workspace, (*cctx).customMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCCtx(mut cctx: *mut ZSTD_CCtx) -> size_t {
    if cctx.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if (*cctx).staticSize != 0 {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    let mut cctxInWorkspace = ZSTD_cwksp_owns_buffer(
        &mut (*cctx).workspace,
        cctx as *const libc::c_void,
    );
    ZSTD_freeCCtxContent(cctx);
    if cctxInWorkspace == 0 {
        ZSTD_customFree(cctx as *mut libc::c_void, (*cctx).customMem);
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_sizeof_mtctx(mut cctx: *const ZSTD_CCtx) -> size_t {
    return ZSTDMT_sizeof_CCtx((*cctx).mtctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CCtx(mut cctx: *const ZSTD_CCtx) -> size_t {
    if cctx.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return (if (*cctx).workspace.workspace == cctx as *mut libc::c_void {
        0 as libc::c_int as libc::c_ulong
    } else {
        ::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong
    })
        .wrapping_add(ZSTD_cwksp_sizeof(&(*cctx).workspace))
        .wrapping_add(ZSTD_sizeof_localDict((*cctx).localDict))
        .wrapping_add(ZSTD_sizeof_mtctx(cctx));
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CStream(mut zcs: *const ZSTD_CStream) -> size_t {
    return ZSTD_sizeof_CCtx(zcs);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getSeqStore(
    mut ctx: *const ZSTD_CCtx,
) -> *const seqStore_t {
    return &(*ctx).seqStore;
}
unsafe extern "C" fn ZSTD_rowMatchFinderSupported(
    strategy: ZSTD_strategy,
) -> libc::c_int {
    return (strategy as libc::c_uint >= ZSTD_greedy as libc::c_int as libc::c_uint
        && strategy as libc::c_uint <= ZSTD_lazy2 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn ZSTD_rowMatchFinderUsed(
    strategy: ZSTD_strategy,
    mode: ZSTD_paramSwitch_e,
) -> libc::c_int {
    if mode as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"mode != ZSTD_ps_auto\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ZSTD_rowMatchFinderUsed(const ZSTD_strategy, const ZSTD_paramSwitch_e)\0",
            ))
                .as_ptr(),
        );
    }
    return (ZSTD_rowMatchFinderSupported(strategy) != 0
        && mode as libc::c_uint == ZSTD_ps_enable as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn ZSTD_resolveRowMatchFinderMode(
    mut mode: ZSTD_paramSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_paramSwitch_e {
    let kHasSIMD128 = 1 as libc::c_int;
    if mode as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint {
        return mode;
    }
    mode = ZSTD_ps_disable;
    if ZSTD_rowMatchFinderSupported((*cParams).strategy) == 0 {
        return mode;
    }
    if kHasSIMD128 != 0 {
        if (*cParams).windowLog > 14 as libc::c_int as libc::c_uint {
            mode = ZSTD_ps_enable;
        }
    } else if (*cParams).windowLog > 17 as libc::c_int as libc::c_uint {
        mode = ZSTD_ps_enable;
    }
    return mode;
}
unsafe extern "C" fn ZSTD_resolveBlockSplitterMode(
    mut mode: ZSTD_paramSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_paramSwitch_e {
    if mode as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint {
        return mode;
    }
    return (if (*cParams).strategy as libc::c_uint
        >= ZSTD_btopt as libc::c_int as libc::c_uint
        && (*cParams).windowLog >= 17 as libc::c_int as libc::c_uint
    {
        ZSTD_ps_enable as libc::c_int
    } else {
        ZSTD_ps_disable as libc::c_int
    }) as ZSTD_paramSwitch_e;
}
unsafe extern "C" fn ZSTD_allocateChainTable(
    strategy: ZSTD_strategy,
    useRowMatchFinder: ZSTD_paramSwitch_e,
    forDDSDict: U32,
) -> libc::c_int {
    if useRowMatchFinder as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"useRowMatchFinder != ZSTD_ps_auto\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            264 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"int ZSTD_allocateChainTable(const ZSTD_strategy, const ZSTD_paramSwitch_e, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    return (forDDSDict != 0
        || strategy as libc::c_uint != ZSTD_fast as libc::c_int as libc::c_uint
            && ZSTD_rowMatchFinderUsed(strategy, useRowMatchFinder) == 0) as libc::c_int;
}
unsafe extern "C" fn ZSTD_resolveEnableLdm(
    mut mode: ZSTD_paramSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_paramSwitch_e {
    if mode as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint {
        return mode;
    }
    return (if (*cParams).strategy as libc::c_uint
        >= ZSTD_btopt as libc::c_int as libc::c_uint
        && (*cParams).windowLog >= 27 as libc::c_int as libc::c_uint
    {
        ZSTD_ps_enable as libc::c_int
    } else {
        ZSTD_ps_disable as libc::c_int
    }) as ZSTD_paramSwitch_e;
}
unsafe extern "C" fn ZSTD_resolveExternalSequenceValidation(
    mut mode: libc::c_int,
) -> libc::c_int {
    return mode;
}
unsafe extern "C" fn ZSTD_resolveMaxBlockSize(mut maxBlockSize: size_t) -> size_t {
    if maxBlockSize == 0 as libc::c_int as libc::c_ulong {
        return ZSTD_BLOCKSIZE_MAX as size_t
    } else {
        return maxBlockSize
    };
}
unsafe extern "C" fn ZSTD_resolveExternalRepcodeSearch(
    mut value: ZSTD_paramSwitch_e,
    mut cLevel: libc::c_int,
) -> ZSTD_paramSwitch_e {
    if value as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint {
        return value;
    }
    if cLevel < 10 as libc::c_int {
        return ZSTD_ps_disable
    } else {
        return ZSTD_ps_enable
    };
}
unsafe extern "C" fn ZSTD_CDictIndicesAreTagged(
    cParams: *const ZSTD_compressionParameters,
) -> libc::c_int {
    return ((*cParams).strategy as libc::c_uint
        == ZSTD_fast as libc::c_int as libc::c_uint
        || (*cParams).strategy as libc::c_uint
            == ZSTD_dfast as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_makeCCtxParamsFromCParams(
    mut cParams: ZSTD_compressionParameters,
) -> ZSTD_CCtx_params {
    let mut cctxParams = ZSTD_CCtx_params {
        format: ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        useBlockSplitter: ZSTD_ps_auto,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        useSequenceProducer: 0,
        maxBlockSize: 0,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    ZSTD_CCtxParams_init(&mut cctxParams, ZSTD_CLEVEL_DEFAULT);
    cctxParams.cParams = cParams;
    cctxParams
        .ldmParams
        .enableLdm = ZSTD_resolveEnableLdm(cctxParams.ldmParams.enableLdm, &mut cParams);
    if cctxParams.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        ZSTD_ldm_adjustParameters(&mut cctxParams.ldmParams, &mut cParams);
        if cctxParams.ldmParams.hashLog >= cctxParams.ldmParams.bucketSizeLog {} else {
            __assert_fail(
                b"cctxParams.ldmParams.hashLog >= cctxParams.ldmParams.bucketSizeLog\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                321 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"ZSTD_CCtx_params ZSTD_makeCCtxParamsFromCParams(ZSTD_compressionParameters)\0",
                ))
                    .as_ptr(),
            );
        }
        if cctxParams.ldmParams.hashRateLog < 32 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"cctxParams.ldmParams.hashRateLog < 32\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                322 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"ZSTD_CCtx_params ZSTD_makeCCtxParamsFromCParams(ZSTD_compressionParameters)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    cctxParams
        .useBlockSplitter = ZSTD_resolveBlockSplitterMode(
        cctxParams.useBlockSplitter,
        &mut cParams,
    );
    cctxParams
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        cctxParams.useRowMatchFinder,
        &mut cParams,
    );
    cctxParams
        .validateSequences = ZSTD_resolveExternalSequenceValidation(
        cctxParams.validateSequences,
    );
    cctxParams.maxBlockSize = ZSTD_resolveMaxBlockSize(cctxParams.maxBlockSize);
    cctxParams
        .searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        cctxParams.searchForExternalRepcodes,
        cctxParams.compressionLevel,
    );
    if ZSTD_checkCParams(cParams) == 0 {} else {
        __assert_fail(
            b"!ZSTD_checkCParams(cParams)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"ZSTD_CCtx_params ZSTD_makeCCtxParamsFromCParams(ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    return cctxParams;
}
unsafe extern "C" fn ZSTD_createCCtxParams_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CCtx_params {
    let mut params = 0 as *mut ZSTD_CCtx_params;
    if (customMem.customAlloc).is_none() as libc::c_int
        ^ (customMem.customFree).is_none() as libc::c_int != 0
    {
        return NULL as *mut ZSTD_CCtx_params;
    }
    params = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong,
        customMem,
    ) as *mut ZSTD_CCtx_params;
    if params.is_null() {
        return NULL as *mut ZSTD_CCtx_params;
    }
    ZSTD_CCtxParams_init(params, ZSTD_CLEVEL_DEFAULT);
    (*params).customMem = customMem;
    return params;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCCtxParams() -> *mut ZSTD_CCtx_params {
    return ZSTD_createCCtxParams_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCCtxParams(
    mut params: *mut ZSTD_CCtx_params,
) -> size_t {
    if params.is_null() {
        return 0 as libc::c_int as size_t;
    }
    ZSTD_customFree(params as *mut libc::c_void, (*params).customMem);
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_reset(
    mut params: *mut ZSTD_CCtx_params,
) -> size_t {
    return ZSTD_CCtxParams_init(params, ZSTD_CLEVEL_DEFAULT);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_init(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut compressionLevel: libc::c_int,
) -> size_t {
    if cctxParams.is_null() {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    libc::memset(
        cctxParams as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong as libc::size_t,
    );
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).fParams.contentSizeFlag = 1 as libc::c_int;
    return 0 as libc::c_int as size_t;
}
pub const ZSTD_NO_CLEVEL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn ZSTD_CCtxParams_init_internal(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut params: *const ZSTD_parameters,
    mut compressionLevel: libc::c_int,
) {
    if ZSTD_checkCParams((*params).cParams) == 0 {} else {
        __assert_fail(
            b"!ZSTD_checkCParams(params->cParams)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            383 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_CCtxParams_init_internal(ZSTD_CCtx_params *, const ZSTD_parameters *, int)\0",
            ))
                .as_ptr(),
        );
    }
    libc::memset(
        cctxParams as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong as libc::size_t,
    );
    (*cctxParams).cParams = (*params).cParams;
    (*cctxParams).fParams = (*params).fParams;
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams)
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        (*cctxParams).useRowMatchFinder,
        &(*params).cParams,
    );
    (*cctxParams)
        .useBlockSplitter = ZSTD_resolveBlockSplitterMode(
        (*cctxParams).useBlockSplitter,
        &(*params).cParams,
    );
    (*cctxParams)
        .ldmParams
        .enableLdm = ZSTD_resolveEnableLdm(
        (*cctxParams).ldmParams.enableLdm,
        &(*params).cParams,
    );
    (*cctxParams)
        .validateSequences = ZSTD_resolveExternalSequenceValidation(
        (*cctxParams).validateSequences,
    );
    (*cctxParams).maxBlockSize = ZSTD_resolveMaxBlockSize((*cctxParams).maxBlockSize);
    (*cctxParams)
        .searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        (*cctxParams).searchForExternalRepcodes,
        compressionLevel,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_init_advanced(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut params: ZSTD_parameters,
) -> size_t {
    if cctxParams.is_null() {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    let err_code = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    ZSTD_CCtxParams_init_internal(cctxParams, &mut params, ZSTD_NO_CLEVEL);
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_CCtxParams_setZstdParams(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut params: *const ZSTD_parameters,
) {
    if ZSTD_checkCParams((*params).cParams) == 0 {} else {
        __assert_fail(
            b"!ZSTD_checkCParams(params->cParams)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            416 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void ZSTD_CCtxParams_setZstdParams(ZSTD_CCtx_params *, const ZSTD_parameters *)\0",
            ))
                .as_ptr(),
        );
    }
    (*cctxParams).cParams = (*params).cParams;
    (*cctxParams).fParams = (*params).fParams;
    (*cctxParams).compressionLevel = ZSTD_NO_CLEVEL;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_cParam_getBounds(
    mut param: ZSTD_cParameter,
) -> ZSTD_bounds {
    let mut bounds = {
        let mut init = ZSTD_bounds {
            error: 0 as libc::c_int as size_t,
            lowerBound: 0 as libc::c_int,
            upperBound: 0 as libc::c_int,
        };
        init
    };
    match param as libc::c_uint {
        100 => {
            bounds.lowerBound = ZSTD_minCLevel();
            bounds.upperBound = ZSTD_maxCLevel();
            return bounds;
        }
        101 => {
            bounds.lowerBound = ZSTD_WINDOWLOG_MIN;
            bounds
                .upperBound = if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            };
            return bounds;
        }
        102 => {
            bounds.lowerBound = ZSTD_HASHLOG_MIN;
            bounds
                .upperBound = if (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) < 30 as libc::c_int
            {
                if ::core::mem::size_of::<size_t>() as libc::c_ulong
                    == 4 as libc::c_int as libc::c_ulong
                {
                    ZSTD_WINDOWLOG_MAX_32
                } else {
                    ZSTD_WINDOWLOG_MAX_64
                }
            } else {
                30 as libc::c_int
            };
            return bounds;
        }
        103 => {
            bounds.lowerBound = ZSTD_CHAINLOG_MIN;
            bounds
                .upperBound = if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_CHAINLOG_MAX_32
            } else {
                ZSTD_CHAINLOG_MAX_64
            };
            return bounds;
        }
        104 => {
            bounds.lowerBound = ZSTD_SEARCHLOG_MIN;
            bounds
                .upperBound = (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) - 1 as libc::c_int;
            return bounds;
        }
        105 => {
            bounds.lowerBound = ZSTD_MINMATCH_MIN;
            bounds.upperBound = ZSTD_MINMATCH_MAX;
            return bounds;
        }
        106 => {
            bounds.lowerBound = ZSTD_TARGETLENGTH_MIN;
            bounds.upperBound = ZSTD_TARGETLENGTH_MAX;
            return bounds;
        }
        107 => {
            bounds.lowerBound = ZSTD_STRATEGY_MIN;
            bounds.upperBound = ZSTD_STRATEGY_MAX;
            return bounds;
        }
        200 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        201 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        202 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        400 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds
                .upperBound = if ::core::mem::size_of::<*mut libc::c_void>()
                as libc::c_ulong == 4 as libc::c_int as libc::c_ulong
            {
                64 as libc::c_int
            } else {
                256 as libc::c_int
            };
            return bounds;
        }
        401 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds
                .upperBound = if MEM_32bits() != 0 {
                512 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)
            } else {
                1024 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)
            };
            return bounds;
        }
        402 => {
            bounds.lowerBound = ZSTD_OVERLAPLOG_MIN;
            bounds.upperBound = ZSTD_OVERLAPLOG_MAX;
            return bounds;
        }
        1005 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        160 => {
            bounds.lowerBound = ZSTD_ps_auto as libc::c_int;
            bounds.upperBound = ZSTD_ps_disable as libc::c_int;
            return bounds;
        }
        161 => {
            bounds.lowerBound = ZSTD_LDM_HASHLOG_MIN;
            bounds
                .upperBound = if (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) < 30 as libc::c_int
            {
                if ::core::mem::size_of::<size_t>() as libc::c_ulong
                    == 4 as libc::c_int as libc::c_ulong
                {
                    ZSTD_WINDOWLOG_MAX_32
                } else {
                    ZSTD_WINDOWLOG_MAX_64
                }
            } else {
                30 as libc::c_int
            };
            return bounds;
        }
        162 => {
            bounds.lowerBound = ZSTD_LDM_MINMATCH_MIN;
            bounds.upperBound = ZSTD_LDM_MINMATCH_MAX;
            return bounds;
        }
        163 => {
            bounds.lowerBound = ZSTD_LDM_BUCKETSIZELOG_MIN;
            bounds.upperBound = ZSTD_LDM_BUCKETSIZELOG_MAX;
            return bounds;
        }
        164 => {
            bounds.lowerBound = ZSTD_LDM_HASHRATELOG_MIN;
            bounds
                .upperBound = (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) - ZSTD_HASHLOG_MIN;
            return bounds;
        }
        500 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        1000 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        10 => {
            bounds.lowerBound = ZSTD_f_zstd1 as libc::c_int;
            bounds.upperBound = ZSTD_f_zstd1_magicless as libc::c_int;
            return bounds;
        }
        1001 => {
            bounds.lowerBound = ZSTD_dictDefaultAttach as libc::c_int;
            bounds.upperBound = ZSTD_dictForceLoad as libc::c_int;
            return bounds;
        }
        1002 => {
            bounds.lowerBound = ZSTD_ps_auto as libc::c_int;
            bounds.upperBound = ZSTD_ps_disable as libc::c_int;
            return bounds;
        }
        1003 => {
            bounds.lowerBound = ZSTD_TARGETCBLOCKSIZE_MIN;
            bounds.upperBound = ZSTD_TARGETCBLOCKSIZE_MAX;
            return bounds;
        }
        1004 => {
            bounds.lowerBound = ZSTD_SRCSIZEHINT_MIN;
            bounds.upperBound = ZSTD_SRCSIZEHINT_MAX;
            return bounds;
        }
        1006 | 1007 => {
            bounds.lowerBound = ZSTD_bm_buffered as libc::c_int;
            bounds.upperBound = ZSTD_bm_stable as libc::c_int;
            return bounds;
        }
        1008 => {
            bounds.lowerBound = ZSTD_sf_noBlockDelimiters as libc::c_int;
            bounds.upperBound = ZSTD_sf_explicitBlockDelimiters as libc::c_int;
            return bounds;
        }
        1009 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        1010 => {
            bounds.lowerBound = ZSTD_ps_auto as libc::c_int;
            bounds.upperBound = ZSTD_ps_disable as libc::c_int;
            return bounds;
        }
        1011 => {
            bounds.lowerBound = ZSTD_ps_auto as libc::c_int;
            bounds.upperBound = ZSTD_ps_disable as libc::c_int;
            return bounds;
        }
        1012 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        1013 => {
            bounds.lowerBound = ZSTD_ps_auto as libc::c_int;
            bounds.upperBound = ZSTD_ps_disable as libc::c_int;
            return bounds;
        }
        1014 => {
            bounds.lowerBound = 0 as libc::c_int;
            bounds.upperBound = 1 as libc::c_int;
            return bounds;
        }
        1015 => {
            bounds.lowerBound = ZSTD_BLOCKSIZE_MAX_MIN;
            bounds.upperBound = ZSTD_BLOCKSIZE_MAX;
            return bounds;
        }
        1016 => {
            bounds.lowerBound = ZSTD_ps_auto as libc::c_int;
            bounds.upperBound = ZSTD_ps_disable as libc::c_int;
            return bounds;
        }
        _ => {
            bounds.error = -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t;
            return bounds;
        }
    };
}
unsafe extern "C" fn ZSTD_cParam_clampBounds(
    mut cParam: ZSTD_cParameter,
    mut value: *mut libc::c_int,
) -> size_t {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) != 0 {
        return bounds.error;
    }
    if *value < bounds.lowerBound {
        *value = bounds.lowerBound;
    }
    if *value > bounds.upperBound {
        *value = bounds.upperBound;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_isUpdateAuthorized(mut param: ZSTD_cParameter) -> libc::c_int {
    match param as libc::c_uint {
        100 | 102 | 103 | 104 | 105 | 106 | 107 => return 1 as libc::c_int,
        10 | 101 | 200 | 201 | 202 | 1000 | 400 | 401 | 402 | 500 | 1005 | 160 | 161
        | 162 | 163 | 164 | 1001 | 1002 | 1003 | 1004 | 1006 | 1007 | 1008 | 1009 | 1010
        | 1011 | 1012 | 1013 | 1014 | 1015 | 1016 | _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setParameter(
    mut cctx: *mut ZSTD_CCtx,
    mut param: ZSTD_cParameter,
    mut value: libc::c_int,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        if ZSTD_isUpdateAuthorized(param) != 0 {
            (*cctx).cParamsChanged = 1 as libc::c_int;
        } else {
            return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
        }
    }
    match param as libc::c_uint {
        400 => {
            if value != 0 as libc::c_int && (*cctx).staticSize != 0 {
                return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t;
            }
        }
        100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 164 | 10 | 200 | 201 | 202 | 1000
        | 1001 | 1002 | 401 | 402 | 500 | 1005 | 160 | 161 | 162 | 163 | 1003 | 1004
        | 1006 | 1007 | 1008 | 1009 | 1010 | 1011 | 1012 | 1013 | 1014 | 1015 | 1016 => {}
        _ => return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t,
    }
    return ZSTD_CCtxParams_setParameter(&mut (*cctx).requestedParams, param, value);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_setParameter(
    mut CCtxParams: *mut ZSTD_CCtx_params,
    mut param: ZSTD_cParameter,
    mut value: libc::c_int,
) -> size_t {
    match param as libc::c_uint {
        10 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam2, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).format = value as ZSTD_format_e;
            return (*CCtxParams).format as size_t;
        }
        100 => {
            let err_code = ZSTD_cParam_clampBounds(param, &mut value);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if value == 0 as libc::c_int {
                (*CCtxParams).compressionLevel = ZSTD_CLEVEL_DEFAULT;
            } else {
                (*CCtxParams).compressionLevel = value;
            }
            if (*CCtxParams).compressionLevel >= 0 as libc::c_int {
                return (*CCtxParams).compressionLevel as size_t;
            }
            return 0 as libc::c_int as size_t;
        }
        101 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_windowLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).cParams.windowLog = value as U32;
            return (*CCtxParams).cParams.windowLog as size_t;
        }
        102 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_hashLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).cParams.hashLog = value as U32;
            return (*CCtxParams).cParams.hashLog as size_t;
        }
        103 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_chainLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).cParams.chainLog = value as U32;
            return (*CCtxParams).cParams.chainLog as size_t;
        }
        104 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_searchLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).cParams.searchLog = value as U32;
            return value as size_t;
        }
        105 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_minMatch, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).cParams.minMatch = value as U32;
            return (*CCtxParams).cParams.minMatch as size_t;
        }
        106 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_targetLength, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).cParams.targetLength = value as U32;
            return (*CCtxParams).cParams.targetLength as size_t;
        }
        107 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_strategy, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).cParams.strategy = value as ZSTD_strategy;
            return (*CCtxParams).cParams.strategy as size_t;
        }
        200 => {
            (*CCtxParams)
                .fParams
                .contentSizeFlag = (value != 0 as libc::c_int) as libc::c_int;
            return (*CCtxParams).fParams.contentSizeFlag as size_t;
        }
        201 => {
            (*CCtxParams)
                .fParams
                .checksumFlag = (value != 0 as libc::c_int) as libc::c_int;
            return (*CCtxParams).fParams.checksumFlag as size_t;
        }
        202 => {
            (*CCtxParams).fParams.noDictIDFlag = (value == 0) as libc::c_int;
            return ((*CCtxParams).fParams.noDictIDFlag == 0) as libc::c_int as size_t;
        }
        1000 => {
            (*CCtxParams).forceWindow = (value != 0 as libc::c_int) as libc::c_int;
            return (*CCtxParams).forceWindow as size_t;
        }
        1001 => {
            let pref = value as ZSTD_dictAttachPref_e;
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam4, pref as libc::c_int)
                == 0
            {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).attachDictPref = pref;
            return (*CCtxParams).attachDictPref as size_t;
        }
        1002 => {
            let lcm = value as ZSTD_paramSwitch_e;
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam5, lcm as libc::c_int)
                == 0
            {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).literalCompressionMode = lcm;
            return (*CCtxParams).literalCompressionMode as size_t;
        }
        400 => {
            let err_code_0 = ZSTD_cParam_clampBounds(param, &mut value);
            if ERR_isError(err_code_0) != 0 {
                return err_code_0;
            }
            (*CCtxParams).nbWorkers = value;
            return (*CCtxParams).nbWorkers as size_t;
        }
        401 => {
            if value != 0 as libc::c_int && value < ZSTDMT_JOBSIZE_MIN {
                value = ZSTDMT_JOBSIZE_MIN;
            }
            let err_code_1 = ZSTD_cParam_clampBounds(param, &mut value);
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            if value >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"value >= 0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    883 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"size_t ZSTD_CCtxParams_setParameter(ZSTD_CCtx_params *, ZSTD_cParameter, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            (*CCtxParams).jobSize = value as size_t;
            return (*CCtxParams).jobSize;
        }
        402 => {
            let err_code_2 = ZSTD_cParam_clampBounds(ZSTD_c_overlapLog, &mut value);
            if ERR_isError(err_code_2) != 0 {
                return err_code_2;
            }
            (*CCtxParams).overlapLog = value;
            return (*CCtxParams).overlapLog as size_t;
        }
        500 => {
            let err_code_3 = ZSTD_cParam_clampBounds(ZSTD_c_overlapLog, &mut value);
            if ERR_isError(err_code_3) != 0 {
                return err_code_3;
            }
            (*CCtxParams).rsyncable = value;
            return (*CCtxParams).rsyncable as size_t;
        }
        1005 => {
            (*CCtxParams)
                .enableDedicatedDictSearch = (value != 0 as libc::c_int) as libc::c_int;
            return (*CCtxParams).enableDedicatedDictSearch as size_t;
        }
        160 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_enableLongDistanceMatching, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).ldmParams.enableLdm = value as ZSTD_paramSwitch_e;
            return (*CCtxParams).ldmParams.enableLdm as size_t;
        }
        161 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_ldmHashLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).ldmParams.hashLog = value as U32;
            return (*CCtxParams).ldmParams.hashLog as size_t;
        }
        162 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_ldmMinMatch, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).ldmParams.minMatchLength = value as U32;
            return (*CCtxParams).ldmParams.minMatchLength as size_t;
        }
        163 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_ldmBucketSizeLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).ldmParams.bucketSizeLog = value as U32;
            return (*CCtxParams).ldmParams.bucketSizeLog as size_t;
        }
        164 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_ldmHashRateLog, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).ldmParams.hashRateLog = value as U32;
            return (*CCtxParams).ldmParams.hashRateLog as size_t;
        }
        1003 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam6, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).targetCBlockSize = value as U32 as size_t;
            return (*CCtxParams).targetCBlockSize;
        }
        1004 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam7, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).srcSizeHint = value;
            return (*CCtxParams).srcSizeHint as size_t;
        }
        1006 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam9, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).inBufferMode = value as ZSTD_bufferMode_e;
            return (*CCtxParams).inBufferMode as size_t;
        }
        1007 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam10, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).outBufferMode = value as ZSTD_bufferMode_e;
            return (*CCtxParams).outBufferMode as size_t;
        }
        1008 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam11, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).blockDelimiters = value as ZSTD_sequenceFormat_e;
            return (*CCtxParams).blockDelimiters as size_t;
        }
        1009 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam12, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).validateSequences = value;
            return (*CCtxParams).validateSequences as size_t;
        }
        1010 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam13, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).useBlockSplitter = value as ZSTD_paramSwitch_e;
            return (*CCtxParams).useBlockSplitter as size_t;
        }
        1011 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam14, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).useRowMatchFinder = value as ZSTD_paramSwitch_e;
            return (*CCtxParams).useRowMatchFinder as size_t;
        }
        1012 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam15, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).deterministicRefPrefix = (value != 0) as libc::c_int;
            return (*CCtxParams).deterministicRefPrefix as size_t;
        }
        1013 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam16, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).prefetchCDictTables = value as ZSTD_paramSwitch_e;
            return (*CCtxParams).prefetchCDictTables as size_t;
        }
        1014 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam17, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).enableMatchFinderFallback = value;
            return (*CCtxParams).enableMatchFinderFallback as size_t;
        }
        1015 => {
            if value != 0 as libc::c_int {
                if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam18, value) == 0 {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
                }
            }
            (*CCtxParams).maxBlockSize = value as size_t;
            return (*CCtxParams).maxBlockSize;
        }
        1016 => {
            if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam19, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
            }
            (*CCtxParams).searchForExternalRepcodes = value as ZSTD_paramSwitch_e;
            return (*CCtxParams).searchForExternalRepcodes as size_t;
        }
        _ => return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_getParameter(
    mut cctx: *const ZSTD_CCtx,
    mut param: ZSTD_cParameter,
    mut value: *mut libc::c_int,
) -> size_t {
    return ZSTD_CCtxParams_getParameter(&(*cctx).requestedParams, param, value);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_getParameter(
    mut CCtxParams: *const ZSTD_CCtx_params,
    mut param: ZSTD_cParameter,
    mut value: *mut libc::c_int,
) -> size_t {
    match param as libc::c_uint {
        10 => {
            *value = (*CCtxParams).format as libc::c_int;
        }
        100 => {
            *value = (*CCtxParams).compressionLevel;
        }
        101 => {
            *value = (*CCtxParams).cParams.windowLog as libc::c_int;
        }
        102 => {
            *value = (*CCtxParams).cParams.hashLog as libc::c_int;
        }
        103 => {
            *value = (*CCtxParams).cParams.chainLog as libc::c_int;
        }
        104 => {
            *value = (*CCtxParams).cParams.searchLog as libc::c_int;
        }
        105 => {
            *value = (*CCtxParams).cParams.minMatch as libc::c_int;
        }
        106 => {
            *value = (*CCtxParams).cParams.targetLength as libc::c_int;
        }
        107 => {
            *value = (*CCtxParams).cParams.strategy as libc::c_uint as libc::c_int;
        }
        200 => {
            *value = (*CCtxParams).fParams.contentSizeFlag;
        }
        201 => {
            *value = (*CCtxParams).fParams.checksumFlag;
        }
        202 => {
            *value = ((*CCtxParams).fParams.noDictIDFlag == 0) as libc::c_int;
        }
        1000 => {
            *value = (*CCtxParams).forceWindow;
        }
        1001 => {
            *value = (*CCtxParams).attachDictPref as libc::c_int;
        }
        1002 => {
            *value = (*CCtxParams).literalCompressionMode as libc::c_int;
        }
        400 => {
            *value = (*CCtxParams).nbWorkers;
        }
        401 => {
            if (*CCtxParams).jobSize <= 2147483647 as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"CCtxParams->jobSize <= INT_MAX\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    1078 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"size_t ZSTD_CCtxParams_getParameter(const ZSTD_CCtx_params *, ZSTD_cParameter, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            *value = (*CCtxParams).jobSize as libc::c_int;
        }
        402 => {
            *value = (*CCtxParams).overlapLog;
        }
        500 => {
            *value = (*CCtxParams).rsyncable;
        }
        1005 => {
            *value = (*CCtxParams).enableDedicatedDictSearch;
        }
        160 => {
            *value = (*CCtxParams).ldmParams.enableLdm as libc::c_int;
        }
        161 => {
            *value = (*CCtxParams).ldmParams.hashLog as libc::c_int;
        }
        162 => {
            *value = (*CCtxParams).ldmParams.minMatchLength as libc::c_int;
        }
        163 => {
            *value = (*CCtxParams).ldmParams.bucketSizeLog as libc::c_int;
        }
        164 => {
            *value = (*CCtxParams).ldmParams.hashRateLog as libc::c_int;
        }
        1003 => {
            *value = (*CCtxParams).targetCBlockSize as libc::c_int;
        }
        1004 => {
            *value = (*CCtxParams).srcSizeHint;
        }
        1006 => {
            *value = (*CCtxParams).inBufferMode as libc::c_int;
        }
        1007 => {
            *value = (*CCtxParams).outBufferMode as libc::c_int;
        }
        1008 => {
            *value = (*CCtxParams).blockDelimiters as libc::c_int;
        }
        1009 => {
            *value = (*CCtxParams).validateSequences;
        }
        1010 => {
            *value = (*CCtxParams).useBlockSplitter as libc::c_int;
        }
        1011 => {
            *value = (*CCtxParams).useRowMatchFinder as libc::c_int;
        }
        1012 => {
            *value = (*CCtxParams).deterministicRefPrefix;
        }
        1013 => {
            *value = (*CCtxParams).prefetchCDictTables as libc::c_int;
        }
        1014 => {
            *value = (*CCtxParams).enableMatchFinderFallback;
        }
        1015 => {
            *value = (*CCtxParams).maxBlockSize as libc::c_int;
        }
        1016 => {
            *value = (*CCtxParams).searchForExternalRepcodes as libc::c_int;
        }
        _ => return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t,
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setParametersUsingCCtxParams(
    mut cctx: *mut ZSTD_CCtx,
    mut params: *const ZSTD_CCtx_params,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    if !((*cctx).cdict).is_null() {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    (*cctx).requestedParams = *params;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setCParams(
    mut cctx: *mut ZSTD_CCtx,
    mut cparams: ZSTD_compressionParameters,
) -> size_t {
    let err_code = ZSTD_checkCParams(cparams);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_windowLog,
        cparams.windowLog as libc::c_int,
    );
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_chainLog,
        cparams.chainLog as libc::c_int,
    );
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    let err_code_2 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_hashLog,
        cparams.hashLog as libc::c_int,
    );
    if ERR_isError(err_code_2) != 0 {
        return err_code_2;
    }
    let err_code_3 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_searchLog,
        cparams.searchLog as libc::c_int,
    );
    if ERR_isError(err_code_3) != 0 {
        return err_code_3;
    }
    let err_code_4 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_minMatch,
        cparams.minMatch as libc::c_int,
    );
    if ERR_isError(err_code_4) != 0 {
        return err_code_4;
    }
    let err_code_5 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_targetLength,
        cparams.targetLength as libc::c_int,
    );
    if ERR_isError(err_code_5) != 0 {
        return err_code_5;
    }
    let err_code_6 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_strategy,
        cparams.strategy as libc::c_int,
    );
    if ERR_isError(err_code_6) != 0 {
        return err_code_6;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setFParams(
    mut cctx: *mut ZSTD_CCtx,
    mut fparams: ZSTD_frameParameters,
) -> size_t {
    let err_code = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_contentSizeFlag,
        (fparams.contentSizeFlag != 0 as libc::c_int) as libc::c_int,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_checksumFlag,
        (fparams.checksumFlag != 0 as libc::c_int) as libc::c_int,
    );
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_c_dictIDFlag,
        (fparams.noDictIDFlag == 0 as libc::c_int) as libc::c_int,
    );
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setParams(
    mut cctx: *mut ZSTD_CCtx,
    mut params: ZSTD_parameters,
) -> size_t {
    let err_code = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setFParams(cctx, params.fParams);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setCParams(cctx, params.cParams);
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setPledgedSrcSize(
    mut cctx: *mut ZSTD_CCtx,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    (*cctx)
        .pledgedSrcSizePlusOne = pledgedSrcSize
        .wrapping_add(1 as libc::c_int as libc::c_ulonglong);
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_initLocalDict(mut cctx: *mut ZSTD_CCtx) -> size_t {
    let dl: *mut ZSTD_localDict = &mut (*cctx).localDict;
    if ((*dl).dict).is_null() {
        if ((*dl).dictBuffer).is_null() {} else {
            __assert_fail(
                b"dl->dictBuffer == NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1244 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                    .as_ptr(),
            );
        }
        if ((*dl).cdict).is_null() {} else {
            __assert_fail(
                b"dl->cdict == NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1245 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                    .as_ptr(),
            );
        }
        if (*dl).dictSize == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"dl->dictSize == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1246 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                    .as_ptr(),
            );
        }
        return 0 as libc::c_int as size_t;
    }
    if !((*dl).cdict).is_null() {
        if (*cctx).cdict == (*dl).cdict as *const ZSTD_CDict {} else {
            __assert_fail(
                b"cctx->cdict == dl->cdict\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1251 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                    .as_ptr(),
            );
        }
        return 0 as libc::c_int as size_t;
    }
    if (*dl).dictSize > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"dl->dictSize > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1254 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    if ((*cctx).cdict).is_null() {} else {
        __assert_fail(
            b"cctx->cdict == NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1255 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    if ((*cctx).prefixDict.dict).is_null() {} else {
        __assert_fail(
            b"cctx->prefixDict.dict == NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1256 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"size_t ZSTD_initLocalDict(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    (*dl)
        .cdict = ZSTD_createCDict_advanced2(
        (*dl).dict,
        (*dl).dictSize,
        ZSTD_dlm_byRef,
        (*dl).dictContentType,
        &mut (*cctx).requestedParams,
        (*cctx).customMem,
    );
    if ((*dl).cdict).is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    (*cctx).cdict = (*dl).cdict;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    ZSTD_clearAllDicts(cctx);
    if dict.is_null() || dictSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    if dictLoadMethod as libc::c_uint == ZSTD_dlm_byRef as libc::c_int as libc::c_uint {
        (*cctx).localDict.dict = dict;
    } else {
        let mut dictBuffer = 0 as *mut libc::c_void;
        if (*cctx).staticSize != 0 {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
        dictBuffer = ZSTD_customMalloc(dictSize, (*cctx).customMem);
        if dictBuffer.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
        libc::memcpy(dictBuffer, dict, dictSize as libc::size_t);
        (*cctx).localDict.dictBuffer = dictBuffer;
        (*cctx).localDict.dict = dictBuffer;
    }
    (*cctx).localDict.dictSize = dictSize;
    (*cctx).localDict.dictContentType = dictContentType;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary_byReference(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
) -> size_t {
    return ZSTD_CCtx_loadDictionary_advanced(
        cctx,
        dict,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
) -> size_t {
    return ZSTD_CCtx_loadDictionary_advanced(
        cctx,
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    ZSTD_clearAllDicts(cctx);
    (*cctx).cdict = cdict;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refThreadPool(
    mut cctx: *mut ZSTD_CCtx,
    mut pool: *mut ZSTD_threadPool,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    (*cctx).pool = pool;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix(
    mut cctx: *mut ZSTD_CCtx,
    mut prefix: *const libc::c_void,
    mut prefixSize: size_t,
) -> size_t {
    return ZSTD_CCtx_refPrefix_advanced(cctx, prefix, prefixSize, ZSTD_dct_rawContent);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut prefix: *const libc::c_void,
    mut prefixSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if (*cctx).streamStage as libc::c_uint != zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    ZSTD_clearAllDicts(cctx);
    if !prefix.is_null() && prefixSize > 0 as libc::c_int as libc::c_ulong {
        (*cctx).prefixDict.dict = prefix;
        (*cctx).prefixDict.dictSize = prefixSize;
        (*cctx).prefixDict.dictContentType = dictContentType;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_reset(
    mut cctx: *mut ZSTD_CCtx,
    mut reset: ZSTD_ResetDirective,
) -> size_t {
    if reset as libc::c_uint == ZSTD_reset_session_only as libc::c_int as libc::c_uint
        || reset as libc::c_uint
            == ZSTD_reset_session_and_parameters as libc::c_int as libc::c_uint
    {
        (*cctx).streamStage = zcss_init;
        (*cctx).pledgedSrcSizePlusOne = 0 as libc::c_int as libc::c_ulonglong;
    }
    if reset as libc::c_uint == ZSTD_reset_parameters as libc::c_int as libc::c_uint
        || reset as libc::c_uint
            == ZSTD_reset_session_and_parameters as libc::c_int as libc::c_uint
    {
        if (*cctx).streamStage as libc::c_uint
            != zcss_init as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
        }
        ZSTD_clearAllDicts(cctx);
        libc::memset(
            &mut (*cctx).externalMatchCtx as *mut ZSTD_externalMatchCtx
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ZSTD_externalMatchCtx>() as libc::c_ulong
                as libc::size_t,
        );
        return ZSTD_CCtxParams_reset(&mut (*cctx).requestedParams);
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_checkCParams(
    mut cParams: ZSTD_compressionParameters,
) -> size_t {
    if ZSTD_cParam_withinBounds(ZSTD_c_windowLog, cParams.windowLog as libc::c_int) == 0
    {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if ZSTD_cParam_withinBounds(ZSTD_c_chainLog, cParams.chainLog as libc::c_int) == 0 {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if ZSTD_cParam_withinBounds(ZSTD_c_hashLog, cParams.hashLog as libc::c_int) == 0 {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if ZSTD_cParam_withinBounds(ZSTD_c_searchLog, cParams.searchLog as libc::c_int) == 0
    {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if ZSTD_cParam_withinBounds(ZSTD_c_minMatch, cParams.minMatch as libc::c_int) == 0 {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if ZSTD_cParam_withinBounds(ZSTD_c_targetLength, cParams.targetLength as libc::c_int)
        == 0
    {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if ZSTD_cParam_withinBounds(ZSTD_c_strategy, cParams.strategy as libc::c_int) == 0 {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_clampCParams(
    mut cParams: ZSTD_compressionParameters,
) -> ZSTD_compressionParameters {
    let bounds = ZSTD_cParam_getBounds(ZSTD_c_windowLog);
    if (cParams.windowLog as libc::c_int) < bounds.lowerBound {
        cParams.windowLog = bounds.lowerBound as libc::c_uint;
    } else if cParams.windowLog as libc::c_int > bounds.upperBound {
        cParams.windowLog = bounds.upperBound as libc::c_uint;
    }
    let bounds_0 = ZSTD_cParam_getBounds(ZSTD_c_chainLog);
    if (cParams.chainLog as libc::c_int) < bounds_0.lowerBound {
        cParams.chainLog = bounds_0.lowerBound as libc::c_uint;
    } else if cParams.chainLog as libc::c_int > bounds_0.upperBound {
        cParams.chainLog = bounds_0.upperBound as libc::c_uint;
    }
    let bounds_1 = ZSTD_cParam_getBounds(ZSTD_c_hashLog);
    if (cParams.hashLog as libc::c_int) < bounds_1.lowerBound {
        cParams.hashLog = bounds_1.lowerBound as libc::c_uint;
    } else if cParams.hashLog as libc::c_int > bounds_1.upperBound {
        cParams.hashLog = bounds_1.upperBound as libc::c_uint;
    }
    let bounds_2 = ZSTD_cParam_getBounds(ZSTD_c_searchLog);
    if (cParams.searchLog as libc::c_int) < bounds_2.lowerBound {
        cParams.searchLog = bounds_2.lowerBound as libc::c_uint;
    } else if cParams.searchLog as libc::c_int > bounds_2.upperBound {
        cParams.searchLog = bounds_2.upperBound as libc::c_uint;
    }
    let bounds_3 = ZSTD_cParam_getBounds(ZSTD_c_minMatch);
    if (cParams.minMatch as libc::c_int) < bounds_3.lowerBound {
        cParams.minMatch = bounds_3.lowerBound as libc::c_uint;
    } else if cParams.minMatch as libc::c_int > bounds_3.upperBound {
        cParams.minMatch = bounds_3.upperBound as libc::c_uint;
    }
    let bounds_4 = ZSTD_cParam_getBounds(ZSTD_c_targetLength);
    if (cParams.targetLength as libc::c_int) < bounds_4.lowerBound {
        cParams.targetLength = bounds_4.lowerBound as libc::c_uint;
    } else if cParams.targetLength as libc::c_int > bounds_4.upperBound {
        cParams.targetLength = bounds_4.upperBound as libc::c_uint;
    }
    let bounds_5 = ZSTD_cParam_getBounds(ZSTD_c_strategy);
    if (cParams.strategy as libc::c_int) < bounds_5.lowerBound {
        cParams.strategy = bounds_5.lowerBound as ZSTD_strategy;
    } else if cParams.strategy as libc::c_int > bounds_5.upperBound {
        cParams.strategy = bounds_5.upperBound as ZSTD_strategy;
    }
    return cParams;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_cycleLog(
    mut hashLog: U32,
    mut strat: ZSTD_strategy,
) -> U32 {
    let btScale = (strat as U32 >= ZSTD_btlazy2 as libc::c_int as U32) as libc::c_int
        as U32;
    return hashLog.wrapping_sub(btScale);
}
unsafe extern "C" fn ZSTD_dictAndWindowLog(
    mut windowLog: U32,
    mut srcSize: U64,
    mut dictSize: U64,
) -> U32 {
    let maxWindowSize = ((1 as libc::c_ulonglong)
        << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        })) as U64;
    if dictSize == 0 as libc::c_int as libc::c_ulong {
        return windowLog;
    }
    if windowLog
        <= (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            30 as libc::c_int
        } else {
            31 as libc::c_int
        }) as libc::c_uint
    {} else {
        __assert_fail(
            b"windowLog <= ZSTD_WINDOWLOG_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1433 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"U32 ZSTD_dictAndWindowLog(U32, U64, U64)\0"))
                .as_ptr(),
        );
    }
    if srcSize as libc::c_ulonglong
        != (0 as libc::c_ulonglong).wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
    {} else {
        __assert_fail(
            b"srcSize != ZSTD_CONTENTSIZE_UNKNOWN\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1434 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"U32 ZSTD_dictAndWindowLog(U32, U64, U64)\0"))
                .as_ptr(),
        );
    }
    let windowSize = ((1 as libc::c_ulonglong) << windowLog) as U64;
    let dictAndWindowSize = dictSize.wrapping_add(windowSize);
    if windowSize >= dictSize.wrapping_add(srcSize) {
        return windowLog
    } else if dictAndWindowSize >= maxWindowSize {
        return (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as U32
    } else {
        return (ZSTD_highbit32(
            (dictAndWindowSize as U32).wrapping_sub(1 as libc::c_int as libc::c_uint),
        ))
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    };
}
unsafe extern "C" fn ZSTD_adjustCParams_internal(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: libc::c_ulonglong,
    mut dictSize: size_t,
    mut mode: ZSTD_cParamMode_e,
    mut useRowMatchFinder: ZSTD_paramSwitch_e,
) -> ZSTD_compressionParameters {
    let minSrcSize = 513 as libc::c_int as U64;
    let maxWindowResize = ((1 as libc::c_ulonglong)
        << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) - 1 as libc::c_int) as U64;
    if ZSTD_checkCParams(cPar) == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ZSTD_checkCParams(cPar)==0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1468 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 150],
                &[libc::c_char; 150],
            >(
                b"ZSTD_compressionParameters ZSTD_adjustCParams_internal(ZSTD_compressionParameters, unsigned long long, size_t, ZSTD_cParamMode_e, ZSTD_paramSwitch_e)\0",
            ))
                .as_ptr(),
        );
    }
    match mode as libc::c_uint {
        3 | 0 => {}
        2 => {
            if dictSize != 0 && srcSize == ZSTD_CONTENTSIZE_UNKNOWN {
                srcSize = minSrcSize as libc::c_ulonglong;
            }
        }
        1 => {
            dictSize = 0 as libc::c_int as size_t;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1493 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 150],
                    &[libc::c_char; 150],
                >(
                    b"ZSTD_compressionParameters ZSTD_adjustCParams_internal(ZSTD_compressionParameters, unsigned long long, size_t, ZSTD_cParamMode_e, ZSTD_paramSwitch_e)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if srcSize <= maxWindowResize as libc::c_ulonglong && dictSize <= maxWindowResize {
        let tSize = srcSize.wrapping_add(dictSize as libc::c_ulonglong) as U32;
        static mut hashSizeMin: U32 = ((1 as libc::c_int) << ZSTD_HASHLOG_MIN) as U32;
        let srcLog = if tSize < hashSizeMin {
            ZSTD_HASHLOG_MIN as libc::c_uint
        } else {
            (ZSTD_highbit32(tSize.wrapping_sub(1 as libc::c_int as libc::c_uint)))
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        };
        if cPar.windowLog > srcLog {
            cPar.windowLog = srcLog;
        }
    }
    if srcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let dictAndWindowLog = ZSTD_dictAndWindowLog(
            cPar.windowLog,
            srcSize as U64,
            dictSize,
        );
        let cycleLog = ZSTD_cycleLog(cPar.chainLog, cPar.strategy);
        if cPar.hashLog > dictAndWindowLog.wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            cPar
                .hashLog = dictAndWindowLog
                .wrapping_add(1 as libc::c_int as libc::c_uint);
        }
        if cycleLog > dictAndWindowLog {
            cPar
                .chainLog = (cPar.chainLog)
                .wrapping_sub(cycleLog.wrapping_sub(dictAndWindowLog));
        }
    }
    if cPar.windowLog < ZSTD_WINDOWLOG_ABSOLUTEMIN as libc::c_uint {
        cPar.windowLog = ZSTD_WINDOWLOG_ABSOLUTEMIN as libc::c_uint;
    }
    if mode as libc::c_uint == ZSTD_cpm_createCDict as libc::c_int as libc::c_uint
        && ZSTD_CDictIndicesAreTagged(&mut cPar) != 0
    {
        let maxShortCacheHashLog = (32 as libc::c_int - ZSTD_SHORT_CACHE_TAG_BITS)
            as U32;
        if cPar.hashLog > maxShortCacheHashLog {
            cPar.hashLog = maxShortCacheHashLog;
        }
        if cPar.chainLog > maxShortCacheHashLog {
            cPar.chainLog = maxShortCacheHashLog;
        }
    }
    if useRowMatchFinder as libc::c_uint == ZSTD_ps_auto as libc::c_int as libc::c_uint {
        useRowMatchFinder = ZSTD_ps_enable;
    }
    if ZSTD_rowMatchFinderUsed(cPar.strategy, useRowMatchFinder) != 0 {
        let rowLog = if 4 as libc::c_int as libc::c_uint
            > (if cPar.searchLog < 6 as libc::c_int as libc::c_uint {
                cPar.searchLog
            } else {
                6 as libc::c_int as libc::c_uint
            })
        {
            4 as libc::c_int as libc::c_uint
        } else if cPar.searchLog < 6 as libc::c_int as libc::c_uint {
            cPar.searchLog
        } else {
            6 as libc::c_int as libc::c_uint
        };
        let maxRowHashLog = (32 as libc::c_int - ZSTD_ROW_HASH_TAG_BITS) as U32;
        let maxHashLog = maxRowHashLog.wrapping_add(rowLog);
        if cPar.hashLog >= rowLog {} else {
            __assert_fail(
                b"cPar.hashLog >= rowLog\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1547 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 150],
                    &[libc::c_char; 150],
                >(
                    b"ZSTD_compressionParameters ZSTD_adjustCParams_internal(ZSTD_compressionParameters, unsigned long long, size_t, ZSTD_cParamMode_e, ZSTD_paramSwitch_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if cPar.hashLog > maxHashLog {
            cPar.hashLog = maxHashLog;
        }
    }
    return cPar;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_adjustCParams(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: libc::c_ulonglong,
    mut dictSize: size_t,
) -> ZSTD_compressionParameters {
    cPar = ZSTD_clampCParams(cPar);
    if srcSize == 0 as libc::c_int as libc::c_ulonglong {
        srcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    return ZSTD_adjustCParams_internal(
        cPar,
        srcSize,
        dictSize,
        ZSTD_cpm_unknown,
        ZSTD_ps_auto,
    );
}
unsafe extern "C" fn ZSTD_overrideCParams(
    mut cParams: *mut ZSTD_compressionParameters,
    mut overrides: *const ZSTD_compressionParameters,
) {
    if (*overrides).windowLog != 0 {
        (*cParams).windowLog = (*overrides).windowLog;
    }
    if (*overrides).hashLog != 0 {
        (*cParams).hashLog = (*overrides).hashLog;
    }
    if (*overrides).chainLog != 0 {
        (*cParams).chainLog = (*overrides).chainLog;
    }
    if (*overrides).searchLog != 0 {
        (*cParams).searchLog = (*overrides).searchLog;
    }
    if (*overrides).minMatch != 0 {
        (*cParams).minMatch = (*overrides).minMatch;
    }
    if (*overrides).targetLength != 0 {
        (*cParams).targetLength = (*overrides).targetLength;
    }
    if (*overrides).strategy as u64 != 0 {
        (*cParams).strategy = (*overrides).strategy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParamsFromCCtxParams(
    mut CCtxParams: *const ZSTD_CCtx_params,
    mut srcSizeHint: U64,
    mut dictSize: size_t,
    mut mode: ZSTD_cParamMode_e,
) -> ZSTD_compressionParameters {
    let mut cParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0 as ZSTD_strategy,
    };
    if srcSizeHint as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
        && (*CCtxParams).srcSizeHint > 0 as libc::c_int
    {
        srcSizeHint = (*CCtxParams).srcSizeHint as U64;
    }
    cParams = ZSTD_getCParams_internal(
        (*CCtxParams).compressionLevel,
        srcSizeHint as libc::c_ulonglong,
        dictSize,
        mode,
    );
    if (*CCtxParams).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        cParams.windowLog = ZSTD_LDM_DEFAULT_WINDOW_LOG as libc::c_uint;
    }
    ZSTD_overrideCParams(&mut cParams, &(*CCtxParams).cParams);
    if ZSTD_checkCParams(cParams) == 0 {} else {
        __assert_fail(
            b"!ZSTD_checkCParams(cParams)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1592 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"ZSTD_compressionParameters ZSTD_getCParamsFromCCtxParams(const ZSTD_CCtx_params *, U64, size_t, ZSTD_cParamMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_adjustCParams_internal(
        cParams,
        srcSizeHint as libc::c_ulonglong,
        dictSize,
        mode,
        (*CCtxParams).useRowMatchFinder,
    );
}
unsafe extern "C" fn ZSTD_sizeof_matchState(
    cParams: *const ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_paramSwitch_e,
    enableDedicatedDictSearch: U32,
    forCCtx: U32,
) -> size_t {
    let chainSize = if ZSTD_allocateChainTable(
        (*cParams).strategy,
        useRowMatchFinder,
        (enableDedicatedDictSearch != 0 && forCCtx == 0) as libc::c_int as U32,
    ) != 0
    {
        (1 as libc::c_int as size_t) << (*cParams).chainLog
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let hSize = (1 as libc::c_int as size_t) << (*cParams).hashLog;
    let hashLog3 = if forCCtx != 0
        && (*cParams).minMatch == 3 as libc::c_int as libc::c_uint
    {
        if (17 as libc::c_int as libc::c_uint) < (*cParams).windowLog {
            17 as libc::c_int as libc::c_uint
        } else {
            (*cParams).windowLog
        }
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let h3Size = if hashLog3 != 0 {
        (1 as libc::c_int as size_t) << hashLog3
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let tableSpace = chainSize
        .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong)
        .wrapping_add(hSize.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong))
        .wrapping_add(
            h3Size.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
        );
    let optPotentialSpace = (ZSTD_cwksp_aligned_alloc_size(
        ((MaxML + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
    ))
        .wrapping_add(
            ZSTD_cwksp_aligned_alloc_size(
                ((MaxLL + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned_alloc_size(
                ((MaxOff + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned_alloc_size(
                (((1 as libc::c_int) << Litbits) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned_alloc_size(
                ((ZSTD_OPT_NUM + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<ZSTD_match_t>() as libc::c_ulong,
                    ),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned_alloc_size(
                ((ZSTD_OPT_NUM + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<ZSTD_optimal_t>() as libc::c_ulong,
                    ),
            ),
        );
    let lazyAdditionalSpace = if ZSTD_rowMatchFinderUsed(
        (*cParams).strategy,
        useRowMatchFinder,
    ) != 0
    {
        ZSTD_cwksp_aligned_alloc_size(hSize)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let optSpace = if forCCtx != 0
        && (*cParams).strategy as libc::c_uint
            >= ZSTD_btopt as libc::c_int as libc::c_uint
    {
        optPotentialSpace
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let slackSpace = ZSTD_cwksp_slack_space_required();
    if useRowMatchFinder as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"useRowMatchFinder != ZSTD_ps_auto\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1632 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"size_t ZSTD_sizeof_matchState(const ZSTD_compressionParameters *const, const ZSTD_paramSwitch_e, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    return tableSpace
        .wrapping_add(optSpace)
        .wrapping_add(slackSpace)
        .wrapping_add(lazyAdditionalSpace);
}
unsafe extern "C" fn ZSTD_maxNbSeq(
    mut blockSize: size_t,
    mut minMatch: libc::c_uint,
    mut useSequenceProducer: libc::c_int,
) -> size_t {
    let divider = (if minMatch == 3 as libc::c_int as libc::c_uint
        || useSequenceProducer != 0
    {
        3 as libc::c_int
    } else {
        4 as libc::c_int
    }) as U32;
    return blockSize.wrapping_div(divider as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCCtxParams_internal(
    mut cParams: *const ZSTD_compressionParameters,
    mut ldmParams: *const ldmParams_t,
    isStatic: libc::c_int,
    useRowMatchFinder: ZSTD_paramSwitch_e,
    buffInSize: size_t,
    buffOutSize: size_t,
    pledgedSrcSize: U64,
    mut useSequenceProducer: libc::c_int,
    mut maxBlockSize: size_t,
) -> size_t {
    let windowSize = (if 1 as libc::c_ulonglong
        > (if (1 as libc::c_ulonglong) << (*cParams).windowLog
            < pledgedSrcSize as libc::c_ulonglong
        {
            (1 as libc::c_ulonglong) << (*cParams).windowLog
        } else {
            pledgedSrcSize as libc::c_ulonglong
        })
    {
        1 as libc::c_ulonglong
    } else if (1 as libc::c_ulonglong) << (*cParams).windowLog
        < pledgedSrcSize as libc::c_ulonglong
    {
        (1 as libc::c_ulonglong) << (*cParams).windowLog
    } else {
        pledgedSrcSize as libc::c_ulonglong
    }) as size_t;
    let blockSize = if ZSTD_resolveMaxBlockSize(maxBlockSize) < windowSize {
        ZSTD_resolveMaxBlockSize(maxBlockSize)
    } else {
        windowSize
    };
    let maxNbSeq = ZSTD_maxNbSeq(blockSize, (*cParams).minMatch, useSequenceProducer);
    let tokenSpace = (ZSTD_cwksp_alloc_size(
        (WILDCOPY_OVERLENGTH as libc::c_ulong).wrapping_add(blockSize),
    ))
        .wrapping_add(
            ZSTD_cwksp_aligned_alloc_size(
                maxNbSeq.wrapping_mul(::core::mem::size_of::<seqDef>() as libc::c_ulong),
            ),
        )
        .wrapping_add(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ZSTD_cwksp_alloc_size(
                        maxNbSeq
                            .wrapping_mul(
                                ::core::mem::size_of::<BYTE>() as libc::c_ulong,
                            ),
                    ),
                ),
        );
    let entropySpace = ZSTD_cwksp_alloc_size(
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
    let blockStateSpace = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(
            ZSTD_cwksp_alloc_size(
                ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong,
            ),
        );
    let matchStateSize = ZSTD_sizeof_matchState(
        cParams,
        useRowMatchFinder,
        0 as libc::c_int as U32,
        1 as libc::c_int as U32,
    );
    let ldmSpace = ZSTD_ldm_getTableSize(*ldmParams);
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq(*ldmParams, blockSize);
    let ldmSeqSpace = if (*ldmParams).enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        ZSTD_cwksp_aligned_alloc_size(
            maxNbLdmSeq.wrapping_mul(::core::mem::size_of::<rawSeq>() as libc::c_ulong),
        )
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let bufferSpace = (ZSTD_cwksp_alloc_size(buffInSize))
        .wrapping_add(ZSTD_cwksp_alloc_size(buffOutSize));
    let cctxSpace = if isStatic != 0 {
        ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
    let externalSeqSpace = if useSequenceProducer != 0 {
        ZSTD_cwksp_aligned_alloc_size(
            maxNbExternalSeq
                .wrapping_mul(::core::mem::size_of::<ZSTD_Sequence>() as libc::c_ulong),
        )
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let neededSpace = cctxSpace
        .wrapping_add(entropySpace)
        .wrapping_add(blockStateSpace)
        .wrapping_add(ldmSpace)
        .wrapping_add(ldmSeqSpace)
        .wrapping_add(matchStateSize)
        .wrapping_add(tokenSpace)
        .wrapping_add(bufferSpace)
        .wrapping_add(externalSeqSpace);
    return neededSpace;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCCtxParams(
    mut params: *const ZSTD_CCtx_params,
) -> size_t {
    let cParams = ZSTD_getCParamsFromCCtxParams(
        params,
        ZSTD_CONTENTSIZE_UNKNOWN as U64,
        0 as libc::c_int as size_t,
        ZSTD_cpm_noAttachDict,
    );
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        (*params).useRowMatchFinder,
        &cParams,
    );
    if (*params).nbWorkers > 0 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    return ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &cParams,
        &(*params).ldmParams,
        1 as libc::c_int,
        useRowMatchFinder,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        ZSTD_CONTENTSIZE_UNKNOWN as U64,
        (*params).useSequenceProducer,
        (*params).maxBlockSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCParams(
    mut cParams: ZSTD_compressionParameters,
) -> size_t {
    let mut initialParams = ZSTD_makeCCtxParamsFromCParams(cParams);
    if ZSTD_rowMatchFinderSupported(cParams.strategy) != 0 {
        let mut noRowCCtxSize: size_t = 0;
        let mut rowCCtxSize: size_t = 0;
        initialParams.useRowMatchFinder = ZSTD_ps_disable;
        noRowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&mut initialParams);
        initialParams.useRowMatchFinder = ZSTD_ps_enable;
        rowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&mut initialParams);
        return if noRowCCtxSize > rowCCtxSize { noRowCCtxSize } else { rowCCtxSize };
    } else {
        return ZSTD_estimateCCtxSize_usingCCtxParams(&mut initialParams)
    };
}
static mut srcSizeTiers: [libc::c_ulonglong; 4] = [0; 4];
unsafe extern "C" fn ZSTD_estimateCCtxSize_internal(
    mut compressionLevel: libc::c_int,
) -> size_t {
    let mut tier = 0 as libc::c_int;
    let mut largestSize = 0 as libc::c_int as size_t;
    while tier < 4 as libc::c_int {
        let cParams = ZSTD_getCParams_internal(
            compressionLevel,
            srcSizeTiers[tier as usize],
            0 as libc::c_int as size_t,
            ZSTD_cpm_noAttachDict,
        );
        largestSize = if ZSTD_estimateCCtxSize_usingCParams(cParams) > largestSize {
            ZSTD_estimateCCtxSize_usingCParams(cParams)
        } else {
            largestSize
        };
        tier += 1;
    }
    return largestSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize(
    mut compressionLevel: libc::c_int,
) -> size_t {
    let mut level: libc::c_int = 0;
    let mut memBudget = 0 as libc::c_int as size_t;
    level = if compressionLevel < 1 as libc::c_int {
        compressionLevel
    } else {
        1 as libc::c_int
    };
    while level <= compressionLevel {
        let newMB = ZSTD_estimateCCtxSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
        level += 1;
    }
    return memBudget;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCCtxParams(
    mut params: *const ZSTD_CCtx_params,
) -> size_t {
    if (*params).nbWorkers > 0 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    let cParams = ZSTD_getCParamsFromCCtxParams(
        params,
        ZSTD_CONTENTSIZE_UNKNOWN as U64,
        0 as libc::c_int as size_t,
        ZSTD_cpm_noAttachDict,
    );
    let blockSize = if ZSTD_resolveMaxBlockSize((*params).maxBlockSize)
        < (1 as libc::c_int as size_t) << cParams.windowLog
    {
        ZSTD_resolveMaxBlockSize((*params).maxBlockSize)
    } else {
        (1 as libc::c_int as size_t) << cParams.windowLog
    };
    let inBuffSize = if (*params).inBufferMode as libc::c_uint
        == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {
        ((1 as libc::c_int as size_t) << cParams.windowLog).wrapping_add(blockSize)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let outBuffSize = if (*params).outBufferMode as libc::c_uint
        == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {
        (ZSTD_compressBound(blockSize)).wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        (*params).useRowMatchFinder,
        &(*params).cParams,
    );
    return ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &cParams,
        &(*params).ldmParams,
        1 as libc::c_int,
        useRowMatchFinder,
        inBuffSize,
        outBuffSize,
        ZSTD_CONTENTSIZE_UNKNOWN as U64,
        (*params).useSequenceProducer,
        (*params).maxBlockSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCParams(
    mut cParams: ZSTD_compressionParameters,
) -> size_t {
    let mut initialParams = ZSTD_makeCCtxParamsFromCParams(cParams);
    if ZSTD_rowMatchFinderSupported(cParams.strategy) != 0 {
        let mut noRowCCtxSize: size_t = 0;
        let mut rowCCtxSize: size_t = 0;
        initialParams.useRowMatchFinder = ZSTD_ps_disable;
        noRowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&mut initialParams);
        initialParams.useRowMatchFinder = ZSTD_ps_enable;
        rowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&mut initialParams);
        return if noRowCCtxSize > rowCCtxSize { noRowCCtxSize } else { rowCCtxSize };
    } else {
        return ZSTD_estimateCStreamSize_usingCCtxParams(&mut initialParams)
    };
}
unsafe extern "C" fn ZSTD_estimateCStreamSize_internal(
    mut compressionLevel: libc::c_int,
) -> size_t {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0 as libc::c_int as size_t,
        ZSTD_cpm_noAttachDict,
    );
    return ZSTD_estimateCStreamSize_usingCParams(cParams);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize(
    mut compressionLevel: libc::c_int,
) -> size_t {
    let mut level: libc::c_int = 0;
    let mut memBudget = 0 as libc::c_int as size_t;
    level = if compressionLevel < 1 as libc::c_int {
        compressionLevel
    } else {
        1 as libc::c_int
    };
    while level <= compressionLevel {
        let newMB = ZSTD_estimateCStreamSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
        level += 1;
    }
    return memBudget;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameProgression(
    mut cctx: *const ZSTD_CCtx,
) -> ZSTD_frameProgression {
    if (*cctx).appliedParams.nbWorkers > 0 as libc::c_int {
        return ZSTDMT_getFrameProgression((*cctx).mtctx);
    }
    let mut fp = ZSTD_frameProgression {
        ingested: 0,
        consumed: 0,
        produced: 0,
        flushed: 0,
        currentJobID: 0,
        nbActiveWorkers: 0,
    };
    let buffered = if ((*cctx).inBuff).is_null() {
        0 as libc::c_int as libc::c_ulong
    } else {
        ((*cctx).inBuffPos).wrapping_sub((*cctx).inToCompress)
    };
    if buffered != 0 {
        if (*cctx).inBuffPos >= (*cctx).inToCompress {} else {
            __assert_fail(
                b"cctx->inBuffPos >= cctx->inToCompress\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1823 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"ZSTD_frameProgression ZSTD_getFrameProgression(const ZSTD_CCtx *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if buffered <= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"buffered <= ZSTD_BLOCKSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1824 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"ZSTD_frameProgression ZSTD_getFrameProgression(const ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    fp.ingested = ((*cctx).consumedSrcSize).wrapping_add(buffered as libc::c_ulonglong);
    fp.consumed = (*cctx).consumedSrcSize;
    fp.produced = (*cctx).producedCSize;
    fp.flushed = (*cctx).producedCSize;
    fp.currentJobID = 0 as libc::c_int as libc::c_uint;
    fp.nbActiveWorkers = 0 as libc::c_int as libc::c_uint;
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_toFlushNow(mut cctx: *mut ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.nbWorkers > 0 as libc::c_int {
        return ZSTDMT_toFlushNow((*cctx).mtctx);
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_assertEqualCParams(
    mut cParams1: ZSTD_compressionParameters,
    mut cParams2: ZSTD_compressionParameters,
) {
    if cParams1.windowLog == cParams2.windowLog {} else {
        __assert_fail(
            b"cParams1.windowLog == cParams2.windowLog\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1853 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if cParams1.chainLog == cParams2.chainLog {} else {
        __assert_fail(
            b"cParams1.chainLog == cParams2.chainLog\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1854 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if cParams1.hashLog == cParams2.hashLog {} else {
        __assert_fail(
            b"cParams1.hashLog == cParams2.hashLog\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1855 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if cParams1.searchLog == cParams2.searchLog {} else {
        __assert_fail(
            b"cParams1.searchLog == cParams2.searchLog\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1856 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if cParams1.minMatch == cParams2.minMatch {} else {
        __assert_fail(
            b"cParams1.minMatch == cParams2.minMatch\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1857 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if cParams1.targetLength == cParams2.targetLength {} else {
        __assert_fail(
            b"cParams1.targetLength == cParams2.targetLength\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1858 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if cParams1.strategy as libc::c_uint == cParams2.strategy as libc::c_uint {} else {
        __assert_fail(
            b"cParams1.strategy == cParams2.strategy\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1859 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void ZSTD_assertEqualCParams(ZSTD_compressionParameters, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_reset_compressedBlockState(
    mut bs: *mut ZSTD_compressedBlockState_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ZSTD_REP_NUM {
        (*bs).rep[i as usize] = repStartValue[i as usize];
        i += 1;
    }
    (*bs).entropy.huf.repeatMode = HUF_repeat_none;
    (*bs).entropy.fse.offcode_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.matchlength_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.litlength_repeatMode = FSE_repeat_none;
}
unsafe extern "C" fn ZSTD_invalidateMatchState(mut ms: *mut ZSTD_matchState_t) {
    ZSTD_window_clear(&mut (*ms).window);
    (*ms).nextToUpdate = (*ms).window.dictLimit;
    (*ms).loadedDictEnd = 0 as libc::c_int as U32;
    (*ms).opt.litLengthSum = 0 as libc::c_int as U32;
    (*ms).dictMatchState = NULL as *const ZSTD_matchState_t;
}
unsafe extern "C" fn ZSTD_bitmix(mut val: U64, mut len: U64) -> U64 {
    val
        ^= ZSTD_rotateRight_U64(val, 49 as libc::c_int as U32)
            ^ ZSTD_rotateRight_U64(val, 24 as libc::c_int as U32);
    val = (val as libc::c_ulonglong)
        .wrapping_mul(0x9fb21c651e98df25 as libc::c_ulonglong) as U64 as U64;
    val ^= (val >> 35 as libc::c_int).wrapping_add(len);
    val = (val as libc::c_ulonglong)
        .wrapping_mul(0x9fb21c651e98df25 as libc::c_ulonglong) as U64 as U64;
    return val ^ val >> 28 as libc::c_int;
}
unsafe extern "C" fn ZSTD_advanceHashSalt(mut ms: *mut ZSTD_matchState_t) {
    (*ms)
        .hashSalt = ZSTD_bitmix((*ms).hashSalt, 8 as libc::c_int as U64)
        ^ ZSTD_bitmix((*ms).hashSaltEntropy as U64, 4 as libc::c_int as U64);
}
unsafe extern "C" fn ZSTD_reset_matchState(
    mut ms: *mut ZSTD_matchState_t,
    mut ws: *mut ZSTD_cwksp,
    mut cParams: *const ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_paramSwitch_e,
    crp: ZSTD_compResetPolicy_e,
    forceResetIndex: ZSTD_indexResetPolicy_e,
    forWho: ZSTD_resetTarget_e,
) -> size_t {
    let chainSize = if ZSTD_allocateChainTable(
        (*cParams).strategy,
        useRowMatchFinder,
        ((*ms).dedicatedDictSearch != 0
            && forWho as libc::c_uint
                == ZSTD_resetTarget_CDict as libc::c_int as libc::c_uint) as libc::c_int
            as U32,
    ) != 0
    {
        (1 as libc::c_int as size_t) << (*cParams).chainLog
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let hSize = (1 as libc::c_int as size_t) << (*cParams).hashLog;
    let hashLog3 = if forWho as libc::c_uint
        == ZSTD_resetTarget_CCtx as libc::c_int as libc::c_uint
        && (*cParams).minMatch == 3 as libc::c_int as libc::c_uint
    {
        if (17 as libc::c_int as libc::c_uint) < (*cParams).windowLog {
            17 as libc::c_int as libc::c_uint
        } else {
            (*cParams).windowLog
        }
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let h3Size = if hashLog3 != 0 {
        (1 as libc::c_int as size_t) << hashLog3
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if useRowMatchFinder as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"useRowMatchFinder != ZSTD_ps_auto\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1947 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 213],
                &[libc::c_char; 213],
            >(
                b"size_t ZSTD_reset_matchState(ZSTD_matchState_t *, ZSTD_cwksp *, const ZSTD_compressionParameters *, const ZSTD_paramSwitch_e, const ZSTD_compResetPolicy_e, const ZSTD_indexResetPolicy_e, const ZSTD_resetTarget_e)\0",
            ))
                .as_ptr(),
        );
    }
    if forceResetIndex as libc::c_uint == ZSTDirp_reset as libc::c_int as libc::c_uint {
        ZSTD_window_init(&mut (*ms).window);
        ZSTD_cwksp_mark_tables_dirty(ws);
    }
    (*ms).hashLog3 = hashLog3;
    (*ms).lazySkipping = 0 as libc::c_int;
    ZSTD_invalidateMatchState(ms);
    if ZSTD_cwksp_reserve_failed(ws) == 0 {} else {
        __assert_fail(
            b"!ZSTD_cwksp_reserve_failed(ws)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            1958 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 213],
                &[libc::c_char; 213],
            >(
                b"size_t ZSTD_reset_matchState(ZSTD_matchState_t *, ZSTD_cwksp *, const ZSTD_compressionParameters *, const ZSTD_paramSwitch_e, const ZSTD_compResetPolicy_e, const ZSTD_indexResetPolicy_e, const ZSTD_resetTarget_e)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_cwksp_clear_tables(ws);
    (*ms)
        .hashTable = ZSTD_cwksp_reserve_table(
        ws,
        hSize.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
    ) as *mut U32;
    (*ms)
        .chainTable = ZSTD_cwksp_reserve_table(
        ws,
        chainSize.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
    ) as *mut U32;
    (*ms)
        .hashTable3 = ZSTD_cwksp_reserve_table(
        ws,
        h3Size.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
    ) as *mut U32;
    if ZSTD_cwksp_reserve_failed(ws) != 0 {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    if crp as libc::c_uint != ZSTDcrp_leaveDirty as libc::c_int as libc::c_uint {
        ZSTD_cwksp_clean_tables(ws);
    }
    if ZSTD_rowMatchFinderUsed((*cParams).strategy, useRowMatchFinder) != 0 {
        let tagTableSize = hSize;
        if forWho as libc::c_uint == ZSTD_resetTarget_CCtx as libc::c_int as libc::c_uint
        {
            (*ms)
                .tagTable = ZSTD_cwksp_reserve_aligned_init_once(ws, tagTableSize)
                as *mut BYTE;
            ZSTD_advanceHashSalt(ms);
        } else {
            (*ms).tagTable = ZSTD_cwksp_reserve_aligned(ws, tagTableSize) as *mut BYTE;
            libc::memset(
                (*ms).tagTable as *mut libc::c_void,
                0 as libc::c_int,
                tagTableSize as libc::size_t,
            );
            (*ms).hashSalt = 0 as libc::c_int as U64;
        }
        let rowLog = if 4 as libc::c_int as libc::c_uint
            > (if (*cParams).searchLog < 6 as libc::c_int as libc::c_uint {
                (*cParams).searchLog
            } else {
                6 as libc::c_int as libc::c_uint
            })
        {
            4 as libc::c_int as libc::c_uint
        } else if (*cParams).searchLog < 6 as libc::c_int as libc::c_uint {
            (*cParams).searchLog
        } else {
            6 as libc::c_int as libc::c_uint
        };
        if (*cParams).hashLog >= rowLog {} else {
            __assert_fail(
                b"cParams->hashLog >= rowLog\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                1992 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 213],
                    &[libc::c_char; 213],
                >(
                    b"size_t ZSTD_reset_matchState(ZSTD_matchState_t *, ZSTD_cwksp *, const ZSTD_compressionParameters *, const ZSTD_paramSwitch_e, const ZSTD_compResetPolicy_e, const ZSTD_indexResetPolicy_e, const ZSTD_resetTarget_e)\0",
                ))
                    .as_ptr(),
            );
        }
        (*ms).rowHashLog = ((*cParams).hashLog).wrapping_sub(rowLog);
    }
    if forWho as libc::c_uint == ZSTD_resetTarget_CCtx as libc::c_int as libc::c_uint
        && (*cParams).strategy as libc::c_uint
            >= ZSTD_btopt as libc::c_int as libc::c_uint
    {
        (*ms)
            .opt
            .litFreq = ZSTD_cwksp_reserve_aligned(
            ws,
            (((1 as libc::c_int) << Litbits) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint;
        (*ms)
            .opt
            .litLengthFreq = ZSTD_cwksp_reserve_aligned(
            ws,
            ((MaxLL + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint;
        (*ms)
            .opt
            .matchLengthFreq = ZSTD_cwksp_reserve_aligned(
            ws,
            ((MaxML + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint;
        (*ms)
            .opt
            .offCodeFreq = ZSTD_cwksp_reserve_aligned(
            ws,
            ((MaxOff + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint;
        (*ms)
            .opt
            .matchTable = ZSTD_cwksp_reserve_aligned(
            ws,
            ((ZSTD_OPT_NUM + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<ZSTD_match_t>() as libc::c_ulong),
        ) as *mut ZSTD_match_t;
        (*ms)
            .opt
            .priceTable = ZSTD_cwksp_reserve_aligned(
            ws,
            ((ZSTD_OPT_NUM + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<ZSTD_optimal_t>() as libc::c_ulong),
        ) as *mut ZSTD_optimal_t;
    }
    (*ms).cParams = *cParams;
    if ZSTD_cwksp_reserve_failed(ws) != 0 {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    return 0 as libc::c_int as size_t;
}
pub const ZSTD_INDEXOVERFLOW_MARGIN: libc::c_int = 16 as libc::c_int
    * ((1 as libc::c_int) << 20 as libc::c_int);
unsafe extern "C" fn ZSTD_indexTooCloseToMax(mut w: ZSTD_window_t) -> libc::c_int {
    return ((w.nextSrc).offset_from(w.base) as libc::c_long as size_t
        > ((3 as libc::c_uint) << 29 as libc::c_int)
            .wrapping_add(
                (1 as libc::c_uint)
                    << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                        == 4 as libc::c_int as libc::c_ulong
                    {
                        ZSTD_WINDOWLOG_MAX_32
                    } else {
                        ZSTD_WINDOWLOG_MAX_64
                    }),
            )
            .wrapping_sub(ZSTD_INDEXOVERFLOW_MARGIN as libc::c_uint) as libc::c_ulong)
        as libc::c_int;
}
unsafe extern "C" fn ZSTD_dictTooBig(loadedDictSize: size_t) -> libc::c_int {
    return (loadedDictSize
        > (-(1 as libc::c_int) as U32)
            .wrapping_sub(
                ((3 as libc::c_uint) << 29 as libc::c_int)
                    .wrapping_add(
                        (1 as libc::c_uint)
                            << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                                == 4 as libc::c_int as libc::c_ulong
                            {
                                ZSTD_WINDOWLOG_MAX_32
                            } else {
                                ZSTD_WINDOWLOG_MAX_64
                            }),
                    ),
            ) as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn ZSTD_resetCCtx_internal(
    mut zc: *mut ZSTD_CCtx,
    mut params: *const ZSTD_CCtx_params,
    pledgedSrcSize: U64,
    loadedDictSize: size_t,
    crp: ZSTD_compResetPolicy_e,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let ws: *mut ZSTD_cwksp = &mut (*zc).workspace;
    if ERR_isError(ZSTD_checkCParams((*params).cParams)) == 0 {} else {
        __assert_fail(
            b"!ZSTD_isError(ZSTD_checkCParams(params->cParams))\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2054 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 155],
                &[libc::c_char; 155],
            >(
                b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    (*zc).isFirstBlock = 1 as libc::c_int;
    (*zc).appliedParams = *params;
    params = &mut (*zc).appliedParams;
    if (*params).useRowMatchFinder as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"params->useRowMatchFinder != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2064 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 155],
                &[libc::c_char; 155],
            >(
                b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*params).useBlockSplitter as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"params->useBlockSplitter != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2065 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 155],
                &[libc::c_char; 155],
            >(
                b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*params).ldmParams.enableLdm as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"params->ldmParams.enableLdm != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2066 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 155],
                &[libc::c_char; 155],
            >(
                b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*params).maxBlockSize != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"params->maxBlockSize != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2067 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 155],
                &[libc::c_char; 155],
            >(
                b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*params).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        ZSTD_ldm_adjustParameters(
            &mut (*zc).appliedParams.ldmParams,
            &(*params).cParams,
        );
        if (*params).ldmParams.hashLog >= (*params).ldmParams.bucketSizeLog {} else {
            __assert_fail(
                b"params->ldmParams.hashLog >= params->ldmParams.bucketSizeLog\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                2071 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 155],
                    &[libc::c_char; 155],
                >(
                    b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*params).ldmParams.hashRateLog < 32 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"params->ldmParams.hashRateLog < 32\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                2072 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 155],
                    &[libc::c_char; 155],
                >(
                    b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    let windowSize = if 1 as libc::c_int as libc::c_ulong
        > (if (1 as libc::c_int as U64) << (*params).cParams.windowLog < pledgedSrcSize {
            (1 as libc::c_int as U64) << (*params).cParams.windowLog
        } else {
            pledgedSrcSize
        })
    {
        1 as libc::c_int as libc::c_ulong
    } else if (1 as libc::c_int as U64) << (*params).cParams.windowLog < pledgedSrcSize {
        (1 as libc::c_int as U64) << (*params).cParams.windowLog
    } else {
        pledgedSrcSize
    };
    let blockSize = if (*params).maxBlockSize < windowSize {
        (*params).maxBlockSize
    } else {
        windowSize
    };
    let maxNbSeq = ZSTD_maxNbSeq(
        blockSize,
        (*params).cParams.minMatch,
        (*params).useSequenceProducer,
    );
    let buffOutSize = if zbuff as libc::c_uint
        == ZSTDb_buffered as libc::c_int as libc::c_uint
        && (*params).outBufferMode as libc::c_uint
            == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {
        (ZSTD_compressBound(blockSize)).wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let buffInSize = if zbuff as libc::c_uint
        == ZSTDb_buffered as libc::c_int as libc::c_uint
        && (*params).inBufferMode as libc::c_uint
            == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {
        windowSize.wrapping_add(blockSize)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq((*params).ldmParams, blockSize);
    let indexTooClose = ZSTD_indexTooCloseToMax((*zc).blockState.matchState.window);
    let dictTooBig = ZSTD_dictTooBig(loadedDictSize);
    let mut needsIndexReset = (if indexTooClose != 0 || dictTooBig != 0
        || (*zc).initialized == 0
    {
        ZSTDirp_reset as libc::c_int
    } else {
        ZSTDirp_continue as libc::c_int
    }) as ZSTD_indexResetPolicy_e;
    let neededSpace = ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &(*params).cParams,
        &(*params).ldmParams,
        ((*zc).staticSize != 0 as libc::c_int as libc::c_ulong) as libc::c_int,
        (*params).useRowMatchFinder,
        buffInSize,
        buffOutSize,
        pledgedSrcSize,
        (*params).useSequenceProducer,
        (*params).maxBlockSize,
    );
    let mut resizeWorkspace: libc::c_int = 0;
    let err_code = neededSpace;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if (*zc).staticSize == 0 {
        ZSTD_cwksp_bump_oversized_duration(ws, 0 as libc::c_int as size_t);
    }
    let workspaceTooSmall = (ZSTD_cwksp_sizeof(ws) < neededSpace) as libc::c_int;
    let workspaceWasteful = ZSTD_cwksp_check_wasteful(ws, neededSpace);
    resizeWorkspace = (workspaceTooSmall != 0 || workspaceWasteful != 0) as libc::c_int;
    if resizeWorkspace != 0 {
        if (*zc).staticSize != 0 {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
        needsIndexReset = ZSTDirp_reset;
        ZSTD_cwksp_free(ws, (*zc).customMem);
        let err_code_0 = ZSTD_cwksp_create(ws, neededSpace, (*zc).customMem);
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        if ZSTD_cwksp_check_available(
            ws,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<ZSTD_compressedBlockState_t>()
                        as libc::c_ulong,
                ),
        ) != 0
        {} else {
            __assert_fail(
                b"ZSTD_cwksp_check_available(ws, 2 * sizeof(ZSTD_compressedBlockState_t))\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                2124 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 155],
                    &[libc::c_char; 155],
                >(
                    b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
                ))
                    .as_ptr(),
            );
        }
        (*zc)
            .blockState
            .prevCBlock = ZSTD_cwksp_reserve_object(
            ws,
            ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong,
        ) as *mut ZSTD_compressedBlockState_t;
        if ((*zc).blockState.prevCBlock).is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
        (*zc)
            .blockState
            .nextCBlock = ZSTD_cwksp_reserve_object(
            ws,
            ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong,
        ) as *mut ZSTD_compressedBlockState_t;
        if ((*zc).blockState.nextCBlock).is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
        (*zc)
            .entropyWorkspace = ZSTD_cwksp_reserve_object(
            ws,
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
        ) as *mut U32;
        if ((*zc).entropyWorkspace).is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
    }
    ZSTD_cwksp_clear(ws);
    (*zc).blockState.matchState.cParams = (*params).cParams;
    (*zc)
        .blockState
        .matchState
        .prefetchCDictTables = ((*params).prefetchCDictTables as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint) as libc::c_int;
    (*zc)
        .pledgedSrcSizePlusOne = pledgedSrcSize
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong;
    (*zc).consumedSrcSize = 0 as libc::c_int as libc::c_ulonglong;
    (*zc).producedCSize = 0 as libc::c_int as libc::c_ulonglong;
    if pledgedSrcSize as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN {
        (*zc).appliedParams.fParams.contentSizeFlag = 0 as libc::c_int;
    }
    (*zc).blockSize = blockSize;
    ZSTD_XXH64_reset(&mut (*zc).xxhState, 0 as libc::c_int as XXH64_hash_t);
    (*zc).stage = ZSTDcs_init;
    (*zc).dictID = 0 as libc::c_int as U32;
    (*zc).dictContentSize = 0 as libc::c_int as size_t;
    ZSTD_reset_compressedBlockState((*zc).blockState.prevCBlock);
    let err_code_1 = ZSTD_reset_matchState(
        &mut (*zc).blockState.matchState,
        ws,
        &(*params).cParams,
        (*params).useRowMatchFinder,
        crp,
        needsIndexReset,
        ZSTD_resetTarget_CCtx,
    );
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    (*zc)
        .seqStore
        .sequencesStart = ZSTD_cwksp_reserve_aligned(
        ws,
        maxNbSeq.wrapping_mul(::core::mem::size_of::<seqDef>() as libc::c_ulong),
    ) as *mut seqDef;
    if (*params).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        let ldmHSize = (1 as libc::c_int as size_t) << (*params).ldmParams.hashLog;
        (*zc)
            .ldmState
            .hashTable = ZSTD_cwksp_reserve_aligned(
            ws,
            ldmHSize.wrapping_mul(::core::mem::size_of::<ldmEntry_t>() as libc::c_ulong),
        ) as *mut ldmEntry_t;
        libc::memset(
            (*zc).ldmState.hashTable as *mut libc::c_void,
            0 as libc::c_int,
            ldmHSize.wrapping_mul(::core::mem::size_of::<ldmEntry_t>() as libc::c_ulong)
                as libc::size_t,
        );
        (*zc)
            .ldmSequences = ZSTD_cwksp_reserve_aligned(
            ws,
            maxNbLdmSeq.wrapping_mul(::core::mem::size_of::<rawSeq>() as libc::c_ulong),
        ) as *mut rawSeq;
        (*zc).maxNbLdmSequences = maxNbLdmSeq;
        ZSTD_window_init(&mut (*zc).ldmState.window);
        (*zc).ldmState.loadedDictEnd = 0 as libc::c_int as U32;
    }
    if (*params).useSequenceProducer != 0 {
        let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
        (*zc).externalMatchCtx.seqBufferCapacity = maxNbExternalSeq;
        (*zc)
            .externalMatchCtx
            .seqBuffer = ZSTD_cwksp_reserve_aligned(
            ws,
            maxNbExternalSeq
                .wrapping_mul(::core::mem::size_of::<ZSTD_Sequence>() as libc::c_ulong),
        ) as *mut ZSTD_Sequence;
    }
    (*zc)
        .seqStore
        .litStart = ZSTD_cwksp_reserve_buffer(
        ws,
        blockSize.wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong),
    );
    (*zc).seqStore.maxNbLit = blockSize;
    (*zc).bufferedPolicy = zbuff;
    (*zc).inBuffSize = buffInSize;
    (*zc).inBuff = ZSTD_cwksp_reserve_buffer(ws, buffInSize) as *mut libc::c_char;
    (*zc).outBuffSize = buffOutSize;
    (*zc).outBuff = ZSTD_cwksp_reserve_buffer(ws, buffOutSize) as *mut libc::c_char;
    if (*params).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        let numBuckets = (1 as libc::c_int as size_t)
            << ((*params).ldmParams.hashLog)
                .wrapping_sub((*params).ldmParams.bucketSizeLog);
        (*zc).ldmState.bucketOffsets = ZSTD_cwksp_reserve_buffer(ws, numBuckets);
        libc::memset(
            (*zc).ldmState.bucketOffsets as *mut libc::c_void,
            0 as libc::c_int,
            numBuckets as libc::size_t,
        );
    }
    ZSTD_referenceExternalSequences(zc, NULL as *mut rawSeq, 0 as libc::c_int as size_t);
    (*zc).seqStore.maxNbSeq = maxNbSeq;
    (*zc)
        .seqStore
        .llCode = ZSTD_cwksp_reserve_buffer(
        ws,
        maxNbSeq.wrapping_mul(::core::mem::size_of::<BYTE>() as libc::c_ulong),
    );
    (*zc)
        .seqStore
        .mlCode = ZSTD_cwksp_reserve_buffer(
        ws,
        maxNbSeq.wrapping_mul(::core::mem::size_of::<BYTE>() as libc::c_ulong),
    );
    (*zc)
        .seqStore
        .ofCode = ZSTD_cwksp_reserve_buffer(
        ws,
        maxNbSeq.wrapping_mul(::core::mem::size_of::<BYTE>() as libc::c_ulong),
    );
    if ZSTD_cwksp_estimated_space_within_bounds(ws, neededSpace) != 0 {} else {
        __assert_fail(
            b"ZSTD_cwksp_estimated_space_within_bounds(ws, neededSpace)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2218 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 155],
                &[libc::c_char; 155],
            >(
                b"size_t ZSTD_resetCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx_params *, const U64, const size_t, const ZSTD_compResetPolicy_e, const ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    (*zc).initialized = 1 as libc::c_int;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_invalidateRepCodes(mut cctx: *mut ZSTD_CCtx) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ZSTD_REP_NUM {
        (*(*cctx).blockState.prevCBlock).rep[i as usize] = 0 as libc::c_int as U32;
        i += 1;
    }
    if ZSTD_window_hasExtDict((*cctx).blockState.matchState.window) == 0 {} else {
        __assert_fail(
            b"!ZSTD_window_hasExtDict(cctx->blockState.matchState.window)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2233 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void ZSTD_invalidateRepCodes(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    };
}
static mut attachDictSizeCutoffs: [size_t; 10] = [
    (8 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (8 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (8 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
    (8 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
];
unsafe extern "C" fn ZSTD_shouldAttachDict(
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
) -> libc::c_int {
    let mut cutoff = attachDictSizeCutoffs[(*cdict).matchState.cParams.strategy
        as usize];
    let dedicatedDictSearch = (*cdict).matchState.dedicatedDictSearch;
    return (dedicatedDictSearch != 0
        || (pledgedSrcSize <= cutoff
            || pledgedSrcSize as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
            || (*params).attachDictPref as libc::c_uint
                == ZSTD_dictForceAttach as libc::c_int as libc::c_uint)
            && (*params).attachDictPref as libc::c_uint
                != ZSTD_dictForceCopy as libc::c_int as libc::c_uint
            && (*params).forceWindow == 0) as libc::c_int;
}
unsafe extern "C" fn ZSTD_resetCCtx_byAttachingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let mut adjusted_cdict_cParams = (*cdict).matchState.cParams;
    let windowLog = params.cParams.windowLog;
    if windowLog != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"windowLog != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2280 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"size_t ZSTD_resetCCtx_byAttachingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cdict).matchState.dedicatedDictSearch != 0 {
        ZSTD_dedicatedDictSearch_revertCParams(&mut adjusted_cdict_cParams);
    }
    params
        .cParams = ZSTD_adjustCParams_internal(
        adjusted_cdict_cParams,
        pledgedSrcSize as libc::c_ulonglong,
        (*cdict).dictContentSize,
        ZSTD_cpm_attachDict,
        params.useRowMatchFinder,
    );
    params.cParams.windowLog = windowLog;
    params.useRowMatchFinder = (*cdict).useRowMatchFinder;
    let err_code = ZSTD_resetCCtx_internal(
        cctx,
        &mut params,
        pledgedSrcSize,
        0 as libc::c_int as size_t,
        ZSTDcrp_makeClean,
        zbuff,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if (*cctx).appliedParams.cParams.strategy as libc::c_uint
        == adjusted_cdict_cParams.strategy as libc::c_uint
    {} else {
        __assert_fail(
            b"cctx->appliedParams.cParams.strategy == adjusted_cdict_cParams.strategy\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2297 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"size_t ZSTD_resetCCtx_byAttachingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    let cdictEnd = ((*cdict).matchState.window.nextSrc)
        .offset_from((*cdict).matchState.window.base) as libc::c_long as U32;
    let cdictLen = cdictEnd.wrapping_sub((*cdict).matchState.window.dictLimit);
    if !(cdictLen == 0 as libc::c_int as libc::c_uint) {
        (*cctx).blockState.matchState.dictMatchState = &(*cdict).matchState;
        if (*cctx).blockState.matchState.window.dictLimit < cdictEnd {
            (*cctx)
                .blockState
                .matchState
                .window
                .nextSrc = ((*cctx).blockState.matchState.window.base)
                .offset(cdictEnd as isize);
            ZSTD_window_clear(&mut (*cctx).blockState.matchState.window);
        }
        (*cctx)
            .blockState
            .matchState
            .loadedDictEnd = (*cctx).blockState.matchState.window.dictLimit;
    }
    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;
    libc::memcpy(
        (*cctx).blockState.prevCBlock as *mut libc::c_void,
        &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t
            as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong
            as libc::size_t,
    );
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_copyCDictTableIntoCCtx(
    mut dst: *mut U32,
    mut src: *const U32,
    mut tableSize: size_t,
    mut cParams: *const ZSTD_compressionParameters,
) {
    if ZSTD_CDictIndicesAreTagged(cParams) != 0 {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < tableSize {
            let taggedIndex = *src.offset(i as isize);
            let index = taggedIndex >> ZSTD_SHORT_CACHE_TAG_BITS;
            *dst.offset(i as isize) = index;
            i = i.wrapping_add(1);
        }
    } else {
        libc::memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            tableSize.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong)
                as libc::size_t,
        );
    };
}
unsafe extern "C" fn ZSTD_resetCCtx_byCopyingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let mut cdict_cParams: *const ZSTD_compressionParameters = &(*cdict)
        .matchState
        .cParams;
    if (*cdict).matchState.dedicatedDictSearch == 0 {} else {
        __assert_fail(
            b"!cdict->matchState.dedicatedDictSearch\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2354 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    let windowLog = params.cParams.windowLog;
    if windowLog != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"windowLog != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2359 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    params.cParams = *cdict_cParams;
    params.cParams.windowLog = windowLog;
    params.useRowMatchFinder = (*cdict).useRowMatchFinder;
    let err_code = ZSTD_resetCCtx_internal(
        cctx,
        &mut params,
        pledgedSrcSize,
        0 as libc::c_int as size_t,
        ZSTDcrp_leaveDirty,
        zbuff,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if (*cctx).appliedParams.cParams.strategy as libc::c_uint
        == (*cdict_cParams).strategy as libc::c_uint
    {} else {
        __assert_fail(
            b"cctx->appliedParams.cParams.strategy == cdict_cParams->strategy\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2367 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cctx).appliedParams.cParams.hashLog == (*cdict_cParams).hashLog {} else {
        __assert_fail(
            b"cctx->appliedParams.cParams.hashLog == cdict_cParams->hashLog\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2368 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cctx).appliedParams.cParams.chainLog == (*cdict_cParams).chainLog {} else {
        __assert_fail(
            b"cctx->appliedParams.cParams.chainLog == cdict_cParams->chainLog\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2369 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_cwksp_mark_tables_dirty(&mut (*cctx).workspace);
    if params.useRowMatchFinder as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"params.useRowMatchFinder != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2373 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    let chainSize = if ZSTD_allocateChainTable(
        (*cdict_cParams).strategy,
        (*cdict).useRowMatchFinder,
        0 as libc::c_int as U32,
    ) != 0
    {
        (1 as libc::c_int as size_t) << (*cdict_cParams).chainLog
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let hSize = (1 as libc::c_int as size_t) << (*cdict_cParams).hashLog;
    ZSTD_copyCDictTableIntoCCtx(
        (*cctx).blockState.matchState.hashTable,
        (*cdict).matchState.hashTable,
        hSize,
        cdict_cParams,
    );
    if ZSTD_allocateChainTable(
        (*cctx).appliedParams.cParams.strategy,
        (*cctx).appliedParams.useRowMatchFinder,
        0 as libc::c_int as U32,
    ) != 0
    {
        ZSTD_copyCDictTableIntoCCtx(
            (*cctx).blockState.matchState.chainTable,
            (*cdict).matchState.chainTable,
            chainSize,
            cdict_cParams,
        );
    }
    if ZSTD_rowMatchFinderUsed((*cdict_cParams).strategy, (*cdict).useRowMatchFinder)
        != 0
    {
        let tagTableSize = hSize;
        libc::memcpy(
            (*cctx).blockState.matchState.tagTable as *mut libc::c_void,
            (*cdict).matchState.tagTable as *const libc::c_void,
            tagTableSize as libc::size_t,
        );
        (*cctx).blockState.matchState.hashSalt = (*cdict).matchState.hashSalt;
    }
    let h3log = (*cctx).blockState.matchState.hashLog3 as libc::c_int;
    let h3Size = if h3log != 0 {
        (1 as libc::c_int as size_t) << h3log
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if (*cdict).matchState.hashLog3 == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"cdict->matchState.hashLog3 == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2404 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_resetCCtx_byCopyingCDict(ZSTD_CCtx *, const ZSTD_CDict *, ZSTD_CCtx_params, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    libc::memset(
        (*cctx).blockState.matchState.hashTable3 as *mut libc::c_void,
        0 as libc::c_int,
        h3Size.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong)
            as libc::size_t,
    );
    ZSTD_cwksp_mark_tables_clean(&mut (*cctx).workspace);
    let mut srcMatchState: *const ZSTD_matchState_t = &(*cdict).matchState;
    let mut dstMatchState: *mut ZSTD_matchState_t = &mut (*cctx).blockState.matchState;
    (*dstMatchState).window = (*srcMatchState).window;
    (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
    (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;
    libc::memcpy(
        (*cctx).blockState.prevCBlock as *mut libc::c_void,
        &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t
            as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong
            as libc::size_t,
    );
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_resetCCtx_usingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    if ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) != 0 {
        return ZSTD_resetCCtx_byAttachingCDict(
            cctx,
            cdict,
            *params,
            pledgedSrcSize,
            zbuff,
        )
    } else {
        return ZSTD_resetCCtx_byCopyingCDict(cctx, cdict, *params, pledgedSrcSize, zbuff)
    };
}
unsafe extern "C" fn ZSTD_copyCCtx_internal(
    mut dstCCtx: *mut ZSTD_CCtx,
    mut srcCCtx: *const ZSTD_CCtx,
    mut fParams: ZSTD_frameParameters,
    mut pledgedSrcSize: U64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    if (*srcCCtx).stage as libc::c_uint != ZSTDcs_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    libc::memcpy(
        &mut (*dstCCtx).customMem as *mut ZSTD_customMem as *mut libc::c_void,
        &(*srcCCtx).customMem as *const ZSTD_customMem as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_customMem>() as libc::c_ulong as libc::size_t,
    );
    let mut params = (*dstCCtx).requestedParams;
    params.cParams = (*srcCCtx).appliedParams.cParams;
    if (*srcCCtx).appliedParams.useRowMatchFinder as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"srcCCtx->appliedParams.useRowMatchFinder != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2469 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*srcCCtx).appliedParams.useBlockSplitter as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"srcCCtx->appliedParams.useBlockSplitter != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2470 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*srcCCtx).appliedParams.ldmParams.enableLdm as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"srcCCtx->appliedParams.ldmParams.enableLdm != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2471 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    params.useRowMatchFinder = (*srcCCtx).appliedParams.useRowMatchFinder;
    params.useBlockSplitter = (*srcCCtx).appliedParams.useBlockSplitter;
    params.ldmParams = (*srcCCtx).appliedParams.ldmParams;
    params.fParams = fParams;
    params.maxBlockSize = (*srcCCtx).appliedParams.maxBlockSize;
    ZSTD_resetCCtx_internal(
        dstCCtx,
        &mut params,
        pledgedSrcSize,
        0 as libc::c_int as size_t,
        ZSTDcrp_leaveDirty,
        zbuff,
    );
    if (*dstCCtx).appliedParams.cParams.windowLog
        == (*srcCCtx).appliedParams.cParams.windowLog
    {} else {
        __assert_fail(
            b"dstCCtx->appliedParams.cParams.windowLog == srcCCtx->appliedParams.cParams.windowLog\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2480 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*dstCCtx).appliedParams.cParams.strategy as libc::c_uint
        == (*srcCCtx).appliedParams.cParams.strategy as libc::c_uint
    {} else {
        __assert_fail(
            b"dstCCtx->appliedParams.cParams.strategy == srcCCtx->appliedParams.cParams.strategy\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2481 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*dstCCtx).appliedParams.cParams.hashLog
        == (*srcCCtx).appliedParams.cParams.hashLog
    {} else {
        __assert_fail(
            b"dstCCtx->appliedParams.cParams.hashLog == srcCCtx->appliedParams.cParams.hashLog\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2482 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*dstCCtx).appliedParams.cParams.chainLog
        == (*srcCCtx).appliedParams.cParams.chainLog
    {} else {
        __assert_fail(
            b"dstCCtx->appliedParams.cParams.chainLog == srcCCtx->appliedParams.cParams.chainLog\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2483 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if (*dstCCtx).blockState.matchState.hashLog3
        == (*srcCCtx).blockState.matchState.hashLog3
    {} else {
        __assert_fail(
            b"dstCCtx->blockState.matchState.hashLog3 == srcCCtx->blockState.matchState.hashLog3\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2484 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"size_t ZSTD_copyCCtx_internal(ZSTD_CCtx *, const ZSTD_CCtx *, ZSTD_frameParameters, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_cwksp_mark_tables_dirty(&mut (*dstCCtx).workspace);
    let chainSize = if ZSTD_allocateChainTable(
        (*srcCCtx).appliedParams.cParams.strategy,
        (*srcCCtx).appliedParams.useRowMatchFinder,
        0 as libc::c_int as U32,
    ) != 0
    {
        (1 as libc::c_int as size_t) << (*srcCCtx).appliedParams.cParams.chainLog
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let hSize = (1 as libc::c_int as size_t) << (*srcCCtx).appliedParams.cParams.hashLog;
    let h3log = (*srcCCtx).blockState.matchState.hashLog3 as libc::c_int;
    let h3Size = if h3log != 0 {
        (1 as libc::c_int as size_t) << h3log
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    libc::memcpy(
        (*dstCCtx).blockState.matchState.hashTable as *mut libc::c_void,
        (*srcCCtx).blockState.matchState.hashTable as *const libc::c_void,
        hSize.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong)
            as libc::size_t,
    );
    libc::memcpy(
        (*dstCCtx).blockState.matchState.chainTable as *mut libc::c_void,
        (*srcCCtx).blockState.matchState.chainTable as *const libc::c_void,
        chainSize.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong)
            as libc::size_t,
    );
    libc::memcpy(
        (*dstCCtx).blockState.matchState.hashTable3 as *mut libc::c_void,
        (*srcCCtx).blockState.matchState.hashTable3 as *const libc::c_void,
        h3Size.wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong)
            as libc::size_t,
    );
    ZSTD_cwksp_mark_tables_clean(&mut (*dstCCtx).workspace);
    let mut srcMatchState: *const ZSTD_matchState_t = &(*srcCCtx).blockState.matchState;
    let mut dstMatchState: *mut ZSTD_matchState_t = &mut (*dstCCtx)
        .blockState
        .matchState;
    (*dstMatchState).window = (*srcMatchState).window;
    (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
    (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*dstCCtx).dictID = (*srcCCtx).dictID;
    (*dstCCtx).dictContentSize = (*srcCCtx).dictContentSize;
    libc::memcpy(
        (*dstCCtx).blockState.prevCBlock as *mut libc::c_void,
        (*srcCCtx).blockState.prevCBlock as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::c_ulong
            as libc::size_t,
    );
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyCCtx(
    mut dstCCtx: *mut ZSTD_CCtx,
    mut srcCCtx: *const ZSTD_CCtx,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    let mut fParams = {
        let mut init = ZSTD_frameParameters {
            contentSizeFlag: 1 as libc::c_int,
            checksumFlag: 0 as libc::c_int,
            noDictIDFlag: 0 as libc::c_int,
        };
        init
    };
    let zbuff = (*srcCCtx).bufferedPolicy;
    if pledgedSrcSize == 0 as libc::c_int as libc::c_ulonglong {
        pledgedSrcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    fParams
        .contentSizeFlag = (pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN) as libc::c_int;
    return ZSTD_copyCCtx_internal(
        dstCCtx,
        srcCCtx,
        fParams,
        pledgedSrcSize as U64,
        zbuff,
    );
}
pub const ZSTD_ROWSIZE: libc::c_int = 16 as libc::c_int;
#[inline(always)]
unsafe extern "C" fn ZSTD_reduceTable_internal(
    table: *mut U32,
    size: U32,
    reducerValue: U32,
    preserveMark: libc::c_int,
) {
    let nbRows = size as libc::c_int / ZSTD_ROWSIZE;
    let mut cellNb = 0 as libc::c_int;
    let mut rowNb: libc::c_int = 0;
    let reducerThreshold = reducerValue
        .wrapping_add(ZSTD_WINDOW_START_INDEX as libc::c_uint);
    if size & (16 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"(size & (ZSTD_ROWSIZE-1)) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2563 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void ZSTD_reduceTable_internal(U32 *const, const U32, const U32, const int)\0",
            ))
                .as_ptr(),
        );
    }
    if size < (1 as libc::c_uint) << 31 as libc::c_int {} else {
        __assert_fail(
            b"size < (1U<<31)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2564 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void ZSTD_reduceTable_internal(U32 *const, const U32, const U32, const int)\0",
            ))
                .as_ptr(),
        );
    }
    rowNb = 0 as libc::c_int;
    while rowNb < nbRows {
        let mut column: libc::c_int = 0;
        column = 0 as libc::c_int;
        while column < ZSTD_ROWSIZE {
            let mut newVal: U32 = 0;
            if preserveMark != 0
                && *table.offset(cellNb as isize)
                    == ZSTD_DUBT_UNSORTED_MARK as libc::c_uint
            {
                newVal = ZSTD_DUBT_UNSORTED_MARK as U32;
            } else if *table.offset(cellNb as isize) < reducerThreshold {
                newVal = 0 as libc::c_int as U32;
            } else {
                newVal = (*table.offset(cellNb as isize)).wrapping_sub(reducerValue);
            }
            *table.offset(cellNb as isize) = newVal;
            cellNb += 1;
            column += 1;
        }
        rowNb += 1;
    }
}
unsafe extern "C" fn ZSTD_reduceTable(table: *mut U32, size: U32, reducerValue: U32) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 0 as libc::c_int);
}
unsafe extern "C" fn ZSTD_reduceTable_btlazy2(
    table: *mut U32,
    size: U32,
    reducerValue: U32,
) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 1 as libc::c_int);
}
unsafe extern "C" fn ZSTD_reduceIndex(
    mut ms: *mut ZSTD_matchState_t,
    mut params: *const ZSTD_CCtx_params,
    reducerValue: U32,
) {
    let hSize = (1 as libc::c_int as U32) << (*params).cParams.hashLog;
    ZSTD_reduceTable((*ms).hashTable, hSize, reducerValue);
    if ZSTD_allocateChainTable(
        (*params).cParams.strategy,
        (*params).useRowMatchFinder,
        (*ms).dedicatedDictSearch as U32,
    ) != 0
    {
        let chainSize = (1 as libc::c_int as U32) << (*params).cParams.chainLog;
        if (*params).cParams.strategy as libc::c_uint
            == ZSTD_btlazy2 as libc::c_int as libc::c_uint
        {
            ZSTD_reduceTable_btlazy2((*ms).chainTable, chainSize, reducerValue);
        } else {
            ZSTD_reduceTable((*ms).chainTable, chainSize, reducerValue);
        }
    }
    if (*ms).hashLog3 != 0 {
        let h3Size = (1 as libc::c_int as U32) << (*ms).hashLog3;
        ZSTD_reduceTable((*ms).hashTable3, h3Size, reducerValue);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_seqToCodes(
    mut seqStorePtr: *const seqStore_t,
) -> libc::c_int {
    let sequences: *const seqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable = (*seqStorePtr).llCode;
    let ofCodeTable = (*seqStorePtr).ofCode;
    let mlCodeTable = (*seqStorePtr).mlCode;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as libc::c_long as U32;
    let mut u: U32 = 0;
    let mut longOffsets = 0 as libc::c_int;
    if nbSeq as libc::c_ulong <= (*seqStorePtr).maxNbSeq {} else {
        __assert_fail(
            b"nbSeq <= seqStorePtr->maxNbSeq\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2645 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"int ZSTD_seqToCodes(const seqStore_t *)\0"))
                .as_ptr(),
        );
    }
    u = 0 as libc::c_int as U32;
    while u < nbSeq {
        let llv = (*sequences.offset(u as isize)).litLength as U32;
        let ofCode = ZSTD_highbit32((*sequences.offset(u as isize)).offBase);
        let mlv = (*sequences.offset(u as isize)).mlBase as U32;
        *llCodeTable.offset(u as isize) = ZSTD_LLcode(llv) as BYTE;
        *ofCodeTable.offset(u as isize) = ofCode as BYTE;
        *mlCodeTable.offset(u as isize) = ZSTD_MLcode(mlv) as BYTE;
        if !(MEM_64bits() != 0
            && ofCode
                >= (if MEM_32bits() != 0 {
                    25 as libc::c_int
                } else {
                    57 as libc::c_int
                }) as U32)
        {} else {
            __assert_fail(
                b"!(MEM_64bits() && ofCode >= STREAM_ACCUMULATOR_MIN)\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                2653 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"int ZSTD_seqToCodes(const seqStore_t *)\0"))
                    .as_ptr(),
            );
        }
        if MEM_32bits() != 0
            && ofCode
                >= (if MEM_32bits() != 0 {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as U32
        {
            longOffsets = 1 as libc::c_int;
        }
        u = u.wrapping_add(1);
    }
    if (*seqStorePtr).longLengthType as libc::c_uint
        == ZSTD_llt_literalLength as libc::c_int as libc::c_uint
    {
        *llCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxLL as BYTE;
    }
    if (*seqStorePtr).longLengthType as libc::c_uint
        == ZSTD_llt_matchLength as libc::c_int as libc::c_uint
    {
        *mlCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxML as BYTE;
    }
    return longOffsets;
}
unsafe extern "C" fn ZSTD_useTargetCBlockSize(
    mut cctxParams: *const ZSTD_CCtx_params,
) -> libc::c_int {
    return ((*cctxParams).targetCBlockSize != 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
unsafe extern "C" fn ZSTD_blockSplitterEnabled(
    mut cctxParams: *mut ZSTD_CCtx_params,
) -> libc::c_int {
    if (*cctxParams).useBlockSplitter as libc::c_uint
        != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"cctxParams->useBlockSplitter != ZSTD_ps_auto\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2682 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int ZSTD_blockSplitterEnabled(ZSTD_CCtx_params *)\0"))
                .as_ptr(),
        );
    }
    return ((*cctxParams).useBlockSplitter as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_buildSequencesStatistics(
    mut seqStorePtr: *const seqStore_t,
    mut nbSeq: size_t,
    mut prevEntropy: *const ZSTD_fseCTables_t,
    mut nextEntropy: *mut ZSTD_fseCTables_t,
    mut dst: *mut BYTE,
    dstEnd: *const BYTE,
    mut strategy: ZSTD_strategy,
    mut countWorkspace: *mut libc::c_uint,
    mut entropyWorkspace: *mut libc::c_void,
    mut entropyWkspSize: size_t,
) -> ZSTD_symbolEncodingTypeStats_t {
    let ostart = dst;
    let oend = dstEnd;
    let mut op = ostart;
    let mut CTable_LitLength = ((*nextEntropy).litlengthCTable).as_mut_ptr();
    let mut CTable_OffsetBits = ((*nextEntropy).offcodeCTable).as_mut_ptr();
    let mut CTable_MatchLength = ((*nextEntropy).matchlengthCTable).as_mut_ptr();
    let ofCodeTable: *const BYTE = (*seqStorePtr).ofCode;
    let llCodeTable: *const BYTE = (*seqStorePtr).llCode;
    let mlCodeTable: *const BYTE = (*seqStorePtr).mlCode;
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        LLtype: 0,
        Offtype: 0,
        MLtype: 0,
        size: 0,
        lastCountSize: 0,
        longOffsets: 0,
    };
    stats.lastCountSize = 0 as libc::c_int as size_t;
    stats.longOffsets = ZSTD_seqToCodes(seqStorePtr);
    if op <= oend as *mut BYTE {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2727 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if nbSeq != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nbSeq != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2728 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mut max = MaxLL as libc::c_uint;
    let mostFrequent = HIST_countFast_wksp(
        countWorkspace,
        &mut max,
        llCodeTable as *const libc::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    (*nextEntropy).litlength_repeatMode = (*prevEntropy).litlength_repeatMode;
    stats
        .LLtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).litlength_repeatMode,
        countWorkspace,
        max,
        mostFrequent,
        nbSeq,
        LLFSELog as libc::c_uint,
        ((*prevEntropy).litlengthCTable).as_ptr(),
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        ZSTD_defaultAllowed,
        strategy,
    ) as U32;
    if (set_basic as libc::c_int) < set_compressed as libc::c_int
        && (set_rle as libc::c_int) < set_compressed as libc::c_int
    {} else {
        __assert_fail(
            b"set_basic < set_compressed && set_rle < set_compressed\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2739 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !(stats.LLtype < set_compressed as libc::c_int as libc::c_uint
        && (*nextEntropy).litlength_repeatMode as libc::c_uint
            != FSE_repeat_none as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"!(stats.LLtype < set_compressed && nextEntropy->litlength_repeatMode != FSE_repeat_none)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2740 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let countSize = ZSTD_buildCTable(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        CTable_LitLength,
        LLFSELog as U32,
        stats.LLtype as symbolEncodingType_e,
        countWorkspace,
        max,
        llCodeTable,
        nbSeq,
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        MaxLL as U32,
        ((*prevEntropy).litlengthCTable).as_ptr(),
        ::core::mem::size_of::<[FSE_CTable; 329]>() as libc::c_ulong,
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize) != 0 {
        stats.size = countSize;
        return stats;
    }
    if stats.LLtype == set_compressed as libc::c_int as libc::c_uint {
        stats.lastCountSize = countSize;
    }
    op = op.offset(countSize as isize);
    if op <= oend as *mut BYTE {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2757 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mut max_0 = MaxOff as libc::c_uint;
    let mostFrequent_0 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_0,
        ofCodeTable as *const libc::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    let defaultPolicy = (if max_0 <= DefaultMaxOff as libc::c_uint {
        ZSTD_defaultAllowed as libc::c_int
    } else {
        ZSTD_defaultDisallowed as libc::c_int
    }) as ZSTD_defaultPolicy_e;
    (*nextEntropy).offcode_repeatMode = (*prevEntropy).offcode_repeatMode;
    stats
        .Offtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).offcode_repeatMode,
        countWorkspace,
        max_0,
        mostFrequent_0,
        nbSeq,
        OffFSELog as libc::c_uint,
        ((*prevEntropy).offcodeCTable).as_ptr(),
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        defaultPolicy,
        strategy,
    ) as U32;
    if !(stats.Offtype < set_compressed as libc::c_int as libc::c_uint
        && (*nextEntropy).offcode_repeatMode as libc::c_uint
            != FSE_repeat_none as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"!(stats.Offtype < set_compressed && nextEntropy->offcode_repeatMode != FSE_repeat_none)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2772 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let countSize_0 = ZSTD_buildCTable(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        CTable_OffsetBits,
        OffFSELog as U32,
        stats.Offtype as symbolEncodingType_e,
        countWorkspace,
        max_0,
        ofCodeTable,
        nbSeq,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        DefaultMaxOff as U32,
        ((*prevEntropy).offcodeCTable).as_ptr(),
        ::core::mem::size_of::<[FSE_CTable; 193]>() as libc::c_ulong,
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_0) != 0 {
        stats.size = countSize_0;
        return stats;
    }
    if stats.Offtype == set_compressed as libc::c_int as libc::c_uint {
        stats.lastCountSize = countSize_0;
    }
    op = op.offset(countSize_0 as isize);
    if op <= oend as *mut BYTE {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2789 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mut max_1 = MaxML as libc::c_uint;
    let mostFrequent_1 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_1,
        mlCodeTable as *const libc::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    (*nextEntropy).matchlength_repeatMode = (*prevEntropy).matchlength_repeatMode;
    stats
        .MLtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).matchlength_repeatMode,
        countWorkspace,
        max_1,
        mostFrequent_1,
        nbSeq,
        MLFSELog as libc::c_uint,
        ((*prevEntropy).matchlengthCTable).as_ptr(),
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        ZSTD_defaultAllowed,
        strategy,
    ) as U32;
    if !(stats.MLtype < set_compressed as libc::c_int as libc::c_uint
        && (*nextEntropy).matchlength_repeatMode as libc::c_uint
            != FSE_repeat_none as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"!(stats.MLtype < set_compressed && nextEntropy->matchlength_repeatMode != FSE_repeat_none)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2802 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let countSize_1 = ZSTD_buildCTable(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        CTable_MatchLength,
        MLFSELog as U32,
        stats.MLtype as symbolEncodingType_e,
        countWorkspace,
        max_1,
        mlCodeTable,
        nbSeq,
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        MaxML as U32,
        ((*prevEntropy).matchlengthCTable).as_ptr(),
        ::core::mem::size_of::<[FSE_CTable; 363]>() as libc::c_ulong,
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_1) != 0 {
        stats.size = countSize_1;
        return stats;
    }
    if stats.MLtype == set_compressed as libc::c_int as libc::c_uint {
        stats.lastCountSize = countSize_1;
    }
    op = op.offset(countSize_1 as isize);
    if op <= oend as *mut BYTE {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2819 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 211],
                &[libc::c_char; 211],
            >(
                b"ZSTD_symbolEncodingTypeStats_t ZSTD_buildSequencesStatistics(const seqStore_t *, size_t, const ZSTD_fseCTables_t *, ZSTD_fseCTables_t *, BYTE *, const BYTE *const, ZSTD_strategy, unsigned int *, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    stats.size = op.offset_from(ostart) as libc::c_long as size_t;
    return stats;
}
pub const SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO: libc::c_int = 20 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_entropyCompressSeqStore_internal(
    mut seqStorePtr: *const seqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut entropyWorkspace: *mut libc::c_void,
    mut entropyWkspSize: size_t,
    bmi2: libc::c_int,
) -> size_t {
    let strategy = (*cctxParams).cParams.strategy;
    let mut count = entropyWorkspace as *mut libc::c_uint;
    let mut CTable_LitLength = ((*nextEntropy).fse.litlengthCTable).as_mut_ptr();
    let mut CTable_OffsetBits = ((*nextEntropy).fse.offcodeCTable).as_mut_ptr();
    let mut CTable_MatchLength = ((*nextEntropy).fse.matchlengthCTable).as_mut_ptr();
    let sequences: *const seqDef = (*seqStorePtr).sequencesStart;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as libc::c_long as size_t;
    let ofCodeTable: *const BYTE = (*seqStorePtr).ofCode;
    let llCodeTable: *const BYTE = (*seqStorePtr).llCode;
    let mlCodeTable: *const BYTE = (*seqStorePtr).mlCode;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut lastCountSize: size_t = 0;
    let mut longOffsets = 0 as libc::c_int;
    entropyWorkspace = count
        .offset(
            ((if 35 as libc::c_int > 52 as libc::c_int {
                35 as libc::c_int
            } else {
                52 as libc::c_int
            }) + 1 as libc::c_int) as isize,
        ) as *mut libc::c_void;
    entropyWkspSize = (entropyWkspSize as libc::c_ulong)
        .wrapping_sub(
            (((if 35 as libc::c_int > 52 as libc::c_int {
                35 as libc::c_int
            } else {
                52 as libc::c_int
            }) + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as size_t as size_t;
    if entropyWkspSize
        >= (((8 as libc::c_int) << 10 as libc::c_int) + 512 as libc::c_int)
            as libc::c_ulong
    {} else {
        __assert_fail(
            b"entropyWkspSize >= HUF_WORKSPACE_SIZE\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2861 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 190],
                &[libc::c_char; 190],
            >(
                b"size_t ZSTD_entropyCompressSeqStore_internal(const seqStore_t *, const ZSTD_entropyCTables_t *, ZSTD_entropyCTables_t *, const ZSTD_CCtx_params *, void *, size_t, void *, size_t, const int)\0",
            ))
                .as_ptr(),
        );
    }
    let literals: *const BYTE = (*seqStorePtr).litStart;
    let numSequences = ((*seqStorePtr).sequences)
        .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as size_t;
    let numLiterals = ((*seqStorePtr).lit).offset_from((*seqStorePtr).litStart)
        as libc::c_long as size_t;
    let suspectUncompressible = (numSequences == 0 as libc::c_int as libc::c_ulong
        || numLiterals.wrapping_div(numSequences)
            >= SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO as libc::c_ulong) as libc::c_int
        as libc::c_uint;
    let litSize = ((*seqStorePtr).lit).offset_from(literals) as libc::c_long as size_t;
    let cSize = ZSTD_compressLiterals(
        op as *mut libc::c_void,
        dstCapacity,
        literals as *const libc::c_void,
        litSize,
        entropyWorkspace,
        entropyWkspSize,
        &(*prevEntropy).huf,
        &mut (*nextEntropy).huf,
        (*cctxParams).cParams.strategy,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        suspectUncompressible as libc::c_int,
        bmi2,
    );
    let err_code = cSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if cSize <= dstCapacity {} else {
        __assert_fail(
            b"cSize <= dstCapacity\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2880 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 190],
                &[libc::c_char; 190],
            >(
                b"size_t ZSTD_entropyCompressSeqStore_internal(const seqStore_t *, const ZSTD_entropyCTables_t *, ZSTD_entropyCTables_t *, const ZSTD_CCtx_params *, void *, size_t, void *, size_t, const int)\0",
            ))
                .as_ptr(),
        );
    }
    op = op.offset(cSize as isize);
    if (oend.offset_from(op) as libc::c_long)
        < (3 as libc::c_int + 1 as libc::c_int) as libc::c_long
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if nbSeq < 128 as libc::c_int as libc::c_ulong {
        let fresh2 = op;
        op = op.offset(1);
        *fresh2 = nbSeq as BYTE;
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
    if op <= oend {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2898 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 190],
                &[libc::c_char; 190],
            >(
                b"size_t ZSTD_entropyCompressSeqStore_internal(const seqStore_t *, const ZSTD_entropyCTables_t *, ZSTD_entropyCTables_t *, const ZSTD_CCtx_params *, void *, size_t, void *, size_t, const int)\0",
            ))
                .as_ptr(),
        );
    }
    if nbSeq == 0 as libc::c_int as libc::c_ulong {
        libc::memcpy(
            &mut (*nextEntropy).fse as *mut ZSTD_fseCTables_t as *mut libc::c_void,
            &(*prevEntropy).fse as *const ZSTD_fseCTables_t as *const libc::c_void,
            ::core::mem::size_of::<ZSTD_fseCTables_t>() as libc::c_ulong as libc::size_t,
        );
        return op.offset_from(ostart) as libc::c_long as size_t;
    }
    let fresh3 = op;
    op = op.offset(1);
    let seqHead = fresh3;
    let stats = ZSTD_buildSequencesStatistics(
        seqStorePtr,
        nbSeq,
        &(*prevEntropy).fse,
        &mut (*nextEntropy).fse,
        op,
        oend,
        strategy,
        count,
        entropyWorkspace,
        entropyWkspSize,
    );
    let err_code_0 = stats.size;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    *seqHead = (stats.LLtype << 6 as libc::c_int)
        .wrapping_add(stats.Offtype << 4 as libc::c_int)
        .wrapping_add(stats.MLtype << 2 as libc::c_int) as BYTE;
    lastCountSize = stats.lastCountSize;
    op = op.offset(stats.size as isize);
    longOffsets = stats.longOffsets;
    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
        bmi2,
    );
    let err_code_1 = bitstreamSize;
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    op = op.offset(bitstreamSize as isize);
    if op <= oend {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2928 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 190],
                &[libc::c_char; 190],
            >(
                b"size_t ZSTD_entropyCompressSeqStore_internal(const seqStore_t *, const ZSTD_entropyCTables_t *, ZSTD_entropyCTables_t *, const ZSTD_CCtx_params *, void *, size_t, void *, size_t, const int)\0",
            ))
                .as_ptr(),
        );
    }
    if lastCountSize != 0
        && lastCountSize.wrapping_add(bitstreamSize) < 4 as libc::c_int as libc::c_ulong
    {
        if lastCountSize.wrapping_add(bitstreamSize) == 3 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"lastCountSize + bitstreamSize == 3\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                2939 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 190],
                    &[libc::c_char; 190],
                >(
                    b"size_t ZSTD_entropyCompressSeqStore_internal(const seqStore_t *, const ZSTD_entropyCTables_t *, ZSTD_entropyCTables_t *, const ZSTD_CCtx_params *, void *, size_t, void *, size_t, const int)\0",
                ))
                    .as_ptr(),
            );
        }
        return 0 as libc::c_int as size_t;
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_entropyCompressSeqStore(
    mut seqStorePtr: *const seqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut srcSize: size_t,
    mut entropyWorkspace: *mut libc::c_void,
    mut entropyWkspSize: size_t,
    mut bmi2: libc::c_int,
) -> size_t {
    let cSize = ZSTD_entropyCompressSeqStore_internal(
        seqStorePtr,
        prevEntropy,
        nextEntropy,
        cctxParams,
        dst,
        dstCapacity,
        entropyWorkspace,
        entropyWkspSize,
        bmi2,
    );
    if cSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    if (cSize == -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t) as libc::c_int
        & (srcSize <= dstCapacity) as libc::c_int != 0
    {
        return 0 as libc::c_int as size_t;
    }
    let err_code = cSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let maxCSize = srcSize
        .wrapping_sub(ZSTD_minGain(srcSize, (*cctxParams).cParams.strategy));
    if cSize >= maxCSize {
        return 0 as libc::c_int as size_t;
    }
    if cSize < ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"cSize < ZSTD_BLOCKSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            2983 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 183],
                &[libc::c_char; 183],
            >(
                b"size_t ZSTD_entropyCompressSeqStore(const seqStore_t *, const ZSTD_entropyCTables_t *, ZSTD_entropyCTables_t *, const ZSTD_CCtx_params *, void *, size_t, size_t, void *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    return cSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_selectBlockCompressor(
    mut strat: ZSTD_strategy,
    mut useRowMatchFinder: ZSTD_paramSwitch_e,
    mut dictMode: ZSTD_dictMode_e,
) -> ZSTD_blockCompressor {
    static mut blockCompressor: [[ZSTD_blockCompressor; 10]; 4] = unsafe {
        [
            [
                Some(
                    ZSTD_compressBlock_fast
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_fast
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_doubleFast
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_greedy
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy2
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btlazy2
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btopt
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btultra
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btultra2
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
            ],
            [
                Some(
                    ZSTD_compressBlock_fast_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_fast_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_doubleFast_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_greedy_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy2_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btlazy2_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btopt_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btultra_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btultra_extDict
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
            ],
            [
                Some(
                    ZSTD_compressBlock_fast_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_fast_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_doubleFast_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_greedy_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy2_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btlazy2_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btopt_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btultra_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_btultra_dictMatchState
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
            ],
            [
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
                Some(
                    ZSTD_compressBlock_greedy_dedicatedDictSearch
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy_dedicatedDictSearch
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                Some(
                    ZSTD_compressBlock_lazy2_dedicatedDictSearch
                        as unsafe extern "C" fn(
                            *mut ZSTD_matchState_t,
                            *mut seqStore_t,
                            *mut U32,
                            *const libc::c_void,
                            size_t,
                        ) -> size_t,
                ),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    ZSTD_blockCompressor,
                >(NULL as libc::intptr_t),
            ],
        ]
    };
    let mut selectedCompressor: ZSTD_blockCompressor = None;
    if ZSTD_cParam_withinBounds(ZSTD_c_strategy, strat as libc::c_int) != 0 {} else {
        __assert_fail(
            b"ZSTD_cParam_withinBounds(ZSTD_c_strategy, strat)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3037 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"ZSTD_blockCompressor ZSTD_selectBlockCompressor(ZSTD_strategy, ZSTD_paramSwitch_e, ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    if ZSTD_rowMatchFinderUsed(strat, useRowMatchFinder) != 0 {
        static mut rowBasedBlockCompressors: [[ZSTD_blockCompressor; 3]; 4] = unsafe {
            [
                [
                    Some(
                        ZSTD_compressBlock_greedy_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy2_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                ],
                [
                    Some(
                        ZSTD_compressBlock_greedy_extDict_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy_extDict_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy2_extDict_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                ],
                [
                    Some(
                        ZSTD_compressBlock_greedy_dictMatchState_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy_dictMatchState_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy2_dictMatchState_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                ],
                [
                    Some(
                        ZSTD_compressBlock_greedy_dedicatedDictSearch_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy_dedicatedDictSearch_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        ZSTD_compressBlock_lazy2_dedicatedDictSearch_row
                            as unsafe extern "C" fn(
                                *mut ZSTD_matchState_t,
                                *mut seqStore_t,
                                *mut U32,
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                ],
            ]
        };
        if useRowMatchFinder as libc::c_uint
            != ZSTD_ps_auto as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"useRowMatchFinder != ZSTD_ps_auto\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3055 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"ZSTD_blockCompressor ZSTD_selectBlockCompressor(ZSTD_strategy, ZSTD_paramSwitch_e, ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        selectedCompressor = rowBasedBlockCompressors[dictMode as libc::c_int
            as usize][(strat as libc::c_int - ZSTD_greedy as libc::c_int) as usize];
    } else {
        selectedCompressor = blockCompressor[dictMode as libc::c_int
            as usize][strat as libc::c_int as usize];
    }
    if selectedCompressor.is_some() {} else {
        __assert_fail(
            b"selectedCompressor != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3060 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"ZSTD_blockCompressor ZSTD_selectBlockCompressor(ZSTD_strategy, ZSTD_paramSwitch_e, ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    return selectedCompressor;
}
unsafe extern "C" fn ZSTD_storeLastLiterals(
    mut seqStorePtr: *mut seqStore_t,
    mut anchor: *const BYTE,
    mut lastLLSize: size_t,
) {
    libc::memcpy(
        (*seqStorePtr).lit as *mut libc::c_void,
        anchor as *const libc::c_void,
        lastLLSize as libc::size_t,
    );
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(lastLLSize as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetSeqStore(mut ssPtr: *mut seqStore_t) {
    (*ssPtr).lit = (*ssPtr).litStart;
    (*ssPtr).sequences = (*ssPtr).sequencesStart;
    (*ssPtr).longLengthType = ZSTD_llt_none;
}
unsafe extern "C" fn ZSTD_postProcessSequenceProducerResult(
    mut outSeqs: *mut ZSTD_Sequence,
    mut nbExternalSeqs: size_t,
    mut outSeqsCapacity: size_t,
    mut srcSize: size_t,
) -> size_t {
    if nbExternalSeqs > outSeqsCapacity {
        return -(ZSTD_error_sequenceProducer_failed as libc::c_int) as size_t;
    }
    if nbExternalSeqs == 0 as libc::c_int as libc::c_ulong
        && srcSize > 0 as libc::c_int as libc::c_ulong
    {
        return -(ZSTD_error_sequenceProducer_failed as libc::c_int) as size_t;
    }
    if srcSize == 0 as libc::c_int as libc::c_ulong {
        libc::memset(
            &mut *outSeqs.offset(0 as libc::c_int as isize) as *mut ZSTD_Sequence
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ZSTD_Sequence>() as libc::c_ulong as libc::size_t,
        );
        return 1 as libc::c_int as size_t;
    }
    let lastSeq = *outSeqs
        .offset(nbExternalSeqs.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    if lastSeq.offset == 0 as libc::c_int as libc::c_uint
        && lastSeq.matchLength == 0 as libc::c_int as libc::c_uint
    {
        return nbExternalSeqs;
    }
    if nbExternalSeqs == outSeqsCapacity {
        return -(ZSTD_error_sequenceProducer_failed as libc::c_int) as size_t;
    }
    libc::memset(
        &mut *outSeqs.offset(nbExternalSeqs as isize) as *mut ZSTD_Sequence
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_Sequence>() as libc::c_ulong as libc::size_t,
    );
    return nbExternalSeqs.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_fastSequenceLengthSum(
    mut seqBuf: *const ZSTD_Sequence,
    mut seqBufSize: size_t,
) -> size_t {
    let mut matchLenSum: size_t = 0;
    let mut litLenSum: size_t = 0;
    let mut i: size_t = 0;
    matchLenSum = 0 as libc::c_int as size_t;
    litLenSum = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < seqBufSize {
        litLenSum = (litLenSum as libc::c_ulong)
            .wrapping_add((*seqBuf.offset(i as isize)).litLength as libc::c_ulong)
            as size_t as size_t;
        matchLenSum = (matchLenSum as libc::c_ulong)
            .wrapping_add((*seqBuf.offset(i as isize)).matchLength as libc::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    return litLenSum.wrapping_add(matchLenSum);
}
unsafe extern "C" fn ZSTD_buildSeqStore(
    mut zc: *mut ZSTD_CCtx,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let ms: *mut ZSTD_matchState_t = &mut (*zc).blockState.matchState;
    if srcSize <= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"srcSize <= ZSTD_BLOCKSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3150 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                .as_ptr(),
        );
    }
    ZSTD_assertEqualCParams((*zc).appliedParams.cParams, (*ms).cParams);
    if srcSize
        < (MIN_CBLOCK_SIZE as libc::c_ulong)
            .wrapping_add(ZSTD_blockHeaderSize)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        if (*zc).appliedParams.cParams.strategy as libc::c_uint
            >= ZSTD_btopt as libc::c_int as libc::c_uint
        {
            ZSTD_ldm_skipRawSeqStoreBytes(&mut (*zc).externSeqStore, srcSize);
        } else {
            ZSTD_ldm_skipSequences(
                &mut (*zc).externSeqStore,
                srcSize,
                (*zc).appliedParams.cParams.minMatch,
            );
        }
        return ZSTDbss_noCompress as libc::c_int as size_t;
    }
    ZSTD_resetSeqStore(&mut (*zc).seqStore);
    (*ms).opt.symbolCosts = &mut (*(*zc).blockState.prevCBlock).entropy;
    (*ms).opt.literalCompressionMode = (*zc).appliedParams.literalCompressionMode;
    if ((*ms).dictMatchState).is_null() || (*ms).loadedDictEnd == (*ms).window.dictLimit
    {} else {
        __assert_fail(
            b"ms->dictMatchState == NULL || ms->loadedDictEnd == ms->window.dictLimit\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                .as_ptr(),
        );
    }
    let base = (*ms).window.base;
    let istart = src as *const BYTE;
    let curr = istart.offset_from(base) as libc::c_long as U32;
    if ::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        if (istart.offset_from(base) as libc::c_long)
            < -(1 as libc::c_int) as U32 as ptrdiff_t
        {} else {
            __assert_fail(
                b"istart - base < (ptrdiff_t)(U32)(-1)\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3177 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    }
    if curr > ((*ms).nextToUpdate).wrapping_add(384 as libc::c_int as libc::c_uint) {
        (*ms)
            .nextToUpdate = curr
            .wrapping_sub(
                (if (192 as libc::c_int as libc::c_uint)
                    < curr
                        .wrapping_sub((*ms).nextToUpdate)
                        .wrapping_sub(384 as libc::c_int as libc::c_uint)
                {
                    192 as libc::c_int as libc::c_uint
                } else {
                    curr.wrapping_sub((*ms).nextToUpdate)
                        .wrapping_sub(384 as libc::c_int as libc::c_uint)
                }),
            );
    }
    let dictMode = ZSTD_matchState_dictMode(ms);
    let mut lastLLSize: size_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ZSTD_REP_NUM {
        (*(*zc).blockState.nextCBlock)
            .rep[i as usize] = (*(*zc).blockState.prevCBlock).rep[i as usize];
        i += 1;
    }
    if (*zc).externSeqStore.pos < (*zc).externSeqStore.size {
        if (*zc).appliedParams.ldmParams.enableLdm as libc::c_uint
            == ZSTD_ps_disable as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"zc->appliedParams.ldmParams.enableLdm == ZSTD_ps_disable\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3190 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
        if (*zc).appliedParams.useSequenceProducer != 0 {
            return -(ZSTD_error_parameter_combination_unsupported as libc::c_int)
                as size_t;
        }
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut (*zc).externSeqStore,
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
        if (*zc).externSeqStore.pos <= (*zc).externSeqStore.size {} else {
            __assert_fail(
                b"zc->externSeqStore.pos <= zc->externSeqStore.size\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3207 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    } else if (*zc).appliedParams.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        let mut ldmSeqStore = kNullRawSeqStore;
        if (*zc).appliedParams.useSequenceProducer != 0 {
            return -(ZSTD_error_parameter_combination_unsupported as libc::c_int)
                as size_t;
        }
        ldmSeqStore.seq = (*zc).ldmSequences;
        ldmSeqStore.capacity = (*zc).maxNbLdmSequences;
        let err_code = ZSTD_ldm_generateSequences(
            &mut (*zc).ldmState,
            &mut ldmSeqStore,
            &mut (*zc).appliedParams.ldmParams,
            src,
            srcSize,
        );
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut ldmSeqStore,
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
        if ldmSeqStore.pos == ldmSeqStore.size {} else {
            __assert_fail(
                b"ldmSeqStore.pos == ldmSeqStore.size\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3232 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    } else if (*zc).appliedParams.useSequenceProducer != 0 {
        if (*zc).externalMatchCtx.seqBufferCapacity >= ZSTD_sequenceBound(srcSize)
        {} else {
            __assert_fail(
                b"zc->externalMatchCtx.seqBufferCapacity >= ZSTD_sequenceBound(srcSize)\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3236 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
        if ((*zc).externalMatchCtx.mFinder).is_some() {} else {
            __assert_fail(
                b"zc->externalMatchCtx.mFinder != NULL\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3237 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t ZSTD_buildSeqStore(ZSTD_CCtx *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
        let windowSize = (1 as libc::c_int as U32)
            << (*zc).appliedParams.cParams.windowLog;
        let nbExternalSeqs = ((*zc).externalMatchCtx.mFinder)
            .expect(
                "non-null function pointer",
            )(
            (*zc).externalMatchCtx.mState,
            (*zc).externalMatchCtx.seqBuffer,
            (*zc).externalMatchCtx.seqBufferCapacity,
            src,
            srcSize,
            NULL as *const libc::c_void,
            0 as libc::c_int as size_t,
            (*zc).appliedParams.compressionLevel,
            windowSize as size_t,
        );
        let nbPostProcessedSeqs = ZSTD_postProcessSequenceProducerResult(
            (*zc).externalMatchCtx.seqBuffer,
            nbExternalSeqs,
            (*zc).externalMatchCtx.seqBufferCapacity,
            srcSize,
        );
        if ERR_isError(nbPostProcessedSeqs) == 0 {
            let mut seqPos = {
                let mut init = ZSTD_sequencePosition {
                    idx: 0 as libc::c_int as U32,
                    posInSequence: 0 as libc::c_int as U32,
                    posInSrc: 0 as libc::c_int as size_t,
                };
                init
            };
            let seqLenSum = ZSTD_fastSequenceLengthSum(
                (*zc).externalMatchCtx.seqBuffer,
                nbPostProcessedSeqs,
            );
            if seqLenSum > srcSize {
                return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
            }
            let err_code_0 = ZSTD_copySequencesToSeqStoreExplicitBlockDelim(
                zc,
                &mut seqPos,
                (*zc).externalMatchCtx.seqBuffer,
                nbPostProcessedSeqs,
                src,
                srcSize,
                (*zc).appliedParams.searchForExternalRepcodes,
            );
            if ERR_isError(err_code_0) != 0 {
                return err_code_0;
            }
            (*ms).ldmSeqStore = NULL as *const rawSeqStore_t;
            return ZSTDbss_compress as libc::c_int as size_t;
        }
        if (*zc).appliedParams.enableMatchFinderFallback == 0 {
            return nbPostProcessedSeqs;
        }
        let blockCompressor = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        (*ms).ldmSeqStore = NULL as *const rawSeqStore_t;
        lastLLSize = blockCompressor
            .expect(
                "non-null function pointer",
            )(
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            src,
            srcSize,
        );
    } else {
        let blockCompressor_0 = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        (*ms).ldmSeqStore = NULL as *const rawSeqStore_t;
        lastLLSize = blockCompressor_0
            .expect(
                "non-null function pointer",
            )(
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            src,
            srcSize,
        );
    }
    let lastLiterals = (src as *const BYTE)
        .offset(srcSize as isize)
        .offset(-(lastLLSize as isize));
    ZSTD_storeLastLiterals(&mut (*zc).seqStore, lastLiterals, lastLLSize);
    return ZSTDbss_compress as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_copyBlockSequences(mut zc: *mut ZSTD_CCtx) {
    let mut seqStore = ZSTD_getSeqStore(zc);
    let mut seqStoreSeqs: *const seqDef = (*seqStore).sequencesStart;
    let mut seqStoreSeqSize = ((*seqStore).sequences).offset_from(seqStoreSeqs)
        as libc::c_long as size_t;
    let mut seqStoreLiteralsSize = ((*seqStore).lit).offset_from((*seqStore).litStart)
        as libc::c_long as size_t;
    let mut literalsRead = 0 as libc::c_int as size_t;
    let mut lastLLSize: size_t = 0;
    let mut outSeqs: *mut ZSTD_Sequence = &mut *((*zc).seqCollector.seqStart)
        .offset((*zc).seqCollector.seqIndex as isize) as *mut ZSTD_Sequence;
    let mut i: size_t = 0;
    let mut updatedRepcodes = repcodes_t { rep: [0; 3] };
    if ((*zc).seqCollector.seqIndex).wrapping_add(1 as libc::c_int as libc::c_ulong)
        < (*zc).seqCollector.maxSequences
    {} else {
        __assert_fail(
            b"zc->seqCollector.seqIndex + 1 < zc->seqCollector.maxSequences\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3320 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void ZSTD_copyBlockSequences(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    if (*zc).seqCollector.maxSequences
        >= seqStoreSeqSize.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"zc->seqCollector.maxSequences >= seqStoreSeqSize + 1\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3322 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void ZSTD_copyBlockSequences(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut libc::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    i = 0 as libc::c_int as size_t;
    while i < seqStoreSeqSize {
        let mut rawOffset = ((*seqStoreSeqs.offset(i as isize)).offBase)
            .wrapping_sub(ZSTD_REP_NUM as libc::c_uint);
        (*outSeqs.offset(i as isize))
            .litLength = (*seqStoreSeqs.offset(i as isize)).litLength as libc::c_uint;
        (*outSeqs.offset(i as isize))
            .matchLength = ((*seqStoreSeqs.offset(i as isize)).mlBase as libc::c_int
            + MINMATCH) as libc::c_uint;
        (*outSeqs.offset(i as isize)).rep = 0 as libc::c_int as libc::c_uint;
        if i == (*seqStore).longLengthPos as libc::c_ulong {
            if (*seqStore).longLengthType as libc::c_uint
                == ZSTD_llt_literalLength as libc::c_int as libc::c_uint
            {
                let ref mut fresh4 = (*outSeqs.offset(i as isize)).litLength;
                *fresh4 = (*fresh4).wrapping_add(0x10000 as libc::c_int as libc::c_uint);
            } else if (*seqStore).longLengthType as libc::c_uint
                == ZSTD_llt_matchLength as libc::c_int as libc::c_uint
            {
                let ref mut fresh5 = (*outSeqs.offset(i as isize)).matchLength;
                *fresh5 = (*fresh5).wrapping_add(0x10000 as libc::c_int as libc::c_uint);
            }
        }
        if (*seqStoreSeqs.offset(i as isize)).offBase <= ZSTD_REP_NUM as libc::c_uint {
            (*outSeqs.offset(i as isize))
                .rep = (*seqStoreSeqs.offset(i as isize)).offBase;
            if (*outSeqs.offset(i as isize)).litLength
                != 0 as libc::c_int as libc::c_uint
            {
                rawOffset = updatedRepcodes
                    .rep[((*outSeqs.offset(i as isize)).rep)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
            } else if (*outSeqs.offset(i as isize)).rep
                == 3 as libc::c_int as libc::c_uint
            {
                rawOffset = (updatedRepcodes.rep[0 as libc::c_int as usize])
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
            } else {
                rawOffset = updatedRepcodes
                    .rep[(*outSeqs.offset(i as isize)).rep as usize];
            }
        }
        (*outSeqs.offset(i as isize)).offset = rawOffset;
        ZSTD_updateRep(
            (updatedRepcodes.rep).as_mut_ptr(),
            (*seqStoreSeqs.offset(i as isize)).offBase,
            ((*seqStoreSeqs.offset(i as isize)).litLength as libc::c_int
                == 0 as libc::c_int) as libc::c_int as U32,
        );
        literalsRead = (literalsRead as libc::c_ulong)
            .wrapping_add((*outSeqs.offset(i as isize)).litLength as libc::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    if seqStoreLiteralsSize >= literalsRead {} else {
        __assert_fail(
            b"seqStoreLiteralsSize >= literalsRead\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3363 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void ZSTD_copyBlockSequences(ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    lastLLSize = seqStoreLiteralsSize.wrapping_sub(literalsRead);
    (*outSeqs.offset(i as isize)).litLength = lastLLSize as U32;
    let ref mut fresh6 = (*outSeqs.offset(i as isize)).rep;
    *fresh6 = 0 as libc::c_int as libc::c_uint;
    let ref mut fresh7 = (*outSeqs.offset(i as isize)).offset;
    *fresh7 = *fresh6;
    (*outSeqs.offset(i as isize)).matchLength = *fresh7;
    seqStoreSeqSize = seqStoreSeqSize.wrapping_add(1);
    (*zc)
        .seqCollector
        .seqIndex = ((*zc).seqCollector.seqIndex as libc::c_ulong)
        .wrapping_add(seqStoreSeqSize) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sequenceBound(mut srcSize: size_t) -> size_t {
    return srcSize
        .wrapping_div(ZSTD_MINMATCH_MIN as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_generateSequences(
    mut zc: *mut ZSTD_CCtx,
    mut outSeqs: *mut ZSTD_Sequence,
    mut outSeqsSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let dstCapacity = ZSTD_compressBound(srcSize);
    let mut dst = ZSTD_customMalloc(dstCapacity, ZSTD_defaultCMem);
    let mut seqCollector = SeqCollector {
        collectSequences: 0,
        seqStart: 0 as *mut ZSTD_Sequence,
        seqIndex: 0,
        maxSequences: 0,
    };
    if dst.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    seqCollector.collectSequences = 1 as libc::c_int;
    seqCollector.seqStart = outSeqs;
    seqCollector.seqIndex = 0 as libc::c_int as size_t;
    seqCollector.maxSequences = outSeqsSize;
    (*zc).seqCollector = seqCollector;
    ZSTD_compress2(zc, dst, dstCapacity, src, srcSize);
    ZSTD_customFree(dst, ZSTD_defaultCMem);
    return (*zc).seqCollector.seqIndex;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_mergeBlockDelimiters(
    mut sequences: *mut ZSTD_Sequence,
    mut seqsSize: size_t,
) -> size_t {
    let mut in_0 = 0 as libc::c_int as size_t;
    let mut out = 0 as libc::c_int as size_t;
    while in_0 < seqsSize {
        if (*sequences.offset(in_0 as isize)).offset == 0 as libc::c_int as libc::c_uint
            && (*sequences.offset(in_0 as isize)).matchLength
                == 0 as libc::c_int as libc::c_uint
        {
            if in_0 != seqsSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let ref mut fresh8 = (*sequences
                    .offset(
                        in_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .litLength;
                *fresh8 = (*fresh8)
                    .wrapping_add((*sequences.offset(in_0 as isize)).litLength);
            }
        } else {
            *sequences.offset(out as isize) = *sequences.offset(in_0 as isize);
            out = out.wrapping_add(1);
        }
        in_0 = in_0.wrapping_add(1);
    }
    return out;
}
unsafe extern "C" fn ZSTD_isRLE(
    mut src: *const BYTE,
    mut length: size_t,
) -> libc::c_int {
    let mut ip = src;
    let value = *ip.offset(0 as libc::c_int as isize);
    let valueST = (value as U64 as libc::c_ulonglong)
        .wrapping_mul(0x101010101010101 as libc::c_ulonglong) as size_t;
    let unrollSize = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong);
    let unrollMask = unrollSize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let prefixLength = length & unrollMask;
    let mut i: size_t = 0;
    if length == 1 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if prefixLength != 0
        && ZSTD_count(
            ip.offset(1 as libc::c_int as isize),
            ip,
            ip.offset(prefixLength as isize),
        ) != prefixLength.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return 0 as libc::c_int;
    }
    i = prefixLength;
    while i != length {
        let mut u: size_t = 0;
        u = 0 as libc::c_int as size_t;
        while u < unrollSize {
            if MEM_readST(
                ip.offset(i as isize).offset(u as isize) as *const libc::c_void,
            ) != valueST
            {
                return 0 as libc::c_int;
            }
            u = (u as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong)
                as size_t as size_t;
        }
        i = (i as libc::c_ulong).wrapping_add(unrollSize) as size_t as size_t;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ZSTD_maybeRLE(mut seqStore: *const seqStore_t) -> libc::c_int {
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
        as libc::c_long as size_t;
    let nbLits = ((*seqStore).lit).offset_from((*seqStore).litStart) as libc::c_long
        as size_t;
    return (nbSeqs < 4 as libc::c_int as libc::c_ulong
        && nbLits < 10 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn ZSTD_blockState_confirmRepcodesAndEntropyTables(
    bs: *mut ZSTD_blockState_t,
) {
    let tmp = (*bs).prevCBlock;
    (*bs).prevCBlock = (*bs).nextCBlock;
    (*bs).nextCBlock = tmp;
}
unsafe extern "C" fn writeBlockHeader(
    mut op: *mut libc::c_void,
    mut cSize: size_t,
    mut blockSize: size_t,
    mut lastBlock: U32,
) {
    let cBlockHeader = if cSize == 1 as libc::c_int as libc::c_ulong {
        lastBlock
            .wrapping_add((bt_rle as libc::c_int as U32) << 1 as libc::c_int)
            .wrapping_add((blockSize << 3 as libc::c_int) as U32)
    } else {
        lastBlock
            .wrapping_add((bt_compressed as libc::c_int as U32) << 1 as libc::c_int)
            .wrapping_add((cSize << 3 as libc::c_int) as U32)
    };
    MEM_writeLE24(op, cBlockHeader);
}
unsafe extern "C" fn ZSTD_buildBlockEntropyStats_literals(
    src: *mut libc::c_void,
    mut srcSize: size_t,
    mut prevHuf: *const ZSTD_hufCTables_t,
    mut nextHuf: *mut ZSTD_hufCTables_t,
    mut hufMetadata: *mut ZSTD_hufCTablesMetadata_t,
    literalsCompressionIsDisabled: libc::c_int,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut hufFlags: libc::c_int,
) -> size_t {
    let wkspStart = workspace as *mut BYTE;
    let wkspEnd = wkspStart.offset(wkspSize as isize);
    let countWkspStart = wkspStart;
    let countWksp = workspace as *mut libc::c_uint;
    let countWkspSize = ((HUF_SYMBOLVALUE_MAX + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong);
    let nodeWksp = countWkspStart.offset(countWkspSize as isize);
    let nodeWkspSize = wkspEnd.offset_from(nodeWksp) as libc::c_long as size_t;
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX as libc::c_uint;
    let mut huffLog = LitHufLog as libc::c_uint;
    let mut repeat = (*prevHuf).repeatMode;
    libc::memcpy(
        nextHuf as *mut libc::c_void,
        prevHuf as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong as libc::size_t,
    );
    if literalsCompressionIsDisabled != 0 {
        (*hufMetadata).hType = set_basic;
        return 0 as libc::c_int as size_t;
    }
    let minLitSize = (if (*prevHuf).repeatMode as libc::c_uint
        == HUF_repeat_valid as libc::c_int as libc::c_uint
    {
        6 as libc::c_int
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    }) as size_t;
    if srcSize <= minLitSize {
        (*hufMetadata).hType = set_basic;
        return 0 as libc::c_int as size_t;
    }
    let largest = HIST_count_wksp(
        countWksp,
        &mut maxSymbolValue,
        src as *const BYTE as *const libc::c_void,
        srcSize,
        workspace,
        wkspSize,
    );
    let err_code = largest;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if largest == srcSize {
        (*hufMetadata).hType = set_rle;
        return 0 as libc::c_int as size_t;
    }
    if largest
        <= (srcSize >> 7 as libc::c_int).wrapping_add(4 as libc::c_int as libc::c_ulong)
    {
        (*hufMetadata).hType = set_basic;
        return 0 as libc::c_int as size_t;
    }
    if repeat as libc::c_uint == HUF_repeat_check as libc::c_int as libc::c_uint
        && HUF_validateCTable(((*prevHuf).CTable).as_ptr(), countWksp, maxSymbolValue)
            == 0
    {
        repeat = HUF_repeat_none;
    }
    libc::memset(
        ((*nextHuf).CTable).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[HUF_CElt; 257]>() as libc::c_ulong as libc::size_t,
    );
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        nodeWksp as *mut libc::c_void,
        nodeWkspSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        hufFlags,
    );
    if huffLog <= 11 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"huffLog <= LitHufLog\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3541 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 174],
                &[libc::c_char; 174],
            >(
                b"size_t ZSTD_buildBlockEntropyStats_literals(void *const, size_t, const ZSTD_hufCTables_t *, ZSTD_hufCTables_t *, ZSTD_hufCTablesMetadata_t *, const int, void *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    let maxBits = HUF_buildCTable_wksp(
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut libc::c_void,
        nodeWkspSize,
    );
    let err_code_0 = maxBits;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    huffLog = maxBits as U32;
    let newCSize = HUF_estimateCompressedSize(
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        maxSymbolValue,
    );
    let hSize = HUF_writeCTable_wksp(
        ((*hufMetadata).hufDesBuffer).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[BYTE; 128]>() as libc::c_ulong,
        ((*nextHuf).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut libc::c_void,
        nodeWkspSize,
    );
    if repeat as libc::c_uint != HUF_repeat_none as libc::c_int as libc::c_uint {
        let oldCSize = HUF_estimateCompressedSize(
            ((*prevHuf).CTable).as_ptr(),
            countWksp,
            maxSymbolValue,
        );
        if oldCSize < srcSize
            && (oldCSize <= hSize.wrapping_add(newCSize)
                || hSize.wrapping_add(12 as libc::c_int as libc::c_ulong) >= srcSize)
        {
            libc::memcpy(
                nextHuf as *mut libc::c_void,
                prevHuf as *const libc::c_void,
                ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong
                    as libc::size_t,
            );
            (*hufMetadata).hType = set_repeat;
            return 0 as libc::c_int as size_t;
        }
    }
    if newCSize.wrapping_add(hSize) >= srcSize {
        libc::memcpy(
            nextHuf as *mut libc::c_void,
            prevHuf as *const libc::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong as libc::size_t,
        );
        (*hufMetadata).hType = set_basic;
        return 0 as libc::c_int as size_t;
    }
    (*hufMetadata).hType = set_compressed;
    (*nextHuf).repeatMode = HUF_repeat_check;
    return hSize;
}
pub const COMPRESS_LITERALS_SIZE_MIN: libc::c_int = 63 as libc::c_int;
unsafe extern "C" fn ZSTD_buildDummySequencesStatistics(
    mut nextEntropy: *mut ZSTD_fseCTables_t,
) -> ZSTD_symbolEncodingTypeStats_t {
    let mut stats = {
        let mut init = ZSTD_symbolEncodingTypeStats_t {
            LLtype: set_basic as libc::c_int as U32,
            Offtype: set_basic as libc::c_int as U32,
            MLtype: set_basic as libc::c_int as U32,
            size: 0 as libc::c_int as size_t,
            lastCountSize: 0 as libc::c_int as size_t,
            longOffsets: 0 as libc::c_int,
        };
        init
    };
    (*nextEntropy).litlength_repeatMode = FSE_repeat_none;
    (*nextEntropy).offcode_repeatMode = FSE_repeat_none;
    (*nextEntropy).matchlength_repeatMode = FSE_repeat_none;
    return stats;
}
unsafe extern "C" fn ZSTD_buildBlockEntropyStats_sequences(
    mut seqStorePtr: *const seqStore_t,
    mut prevEntropy: *const ZSTD_fseCTables_t,
    mut nextEntropy: *mut ZSTD_fseCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut fseMetadata: *mut ZSTD_fseCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let strategy = (*cctxParams).cParams.strategy;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as libc::c_long as size_t;
    let ostart = ((*fseMetadata).fseTablesBuffer).as_mut_ptr();
    let oend = ostart
        .offset(::core::mem::size_of::<[BYTE; 133]>() as libc::c_ulong as isize);
    let mut op = ostart;
    let mut countWorkspace = workspace as *mut libc::c_uint;
    let mut entropyWorkspace = countWorkspace
        .offset(
            ((if 35 as libc::c_int > 52 as libc::c_int {
                35 as libc::c_int
            } else {
                52 as libc::c_int
            }) + 1 as libc::c_int) as isize,
        );
    let mut entropyWorkspaceSize = wkspSize
        .wrapping_sub(
            (((if 35 as libc::c_int > 52 as libc::c_int {
                35 as libc::c_int
            } else {
                52 as libc::c_int
            }) + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        LLtype: 0,
        Offtype: 0,
        MLtype: 0,
        size: 0,
        lastCountSize: 0,
        longOffsets: 0,
    };
    stats = if nbSeq != 0 as libc::c_int as libc::c_ulong {
        ZSTD_buildSequencesStatistics(
            seqStorePtr,
            nbSeq,
            prevEntropy,
            nextEntropy,
            op,
            oend,
            strategy,
            countWorkspace,
            entropyWorkspace as *mut libc::c_void,
            entropyWorkspaceSize,
        )
    } else {
        ZSTD_buildDummySequencesStatistics(nextEntropy)
    };
    let err_code = stats.size;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    (*fseMetadata).llType = stats.LLtype as symbolEncodingType_e;
    (*fseMetadata).ofType = stats.Offtype as symbolEncodingType_e;
    (*fseMetadata).mlType = stats.MLtype as symbolEncodingType_e;
    (*fseMetadata).lastCountSize = stats.lastCountSize;
    return stats.size;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_buildBlockEntropyStats(
    mut seqStorePtr: *const seqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let litSize = ((*seqStorePtr).lit).offset_from((*seqStorePtr).litStart)
        as libc::c_long as size_t;
    let huf_useOptDepth = ((*cctxParams).cParams.strategy as libc::c_uint
        >= HUF_OPTIMAL_DEPTH_THRESHOLD as libc::c_uint) as libc::c_int;
    let hufFlags = if huf_useOptDepth != 0 {
        HUF_flags_optimalDepth as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*entropyMetadata)
        .hufMetadata
        .hufDesSize = ZSTD_buildBlockEntropyStats_literals(
        (*seqStorePtr).litStart as *mut libc::c_void,
        litSize,
        &(*prevEntropy).huf,
        &mut (*nextEntropy).huf,
        &mut (*entropyMetadata).hufMetadata,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        workspace,
        wkspSize,
        hufFlags,
    );
    let err_code = (*entropyMetadata).hufMetadata.hufDesSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    (*entropyMetadata)
        .fseMetadata
        .fseTablesSize = ZSTD_buildBlockEntropyStats_sequences(
        seqStorePtr,
        &(*prevEntropy).fse,
        &mut (*nextEntropy).fse,
        cctxParams,
        &mut (*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
    );
    let err_code_0 = (*entropyMetadata).fseMetadata.fseTablesSize;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_estimateBlockSize_literal(
    mut literals: *const BYTE,
    mut litSize: size_t,
    mut huf: *const ZSTD_hufCTables_t,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut workspace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut writeEntropy: libc::c_int,
) -> size_t {
    let countWksp = workspace as *mut libc::c_uint;
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX as libc::c_uint;
    let mut literalSectionHeaderSize = (3 as libc::c_int
        + (litSize
            >= (1 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong) as libc::c_int
        + (litSize
            >= (16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong) as libc::c_int) as size_t;
    let mut singleStream = (litSize < 256 as libc::c_int as libc::c_ulong) as libc::c_int
        as U32;
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
                if singleStream == 0 {
                    cLitSizeEstimate = (cLitSizeEstimate as libc::c_ulong)
                        .wrapping_add(6 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
            }
        }
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
            as *const libc::c_char,
        3691 as libc::c_int as libc::c_uint,
        (*::core::mem::transmute::<
            &[u8; 143],
            &[libc::c_char; 143],
        >(
            b"size_t ZSTD_estimateBlockSize_literal(const BYTE *, size_t, const ZSTD_hufCTables_t *, const ZSTD_hufCTablesMetadata_t *, void *, size_t, int)\0",
        ))
            .as_ptr(),
    );
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_estimateBlockSize_symbolType(
    mut type_0: symbolEncodingType_e,
    mut codeTable: *const BYTE,
    mut nbSeq: size_t,
    mut maxCode: libc::c_uint,
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
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3714 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 172],
                    &[libc::c_char; 172],
                >(
                    b"size_t ZSTD_estimateBlockSize_symbolType(symbolEncodingType_e, const BYTE *, size_t, unsigned int, const FSE_CTable *, const U8 *, const short *, U32, U32, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        cSymbolTypeSizeEstimateInBits = ZSTD_crossEntropyCost(
            defaultNorm,
            defaultNormLog,
            countWksp,
            max,
        );
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
    return cSymbolTypeSizeEstimateInBits >> 3 as libc::c_int;
}
unsafe extern "C" fn ZSTD_estimateBlockSize_sequences(
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
    let mut sequencesSectionHeaderSize = (1 as libc::c_int + 1 as libc::c_int
        + (nbSeq >= 128 as libc::c_int as libc::c_ulong) as libc::c_int
        + (nbSeq >= LONGNBSEQ as libc::c_ulong) as libc::c_int) as size_t;
    let mut cSeqSizeEstimate = 0 as libc::c_int as size_t;
    cSeqSizeEstimate = (cSeqSizeEstimate as libc::c_ulong)
        .wrapping_add(
            ZSTD_estimateBlockSize_symbolType(
                (*fseMetadata).ofType,
                ofCodeTable,
                nbSeq,
                MaxOff as libc::c_uint,
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
            ZSTD_estimateBlockSize_symbolType(
                (*fseMetadata).llType,
                llCodeTable,
                nbSeq,
                MaxLL as libc::c_uint,
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
            ZSTD_estimateBlockSize_symbolType(
                (*fseMetadata).mlType,
                mlCodeTable,
                nbSeq,
                MaxML as libc::c_uint,
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
unsafe extern "C" fn ZSTD_estimateBlockSize(
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
    let literalsSize = ZSTD_estimateBlockSize_literal(
        literals,
        litSize,
        &(*entropy).huf,
        &(*entropyMetadata).hufMetadata,
        workspace,
        wkspSize,
        writeLitEntropy,
    );
    let seqSize = ZSTD_estimateBlockSize_sequences(
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
    return seqSize.wrapping_add(literalsSize).wrapping_add(ZSTD_blockHeaderSize);
}
unsafe extern "C" fn ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
    mut seqStore: *mut seqStore_t,
    mut zc: *mut ZSTD_CCtx,
) -> size_t {
    let entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t = &mut (*zc)
        .blockSplitCtx
        .entropyMetadata;
    let err_code = ZSTD_buildBlockEntropyStats(
        seqStore,
        &mut (*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &mut (*zc).appliedParams,
        entropyMetadata,
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
    return ZSTD_estimateBlockSize(
        (*seqStore).litStart,
        ((*seqStore).lit).offset_from((*seqStore).litStart) as libc::c_long as size_t,
        (*seqStore).ofCode,
        (*seqStore).llCode,
        (*seqStore).mlCode,
        ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as libc::c_long
            as size_t,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        entropyMetadata,
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
        ((*entropyMetadata).hufMetadata.hType as libc::c_uint
            == set_compressed as libc::c_int as libc::c_uint) as libc::c_int,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn ZSTD_countSeqStoreLiteralsBytes(
    seqStore: *const seqStore_t,
) -> size_t {
    let mut literalsBytes = 0 as libc::c_int as size_t;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
        as libc::c_long as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nbSeqs {
        let seq = *((*seqStore).sequencesStart).offset(i as isize);
        literalsBytes = (literalsBytes as libc::c_ulong)
            .wrapping_add(seq.litLength as libc::c_ulong) as size_t as size_t;
        if i == (*seqStore).longLengthPos as libc::c_ulong
            && (*seqStore).longLengthType as libc::c_uint
                == ZSTD_llt_literalLength as libc::c_int as libc::c_uint
        {
            literalsBytes = (literalsBytes as libc::c_ulong)
                .wrapping_add(0x10000 as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        i = i.wrapping_add(1);
    }
    return literalsBytes;
}
unsafe extern "C" fn ZSTD_countSeqStoreMatchBytes(
    seqStore: *const seqStore_t,
) -> size_t {
    let mut matchBytes = 0 as libc::c_int as size_t;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
        as libc::c_long as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nbSeqs {
        let mut seq = *((*seqStore).sequencesStart).offset(i as isize);
        matchBytes = (matchBytes as libc::c_ulong)
            .wrapping_add((seq.mlBase as libc::c_int + MINMATCH) as libc::c_ulong)
            as size_t as size_t;
        if i == (*seqStore).longLengthPos as libc::c_ulong
            && (*seqStore).longLengthType as libc::c_uint
                == ZSTD_llt_matchLength as libc::c_int as libc::c_uint
        {
            matchBytes = (matchBytes as libc::c_ulong)
                .wrapping_add(0x10000 as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        i = i.wrapping_add(1);
    }
    return matchBytes;
}
unsafe extern "C" fn ZSTD_deriveSeqStoreChunk(
    mut resultSeqStore: *mut seqStore_t,
    mut originalSeqStore: *const seqStore_t,
    mut startIdx: size_t,
    mut endIdx: size_t,
) {
    *resultSeqStore = *originalSeqStore;
    if startIdx > 0 as libc::c_int as libc::c_ulong {
        (*resultSeqStore)
            .sequences = ((*originalSeqStore).sequencesStart).offset(startIdx as isize);
        (*resultSeqStore)
            .litStart = ((*resultSeqStore).litStart)
            .offset(ZSTD_countSeqStoreLiteralsBytes(resultSeqStore) as isize);
    }
    if (*originalSeqStore).longLengthType as libc::c_uint
        != ZSTD_llt_none as libc::c_int as libc::c_uint
    {
        if ((*originalSeqStore).longLengthPos as libc::c_ulong) < startIdx
            || (*originalSeqStore).longLengthPos as libc::c_ulong > endIdx
        {
            (*resultSeqStore).longLengthType = ZSTD_llt_none;
        } else {
            (*resultSeqStore)
                .longLengthPos = ((*resultSeqStore).longLengthPos as libc::c_uint)
                .wrapping_sub(startIdx as U32) as U32 as U32;
        }
    }
    (*resultSeqStore)
        .sequencesStart = ((*originalSeqStore).sequencesStart).offset(startIdx as isize);
    (*resultSeqStore)
        .sequences = ((*originalSeqStore).sequencesStart).offset(endIdx as isize);
    if endIdx
        == ((*originalSeqStore).sequences)
            .offset_from((*originalSeqStore).sequencesStart) as libc::c_long as size_t
    {
        if (*resultSeqStore).lit == (*originalSeqStore).lit {} else {
            __assert_fail(
                b"resultSeqStore->lit == originalSeqStore->lit\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3863 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"void ZSTD_deriveSeqStoreChunk(seqStore_t *, const seqStore_t *, size_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        let literalsBytes = ZSTD_countSeqStoreLiteralsBytes(resultSeqStore);
        (*resultSeqStore)
            .lit = ((*resultSeqStore).litStart).offset(literalsBytes as isize);
    }
    (*resultSeqStore).llCode = ((*resultSeqStore).llCode).offset(startIdx as isize);
    (*resultSeqStore).mlCode = ((*resultSeqStore).mlCode).offset(startIdx as isize);
    (*resultSeqStore).ofCode = ((*resultSeqStore).ofCode).offset(startIdx as isize);
}
unsafe extern "C" fn ZSTD_resolveRepcodeToRawOffset(
    mut rep: *const U32,
    offBase: U32,
    ll0: U32,
) -> U32 {
    if 1 as libc::c_int as libc::c_uint <= offBase
        && offBase <= 3 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"OFFBASE_IS_REPCODE(offBase)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3880 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"U32 ZSTD_resolveRepcodeToRawOffset(const U32 *, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    let adjustedRepCode = offBase
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_add(ll0);
    if 1 as libc::c_int as libc::c_uint <= offBase
        && offBase <= 3 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"OFFBASE_IS_REPCODE(offBase)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            3881 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"U32 ZSTD_resolveRepcodeToRawOffset(const U32 *, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if adjustedRepCode == ZSTD_REP_NUM as libc::c_uint {
        if ll0 != 0 {} else {
            __assert_fail(
                b"ll0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3883 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"U32 ZSTD_resolveRepcodeToRawOffset(const U32 *, const U32, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        return (*rep.offset(0 as libc::c_int as isize))
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
    return *rep.offset(adjustedRepCode as isize);
}
unsafe extern "C" fn ZSTD_seqStore_resolveOffCodes(
    dRepcodes: *mut repcodes_t,
    cRepcodes: *mut repcodes_t,
    seqStore: *const seqStore_t,
    nbSeq: U32,
) {
    let mut idx = 0 as libc::c_int as U32;
    let longLitLenIdx = if (*seqStore).longLengthType as libc::c_uint
        == ZSTD_llt_literalLength as libc::c_int as libc::c_uint
    {
        (*seqStore).longLengthPos
    } else {
        nbSeq
    };
    while idx < nbSeq {
        let seq = ((*seqStore).sequencesStart).offset(idx as isize);
        let ll0 = ((*seq).litLength as libc::c_int == 0 as libc::c_int
            && idx != longLitLenIdx) as libc::c_int as U32;
        let offBase = (*seq).offBase;
        if offBase > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"offBase > 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                3919 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 109],
                    &[libc::c_char; 109],
                >(
                    b"void ZSTD_seqStore_resolveOffCodes(repcodes_t *const, repcodes_t *const, const seqStore_t *const, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        if 1 as libc::c_int as libc::c_uint <= offBase
            && offBase <= ZSTD_REP_NUM as libc::c_uint
        {
            let dRawOffset = ZSTD_resolveRepcodeToRawOffset(
                ((*dRepcodes).rep).as_mut_ptr() as *const U32,
                offBase,
                ll0,
            );
            let cRawOffset = ZSTD_resolveRepcodeToRawOffset(
                ((*cRepcodes).rep).as_mut_ptr() as *const U32,
                offBase,
                ll0,
            );
            if dRawOffset != cRawOffset {
                if cRawOffset > 0 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"(cRawOffset)>0\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        3928 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 109],
                            &[libc::c_char; 109],
                        >(
                            b"void ZSTD_seqStore_resolveOffCodes(repcodes_t *const, repcodes_t *const, const seqStore_t *const, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                (*seq).offBase = cRawOffset.wrapping_add(ZSTD_REP_NUM as libc::c_uint);
            }
        }
        ZSTD_updateRep(((*dRepcodes).rep).as_mut_ptr(), (*seq).offBase, ll0);
        ZSTD_updateRep(((*cRepcodes).rep).as_mut_ptr(), offBase, ll0);
        idx = idx.wrapping_add(1);
    }
}
unsafe extern "C" fn ZSTD_compressSeqStore_singleBlock(
    mut zc: *mut ZSTD_CCtx,
    seqStore: *const seqStore_t,
    dRep: *mut repcodes_t,
    cRep: *mut repcodes_t,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut lastBlock: U32,
    mut isPartition: U32,
) -> size_t {
    let rleMaxLength = 25 as libc::c_int as U32;
    let mut op = dst as *mut BYTE;
    let mut ip = src as *const BYTE;
    let mut cSize: size_t = 0;
    let mut cSeqsSize: size_t = 0;
    let dRepOriginal = *dRep;
    if isPartition != 0 {
        ZSTD_seqStore_resolveOffCodes(
            dRep,
            cRep,
            seqStore,
            ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
                as libc::c_long as U32,
        );
    }
    if dstCapacity < ZSTD_blockHeaderSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    cSeqsSize = ZSTD_entropyCompressSeqStore(
        seqStore,
        &mut (*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &mut (*zc).appliedParams,
        op.offset(ZSTD_blockHeaderSize as isize) as *mut libc::c_void,
        dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
        srcSize,
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
        (*zc).bmi2,
    );
    let err_code = cSeqsSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if (*zc).isFirstBlock == 0 && cSeqsSize < rleMaxLength as libc::c_ulong
        && ZSTD_isRLE(src as *const BYTE, srcSize) != 0
    {
        cSeqsSize = 1 as libc::c_int as size_t;
    }
    if (*zc).seqCollector.collectSequences != 0 {
        ZSTD_copyBlockSequences(zc);
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        return 0 as libc::c_int as size_t;
    }
    if cSeqsSize == 0 as libc::c_int as libc::c_ulong {
        cSize = ZSTD_noCompressBlock(
            op as *mut libc::c_void,
            dstCapacity,
            ip as *const libc::c_void,
            srcSize,
            lastBlock,
        );
        let err_code_0 = cSize;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        *dRep = dRepOriginal;
    } else if cSeqsSize == 1 as libc::c_int as libc::c_ulong {
        cSize = ZSTD_rleCompressBlock(
            op as *mut libc::c_void,
            dstCapacity,
            *ip,
            srcSize,
            lastBlock,
        );
        let err_code_1 = cSize;
        if ERR_isError(err_code_1) != 0 {
            return err_code_1;
        }
        *dRep = dRepOriginal;
    } else {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        writeBlockHeader(op as *mut libc::c_void, cSeqsSize, srcSize, lastBlock);
        cSize = ZSTD_blockHeaderSize.wrapping_add(cSeqsSize);
    }
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as libc::c_uint
        == FSE_repeat_valid as libc::c_int as libc::c_uint
    {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode = FSE_repeat_check;
    }
    return cSize;
}
pub const MIN_SEQUENCES_BLOCK_SPLITTING: libc::c_int = 300 as libc::c_int;
unsafe extern "C" fn ZSTD_deriveBlockSplitsHelper(
    mut splits: *mut seqStoreSplits,
    mut startIdx: size_t,
    mut endIdx: size_t,
    mut zc: *mut ZSTD_CCtx,
    mut origSeqStore: *const seqStore_t,
) {
    let fullSeqStoreChunk: *mut seqStore_t = &mut (*zc).blockSplitCtx.fullSeqStoreChunk;
    let firstHalfSeqStore: *mut seqStore_t = &mut (*zc).blockSplitCtx.firstHalfSeqStore;
    let secondHalfSeqStore: *mut seqStore_t = &mut (*zc)
        .blockSplitCtx
        .secondHalfSeqStore;
    let mut estimatedOriginalSize: size_t = 0;
    let mut estimatedFirstHalfSize: size_t = 0;
    let mut estimatedSecondHalfSize: size_t = 0;
    let mut midIdx = startIdx
        .wrapping_add(endIdx)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    if endIdx >= startIdx {} else {
        __assert_fail(
            b"endIdx >= startIdx\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4048 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"void ZSTD_deriveBlockSplitsHelper(seqStoreSplits *, size_t, size_t, ZSTD_CCtx *, const seqStore_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if endIdx.wrapping_sub(startIdx) < MIN_SEQUENCES_BLOCK_SPLITTING as libc::c_ulong
        || (*splits).idx >= ZSTD_MAX_NB_BLOCK_SPLITS as libc::c_ulong
    {
        return;
    }
    ZSTD_deriveSeqStoreChunk(fullSeqStoreChunk, origSeqStore, startIdx, endIdx);
    ZSTD_deriveSeqStoreChunk(firstHalfSeqStore, origSeqStore, startIdx, midIdx);
    ZSTD_deriveSeqStoreChunk(secondHalfSeqStore, origSeqStore, midIdx, endIdx);
    estimatedOriginalSize = ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
        fullSeqStoreChunk,
        zc,
    );
    estimatedFirstHalfSize = ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
        firstHalfSeqStore,
        zc,
    );
    estimatedSecondHalfSize = ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
        secondHalfSeqStore,
        zc,
    );
    if ERR_isError(estimatedOriginalSize) != 0
        || ERR_isError(estimatedFirstHalfSize) != 0
        || ERR_isError(estimatedSecondHalfSize) != 0
    {
        return;
    }
    if estimatedFirstHalfSize.wrapping_add(estimatedSecondHalfSize)
        < estimatedOriginalSize
    {
        ZSTD_deriveBlockSplitsHelper(splits, startIdx, midIdx, zc, origSeqStore);
        *((*splits).splitLocations).offset((*splits).idx as isize) = midIdx as U32;
        (*splits).idx = ((*splits).idx).wrapping_add(1);
        ZSTD_deriveBlockSplitsHelper(splits, midIdx, endIdx, zc, origSeqStore);
    }
}
unsafe extern "C" fn ZSTD_deriveBlockSplits(
    mut zc: *mut ZSTD_CCtx,
    mut partitions: *mut U32,
    mut nbSeq: U32,
) -> size_t {
    let mut splits = seqStoreSplits {
        splitLocations: 0 as *mut U32,
        idx: 0,
    };
    splits.splitLocations = partitions;
    splits.idx = 0 as libc::c_int as size_t;
    if nbSeq <= 4 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as size_t;
    }
    ZSTD_deriveBlockSplitsHelper(
        &mut splits,
        0 as libc::c_int as size_t,
        nbSeq as size_t,
        zc,
        &mut (*zc).seqStore,
    );
    *(splits.splitLocations).offset(splits.idx as isize) = nbSeq;
    return splits.idx;
}
unsafe extern "C" fn ZSTD_compressBlock_splitBlock_internal(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut blockSize: size_t,
    mut lastBlock: U32,
    mut nbSeq: U32,
) -> size_t {
    let mut cSize = 0 as libc::c_int as size_t;
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let mut i = 0 as libc::c_int as size_t;
    let mut srcBytesTotal = 0 as libc::c_int as size_t;
    let partitions = ((*zc).blockSplitCtx.partitions).as_mut_ptr();
    let nextSeqStore: *mut seqStore_t = &mut (*zc).blockSplitCtx.nextSeqStore;
    let currSeqStore: *mut seqStore_t = &mut (*zc).blockSplitCtx.currSeqStore;
    let numSplits = ZSTD_deriveBlockSplits(zc, partitions, nbSeq);
    let mut dRep = repcodes_t { rep: [0; 3] };
    let mut cRep = repcodes_t { rep: [0; 3] };
    libc::memcpy(
        (dRep.rep).as_mut_ptr() as *mut libc::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        (cRep.rep).as_mut_ptr() as *mut libc::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    libc::memset(
        nextSeqStore as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<seqStore_t>() as libc::c_ulong as libc::size_t,
    );
    if numSplits == 0 as libc::c_int as libc::c_ulong {
        let mut cSizeSingleBlock = ZSTD_compressSeqStore_singleBlock(
            zc,
            &mut (*zc).seqStore,
            &mut dRep,
            &mut cRep,
            op as *mut libc::c_void,
            dstCapacity,
            ip as *const libc::c_void,
            blockSize,
            lastBlock,
            0 as libc::c_int as U32,
        );
        let err_code = cSizeSingleBlock;
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        if (*zc).blockSize <= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong
        {} else {
            __assert_fail(
                b"zc->blockSize <= ZSTD_BLOCKSIZE_MAX\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4148 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 107],
                    &[libc::c_char; 107],
                >(
                    b"size_t ZSTD_compressBlock_splitBlock_internal(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32, U32)\0",
                ))
                    .as_ptr(),
            );
        }
        if cSizeSingleBlock <= ((*zc).blockSize).wrapping_add(ZSTD_blockHeaderSize)
        {} else {
            __assert_fail(
                b"cSizeSingleBlock <= zc->blockSize + ZSTD_blockHeaderSize\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4149 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 107],
                    &[libc::c_char; 107],
                >(
                    b"size_t ZSTD_compressBlock_splitBlock_internal(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32, U32)\0",
                ))
                    .as_ptr(),
            );
        }
        return cSizeSingleBlock;
    }
    ZSTD_deriveSeqStoreChunk(
        currSeqStore,
        &mut (*zc).seqStore,
        0 as libc::c_int as size_t,
        *partitions.offset(0 as libc::c_int as isize) as size_t,
    );
    i = 0 as libc::c_int as size_t;
    while i <= numSplits {
        let mut cSizeChunk: size_t = 0;
        let lastPartition = (i == numSplits) as libc::c_int as U32;
        let mut lastBlockEntireSrc = 0 as libc::c_int as U32;
        let mut srcBytes = (ZSTD_countSeqStoreLiteralsBytes(currSeqStore))
            .wrapping_add(ZSTD_countSeqStoreMatchBytes(currSeqStore));
        srcBytesTotal = (srcBytesTotal as libc::c_ulong).wrapping_add(srcBytes) as size_t
            as size_t;
        if lastPartition != 0 {
            srcBytes = (srcBytes as libc::c_ulong)
                .wrapping_add(blockSize.wrapping_sub(srcBytesTotal)) as size_t as size_t;
            lastBlockEntireSrc = lastBlock;
        } else {
            ZSTD_deriveSeqStoreChunk(
                nextSeqStore,
                &mut (*zc).seqStore,
                *partitions.offset(i as isize) as size_t,
                *partitions
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as size_t,
            );
        }
        cSizeChunk = ZSTD_compressSeqStore_singleBlock(
            zc,
            currSeqStore,
            &mut dRep,
            &mut cRep,
            op as *mut libc::c_void,
            dstCapacity,
            ip as *const libc::c_void,
            srcBytes,
            lastBlockEntireSrc,
            1 as libc::c_int as U32,
        );
        let err_code_0 = cSizeChunk;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        ip = ip.offset(srcBytes as isize);
        op = op.offset(cSizeChunk as isize);
        dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(cSizeChunk) as size_t
            as size_t;
        cSize = (cSize as libc::c_ulong).wrapping_add(cSizeChunk) as size_t as size_t;
        *currSeqStore = *nextSeqStore;
        if cSizeChunk <= ((*zc).blockSize).wrapping_add(ZSTD_blockHeaderSize) {} else {
            __assert_fail(
                b"cSizeChunk <= zc->blockSize + ZSTD_blockHeaderSize\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4183 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 107],
                    &[libc::c_char; 107],
                >(
                    b"size_t ZSTD_compressBlock_splitBlock_internal(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32, U32)\0",
                ))
                    .as_ptr(),
            );
        }
        i = i.wrapping_add(1);
    }
    libc::memcpy(
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *mut libc::c_void,
        (dRep.rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    return cSize;
}
unsafe extern "C" fn ZSTD_compressBlock_splitBlock(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut lastBlock: U32,
) -> size_t {
    let mut nbSeq: U32 = 0;
    let mut cSize: size_t = 0;
    if (*zc).appliedParams.useBlockSplitter as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"zc->appliedParams.useBlockSplitter == ZSTD_ps_enable\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4200 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"size_t ZSTD_compressBlock_splitBlock(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
            ))
                .as_ptr(),
        );
    }
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if bss == ZSTDbss_noCompress as libc::c_int as libc::c_ulong {
        if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as libc::c_uint
            == FSE_repeat_valid as libc::c_int as libc::c_uint
        {
            (*(*zc).blockState.prevCBlock)
                .entropy
                .fse
                .offcode_repeatMode = FSE_repeat_check;
        }
        cSize = ZSTD_noCompressBlock(dst, dstCapacity, src, srcSize, lastBlock);
        let err_code_0 = cSize;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        return cSize;
    }
    nbSeq = ((*zc).seqStore.sequences).offset_from((*zc).seqStore.sequencesStart)
        as libc::c_long as U32;
    cSize = ZSTD_compressBlock_splitBlock_internal(
        zc,
        dst,
        dstCapacity,
        src,
        srcSize,
        lastBlock,
        nbSeq,
    );
    let err_code_1 = cSize;
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    return cSize;
}
unsafe extern "C" fn ZSTD_compressBlock_internal(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut frame: U32,
) -> size_t {
    let rleMaxLength = 25 as libc::c_int as U32;
    let mut cSize: size_t = 0;
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if bss == ZSTDbss_noCompress as libc::c_int as libc::c_ulong {
        cSize = 0 as libc::c_int as size_t;
    } else {
        if (*zc).seqCollector.collectSequences != 0 {
            ZSTD_copyBlockSequences(zc);
            ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
            return 0 as libc::c_int as size_t;
        }
        cSize = ZSTD_entropyCompressSeqStore(
            &mut (*zc).seqStore,
            &mut (*(*zc).blockState.prevCBlock).entropy,
            &mut (*(*zc).blockState.nextCBlock).entropy,
            &mut (*zc).appliedParams,
            dst,
            dstCapacity,
            srcSize,
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
            (*zc).bmi2,
        );
        if frame != 0 && (*zc).isFirstBlock == 0 && cSize < rleMaxLength as libc::c_ulong
            && ZSTD_isRLE(ip, srcSize) != 0
        {
            cSize = 1 as libc::c_int as size_t;
            *op
                .offset(
                    0 as libc::c_int as isize,
                ) = *ip.offset(0 as libc::c_int as isize);
        }
    }
    if ERR_isError(cSize) == 0 && cSize > 1 as libc::c_int as libc::c_ulong {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
    }
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as libc::c_uint
        == FSE_repeat_valid as libc::c_int as libc::c_uint
    {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode = FSE_repeat_check;
    }
    return cSize;
}
unsafe extern "C" fn ZSTD_compressBlock_targetCBlockSize_body(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    bss: size_t,
    mut lastBlock: U32,
) -> size_t {
    if bss == ZSTDbss_compress as libc::c_int as libc::c_ulong {
        if (*zc).isFirstBlock == 0 && ZSTD_maybeRLE(&mut (*zc).seqStore) != 0
            && ZSTD_isRLE(src as *const BYTE, srcSize) != 0
        {
            return ZSTD_rleCompressBlock(
                dst,
                dstCapacity,
                *(src as *const BYTE),
                srcSize,
                lastBlock,
            );
        }
        let cSize = ZSTD_compressSuperBlock(
            zc,
            dst,
            dstCapacity,
            src,
            srcSize,
            lastBlock,
        );
        if cSize != -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t {
            let maxCSize = srcSize
                .wrapping_sub(
                    ZSTD_minGain(srcSize, (*zc).appliedParams.cParams.strategy),
                );
            let err_code = cSize;
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if cSize != 0 as libc::c_int as libc::c_ulong
                && cSize < maxCSize.wrapping_add(ZSTD_blockHeaderSize)
            {
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
                return cSize;
            }
        }
    }
    return ZSTD_noCompressBlock(dst, dstCapacity, src, srcSize, lastBlock);
}
unsafe extern "C" fn ZSTD_compressBlock_targetCBlockSize(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut lastBlock: U32,
) -> size_t {
    let mut cSize = 0 as libc::c_int as size_t;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    cSize = ZSTD_compressBlock_targetCBlockSize_body(
        zc,
        dst,
        dstCapacity,
        src,
        srcSize,
        bss,
        lastBlock,
    );
    let err_code_0 = cSize;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as libc::c_uint
        == FSE_repeat_valid as libc::c_int as libc::c_uint
    {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode = FSE_repeat_check;
    }
    return cSize;
}
unsafe extern "C" fn ZSTD_overflowCorrectIfNeeded(
    mut ms: *mut ZSTD_matchState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut ip: *const libc::c_void,
    mut iend: *const libc::c_void,
) {
    let cycleLog = ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy);
    let maxDist = (1 as libc::c_int as U32) << (*params).cParams.windowLog;
    if ZSTD_window_needOverflowCorrection(
        (*ms).window,
        cycleLog,
        maxDist,
        (*ms).loadedDictEnd,
        ip,
        iend,
    ) != 0
    {
        let correction = ZSTD_window_correctOverflow(
            &mut (*ms).window,
            cycleLog,
            maxDist,
            ip,
        );
        ZSTD_cwksp_mark_tables_dirty(ws);
        ZSTD_reduceIndex(ms, params, correction);
        ZSTD_cwksp_mark_tables_clean(ws);
        if (*ms).nextToUpdate < correction {
            (*ms).nextToUpdate = 0 as libc::c_int as U32;
        } else {
            (*ms)
                .nextToUpdate = ((*ms).nextToUpdate as libc::c_uint)
                .wrapping_sub(correction) as U32 as U32;
        }
        (*ms).loadedDictEnd = 0 as libc::c_int as U32;
        (*ms).dictMatchState = NULL as *const ZSTD_matchState_t;
    }
}
unsafe extern "C" fn ZSTD_compress_frameChunk(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut lastFrameChunk: U32,
) -> size_t {
    let mut blockSize = (*cctx).blockSize;
    let mut remaining = srcSize;
    let mut ip = src as *const BYTE;
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let maxDist = (1 as libc::c_int as U32) << (*cctx).appliedParams.cParams.windowLog;
    if (*cctx).appliedParams.cParams.windowLog
        <= (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            30 as libc::c_int
        } else {
            31 as libc::c_int
        }) as libc::c_uint
    {} else {
        __assert_fail(
            b"cctx->appliedParams.cParams.windowLog <= ZSTD_WINDOWLOG_MAX\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4403 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"size_t ZSTD_compress_frameChunk(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cctx).appliedParams.fParams.checksumFlag != 0 && srcSize != 0 {
        ZSTD_XXH64_update(&mut (*cctx).xxhState, src, srcSize);
    }
    while remaining != 0 {
        let ms: *mut ZSTD_matchState_t = &mut (*cctx).blockState.matchState;
        let lastBlock = lastFrameChunk
            & (blockSize >= remaining) as libc::c_int as libc::c_uint;
        if dstCapacity
            < ZSTD_blockHeaderSize
                .wrapping_add((1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        if remaining < blockSize {
            blockSize = remaining;
        }
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            ip as *const libc::c_void,
            ip.offset(blockSize as isize) as *const libc::c_void,
        );
        ZSTD_checkDictValidity(
            &mut (*ms).window,
            ip.offset(blockSize as isize) as *const libc::c_void,
            maxDist,
            &mut (*ms).loadedDictEnd,
            &mut (*ms).dictMatchState,
        );
        ZSTD_window_enforceMaxDist(
            &mut (*ms).window,
            ip as *const libc::c_void,
            maxDist,
            &mut (*ms).loadedDictEnd,
            &mut (*ms).dictMatchState,
        );
        if (*ms).nextToUpdate < (*ms).window.lowLimit {
            (*ms).nextToUpdate = (*ms).window.lowLimit;
        }
        let mut cSize: size_t = 0;
        if ZSTD_useTargetCBlockSize(&mut (*cctx).appliedParams) != 0 {
            cSize = ZSTD_compressBlock_targetCBlockSize(
                cctx,
                op as *mut libc::c_void,
                dstCapacity,
                ip as *const libc::c_void,
                blockSize,
                lastBlock,
            );
            let err_code = cSize;
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if cSize > 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"cSize > 0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    4432 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 88],
                        &[libc::c_char; 88],
                    >(
                        b"size_t ZSTD_compress_frameChunk(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if cSize <= blockSize.wrapping_add(ZSTD_blockHeaderSize) {} else {
                __assert_fail(
                    b"cSize <= blockSize + ZSTD_blockHeaderSize\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    4433 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 88],
                        &[libc::c_char; 88],
                    >(
                        b"size_t ZSTD_compress_frameChunk(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
                    ))
                        .as_ptr(),
                );
            }
        } else if ZSTD_blockSplitterEnabled(&mut (*cctx).appliedParams) != 0 {
            cSize = ZSTD_compressBlock_splitBlock(
                cctx,
                op as *mut libc::c_void,
                dstCapacity,
                ip as *const libc::c_void,
                blockSize,
                lastBlock,
            );
            let err_code_0 = cSize;
            if ERR_isError(err_code_0) != 0 {
                return err_code_0;
            }
            if cSize > 0 as libc::c_int as libc::c_ulong
                || (*cctx).seqCollector.collectSequences == 1 as libc::c_int
            {} else {
                __assert_fail(
                    b"cSize > 0 || cctx->seqCollector.collectSequences == 1\0"
                        as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    4437 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 88],
                        &[libc::c_char; 88],
                    >(
                        b"size_t ZSTD_compress_frameChunk(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
                    ))
                        .as_ptr(),
                );
            }
        } else {
            cSize = ZSTD_compressBlock_internal(
                cctx,
                op.offset(ZSTD_blockHeaderSize as isize) as *mut libc::c_void,
                dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                ip as *const libc::c_void,
                blockSize,
                1 as libc::c_int as U32,
            );
            let err_code_1 = cSize;
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            if cSize == 0 as libc::c_int as libc::c_ulong {
                cSize = ZSTD_noCompressBlock(
                    op as *mut libc::c_void,
                    dstCapacity,
                    ip as *const libc::c_void,
                    blockSize,
                    lastBlock,
                );
                let err_code_2 = cSize;
                if ERR_isError(err_code_2) != 0 {
                    return err_code_2;
                }
            } else {
                let cBlockHeader = if cSize == 1 as libc::c_int as libc::c_ulong {
                    lastBlock
                        .wrapping_add((bt_rle as libc::c_int as U32) << 1 as libc::c_int)
                        .wrapping_add((blockSize << 3 as libc::c_int) as U32)
                } else {
                    lastBlock
                        .wrapping_add(
                            (bt_compressed as libc::c_int as U32) << 1 as libc::c_int,
                        )
                        .wrapping_add((cSize << 3 as libc::c_int) as U32)
                };
                MEM_writeLE24(op as *mut libc::c_void, cBlockHeader);
                cSize = (cSize as libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize)
                    as size_t as size_t;
            }
        }
        ip = ip.offset(blockSize as isize);
        if remaining >= blockSize {} else {
            __assert_fail(
                b"remaining >= blockSize\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4458 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"size_t ZSTD_compress_frameChunk(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
                ))
                    .as_ptr(),
            );
        }
        remaining = (remaining as libc::c_ulong).wrapping_sub(blockSize) as size_t
            as size_t;
        op = op.offset(cSize as isize);
        if dstCapacity >= cSize {} else {
            __assert_fail(
                b"dstCapacity >= cSize\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4461 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"size_t ZSTD_compress_frameChunk(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32)\0",
                ))
                    .as_ptr(),
            );
        }
        dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(cSize) as size_t
            as size_t;
        (*cctx).isFirstBlock = 0 as libc::c_int;
    }
    if lastFrameChunk != 0 && op > ostart {
        (*cctx).stage = ZSTDcs_ending;
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_writeFrameHeader(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
    mut dictID: U32,
) -> size_t {
    let op = dst as *mut BYTE;
    let dictIDSizeCodeLength = ((dictID > 0 as libc::c_int as libc::c_uint)
        as libc::c_int + (dictID >= 256 as libc::c_int as libc::c_uint) as libc::c_int
        + (dictID >= 65536 as libc::c_int as libc::c_uint) as libc::c_int) as U32;
    let dictIDSizeCode = if (*params).fParams.noDictIDFlag != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        dictIDSizeCodeLength
    };
    let checksumFlag = ((*params).fParams.checksumFlag > 0 as libc::c_int) as libc::c_int
        as U32;
    let windowSize = (1 as libc::c_int as U32) << (*params).cParams.windowLog;
    let singleSegment = ((*params).fParams.contentSizeFlag != 0
        && windowSize as libc::c_ulong >= pledgedSrcSize) as libc::c_int as U32;
    let windowLogByte = (((*params).cParams.windowLog)
        .wrapping_sub(ZSTD_WINDOWLOG_ABSOLUTEMIN as libc::c_uint) << 3 as libc::c_int)
        as BYTE;
    let fcsCode = (if (*params).fParams.contentSizeFlag != 0 {
        (pledgedSrcSize >= 256 as libc::c_int as libc::c_ulong) as libc::c_int
            + (pledgedSrcSize
                >= (65536 as libc::c_int + 256 as libc::c_int) as libc::c_ulong)
                as libc::c_int
            + (pledgedSrcSize >= 0xffffffff as libc::c_uint as libc::c_ulong)
                as libc::c_int
    } else {
        0 as libc::c_int
    }) as U32;
    let frameHeaderDescriptionByte = dictIDSizeCode
        .wrapping_add(checksumFlag << 2 as libc::c_int)
        .wrapping_add(singleSegment << 5 as libc::c_int)
        .wrapping_add(fcsCode << 6 as libc::c_int) as BYTE;
    let mut pos = 0 as libc::c_int as size_t;
    if !((*params).fParams.contentSizeFlag != 0
        && pledgedSrcSize as libc::c_ulonglong
            == (0 as libc::c_ulonglong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong))
    {} else {
        __assert_fail(
            b"!(params->fParams.contentSizeFlag && pledgedSrcSize == ZSTD_CONTENTSIZE_UNKNOWN)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4487 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"size_t ZSTD_writeFrameHeader(void *, size_t, const ZSTD_CCtx_params *, U64, U32)\0",
            ))
                .as_ptr(),
        );
    }
    if dstCapacity < 18 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if (*params).format as libc::c_uint == ZSTD_f_zstd1 as libc::c_int as libc::c_uint {
        MEM_writeLE32(dst, ZSTD_MAGICNUMBER);
        pos = 4 as libc::c_int as size_t;
    }
    let fresh9 = pos;
    pos = pos.wrapping_add(1);
    *op.offset(fresh9 as isize) = frameHeaderDescriptionByte;
    if singleSegment == 0 {
        let fresh10 = pos;
        pos = pos.wrapping_add(1);
        *op.offset(fresh10 as isize) = windowLogByte;
    }
    match dictIDSizeCode {
        0 => {}
        1 => {
            *op.offset(pos as isize) = dictID as BYTE;
            pos = pos.wrapping_add(1);
        }
        2 => {
            MEM_writeLE16(op.offset(pos as isize) as *mut libc::c_void, dictID as U16);
            pos = (pos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        3 => {
            MEM_writeLE32(op.offset(pos as isize) as *mut libc::c_void, dictID);
            pos = (pos as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4501 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"size_t ZSTD_writeFrameHeader(void *, size_t, const ZSTD_CCtx_params *, U64, U32)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    let mut current_block_44: u64;
    match fcsCode {
        0 => {
            current_block_44 = 6598120286135757988;
        }
        1 => {
            MEM_writeLE16(
                op.offset(pos as isize) as *mut libc::c_void,
                pledgedSrcSize.wrapping_sub(256 as libc::c_int as libc::c_ulong) as U16,
            );
            pos = (pos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            current_block_44 = 5330834795799507926;
        }
        2 => {
            MEM_writeLE32(
                op.offset(pos as isize) as *mut libc::c_void,
                pledgedSrcSize as U32,
            );
            pos = (pos as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            current_block_44 = 5330834795799507926;
        }
        3 => {
            MEM_writeLE64(op.offset(pos as isize) as *mut libc::c_void, pledgedSrcSize);
            pos = (pos as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            current_block_44 = 5330834795799507926;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4511 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"size_t ZSTD_writeFrameHeader(void *, size_t, const ZSTD_CCtx_params *, U64, U32)\0",
                ))
                    .as_ptr(),
            );
            current_block_44 = 6598120286135757988;
        }
    }
    match current_block_44 {
        6598120286135757988 => {
            if singleSegment != 0 {
                let fresh11 = pos;
                pos = pos.wrapping_add(1);
                *op.offset(fresh11 as isize) = pledgedSrcSize as BYTE;
            }
        }
        _ => {}
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_writeSkippableFrame(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut magicVariant: libc::c_uint,
) -> size_t {
    let mut op = dst as *mut BYTE;
    if dstCapacity < srcSize.wrapping_add(8 as libc::c_int as libc::c_ulong) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if srcSize > 0xffffffff as libc::c_uint as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if magicVariant > 15 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    MEM_writeLE32(
        op as *mut libc::c_void,
        (ZSTD_MAGIC_SKIPPABLE_START as libc::c_uint).wrapping_add(magicVariant),
    );
    MEM_writeLE32(
        op.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        srcSize as U32,
    );
    libc::memcpy(
        op.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        src,
        srcSize as libc::size_t,
    );
    return srcSize.wrapping_add(ZSTD_SKIPPABLEHEADERSIZE as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_writeLastEmptyBlock(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    if dstCapacity < ZSTD_blockHeaderSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    let cBlockHeader24 = (1 as libc::c_int as libc::c_uint)
        .wrapping_add((bt_raw as libc::c_int as U32) << 1 as libc::c_int);
    MEM_writeLE24(dst, cBlockHeader24);
    return ZSTD_blockHeaderSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_referenceExternalSequences(
    mut cctx: *mut ZSTD_CCtx,
    mut seq: *mut rawSeq,
    mut nbSeq: size_t,
) -> size_t {
    if (*cctx).stage as libc::c_uint != ZSTDcs_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    if (*cctx).appliedParams.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t;
    }
    (*cctx).externSeqStore.seq = seq;
    (*cctx).externSeqStore.size = nbSeq;
    (*cctx).externSeqStore.capacity = nbSeq;
    (*cctx).externSeqStore.pos = 0 as libc::c_int as size_t;
    (*cctx).externSeqStore.posInSequence = 0 as libc::c_int as size_t;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_compressContinue_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut frame: U32,
    mut lastFrameChunk: U32,
) -> size_t {
    let ms: *mut ZSTD_matchState_t = &mut (*cctx).blockState.matchState;
    let mut fhSize = 0 as libc::c_int as size_t;
    if (*cctx).stage as libc::c_uint == ZSTDcs_created as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    if frame != 0
        && (*cctx).stage as libc::c_uint == ZSTDcs_init as libc::c_int as libc::c_uint
    {
        fhSize = ZSTD_writeFrameHeader(
            dst,
            dstCapacity,
            &mut (*cctx).appliedParams,
            ((*cctx).pledgedSrcSizePlusOne)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as U64,
            (*cctx).dictID,
        );
        let err_code = fhSize;
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        if fhSize <= dstCapacity {} else {
            __assert_fail(
                b"fhSize <= dstCapacity\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4589 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"size_t ZSTD_compressContinue_internal(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32, U32)\0",
                ))
                    .as_ptr(),
            );
        }
        dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(fhSize) as size_t
            as size_t;
        dst = (dst as *mut libc::c_char).offset(fhSize as isize) as *mut libc::c_void;
        (*cctx).stage = ZSTDcs_ongoing;
    }
    if srcSize == 0 {
        return fhSize;
    }
    if ZSTD_window_update(
        &mut (*ms).window,
        src,
        srcSize,
        (*ms).forceNonContiguous as libc::c_int,
    ) == 0
    {
        (*ms).forceNonContiguous = 0 as libc::c_int as U32;
        (*ms).nextToUpdate = (*ms).window.dictLimit;
    }
    if (*cctx).appliedParams.ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint
    {
        ZSTD_window_update(&mut (*cctx).ldmState.window, src, srcSize, 0 as libc::c_int);
    }
    if frame == 0 {
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            src,
            (src as *const BYTE).offset(srcSize as isize) as *const libc::c_void,
        );
    }
    let cSize = if frame != 0 {
        ZSTD_compress_frameChunk(cctx, dst, dstCapacity, src, srcSize, lastFrameChunk)
    } else {
        ZSTD_compressBlock_internal(
            cctx,
            dst,
            dstCapacity,
            src,
            srcSize,
            0 as libc::c_int as U32,
        )
    };
    let err_code_0 = cSize;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    (*cctx)
        .consumedSrcSize = ((*cctx).consumedSrcSize)
        .wrapping_add(srcSize as libc::c_ulonglong);
    (*cctx)
        .producedCSize = ((*cctx).producedCSize)
        .wrapping_add(cSize.wrapping_add(fhSize) as libc::c_ulonglong);
    if !((*cctx).appliedParams.fParams.contentSizeFlag != 0
        && (*cctx).pledgedSrcSizePlusOne == 0 as libc::c_int as libc::c_ulonglong)
    {} else {
        __assert_fail(
            b"!(cctx->appliedParams.fParams.contentSizeFlag && cctx->pledgedSrcSizePlusOne == 0)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4619 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"size_t ZSTD_compressContinue_internal(ZSTD_CCtx *, void *, size_t, const void *, size_t, U32, U32)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cctx).pledgedSrcSizePlusOne != 0 as libc::c_int as libc::c_ulonglong {
        if ((*cctx).consumedSrcSize).wrapping_add(1 as libc::c_int as libc::c_ulonglong)
            > (*cctx).pledgedSrcSizePlusOne
        {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
    }
    return cSize.wrapping_add(fhSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressContinue_public(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressContinue_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        1 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressContinue(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressContinue_public(cctx, dst, dstCapacity, src, srcSize);
}
unsafe extern "C" fn ZSTD_getBlockSize_deprecated(mut cctx: *const ZSTD_CCtx) -> size_t {
    let cParams = (*cctx).appliedParams.cParams;
    if ZSTD_checkCParams(cParams) == 0 {} else {
        __assert_fail(
            b"!ZSTD_checkCParams(cParams)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4652 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"size_t ZSTD_getBlockSize_deprecated(const ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    return if (*cctx).appliedParams.maxBlockSize
        < (1 as libc::c_int as size_t) << cParams.windowLog
    {
        (*cctx).appliedParams.maxBlockSize
    } else {
        (1 as libc::c_int as size_t) << cParams.windowLog
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getBlockSize(mut cctx: *const ZSTD_CCtx) -> size_t {
    return ZSTD_getBlockSize_deprecated(cctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_deprecated(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let blockSizeMax = ZSTD_getBlockSize_deprecated(cctx);
    if srcSize > blockSizeMax {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    return ZSTD_compressContinue_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        0 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_deprecated(cctx, dst, dstCapacity, src, srcSize);
}
unsafe extern "C" fn ZSTD_loadDictionaryContent(
    mut ms: *mut ZSTD_matchState_t,
    mut ls: *mut ldmState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
) -> size_t {
    let mut ip = src as *const BYTE;
    let iend = ip.offset(srcSize as isize);
    let loadLdmDict = ((*params).ldmParams.enableLdm as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint && !ls.is_null())
        as libc::c_int;
    ZSTD_assertEqualCParams((*params).cParams, (*ms).cParams);
    let mut maxDictSize = ((3 as libc::c_uint) << 29 as libc::c_int)
        .wrapping_add(
            (1 as libc::c_uint)
                << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                    == 4 as libc::c_int as libc::c_ulong
                {
                    ZSTD_WINDOWLOG_MAX_32
                } else {
                    ZSTD_WINDOWLOG_MAX_64
                }),
        )
        .wrapping_sub(ZSTD_WINDOW_START_INDEX as libc::c_uint);
    let CDictTaggedIndices = ZSTD_CDictIndicesAreTagged(&(*params).cParams);
    if CDictTaggedIndices != 0
        && tfp as libc::c_uint == ZSTD_tfp_forCDict as libc::c_int as libc::c_uint
    {
        let shortCacheMaxDictSize = ((1 as libc::c_uint)
            << 32 as libc::c_int - ZSTD_SHORT_CACHE_TAG_BITS)
            .wrapping_sub(ZSTD_WINDOW_START_INDEX as libc::c_uint);
        maxDictSize = if maxDictSize < shortCacheMaxDictSize {
            maxDictSize
        } else {
            shortCacheMaxDictSize
        };
        if loadLdmDict == 0 {} else {
            __assert_fail(
                b"!loadLdmDict\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4713 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 184],
                    &[libc::c_char; 184],
                >(
                    b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if srcSize > maxDictSize as libc::c_ulong {
        ip = iend.offset(-(maxDictSize as isize));
        src = ip as *const libc::c_void;
        srcSize = maxDictSize as size_t;
    }
    if srcSize
        > (-(1 as libc::c_int) as U32)
            .wrapping_sub(
                ((3 as libc::c_uint) << 29 as libc::c_int)
                    .wrapping_add(
                        (1 as libc::c_uint)
                            << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                                == 4 as libc::c_int as libc::c_ulong
                            {
                                ZSTD_WINDOWLOG_MAX_32
                            } else {
                                ZSTD_WINDOWLOG_MAX_64
                            }),
                    ),
            ) as libc::c_ulong
    {
        if ZSTD_window_isEmpty((*ms).window) != 0 {} else {
            __assert_fail(
                b"ZSTD_window_isEmpty(ms->window)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4726 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 184],
                    &[libc::c_char; 184],
                >(
                    b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if loadLdmDict != 0 {
            if ZSTD_window_isEmpty((*ls).window) != 0 {} else {
                __assert_fail(
                    b"ZSTD_window_isEmpty(ls->window)\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    4727 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 184],
                        &[libc::c_char; 184],
                    >(
                        b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    ZSTD_window_update(&mut (*ms).window, src, srcSize, 0 as libc::c_int);
    if loadLdmDict != 0 {
        ZSTD_window_update(&mut (*ls).window, src, srcSize, 0 as libc::c_int);
        (*ls)
            .loadedDictEnd = if (*params).forceWindow != 0 {
            0 as libc::c_int as libc::c_uint
        } else {
            iend.offset_from((*ls).window.base) as libc::c_long as U32
        };
        ZSTD_ldm_fillHashTable(ls, ip, iend, &(*params).ldmParams);
    }
    if ((*params).cParams.strategy as libc::c_uint)
        < ZSTD_btultra as libc::c_int as libc::c_uint
    {
        let mut maxDictSize_0 = (8 as libc::c_uint)
            << (if (if (*params).cParams.hashLog > (*params).cParams.chainLog {
                (*params).cParams.hashLog
            } else {
                (*params).cParams.chainLog
            }) < 28 as libc::c_int as libc::c_uint
            {
                (if (*params).cParams.hashLog > (*params).cParams.chainLog {
                    (*params).cParams.hashLog
                } else {
                    (*params).cParams.chainLog
                })
            } else {
                28 as libc::c_int as libc::c_uint
            });
        if srcSize > maxDictSize_0 as libc::c_ulong {
            ip = iend.offset(-(maxDictSize_0 as isize));
            src = ip as *const libc::c_void;
            srcSize = maxDictSize_0 as size_t;
        }
    }
    (*ms).nextToUpdate = ip.offset_from((*ms).window.base) as libc::c_long as U32;
    (*ms)
        .loadedDictEnd = if (*params).forceWindow != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        iend.offset_from((*ms).window.base) as libc::c_long as U32
    };
    (*ms).forceNonContiguous = (*params).deterministicRefPrefix as U32;
    if srcSize <= HASH_READ_SIZE as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    ZSTD_overflowCorrectIfNeeded(
        ms,
        ws,
        params,
        ip as *const libc::c_void,
        iend as *const libc::c_void,
    );
    match (*params).cParams.strategy as libc::c_uint {
        1 => {
            ZSTD_fillHashTable(ms, iend as *const libc::c_void, dtlm, tfp);
        }
        2 => {
            ZSTD_fillDoubleHashTable(ms, iend as *const libc::c_void, dtlm, tfp);
        }
        3 | 4 | 5 => {
            if srcSize >= 8 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"srcSize >= HASH_READ_SIZE\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    4769 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 184],
                        &[libc::c_char; 184],
                    >(
                        b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (*ms).dedicatedDictSearch != 0 {
                if !((*ms).chainTable).is_null() {} else {
                    __assert_fail(
                        b"ms->chainTable != NULL\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        4771 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 184],
                            &[libc::c_char; 184],
                        >(
                            b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ZSTD_dedicatedDictSearch_lazy_loadDictionary(
                    ms,
                    iend.offset(-(HASH_READ_SIZE as isize)),
                );
            } else {
                if (*params).useRowMatchFinder as libc::c_uint
                    != ZSTD_ps_auto as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"params->useRowMatchFinder != ZSTD_ps_auto\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        4774 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 184],
                            &[libc::c_char; 184],
                        >(
                            b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if (*params).useRowMatchFinder as libc::c_uint
                    == ZSTD_ps_enable as libc::c_int as libc::c_uint
                {
                    let tagTableSize = (1 as libc::c_int as size_t)
                        << (*params).cParams.hashLog;
                    libc::memset(
                        (*ms).tagTable as *mut libc::c_void,
                        0 as libc::c_int,
                        tagTableSize as libc::size_t,
                    );
                    ZSTD_row_update(ms, iend.offset(-(HASH_READ_SIZE as isize)));
                } else {
                    ZSTD_insertAndFindFirstIndex(
                        ms,
                        iend.offset(-(HASH_READ_SIZE as isize)),
                    );
                }
            }
        }
        6 | 7 | 8 | 9 => {
            if srcSize >= 8 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"srcSize >= HASH_READ_SIZE\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    4791 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 184],
                        &[libc::c_char; 184],
                    >(
                        b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_updateTree(ms, iend.offset(-(HASH_READ_SIZE as isize)), iend);
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                4796 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 184],
                    &[libc::c_char; 184],
                >(
                    b"size_t ZSTD_loadDictionaryContent(ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    (*ms).nextToUpdate = iend.offset_from((*ms).window.base) as libc::c_long as U32;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_dictNCountRepeat(
    mut normalizedCounter: *mut libc::c_short,
    mut dictMaxSymbolValue: libc::c_uint,
    mut maxSymbolValue: libc::c_uint,
) -> FSE_repeat {
    let mut s: U32 = 0;
    if dictMaxSymbolValue < maxSymbolValue {
        return FSE_repeat_check;
    }
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        if *normalizedCounter.offset(s as isize) as libc::c_int == 0 as libc::c_int {
            return FSE_repeat_check;
        }
        s = s.wrapping_add(1);
    }
    return FSE_repeat_valid;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_loadCEntropy(
    mut bs: *mut ZSTD_compressedBlockState_t,
    mut workspace: *mut libc::c_void,
    dict: *const libc::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut offcodeNCount: [libc::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as libc::c_uint;
    let mut dictPtr = dict as *const BYTE;
    let dictEnd = dictPtr.offset(dictSize as isize);
    dictPtr = dictPtr.offset(8 as libc::c_int as isize);
    (*bs).entropy.huf.repeatMode = HUF_repeat_check;
    let mut maxSymbolValue = 255 as libc::c_int as libc::c_uint;
    let mut hasZeroWeights = 1 as libc::c_int as libc::c_uint;
    let hufHeaderSize = HUF_readCTable(
        ((*bs).entropy.huf.CTable).as_mut_ptr(),
        &mut maxSymbolValue,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as size_t,
        &mut hasZeroWeights,
    );
    if hasZeroWeights == 0 {
        (*bs).entropy.huf.repeatMode = HUF_repeat_valid;
    }
    if ERR_isError(hufHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if maxSymbolValue < 255 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    dictPtr = dictPtr.offset(hufHeaderSize as isize);
    let mut offcodeLog: libc::c_uint = 0;
    let offcodeHeaderSize = FSE_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as size_t,
    );
    if ERR_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if offcodeLog > 8 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if ERR_isError(
        FSE_buildCTable_wksp(
            ((*bs).entropy.fse.offcodeCTable).as_mut_ptr(),
            offcodeNCount.as_mut_ptr(),
            31 as libc::c_int as libc::c_uint,
            offcodeLog,
            workspace,
            (((8 as libc::c_int) << 10 as libc::c_int) + 512 as libc::c_int) as size_t,
        ),
    ) != 0
    {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    dictPtr = dictPtr.offset(offcodeHeaderSize as isize);
    let mut matchlengthNCount: [libc::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as libc::c_uint;
    let mut matchlengthLog: libc::c_uint = 0;
    let matchlengthHeaderSize = FSE_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as size_t,
    );
    if ERR_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if matchlengthLog > 9 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if ERR_isError(
        FSE_buildCTable_wksp(
            ((*bs).entropy.fse.matchlengthCTable).as_mut_ptr(),
            matchlengthNCount.as_mut_ptr(),
            matchlengthMaxValue,
            matchlengthLog,
            workspace,
            (((8 as libc::c_int) << 10 as libc::c_int) + 512 as libc::c_int) as size_t,
        ),
    ) != 0
    {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    (*bs)
        .entropy
        .fse
        .matchlength_repeatMode = ZSTD_dictNCountRepeat(
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        MaxML as libc::c_uint,
    );
    dictPtr = dictPtr.offset(matchlengthHeaderSize as isize);
    let mut litlengthNCount: [libc::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as libc::c_uint;
    let mut litlengthLog: libc::c_uint = 0;
    let litlengthHeaderSize = FSE_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as size_t,
    );
    if ERR_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if litlengthLog > 9 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    if ERR_isError(
        FSE_buildCTable_wksp(
            ((*bs).entropy.fse.litlengthCTable).as_mut_ptr(),
            litlengthNCount.as_mut_ptr(),
            litlengthMaxValue,
            litlengthLog,
            workspace,
            (((8 as libc::c_int) << 10 as libc::c_int) + 512 as libc::c_int) as size_t,
        ),
    ) != 0
    {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    (*bs)
        .entropy
        .fse
        .litlength_repeatMode = ZSTD_dictNCountRepeat(
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        MaxLL as libc::c_uint,
    );
    dictPtr = dictPtr.offset(litlengthHeaderSize as isize);
    if dictPtr.offset(12 as libc::c_int as isize) > dictEnd {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
    }
    (*bs)
        .rep[0 as libc::c_int
        as usize] = MEM_readLE32(
        dictPtr.offset(0 as libc::c_int as isize) as *const libc::c_void,
    );
    (*bs)
        .rep[1 as libc::c_int
        as usize] = MEM_readLE32(
        dictPtr.offset(4 as libc::c_int as isize) as *const libc::c_void,
    );
    (*bs)
        .rep[2 as libc::c_int
        as usize] = MEM_readLE32(
        dictPtr.offset(8 as libc::c_int as isize) as *const libc::c_void,
    );
    dictPtr = dictPtr.offset(12 as libc::c_int as isize);
    let dictContentSize = dictEnd.offset_from(dictPtr) as libc::c_long as size_t;
    let mut offcodeMax = MaxOff as U32;
    if dictContentSize
        <= (-(1 as libc::c_int) as U32)
            .wrapping_sub(
                (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_ulong
    {
        let maxOffset = (dictContentSize as U32)
            .wrapping_add(
                (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                    as libc::c_uint,
            );
        offcodeMax = ZSTD_highbit32(maxOffset);
    }
    (*bs)
        .entropy
        .fse
        .offcode_repeatMode = ZSTD_dictNCountRepeat(
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        if offcodeMax < 31 as libc::c_int as libc::c_uint {
            offcodeMax
        } else {
            31 as libc::c_int as libc::c_uint
        },
    );
    let mut u: U32 = 0;
    u = 0 as libc::c_int as U32;
    while u < 3 as libc::c_int as libc::c_uint {
        if (*bs).rep[u as usize] == 0 as libc::c_int as libc::c_uint {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
        }
        if (*bs).rep[u as usize] as libc::c_ulong > dictContentSize {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
        }
        u = u.wrapping_add(1);
    }
    return dictPtr.offset_from(dict as *const BYTE) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_loadZstdDictionary(
    mut bs: *mut ZSTD_compressedBlockState_t,
    mut ms: *mut ZSTD_matchState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
    mut workspace: *mut libc::c_void,
) -> size_t {
    let mut dictPtr = dict as *const BYTE;
    let dictEnd = dictPtr.offset(dictSize as isize);
    let mut dictID: size_t = 0;
    let mut eSize: size_t = 0;
    if dictSize >= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"dictSize >= 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4937 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 206],
                &[libc::c_char; 206],
            >(
                b"size_t ZSTD_loadZstdDictionary(ZSTD_compressedBlockState_t *, ZSTD_matchState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e, void *)\0",
            ))
                .as_ptr(),
        );
    }
    if MEM_readLE32(dictPtr as *const libc::c_void) == 0xec30a437 as libc::c_uint
    {} else {
        __assert_fail(
            b"MEM_readLE32(dictPtr) == ZSTD_MAGIC_DICTIONARY\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4938 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 206],
                &[libc::c_char; 206],
            >(
                b"size_t ZSTD_loadZstdDictionary(ZSTD_compressedBlockState_t *, ZSTD_matchState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e, void *)\0",
            ))
                .as_ptr(),
        );
    }
    dictID = (if (*params).fParams.noDictIDFlag != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        MEM_readLE32(dictPtr.offset(4 as libc::c_int as isize) as *const libc::c_void)
    }) as size_t;
    eSize = ZSTD_loadCEntropy(bs, workspace, dict, dictSize);
    let err_code = eSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    dictPtr = dictPtr.offset(eSize as isize);
    let dictContentSize = dictEnd.offset_from(dictPtr) as libc::c_long as size_t;
    let err_code_0 = ZSTD_loadDictionaryContent(
        ms,
        0 as *mut ldmState_t,
        ws,
        params,
        dictPtr as *const libc::c_void,
        dictContentSize,
        dtlm,
        tfp,
    );
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    return dictID;
}
unsafe extern "C" fn ZSTD_compress_insertDictionary(
    mut bs: *mut ZSTD_compressedBlockState_t,
    mut ms: *mut ZSTD_matchState_t,
    mut ls: *mut ldmState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
    mut workspace: *mut libc::c_void,
) -> size_t {
    if dict.is_null() || dictSize < 8 as libc::c_int as libc::c_ulong {
        if dictContentType as libc::c_uint
            == ZSTD_dct_fullDict as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t;
        }
        return 0 as libc::c_int as size_t;
    }
    ZSTD_reset_compressedBlockState(bs);
    if dictContentType as libc::c_uint
        == ZSTD_dct_rawContent as libc::c_int as libc::c_uint
    {
        return ZSTD_loadDictionaryContent(ms, ls, ws, params, dict, dictSize, dtlm, tfp);
    }
    if MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY {
        if dictContentType as libc::c_uint
            == ZSTD_dct_auto as libc::c_int as libc::c_uint
        {
            return ZSTD_loadDictionaryContent(
                ms,
                ls,
                ws,
                params,
                dict,
                dictSize,
                dtlm,
                tfp,
            );
        }
        if dictContentType as libc::c_uint
            == ZSTD_dct_fullDict as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t;
        }
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            4986 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 251],
                &[libc::c_char; 251],
            >(
                b"size_t ZSTD_compress_insertDictionary(ZSTD_compressedBlockState_t *, ZSTD_matchState_t *, ldmState_t *, ZSTD_cwksp *, const ZSTD_CCtx_params *, const void *, size_t, ZSTD_dictContentType_e, ZSTD_dictTableLoadMethod_e, ZSTD_tableFillPurpose_e, void *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_loadZstdDictionary(
        bs,
        ms,
        ws,
        params,
        dict,
        dictSize,
        dtlm,
        tfp,
        workspace,
    );
}
pub const ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF: libc::c_int = 128 as libc::c_int
    * ((1 as libc::c_int) << 10 as libc::c_int);
pub const ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER: libc::c_ulonglong = 6
    as libc::c_ulonglong;
unsafe extern "C" fn ZSTD_compressBegin_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let dictContentSize = if !cdict.is_null() {
        (*cdict).dictContentSize
    } else {
        dictSize
    };
    (*cctx)
        .traceCtx = if (Some(
        ZSTD_trace_compress_begin
            as unsafe extern "C" fn(*const ZSTD_CCtx_s) -> ZSTD_TraceCtx,
    ))
        .is_some()
    {
        ZSTD_trace_compress_begin(cctx)
    } else {
        0 as libc::c_int as libc::c_ulonglong
    };
    if ERR_isError(ZSTD_checkCParams((*params).cParams)) == 0 {} else {
        __assert_fail(
            b"!ZSTD_isError(ZSTD_checkCParams(params->cParams))\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5014 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 197],
                &[libc::c_char; 197],
            >(
                b"size_t ZSTD_compressBegin_internal(ZSTD_CCtx *, const void *, size_t, ZSTD_dictContentType_e, ZSTD_dictTableLoadMethod_e, const ZSTD_CDict *, const ZSTD_CCtx_params *, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if !(!dict.is_null() && !cdict.is_null()) {} else {
        __assert_fail(
            b"!((dict) && (cdict))\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5015 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 197],
                &[libc::c_char; 197],
            >(
                b"size_t ZSTD_compressBegin_internal(ZSTD_CCtx *, const void *, size_t, ZSTD_dictContentType_e, ZSTD_dictTableLoadMethod_e, const ZSTD_CDict *, const ZSTD_CCtx_params *, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    if !cdict.is_null() && (*cdict).dictContentSize > 0 as libc::c_int as libc::c_ulong
        && (pledgedSrcSize < ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF as libc::c_ulong
            || (pledgedSrcSize as libc::c_ulonglong)
                < ((*cdict).dictContentSize as libc::c_ulonglong)
                    .wrapping_mul(ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER)
            || pledgedSrcSize as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
            || (*cdict).compressionLevel == 0 as libc::c_int)
        && (*params).attachDictPref as libc::c_uint
            != ZSTD_dictForceLoad as libc::c_int as libc::c_uint
    {
        return ZSTD_resetCCtx_usingCDict(cctx, cdict, params, pledgedSrcSize, zbuff);
    }
    let err_code = ZSTD_resetCCtx_internal(
        cctx,
        params,
        pledgedSrcSize,
        dictContentSize,
        ZSTDcrp_makeClean,
        zbuff,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let dictID = if !cdict.is_null() {
        ZSTD_compress_insertDictionary(
            (*cctx).blockState.prevCBlock,
            &mut (*cctx).blockState.matchState,
            &mut (*cctx).ldmState,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            (*cdict).dictContent,
            (*cdict).dictContentSize,
            (*cdict).dictContentType,
            dtlm,
            ZSTD_tfp_forCCtx,
            (*cctx).entropyWorkspace as *mut libc::c_void,
        )
    } else {
        ZSTD_compress_insertDictionary(
            (*cctx).blockState.prevCBlock,
            &mut (*cctx).blockState.matchState,
            &mut (*cctx).ldmState,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            dict,
            dictSize,
            dictContentType,
            dtlm,
            ZSTD_tfp_forCCtx,
            (*cctx).entropyWorkspace as *mut libc::c_void,
        )
    };
    let err_code_0 = dictID;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if dictID
        <= (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
    {} else {
        __assert_fail(
            b"dictID <= UINT_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5040 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 197],
                &[libc::c_char; 197],
            >(
                b"size_t ZSTD_compressBegin_internal(ZSTD_CCtx *, const void *, size_t, ZSTD_dictContentType_e, ZSTD_dictTableLoadMethod_e, const ZSTD_CDict *, const ZSTD_CCtx_params *, U64, ZSTD_buffered_policy_e)\0",
            ))
                .as_ptr(),
        );
    }
    (*cctx).dictID = dictID as U32;
    (*cctx).dictContentSize = dictContentSize;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_advanced_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    let err_code = ZSTD_checkCParams((*params).cParams);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        dictContentType,
        dtlm,
        cdict,
        params,
        pledgedSrcSize as U64,
        ZSTDb_not_buffered,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut params: ZSTD_parameters,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    let mut cctxParams = ZSTD_CCtx_params {
        format: ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        useBlockSplitter: ZSTD_ps_auto,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        useSequenceProducer: 0,
        maxBlockSize: 0,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    ZSTD_CCtxParams_init_internal(&mut cctxParams, &mut params, ZSTD_NO_CLEVEL);
    return ZSTD_compressBegin_advanced_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        NULL as *const ZSTD_CDict,
        &mut cctxParams,
        pledgedSrcSize,
    );
}
unsafe extern "C" fn ZSTD_compressBegin_usingDict_deprecated(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    let mut cctxParams = ZSTD_CCtx_params {
        format: ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        useBlockSplitter: ZSTD_ps_auto,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        useSequenceProducer: 0,
        maxBlockSize: 0,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    let params = ZSTD_getParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_noAttachDict,
    );
    ZSTD_CCtxParams_init_internal(
        &mut cctxParams,
        &params,
        if compressionLevel == 0 as libc::c_int {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        },
    );
    return ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        NULL as *const ZSTD_CDict,
        &mut cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN as U64,
        ZSTDb_not_buffered,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingDict(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    return ZSTD_compressBegin_usingDict_deprecated(
        cctx,
        dict,
        dictSize,
        compressionLevel,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin(
    mut cctx: *mut ZSTD_CCtx,
    mut compressionLevel: libc::c_int,
) -> size_t {
    return ZSTD_compressBegin_usingDict_deprecated(
        cctx,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
        compressionLevel,
    );
}
unsafe extern "C" fn ZSTD_writeEpilogue(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let mut fhSize = 0 as libc::c_int as size_t;
    if (*cctx).stage as libc::c_uint == ZSTDcs_created as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t;
    }
    if (*cctx).stage as libc::c_uint == ZSTDcs_init as libc::c_int as libc::c_uint {
        fhSize = ZSTD_writeFrameHeader(
            dst,
            dstCapacity,
            &mut (*cctx).appliedParams,
            0 as libc::c_int as U64,
            0 as libc::c_int as U32,
        );
        let err_code = fhSize;
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(fhSize) as size_t
            as size_t;
        op = op.offset(fhSize as isize);
        (*cctx).stage = ZSTDcs_ongoing;
    }
    if (*cctx).stage as libc::c_uint != ZSTDcs_ending as libc::c_int as libc::c_uint {
        let cBlockHeader24 = (1 as libc::c_int as libc::c_uint)
            .wrapping_add((bt_raw as libc::c_int as U32) << 1 as libc::c_int)
            .wrapping_add(0 as libc::c_int as libc::c_uint);
        if dstCapacity < 4 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        MEM_writeLE32(op as *mut libc::c_void, cBlockHeader24);
        op = op.offset(ZSTD_blockHeaderSize as isize);
        dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize)
            as size_t as size_t;
    }
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        let checksum = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as U32;
        if dstCapacity < 4 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        MEM_writeLE32(op as *mut libc::c_void, checksum);
        op = op.offset(4 as libc::c_int as isize);
    }
    (*cctx).stage = ZSTDcs_created;
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_trace(
    mut cctx: *mut ZSTD_CCtx,
    mut extraCSize: size_t,
) {
    if (*cctx).traceCtx != 0
        && (Some(
            ZSTD_trace_compress_end
                as unsafe extern "C" fn(ZSTD_TraceCtx, *const ZSTD_Trace) -> (),
        ))
            .is_some()
    {
        let streaming = ((*cctx).inBuffSize > 0 as libc::c_int as libc::c_ulong
            || (*cctx).outBuffSize > 0 as libc::c_int as libc::c_ulong
            || (*cctx).appliedParams.nbWorkers > 0 as libc::c_int) as libc::c_int;
        let mut trace = ZSTD_Trace {
            version: 0,
            streaming: 0,
            dictionaryID: 0,
            dictionaryIsCold: 0,
            dictionarySize: 0,
            uncompressedSize: 0,
            compressedSize: 0,
            params: 0 as *const ZSTD_CCtx_params_s,
            cctx: 0 as *const ZSTD_CCtx_s,
            dctx: 0 as *const ZSTD_DCtx_s,
        };
        libc::memset(
            &mut trace as *mut ZSTD_Trace as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ZSTD_Trace>() as libc::c_ulong as libc::size_t,
        );
        trace.version = ZSTD_VERSION_NUMBER as libc::c_uint;
        trace.streaming = streaming as libc::c_uint;
        trace.dictionaryID = (*cctx).dictID;
        trace.dictionarySize = (*cctx).dictContentSize;
        trace.uncompressedSize = (*cctx).consumedSrcSize as size_t;
        trace
            .compressedSize = ((*cctx).producedCSize)
            .wrapping_add(extraCSize as libc::c_ulonglong) as size_t;
        trace.params = &mut (*cctx).appliedParams;
        trace.cctx = cctx;
        ZSTD_trace_compress_end((*cctx).traceCtx, &mut trace);
    }
    (*cctx).traceCtx = 0 as libc::c_int as ZSTD_TraceCtx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressEnd_public(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut endResult: size_t = 0;
    let cSize = ZSTD_compressContinue_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        1 as libc::c_int as U32,
        1 as libc::c_int as U32,
    );
    let err_code = cSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    endResult = ZSTD_writeEpilogue(
        cctx,
        (dst as *mut libc::c_char).offset(cSize as isize) as *mut libc::c_void,
        dstCapacity.wrapping_sub(cSize),
    );
    let err_code_0 = endResult;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if !((*cctx).appliedParams.fParams.contentSizeFlag != 0
        && (*cctx).pledgedSrcSizePlusOne == 0 as libc::c_int as libc::c_ulonglong)
    {} else {
        __assert_fail(
            b"!(cctx->appliedParams.fParams.contentSizeFlag && cctx->pledgedSrcSizePlusOne == 0)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5180 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"size_t ZSTD_compressEnd_public(ZSTD_CCtx *, void *, size_t, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cctx).pledgedSrcSizePlusOne != 0 as libc::c_int as libc::c_ulonglong {
        if (*cctx).pledgedSrcSizePlusOne
            != ((*cctx).consumedSrcSize)
                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
    }
    ZSTD_CCtx_trace(cctx, endResult);
    return cSize.wrapping_add(endResult);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressEnd(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut params: ZSTD_parameters,
) -> size_t {
    let err_code = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    ZSTD_CCtxParams_init_internal(
        &mut (*cctx).simpleApiParams,
        &mut params,
        ZSTD_NO_CLEVEL,
    );
    return ZSTD_compress_advanced_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        &mut (*cctx).simpleApiParams,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_advanced_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut params: *const ZSTD_CCtx_params,
) -> size_t {
    let err_code = ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        0 as *const ZSTD_CDict,
        params,
        srcSize,
        ZSTDb_not_buffered,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_usingDict(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    let params = ZSTD_getParams_internal(
        compressionLevel,
        srcSize as libc::c_ulonglong,
        if !dict.is_null() { dictSize } else { 0 as libc::c_int as libc::c_ulong },
        ZSTD_cpm_noAttachDict,
    );
    if params.fParams.contentSizeFlag == 1 as libc::c_int {} else {
        __assert_fail(
            b"params.fParams.contentSizeFlag == 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5242 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_compress_usingDict(ZSTD_CCtx *, void *, size_t, const void *, size_t, const void *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_CCtxParams_init_internal(
        &mut (*cctx).simpleApiParams,
        &params,
        if compressionLevel == 0 as libc::c_int {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        },
    );
    return ZSTD_compress_advanced_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        &mut (*cctx).simpleApiParams,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressCCtx(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    if !cctx.is_null() {} else {
        __assert_fail(
            b"cctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5255 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"size_t ZSTD_compressCCtx(ZSTD_CCtx *, void *, size_t, const void *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_compress_usingDict(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
        compressionLevel,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    let mut result: size_t = 0;
    let mut ctxBody = ZSTD_CCtx {
        stage: ZSTDcs_created,
        cParamsChanged: 0,
        bmi2: 0,
        requestedParams: ZSTD_CCtx_params {
            format: ZSTD_f_zstd1,
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
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictDefaultAttach,
            literalCompressionMode: ZSTD_ps_auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            useBlockSplitter: ZSTD_ps_auto,
            useRowMatchFinder: ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem {
                customAlloc: None,
                customFree: None,
                opaque: 0 as *mut libc::c_void,
            },
            prefetchCDictTables: ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            useSequenceProducer: 0,
            maxBlockSize: 0,
            searchForExternalRepcodes: ZSTD_ps_auto,
        },
        appliedParams: ZSTD_CCtx_params {
            format: ZSTD_f_zstd1,
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
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictDefaultAttach,
            literalCompressionMode: ZSTD_ps_auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            useBlockSplitter: ZSTD_ps_auto,
            useRowMatchFinder: ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem {
                customAlloc: None,
                customFree: None,
                opaque: 0 as *mut libc::c_void,
            },
            prefetchCDictTables: ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            useSequenceProducer: 0,
            maxBlockSize: 0,
            searchForExternalRepcodes: ZSTD_ps_auto,
        },
        simpleApiParams: ZSTD_CCtx_params {
            format: ZSTD_f_zstd1,
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
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictDefaultAttach,
            literalCompressionMode: ZSTD_ps_auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            useBlockSplitter: ZSTD_ps_auto,
            useRowMatchFinder: ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem {
                customAlloc: None,
                customFree: None,
                opaque: 0 as *mut libc::c_void,
            },
            prefetchCDictTables: ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            useSequenceProducer: 0,
            maxBlockSize: 0,
            searchForExternalRepcodes: ZSTD_ps_auto,
        },
        dictID: 0,
        dictContentSize: 0,
        workspace: ZSTD_cwksp {
            workspace: 0 as *mut libc::c_void,
            workspaceEnd: 0 as *mut libc::c_void,
            objectEnd: 0 as *mut libc::c_void,
            tableEnd: 0 as *mut libc::c_void,
            tableValidEnd: 0 as *mut libc::c_void,
            allocStart: 0 as *mut libc::c_void,
            initOnceStart: 0 as *mut libc::c_void,
            allocFailed: 0,
            workspaceOversizedDuration: 0,
            phase: ZSTD_cwksp_alloc_objects,
            isStatic: ZSTD_cwksp_dynamic_alloc,
        },
        blockSize: 0,
        pledgedSrcSizePlusOne: 0,
        consumedSrcSize: 0,
        producedCSize: 0,
        xxhState: XXH64_state_t {
            total_len: 0,
            v: [0; 4],
            mem64: [0; 4],
            memsize: 0,
            reserved32: 0,
            reserved64: 0,
        },
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        pool: 0 as *mut ZSTD_threadPool,
        staticSize: 0,
        seqCollector: SeqCollector {
            collectSequences: 0,
            seqStart: 0 as *mut ZSTD_Sequence,
            seqIndex: 0,
            maxSequences: 0,
        },
        isFirstBlock: 0,
        initialized: 0,
        seqStore: seqStore_t {
            sequencesStart: 0 as *mut seqDef,
            sequences: 0 as *mut seqDef,
            litStart: 0 as *mut BYTE,
            lit: 0 as *mut BYTE,
            llCode: 0 as *mut BYTE,
            mlCode: 0 as *mut BYTE,
            ofCode: 0 as *mut BYTE,
            maxNbSeq: 0,
            maxNbLit: 0,
            longLengthType: ZSTD_llt_none,
            longLengthPos: 0,
        },
        ldmState: ldmState_t {
            window: ZSTD_window_t {
                nextSrc: 0 as *const BYTE,
                base: 0 as *const BYTE,
                dictBase: 0 as *const BYTE,
                dictLimit: 0,
                lowLimit: 0,
                nbOverflowCorrections: 0,
            },
            hashTable: 0 as *mut ldmEntry_t,
            loadedDictEnd: 0,
            bucketOffsets: 0 as *mut BYTE,
            splitIndices: [0; 64],
            matchCandidates: [ldmMatchCandidate_t {
                split: 0 as *const BYTE,
                hash: 0,
                checksum: 0,
                bucket: 0 as *mut ldmEntry_t,
            }; 64],
        },
        ldmSequences: 0 as *mut rawSeq,
        maxNbLdmSequences: 0,
        externSeqStore: rawSeqStore_t {
            seq: 0 as *mut rawSeq,
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        },
        blockState: ZSTD_blockState_t {
            prevCBlock: 0 as *mut ZSTD_compressedBlockState_t,
            nextCBlock: 0 as *mut ZSTD_compressedBlockState_t,
            matchState: ZSTD_matchState_t {
                window: ZSTD_window_t {
                    nextSrc: 0 as *const BYTE,
                    base: 0 as *const BYTE,
                    dictBase: 0 as *const BYTE,
                    dictLimit: 0,
                    lowLimit: 0,
                    nbOverflowCorrections: 0,
                },
                loadedDictEnd: 0,
                nextToUpdate: 0,
                hashLog3: 0,
                rowHashLog: 0,
                tagTable: 0 as *mut BYTE,
                hashCache: [0; 8],
                hashSalt: 0,
                hashSaltEntropy: 0,
                hashTable: 0 as *mut U32,
                hashTable3: 0 as *mut U32,
                chainTable: 0 as *mut U32,
                forceNonContiguous: 0,
                dedicatedDictSearch: 0,
                opt: optState_t {
                    litFreq: 0 as *mut libc::c_uint,
                    litLengthFreq: 0 as *mut libc::c_uint,
                    matchLengthFreq: 0 as *mut libc::c_uint,
                    offCodeFreq: 0 as *mut libc::c_uint,
                    matchTable: 0 as *mut ZSTD_match_t,
                    priceTable: 0 as *mut ZSTD_optimal_t,
                    litSum: 0,
                    litLengthSum: 0,
                    matchLengthSum: 0,
                    offCodeSum: 0,
                    litSumBasePrice: 0,
                    litLengthSumBasePrice: 0,
                    matchLengthSumBasePrice: 0,
                    offCodeSumBasePrice: 0,
                    priceType: zop_dynamic,
                    symbolCosts: 0 as *const ZSTD_entropyCTables_t,
                    literalCompressionMode: ZSTD_ps_auto,
                },
                dictMatchState: 0 as *const ZSTD_matchState_t,
                cParams: ZSTD_compressionParameters {
                    windowLog: 0,
                    chainLog: 0,
                    hashLog: 0,
                    searchLog: 0,
                    minMatch: 0,
                    targetLength: 0,
                    strategy: 0 as ZSTD_strategy,
                },
                ldmSeqStore: 0 as *const rawSeqStore_t,
                prefetchCDictTables: 0,
                lazySkipping: 0,
            },
        },
        entropyWorkspace: 0 as *mut U32,
        bufferedPolicy: ZSTDb_not_buffered,
        inBuff: 0 as *mut libc::c_char,
        inBuffSize: 0,
        inToCompress: 0,
        inBuffPos: 0,
        inBuffTarget: 0,
        outBuff: 0 as *mut libc::c_char,
        outBuffSize: 0,
        outBuffContentSize: 0,
        outBuffFlushedSize: 0,
        streamStage: zcss_init,
        frameEnded: 0,
        expectedInBuffer: ZSTD_inBuffer {
            src: 0 as *const libc::c_void,
            size: 0,
            pos: 0,
        },
        stableIn_notConsumed: 0,
        expectedOutBufferSize: 0,
        localDict: ZSTD_localDict {
            dictBuffer: 0 as *mut libc::c_void,
            dict: 0 as *const libc::c_void,
            dictSize: 0,
            dictContentType: ZSTD_dct_auto,
            cdict: 0 as *mut ZSTD_CDict,
        },
        cdict: 0 as *const ZSTD_CDict,
        prefixDict: ZSTD_prefixDict {
            dict: 0 as *const libc::c_void,
            dictSize: 0,
            dictContentType: ZSTD_dct_auto,
        },
        mtctx: 0 as *mut ZSTDMT_CCtx,
        traceCtx: 0,
        blockSplitCtx: ZSTD_blockSplitCtx {
            fullSeqStoreChunk: seqStore_t {
                sequencesStart: 0 as *mut seqDef,
                sequences: 0 as *mut seqDef,
                litStart: 0 as *mut BYTE,
                lit: 0 as *mut BYTE,
                llCode: 0 as *mut BYTE,
                mlCode: 0 as *mut BYTE,
                ofCode: 0 as *mut BYTE,
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            firstHalfSeqStore: seqStore_t {
                sequencesStart: 0 as *mut seqDef,
                sequences: 0 as *mut seqDef,
                litStart: 0 as *mut BYTE,
                lit: 0 as *mut BYTE,
                llCode: 0 as *mut BYTE,
                mlCode: 0 as *mut BYTE,
                ofCode: 0 as *mut BYTE,
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            secondHalfSeqStore: seqStore_t {
                sequencesStart: 0 as *mut seqDef,
                sequences: 0 as *mut seqDef,
                litStart: 0 as *mut BYTE,
                lit: 0 as *mut BYTE,
                llCode: 0 as *mut BYTE,
                mlCode: 0 as *mut BYTE,
                ofCode: 0 as *mut BYTE,
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            currSeqStore: seqStore_t {
                sequencesStart: 0 as *mut seqDef,
                sequences: 0 as *mut seqDef,
                litStart: 0 as *mut BYTE,
                lit: 0 as *mut BYTE,
                llCode: 0 as *mut BYTE,
                mlCode: 0 as *mut BYTE,
                ofCode: 0 as *mut BYTE,
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            nextSeqStore: seqStore_t {
                sequencesStart: 0 as *mut seqDef,
                sequences: 0 as *mut seqDef,
                litStart: 0 as *mut BYTE,
                lit: 0 as *mut BYTE,
                llCode: 0 as *mut BYTE,
                mlCode: 0 as *mut BYTE,
                ofCode: 0 as *mut BYTE,
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            partitions: [0; 196],
            entropyMetadata: ZSTD_entropyCTablesMetadata_t {
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
            },
        },
        externalMatchCtx: ZSTD_externalMatchCtx {
            mState: 0 as *mut libc::c_void,
            mFinder: None,
            seqBuffer: 0 as *mut ZSTD_Sequence,
            seqBufferCapacity: 0,
        },
    };
    ZSTD_initCCtx(&mut ctxBody, ZSTD_defaultCMem);
    result = ZSTD_compressCCtx(
        &mut ctxBody,
        dst,
        dstCapacity,
        src,
        srcSize,
        compressionLevel,
    );
    ZSTD_freeCCtxContent(&mut ctxBody);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCDictSize_advanced(
    mut dictSize: size_t,
    mut cParams: ZSTD_compressionParameters,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
) -> size_t {
    return (ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_CDict>() as libc::c_ulong))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE as size_t))
        .wrapping_add(
            ZSTD_sizeof_matchState(
                &mut cParams,
                ZSTD_resolveRowMatchFinderMode(ZSTD_ps_auto, &mut cParams),
                1 as libc::c_int as U32,
                0 as libc::c_int as U32,
            ),
        )
        .wrapping_add(
            (if dictLoadMethod as libc::c_uint
                == ZSTD_dlm_byRef as libc::c_int as libc::c_uint
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                ZSTD_cwksp_alloc_size(
                    ZSTD_cwksp_align(
                        dictSize,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
                )
            }),
        );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCDictSize(
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    return ZSTD_estimateCDictSize_advanced(dictSize, cParams, ZSTD_dlm_byCopy);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CDict(mut cdict: *const ZSTD_CDict) -> size_t {
    if cdict.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return (if (*cdict).workspace.workspace == cdict as *mut libc::c_void {
        0 as libc::c_int as libc::c_ulong
    } else {
        ::core::mem::size_of::<ZSTD_CDict>() as libc::c_ulong
    })
        .wrapping_add(ZSTD_cwksp_sizeof(&(*cdict).workspace));
}
unsafe extern "C" fn ZSTD_initCDict_internal(
    mut cdict: *mut ZSTD_CDict,
    mut dictBuffer: *const libc::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut params: ZSTD_CCtx_params,
) -> size_t {
    if ZSTD_checkCParams(params.cParams) == 0 {} else {
        __assert_fail(
            b"!ZSTD_checkCParams(params.cParams)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5321 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 132],
                &[libc::c_char; 132],
            >(
                b"size_t ZSTD_initCDict_internal(ZSTD_CDict *, const void *, size_t, ZSTD_dictLoadMethod_e, ZSTD_dictContentType_e, ZSTD_CCtx_params)\0",
            ))
                .as_ptr(),
        );
    }
    (*cdict).matchState.cParams = params.cParams;
    (*cdict).matchState.dedicatedDictSearch = params.enableDedicatedDictSearch;
    if dictLoadMethod as libc::c_uint == ZSTD_dlm_byRef as libc::c_int as libc::c_uint
        || dictBuffer.is_null() || dictSize == 0
    {
        (*cdict).dictContent = dictBuffer;
    } else {
        let mut internalBuffer = ZSTD_cwksp_reserve_object(
            &mut (*cdict).workspace,
            ZSTD_cwksp_align(
                dictSize,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
            ),
        );
        if internalBuffer.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
        (*cdict).dictContent = internalBuffer;
        libc::memcpy(internalBuffer, dictBuffer, dictSize as libc::size_t);
    }
    (*cdict).dictContentSize = dictSize;
    (*cdict).dictContentType = dictContentType;
    (*cdict)
        .entropyWorkspace = ZSTD_cwksp_reserve_object(
        &mut (*cdict).workspace,
        HUF_WORKSPACE_SIZE as size_t,
    ) as *mut U32;
    ZSTD_reset_compressedBlockState(&mut (*cdict).cBlockState);
    let err_code = ZSTD_reset_matchState(
        &mut (*cdict).matchState,
        &mut (*cdict).workspace,
        &mut params.cParams,
        params.useRowMatchFinder,
        ZSTDcrp_makeClean,
        ZSTDirp_reset,
        ZSTD_resetTarget_CDict,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    params.compressionLevel = ZSTD_CLEVEL_DEFAULT;
    params.fParams.contentSizeFlag = 1 as libc::c_int;
    let dictID = ZSTD_compress_insertDictionary(
        &mut (*cdict).cBlockState,
        &mut (*cdict).matchState,
        NULL as *mut ldmState_t,
        &mut (*cdict).workspace,
        &mut params,
        (*cdict).dictContent,
        (*cdict).dictContentSize,
        dictContentType,
        ZSTD_dtlm_full,
        ZSTD_tfp_forCDict,
        (*cdict).entropyWorkspace as *mut libc::c_void,
    );
    let err_code_0 = dictID;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if dictID <= -(1 as libc::c_int) as U32 as size_t {} else {
        __assert_fail(
            b"dictID <= (size_t)(U32)-1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5358 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 132],
                &[libc::c_char; 132],
            >(
                b"size_t ZSTD_initCDict_internal(ZSTD_CDict *, const void *, size_t, ZSTD_dictLoadMethod_e, ZSTD_dictContentType_e, ZSTD_CCtx_params)\0",
            ))
                .as_ptr(),
        );
    }
    (*cdict).dictID = dictID as U32;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_createCDict_advanced_internal(
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut cParams: ZSTD_compressionParameters,
    mut useRowMatchFinder: ZSTD_paramSwitch_e,
    mut enableDedicatedDictSearch: U32,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    if (customMem.customAlloc).is_none() as libc::c_int
        ^ (customMem.customFree).is_none() as libc::c_int != 0
    {
        return NULL as *mut ZSTD_CDict;
    }
    let workspaceSize = (ZSTD_cwksp_alloc_size(
        ::core::mem::size_of::<ZSTD_CDict>() as libc::c_ulong,
    ))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE as size_t))
        .wrapping_add(
            ZSTD_sizeof_matchState(
                &mut cParams,
                useRowMatchFinder,
                enableDedicatedDictSearch,
                0 as libc::c_int as U32,
            ),
        )
        .wrapping_add(
            (if dictLoadMethod as libc::c_uint
                == ZSTD_dlm_byRef as libc::c_int as libc::c_uint
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                ZSTD_cwksp_alloc_size(
                    ZSTD_cwksp_align(
                        dictSize,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
                )
            }),
        );
    let workspace = ZSTD_customMalloc(workspaceSize, customMem);
    let mut ws = ZSTD_cwksp {
        workspace: 0 as *mut libc::c_void,
        workspaceEnd: 0 as *mut libc::c_void,
        objectEnd: 0 as *mut libc::c_void,
        tableEnd: 0 as *mut libc::c_void,
        tableValidEnd: 0 as *mut libc::c_void,
        allocStart: 0 as *mut libc::c_void,
        initOnceStart: 0 as *mut libc::c_void,
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    let mut cdict = 0 as *mut ZSTD_CDict;
    if workspace.is_null() {
        ZSTD_customFree(workspace, customMem);
        return NULL as *mut ZSTD_CDict;
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_dynamic_alloc);
    cdict = ZSTD_cwksp_reserve_object(
        &mut ws,
        ::core::mem::size_of::<ZSTD_CDict>() as libc::c_ulong,
    ) as *mut ZSTD_CDict;
    if !cdict.is_null() {} else {
        __assert_fail(
            b"cdict != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5393 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 147],
                &[libc::c_char; 147],
            >(
                b"ZSTD_CDict *ZSTD_createCDict_advanced_internal(size_t, ZSTD_dictLoadMethod_e, ZSTD_compressionParameters, ZSTD_paramSwitch_e, U32, ZSTD_customMem)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    (*cdict).customMem = customMem;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL;
    (*cdict).useRowMatchFinder = useRowMatchFinder;
    return cdict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict_advanced(
    mut dictBuffer: *const libc::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cParams: ZSTD_compressionParameters,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let mut cctxParams = ZSTD_CCtx_params {
        format: ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        useBlockSplitter: ZSTD_ps_auto,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        useSequenceProducer: 0,
        maxBlockSize: 0,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    libc::memset(
        &mut cctxParams as *mut ZSTD_CCtx_params as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong as libc::size_t,
    );
    ZSTD_CCtxParams_init(&mut cctxParams, 0 as libc::c_int);
    cctxParams.cParams = cParams;
    cctxParams.customMem = customMem;
    return ZSTD_createCDict_advanced2(
        dictBuffer,
        dictSize,
        dictLoadMethod,
        dictContentType,
        &mut cctxParams,
        customMem,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict_advanced2(
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut originalCctxParams: *const ZSTD_CCtx_params,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let mut cctxParams = *originalCctxParams;
    let mut cParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0 as ZSTD_strategy,
    };
    let mut cdict = 0 as *mut ZSTD_CDict;
    if (customMem.customAlloc).is_none() as libc::c_int
        ^ (customMem.customFree).is_none() as libc::c_int != 0
    {
        return NULL as *mut ZSTD_CDict;
    }
    if cctxParams.enableDedicatedDictSearch != 0 {
        cParams = ZSTD_dedicatedDictSearch_getCParams(
            cctxParams.compressionLevel,
            dictSize,
        );
        ZSTD_overrideCParams(&mut cParams, &mut cctxParams.cParams);
    } else {
        cParams = ZSTD_getCParamsFromCCtxParams(
            &mut cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN as U64,
            dictSize,
            ZSTD_cpm_createCDict,
        );
    }
    if ZSTD_dedicatedDictSearch_isSupported(&mut cParams) == 0 {
        cctxParams.enableDedicatedDictSearch = 0 as libc::c_int;
        cParams = ZSTD_getCParamsFromCCtxParams(
            &mut cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN as U64,
            dictSize,
            ZSTD_cpm_createCDict,
        );
    }
    cctxParams.cParams = cParams;
    cctxParams
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        cctxParams.useRowMatchFinder,
        &mut cParams,
    );
    cdict = ZSTD_createCDict_advanced_internal(
        dictSize,
        dictLoadMethod,
        cctxParams.cParams,
        cctxParams.useRowMatchFinder,
        cctxParams.enableDedicatedDictSearch as U32,
        customMem,
    );
    if ERR_isError(
        ZSTD_initCDict_internal(
            cdict,
            dict,
            dictSize,
            dictLoadMethod,
            dictContentType,
            cctxParams,
        ),
    ) != 0
    {
        ZSTD_freeCDict(cdict);
        return NULL as *mut ZSTD_CDict;
    }
    return cdict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict(
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> *mut ZSTD_CDict {
    let mut cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    let cdict = ZSTD_createCDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
        cParams,
        ZSTD_defaultCMem,
    );
    if !cdict.is_null() {
        (*cdict)
            .compressionLevel = if compressionLevel == 0 as libc::c_int {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        };
    }
    return cdict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict_byReference(
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> *mut ZSTD_CDict {
    let mut cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    let cdict = ZSTD_createCDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        cParams,
        ZSTD_defaultCMem,
    );
    if !cdict.is_null() {
        (*cdict)
            .compressionLevel = if compressionLevel == 0 as libc::c_int {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        };
    }
    return cdict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCDict(mut cdict: *mut ZSTD_CDict) -> size_t {
    if cdict.is_null() {
        return 0 as libc::c_int as size_t;
    }
    let cMem = (*cdict).customMem;
    let mut cdictInWorkspace = ZSTD_cwksp_owns_buffer(
        &mut (*cdict).workspace,
        cdict as *const libc::c_void,
    );
    ZSTD_cwksp_free(&mut (*cdict).workspace, cMem);
    if cdictInWorkspace == 0 {
        ZSTD_customFree(cdict as *mut libc::c_void, cMem);
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCDict(
    mut workspace: *mut libc::c_void,
    mut workspaceSize: size_t,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cParams: ZSTD_compressionParameters,
) -> *const ZSTD_CDict {
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(ZSTD_ps_auto, &mut cParams);
    let matchStateSize = ZSTD_sizeof_matchState(
        &mut cParams,
        useRowMatchFinder,
        1 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
    let neededSize = (ZSTD_cwksp_alloc_size(
        ::core::mem::size_of::<ZSTD_CDict>() as libc::c_ulong,
    ))
        .wrapping_add(
            (if dictLoadMethod as libc::c_uint
                == ZSTD_dlm_byRef as libc::c_int as libc::c_uint
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                ZSTD_cwksp_alloc_size(
                    ZSTD_cwksp_align(
                        dictSize,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
                )
            }),
        )
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE as size_t))
        .wrapping_add(matchStateSize);
    let mut cdict = 0 as *mut ZSTD_CDict;
    let mut params = ZSTD_CCtx_params {
        format: ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        useBlockSplitter: ZSTD_ps_auto,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        useSequenceProducer: 0,
        maxBlockSize: 0,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    if workspace as size_t & 7 as libc::c_int as libc::c_ulong != 0 {
        return NULL as *const ZSTD_CDict;
    }
    let mut ws = ZSTD_cwksp {
        workspace: 0 as *mut libc::c_void,
        workspaceEnd: 0 as *mut libc::c_void,
        objectEnd: 0 as *mut libc::c_void,
        tableEnd: 0 as *mut libc::c_void,
        tableValidEnd: 0 as *mut libc::c_void,
        allocStart: 0 as *mut libc::c_void,
        initOnceStart: 0 as *mut libc::c_void,
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_static_alloc);
    cdict = ZSTD_cwksp_reserve_object(
        &mut ws,
        ::core::mem::size_of::<ZSTD_CDict>() as libc::c_ulong,
    ) as *mut ZSTD_CDict;
    if cdict.is_null() {
        return NULL as *const ZSTD_CDict;
    }
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    if workspaceSize < neededSize {
        return NULL as *const ZSTD_CDict;
    }
    ZSTD_CCtxParams_init(&mut params, 0 as libc::c_int);
    params.cParams = cParams;
    params.useRowMatchFinder = useRowMatchFinder;
    (*cdict).useRowMatchFinder = useRowMatchFinder;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL;
    if ERR_isError(
        ZSTD_initCDict_internal(
            cdict,
            dict,
            dictSize,
            dictLoadMethod,
            dictContentType,
            params,
        ),
    ) != 0
    {
        return NULL as *const ZSTD_CDict;
    }
    return cdict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParamsFromCDict(
    mut cdict: *const ZSTD_CDict,
) -> ZSTD_compressionParameters {
    if !cdict.is_null() {} else {
        __assert_fail(
            b"cdict != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5566 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"ZSTD_compressionParameters ZSTD_getCParamsFromCDict(const ZSTD_CDict *)\0",
            ))
                .as_ptr(),
        );
    }
    return (*cdict).matchState.cParams;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromCDict(
    mut cdict: *const ZSTD_CDict,
) -> libc::c_uint {
    if cdict.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    return (*cdict).dictID;
}
unsafe extern "C" fn ZSTD_compressBegin_usingCDict_internal(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    let mut cctxParams = ZSTD_CCtx_params {
        format: ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        useBlockSplitter: ZSTD_ps_auto,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: 0 as *mut libc::c_void,
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        useSequenceProducer: 0,
        maxBlockSize: 0,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    if cdict.is_null() {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t;
    }
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
    params.fParams = fParams;
    params
        .cParams = if pledgedSrcSize
        < ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF as libc::c_ulonglong
        || pledgedSrcSize
            < ((*cdict).dictContentSize as libc::c_ulonglong)
                .wrapping_mul(ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER)
        || pledgedSrcSize == ZSTD_CONTENTSIZE_UNKNOWN
        || (*cdict).compressionLevel == 0 as libc::c_int
    {
        ZSTD_getCParamsFromCDict(cdict)
    } else {
        ZSTD_getCParams(
            (*cdict).compressionLevel,
            pledgedSrcSize,
            (*cdict).dictContentSize,
        )
    };
    ZSTD_CCtxParams_init_internal(
        &mut cctxParams,
        &mut params,
        (*cdict).compressionLevel,
    );
    if pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let limitedSrcSize = (if pledgedSrcSize
            < ((1 as libc::c_uint) << 19 as libc::c_int) as libc::c_ulonglong
        {
            pledgedSrcSize
        } else {
            ((1 as libc::c_uint) << 19 as libc::c_int) as libc::c_ulonglong
        }) as U32;
        let limitedSrcLog = if limitedSrcSize > 1 as libc::c_int as libc::c_uint {
            (ZSTD_highbit32(
                limitedSrcSize.wrapping_sub(1 as libc::c_int as libc::c_uint),
            ))
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } else {
            1 as libc::c_int as libc::c_uint
        };
        cctxParams
            .cParams
            .windowLog = if cctxParams.cParams.windowLog > limitedSrcLog {
            cctxParams.cParams.windowLog
        } else {
            limitedSrcLog
        };
    }
    return ZSTD_compressBegin_internal(
        cctx,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        cdict,
        &mut cctxParams,
        pledgedSrcSize as U64,
        ZSTDb_not_buffered,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict_advanced(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    return ZSTD_compressBegin_usingCDict_internal(cctx, cdict, fParams, pledgedSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict_deprecated(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
) -> size_t {
    let fParams = {
        let mut init = ZSTD_frameParameters {
            contentSizeFlag: 0 as libc::c_int,
            checksumFlag: 0 as libc::c_int,
            noDictIDFlag: 0 as libc::c_int,
        };
        init
    };
    return ZSTD_compressBegin_usingCDict_internal(
        cctx,
        cdict,
        fParams,
        ZSTD_CONTENTSIZE_UNKNOWN,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
) -> size_t {
    return ZSTD_compressBegin_usingCDict_deprecated(cctx, cdict);
}
unsafe extern "C" fn ZSTD_compress_usingCDict_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut cdict: *const ZSTD_CDict,
    mut fParams: ZSTD_frameParameters,
) -> size_t {
    let err_code = ZSTD_compressBegin_usingCDict_internal(
        cctx,
        cdict,
        fParams,
        srcSize as libc::c_ulonglong,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_usingCDict_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut cdict: *const ZSTD_CDict,
    mut fParams: ZSTD_frameParameters,
) -> size_t {
    return ZSTD_compress_usingCDict_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        cdict,
        fParams,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_usingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut cdict: *const ZSTD_CDict,
) -> size_t {
    let fParams = {
        let mut init = ZSTD_frameParameters {
            contentSizeFlag: 1 as libc::c_int,
            checksumFlag: 0 as libc::c_int,
            noDictIDFlag: 0 as libc::c_int,
        };
        init
    };
    return ZSTD_compress_usingCDict_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        cdict,
        fParams,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCStream() -> *mut ZSTD_CStream {
    return ZSTD_createCStream_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCStream(
    mut workspace: *mut libc::c_void,
    mut workspaceSize: size_t,
) -> *mut ZSTD_CStream {
    return ZSTD_initStaticCCtx(workspace, workspaceSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCStream_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CStream {
    return ZSTD_createCCtx_advanced(customMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCStream(mut zcs: *mut ZSTD_CStream) -> size_t {
    return ZSTD_freeCCtx(zcs);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CStreamInSize() -> size_t {
    return ZSTD_BLOCKSIZE_MAX as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CStreamOutSize() -> size_t {
    return (ZSTD_compressBound(ZSTD_BLOCKSIZE_MAX as size_t))
        .wrapping_add(ZSTD_blockHeaderSize)
        .wrapping_add(4 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_getCParamMode(
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: U64,
) -> ZSTD_cParamMode_e {
    if !cdict.is_null() && ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) != 0 {
        return ZSTD_cpm_attachDict
    } else {
        return ZSTD_cpm_noAttachDict
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetCStream(
    mut zcs: *mut ZSTD_CStream,
    mut pss: libc::c_ulonglong,
) -> size_t {
    let pledgedSrcSize = (if pss == 0 as libc::c_int as libc::c_ulonglong {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    }) as U64;
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(
        zcs,
        pledgedSrcSize as libc::c_ulonglong,
    );
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_internal(
    mut zcs: *mut ZSTD_CStream,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if ERR_isError(ZSTD_checkCParams((*params).cParams)) == 0 {} else {
        __assert_fail(
            b"!ZSTD_isError(ZSTD_checkCParams(params->cParams))\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5754 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 137],
                &[libc::c_char; 137],
            >(
                b"size_t ZSTD_initCStream_internal(ZSTD_CStream *, const void *, size_t, const ZSTD_CDict *, const ZSTD_CCtx_params *, unsigned long long)\0",
            ))
                .as_ptr(),
        );
    }
    (*zcs).requestedParams = *params;
    if !(!dict.is_null() && !cdict.is_null()) {} else {
        __assert_fail(
            b"!((dict) && (cdict))\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5756 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 137],
                &[libc::c_char; 137],
            >(
                b"size_t ZSTD_initCStream_internal(ZSTD_CStream *, const void *, size_t, const ZSTD_CDict *, const ZSTD_CCtx_params *, unsigned long long)\0",
            ))
                .as_ptr(),
        );
    }
    if !dict.is_null() {
        let err_code_1 = ZSTD_CCtx_loadDictionary(zcs, dict, dictSize);
        if ERR_isError(err_code_1) != 0 {
            return err_code_1;
        }
    } else {
        let err_code_2 = ZSTD_CCtx_refCDict(zcs, cdict);
        if ERR_isError(err_code_2) != 0 {
            return err_code_2;
        }
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict_advanced(
    mut zcs: *mut ZSTD_CStream,
    mut cdict: *const ZSTD_CDict,
    mut fParams: ZSTD_frameParameters,
    mut pledgedSrcSize: libc::c_ulonglong,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    (*zcs).requestedParams.fParams = fParams;
    let err_code_1 = ZSTD_CCtx_refCDict(zcs, cdict);
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict(
    mut zcs: *mut ZSTD_CStream,
    mut cdict: *const ZSTD_CDict,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_refCDict(zcs, cdict);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_advanced(
    mut zcs: *mut ZSTD_CStream,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut params: ZSTD_parameters,
    mut pss: libc::c_ulonglong,
) -> size_t {
    let pledgedSrcSize = (if pss == 0 as libc::c_int as libc::c_ulonglong
        && params.fParams.contentSizeFlag == 0 as libc::c_int
    {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    }) as U64;
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(
        zcs,
        pledgedSrcSize as libc::c_ulonglong,
    );
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    ZSTD_CCtxParams_setZstdParams(&mut (*zcs).requestedParams, &mut params);
    let err_code_2 = ZSTD_CCtx_loadDictionary(zcs, dict, dictSize);
    if ERR_isError(err_code_2) != 0 {
        return err_code_2;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingDict(
    mut zcs: *mut ZSTD_CStream,
    mut dict: *const libc::c_void,
    mut dictSize: size_t,
    mut compressionLevel: libc::c_int,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        zcs,
        ZSTD_c_compressionLevel,
        compressionLevel,
    );
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_loadDictionary(zcs, dict, dictSize);
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_srcSize(
    mut zcs: *mut ZSTD_CStream,
    mut compressionLevel: libc::c_int,
    mut pss: libc::c_ulonglong,
) -> size_t {
    let pledgedSrcSize = (if pss == 0 as libc::c_int as libc::c_ulonglong {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    }) as U64;
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_refCDict(zcs, 0 as *const ZSTD_CDict);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        zcs,
        ZSTD_c_compressionLevel,
        compressionLevel,
    );
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    let err_code_2 = ZSTD_CCtx_setPledgedSrcSize(
        zcs,
        pledgedSrcSize as libc::c_ulonglong,
    );
    if ERR_isError(err_code_2) != 0 {
        return err_code_2;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream(
    mut zcs: *mut ZSTD_CStream,
    mut compressionLevel: libc::c_int,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_refCDict(zcs, 0 as *const ZSTD_CDict);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        zcs,
        ZSTD_c_compressionLevel,
        compressionLevel,
    );
    if ERR_isError(err_code_1) != 0 {
        return err_code_1;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_nextInputSizeHint(mut cctx: *const ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint
    {
        return ((*cctx).blockSize).wrapping_sub((*cctx).stableIn_notConsumed);
    }
    if (*cctx).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"cctx->appliedParams.inBufferMode == ZSTD_bm_buffered\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5853 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"size_t ZSTD_nextInputSizeHint(const ZSTD_CCtx *)\0"))
                .as_ptr(),
        );
    }
    let mut hintInSize = ((*cctx).inBuffTarget).wrapping_sub((*cctx).inBuffPos);
    if hintInSize == 0 as libc::c_int as libc::c_ulong {
        hintInSize = (*cctx).blockSize;
    }
    return hintInSize;
}
unsafe extern "C" fn ZSTD_compressStream_generic(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
    flushMode: ZSTD_EndDirective,
) -> size_t {
    if !input.is_null() {} else {
        __assert_fail(
            b"input != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5868 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    let istart = (*input).src as *const libc::c_char;
    let iend = if !istart.is_null() {
        istart.offset((*input).size as isize)
    } else {
        istart
    };
    let mut ip = if !istart.is_null() {
        istart.offset((*input).pos as isize)
    } else {
        istart
    };
    if !output.is_null() {} else {
        __assert_fail(
            b"output != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5871 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    let ostart = (*output).dst as *mut libc::c_char;
    let oend = if !ostart.is_null() {
        ostart.offset((*output).size as isize)
    } else {
        ostart
    };
    let mut op = if !ostart.is_null() {
        ostart.offset((*output).pos as isize)
    } else {
        ostart
    };
    let mut someMoreWork = 1 as libc::c_int as U32;
    if !zcs.is_null() {} else {
        __assert_fail(
            b"zcs != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5878 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    if (*zcs).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint
    {
        if (*input).pos >= (*zcs).stableIn_notConsumed {} else {
            __assert_fail(
                b"input->pos >= zcs->stableIn_notConsumed\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5880 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
        (*input)
            .pos = ((*input).pos as libc::c_ulong)
            .wrapping_sub((*zcs).stableIn_notConsumed) as size_t as size_t;
        ip = ip.offset(-((*zcs).stableIn_notConsumed as isize));
        (*zcs).stableIn_notConsumed = 0 as libc::c_int as size_t;
    }
    if (*zcs).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {
        if !((*zcs).inBuff).is_null() {} else {
            __assert_fail(
                b"zcs->inBuff != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5886 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*zcs).inBuffSize > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"zcs->inBuffSize > 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5887 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*zcs).appliedParams.outBufferMode as libc::c_uint
        == ZSTD_bm_buffered as libc::c_int as libc::c_uint
    {
        if !((*zcs).outBuff).is_null() {} else {
            __assert_fail(
                b"zcs->outBuff != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5890 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*zcs).outBuffSize > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"zcs->outBuffSize > 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5891 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if ((*input).src).is_null() {
        if (*input).size == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"input->size == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5893 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*input).pos <= (*input).size {} else {
        __assert_fail(
            b"input->pos <= input->size\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5894 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*output).dst).is_null() {
        if (*output).size == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"output->size == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                5895 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*output).pos <= (*output).size {} else {
        __assert_fail(
            b"output->pos <= output->size\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5896 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    if flushMode as U32 <= ZSTD_e_end as libc::c_int as U32 {} else {
        __assert_fail(
            b"(U32)flushMode <= (U32)ZSTD_e_end\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            5897 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    while someMoreWork != 0 {
        let mut current_block_155: u64;
        match (*zcs).streamStage as libc::c_uint {
            0 => return -(ZSTD_error_init_missing as libc::c_int) as size_t,
            1 => {
                if flushMode as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
                    && (oend.offset_from(op) as libc::c_long as size_t
                        >= ZSTD_compressBound(
                            iend.offset_from(ip) as libc::c_long as size_t,
                        )
                        || (*zcs).appliedParams.outBufferMode as libc::c_uint
                            == ZSTD_bm_stable as libc::c_int as libc::c_uint)
                    && (*zcs).inBuffPos == 0 as libc::c_int as libc::c_ulong
                {
                    let cSize = ZSTD_compressEnd_public(
                        zcs,
                        op as *mut libc::c_void,
                        oend.offset_from(op) as libc::c_long as size_t,
                        ip as *const libc::c_void,
                        iend.offset_from(ip) as libc::c_long as size_t,
                    );
                    let err_code = cSize;
                    if ERR_isError(err_code) != 0 {
                        return err_code;
                    }
                    ip = iend;
                    op = op.offset(cSize as isize);
                    (*zcs).frameEnded = 1 as libc::c_int as U32;
                    ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                    someMoreWork = 0 as libc::c_int as U32;
                    current_block_155 = 7293850626974290116;
                } else {
                    if (*zcs).appliedParams.inBufferMode as libc::c_uint
                        == ZSTD_bm_buffered as libc::c_int as libc::c_uint
                    {
                        let toLoad = ((*zcs).inBuffTarget)
                            .wrapping_sub((*zcs).inBuffPos);
                        let loaded = ZSTD_limitCopy(
                            ((*zcs).inBuff).offset((*zcs).inBuffPos as isize)
                                as *mut libc::c_void,
                            toLoad,
                            ip as *const libc::c_void,
                            iend.offset_from(ip) as libc::c_long as size_t,
                        );
                        (*zcs)
                            .inBuffPos = ((*zcs).inBuffPos as libc::c_ulong)
                            .wrapping_add(loaded) as size_t as size_t;
                        if !ip.is_null() {
                            ip = ip.offset(loaded as isize);
                        }
                        if flushMode as libc::c_uint
                            == ZSTD_e_continue as libc::c_int as libc::c_uint
                            && (*zcs).inBuffPos < (*zcs).inBuffTarget
                        {
                            someMoreWork = 0 as libc::c_int as U32;
                            current_block_155 = 7293850626974290116;
                        } else if flushMode as libc::c_uint
                            == ZSTD_e_flush as libc::c_int as libc::c_uint
                            && (*zcs).inBuffPos == (*zcs).inToCompress
                        {
                            someMoreWork = 0 as libc::c_int as U32;
                            current_block_155 = 7293850626974290116;
                        } else {
                            current_block_155 = 307447392441238883;
                        }
                    } else {
                        if (*zcs).appliedParams.inBufferMode as libc::c_uint
                            == ZSTD_bm_stable as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"zcs->appliedParams.inBufferMode == ZSTD_bm_stable\0"
                                    as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                                    as *const u8 as *const libc::c_char,
                                5940 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 111],
                                    &[libc::c_char; 111],
                                >(
                                    b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        if flushMode as libc::c_uint
                            == ZSTD_e_continue as libc::c_int as libc::c_uint
                            && (iend.offset_from(ip) as libc::c_long as size_t)
                                < (*zcs).blockSize
                        {
                            (*zcs)
                                .stableIn_notConsumed = iend.offset_from(ip) as libc::c_long
                                as size_t;
                            ip = iend;
                            someMoreWork = 0 as libc::c_int as U32;
                            current_block_155 = 7293850626974290116;
                        } else if flushMode as libc::c_uint
                            == ZSTD_e_flush as libc::c_int as libc::c_uint && ip == iend
                        {
                            someMoreWork = 0 as libc::c_int as U32;
                            current_block_155 = 7293850626974290116;
                        } else {
                            current_block_155 = 307447392441238883;
                        }
                    }
                    match current_block_155 {
                        7293850626974290116 => {}
                        _ => {
                            let inputBuffered = ((*zcs).appliedParams.inBufferMode
                                as libc::c_uint
                                == ZSTD_bm_buffered as libc::c_int as libc::c_uint)
                                as libc::c_int;
                            let mut cDst = 0 as *mut libc::c_void;
                            let mut cSize_0: size_t = 0;
                            let mut oSize = oend.offset_from(op) as libc::c_long
                                as size_t;
                            let iSize = if inputBuffered != 0 {
                                ((*zcs).inBuffPos).wrapping_sub((*zcs).inToCompress)
                            } else if (iend.offset_from(ip) as libc::c_long as size_t)
                                < (*zcs).blockSize
                            {
                                iend.offset_from(ip) as libc::c_long as size_t
                            } else {
                                (*zcs).blockSize
                            };
                            if oSize >= ZSTD_compressBound(iSize)
                                || (*zcs).appliedParams.outBufferMode as libc::c_uint
                                    == ZSTD_bm_stable as libc::c_int as libc::c_uint
                            {
                                cDst = op as *mut libc::c_void;
                            } else {
                                cDst = (*zcs).outBuff as *mut libc::c_void;
                                oSize = (*zcs).outBuffSize;
                            }
                            if inputBuffered != 0 {
                                let lastBlock = (flushMode as libc::c_uint
                                    == ZSTD_e_end as libc::c_int as libc::c_uint && ip == iend)
                                    as libc::c_int as libc::c_uint;
                                cSize_0 = if lastBlock != 0 {
                                    ZSTD_compressEnd_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ((*zcs).inBuff).offset((*zcs).inToCompress as isize)
                                            as *const libc::c_void,
                                        iSize,
                                    )
                                } else {
                                    ZSTD_compressContinue_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ((*zcs).inBuff).offset((*zcs).inToCompress as isize)
                                            as *const libc::c_void,
                                        iSize,
                                    )
                                };
                                let err_code_0 = cSize_0;
                                if ERR_isError(err_code_0) != 0 {
                                    return err_code_0;
                                }
                                (*zcs).frameEnded = lastBlock;
                                (*zcs)
                                    .inBuffTarget = ((*zcs).inBuffPos)
                                    .wrapping_add((*zcs).blockSize);
                                if (*zcs).inBuffTarget > (*zcs).inBuffSize {
                                    (*zcs).inBuffPos = 0 as libc::c_int as size_t;
                                    (*zcs).inBuffTarget = (*zcs).blockSize;
                                }
                                if lastBlock == 0 {
                                    if (*zcs).inBuffTarget <= (*zcs).inBuffSize {} else {
                                        __assert_fail(
                                            b"zcs->inBuffTarget <= zcs->inBuffSize\0" as *const u8
                                                as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                                                as *const u8 as *const libc::c_char,
                                            5982 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 111],
                                                &[libc::c_char; 111],
                                            >(
                                                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                }
                                (*zcs).inToCompress = (*zcs).inBuffPos;
                            } else {
                                let lastBlock_0 = (flushMode as libc::c_uint
                                    == ZSTD_e_end as libc::c_int as libc::c_uint
                                    && ip.offset(iSize as isize) == iend) as libc::c_int
                                    as libc::c_uint;
                                cSize_0 = if lastBlock_0 != 0 {
                                    ZSTD_compressEnd_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ip as *const libc::c_void,
                                        iSize,
                                    )
                                } else {
                                    ZSTD_compressContinue_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ip as *const libc::c_void,
                                        iSize,
                                    )
                                };
                                if !ip.is_null() {
                                    ip = ip.offset(iSize as isize);
                                }
                                let err_code_1 = cSize_0;
                                if ERR_isError(err_code_1) != 0 {
                                    return err_code_1;
                                }
                                (*zcs).frameEnded = lastBlock_0;
                                if lastBlock_0 != 0 {
                                    if ip == iend {} else {
                                        __assert_fail(
                                            b"ip == iend\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                                                as *const u8 as *const libc::c_char,
                                            5993 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 111],
                                                &[libc::c_char; 111],
                                            >(
                                                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                }
                            }
                            if cDst == op as *mut libc::c_void {
                                op = op.offset(cSize_0 as isize);
                                if (*zcs).frameEnded != 0 {
                                    someMoreWork = 0 as libc::c_int as U32;
                                    ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                                }
                                current_block_155 = 7293850626974290116;
                            } else {
                                (*zcs).outBuffContentSize = cSize_0;
                                (*zcs).outBuffFlushedSize = 0 as libc::c_int as size_t;
                                (*zcs).streamStage = zcss_flush;
                                current_block_155 = 5636883459695696059;
                            }
                        }
                    }
                }
            }
            2 => {
                current_block_155 = 5636883459695696059;
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    6038 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 111],
                        &[libc::c_char; 111],
                    >(
                        b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                    ))
                        .as_ptr(),
                );
                current_block_155 = 7293850626974290116;
            }
        }
        match current_block_155 {
            5636883459695696059 => {
                if (*zcs).appliedParams.outBufferMode as libc::c_uint
                    == ZSTD_bm_buffered as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"zcs->appliedParams.outBufferMode == ZSTD_bm_buffered\0"
                            as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        6011 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 111],
                            &[libc::c_char; 111],
                        >(
                            b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                        ))
                            .as_ptr(),
                    );
                }
                let toFlush = ((*zcs).outBuffContentSize)
                    .wrapping_sub((*zcs).outBuffFlushedSize);
                let flushed = ZSTD_limitCopy(
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as size_t,
                    ((*zcs).outBuff).offset((*zcs).outBuffFlushedSize as isize)
                        as *const libc::c_void,
                    toFlush,
                );
                if flushed != 0 {
                    op = op.offset(flushed as isize);
                }
                (*zcs)
                    .outBuffFlushedSize = ((*zcs).outBuffFlushedSize as libc::c_ulong)
                    .wrapping_add(flushed) as size_t as size_t;
                if toFlush != flushed {
                    if op == oend {} else {
                        __assert_fail(
                            b"op==oend\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                                as *const u8 as *const libc::c_char,
                            6022 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 111],
                                &[libc::c_char; 111],
                            >(
                                b"size_t ZSTD_compressStream_generic(ZSTD_CStream *, ZSTD_outBuffer *, ZSTD_inBuffer *, const ZSTD_EndDirective)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    someMoreWork = 0 as libc::c_int as U32;
                } else {
                    (*zcs).outBuffFlushedSize = 0 as libc::c_int as size_t;
                    (*zcs).outBuffContentSize = (*zcs).outBuffFlushedSize;
                    if (*zcs).frameEnded != 0 {
                        someMoreWork = 0 as libc::c_int as U32;
                        ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                    } else {
                        (*zcs).streamStage = zcss_load;
                    }
                }
            }
            _ => {}
        }
    }
    (*input).pos = ip.offset_from(istart) as libc::c_long as size_t;
    (*output).pos = op.offset_from(ostart) as libc::c_long as size_t;
    if (*zcs).frameEnded != 0 {
        return 0 as libc::c_int as size_t;
    }
    return ZSTD_nextInputSizeHint(zcs);
}
unsafe extern "C" fn ZSTD_nextInputSizeHint_MTorST(
    mut cctx: *const ZSTD_CCtx,
) -> size_t {
    if (*cctx).appliedParams.nbWorkers >= 1 as libc::c_int {
        if !((*cctx).mtctx).is_null() {} else {
            __assert_fail(
                b"cctx->mtctx != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6052 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"size_t ZSTD_nextInputSizeHint_MTorST(const ZSTD_CCtx *)\0"))
                    .as_ptr(),
            );
        }
        return ZSTDMT_nextInputSizeHint((*cctx).mtctx);
    }
    return ZSTD_nextInputSizeHint(cctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
) -> size_t {
    let err_code = ZSTD_compressStream2(zcs, output, input, ZSTD_e_continue);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return ZSTD_nextInputSizeHint_MTorST(zcs);
}
unsafe extern "C" fn ZSTD_setBufferExpectations(
    mut cctx: *mut ZSTD_CCtx,
    mut output: *const ZSTD_outBuffer,
    mut input: *const ZSTD_inBuffer,
) {
    if (*cctx).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint
    {
        (*cctx).expectedInBuffer = *input;
    }
    if (*cctx).appliedParams.outBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint
    {
        (*cctx).expectedOutBufferSize = ((*output).size).wrapping_sub((*output).pos);
    }
}
unsafe extern "C" fn ZSTD_checkBufferStability(
    mut cctx: *const ZSTD_CCtx,
    mut output: *const ZSTD_outBuffer,
    mut input: *const ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> size_t {
    if (*cctx).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint
    {
        let expect = (*cctx).expectedInBuffer;
        if expect.src != (*input).src || expect.pos != (*input).pos {
            return -(ZSTD_error_stabilityCondition_notRespected as libc::c_int)
                as size_t;
        }
    }
    if (*cctx).appliedParams.outBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint
    {
        let outBufferSize = ((*output).size).wrapping_sub((*output).pos);
        if (*cctx).expectedOutBufferSize != outBufferSize {
            return -(ZSTD_error_stabilityCondition_notRespected as libc::c_int)
                as size_t;
        }
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_CCtx_init_compressStream2(
    mut cctx: *mut ZSTD_CCtx,
    mut endOp: ZSTD_EndDirective,
    mut inSize: size_t,
) -> size_t {
    let mut params = (*cctx).requestedParams;
    let prefixDict = (*cctx).prefixDict;
    let err_code = ZSTD_initLocalDict(cctx);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    libc::memset(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_prefixDict>() as libc::c_ulong as libc::size_t,
    );
    if (prefixDict.dict).is_null() || ((*cctx).cdict).is_null() {} else {
        __assert_fail(
            b"prefixDict.dict==NULL || cctx->cdict==NULL\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"size_t ZSTD_CCtx_init_compressStream2(ZSTD_CCtx *, ZSTD_EndDirective, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !((*cctx).cdict).is_null() && ((*cctx).localDict.cdict).is_null() {
        params.compressionLevel = (*(*cctx).cdict).compressionLevel;
    }
    if endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint {
        (*cctx)
            .pledgedSrcSizePlusOne = inSize
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong;
    }
    let dictSize = if !(prefixDict.dict).is_null() {
        prefixDict.dictSize
    } else if !((*cctx).cdict).is_null() {
        (*(*cctx).cdict).dictContentSize
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mode = ZSTD_getCParamMode(
        (*cctx).cdict,
        &mut params,
        ((*cctx).pledgedSrcSizePlusOne)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as U64,
    );
    params
        .cParams = ZSTD_getCParamsFromCCtxParams(
        &mut params,
        ((*cctx).pledgedSrcSizePlusOne)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as U64,
        dictSize,
        mode,
    );
    params
        .useBlockSplitter = ZSTD_resolveBlockSplitterMode(
        params.useBlockSplitter,
        &mut params.cParams,
    );
    params
        .ldmParams
        .enableLdm = ZSTD_resolveEnableLdm(
        params.ldmParams.enableLdm,
        &mut params.cParams,
    );
    params
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        params.useRowMatchFinder,
        &mut params.cParams,
    );
    params
        .validateSequences = ZSTD_resolveExternalSequenceValidation(
        params.validateSequences,
    );
    params.maxBlockSize = ZSTD_resolveMaxBlockSize(params.maxBlockSize);
    params
        .searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        params.searchForExternalRepcodes,
        params.compressionLevel,
    );
    if params.useSequenceProducer == 1 as libc::c_int
        && params.nbWorkers >= 1 as libc::c_int
    {
        return -(ZSTD_error_parameter_combination_unsupported as libc::c_int) as size_t;
    }
    if ((*cctx).pledgedSrcSizePlusOne)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
        <= ZSTDMT_JOBSIZE_MIN as libc::c_ulonglong
    {
        params.nbWorkers = 0 as libc::c_int;
    }
    if params.nbWorkers > 0 as libc::c_int {
        (*cctx)
            .traceCtx = if (Some(
            ZSTD_trace_compress_begin
                as unsafe extern "C" fn(*const ZSTD_CCtx_s) -> ZSTD_TraceCtx,
        ))
            .is_some()
        {
            ZSTD_trace_compress_begin(cctx)
        } else {
            0 as libc::c_int as libc::c_ulonglong
        };
        if ((*cctx).mtctx).is_null() {
            (*cctx)
                .mtctx = ZSTDMT_createCCtx_advanced(
                params.nbWorkers as U32,
                (*cctx).customMem,
                (*cctx).pool,
            );
            if ((*cctx).mtctx).is_null() {
                return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
            }
        }
        let err_code_0 = ZSTDMT_initCStream_internal(
            (*cctx).mtctx,
            prefixDict.dict,
            prefixDict.dictSize,
            prefixDict.dictContentType,
            (*cctx).cdict,
            params,
            ((*cctx).pledgedSrcSizePlusOne)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong),
        );
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        (*cctx)
            .dictID = if !((*cctx).cdict).is_null() {
            (*(*cctx).cdict).dictID
        } else {
            0 as libc::c_int as libc::c_uint
        };
        (*cctx)
            .dictContentSize = if !((*cctx).cdict).is_null() {
            (*(*cctx).cdict).dictContentSize
        } else {
            prefixDict.dictSize
        };
        (*cctx).consumedSrcSize = 0 as libc::c_int as libc::c_ulonglong;
        (*cctx).producedCSize = 0 as libc::c_int as libc::c_ulonglong;
        (*cctx).streamStage = zcss_load;
        (*cctx).appliedParams = params;
    } else {
        let pledgedSrcSize = ((*cctx).pledgedSrcSizePlusOne)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as U64;
        if ERR_isError(ZSTD_checkCParams(params.cParams)) == 0 {} else {
            __assert_fail(
                b"!ZSTD_isError(ZSTD_checkCParams(params.cParams))\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6175 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"size_t ZSTD_CCtx_init_compressStream2(ZSTD_CCtx *, ZSTD_EndDirective, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        let err_code_1 = ZSTD_compressBegin_internal(
            cctx,
            prefixDict.dict,
            prefixDict.dictSize,
            prefixDict.dictContentType,
            ZSTD_dtlm_fast,
            (*cctx).cdict,
            &mut params,
            pledgedSrcSize,
            ZSTDb_buffered,
        );
        if ERR_isError(err_code_1) != 0 {
            return err_code_1;
        }
        if (*cctx).appliedParams.nbWorkers == 0 as libc::c_int {} else {
            __assert_fail(
                b"cctx->appliedParams.nbWorkers == 0\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6181 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"size_t ZSTD_CCtx_init_compressStream2(ZSTD_CCtx *, ZSTD_EndDirective, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        (*cctx).inToCompress = 0 as libc::c_int as size_t;
        (*cctx).inBuffPos = 0 as libc::c_int as size_t;
        if (*cctx).appliedParams.inBufferMode as libc::c_uint
            == ZSTD_bm_buffered as libc::c_int as libc::c_uint
        {
            (*cctx)
                .inBuffTarget = ((*cctx).blockSize)
                .wrapping_add(
                    ((*cctx).blockSize == pledgedSrcSize) as libc::c_int as libc::c_ulong,
                );
        } else {
            (*cctx).inBuffTarget = 0 as libc::c_int as size_t;
        }
        (*cctx).outBuffFlushedSize = 0 as libc::c_int as size_t;
        (*cctx).outBuffContentSize = (*cctx).outBuffFlushedSize;
        (*cctx).streamStage = zcss_load;
        (*cctx).frameEnded = 0 as libc::c_int as U32;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream2(
    mut cctx: *mut ZSTD_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> size_t {
    if (*output).pos > (*output).size {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if (*input).pos > (*input).size {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if endOp as U32 > ZSTD_e_end as libc::c_int as U32 {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if !cctx.is_null() {} else {
        __assert_fail(
            b"cctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6211 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"size_t ZSTD_compressStream2(ZSTD_CCtx *, ZSTD_outBuffer *, ZSTD_inBuffer *, ZSTD_EndDirective)\0",
            ))
                .as_ptr(),
        );
    }
    if (*cctx).streamStage as libc::c_uint == zcss_init as libc::c_int as libc::c_uint {
        let inputSize = ((*input).size).wrapping_sub((*input).pos);
        let totalInputSize = inputSize.wrapping_add((*cctx).stableIn_notConsumed);
        if (*cctx).requestedParams.inBufferMode as libc::c_uint
            == ZSTD_bm_stable as libc::c_int as libc::c_uint
            && endOp as libc::c_uint == ZSTD_e_continue as libc::c_int as libc::c_uint
            && totalInputSize < ZSTD_BLOCKSIZE_MAX as libc::c_ulong
        {
            if (*cctx).stableIn_notConsumed != 0 {
                if (*input).src != (*cctx).expectedInBuffer.src {
                    return -(ZSTD_error_stabilityCondition_notRespected as libc::c_int)
                        as size_t;
                }
                if (*input).pos != (*cctx).expectedInBuffer.size {
                    return -(ZSTD_error_stabilityCondition_notRespected as libc::c_int)
                        as size_t;
                }
            }
            (*input).pos = (*input).size;
            (*cctx).expectedInBuffer = *input;
            (*cctx)
                .stableIn_notConsumed = ((*cctx).stableIn_notConsumed as libc::c_ulong)
                .wrapping_add(inputSize) as size_t as size_t;
            return (if (*cctx).requestedParams.format as libc::c_uint
                == ZSTD_f_zstd1 as libc::c_int as libc::c_uint
            {
                6 as libc::c_int
            } else {
                2 as libc::c_int
            }) as size_t;
        }
        let err_code = ZSTD_CCtx_init_compressStream2(cctx, endOp, totalInputSize);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        ZSTD_setBufferExpectations(cctx, output, input);
    }
    let err_code_0 = ZSTD_checkBufferStability(cctx, output, input, endOp);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    if (*cctx).appliedParams.nbWorkers > 0 as libc::c_int {
        let mut flushMin: size_t = 0;
        if (*cctx).cParamsChanged != 0 {
            ZSTDMT_updateCParams_whileCompressing(
                (*cctx).mtctx,
                &mut (*cctx).requestedParams,
            );
            (*cctx).cParamsChanged = 0 as libc::c_int;
        }
        if (*cctx).stableIn_notConsumed != 0 {
            if (*cctx).appliedParams.inBufferMode as libc::c_uint
                == ZSTD_bm_stable as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"cctx->appliedParams.inBufferMode == ZSTD_bm_stable\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    6249 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 95],
                        &[libc::c_char; 95],
                    >(
                        b"size_t ZSTD_compressStream2(ZSTD_CCtx *, ZSTD_outBuffer *, ZSTD_inBuffer *, ZSTD_EndDirective)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (*input).pos >= (*cctx).stableIn_notConsumed {} else {
                __assert_fail(
                    b"input->pos >= cctx->stableIn_notConsumed\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    6251 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 95],
                        &[libc::c_char; 95],
                    >(
                        b"size_t ZSTD_compressStream2(ZSTD_CCtx *, ZSTD_outBuffer *, ZSTD_inBuffer *, ZSTD_EndDirective)\0",
                    ))
                        .as_ptr(),
                );
            }
            (*input)
                .pos = ((*input).pos as libc::c_ulong)
                .wrapping_sub((*cctx).stableIn_notConsumed) as size_t as size_t;
            (*cctx).stableIn_notConsumed = 0 as libc::c_int as size_t;
        }
        loop {
            let ipos = (*input).pos;
            let opos = (*output).pos;
            flushMin = ZSTDMT_compressStream_generic(
                (*cctx).mtctx,
                output,
                input,
                endOp,
            );
            (*cctx)
                .consumedSrcSize = ((*cctx).consumedSrcSize)
                .wrapping_add(((*input).pos).wrapping_sub(ipos) as libc::c_ulonglong);
            (*cctx)
                .producedCSize = ((*cctx).producedCSize)
                .wrapping_add(((*output).pos).wrapping_sub(opos) as libc::c_ulonglong);
            if ERR_isError(flushMin) != 0
                || endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
                    && flushMin == 0 as libc::c_int as libc::c_ulong
            {
                if flushMin == 0 as libc::c_int as libc::c_ulong {
                    ZSTD_CCtx_trace(cctx, 0 as libc::c_int as size_t);
                }
                ZSTD_CCtx_reset(cctx, ZSTD_reset_session_only);
            }
            let err_code_1 = flushMin;
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            if endOp as libc::c_uint == ZSTD_e_continue as libc::c_int as libc::c_uint {
                if (*input).pos != ipos || (*output).pos != opos
                    || (*input).pos == (*input).size || (*output).pos == (*output).size
                {
                    break;
                }
            } else {
                if endOp as libc::c_uint == ZSTD_e_flush as libc::c_int as libc::c_uint
                    || endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"endOp == ZSTD_e_flush || endOp == ZSTD_e_end\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        6277 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 95],
                            &[libc::c_char; 95],
                        >(
                            b"size_t ZSTD_compressStream2(ZSTD_CCtx *, ZSTD_outBuffer *, ZSTD_inBuffer *, ZSTD_EndDirective)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if flushMin == 0 as libc::c_int as libc::c_ulong
                    || (*output).pos == (*output).size
                {
                    break;
                }
            }
        }
        if endOp as libc::c_uint == ZSTD_e_continue as libc::c_int as libc::c_uint
            || flushMin == 0 as libc::c_int as libc::c_ulong
            || (*output).pos == (*output).size
        {} else {
            __assert_fail(
                b"endOp == ZSTD_e_continue || flushMin == 0 || output->pos == output->size\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6289 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"size_t ZSTD_compressStream2(ZSTD_CCtx *, ZSTD_outBuffer *, ZSTD_inBuffer *, ZSTD_EndDirective)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_setBufferExpectations(cctx, output, input);
        return flushMin;
    }
    let err_code_2 = ZSTD_compressStream_generic(cctx, output, input, endOp);
    if ERR_isError(err_code_2) != 0 {
        return err_code_2;
    }
    ZSTD_setBufferExpectations(cctx, output, input);
    return ((*cctx).outBuffContentSize).wrapping_sub((*cctx).outBuffFlushedSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream2_simpleArgs(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut dstPos: *mut size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut srcPos: *mut size_t,
    mut endOp: ZSTD_EndDirective,
) -> size_t {
    let mut output = ZSTD_outBuffer {
        dst: 0 as *mut libc::c_void,
        size: 0,
        pos: 0,
    };
    let mut input = ZSTD_inBuffer {
        src: 0 as *const libc::c_void,
        size: 0,
        pos: 0,
    };
    output.dst = dst;
    output.size = dstCapacity;
    output.pos = *dstPos;
    input.src = src;
    input.size = srcSize;
    input.pos = *srcPos;
    let cErr = ZSTD_compressStream2(cctx, &mut output, &mut input, endOp);
    *dstPos = output.pos;
    *srcPos = input.pos;
    return cErr;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress2(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let originalInBufferMode = (*cctx).requestedParams.inBufferMode;
    let originalOutBufferMode = (*cctx).requestedParams.outBufferMode;
    ZSTD_CCtx_reset(cctx, ZSTD_reset_session_only);
    (*cctx).requestedParams.inBufferMode = ZSTD_bm_stable;
    (*cctx).requestedParams.outBufferMode = ZSTD_bm_stable;
    let mut oPos = 0 as libc::c_int as size_t;
    let mut iPos = 0 as libc::c_int as size_t;
    let result = ZSTD_compressStream2_simpleArgs(
        cctx,
        dst,
        dstCapacity,
        &mut oPos,
        src,
        srcSize,
        &mut iPos,
        ZSTD_e_end,
    );
    (*cctx).requestedParams.inBufferMode = originalInBufferMode;
    (*cctx).requestedParams.outBufferMode = originalOutBufferMode;
    let err_code = result;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if result != 0 as libc::c_int as libc::c_ulong {
        if oPos == dstCapacity {} else {
            __assert_fail(
                b"oPos == dstCapacity\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6345 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"size_t ZSTD_compress2(ZSTD_CCtx *, void *, size_t, const void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if iPos == srcSize {} else {
        __assert_fail(
            b"iPos == srcSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6348 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"size_t ZSTD_compress2(ZSTD_CCtx *, void *, size_t, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    return oPos;
}
unsafe extern "C" fn ZSTD_validateSequence(
    mut offCode: U32,
    mut matchLength: U32,
    mut minMatch: U32,
    mut posInSrc: size_t,
    mut windowLog: U32,
    mut dictSize: size_t,
    mut useSequenceProducer: libc::c_int,
) -> size_t {
    let windowSize = (1 as libc::c_uint) << windowLog;
    let offsetBound = if posInSrc > windowSize as libc::c_ulong {
        windowSize as size_t
    } else {
        posInSrc.wrapping_add(dictSize)
    };
    let matchLenLowerBound = (if minMatch == 3 as libc::c_int as libc::c_uint
        || useSequenceProducer != 0
    {
        3 as libc::c_int
    } else {
        4 as libc::c_int
    }) as size_t;
    if offsetBound > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"(offsetBound)>0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6369 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"size_t ZSTD_validateSequence(U32, U32, U32, size_t, U32, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    if offCode as libc::c_ulong
        > offsetBound.wrapping_add(3 as libc::c_int as libc::c_ulong)
    {
        return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
    }
    if (matchLength as libc::c_ulong) < matchLenLowerBound {
        return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_finalizeOffBase(
    mut rawOffset: U32,
    mut rep: *const U32,
    mut ll0: U32,
) -> U32 {
    if rawOffset > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"(rawOffset)>0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6378 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                .as_ptr(),
        );
    }
    let mut offBase = rawOffset.wrapping_add(ZSTD_REP_NUM as libc::c_uint);
    if ll0 == 0 && rawOffset == *rep.offset(0 as libc::c_int as isize) {
        if 1 as libc::c_int >= 1 as libc::c_int {} else {
            __assert_fail(
                b"(1)>=1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6381 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        if 1 as libc::c_int <= 3 as libc::c_int {} else {
            __assert_fail(
                b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6381 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        offBase = 1 as libc::c_int as U32;
    } else if rawOffset == *rep.offset(1 as libc::c_int as isize) {
        if (2 as libc::c_int as libc::c_uint).wrapping_sub(ll0)
            >= 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"(2 - ll0)>=1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6383 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        if (2 as libc::c_int as libc::c_uint).wrapping_sub(ll0)
            <= 3 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"(2 - ll0)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6383 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        offBase = (2 as libc::c_int as libc::c_uint).wrapping_sub(ll0);
    } else if rawOffset == *rep.offset(2 as libc::c_int as isize) {
        if (3 as libc::c_int as libc::c_uint).wrapping_sub(ll0)
            >= 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"(3 - ll0)>=1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6385 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        if (3 as libc::c_int as libc::c_uint).wrapping_sub(ll0)
            <= 3 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"(3 - ll0)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6385 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        offBase = (3 as libc::c_int as libc::c_uint).wrapping_sub(ll0);
    } else if ll0 != 0
        && rawOffset
            == (*rep.offset(0 as libc::c_int as isize))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        if 3 as libc::c_int >= 1 as libc::c_int {} else {
            __assert_fail(
                b"(3)>=1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6387 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        if 3 as libc::c_int <= 3 as libc::c_int {} else {
            __assert_fail(
                b"(3)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6387 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"U32 ZSTD_finalizeOffBase(U32, const U32 *, U32)\0"))
                    .as_ptr(),
            );
        }
        offBase = 3 as libc::c_int as U32;
    }
    return offBase;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copySequencesToSeqStoreExplicitBlockDelim(
    mut cctx: *mut ZSTD_CCtx,
    mut seqPos: *mut ZSTD_sequencePosition,
    inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: size_t,
    mut src: *const libc::c_void,
    mut blockSize: size_t,
    mut externalRepSearch: ZSTD_paramSwitch_e,
) -> size_t {
    let mut idx = (*seqPos).idx;
    let startIdx = idx;
    let mut ip = src as *const BYTE;
    let iend = ip.offset(blockSize as isize);
    let mut updatedRepcodes = repcodes_t { rep: [0; 3] };
    let mut dictSize: U32 = 0;
    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize as U32;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize as U32;
    } else {
        dictSize = 0 as libc::c_int as U32;
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut libc::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    while (idx as libc::c_ulong) < inSeqsSize
        && ((*inSeqs.offset(idx as isize)).matchLength
            != 0 as libc::c_int as libc::c_uint
            || (*inSeqs.offset(idx as isize)).offset != 0 as libc::c_int as libc::c_uint)
    {
        let litLength = (*inSeqs.offset(idx as isize)).litLength;
        let matchLength = (*inSeqs.offset(idx as isize)).matchLength;
        let mut offBase: U32 = 0;
        if externalRepSearch as libc::c_uint
            == ZSTD_ps_disable as libc::c_int as libc::c_uint
        {
            if (*inSeqs.offset(idx as isize)).offset > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"(inSeqs[idx].offset)>0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    6422 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 170],
                        &[libc::c_char; 170],
                    >(
                        b"size_t ZSTD_copySequencesToSeqStoreExplicitBlockDelim(ZSTD_CCtx *, ZSTD_sequencePosition *, const ZSTD_Sequence *const, size_t, const void *, size_t, ZSTD_paramSwitch_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            offBase = ((*inSeqs.offset(idx as isize)).offset)
                .wrapping_add(ZSTD_REP_NUM as libc::c_uint);
        } else {
            let ll0 = (litLength == 0 as libc::c_int as libc::c_uint) as libc::c_int
                as U32;
            offBase = ZSTD_finalizeOffBase(
                (*inSeqs.offset(idx as isize)).offset,
                (updatedRepcodes.rep).as_mut_ptr() as *const U32,
                ll0,
            );
            ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        }
        if (*cctx).appliedParams.validateSequences != 0 {
            (*seqPos)
                .posInSrc = ((*seqPos).posInSrc as libc::c_ulong)
                .wrapping_add(litLength.wrapping_add(matchLength) as libc::c_ulong)
                as size_t as size_t;
            let err_code = ZSTD_validateSequence(
                offBase,
                matchLength,
                (*cctx).appliedParams.cParams.minMatch,
                (*seqPos).posInSrc,
                (*cctx).appliedParams.cParams.windowLog,
                dictSize as size_t,
                (*cctx).appliedParams.useSequenceProducer,
            );
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
        }
        if idx.wrapping_sub((*seqPos).idx) as libc::c_ulong >= (*cctx).seqStore.maxNbSeq
        {
            return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
        }
        ZSTD_storeSeq(
            &mut (*cctx).seqStore,
            litLength as size_t,
            ip,
            iend,
            offBase,
            matchLength as size_t,
        );
        ip = ip.offset(matchLength.wrapping_add(litLength) as isize);
        idx = idx.wrapping_add(1);
    }
    if externalRepSearch as libc::c_uint != ZSTD_ps_auto as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"externalRepSearch != ZSTD_ps_auto\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6443 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 170],
                &[libc::c_char; 170],
            >(
                b"size_t ZSTD_copySequencesToSeqStoreExplicitBlockDelim(ZSTD_CCtx *, ZSTD_sequencePosition *, const ZSTD_Sequence *const, size_t, const void *, size_t, ZSTD_paramSwitch_e)\0",
            ))
                .as_ptr(),
        );
    }
    if idx >= startIdx {} else {
        __assert_fail(
            b"idx >= startIdx\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6444 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 170],
                &[libc::c_char; 170],
            >(
                b"size_t ZSTD_copySequencesToSeqStoreExplicitBlockDelim(ZSTD_CCtx *, ZSTD_sequencePosition *, const ZSTD_Sequence *const, size_t, const void *, size_t, ZSTD_paramSwitch_e)\0",
            ))
                .as_ptr(),
        );
    }
    if externalRepSearch as libc::c_uint
        == ZSTD_ps_disable as libc::c_int as libc::c_uint && idx != startIdx
    {
        let rep = (updatedRepcodes.rep).as_mut_ptr();
        let mut lastSeqIdx = idx.wrapping_sub(1 as libc::c_int as libc::c_uint);
        if lastSeqIdx >= startIdx.wrapping_add(2 as libc::c_int as libc::c_uint) {
            *rep
                .offset(
                    2 as libc::c_int as isize,
                ) = (*inSeqs
                .offset(
                    lastSeqIdx.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                ))
                .offset;
            *rep
                .offset(
                    1 as libc::c_int as isize,
                ) = (*inSeqs
                .offset(
                    lastSeqIdx.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ))
                .offset;
            *rep
                .offset(
                    0 as libc::c_int as isize,
                ) = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else if lastSeqIdx == startIdx.wrapping_add(1 as libc::c_int as libc::c_uint) {
            *rep
                .offset(
                    2 as libc::c_int as isize,
                ) = *rep.offset(0 as libc::c_int as isize);
            *rep
                .offset(
                    1 as libc::c_int as isize,
                ) = (*inSeqs
                .offset(
                    lastSeqIdx.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ))
                .offset;
            *rep
                .offset(
                    0 as libc::c_int as isize,
                ) = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else {
            if lastSeqIdx == startIdx {} else {
                __assert_fail(
                    b"lastSeqIdx == startIdx\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    6458 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 170],
                        &[libc::c_char; 170],
                    >(
                        b"size_t ZSTD_copySequencesToSeqStoreExplicitBlockDelim(ZSTD_CCtx *, ZSTD_sequencePosition *, const ZSTD_Sequence *const, size_t, const void *, size_t, ZSTD_paramSwitch_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            *rep
                .offset(
                    2 as libc::c_int as isize,
                ) = *rep.offset(1 as libc::c_int as isize);
            *rep
                .offset(
                    1 as libc::c_int as isize,
                ) = *rep.offset(0 as libc::c_int as isize);
            *rep
                .offset(
                    0 as libc::c_int as isize,
                ) = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        }
    }
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut libc::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    if (*inSeqs.offset(idx as isize)).litLength != 0 {
        ZSTD_storeLastLiterals(
            &mut (*cctx).seqStore,
            ip,
            (*inSeqs.offset(idx as isize)).litLength as size_t,
        );
        ip = ip.offset((*inSeqs.offset(idx as isize)).litLength as isize);
        (*seqPos)
            .posInSrc = ((*seqPos).posInSrc as libc::c_ulong)
            .wrapping_add((*inSeqs.offset(idx as isize)).litLength as libc::c_ulong)
            as size_t as size_t;
    }
    if ip != iend {
        return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
    }
    (*seqPos).idx = idx.wrapping_add(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copySequencesToSeqStoreNoBlockDelim(
    mut cctx: *mut ZSTD_CCtx,
    mut seqPos: *mut ZSTD_sequencePosition,
    inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: size_t,
    mut src: *const libc::c_void,
    mut blockSize: size_t,
    mut externalRepSearch: ZSTD_paramSwitch_e,
) -> size_t {
    let mut idx = (*seqPos).idx;
    let mut startPosInSequence = (*seqPos).posInSequence;
    let mut endPosInSequence = ((*seqPos).posInSequence).wrapping_add(blockSize as U32);
    let mut dictSize: size_t = 0;
    let mut ip = src as *const BYTE;
    let mut iend = ip.offset(blockSize as isize);
    let mut updatedRepcodes = repcodes_t { rep: [0; 3] };
    let mut bytesAdjustment = 0 as libc::c_int as U32;
    let mut finalMatchSplit = 0 as libc::c_int as U32;
    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize;
    } else {
        dictSize = 0 as libc::c_int as size_t;
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut libc::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    while endPosInSequence != 0 && (idx as libc::c_ulong) < inSeqsSize
        && finalMatchSplit == 0
    {
        let currSeq = *inSeqs.offset(idx as isize);
        let mut litLength = currSeq.litLength;
        let mut matchLength = currSeq.matchLength;
        let rawOffset = currSeq.offset;
        let mut offBase: U32 = 0;
        if endPosInSequence >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            if startPosInSequence >= litLength {
                startPosInSequence = (startPosInSequence as libc::c_uint)
                    .wrapping_sub(litLength) as U32 as U32;
                litLength = 0 as libc::c_int as U32;
                matchLength = (matchLength as libc::c_uint)
                    .wrapping_sub(startPosInSequence) as U32 as U32;
            } else {
                litLength = (litLength as libc::c_uint).wrapping_sub(startPosInSequence)
                    as U32 as U32;
            }
            endPosInSequence = (endPosInSequence as libc::c_uint)
                .wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength))
                as U32 as U32;
            startPosInSequence = 0 as libc::c_int as U32;
        } else {
            if !(endPosInSequence > litLength) {
                break;
            }
            let mut firstHalfMatchLength: U32 = 0;
            litLength = if startPosInSequence >= litLength {
                0 as libc::c_int as libc::c_uint
            } else {
                litLength.wrapping_sub(startPosInSequence)
            };
            firstHalfMatchLength = endPosInSequence
                .wrapping_sub(startPosInSequence)
                .wrapping_sub(litLength);
            if matchLength as libc::c_ulong > blockSize
                && firstHalfMatchLength >= (*cctx).appliedParams.cParams.minMatch
            {
                let mut secondHalfMatchLength = (currSeq.matchLength)
                    .wrapping_add(currSeq.litLength)
                    .wrapping_sub(endPosInSequence);
                if secondHalfMatchLength < (*cctx).appliedParams.cParams.minMatch {
                    endPosInSequence = (endPosInSequence as libc::c_uint)
                        .wrapping_sub(
                            ((*cctx).appliedParams.cParams.minMatch)
                                .wrapping_sub(secondHalfMatchLength),
                        ) as U32 as U32;
                    bytesAdjustment = ((*cctx).appliedParams.cParams.minMatch)
                        .wrapping_sub(secondHalfMatchLength);
                    firstHalfMatchLength = (firstHalfMatchLength as libc::c_uint)
                        .wrapping_sub(bytesAdjustment) as U32 as U32;
                }
                matchLength = firstHalfMatchLength;
                finalMatchSplit = 1 as libc::c_int as U32;
            } else {
                bytesAdjustment = endPosInSequence.wrapping_sub(currSeq.litLength);
                endPosInSequence = currSeq.litLength;
                break;
            }
        }
        let ll0 = (litLength == 0 as libc::c_int as libc::c_uint) as libc::c_int as U32;
        offBase = ZSTD_finalizeOffBase(
            rawOffset,
            (updatedRepcodes.rep).as_mut_ptr() as *const U32,
            ll0,
        );
        ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        if (*cctx).appliedParams.validateSequences != 0 {
            (*seqPos)
                .posInSrc = ((*seqPos).posInSrc as libc::c_ulong)
                .wrapping_add(litLength.wrapping_add(matchLength) as libc::c_ulong)
                as size_t as size_t;
            let err_code = ZSTD_validateSequence(
                offBase,
                matchLength,
                (*cctx).appliedParams.cParams.minMatch,
                (*seqPos).posInSrc,
                (*cctx).appliedParams.cParams.windowLog,
                dictSize,
                (*cctx).appliedParams.useSequenceProducer,
            );
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
        }
        if idx.wrapping_sub((*seqPos).idx) as libc::c_ulong >= (*cctx).seqStore.maxNbSeq
        {
            return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
        }
        ZSTD_storeSeq(
            &mut (*cctx).seqStore,
            litLength as size_t,
            ip,
            iend,
            offBase,
            matchLength as size_t,
        );
        ip = ip.offset(matchLength.wrapping_add(litLength) as isize);
        if finalMatchSplit == 0 {
            idx = idx.wrapping_add(1);
        }
    }
    if idx as libc::c_ulong == inSeqsSize
        || endPosInSequence
            <= ((*inSeqs.offset(idx as isize)).litLength)
                .wrapping_add((*inSeqs.offset(idx as isize)).matchLength)
    {} else {
        __assert_fail(
            b"idx == inSeqsSize || endPosInSequence <= inSeqs[idx].litLength + inSeqs[idx].matchLength\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6583 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 164],
                &[libc::c_char; 164],
            >(
                b"size_t ZSTD_copySequencesToSeqStoreNoBlockDelim(ZSTD_CCtx *, ZSTD_sequencePosition *, const ZSTD_Sequence *const, size_t, const void *, size_t, ZSTD_paramSwitch_e)\0",
            ))
                .as_ptr(),
        );
    }
    (*seqPos).idx = idx;
    (*seqPos).posInSequence = endPosInSequence;
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut libc::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    iend = iend.offset(-(bytesAdjustment as isize));
    if ip != iend {
        let mut lastLLSize = iend.offset_from(ip) as libc::c_long as U32;
        if ip <= iend {} else {
            __assert_fail(
                b"ip <= iend\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6592 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 164],
                    &[libc::c_char; 164],
                >(
                    b"size_t ZSTD_copySequencesToSeqStoreNoBlockDelim(ZSTD_CCtx *, ZSTD_sequencePosition *, const ZSTD_Sequence *const, size_t, const void *, size_t, ZSTD_paramSwitch_e)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_storeLastLiterals(&mut (*cctx).seqStore, ip, lastLLSize as size_t);
        (*seqPos)
            .posInSrc = ((*seqPos).posInSrc as libc::c_ulong)
            .wrapping_add(lastLLSize as libc::c_ulong) as size_t as size_t;
    }
    return bytesAdjustment as size_t;
}
unsafe extern "C" fn ZSTD_selectSequenceCopier(
    mut mode: ZSTD_sequenceFormat_e,
) -> ZSTD_sequenceCopier {
    let mut sequenceCopier: ZSTD_sequenceCopier = ::core::mem::transmute::<
        libc::intptr_t,
        ZSTD_sequenceCopier,
    >(NULL as libc::intptr_t);
    if ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam11, mode as libc::c_int) != 0
    {} else {
        __assert_fail(
            b"ZSTD_cParam_withinBounds(ZSTD_c_blockDelimiters, mode)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6607 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"ZSTD_sequenceCopier ZSTD_selectSequenceCopier(ZSTD_sequenceFormat_e)\0"))
                .as_ptr(),
        );
    }
    if mode as libc::c_uint
        == ZSTD_sf_explicitBlockDelimiters as libc::c_int as libc::c_uint
    {
        return Some(
            ZSTD_copySequencesToSeqStoreExplicitBlockDelim
                as unsafe extern "C" fn(
                    *mut ZSTD_CCtx,
                    *mut ZSTD_sequencePosition,
                    *const ZSTD_Sequence,
                    size_t,
                    *const libc::c_void,
                    size_t,
                    ZSTD_paramSwitch_e,
                ) -> size_t,
        )
    } else {
        if mode as libc::c_uint
            == ZSTD_sf_noBlockDelimiters as libc::c_int as libc::c_uint
        {
            return Some(
                ZSTD_copySequencesToSeqStoreNoBlockDelim
                    as unsafe extern "C" fn(
                        *mut ZSTD_CCtx,
                        *mut ZSTD_sequencePosition,
                        *const ZSTD_Sequence,
                        size_t,
                        *const libc::c_void,
                        size_t,
                        ZSTD_paramSwitch_e,
                    ) -> size_t,
            );
        }
    }
    if sequenceCopier.is_some() {} else {
        __assert_fail(
            b"sequenceCopier != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6613 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"ZSTD_sequenceCopier ZSTD_selectSequenceCopier(ZSTD_sequenceFormat_e)\0"))
                .as_ptr(),
        );
    }
    return sequenceCopier;
}
unsafe extern "C" fn blockSize_explicitDelimiter(
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: size_t,
    mut seqPos: ZSTD_sequencePosition,
) -> size_t {
    let mut end = 0 as libc::c_int;
    let mut blockSize = 0 as libc::c_int as size_t;
    let mut spos = seqPos.idx as size_t;
    if spos <= inSeqsSize {} else {
        __assert_fail(
            b"spos <= inSeqsSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6628 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"size_t blockSize_explicitDelimiter(const ZSTD_Sequence *, size_t, ZSTD_sequencePosition)\0",
            ))
                .as_ptr(),
        );
    }
    while spos < inSeqsSize {
        end = ((*inSeqs.offset(spos as isize)).offset
            == 0 as libc::c_int as libc::c_uint) as libc::c_int;
        blockSize = (blockSize as libc::c_ulong)
            .wrapping_add(
                ((*inSeqs.offset(spos as isize)).litLength)
                    .wrapping_add((*inSeqs.offset(spos as isize)).matchLength)
                    as libc::c_ulong,
            ) as size_t as size_t;
        if end != 0 {
            if (*inSeqs.offset(spos as isize)).matchLength
                != 0 as libc::c_int as libc::c_uint
            {
                return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
            }
            break;
        } else {
            spos = spos.wrapping_add(1);
        }
    }
    if end == 0 {
        return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
    }
    return blockSize;
}
unsafe extern "C" fn blockSize_noDelimiter(
    mut blockSize: size_t,
    mut remaining: size_t,
) -> size_t {
    let lastBlock = (remaining <= blockSize) as libc::c_int;
    return if lastBlock != 0 { remaining } else { blockSize };
}
unsafe extern "C" fn determine_blockSize(
    mut mode: ZSTD_sequenceFormat_e,
    mut blockSize: size_t,
    mut remaining: size_t,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: size_t,
    mut seqPos: ZSTD_sequencePosition,
) -> size_t {
    if mode as libc::c_uint == ZSTD_sf_noBlockDelimiters as libc::c_int as libc::c_uint {
        return blockSize_noDelimiter(blockSize, remaining);
    }
    let explicitBlockSize = blockSize_explicitDelimiter(inSeqs, inSeqsSize, seqPos);
    let err_code = explicitBlockSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if explicitBlockSize > blockSize {
        return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
    }
    if explicitBlockSize > remaining {
        return -(ZSTD_error_externalSequences_invalid as libc::c_int) as size_t;
    }
    return explicitBlockSize;
}
unsafe extern "C" fn ZSTD_compressSequences_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut cSize = 0 as libc::c_int as size_t;
    let mut remaining = srcSize;
    let mut seqPos = {
        let mut init = ZSTD_sequencePosition {
            idx: 0 as libc::c_int as U32,
            posInSequence: 0 as libc::c_int as U32,
            posInSrc: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let sequenceCopier = ZSTD_selectSequenceCopier(
        (*cctx).appliedParams.blockDelimiters,
    );
    if remaining == 0 as libc::c_int as libc::c_ulong {
        let cBlockHeader24 = (1 as libc::c_int as libc::c_uint)
            .wrapping_add((bt_raw as libc::c_int as U32) << 1 as libc::c_int);
        if dstCapacity < 4 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        MEM_writeLE32(op as *mut libc::c_void, cBlockHeader24);
        op = op.offset(ZSTD_blockHeaderSize as isize);
        dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize)
            as size_t as size_t;
        cSize = (cSize as libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize) as size_t
            as size_t;
    }
    while remaining != 0 {
        let mut compressedSeqsSize: size_t = 0;
        let mut cBlockSize: size_t = 0;
        let mut additionalByteAdjustment: size_t = 0;
        let mut blockSize = determine_blockSize(
            (*cctx).appliedParams.blockDelimiters,
            (*cctx).blockSize,
            remaining,
            inSeqs,
            inSeqsSize,
            seqPos,
        );
        let lastBlock = (blockSize == remaining) as libc::c_int as U32;
        let err_code = blockSize;
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        if blockSize <= remaining {} else {
            __assert_fail(
                b"blockSize <= remaining\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6707 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 121],
                    &[libc::c_char; 121],
                >(
                    b"size_t ZSTD_compressSequences_internal(ZSTD_CCtx *, void *, size_t, const ZSTD_Sequence *, size_t, const void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_resetSeqStore(&mut (*cctx).seqStore);
        additionalByteAdjustment = sequenceCopier
            .expect(
                "non-null function pointer",
            )(
            cctx,
            &mut seqPos,
            inSeqs,
            inSeqsSize,
            ip as *const libc::c_void,
            blockSize,
            (*cctx).appliedParams.searchForExternalRepcodes,
        );
        let err_code_0 = additionalByteAdjustment;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        blockSize = (blockSize as libc::c_ulong).wrapping_sub(additionalByteAdjustment)
            as size_t as size_t;
        if blockSize
            < (MIN_CBLOCK_SIZE as libc::c_ulong)
                .wrapping_add(ZSTD_blockHeaderSize)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            cBlockSize = ZSTD_noCompressBlock(
                op as *mut libc::c_void,
                dstCapacity,
                ip as *const libc::c_void,
                blockSize,
                lastBlock,
            );
            let err_code_1 = cBlockSize;
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            cSize = (cSize as libc::c_ulong).wrapping_add(cBlockSize) as size_t
                as size_t;
            ip = ip.offset(blockSize as isize);
            op = op.offset(cBlockSize as isize);
            remaining = (remaining as libc::c_ulong).wrapping_sub(blockSize) as size_t
                as size_t;
            dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(cBlockSize)
                as size_t as size_t;
        } else {
            if dstCapacity < ZSTD_blockHeaderSize {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            compressedSeqsSize = ZSTD_entropyCompressSeqStore(
                &mut (*cctx).seqStore,
                &mut (*(*cctx).blockState.prevCBlock).entropy,
                &mut (*(*cctx).blockState.nextCBlock).entropy,
                &mut (*cctx).appliedParams,
                op.offset(ZSTD_blockHeaderSize as isize) as *mut libc::c_void,
                dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                blockSize,
                (*cctx).entropyWorkspace as *mut libc::c_void,
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
                (*cctx).bmi2,
            );
            let err_code_2 = compressedSeqsSize;
            if ERR_isError(err_code_2) != 0 {
                return err_code_2;
            }
            if (*cctx).isFirstBlock == 0 && ZSTD_maybeRLE(&mut (*cctx).seqStore) != 0
                && ZSTD_isRLE(ip, blockSize) != 0
            {
                compressedSeqsSize = 1 as libc::c_int as size_t;
            }
            if compressedSeqsSize == 0 as libc::c_int as libc::c_ulong {
                cBlockSize = ZSTD_noCompressBlock(
                    op as *mut libc::c_void,
                    dstCapacity,
                    ip as *const libc::c_void,
                    blockSize,
                    lastBlock,
                );
                let err_code_3 = cBlockSize;
                if ERR_isError(err_code_3) != 0 {
                    return err_code_3;
                }
            } else if compressedSeqsSize == 1 as libc::c_int as libc::c_ulong {
                cBlockSize = ZSTD_rleCompressBlock(
                    op as *mut libc::c_void,
                    dstCapacity,
                    *ip,
                    blockSize,
                    lastBlock,
                );
                let err_code_4 = cBlockSize;
                if ERR_isError(err_code_4) != 0 {
                    return err_code_4;
                }
            } else {
                let mut cBlockHeader: U32 = 0;
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
                if (*(*cctx).blockState.prevCBlock).entropy.fse.offcode_repeatMode
                    as libc::c_uint == FSE_repeat_valid as libc::c_int as libc::c_uint
                {
                    (*(*cctx).blockState.prevCBlock)
                        .entropy
                        .fse
                        .offcode_repeatMode = FSE_repeat_check;
                }
                cBlockHeader = lastBlock
                    .wrapping_add(
                        (bt_compressed as libc::c_int as U32) << 1 as libc::c_int,
                    )
                    .wrapping_add((compressedSeqsSize << 3 as libc::c_int) as U32);
                MEM_writeLE24(op as *mut libc::c_void, cBlockHeader);
                cBlockSize = ZSTD_blockHeaderSize.wrapping_add(compressedSeqsSize);
            }
            cSize = (cSize as libc::c_ulong).wrapping_add(cBlockSize) as size_t
                as size_t;
            if lastBlock != 0 {
                break;
            }
            ip = ip.offset(blockSize as isize);
            op = op.offset(cBlockSize as isize);
            remaining = (remaining as libc::c_ulong).wrapping_sub(blockSize) as size_t
                as size_t;
            dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(cBlockSize)
                as size_t as size_t;
            (*cctx).isFirstBlock = 0 as libc::c_int;
        }
    }
    return cSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressSequences(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut op = dst as *mut BYTE;
    let mut cSize = 0 as libc::c_int as size_t;
    let mut compressedBlocksSize = 0 as libc::c_int as size_t;
    let mut frameHeaderSize = 0 as libc::c_int as size_t;
    if !cctx.is_null() {} else {
        __assert_fail(
            b"cctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0" as *const u8
                as *const libc::c_char,
            6804 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_compressSequences(ZSTD_CCtx *, void *, size_t, const ZSTD_Sequence *, size_t, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let err_code = ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, srcSize);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    frameHeaderSize = ZSTD_writeFrameHeader(
        op as *mut libc::c_void,
        dstCapacity,
        &mut (*cctx).appliedParams,
        srcSize,
        (*cctx).dictID,
    );
    op = op.offset(frameHeaderSize as isize);
    dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(frameHeaderSize) as size_t
        as size_t;
    cSize = (cSize as libc::c_ulong).wrapping_add(frameHeaderSize) as size_t as size_t;
    if (*cctx).appliedParams.fParams.checksumFlag != 0 && srcSize != 0 {
        ZSTD_XXH64_update(&mut (*cctx).xxhState, src, srcSize);
    }
    compressedBlocksSize = ZSTD_compressSequences_internal(
        cctx,
        op as *mut libc::c_void,
        dstCapacity,
        inSeqs,
        inSeqsSize,
        src,
        srcSize,
    );
    let err_code_0 = compressedBlocksSize;
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    cSize = (cSize as libc::c_ulong).wrapping_add(compressedBlocksSize) as size_t
        as size_t;
    dstCapacity = (dstCapacity as libc::c_ulong).wrapping_sub(compressedBlocksSize)
        as size_t as size_t;
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        let checksum = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as U32;
        if dstCapacity < 4 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        MEM_writeLE32(
            (dst as *mut libc::c_char).offset(cSize as isize) as *mut libc::c_void,
            checksum,
        );
        cSize = (cSize as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return cSize;
}
unsafe extern "C" fn inBuffer_forEndFlush(
    mut zcs: *const ZSTD_CStream,
) -> ZSTD_inBuffer {
    let nullInput = {
        let mut init = ZSTD_inBuffer_s {
            src: NULL as *const libc::c_void,
            size: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    let stableInput = ((*zcs).appliedParams.inBufferMode as libc::c_uint
        == ZSTD_bm_stable as libc::c_int as libc::c_uint) as libc::c_int;
    return if stableInput != 0 { (*zcs).expectedInBuffer } else { nullInput };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_flushStream(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
) -> size_t {
    let mut input = inBuffer_forEndFlush(zcs);
    input.size = input.pos;
    return ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_flush);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_endStream(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
) -> size_t {
    let mut input = inBuffer_forEndFlush(zcs);
    let remainingToFlush = ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_end);
    let err_code = remainingToFlush;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if (*zcs).appliedParams.nbWorkers > 0 as libc::c_int {
        return remainingToFlush;
    }
    let lastBlockSize = (if (*zcs).frameEnded != 0 {
        0 as libc::c_int
    } else {
        ZSTD_BLOCKHEADERSIZE
    }) as size_t;
    let checksumSize = (if (*zcs).frameEnded != 0 {
        0 as libc::c_int
    } else {
        (*zcs).appliedParams.fParams.checksumFlag * 4 as libc::c_int
    }) as size_t;
    let toFlush = remainingToFlush
        .wrapping_add(lastBlockSize)
        .wrapping_add(checksumSize);
    return toFlush;
}
pub const ZSTD_MAX_CLEVEL: libc::c_int = 22 as libc::c_int;
static mut ZSTD_defaultCParameters: [[ZSTD_compressionParameters; 23]; 4] = [
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 19 as libc::c_int as libc::c_uint,
                chainLog: 12 as libc::c_int as libc::c_uint,
                hashLog: 13 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 6 as libc::c_int as libc::c_uint,
                targetLength: 1 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 19 as libc::c_int as libc::c_uint,
                chainLog: 13 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 7 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 20 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 16 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 6 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 18 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 2 as libc::c_int as libc::c_uint,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 4 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 20 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 20 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 16 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 20 as libc::c_int as libc::c_uint,
                hashLog: 21 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 16 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 21 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 16 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 21 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 16 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 22 as libc::c_int as libc::c_uint,
                hashLog: 23 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 22 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 22 as libc::c_int as libc::c_uint,
                hashLog: 23 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 23 as libc::c_int as libc::c_uint,
                hashLog: 23 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22 as libc::c_int as libc::c_uint,
                chainLog: 22 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 48 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 23 as libc::c_int as libc::c_uint,
                chainLog: 23 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 64 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 23 as libc::c_int as libc::c_uint,
                chainLog: 23 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 64 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 23 as libc::c_int as libc::c_uint,
                chainLog: 24 as libc::c_int as libc::c_uint,
                hashLog: 22 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 25 as libc::c_int as libc::c_uint,
                chainLog: 25 as libc::c_int as libc::c_uint,
                hashLog: 23 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 26 as libc::c_int as libc::c_uint,
                chainLog: 26 as libc::c_int as libc::c_uint,
                hashLog: 24 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 27 as libc::c_int as libc::c_uint,
                chainLog: 27 as libc::c_int as libc::c_uint,
                hashLog: 25 as libc::c_int as libc::c_uint,
                searchLog: 9 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 999 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 12 as libc::c_int as libc::c_uint,
                hashLog: 13 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 1 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 13 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 6 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 16 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 2 as libc::c_int as libc::c_uint,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 17 as libc::c_int as libc::c_uint,
                hashLog: 18 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 2 as libc::c_int as libc::c_uint,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 4 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 4 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 12 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 12 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 16 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 128 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 128 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 8 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 128 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 8 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 10 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 12 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18 as libc::c_int as libc::c_uint,
                chainLog: 19 as libc::c_int as libc::c_uint,
                hashLog: 19 as libc::c_int as libc::c_uint,
                searchLog: 13 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 999 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 12 as libc::c_int as libc::c_uint,
                hashLog: 12 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 1 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 12 as libc::c_int as libc::c_uint,
                hashLog: 13 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 6 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 13 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 16 as libc::c_int as libc::c_uint,
                searchLog: 2 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 17 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 2 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 2 as libc::c_int as libc::c_uint,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 4 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 16 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 17 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 12 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 12 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 128 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 8 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 10 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 9 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17 as libc::c_int as libc::c_uint,
                chainLog: 18 as libc::c_int as libc::c_uint,
                hashLog: 17 as libc::c_int as libc::c_uint,
                searchLog: 11 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 999 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 12 as libc::c_int as libc::c_uint,
                hashLog: 13 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 1 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 5 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 1 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 2 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 0 as libc::c_int as libc::c_uint,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 2 as libc::c_int as libc::c_uint,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 4 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 14 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 8 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 9 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 8 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 3 as libc::c_int as libc::c_uint,
                minMatch: 4 as libc::c_int as libc::c_uint,
                targetLength: 12 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 4 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 24 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 14 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 32 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 64 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 5 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 48 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 6 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 128 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 7 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 8 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 256 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 8 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 9 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 512 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14 as libc::c_int as libc::c_uint,
                chainLog: 15 as libc::c_int as libc::c_uint,
                hashLog: 15 as libc::c_int as libc::c_uint,
                searchLog: 10 as libc::c_int as libc::c_uint,
                minMatch: 3 as libc::c_int as libc::c_uint,
                targetLength: 999 as libc::c_int as libc::c_uint,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
];
#[no_mangle]
pub unsafe extern "C" fn ZSTD_maxCLevel() -> libc::c_int {
    return ZSTD_MAX_CLEVEL;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_minCLevel() -> libc::c_int {
    return -ZSTD_TARGETLENGTH_MAX;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_defaultCLevel() -> libc::c_int {
    return ZSTD_CLEVEL_DEFAULT;
}
unsafe extern "C" fn ZSTD_dedicatedDictSearch_getCParams(
    compressionLevel: libc::c_int,
    dictSize: size_t,
) -> ZSTD_compressionParameters {
    let mut cParams = ZSTD_getCParams_internal(
        compressionLevel,
        0 as libc::c_int as libc::c_ulonglong,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    match cParams.strategy as libc::c_uint {
        3 | 4 | 5 => {
            cParams
                .hashLog = (cParams.hashLog)
                .wrapping_add(ZSTD_LAZY_DDSS_BUCKET_LOG as libc::c_uint);
        }
        1 | 2 | 6 | 7 | 8 | 9 | _ => {}
    }
    return cParams;
}
unsafe extern "C" fn ZSTD_dedicatedDictSearch_isSupported(
    mut cParams: *const ZSTD_compressionParameters,
) -> libc::c_int {
    return ((*cParams).strategy as libc::c_uint
        >= ZSTD_greedy as libc::c_int as libc::c_uint
        && (*cParams).strategy as libc::c_uint
            <= ZSTD_lazy2 as libc::c_int as libc::c_uint
        && (*cParams).hashLog > (*cParams).chainLog
        && (*cParams).chainLog <= 24 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_dedicatedDictSearch_revertCParams(
    mut cParams: *mut ZSTD_compressionParameters,
) {
    match (*cParams).strategy as libc::c_uint {
        3 | 4 | 5 => {
            (*cParams)
                .hashLog = ((*cParams).hashLog)
                .wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as libc::c_uint);
            if (*cParams).hashLog < ZSTD_HASHLOG_MIN as libc::c_uint {
                (*cParams).hashLog = ZSTD_HASHLOG_MIN as libc::c_uint;
            }
        }
        1 | 2 | 6 | 7 | 8 | 9 | _ => {}
    };
}
unsafe extern "C" fn ZSTD_getCParamRowSize(
    mut srcSizeHint: U64,
    mut dictSize: size_t,
    mut mode: ZSTD_cParamMode_e,
) -> U64 {
    match mode as libc::c_uint {
        3 | 0 | 2 => {}
        1 => {
            dictSize = 0 as libc::c_int as size_t;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress.c\0"
                    as *const u8 as *const libc::c_char,
                6945 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"U64 ZSTD_getCParamRowSize(U64, size_t, ZSTD_cParamMode_e)\0"))
                    .as_ptr(),
            );
        }
    }
    let unknown = (srcSizeHint as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN)
        as libc::c_int;
    let addedSize = (if unknown != 0 && dictSize > 0 as libc::c_int as libc::c_ulong {
        500 as libc::c_int
    } else {
        0 as libc::c_int
    }) as size_t;
    return (if unknown != 0 && dictSize == 0 as libc::c_int as libc::c_ulong {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        srcSizeHint.wrapping_add(dictSize).wrapping_add(addedSize) as libc::c_ulonglong
    }) as U64;
}
unsafe extern "C" fn ZSTD_getCParams_internal(
    mut compressionLevel: libc::c_int,
    mut srcSizeHint: libc::c_ulonglong,
    mut dictSize: size_t,
    mut mode: ZSTD_cParamMode_e,
) -> ZSTD_compressionParameters {
    let rSize = ZSTD_getCParamRowSize(srcSizeHint as U64, dictSize, mode);
    let tableID = ((rSize
        <= (256 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulong) as libc::c_int
        + (rSize
            <= (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong) as libc::c_int
        + (rSize
            <= (16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong) as libc::c_int) as U32;
    let mut row: libc::c_int = 0;
    if compressionLevel == 0 as libc::c_int {
        row = ZSTD_CLEVEL_DEFAULT;
    } else if compressionLevel < 0 as libc::c_int {
        row = 0 as libc::c_int;
    } else if compressionLevel > ZSTD_MAX_CLEVEL {
        row = ZSTD_MAX_CLEVEL;
    } else {
        row = compressionLevel;
    }
    let mut cp = ZSTD_defaultCParameters[tableID as usize][row as usize];
    if compressionLevel < 0 as libc::c_int {
        let clampedCompressionLevel = if ZSTD_minCLevel() > compressionLevel {
            ZSTD_minCLevel()
        } else {
            compressionLevel
        };
        cp.targetLength = -clampedCompressionLevel as libc::c_uint;
    }
    return ZSTD_adjustCParams_internal(cp, srcSizeHint, dictSize, mode, ZSTD_ps_auto);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParams(
    mut compressionLevel: libc::c_int,
    mut srcSizeHint: libc::c_ulonglong,
    mut dictSize: size_t,
) -> ZSTD_compressionParameters {
    if srcSizeHint == 0 as libc::c_int as libc::c_ulonglong {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    return ZSTD_getCParams_internal(
        compressionLevel,
        srcSizeHint,
        dictSize,
        ZSTD_cpm_unknown,
    );
}
unsafe extern "C" fn ZSTD_getParams_internal(
    mut compressionLevel: libc::c_int,
    mut srcSizeHint: libc::c_ulonglong,
    mut dictSize: size_t,
    mut mode: ZSTD_cParamMode_e,
) -> ZSTD_parameters {
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
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        srcSizeHint,
        dictSize,
        mode,
    );
    libc::memset(
        &mut params as *mut ZSTD_parameters as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_parameters>() as libc::c_ulong as libc::size_t,
    );
    params.cParams = cParams;
    params.fParams.contentSizeFlag = 1 as libc::c_int;
    return params;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getParams(
    mut compressionLevel: libc::c_int,
    mut srcSizeHint: libc::c_ulonglong,
    mut dictSize: size_t,
) -> ZSTD_parameters {
    if srcSizeHint == 0 as libc::c_int as libc::c_ulonglong {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    return ZSTD_getParams_internal(
        compressionLevel,
        srcSizeHint,
        dictSize,
        ZSTD_cpm_unknown,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_registerSequenceProducer(
    mut zc: *mut ZSTD_CCtx,
    mut mState: *mut libc::c_void,
    mut mFinder: Option::<ZSTD_sequenceProducer_F>,
) {
    if mFinder.is_some() {
        let mut emctx = ZSTD_externalMatchCtx {
            mState: 0 as *mut libc::c_void,
            mFinder: None,
            seqBuffer: 0 as *mut ZSTD_Sequence,
            seqBufferCapacity: 0,
        };
        emctx.mState = mState;
        emctx.mFinder = mFinder;
        emctx.seqBuffer = NULL as *mut ZSTD_Sequence;
        emctx.seqBufferCapacity = 0 as libc::c_int as size_t;
        (*zc).externalMatchCtx = emctx;
        (*zc).requestedParams.useSequenceProducer = 1 as libc::c_int;
    } else {
        libc::memset(
            &mut (*zc).externalMatchCtx as *mut ZSTD_externalMatchCtx
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ZSTD_externalMatchCtx>() as libc::c_ulong
                as libc::size_t,
        );
        (*zc).requestedParams.useSequenceProducer = 0 as libc::c_int;
    };
}
unsafe extern "C" fn run_static_initializers() {
    srcSizeTiers = [
        (16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulonglong,
        (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulonglong,
        (256 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulonglong,
        ZSTD_CONTENTSIZE_UNKNOWN,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];