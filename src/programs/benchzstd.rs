use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type BMK_timedFnState_s;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_DCtx_s;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn UTIL_getTotalFileSize(
        fileNamesTable: *const *const libc::c_char,
        nbFiles: libc::c_uint,
    ) -> U64;
    fn UTIL_getFileSize(infilename: *const libc::c_char) -> U64;
    fn UTIL_isDirectory(infilename: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: libc::c_int,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn UTIL_support_MT_measurements() -> libc::c_int;
    fn BMK_isSuccessful_runOutcome(outcome: BMK_runOutcome_t) -> libc::c_int;
    fn BMK_extract_runTime(outcome: BMK_runOutcome_t) -> BMK_runTime_t;
    fn BMK_benchTimedFn(
        timedFnState: *mut BMK_timedFnState_t,
        params: BMK_benchParams_t,
    ) -> BMK_runOutcome_t;
    fn BMK_isCompleted_TimedFn(timedFnState: *const BMK_timedFnState_t) -> libc::c_int;
    fn BMK_createTimedFnState(
        total_ms: libc::c_uint,
        run_ms: libc::c_uint,
    ) -> *mut BMK_timedFnState_t;
    fn BMK_freeTimedFnState(state: *mut BMK_timedFnState_t);
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    fn ZSTD_maxCLevel() -> libc::c_int;
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
    fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> size_t;
    fn ZSTD_CCtx_setParameter(
        cctx: *mut ZSTD_CCtx,
        param: ZSTD_cParameter,
        value: libc::c_int,
    ) -> size_t;
    fn ZSTD_CCtx_reset(cctx: *mut ZSTD_CCtx, reset: ZSTD_ResetDirective) -> size_t;
    fn ZSTD_compress2(
        cctx: *mut ZSTD_CCtx,
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_DCtx_reset(dctx: *mut ZSTD_DCtx, reset: ZSTD_ResetDirective) -> size_t;
    fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    fn ZSTD_CCtx_loadDictionary(
        cctx: *mut ZSTD_CCtx,
        dict: *const libc::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTD_DCtx_loadDictionary(
        dctx: *mut ZSTD_DCtx,
        dict: *const libc::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> size_t;
    fn ZSTD_findDecompressedSize(
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> libc::c_ulonglong;
    fn RDG_genBuffer(
        buffer: *mut libc::c_void,
        size: size_t,
        matchProba: libc::c_double,
        litProba: libc::c_double,
        seed: libc::c_uint,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __id_t = libc::c_uint;
pub type size_t = libc::c_ulong;
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
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type id_t = __id_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_runTime_t {
    pub nanoSecPerRun: libc::c_double,
    pub sumOfReturn: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_runOutcome_t {
    pub internal_never_ever_use_directly: BMK_runTime_t,
    pub error_result_never_ever_use_directly: size_t,
    pub error_tag_never_ever_use_directly: libc::c_int,
}
pub type BMK_benchFn_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> size_t,
>;
pub type BMK_initFn_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
pub type BMK_errorFn_t = Option::<unsafe extern "C" fn(size_t) -> libc::c_uint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchParams_t {
    pub benchFn: BMK_benchFn_t,
    pub benchPayload: *mut libc::c_void,
    pub initFn: BMK_initFn_t,
    pub initPayload: *mut libc::c_void,
    pub errorFn: BMK_errorFn_t,
    pub blockCount: size_t,
    pub srcBuffers: *const *const libc::c_void,
    pub srcSizes: *const size_t,
    pub dstBuffers: *const *mut libc::c_void,
    pub dstCapacities: *const size_t,
    pub blockResults: *mut size_t,
}
pub type BMK_timedFnState_t = BMK_timedFnState_s;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
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
pub type ZSTD_ResetDirective = libc::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_DStream = ZSTD_DCtx;
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
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
pub type XXH32_hash_t = uint32_t;
pub type xxh_u32 = XXH32_hash_t;
pub type XXH_alignment = libc::c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type xxh_u8 = uint8_t;
pub type XXH64_hash_t = uint64_t;
pub type xxh_u64 = XXH64_hash_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchResult_t {
    pub cSize: size_t,
    pub cSpeed: libc::c_ulonglong,
    pub dSpeed: libc::c_ulonglong,
    pub cMem: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchOutcome_t {
    pub internal_never_use_directly: BMK_benchResult_t,
    pub tag: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_advancedParams_t {
    pub mode: BMK_mode_t,
    pub nbSeconds: libc::c_uint,
    pub blockSize: size_t,
    pub nbWorkers: libc::c_int,
    pub realTime: libc::c_uint,
    pub additionalParam: libc::c_int,
    pub ldmFlag: libc::c_int,
    pub ldmMinMatch: libc::c_int,
    pub ldmHashLog: libc::c_int,
    pub ldmBucketSizeLog: libc::c_int,
    pub ldmHashRateLog: libc::c_int,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
    pub useRowMatchFinder: libc::c_int,
}
pub type BMK_mode_t = libc::c_uint;
pub const BMK_compressOnly: BMK_mode_t = 2;
pub const BMK_decodeOnly: BMK_mode_t = 1;
pub const BMK_both: BMK_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_initDCtxArgs {
    pub dctx: *mut ZSTD_DCtx,
    pub dictBuffer: *const libc::c_void,
    pub dictBufferSize: size_t,
}
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_initCCtxArgs {
    pub cctx: *mut ZSTD_CCtx,
    pub dictBuffer: *const libc::c_void,
    pub dictBufferSize: size_t,
    pub cLevel: libc::c_int,
    pub comprParams: *const ZSTD_compressionParameters,
    pub adv: *const BMK_advancedParams_t,
}
pub type C2RustUnnamed = libc::c_uint;
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
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
pub const BMK_TIMETEST_DEFAULT_S: libc::c_int = 3 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const UTIL_FILESIZE_UNKNOWN: libc::c_int = -(1 as libc::c_int);
pub const PRIO_PROCESS_0: libc::c_int = PRIO_PROCESS as libc::c_int;
pub const ZSTD_CONTENTSIZE_UNKNOWN: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
pub const ZSTD_CONTENTSIZE_ERROR: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(2 as libc::c_int as libc::c_ulonglong);
pub const XXH_FORCE_ALIGN_CHECK: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn XXH_memcpy(
    mut dest: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return libc::memcpy(dest, src, size as libc::size_t);
}
unsafe extern "C" fn XXH_read32(mut memPtr: *const libc::c_void) -> xxh_u32 {
    let mut val: xxh_u32 = 0;
    XXH_memcpy(
        &mut val as *mut xxh_u32 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<xxh_u32>() as libc::c_ulong,
    );
    return val;
}
pub const XXH_CPU_LITTLE_ENDIAN: libc::c_int = 1 as libc::c_int;
pub const XXH_rotl64: unsafe extern "C" fn(
    libc::c_ulong,
    libc::c_ulong,
) -> libc::c_ulong = __builtin_rotateleft64;
unsafe extern "C" fn XXH_swap32(mut x: xxh_u32) -> xxh_u32 {
    return x << 24 as libc::c_int & 0xff000000 as libc::c_uint
        | x << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
        | x >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | x >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn XXH_readLE32(mut ptr: *const libc::c_void) -> xxh_u32 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read32(ptr)
    } else {
        XXH_swap32(XXH_read32(ptr))
    };
}
unsafe extern "C" fn XXH_readLE32_align(
    mut ptr: *const libc::c_void,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return XXH_readLE32(ptr)
    } else {
        return if XXH_CPU_LITTLE_ENDIAN != 0 {
            *(ptr as *const xxh_u32)
        } else {
            XXH_swap32(*(ptr as *const xxh_u32))
        }
    };
}
unsafe extern "C" fn XXH_read64(mut memPtr: *const libc::c_void) -> xxh_u64 {
    let mut val: xxh_u64 = 0;
    XXH_memcpy(
        &mut val as *mut xxh_u64 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<xxh_u64>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn XXH_swap64(mut x: xxh_u64) -> xxh_u64 {
    return ((x << 56 as libc::c_int) as libc::c_ulonglong
        & 0xff00000000000000 as libc::c_ulonglong
        | (x << 40 as libc::c_int) as libc::c_ulonglong
            & 0xff000000000000 as libc::c_ulonglong
        | (x << 24 as libc::c_int) as libc::c_ulonglong
            & 0xff0000000000 as libc::c_ulonglong
        | (x << 8 as libc::c_int) as libc::c_ulonglong
            & 0xff00000000 as libc::c_ulonglong
        | (x >> 8 as libc::c_int) as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong
        | (x >> 24 as libc::c_int) as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong
        | (x >> 40 as libc::c_int) as libc::c_ulonglong & 0xff00 as libc::c_ulonglong
        | (x >> 56 as libc::c_int) as libc::c_ulonglong & 0xff as libc::c_ulonglong)
        as xxh_u64;
}
unsafe extern "C" fn XXH_readLE64(mut ptr: *const libc::c_void) -> xxh_u64 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read64(ptr)
    } else {
        XXH_swap64(XXH_read64(ptr))
    };
}
unsafe extern "C" fn XXH_readLE64_align(
    mut ptr: *const libc::c_void,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return XXH_readLE64(ptr)
    } else {
        return if XXH_CPU_LITTLE_ENDIAN != 0 {
            *(ptr as *const xxh_u64)
        } else {
            XXH_swap64(*(ptr as *const xxh_u64))
        }
    };
}
pub const XXH_PRIME64_1: libc::c_ulonglong = 0x9e3779b185ebca87 as libc::c_ulonglong;
pub const XXH_PRIME64_2: libc::c_ulonglong = 0xc2b2ae3d27d4eb4f as libc::c_ulonglong;
pub const XXH_PRIME64_3: libc::c_ulonglong = 0x165667b19e3779f9 as libc::c_ulonglong;
pub const XXH_PRIME64_4: libc::c_ulonglong = 0x85ebca77c2b2ae63 as libc::c_ulonglong;
pub const XXH_PRIME64_5: libc::c_ulonglong = 0x27d4eb2f165667c5 as libc::c_ulonglong;
unsafe extern "C" fn XXH64_round(mut acc: xxh_u64, mut input: xxh_u64) -> xxh_u64 {
    acc = (acc as libc::c_ulonglong)
        .wrapping_add((input as libc::c_ulonglong).wrapping_mul(XXH_PRIME64_2))
        as xxh_u64 as xxh_u64;
    acc = ::core::intrinsics::rotate_left(acc, 31 as libc::c_int as libc::c_ulong);
    acc = (acc as libc::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64 as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_mergeRound(mut acc: xxh_u64, mut val: xxh_u64) -> xxh_u64 {
    val = XXH64_round(0 as libc::c_int as xxh_u64, val);
    acc ^= val;
    acc = (acc as libc::c_ulonglong)
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_4) as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_avalanche(mut h64: xxh_u64) -> xxh_u64 {
    h64 ^= h64 >> 33 as libc::c_int;
    h64 = (h64 as libc::c_ulonglong).wrapping_mul(XXH_PRIME64_2) as xxh_u64 as xxh_u64;
    h64 ^= h64 >> 29 as libc::c_int;
    h64 = (h64 as libc::c_ulonglong).wrapping_mul(XXH_PRIME64_3) as xxh_u64 as xxh_u64;
    h64 ^= h64 >> 32 as libc::c_int;
    return h64;
}
unsafe extern "C" fn XXH64_finalize(
    mut h64: xxh_u64,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if ptr.is_null() {
        if len == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"len == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/build/cmake/../../programs/../lib/common/xxhash.h\0"
                    as *const u8 as *const libc::c_char,
                2434 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"xxh_u64 XXH64_finalize(xxh_u64, const xxh_u8 *, size_t, XXH_alignment)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    len &= 31 as libc::c_int as libc::c_ulong;
    while len >= 8 as libc::c_int as libc::c_ulong {
        let k1 = XXH64_round(
            0 as libc::c_int as xxh_u64,
            XXH_readLE64_align(ptr as *const libc::c_void, align),
        );
        ptr = ptr.offset(8 as libc::c_int as isize);
        h64 ^= k1;
        h64 = (::core::intrinsics::rotate_left(h64, 27 as libc::c_int as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_4) as xxh_u64;
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    if len >= 4 as libc::c_int as libc::c_ulong {
        h64 = (h64 as libc::c_ulonglong
            ^ (XXH_readLE32_align(ptr as *const libc::c_void, align) as xxh_u64
                as libc::c_ulonglong)
                .wrapping_mul(XXH_PRIME64_1)) as xxh_u64;
        ptr = ptr.offset(4 as libc::c_int as isize);
        h64 = (::core::intrinsics::rotate_left(h64, 23 as libc::c_int as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_2)
            .wrapping_add(XXH_PRIME64_3) as xxh_u64;
        len = (len as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    while len > 0 as libc::c_int as libc::c_ulong {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        h64 = (h64 as libc::c_ulonglong
            ^ (*fresh0 as libc::c_ulonglong).wrapping_mul(XXH_PRIME64_5)) as xxh_u64;
        h64 = (::core::intrinsics::rotate_left(h64, 11 as libc::c_int as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_1) as xxh_u64;
        len = len.wrapping_sub(1);
    }
    return XXH64_avalanche(h64);
}
unsafe extern "C" fn XXH64_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u64,
    mut align: XXH_alignment,
) -> xxh_u64 {
    let mut h64: xxh_u64 = 0;
    if input.is_null() {
        if len == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"len == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/build/cmake/../../programs/../lib/common/xxhash.h\0"
                    as *const u8 as *const libc::c_char,
                2471 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"xxh_u64 XXH64_endian_align(const xxh_u8 *, size_t, xxh_u64, XXH_alignment)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if len >= 32 as libc::c_int as libc::c_ulong {
        let bEnd = input.offset(len as isize);
        let limit = bEnd.offset(-(31 as libc::c_int as isize));
        let mut v1 = (seed as libc::c_ulonglong)
            .wrapping_add(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_2) as xxh_u64;
        let mut v2 = (seed as libc::c_ulonglong).wrapping_add(XXH_PRIME64_2) as xxh_u64;
        let mut v3 = seed.wrapping_add(0 as libc::c_int as libc::c_ulong);
        let mut v4 = (seed as libc::c_ulonglong).wrapping_sub(XXH_PRIME64_1) as xxh_u64;
        loop {
            v1 = XXH64_round(
                v1,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            v2 = XXH64_round(
                v2,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            v3 = XXH64_round(
                v3,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            v4 = XXH64_round(
                v4,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            if !(input < limit) {
                break;
            }
        }
        h64 = (::core::intrinsics::rotate_left(v1, 1 as libc::c_int as libc::c_ulong))
            .wrapping_add(
                ::core::intrinsics::rotate_left(v2, 7 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(v3, 12 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(v4, 18 as libc::c_int as libc::c_ulong),
            );
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4);
    } else {
        h64 = (seed as libc::c_ulonglong).wrapping_add(XXH_PRIME64_5) as xxh_u64;
    }
    h64 = (h64 as libc::c_ulong).wrapping_add(len) as xxh_u64 as xxh_u64;
    return XXH64_finalize(h64, input, len, align);
}
#[inline]
unsafe extern "C" fn XXH_INLINE_XXH64(
    mut input: *const libc::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    return XXH64_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned);
}
pub const MB_UNIT: libc::c_int = 1000000 as libc::c_int;
pub const TIMELOOP_NANOSEC: libc::c_ulonglong = (1 as libc::c_int as libc::c_ulonglong)
    .wrapping_mul(1000000000 as libc::c_ulonglong);
pub const BMK_RUNTEST_DEFAULT_MS: libc::c_int = 1000 as libc::c_int;
static mut maxMemory: size_t = 0;
pub const DEBUG: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn BMK_initAdvancedParams() -> BMK_advancedParams_t {
    let res = {
        let mut init = BMK_advancedParams_t {
            mode: BMK_both,
            nbSeconds: BMK_TIMETEST_DEFAULT_S as libc::c_uint,
            blockSize: 0 as libc::c_int as size_t,
            nbWorkers: 0 as libc::c_int,
            realTime: 0 as libc::c_int as libc::c_uint,
            additionalParam: 0 as libc::c_int,
            ldmFlag: 0 as libc::c_int,
            ldmMinMatch: 0 as libc::c_int,
            ldmHashLog: 0 as libc::c_int,
            ldmBucketSizeLog: 0 as libc::c_int,
            ldmHashRateLog: 0 as libc::c_int,
            literalCompressionMode: ZSTD_ps_auto,
            useRowMatchFinder: 0 as libc::c_int,
        };
        init
    };
    return res;
}
unsafe extern "C" fn BMK_initCCtx(
    mut ctx: *mut ZSTD_CCtx,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: size_t,
    mut cLevel: libc::c_int,
    mut comprParams: *const ZSTD_compressionParameters,
    mut adv: *const BMK_advancedParams_t,
) {
    ZSTD_CCtx_reset(ctx, ZSTD_reset_session_and_parameters);
    if (*adv).nbWorkers == 1 as libc::c_int {
        let zerr = ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, 0 as libc::c_int);
        if ZSTD_isError(zerr) != 0 {
            fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
            fprintf(
                stderr,
                b"%s failed : %s\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, 0)\0" as *const u8
                    as *const libc::c_char,
                ZSTD_getErrorName(zerr),
            );
            fflush(NULL as *mut FILE);
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
            exit(1 as libc::c_int);
        }
    } else {
        let zerr_0 = ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, (*adv).nbWorkers);
        if ZSTD_isError(zerr_0) != 0 {
            fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
            fprintf(
                stderr,
                b"%s failed : %s\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, adv->nbWorkers)\0"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_0),
            );
            fflush(NULL as *mut FILE);
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
            exit(1 as libc::c_int);
        }
    }
    let zerr_1 = ZSTD_CCtx_setParameter(ctx, ZSTD_c_compressionLevel, cLevel);
    if ZSTD_isError(zerr_1) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_compressionLevel, cLevel)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_1),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_2 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_experimentalParam14,
        (*adv).useRowMatchFinder,
    );
    if ZSTD_isError(zerr_2) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_useRowMatchFinder, adv->useRowMatchFinder)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_2),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_3 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_enableLongDistanceMatching,
        (*adv).ldmFlag,
    );
    if ZSTD_isError(zerr_3) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_enableLongDistanceMatching, adv->ldmFlag)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_3),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_4 = ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmMinMatch, (*adv).ldmMinMatch);
    if ZSTD_isError(zerr_4) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmMinMatch, adv->ldmMinMatch)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_4),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_5 = ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashLog, (*adv).ldmHashLog);
    if ZSTD_isError(zerr_5) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashLog, adv->ldmHashLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_5),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_6 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_ldmBucketSizeLog,
        (*adv).ldmBucketSizeLog,
    );
    if ZSTD_isError(zerr_6) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmBucketSizeLog, adv->ldmBucketSizeLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_6),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_7 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_ldmHashRateLog,
        (*adv).ldmHashRateLog,
    );
    if ZSTD_isError(zerr_7) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashRateLog, adv->ldmHashRateLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_7),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_8 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_windowLog,
        (*comprParams).windowLog as libc::c_int,
    );
    if ZSTD_isError(zerr_8) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_windowLog, (int)comprParams->windowLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_8),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_9 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_hashLog,
        (*comprParams).hashLog as libc::c_int,
    );
    if ZSTD_isError(zerr_9) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_hashLog, (int)comprParams->hashLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_9),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_10 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_chainLog,
        (*comprParams).chainLog as libc::c_int,
    );
    if ZSTD_isError(zerr_10) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_chainLog, (int)comprParams->chainLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_10),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_11 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_searchLog,
        (*comprParams).searchLog as libc::c_int,
    );
    if ZSTD_isError(zerr_11) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_searchLog, (int)comprParams->searchLog)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_11),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_12 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_minMatch,
        (*comprParams).minMatch as libc::c_int,
    );
    if ZSTD_isError(zerr_12) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_minMatch, (int)comprParams->minMatch)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_12),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_13 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_targetLength,
        (*comprParams).targetLength as libc::c_int,
    );
    if ZSTD_isError(zerr_13) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_targetLength, (int)comprParams->targetLength)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_13),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_14 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_experimentalParam5,
        (*adv).literalCompressionMode as libc::c_int,
    );
    if ZSTD_isError(zerr_14) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_literalCompressionMode, (int)adv->literalCompressionMode)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_14),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_15 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_c_strategy,
        (*comprParams).strategy as libc::c_int,
    );
    if ZSTD_isError(zerr_15) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_strategy, (int)comprParams->strategy)\0"
                as *const u8 as *const libc::c_char,
            ZSTD_getErrorName(zerr_15),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_16 = ZSTD_CCtx_loadDictionary(ctx, dictBuffer, dictBufferSize);
    if ZSTD_isError(zerr_16) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_CCtx_loadDictionary(ctx, dictBuffer, dictBufferSize)\0" as *const u8
                as *const libc::c_char,
            ZSTD_getErrorName(zerr_16),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn BMK_initDCtx(
    mut dctx: *mut ZSTD_DCtx,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: size_t,
) {
    let zerr = ZSTD_DCtx_reset(dctx, ZSTD_reset_session_and_parameters);
    if ZSTD_isError(zerr) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_DCtx_reset(dctx, ZSTD_reset_session_and_parameters)\0" as *const u8
                as *const libc::c_char,
            ZSTD_getErrorName(zerr),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
    let zerr_0 = ZSTD_DCtx_loadDictionary(dctx, dictBuffer, dictBufferSize);
    if ZSTD_isError(zerr_0) != 0 {
        fprintf(stderr, b"Error : \0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const libc::c_char,
            b"ZSTD_DCtx_loadDictionary(dctx, dictBuffer, dictBufferSize)\0" as *const u8
                as *const libc::c_char,
            ZSTD_getErrorName(zerr_0),
        );
        fflush(NULL as *mut FILE);
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        fflush(NULL as *mut FILE);
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn local_initCCtx(mut payload: *mut libc::c_void) -> size_t {
    let mut ag = payload as *mut BMK_initCCtxArgs;
    BMK_initCCtx(
        (*ag).cctx,
        (*ag).dictBuffer,
        (*ag).dictBufferSize,
        (*ag).cLevel,
        (*ag).comprParams,
        (*ag).adv,
    );
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn local_initDCtx(mut payload: *mut libc::c_void) -> size_t {
    let mut ag = payload as *mut BMK_initDCtxArgs;
    BMK_initDCtx((*ag).dctx, (*ag).dictBuffer, (*ag).dictBufferSize);
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn local_defaultCompress(
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
    mut dstBuffer: *mut libc::c_void,
    mut dstSize: size_t,
    mut addArgs: *mut libc::c_void,
) -> size_t {
    let cctx = addArgs as *mut ZSTD_CCtx;
    return ZSTD_compress2(cctx, dstBuffer, dstSize, srcBuffer, srcSize);
}
unsafe extern "C" fn local_defaultDecompress(
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
    mut dstBuffer: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut addArgs: *mut libc::c_void,
) -> size_t {
    let mut moreToFlush = 1 as libc::c_int as size_t;
    let dctx = addArgs as *mut ZSTD_DCtx;
    let mut in_0 = ZSTD_inBuffer {
        src: 0 as *const libc::c_void,
        size: 0,
        pos: 0,
    };
    let mut out = ZSTD_outBuffer {
        dst: 0 as *mut libc::c_void,
        size: 0,
        pos: 0,
    };
    in_0.src = srcBuffer;
    in_0.size = srcSize;
    in_0.pos = 0 as libc::c_int as size_t;
    out.dst = dstBuffer;
    out.size = dstCapacity;
    out.pos = 0 as libc::c_int as size_t;
    while moreToFlush != 0 {
        if out.pos == out.size {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        moreToFlush = ZSTD_decompressStream(dctx, &mut out, &mut in_0);
        if ZSTD_isError(moreToFlush) != 0 {
            return moreToFlush;
        }
    }
    return out.pos;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_isSuccessful_benchOutcome(
    mut outcome: BMK_benchOutcome_t,
) -> libc::c_int {
    return (outcome.tag == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_extract_benchResult(
    mut outcome: BMK_benchOutcome_t,
) -> BMK_benchResult_t {
    if outcome.tag == 0 as libc::c_int {} else {
        __assert_fail(
            b"outcome.tag == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/benchzstd.c\0" as *const u8
                as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"BMK_benchResult_t BMK_extract_benchResult(BMK_benchOutcome_t)\0"))
                .as_ptr(),
        );
    }
    return outcome.internal_never_use_directly;
}
unsafe extern "C" fn BMK_benchOutcome_error() -> BMK_benchOutcome_t {
    let mut b = BMK_benchOutcome_t {
        internal_never_use_directly: BMK_benchResult_t {
            cSize: 0,
            cSpeed: 0,
            dSpeed: 0,
            cMem: 0,
        },
        tag: 0,
    };
    memset(
        &mut b as *mut BMK_benchOutcome_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
    );
    b.tag = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn BMK_benchOutcome_setValidResult(
    mut result: BMK_benchResult_t,
) -> BMK_benchOutcome_t {
    let mut b = BMK_benchOutcome_t {
        internal_never_use_directly: BMK_benchResult_t {
            cSize: 0,
            cSpeed: 0,
            dSpeed: 0,
            cMem: 0,
        },
        tag: 0,
    };
    b.tag = 0 as libc::c_int;
    b.internal_never_use_directly = result;
    return b;
}
unsafe extern "C" fn BMK_benchMemAdvancedNoAlloc(
    mut srcPtrs: *mut *const libc::c_void,
    mut srcSizes: *mut size_t,
    mut cPtrs: *mut *mut libc::c_void,
    mut cCapacities: *mut size_t,
    mut cSizes: *mut size_t,
    mut resPtrs: *mut *mut libc::c_void,
    mut resSizes: *mut size_t,
    mut resultBufferPtr: *mut *mut libc::c_void,
    mut compressedBuffer: *mut libc::c_void,
    mut maxCompressedSize: size_t,
    mut timeStateCompress: *mut BMK_timedFnState_t,
    mut timeStateDecompress: *mut BMK_timedFnState_t,
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
    mut fileSizes: *const size_t,
    mut nbFiles: libc::c_uint,
    cLevel: libc::c_int,
    mut comprParams: *const ZSTD_compressionParameters,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: size_t,
    mut cctx: *mut ZSTD_CCtx,
    mut dctx: *mut ZSTD_DCtx,
    mut displayLevel: libc::c_int,
    mut displayName: *const libc::c_char,
    mut adv: *const BMK_advancedParams_t,
) -> BMK_benchOutcome_t {
    let blockSize = (if (*adv).blockSize >= 32 as libc::c_int as libc::c_ulong
        && (*adv).mode as libc::c_uint != BMK_decodeOnly as libc::c_int as libc::c_uint
    {
        (*adv).blockSize
    } else {
        srcSize
    })
        .wrapping_add((srcSize == 0) as libc::c_int as libc::c_ulong);
    let mut benchResult = BMK_benchResult_t {
        cSize: 0,
        cSpeed: 0,
        dSpeed: 0,
        cMem: 0,
    };
    let loadedCompressedSize = srcSize;
    let mut cSize = 0 as libc::c_int as size_t;
    let mut ratio = 0.0f64;
    let mut nbBlocks: U32 = 0;
    if !cctx.is_null() {} else {
        __assert_fail(
            b"cctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/benchzstd.c\0" as *const u8
                as *const libc::c_char,
            325 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 385],
                &[libc::c_char; 385],
            >(
                b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !dctx.is_null() {} else {
        __assert_fail(
            b"dctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/benchzstd.c\0" as *const u8
                as *const libc::c_char,
            325 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 385],
                &[libc::c_char; 385],
            >(
                b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\0",
            ))
                .as_ptr(),
        );
    }
    memset(
        &mut benchResult as *mut BMK_benchResult_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<BMK_benchResult_t>() as libc::c_ulong,
    );
    if strlen(displayName) > 17 as libc::c_int as libc::c_ulong {
        displayName = displayName
            .offset(
                (strlen(displayName)).wrapping_sub(17 as libc::c_int as libc::c_ulong)
                    as isize,
            );
    }
    if (*adv).mode as libc::c_uint == BMK_decodeOnly as libc::c_int as libc::c_uint {
        let mut srcPtr = srcBuffer as *const libc::c_char;
        let mut totalDSize64 = 0 as libc::c_int as U64;
        let mut fileNb: U32 = 0;
        fileNb = 0 as libc::c_int as U32;
        while fileNb < nbFiles {
            let fSize64 = ZSTD_findDecompressedSize(
                srcPtr as *const libc::c_void,
                *fileSizes.offset(fileNb as isize),
            ) as U64;
            if fSize64 as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN {
                let mut r = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                memset(
                    &mut r as *mut BMK_benchOutcome_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
                );
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const libc::c_char,
                        32 as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Decompressed size cannot be determined: cannot benchmark\0"
                            as *const u8 as *const libc::c_char,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    fflush(NULL as *mut FILE);
                }
                r.tag = 32 as libc::c_int;
                return r;
            }
            if fSize64 as libc::c_ulonglong == ZSTD_CONTENTSIZE_ERROR {
                let mut r_0 = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                memset(
                    &mut r_0 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
                );
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const libc::c_char,
                        32 as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error while trying to assess decompressed size: data may be invalid\0"
                            as *const u8 as *const libc::c_char,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    fflush(NULL as *mut FILE);
                }
                r_0.tag = 32 as libc::c_int;
                return r_0;
            }
            totalDSize64 = (totalDSize64 as libc::c_ulong).wrapping_add(fSize64) as U64
                as U64;
            srcPtr = srcPtr.offset(*fileSizes.offset(fileNb as isize) as isize);
            fileNb = fileNb.wrapping_add(1);
        }
        let decodedSize = totalDSize64;
        if decodedSize == totalDSize64 {} else {
            __assert_fail(
                b"(U64)decodedSize == totalDSize64\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/benchzstd.c\0" as *const u8
                    as *const libc::c_char,
                347 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 385],
                    &[libc::c_char; 385],
                >(
                    b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        free(*resultBufferPtr);
        if totalDSize64 > decodedSize {
            let mut r_1 = BMK_benchOutcome_t {
                internal_never_use_directly: BMK_benchResult_t {
                    cSize: 0,
                    cSpeed: 0,
                    dSpeed: 0,
                    cMem: 0,
                },
                tag: 0,
            };
            memset(
                &mut r_1 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
            );
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const libc::c_char,
                    32 as libc::c_int,
                );
                fflush(NULL as *mut FILE);
            }
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"decompressed size is too large for local system\0" as *const u8
                        as *const libc::c_char,
                );
                fflush(NULL as *mut FILE);
            }
            if displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                fflush(NULL as *mut FILE);
            }
            r_1.tag = 32 as libc::c_int;
            return r_1;
        }
        *resultBufferPtr = malloc(decodedSize);
        if (*resultBufferPtr).is_null() {
            let mut r_2 = BMK_benchOutcome_t {
                internal_never_use_directly: BMK_benchResult_t {
                    cSize: 0,
                    cSpeed: 0,
                    dSpeed: 0,
                    cMem: 0,
                },
                tag: 0,
            };
            memset(
                &mut r_2 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
            );
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const libc::c_char,
                    33 as libc::c_int,
                );
                fflush(NULL as *mut FILE);
            }
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"allocation error: not enough memory\0" as *const u8
                        as *const libc::c_char,
                );
                fflush(NULL as *mut FILE);
            }
            if displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                fflush(NULL as *mut FILE);
            }
            r_2.tag = 33 as libc::c_int;
            return r_2;
        }
        cSize = srcSize;
        srcSize = decodedSize;
        ratio = srcSize as libc::c_double / cSize as libc::c_double;
    }
    let mut srcPtr_0 = srcBuffer as *const libc::c_char;
    let mut cPtr = compressedBuffer as *mut libc::c_char;
    let mut resPtr = *resultBufferPtr as *mut libc::c_char;
    let mut fileNb_0: U32 = 0;
    nbBlocks = 0 as libc::c_int as U32;
    fileNb_0 = 0 as libc::c_int as U32;
    while fileNb_0 < nbFiles {
        let mut remaining = *fileSizes.offset(fileNb_0 as isize);
        let nbBlocksforThisFile = if (*adv).mode as libc::c_uint
            == BMK_decodeOnly as libc::c_int as libc::c_uint
        {
            1 as libc::c_int as libc::c_uint
        } else {
            remaining
                .wrapping_add(blockSize.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_div(blockSize) as U32
        };
        let blockEnd = nbBlocks.wrapping_add(nbBlocksforThisFile);
        while nbBlocks < blockEnd {
            let thisBlockSize = if remaining < blockSize {
                remaining
            } else {
                blockSize
            };
            let ref mut fresh1 = *srcPtrs.offset(nbBlocks as isize);
            *fresh1 = srcPtr_0 as *const libc::c_void;
            *srcSizes.offset(nbBlocks as isize) = thisBlockSize;
            let ref mut fresh2 = *cPtrs.offset(nbBlocks as isize);
            *fresh2 = cPtr as *mut libc::c_void;
            *cCapacities
                .offset(
                    nbBlocks as isize,
                ) = if (*adv).mode as libc::c_uint
                == BMK_decodeOnly as libc::c_int as libc::c_uint
            {
                thisBlockSize
            } else {
                ZSTD_compressBound(thisBlockSize)
            };
            let ref mut fresh3 = *resPtrs.offset(nbBlocks as isize);
            *fresh3 = resPtr as *mut libc::c_void;
            *resSizes
                .offset(
                    nbBlocks as isize,
                ) = if (*adv).mode as libc::c_uint
                == BMK_decodeOnly as libc::c_int as libc::c_uint
            {
                ZSTD_findDecompressedSize(srcPtr_0 as *const libc::c_void, thisBlockSize)
                    as size_t
            } else {
                thisBlockSize
            };
            srcPtr_0 = srcPtr_0.offset(thisBlockSize as isize);
            cPtr = cPtr.offset(*cCapacities.offset(nbBlocks as isize) as isize);
            resPtr = resPtr.offset(thisBlockSize as isize);
            remaining = (remaining as libc::c_ulong).wrapping_sub(thisBlockSize)
                as size_t as size_t;
            if (*adv).mode as libc::c_uint
                == BMK_decodeOnly as libc::c_int as libc::c_uint
            {
                *cSizes.offset(nbBlocks as isize) = thisBlockSize;
                benchResult.cSize = thisBlockSize;
            }
            nbBlocks = nbBlocks.wrapping_add(1);
        }
        fileNb_0 = fileNb_0.wrapping_add(1);
    }
    if (*adv).mode as libc::c_uint == BMK_decodeOnly as libc::c_int as libc::c_uint {
        memcpy(compressedBuffer, srcBuffer, loadedCompressedSize);
    } else {
        RDG_genBuffer(
            compressedBuffer,
            maxCompressedSize,
            0.10f64,
            0.50f64,
            1 as libc::c_int as libc::c_uint,
        );
    }
    if UTIL_support_MT_measurements() == 0 && (*adv).nbWorkers > 1 as libc::c_int {
        if displayLevel >= 2 as libc::c_int {
            fprintf(
                stdout,
                b"Warning : time measurements may be incorrect in multithreading mode... \n\0"
                    as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
    }
    let crcOrig = if (*adv).mode as libc::c_uint
        == BMK_decodeOnly as libc::c_int as libc::c_uint
    {
        0 as libc::c_int as libc::c_ulong
    } else {
        XXH_INLINE_XXH64(srcBuffer, srcSize, 0 as libc::c_int as XXH64_hash_t)
    };
    let mut marks: [*const libc::c_char; 4] = [
        b" |\0" as *const u8 as *const libc::c_char,
        b" /\0" as *const u8 as *const libc::c_char,
        b" =\0" as *const u8 as *const libc::c_char,
        b" \\\0" as *const u8 as *const libc::c_char,
    ];
    let mut markNb = 0 as libc::c_int as U32;
    let mut compressionCompleted = ((*adv).mode as libc::c_uint
        == BMK_decodeOnly as libc::c_int as libc::c_uint) as libc::c_int;
    let mut decompressionCompleted = ((*adv).mode as libc::c_uint
        == BMK_compressOnly as libc::c_int as libc::c_uint) as libc::c_int;
    let mut cbp = BMK_benchParams_t {
        benchFn: None,
        benchPayload: 0 as *mut libc::c_void,
        initFn: None,
        initPayload: 0 as *mut libc::c_void,
        errorFn: None,
        blockCount: 0,
        srcBuffers: 0 as *const *const libc::c_void,
        srcSizes: 0 as *const size_t,
        dstBuffers: 0 as *const *mut libc::c_void,
        dstCapacities: 0 as *const size_t,
        blockResults: 0 as *mut size_t,
    };
    let mut dbp = BMK_benchParams_t {
        benchFn: None,
        benchPayload: 0 as *mut libc::c_void,
        initFn: None,
        initPayload: 0 as *mut libc::c_void,
        errorFn: None,
        blockCount: 0,
        srcBuffers: 0 as *const *const libc::c_void,
        srcSizes: 0 as *const size_t,
        dstBuffers: 0 as *const *mut libc::c_void,
        dstCapacities: 0 as *const size_t,
        blockResults: 0 as *mut size_t,
    };
    let mut cctxprep = BMK_initCCtxArgs {
        cctx: 0 as *mut ZSTD_CCtx,
        dictBuffer: 0 as *const libc::c_void,
        dictBufferSize: 0,
        cLevel: 0,
        comprParams: 0 as *const ZSTD_compressionParameters,
        adv: 0 as *const BMK_advancedParams_t,
    };
    let mut dctxprep = BMK_initDCtxArgs {
        dctx: 0 as *mut ZSTD_DCtx,
        dictBuffer: 0 as *const libc::c_void,
        dictBufferSize: 0,
    };
    cbp
        .benchFn = Some(
        local_defaultCompress
            as unsafe extern "C" fn(
                *const libc::c_void,
                size_t,
                *mut libc::c_void,
                size_t,
                *mut libc::c_void,
            ) -> size_t,
    );
    cbp.benchPayload = cctx as *mut libc::c_void;
    cbp
        .initFn = Some(
        local_initCCtx as unsafe extern "C" fn(*mut libc::c_void) -> size_t,
    );
    cbp.initPayload = &mut cctxprep as *mut BMK_initCCtxArgs as *mut libc::c_void;
    cbp.errorFn = Some(ZSTD_isError as unsafe extern "C" fn(size_t) -> libc::c_uint);
    cbp.blockCount = nbBlocks as size_t;
    cbp.srcBuffers = srcPtrs;
    cbp.srcSizes = srcSizes;
    cbp.dstBuffers = cPtrs;
    cbp.dstCapacities = cCapacities;
    cbp.blockResults = cSizes;
    cctxprep.cctx = cctx;
    cctxprep.dictBuffer = dictBuffer;
    cctxprep.dictBufferSize = dictBufferSize;
    cctxprep.cLevel = cLevel;
    cctxprep.comprParams = comprParams;
    cctxprep.adv = adv;
    dbp
        .benchFn = Some(
        local_defaultDecompress
            as unsafe extern "C" fn(
                *const libc::c_void,
                size_t,
                *mut libc::c_void,
                size_t,
                *mut libc::c_void,
            ) -> size_t,
    );
    dbp.benchPayload = dctx as *mut libc::c_void;
    dbp
        .initFn = Some(
        local_initDCtx as unsafe extern "C" fn(*mut libc::c_void) -> size_t,
    );
    dbp.initPayload = &mut dctxprep as *mut BMK_initDCtxArgs as *mut libc::c_void;
    dbp.errorFn = Some(ZSTD_isError as unsafe extern "C" fn(size_t) -> libc::c_uint);
    dbp.blockCount = nbBlocks as size_t;
    dbp.srcBuffers = cPtrs as *const *const libc::c_void;
    dbp.srcSizes = cSizes;
    dbp.dstBuffers = resPtrs;
    dbp.dstCapacities = resSizes;
    dbp.blockResults = NULL as *mut size_t;
    dctxprep.dctx = dctx;
    dctxprep.dictBuffer = dictBuffer;
    dctxprep.dictBufferSize = dictBufferSize;
    if displayLevel >= 2 as libc::c_int {
        fprintf(
            stdout,
            b"\r%70s\r\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(NULL as *mut FILE);
    }
    if srcSize
        < (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
    {} else {
        __assert_fail(
            b"srcSize < UINT_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/benchzstd.c\0" as *const u8
                as *const libc::c_char,
            446 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 385],
                &[libc::c_char; 385],
            >(
                b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if displayLevel >= 2 as libc::c_int {
        fprintf(
            stdout,
            b"%2s-%-17.17s :%10u -> \r\0" as *const u8 as *const libc::c_char,
            marks[markNb as usize],
            displayName,
            srcSize as libc::c_uint,
        );
        fflush(NULL as *mut FILE);
    }
    while !(compressionCompleted != 0 && decompressionCompleted != 0) {
        if compressionCompleted == 0 {
            let cOutcome = BMK_benchTimedFn(timeStateCompress, cbp);
            if BMK_isSuccessful_runOutcome(cOutcome) == 0 {
                let mut r_3 = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                memset(
                    &mut r_3 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
                );
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const libc::c_char,
                        30 as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"compression error\0" as *const u8 as *const libc::c_char,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    fflush(NULL as *mut FILE);
                }
                r_3.tag = 30 as libc::c_int;
                return r_3;
            }
            let cResult = BMK_extract_runTime(cOutcome);
            cSize = cResult.sumOfReturn;
            ratio = srcSize as libc::c_double / cSize as libc::c_double;
            let mut newResult = BMK_benchResult_t {
                cSize: 0,
                cSpeed: 0,
                dSpeed: 0,
                cMem: 0,
            };
            newResult
                .cSpeed = (srcSize as libc::c_double * TIMELOOP_NANOSEC as libc::c_double
                / cResult.nanoSecPerRun) as U64 as libc::c_ulonglong;
            benchResult.cSize = cSize;
            if newResult.cSpeed > benchResult.cSpeed {
                benchResult.cSpeed = newResult.cSpeed;
            }
            let ratioAccuracy = if ratio < 10.0f64 {
                3 as libc::c_int
            } else {
                2 as libc::c_int
            };
            if cSize
                < (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as libc::c_ulong
            {} else {
                __assert_fail(
                    b"cSize < UINT_MAX\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/benchzstd.c\0" as *const u8
                        as *const libc::c_char,
                    468 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 385],
                        &[libc::c_char; 385],
                    >(
                        b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if displayLevel >= 2 as libc::c_int {
                fprintf(
                    stdout,
                    b"%2s-%-17.17s :%10u ->%10u (x%5.*f), %6.*f MB/s \r\0" as *const u8
                        as *const libc::c_char,
                    marks[markNb as usize],
                    displayName,
                    srcSize as libc::c_uint,
                    cSize as libc::c_uint,
                    ratioAccuracy,
                    ratio,
                    if benchResult.cSpeed
                        < (10 as libc::c_int * 1000000 as libc::c_int)
                            as libc::c_ulonglong
                    {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    },
                    benchResult.cSpeed as libc::c_double
                        / 1000000 as libc::c_int as libc::c_double,
                );
                fflush(NULL as *mut FILE);
            }
            compressionCompleted = BMK_isCompleted_TimedFn(timeStateCompress);
        }
        if decompressionCompleted == 0 {
            let dOutcome = BMK_benchTimedFn(timeStateDecompress, dbp);
            if BMK_isSuccessful_runOutcome(dOutcome) == 0 {
                let mut r_4 = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                memset(
                    &mut r_4 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
                );
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const libc::c_char,
                        30 as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"decompression error\0" as *const u8 as *const libc::c_char,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    fflush(NULL as *mut FILE);
                }
                r_4.tag = 30 as libc::c_int;
                return r_4;
            }
            let dResult = BMK_extract_runTime(dOutcome);
            let newDSpeed = (srcSize as libc::c_double
                * TIMELOOP_NANOSEC as libc::c_double / dResult.nanoSecPerRun) as U64;
            if newDSpeed as libc::c_ulonglong > benchResult.dSpeed {
                benchResult.dSpeed = newDSpeed as libc::c_ulonglong;
            }
            let ratioAccuracy_0 = if ratio < 10.0f64 {
                3 as libc::c_int
            } else {
                2 as libc::c_int
            };
            if displayLevel >= 2 as libc::c_int {
                fprintf(
                    stdout,
                    b"%2s-%-17.17s :%10u ->%10u (x%5.*f), %6.*f MB/s, %6.1f MB/s\r\0"
                        as *const u8 as *const libc::c_char,
                    marks[markNb as usize],
                    displayName,
                    srcSize as libc::c_uint,
                    cSize as libc::c_uint,
                    ratioAccuracy_0,
                    ratio,
                    if benchResult.cSpeed
                        < (10 as libc::c_int * 1000000 as libc::c_int)
                            as libc::c_ulonglong
                    {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    },
                    benchResult.cSpeed as libc::c_double
                        / 1000000 as libc::c_int as libc::c_double,
                    benchResult.dSpeed as libc::c_double
                        / 1000000 as libc::c_int as libc::c_double,
                );
                fflush(NULL as *mut FILE);
            }
            decompressionCompleted = BMK_isCompleted_TimedFn(timeStateDecompress);
        }
        markNb = markNb
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(NB_MARKS as libc::c_uint);
    }
    let mut resultBuffer = *resultBufferPtr as *const BYTE;
    let crcCheck = XXH_INLINE_XXH64(
        resultBuffer as *const libc::c_void,
        srcSize,
        0 as libc::c_int as XXH64_hash_t,
    );
    if (*adv).mode as libc::c_uint == BMK_both as libc::c_int as libc::c_uint
        && crcOrig != crcCheck
    {
        let mut u: size_t = 0;
        fprintf(
            stderr,
            b"!!! WARNING !!! %14s : Invalid Checksum : %x != %x   \n\0" as *const u8
                as *const libc::c_char,
            displayName,
            crcOrig as libc::c_uint,
            crcCheck as libc::c_uint,
        );
        fflush(NULL as *mut FILE);
        u = 0 as libc::c_int as size_t;
        while u < srcSize {
            if *(srcBuffer as *const BYTE).offset(u as isize) as libc::c_int
                != *resultBuffer.offset(u as isize) as libc::c_int
            {
                let mut segNb: libc::c_uint = 0;
                let mut bNb: libc::c_uint = 0;
                let mut pos: libc::c_uint = 0;
                let mut bacc = 0 as libc::c_int as size_t;
                fprintf(
                    stderr,
                    b"Decoding error at pos %u \0" as *const u8 as *const libc::c_char,
                    u as libc::c_uint,
                );
                fflush(NULL as *mut FILE);
                segNb = 0 as libc::c_int as libc::c_uint;
                while segNb < nbBlocks {
                    if bacc.wrapping_add(*srcSizes.offset(segNb as isize)) > u {
                        break;
                    }
                    bacc = (bacc as libc::c_ulong)
                        .wrapping_add(*srcSizes.offset(segNb as isize)) as size_t
                        as size_t;
                    segNb = segNb.wrapping_add(1);
                }
                pos = u.wrapping_sub(bacc) as U32;
                bNb = pos
                    .wrapping_div(
                        (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                            as libc::c_uint,
                    );
                fprintf(
                    stderr,
                    b"(sample %u, block %u, pos %u) \n\0" as *const u8
                        as *const libc::c_char,
                    segNb,
                    bNb,
                    pos,
                );
                fflush(NULL as *mut FILE);
                let lowest = if u > 5 as libc::c_int as libc::c_ulong {
                    5 as libc::c_int as libc::c_ulong
                } else {
                    u
                };
                let mut n: size_t = 0;
                fprintf(stderr, b"origin: \0" as *const u8 as *const libc::c_char);
                fflush(NULL as *mut FILE);
                n = lowest;
                while n > 0 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const libc::c_char,
                        *(srcBuffer as *const BYTE).offset(u.wrapping_sub(n) as isize)
                            as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                    n = n.wrapping_sub(1);
                }
                fprintf(
                    stderr,
                    b" :%02X:  \0" as *const u8 as *const libc::c_char,
                    *(srcBuffer as *const BYTE).offset(u as isize) as libc::c_int,
                );
                fflush(NULL as *mut FILE);
                n = 1 as libc::c_int as size_t;
                while n < 3 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const libc::c_char,
                        *(srcBuffer as *const BYTE).offset(u.wrapping_add(n) as isize)
                            as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                    n = n.wrapping_add(1);
                }
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                fflush(NULL as *mut FILE);
                fprintf(stderr, b"decode: \0" as *const u8 as *const libc::c_char);
                fflush(NULL as *mut FILE);
                n = lowest;
                while n > 0 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const libc::c_char,
                        *resultBuffer.offset(u.wrapping_sub(n) as isize) as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                    n = n.wrapping_sub(1);
                }
                fprintf(
                    stderr,
                    b" :%02X:  \0" as *const u8 as *const libc::c_char,
                    *resultBuffer.offset(u as isize) as libc::c_int,
                );
                fflush(NULL as *mut FILE);
                n = 1 as libc::c_int as size_t;
                while n < 3 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const libc::c_char,
                        *resultBuffer.offset(u.wrapping_add(n) as isize) as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                    n = n.wrapping_add(1);
                }
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                fflush(NULL as *mut FILE);
                break;
            } else {
                if u == srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    fprintf(
                        stderr,
                        b"no difference detected\n\0" as *const u8 as *const libc::c_char,
                    );
                    fflush(NULL as *mut FILE);
                }
                u = u.wrapping_add(1);
            }
        }
    }
    if displayLevel == 1 as libc::c_int {
        let cSpeed = benchResult.cSpeed as libc::c_double / MB_UNIT as libc::c_double;
        let dSpeed = benchResult.dSpeed as libc::c_double / MB_UNIT as libc::c_double;
        if (*adv).additionalParam != 0 {
            fprintf(
                stdout,
                b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s (param=%d)\n\0"
                    as *const u8 as *const libc::c_char,
                cLevel,
                cSize as libc::c_int,
                ratio,
                cSpeed,
                dSpeed,
                displayName,
                (*adv).additionalParam,
            );
            fflush(NULL as *mut FILE);
        } else {
            fprintf(
                stdout,
                b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s\n\0" as *const u8
                    as *const libc::c_char,
                cLevel,
                cSize as libc::c_int,
                ratio,
                cSpeed,
                dSpeed,
                displayName,
            );
            fflush(NULL as *mut FILE);
        }
    }
    if displayLevel >= 2 as libc::c_int {
        fprintf(stdout, b"%2i#\n\0" as *const u8 as *const libc::c_char, cLevel);
        fflush(NULL as *mut FILE);
    }
    benchResult
        .cMem = ((1 as libc::c_ulonglong) << (*comprParams).windowLog)
        .wrapping_add(ZSTD_sizeof_CCtx(cctx) as libc::c_ulonglong) as size_t;
    return BMK_benchOutcome_setValidResult(benchResult);
}
pub const NB_MARKS: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMemAdvanced(
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
    mut dstBuffer: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut fileSizes: *const size_t,
    mut nbFiles: libc::c_uint,
    mut cLevel: libc::c_int,
    mut comprParams: *const ZSTD_compressionParameters,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: size_t,
    mut displayLevel: libc::c_int,
    mut displayName: *const libc::c_char,
    mut adv: *const BMK_advancedParams_t,
) -> BMK_benchOutcome_t {
    let dstParamsError = dstBuffer.is_null() as libc::c_int
        ^ (dstCapacity == 0) as libc::c_int;
    let blockSize = (if (*adv).blockSize >= 32 as libc::c_int as libc::c_ulong
        && (*adv).mode as libc::c_uint != BMK_decodeOnly as libc::c_int as libc::c_uint
    {
        (*adv).blockSize
    } else {
        srcSize
    })
        .wrapping_add((srcSize == 0) as libc::c_int as libc::c_ulong);
    let maxNbBlocks = (srcSize
        .wrapping_add(blockSize.wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(blockSize) as U32)
        .wrapping_add(nbFiles);
    let srcPtrs = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *const libc::c_void;
    let srcSizes = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let cPtrs = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    let cSizes = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let cCapacities = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let resPtrs = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    let resSizes = malloc(
        (maxNbBlocks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let mut timeStateCompress = BMK_createTimedFnState(
        ((*adv).nbSeconds).wrapping_mul(1000 as libc::c_int as libc::c_uint),
        BMK_RUNTEST_DEFAULT_MS as libc::c_uint,
    );
    let mut timeStateDecompress = BMK_createTimedFnState(
        ((*adv).nbSeconds).wrapping_mul(1000 as libc::c_int as libc::c_uint),
        BMK_RUNTEST_DEFAULT_MS as libc::c_uint,
    );
    let cctx = ZSTD_createCCtx();
    let dctx = ZSTD_createDCtx();
    let maxCompressedSize = if dstCapacity != 0 {
        dstCapacity
    } else {
        (ZSTD_compressBound(srcSize))
            .wrapping_add(
                maxNbBlocks.wrapping_mul(1024 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            )
    };
    let internalDstBuffer = if !dstBuffer.is_null() {
        NULL as *mut libc::c_void
    } else {
        malloc(maxCompressedSize)
    };
    let compressedBuffer = if !dstBuffer.is_null() {
        dstBuffer
    } else {
        internalDstBuffer
    };
    let mut outcome = BMK_benchOutcome_error();
    let mut resultBuffer = if srcSize != 0 {
        malloc(srcSize)
    } else {
        NULL as *mut libc::c_void
    };
    let allocationincomplete = (srcPtrs.is_null() || srcSizes.is_null()
        || cPtrs.is_null() || cSizes.is_null() || cCapacities.is_null()
        || resPtrs.is_null() || resSizes.is_null() || timeStateCompress.is_null()
        || timeStateDecompress.is_null() || cctx.is_null() || dctx.is_null()
        || compressedBuffer.is_null() || resultBuffer.is_null()) as libc::c_int;
    if allocationincomplete == 0 && dstParamsError == 0 {
        outcome = BMK_benchMemAdvancedNoAlloc(
            srcPtrs,
            srcSizes,
            cPtrs,
            cCapacities,
            cSizes,
            resPtrs,
            resSizes,
            &mut resultBuffer,
            compressedBuffer,
            maxCompressedSize,
            timeStateCompress,
            timeStateDecompress,
            srcBuffer,
            srcSize,
            fileSizes,
            nbFiles,
            cLevel,
            comprParams,
            dictBuffer,
            dictBufferSize,
            cctx,
            dctx,
            displayLevel,
            displayName,
            adv,
        );
    }
    BMK_freeTimedFnState(timeStateCompress);
    BMK_freeTimedFnState(timeStateDecompress);
    ZSTD_freeCCtx(cctx);
    ZSTD_freeDCtx(dctx);
    free(internalDstBuffer);
    free(resultBuffer);
    free(srcPtrs as *mut libc::c_void);
    free(srcSizes as *mut libc::c_void);
    free(cPtrs as *mut libc::c_void);
    free(cSizes as *mut libc::c_void);
    free(cCapacities as *mut libc::c_void);
    free(resPtrs as *mut libc::c_void);
    free(resSizes as *mut libc::c_void);
    if allocationincomplete != 0 {
        let mut r = BMK_benchOutcome_t {
            internal_never_use_directly: BMK_benchResult_t {
                cSize: 0,
                cSpeed: 0,
                dSpeed: 0,
                cMem: 0,
            },
            tag: 0,
        };
        memset(
            &mut r as *mut BMK_benchOutcome_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
        );
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error %i : \0" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
            );
            fflush(NULL as *mut FILE);
        }
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"allocation error : not enough memory\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        if displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
        }
        r.tag = 31 as libc::c_int;
        return r;
    }
    if dstParamsError != 0 {
        let mut r_0 = BMK_benchOutcome_t {
            internal_never_use_directly: BMK_benchResult_t {
                cSize: 0,
                cSpeed: 0,
                dSpeed: 0,
                cMem: 0,
            },
            tag: 0,
        };
        memset(
            &mut r_0 as *mut BMK_benchOutcome_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong,
        );
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error %i : \0" as *const u8 as *const libc::c_char,
                32 as libc::c_int,
            );
            fflush(NULL as *mut FILE);
        }
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Dst parameters not coherent\0" as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        if displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
        }
        r_0.tag = 32 as libc::c_int;
        return r_0;
    }
    return outcome;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMem(
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
    mut fileSizes: *const size_t,
    mut nbFiles: libc::c_uint,
    mut cLevel: libc::c_int,
    mut comprParams: *const ZSTD_compressionParameters,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: size_t,
    mut displayLevel: libc::c_int,
    mut displayName: *const libc::c_char,
) -> BMK_benchOutcome_t {
    let adv = BMK_initAdvancedParams();
    return BMK_benchMemAdvanced(
        srcBuffer,
        srcSize,
        NULL as *mut libc::c_void,
        0 as libc::c_int as size_t,
        fileSizes,
        nbFiles,
        cLevel,
        comprParams,
        dictBuffer,
        dictBufferSize,
        displayLevel,
        displayName,
        &adv,
    );
}
unsafe extern "C" fn BMK_benchCLevel(
    mut srcBuffer: *const libc::c_void,
    mut benchedSize: size_t,
    mut fileSizes: *const size_t,
    mut nbFiles: libc::c_uint,
    mut cLevel: libc::c_int,
    mut comprParams: *const ZSTD_compressionParameters,
    mut dictBuffer: *const libc::c_void,
    mut dictBufferSize: size_t,
    mut displayLevel: libc::c_int,
    mut displayName: *const libc::c_char,
    adv: *const BMK_advancedParams_t,
) -> BMK_benchOutcome_t {
    let mut pch: *const libc::c_char = strrchr(displayName, '\\' as i32);
    if pch.is_null() {
        pch = strrchr(displayName, '/' as i32);
    }
    if !pch.is_null() {
        displayName = pch.offset(1 as libc::c_int as isize);
    }
    if (*adv).realTime != 0 {
        if displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Note : switching to real-time priority \n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        setpriority(PRIO_PROCESS_0, 0 as libc::c_int as id_t, -(20 as libc::c_int));
    }
    if displayLevel == 1 as libc::c_int && (*adv).additionalParam == 0 {
        fprintf(
            stdout,
            b"bench %s %s: input %u bytes, %u seconds, %u KB blocks\n\0" as *const u8
                as *const libc::c_char,
            b"1.5.5\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            benchedSize as libc::c_uint,
            (*adv).nbSeconds,
            ((*adv).blockSize >> 10 as libc::c_int) as libc::c_uint,
        );
        fflush(NULL as *mut FILE);
    }
    return BMK_benchMemAdvanced(
        srcBuffer,
        benchedSize,
        NULL as *mut libc::c_void,
        0 as libc::c_int as size_t,
        fileSizes,
        nbFiles,
        cLevel,
        comprParams,
        dictBuffer,
        dictBufferSize,
        displayLevel,
        displayName,
        adv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BMK_syntheticTest(
    mut cLevel: libc::c_int,
    mut compressibility: libc::c_double,
    mut compressionParams: *const ZSTD_compressionParameters,
    mut displayLevel: libc::c_int,
    mut adv: *const BMK_advancedParams_t,
) -> libc::c_int {
    let mut name: [libc::c_char; 20] = [
        0 as libc::c_int as libc::c_char,
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
    let benchedSize = 10000000 as libc::c_int as size_t;
    let mut srcBuffer = 0 as *mut libc::c_void;
    let mut res = BMK_benchOutcome_t {
        internal_never_use_directly: BMK_benchResult_t {
            cSize: 0,
            cSpeed: 0,
            dSpeed: 0,
            cMem: 0,
        },
        tag: 0,
    };
    if cLevel > ZSTD_maxCLevel() {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Invalid Compression Level\0" as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        return 15 as libc::c_int;
    }
    srcBuffer = malloc(benchedSize);
    if srcBuffer.is_null() {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"allocation error : not enough memory\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        return 16 as libc::c_int;
    }
    RDG_genBuffer(
        srcBuffer,
        benchedSize,
        compressibility,
        0.0f64,
        0 as libc::c_int as libc::c_uint,
    );
    snprintf(
        name.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        b"Synthetic %2u%%\0" as *const u8 as *const libc::c_char,
        (compressibility * 100 as libc::c_int as libc::c_double) as libc::c_uint,
    );
    res = BMK_benchCLevel(
        srcBuffer,
        benchedSize,
        &benchedSize,
        1 as libc::c_int as libc::c_uint,
        cLevel,
        compressionParams,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
        displayLevel,
        name.as_mut_ptr(),
        adv,
    );
    free(srcBuffer);
    return (BMK_isSuccessful_benchOutcome(res) == 0) as libc::c_int;
}
unsafe extern "C" fn BMK_findMaxMem(mut requiredMem: U64) -> size_t {
    let step = (64 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as size_t;
    let mut testmem = NULL as *mut BYTE;
    requiredMem = (requiredMem >> 26 as libc::c_int)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) << 26 as libc::c_int;
    requiredMem = (requiredMem as libc::c_ulong).wrapping_add(step) as U64 as U64;
    if requiredMem > maxMemory {
        requiredMem = maxMemory;
    }
    loop {
        testmem = malloc(requiredMem) as *mut BYTE;
        requiredMem = (requiredMem as libc::c_ulong).wrapping_sub(step) as U64 as U64;
        if !(testmem.is_null() && requiredMem > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    free(testmem as *mut libc::c_void);
    return requiredMem;
}
unsafe extern "C" fn BMK_loadFiles(
    mut buffer: *mut libc::c_void,
    mut bufferSize: size_t,
    mut fileSizes: *mut size_t,
    mut fileNamesTable: *const *const libc::c_char,
    mut nbFiles: libc::c_uint,
    mut displayLevel: libc::c_int,
) -> libc::c_int {
    let mut pos = 0 as libc::c_int as size_t;
    let mut totalSize = 0 as libc::c_int as size_t;
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n < nbFiles {
        let mut fileSize = UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        if UTIL_isDirectory(*fileNamesTable.offset(n as isize)) != 0 {
            if displayLevel >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"Ignoring %s directory...       \n\0" as *const u8
                        as *const libc::c_char,
                    *fileNamesTable.offset(n as isize),
                );
                fflush(NULL as *mut FILE);
            }
            *fileSizes.offset(n as isize) = 0 as libc::c_int as size_t;
        } else if fileSize == UTIL_FILESIZE_UNKNOWN as U64 {
            if displayLevel >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"Cannot evaluate size of %s, ignoring ... \n\0" as *const u8
                        as *const libc::c_char,
                    *fileNamesTable.offset(n as isize),
                );
                fflush(NULL as *mut FILE);
            }
            *fileSizes.offset(n as isize) = 0 as libc::c_int as size_t;
        } else {
            let f = fopen(
                *fileNamesTable.offset(n as isize),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if f.is_null() {
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"impossible to open file %s\0" as *const u8
                            as *const libc::c_char,
                        *fileNamesTable.offset(n as isize),
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    fflush(NULL as *mut FILE);
                }
                return 10 as libc::c_int;
            }
            if displayLevel >= 2 as libc::c_int {
                fprintf(
                    stdout,
                    b"Loading %s...       \r\0" as *const u8 as *const libc::c_char,
                    *fileNamesTable.offset(n as isize),
                );
                fflush(NULL as *mut FILE);
            }
            if fileSize > bufferSize.wrapping_sub(pos) {
                fileSize = bufferSize.wrapping_sub(pos);
                nbFiles = n;
            }
            let readSize = fread(
                (buffer as *mut libc::c_char).offset(pos as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                fileSize,
                f,
            );
            if readSize != fileSize {
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int,
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"could not read %s\0" as *const u8 as *const libc::c_char,
                        *fileNamesTable.offset(n as isize),
                    );
                    fflush(NULL as *mut FILE);
                }
                if displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    fflush(NULL as *mut FILE);
                }
                return 11 as libc::c_int;
            }
            pos = (pos as libc::c_ulong).wrapping_add(readSize) as size_t as size_t;
            *fileSizes.offset(n as isize) = fileSize;
            totalSize = (totalSize as libc::c_ulong).wrapping_add(fileSize) as size_t
                as size_t;
            fclose(f);
        }
        n = n.wrapping_add(1);
    }
    if totalSize == 0 as libc::c_int as libc::c_ulong {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error %i : \0" as *const u8 as *const libc::c_char,
                12 as libc::c_int,
            );
            fflush(NULL as *mut FILE);
        }
        if displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"no data to bench\0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
        }
        if displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            fflush(NULL as *mut FILE);
        }
        return 12 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFilesAdvanced(
    mut fileNamesTable: *const *const libc::c_char,
    mut nbFiles: libc::c_uint,
    mut dictFileName: *const libc::c_char,
    mut cLevel: libc::c_int,
    mut compressionParams: *const ZSTD_compressionParameters,
    mut displayLevel: libc::c_int,
    mut adv: *const BMK_advancedParams_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut srcBuffer = NULL as *mut libc::c_void;
    let mut benchedSize: size_t = 0;
    let mut dictBuffer = NULL as *mut libc::c_void;
    let mut dictBufferSize = 0 as libc::c_int as size_t;
    let mut fileSizes = NULL as *mut size_t;
    let mut res = BMK_benchOutcome_t {
        internal_never_use_directly: BMK_benchResult_t {
            cSize: 0,
            cSpeed: 0,
            dSpeed: 0,
            cMem: 0,
        },
        tag: 0,
    };
    let totalSizeToLoad = UTIL_getTotalFileSize(fileNamesTable, nbFiles);
    if nbFiles == 0 {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"No Files to Benchmark\0" as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        return 13 as libc::c_int;
    }
    if cLevel > ZSTD_maxCLevel() {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Invalid Compression Level\0" as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        return 14 as libc::c_int;
    }
    if totalSizeToLoad == UTIL_FILESIZE_UNKNOWN as U64 {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error loading files\0" as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        return 15 as libc::c_int;
    }
    fileSizes = calloc(
        nbFiles as libc::c_ulong,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
    ) as *mut size_t;
    if fileSizes.is_null() {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"not enough memory for fileSizes\0" as *const u8 as *const libc::c_char,
            );
            fflush(NULL as *mut FILE);
        }
        return 16 as libc::c_int;
    }
    if !dictFileName.is_null() {
        let dictFileSize = UTIL_getFileSize(dictFileName);
        if dictFileSize == UTIL_FILESIZE_UNKNOWN as U64 {
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error loading %s : %s \n\0" as *const u8 as *const libc::c_char,
                    dictFileName,
                    strerror(*__errno_location()),
                );
                fflush(NULL as *mut FILE);
            }
            free(fileSizes as *mut libc::c_void);
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"benchmark aborted\0" as *const u8 as *const libc::c_char,
                );
                fflush(NULL as *mut FILE);
            }
            return 17 as libc::c_int;
        }
        if dictFileSize
            > (64 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int))
                as libc::c_ulong
        {
            free(fileSizes as *mut libc::c_void);
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"dictionary file %s too large\0" as *const u8
                        as *const libc::c_char,
                    dictFileName,
                );
                fflush(NULL as *mut FILE);
            }
            return 18 as libc::c_int;
        }
        dictBufferSize = dictFileSize;
        dictBuffer = malloc(dictBufferSize);
        if dictBuffer.is_null() {
            free(fileSizes as *mut libc::c_void);
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"not enough memory for dictionary (%u bytes)\0" as *const u8
                        as *const libc::c_char,
                    dictBufferSize as libc::c_uint,
                );
                fflush(NULL as *mut FILE);
            }
            return 19 as libc::c_int;
        }
        let errorCode = BMK_loadFiles(
            dictBuffer,
            dictBufferSize,
            fileSizes,
            &mut dictFileName,
            1 as libc::c_int as libc::c_uint,
            displayLevel,
        );
        if errorCode != 0 {
            res = BMK_benchOutcome_error();
            current_block = 17016400332584056940;
        } else {
            current_block = 13707613154239713890;
        }
    } else {
        current_block = 13707613154239713890;
    }
    match current_block {
        13707613154239713890 => {
            benchedSize = (BMK_findMaxMem(
                totalSizeToLoad.wrapping_mul(3 as libc::c_int as libc::c_ulong),
            ))
                .wrapping_div(3 as libc::c_int as libc::c_ulong);
            if benchedSize > totalSizeToLoad {
                benchedSize = totalSizeToLoad;
            }
            if benchedSize < totalSizeToLoad {
                fprintf(
                    stderr,
                    b"Not enough memory; testing %u MB only...\n\0" as *const u8
                        as *const libc::c_char,
                    (benchedSize >> 20 as libc::c_int) as libc::c_uint,
                );
                fflush(NULL as *mut FILE);
            }
            srcBuffer = if benchedSize != 0 {
                malloc(benchedSize)
            } else {
                NULL as *mut libc::c_void
            };
            if srcBuffer.is_null() {
                free(dictBuffer);
                free(fileSizes as *mut libc::c_void);
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"not enough memory for srcBuffer\0" as *const u8
                            as *const libc::c_char,
                    );
                    fflush(NULL as *mut FILE);
                }
                return 20 as libc::c_int;
            }
            let errorCode_0 = BMK_loadFiles(
                srcBuffer,
                benchedSize,
                fileSizes,
                fileNamesTable,
                nbFiles,
                displayLevel,
            );
            if errorCode_0 != 0 {
                res = BMK_benchOutcome_error();
            } else {
                let mut mfName: [libc::c_char; 20] = [
                    0 as libc::c_int as libc::c_char,
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
                snprintf(
                    mfName.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                    b" %u files\0" as *const u8 as *const libc::c_char,
                    nbFiles,
                );
                let displayName = if nbFiles > 1 as libc::c_int as libc::c_uint {
                    mfName.as_mut_ptr() as *const libc::c_char
                } else {
                    *fileNamesTable.offset(0 as libc::c_int as isize)
                };
                res = BMK_benchCLevel(
                    srcBuffer,
                    benchedSize,
                    fileSizes,
                    nbFiles,
                    cLevel,
                    compressionParams,
                    dictBuffer,
                    dictBufferSize,
                    displayLevel,
                    displayName,
                    adv,
                );
            }
        }
        _ => {}
    }
    free(srcBuffer);
    free(dictBuffer);
    free(fileSizes as *mut libc::c_void);
    return (BMK_isSuccessful_benchOutcome(res) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFiles(
    mut fileNamesTable: *const *const libc::c_char,
    mut nbFiles: libc::c_uint,
    mut dictFileName: *const libc::c_char,
    mut cLevel: libc::c_int,
    mut compressionParams: *const ZSTD_compressionParameters,
    mut displayLevel: libc::c_int,
) -> libc::c_int {
    let adv = BMK_initAdvancedParams();
    return BMK_benchFilesAdvanced(
        fileNamesTable,
        nbFiles,
        dictFileName,
        cLevel,
        compressionParams,
        displayLevel,
        &adv,
    );
}
unsafe extern "C" fn run_static_initializers() {
    maxMemory = if ::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
    {
        (2 as libc::c_int as libc::c_uint)
            .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int)
            .wrapping_sub(
                (64 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_ulong
    } else {
        ((1 as libc::c_ulonglong)
            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(31 as libc::c_int as libc::c_ulong)) as size_t
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
