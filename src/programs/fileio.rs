use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_DCtx_s;
    pub type POOL_ctx_s;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn UTIL_requireUserConfirmation(
        prompt: *const libc::c_char,
        abortMsg: *const libc::c_char,
        acceptableLetters: *const libc::c_char,
        hasStdinInput: libc::c_int,
    ) -> libc::c_int;
    fn UTIL_stat(filename: *const libc::c_char, statbuf: *mut stat_t) -> libc::c_int;
    fn UTIL_setFDStat(
        fd: libc::c_int,
        filename: *const libc::c_char,
        statbuf: *const stat_t,
    ) -> libc::c_int;
    fn UTIL_utime(filename: *const libc::c_char, statbuf: *const stat_t) -> libc::c_int;
    fn UTIL_isRegularFileStat(statbuf: *const stat_t) -> libc::c_int;
    fn UTIL_isDirectoryStat(statbuf: *const stat_t) -> libc::c_int;
    fn UTIL_isFIFOStat(statbuf: *const stat_t) -> libc::c_int;
    fn UTIL_isBlockDevStat(statbuf: *const stat_t) -> libc::c_int;
    fn UTIL_getFileSizeStat(statbuf: *const stat_t) -> U64;
    fn UTIL_isRegularFile(infilename: *const libc::c_char) -> libc::c_int;
    fn UTIL_isDirectory(infilename: *const libc::c_char) -> libc::c_int;
    fn UTIL_isSameFile(
        file1: *const libc::c_char,
        file2: *const libc::c_char,
    ) -> libc::c_int;
    fn UTIL_isSameFileStat(
        file1: *const libc::c_char,
        file2: *const libc::c_char,
        file1Stat: *const stat_t,
        file2Stat: *const stat_t,
    ) -> libc::c_int;
    fn UTIL_isCompressedFile(
        infilename: *const libc::c_char,
        extensionList: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn UTIL_isConsole(file: *mut FILE) -> libc::c_int;
    fn UTIL_getFileSize(infilename: *const libc::c_char) -> U64;
    fn UTIL_makeHumanReadableSize(size: U64) -> UTIL_HumanReadableSize_t;
    fn UTIL_compareStr(p1: *const libc::c_void, p2: *const libc::c_void) -> libc::c_int;
    fn UTIL_mirrorSourceFilesDirectories(
        fileNamesTable: *mut *const libc::c_char,
        nbFiles: libc::c_uint,
        outDirName: *const libc::c_char,
    );
    fn UTIL_createMirroredDestDirName(
        srcFileName: *const libc::c_char,
        outDirRootName: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn clock() -> clock_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn UTIL_getTime() -> UTIL_time_t;
    fn UTIL_clockSpanNano(clockStart: UTIL_time_t) -> PTime;
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> PTime;
    fn ZSTD_getFrameContentSize(
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> libc::c_ulonglong;
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    fn ZSTD_minCLevel() -> libc::c_int;
    fn ZSTD_maxCLevel() -> libc::c_int;
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    fn ZSTD_CCtx_setParameter(
        cctx: *mut ZSTD_CCtx,
        param: ZSTD_cParameter,
        value: libc::c_int,
    ) -> size_t;
    fn ZSTD_CCtx_setPledgedSrcSize(
        cctx: *mut ZSTD_CCtx,
        pledgedSrcSize: libc::c_ulonglong,
    ) -> size_t;
    fn ZSTD_DCtx_setParameter(
        dctx: *mut ZSTD_DCtx,
        param: ZSTD_dParameter,
        value: libc::c_int,
    ) -> size_t;
    fn ZSTD_DCtx_reset(dctx: *mut ZSTD_DCtx, reset: ZSTD_ResetDirective) -> size_t;
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    fn ZSTD_compressStream2(
        cctx: *mut ZSTD_CCtx,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
        endOp: ZSTD_EndDirective,
    ) -> size_t;
    fn ZSTD_CStreamInSize() -> size_t;
    fn ZSTD_CStreamOutSize() -> size_t;
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    fn ZSTD_DStreamInSize() -> size_t;
    fn ZSTD_DStreamOutSize() -> size_t;
    fn ZSTD_frameHeaderSize(src: *const libc::c_void, srcSize: size_t) -> size_t;
    fn ZSTD_getFrameHeader(
        zfhPtr: *mut ZSTD_frameHeader,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_getCParams(
        compressionLevel: libc::c_int,
        estimatedSrcSize: libc::c_ulonglong,
        dictSize: size_t,
    ) -> ZSTD_compressionParameters;
    fn ZSTD_CCtx_loadDictionary_byReference(
        cctx: *mut ZSTD_CCtx,
        dict: *const libc::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTD_CCtx_getParameter(
        cctx: *const ZSTD_CCtx,
        param: ZSTD_cParameter,
        value: *mut libc::c_int,
    ) -> size_t;
    fn ZSTD_isFrame(buffer: *const libc::c_void, size: size_t) -> libc::c_uint;
    fn ZSTD_DCtx_loadDictionary_byReference(
        dctx: *mut ZSTD_DCtx,
        dict: *const libc::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTD_DCtx_setMaxWindowSize(dctx: *mut ZSTD_DCtx, maxWindowSize: size_t) -> size_t;
    fn ZSTD_getFrameProgression(cctx: *const ZSTD_CCtx) -> ZSTD_frameProgression;
    fn ZSTD_toFlushNow(cctx: *mut ZSTD_CCtx) -> size_t;
    fn ZSTD_DCtx_refPrefix(
        dctx: *mut ZSTD_DCtx,
        prefix: *const libc::c_void,
        prefixSize: size_t,
    ) -> size_t;
    fn ZSTD_CCtx_refPrefix(
        cctx: *mut ZSTD_CCtx,
        prefix: *const libc::c_void,
        prefixSize: size_t,
    ) -> size_t;
    fn AIO_supported() -> libc::c_int;
    fn AIO_WritePool_releaseIoJob(job: *mut IOJob_t);
    fn AIO_WritePool_acquireJob(ctx: *mut WritePoolCtx_t) -> *mut IOJob_t;
    fn AIO_WritePool_enqueueAndReacquireWriteJob(job: *mut *mut IOJob_t);
    fn AIO_WritePool_sparseWriteEnd(ctx: *mut WritePoolCtx_t);
    fn AIO_WritePool_setFile(ctx: *mut WritePoolCtx_t, file: *mut FILE);
    fn AIO_WritePool_getFile(ctx: *const WritePoolCtx_t) -> *mut FILE;
    fn AIO_WritePool_closeFile(ctx: *mut WritePoolCtx_t) -> libc::c_int;
    fn AIO_WritePool_create(
        prefs: *const FIO_prefs_t,
        bufferSize: size_t,
    ) -> *mut WritePoolCtx_t;
    fn AIO_WritePool_free(ctx: *mut WritePoolCtx_t);
    fn AIO_WritePool_setAsync(ctx: *mut WritePoolCtx_t, async_0: libc::c_int);
    fn AIO_ReadPool_create(
        prefs: *const FIO_prefs_t,
        bufferSize: size_t,
    ) -> *mut ReadPoolCtx_t;
    fn AIO_ReadPool_free(ctx: *mut ReadPoolCtx_t);
    fn AIO_ReadPool_setAsync(ctx: *mut ReadPoolCtx_t, async_0: libc::c_int);
    fn AIO_ReadPool_fillBuffer(ctx: *mut ReadPoolCtx_t, n: size_t) -> size_t;
    fn AIO_ReadPool_consumeBytes(ctx: *mut ReadPoolCtx_t, n: size_t);
    fn AIO_ReadPool_closeFile(ctx: *mut ReadPoolCtx_t) -> libc::c_int;
    fn AIO_ReadPool_setFile(ctx: *mut ReadPoolCtx_t, file: *mut FILE);
    fn AIO_ReadPool_getFile(ctx: *const ReadPoolCtx_t) -> *mut FILE;
    fn ZSTD_getErrorCode(functionResult: size_t) -> ZSTD_ErrorCode;
    fn backtrace(__array: *mut *mut libc::c_void, __size: libc::c_int) -> libc::c_int;
    fn backtrace_symbols(
        __array: *const *mut libc::c_void,
        __size: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U8 = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type stat_t = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_HumanReadableSize_t {
    pub value: libc::c_double,
    pub precision: libc::c_int,
    pub suffix: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileNamesTable {
    pub fileNames: *mut *const libc::c_char,
    pub buf: *mut libc::c_char,
    pub tableSize: size_t,
    pub tableCapacity: size_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type PTime = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
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
pub type ZSTD_dParameter = libc::c_uint;
pub const ZSTD_d_experimentalParam6: ZSTD_dParameter = 1005;
pub const ZSTD_d_experimentalParam5: ZSTD_dParameter = 1004;
pub const ZSTD_d_experimentalParam4: ZSTD_dParameter = 1003;
pub const ZSTD_d_experimentalParam3: ZSTD_dParameter = 1002;
pub const ZSTD_d_experimentalParam2: ZSTD_dParameter = 1001;
pub const ZSTD_d_experimentalParam1: ZSTD_dParameter = 1000;
pub const ZSTD_d_windowLogMax: ZSTD_dParameter = 100;
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
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZSTD_EndDirective = libc::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
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
pub type C2RustUnnamed = libc::c_uint;
pub const ZSTD_f_zstd1_magicless: C2RustUnnamed = 1;
pub const ZSTD_f_zstd1: C2RustUnnamed = 0;
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
pub type ZSTD_frameType_e = libc::c_uint;
pub const ZSTD_skippableFrame: ZSTD_frameType_e = 1;
pub const ZSTD_frame: ZSTD_frameType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameHeader {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_ulonglong,
    pub blockSizeMax: libc::c_uint,
    pub frameType: ZSTD_frameType_e,
    pub headerSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
    pub _reserved1: libc::c_uint,
    pub _reserved2: libc::c_uint,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_display_prefs_s {
    pub displayLevel: libc::c_int,
    pub progressSetting: FIO_progressSetting_e,
}
pub type FIO_progressSetting_e = libc::c_uint;
pub const FIO_ps_always: FIO_progressSetting_e = 2;
pub const FIO_ps_never: FIO_progressSetting_e = 1;
pub const FIO_ps_auto: FIO_progressSetting_e = 0;
pub type FIO_display_prefs_t = FIO_display_prefs_s;
pub type FIO_compressionType_t = libc::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_prefs_s {
    pub compressionType: FIO_compressionType_t,
    pub sparseFileSupport: libc::c_int,
    pub dictIDFlag: libc::c_int,
    pub checksumFlag: libc::c_int,
    pub blockSize: libc::c_int,
    pub overlapLog: libc::c_int,
    pub adaptiveMode: libc::c_int,
    pub useRowMatchFinder: libc::c_int,
    pub rsyncable: libc::c_int,
    pub minAdaptLevel: libc::c_int,
    pub maxAdaptLevel: libc::c_int,
    pub ldmFlag: libc::c_int,
    pub ldmHashLog: libc::c_int,
    pub ldmMinMatch: libc::c_int,
    pub ldmBucketSizeLog: libc::c_int,
    pub ldmHashRateLog: libc::c_int,
    pub streamSrcSize: size_t,
    pub targetCBlockSize: size_t,
    pub srcSizeHint: libc::c_int,
    pub testMode: libc::c_int,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
    pub removeSrcFile: libc::c_int,
    pub overwrite: libc::c_int,
    pub asyncIO: libc::c_int,
    pub memLimit: libc::c_uint,
    pub nbWorkers: libc::c_int,
    pub excludeCompressedFiles: libc::c_int,
    pub patchFromMode: libc::c_int,
    pub contentSize: libc::c_int,
    pub allowBlockDevices: libc::c_int,
    pub passThrough: libc::c_int,
    pub mmapDict: ZSTD_paramSwitch_e,
}
pub type FIO_prefs_t = FIO_prefs_s;
pub type FIO_dictBufferType_t = libc::c_uint;
pub const FIO_mmapDict: FIO_dictBufferType_t = 1;
pub const FIO_mallocDict: FIO_dictBufferType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_Dict_t {
    pub dictBuffer: *mut libc::c_void,
    pub dictBufferSize: size_t,
    pub dictBufferType: FIO_dictBufferType_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_ctx_s {
    pub nbFilesTotal: libc::c_int,
    pub hasStdinInput: libc::c_int,
    pub hasStdoutOutput: libc::c_int,
    pub currFileIdx: libc::c_int,
    pub nbFilesProcessed: libc::c_int,
    pub totalBytesInput: size_t,
    pub totalBytesOutput: size_t,
}
pub type FIO_ctx_t = FIO_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cRess_t {
    pub dict: FIO_Dict_t,
    pub dictFileName: *const libc::c_char,
    pub dictFileStat: stat_t,
    pub cctx: *mut ZSTD_CStream,
    pub writeCtx: *mut WritePoolCtx_t,
    pub readCtx: *mut ReadPoolCtx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReadPoolCtx_t {
    pub base: IOPoolCtx_t,
    pub reachedEof: libc::c_int,
    pub nextReadOffset: U64,
    pub waitingOnOffset: U64,
    pub currentJobHeld: *mut libc::c_void,
    pub coalesceBuffer: *mut U8,
    pub srcBuffer: *mut U8,
    pub srcBufferLoaded: size_t,
    pub completedJobs: [*mut libc::c_void; 10],
    pub completedJobsCount: libc::c_int,
    pub jobCompletedCond: ZSTD_pthread_cond_t,
}
pub type ZSTD_pthread_cond_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOPoolCtx_t {
    pub threadPool: *mut POOL_ctx,
    pub threadPoolActive: libc::c_int,
    pub totalIoJobs: libc::c_int,
    pub prefs: *const FIO_prefs_t,
    pub poolFunction: POOL_function,
    pub file: *mut FILE,
    pub ioJobsMutex: ZSTD_pthread_mutex_t,
    pub availableJobs: [*mut libc::c_void; 10],
    pub availableJobsCount: libc::c_int,
    pub jobBufferSize: size_t,
}
pub type ZSTD_pthread_mutex_t = libc::c_int;
pub type POOL_function = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type POOL_ctx = POOL_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WritePoolCtx_t {
    pub base: IOPoolCtx_t,
    pub storedSkips: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOJob_t {
    pub ctx: *mut libc::c_void,
    pub file: *mut FILE,
    pub buffer: *mut libc::c_void,
    pub bufferSize: size_t,
    pub usedBufferSize: size_t,
    pub offset: U64,
}
pub type speedChange_e = libc::c_uint;
pub const faster: speedChange_e = 2;
pub const slower: speedChange_e = 1;
pub const noChange: speedChange_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dRess_t {
    pub dict: FIO_Dict_t,
    pub dctx: *mut ZSTD_DStream,
    pub writeCtx: *mut WritePoolCtx_t,
    pub readCtx: *mut ReadPoolCtx_t,
}
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
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
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileInfo_t {
    pub decompressedSize: U64,
    pub compressedSize: U64,
    pub windowSize: U64,
    pub numActualFrames: libc::c_int,
    pub numSkippableFrames: libc::c_int,
    pub decompUnavailable: libc::c_int,
    pub usesCheck: libc::c_int,
    pub checksum: [BYTE; 4],
    pub nbFiles: U32,
    pub dictID: libc::c_uint,
}
pub type InfoError = libc::c_uint;
pub const info_truncated_input: InfoError = 4;
pub const info_file_error: InfoError = 3;
pub const info_not_zstd: InfoError = 2;
pub const info_frame_error: InfoError = 1;
pub const info_success: InfoError = 0;
pub const _IOFBF: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZSTD_SPARSE_DEFAULT: libc::c_int = 1 as libc::c_int;
pub const ZSTD_START_SYMBOLLIST_FRAME: libc::c_int = 2 as libc::c_int;
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
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as libc::c_int
            + ((*p.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as U16;
    };
}
#[inline]
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as U32)
        .wrapping_add(
            (*(memPtr as *const BYTE).offset(2 as libc::c_int as isize) as U32)
                << 16 as libc::c_int,
        );
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
pub const PATH_SEP: libc::c_int = '/' as i32;
pub const __S_IREAD: libc::c_int = 0o400 as libc::c_int;
pub const __S_IWRITE: libc::c_int = 0o200 as libc::c_int;
pub const UTIL_FILESIZE_UNKNOWN: libc::c_int = -(1 as libc::c_int);
pub const CLOCKS_PER_SEC: libc::c_int = 1000000 as libc::c_int;
pub const S_IRUSR: libc::c_int = __S_IREAD;
pub const S_IWUSR: libc::c_int = __S_IWRITE;
pub const S_IRGRP: libc::c_int = S_IRUSR >> 3 as libc::c_int;
pub const S_IWGRP: libc::c_int = S_IWUSR >> 3 as libc::c_int;
pub const O_WRONLY: libc::c_int = 0o1 as libc::c_int;
pub const O_CREAT: libc::c_int = 0o100 as libc::c_int;
pub const O_TRUNC: libc::c_int = 0o1000 as libc::c_int;
pub const S_IROTH: libc::c_int = S_IRGRP >> 3 as libc::c_int;
pub const S_IWOTH: libc::c_int = S_IWGRP >> 3 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const SIGBUS: libc::c_int = 7 as libc::c_int;
pub const SIGABRT: libc::c_int = 6;
pub const SIGFPE: libc::c_int = 8;
pub const SIGILL: libc::c_int = 4 as libc::c_int;
pub const SIGSEGV: libc::c_int = 11 as libc::c_int;
pub const SIG_DFL: libc::c_int = 0 as libc::c_int;
pub const SIGINT: libc::c_int = 2;
pub const SIG_IGN: libc::c_int = 1 as libc::c_int;
pub const SEC_TO_MICRO: libc::c_int = 1000000 as libc::c_int;
pub const ZSTD_MAGICNUMBER: libc::c_uint = 0xfd2fb528 as libc::c_uint;
pub const ZSTD_MAGIC_SKIPPABLE_START: libc::c_int = 0x184d2a50 as libc::c_int;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: libc::c_uint = 0xfffffff0 as libc::c_uint;
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1 as libc::c_int) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
pub const ZSTD_CONTENTSIZE_ERROR: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(2 as libc::c_int as libc::c_ulonglong);
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT: libc::c_int = 27 as libc::c_int;
pub const LZ4_EXTENSION: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".lz4\0")
};
pub const LZMA_EXTENSION: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b".lzma\0")
};
pub const XZ_EXTENSION: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".xz\0")
};
pub const GZ_EXTENSION: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".gz\0")
};
pub const ZSTD_EXTENSION: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".zst\0")
};
pub const stdinmark: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"/*stdin*\\\0")
};
pub const nulmark: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"/dev/null\0")
};
pub const ZSTD_ALT_EXTENSION: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b".zstd\0")
};
pub const ZSTD_FRAMEHEADERSIZE_MAX: libc::c_int = 18 as libc::c_int;
pub const stdoutmark: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"/*stdout*\\\0")
};
pub const TZSTD_EXTENSION: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b".tzst\0")
};
pub const ZSTD_WINDOWLOG_MAX_64: libc::c_int = 31 as libc::c_int;
pub const ZSTD_WINDOWLOG_MAX_32: libc::c_int = 30 as libc::c_int;
pub const TGZ_EXTENSION: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".tgz\0")
};
pub const TXZ_EXTENSION: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".txz\0")
};
pub const TLZ4_EXTENSION: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b".tlz4\0")
};
pub const REFRESH_RATE: libc::c_ulong = (SEC_TO_MICRO as PTime)
    .wrapping_div(6 as libc::c_int as libc::c_ulong);
pub const LONG_TELL: unsafe extern "C" fn(*mut FILE) -> libc::c_long = ftell;
#[no_mangle]
pub static mut g_display_prefs: FIO_display_prefs_t = {
    let mut init = FIO_display_prefs_s {
        displayLevel: 2 as libc::c_int,
        progressSetting: FIO_ps_auto,
    };
    init
};
#[no_mangle]
pub static mut g_displayClock: UTIL_time_t = {
    let mut init = UTIL_time_t {
        t: 0 as libc::c_int as PTime,
    };
    init
};
pub const LZ4_MAGICNUMBER: libc::c_int = 0x184d2204 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn FIO_zlibVersion() -> *const libc::c_char {
    return b"Unsupported\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_lz4Version() -> *const libc::c_char {
    return b"Unsupported\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_lzmaVersion() -> *const libc::c_char {
    return b"Unsupported\0" as *const u8 as *const libc::c_char;
}
pub const ADAPT_WINDOWLOG_DEFAULT: libc::c_int = 23 as libc::c_int;
pub const DICTSIZE_MAX: libc::c_int = 32 as libc::c_int
    * ((1 as libc::c_int) << 20 as libc::c_int);
pub const DEFAULT_FILE_PERMISSIONS: libc::c_int = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP
    | S_IROTH | S_IWOTH;
pub const TEMPORARY_FILE_PERMISSIONS: libc::c_int = S_IRUSR | S_IWUSR;
static mut g_artefact: *const libc::c_char = NULL as *const libc::c_char;
unsafe extern "C" fn INThandler(mut sig: libc::c_int) {
    if sig == 2 as libc::c_int {} else {
        __assert_fail(
            b"sig==SIGINT\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            134 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void INThandler(int)\0"))
                .as_ptr(),
        );
    }
    signal(
        sig,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(SIG_IGN as libc::intptr_t),
    );
    if !g_artefact.is_null() {
        if UTIL_isRegularFile(g_artefact) != 0 {} else {
            __assert_fail(
                b"UTIL_isRegularFile(g_artefact)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                139 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void INThandler(int)\0"))
                    .as_ptr(),
            );
        }
        remove(g_artefact);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(2 as libc::c_int);
}
unsafe extern "C" fn addHandler(mut dstFileName: *const libc::c_char) {
    if UTIL_isRegularFile(dstFileName) != 0 {
        g_artefact = dstFileName;
        signal(SIGINT, Some(INThandler as unsafe extern "C" fn(libc::c_int) -> ()));
    } else {
        g_artefact = NULL as *const libc::c_char;
    };
}
unsafe extern "C" fn clearHandler() {
    if !g_artefact.is_null() {
        signal(
            SIGINT,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(SIG_DFL as libc::intptr_t),
        );
    }
    g_artefact = NULL as *const libc::c_char;
}
pub const MAX_STACK_FRAMES: libc::c_int = 50 as libc::c_int;
unsafe extern "C" fn ABRThandler(mut sig: libc::c_int) {
    let mut name = 0 as *const libc::c_char;
    let mut addrlist: [*mut libc::c_void; 50] = [0 as *mut libc::c_void; 50];
    let mut symbollist = 0 as *mut *mut libc::c_char;
    let mut addrlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    match sig {
        SIGABRT => {
            name = b"SIGABRT\0" as *const u8 as *const libc::c_char;
        }
        SIGFPE => {
            name = b"SIGFPE\0" as *const u8 as *const libc::c_char;
        }
        SIGILL => {
            name = b"SIGILL\0" as *const u8 as *const libc::c_char;
        }
        SIGINT => {
            name = b"SIGINT\0" as *const u8 as *const libc::c_char;
        }
        SIGSEGV => {
            name = b"SIGSEGV\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            name = b"UNKNOWN\0" as *const u8 as *const libc::c_char;
        }
    }
    fprintf(
        stderr,
        b"Caught %s signal, printing stack:\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    addrlen = backtrace(addrlist.as_mut_ptr(), MAX_STACK_FRAMES);
    if addrlen == 0 as libc::c_int {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    symbollist = backtrace_symbols(addrlist.as_mut_ptr(), addrlen);
    i = ZSTD_START_SYMBOLLIST_FRAME;
    while i < addrlen {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            *symbollist.offset(i as isize),
        );
        i += 1;
    }
    free(symbollist as *mut libc::c_void);
    signal(
        sig,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(SIG_DFL as libc::intptr_t),
    );
    raise(sig);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_addAbortHandler() {
    signal(SIGABRT, Some(ABRThandler as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGFPE, Some(ABRThandler as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGILL, Some(ABRThandler as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGSEGV, Some(ABRThandler as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGBUS, Some(ABRThandler as unsafe extern "C" fn(libc::c_int) -> ()));
}
unsafe extern "C" fn FIO_shouldDisplayFileSummary(
    mut fCtx: *const FIO_ctx_t,
) -> libc::c_int {
    return ((*fCtx).nbFilesTotal <= 1 as libc::c_int
        || g_display_prefs.displayLevel >= 3 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn FIO_shouldDisplayMultipleFileSummary(
    mut fCtx: *const FIO_ctx_t,
) -> libc::c_int {
    let shouldDisplay = ((*fCtx).nbFilesProcessed >= 1 as libc::c_int
        && (*fCtx).nbFilesTotal > 1 as libc::c_int) as libc::c_int;
    if shouldDisplay != 0 || FIO_shouldDisplayFileSummary(fCtx) != 0
        || (*fCtx).nbFilesProcessed == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"shouldDisplay || FIO_shouldDisplayFileSummary(fCtx) || fCtx->nbFilesProcessed == 0\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            265 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"int FIO_shouldDisplayMultipleFileSummary(const FIO_ctx_t *)\0"))
                .as_ptr(),
        );
    }
    return shouldDisplay;
}
pub const FIO_OVERLAP_LOG_NOTSET: libc::c_int = 9999 as libc::c_int;
pub const FIO_LDM_PARAM_NOTSET: libc::c_int = 9999 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn FIO_createPreferences() -> *mut FIO_prefs_t {
    let ret = malloc(::core::mem::size_of::<FIO_prefs_t>() as libc::c_ulong)
        as *mut FIO_prefs_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                281 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                21 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Allocation error : not enough memory\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(21 as libc::c_int);
    }
    (*ret).compressionType = FIO_zstdCompression;
    (*ret).overwrite = 0 as libc::c_int;
    (*ret).sparseFileSupport = ZSTD_SPARSE_DEFAULT;
    (*ret).dictIDFlag = 1 as libc::c_int;
    (*ret).checksumFlag = 1 as libc::c_int;
    (*ret).removeSrcFile = 0 as libc::c_int;
    (*ret).memLimit = 0 as libc::c_int as libc::c_uint;
    (*ret).nbWorkers = 1 as libc::c_int;
    (*ret).blockSize = 0 as libc::c_int;
    (*ret).overlapLog = FIO_OVERLAP_LOG_NOTSET;
    (*ret).adaptiveMode = 0 as libc::c_int;
    (*ret).rsyncable = 0 as libc::c_int;
    (*ret).minAdaptLevel = -(50 as libc::c_int);
    (*ret).maxAdaptLevel = 22 as libc::c_int;
    (*ret).ldmFlag = 0 as libc::c_int;
    (*ret).ldmHashLog = 0 as libc::c_int;
    (*ret).ldmMinMatch = 0 as libc::c_int;
    (*ret).ldmBucketSizeLog = FIO_LDM_PARAM_NOTSET;
    (*ret).ldmHashRateLog = FIO_LDM_PARAM_NOTSET;
    (*ret).streamSrcSize = 0 as libc::c_int as size_t;
    (*ret).targetCBlockSize = 0 as libc::c_int as size_t;
    (*ret).srcSizeHint = 0 as libc::c_int;
    (*ret).testMode = 0 as libc::c_int;
    (*ret).literalCompressionMode = ZSTD_ps_auto;
    (*ret).excludeCompressedFiles = 0 as libc::c_int;
    (*ret).allowBlockDevices = 0 as libc::c_int;
    (*ret).asyncIO = AIO_supported();
    (*ret).passThrough = -(1 as libc::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_createContext() -> *mut FIO_ctx_t {
    let ret = malloc(::core::mem::size_of::<FIO_ctx_t>() as libc::c_ulong)
        as *mut FIO_ctx_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                317 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                21 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Allocation error : not enough memory\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(21 as libc::c_int);
    }
    (*ret).currFileIdx = 0 as libc::c_int;
    (*ret).hasStdinInput = 0 as libc::c_int;
    (*ret).hasStdoutOutput = 0 as libc::c_int;
    (*ret).nbFilesTotal = 1 as libc::c_int;
    (*ret).nbFilesProcessed = 0 as libc::c_int;
    (*ret).totalBytesInput = 0 as libc::c_int as size_t;
    (*ret).totalBytesOutput = 0 as libc::c_int as size_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_freePreferences(prefs: *mut FIO_prefs_t) {
    free(prefs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_freeContext(fCtx: *mut FIO_ctx_t) {
    free(fCtx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNotificationLevel(mut level: libc::c_int) {
    g_display_prefs.displayLevel = level;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setProgressSetting(mut setting: FIO_progressSetting_e) {
    g_display_prefs.progressSetting = setting;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setCompressionType(
    prefs: *mut FIO_prefs_t,
    mut compressionType: FIO_compressionType_t,
) {
    (*prefs).compressionType = compressionType;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_overwriteMode(prefs: *mut FIO_prefs_t) {
    (*prefs).overwrite = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setSparseWrite(
    prefs: *mut FIO_prefs_t,
    mut sparse: libc::c_int,
) {
    (*prefs).sparseFileSupport = sparse;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setDictIDFlag(
    prefs: *mut FIO_prefs_t,
    mut dictIDFlag: libc::c_int,
) {
    (*prefs).dictIDFlag = dictIDFlag;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setChecksumFlag(
    prefs: *mut FIO_prefs_t,
    mut checksumFlag: libc::c_int,
) {
    (*prefs).checksumFlag = checksumFlag;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setRemoveSrcFile(
    prefs: *mut FIO_prefs_t,
    mut flag: libc::c_int,
) {
    (*prefs).removeSrcFile = (flag != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setMemLimit(
    prefs: *mut FIO_prefs_t,
    mut memLimit: libc::c_uint,
) {
    (*prefs).memLimit = memLimit;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNbWorkers(
    prefs: *mut FIO_prefs_t,
    mut nbWorkers: libc::c_int,
) {
    if nbWorkers > 0 as libc::c_int {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Note : multi-threading is disabled \n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    (*prefs).nbWorkers = nbWorkers;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setExcludeCompressedFile(
    prefs: *mut FIO_prefs_t,
    mut excludeCompressedFiles: libc::c_int,
) {
    (*prefs).excludeCompressedFiles = excludeCompressedFiles;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAllowBlockDevices(
    prefs: *mut FIO_prefs_t,
    mut allowBlockDevices: libc::c_int,
) {
    (*prefs).allowBlockDevices = allowBlockDevices;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setBlockSize(
    prefs: *mut FIO_prefs_t,
    mut blockSize: libc::c_int,
) {
    if blockSize != 0 && (*prefs).nbWorkers == 0 as libc::c_int {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Setting block size is useless in single-thread mode \n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    (*prefs).blockSize = blockSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setOverlapLog(
    prefs: *mut FIO_prefs_t,
    mut overlapLog: libc::c_int,
) {
    if overlapLog != 0 && (*prefs).nbWorkers == 0 as libc::c_int {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Setting overlapLog is useless in single-thread mode \n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    (*prefs).overlapLog = overlapLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptiveMode(
    prefs: *mut FIO_prefs_t,
    mut adapt: libc::c_int,
) {
    if adapt > 0 as libc::c_int && (*prefs).nbWorkers == 0 as libc::c_int {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                394 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Adaptive mode is not compatible with single thread mode \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(1 as libc::c_int);
    }
    (*prefs).adaptiveMode = adapt;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setUseRowMatchFinder(
    prefs: *mut FIO_prefs_t,
    mut useRowMatchFinder: libc::c_int,
) {
    (*prefs).useRowMatchFinder = useRowMatchFinder;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setRsyncable(
    prefs: *mut FIO_prefs_t,
    mut rsyncable: libc::c_int,
) {
    if rsyncable > 0 as libc::c_int && (*prefs).nbWorkers == 0 as libc::c_int {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                404 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Rsyncable mode is not compatible with single thread mode \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(1 as libc::c_int);
    }
    (*prefs).rsyncable = rsyncable;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setStreamSrcSize(
    prefs: *mut FIO_prefs_t,
    mut streamSrcSize: size_t,
) {
    (*prefs).streamSrcSize = streamSrcSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setTargetCBlockSize(
    prefs: *mut FIO_prefs_t,
    mut targetCBlockSize: size_t,
) {
    (*prefs).targetCBlockSize = targetCBlockSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setSrcSizeHint(
    prefs: *mut FIO_prefs_t,
    mut srcSizeHint: size_t,
) {
    (*prefs)
        .srcSizeHint = (if (2147483647 as libc::c_int as size_t) < srcSizeHint {
        2147483647 as libc::c_int as size_t
    } else {
        srcSizeHint
    }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setTestMode(
    prefs: *mut FIO_prefs_t,
    mut testMode: libc::c_int,
) {
    (*prefs).testMode = (testMode != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLiteralCompressionMode(
    prefs: *mut FIO_prefs_t,
    mut mode: ZSTD_paramSwitch_e,
) {
    (*prefs).literalCompressionMode = mode;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptMin(
    prefs: *mut FIO_prefs_t,
    mut minCLevel: libc::c_int,
) {
    if minCLevel >= ZSTD_minCLevel() {} else {
        __assert_fail(
            b"minCLevel >= ZSTD_minCLevel()\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            433 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void FIO_setAdaptMin(FIO_prefs_t *const, int)\0"))
                .as_ptr(),
        );
    }
    (*prefs).minAdaptLevel = minCLevel;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptMax(
    prefs: *mut FIO_prefs_t,
    mut maxCLevel: libc::c_int,
) {
    (*prefs).maxAdaptLevel = maxCLevel;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmFlag(
    prefs: *mut FIO_prefs_t,
    mut ldmFlag: libc::c_uint,
) {
    (*prefs).ldmFlag = (ldmFlag > 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashLog(
    prefs: *mut FIO_prefs_t,
    mut ldmHashLog: libc::c_int,
) {
    (*prefs).ldmHashLog = ldmHashLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmMinMatch(
    prefs: *mut FIO_prefs_t,
    mut ldmMinMatch: libc::c_int,
) {
    (*prefs).ldmMinMatch = ldmMinMatch;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmBucketSizeLog(
    prefs: *mut FIO_prefs_t,
    mut ldmBucketSizeLog: libc::c_int,
) {
    (*prefs).ldmBucketSizeLog = ldmBucketSizeLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashRateLog(
    prefs: *mut FIO_prefs_t,
    mut ldmHashRateLog: libc::c_int,
) {
    (*prefs).ldmHashRateLog = ldmHashRateLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setPatchFromMode(
    prefs: *mut FIO_prefs_t,
    mut value: libc::c_int,
) {
    (*prefs).patchFromMode = (value != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setContentSize(
    prefs: *mut FIO_prefs_t,
    mut value: libc::c_int,
) {
    (*prefs).contentSize = (value != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAsyncIOFlag(
    prefs: *mut FIO_prefs_t,
    mut value: libc::c_int,
) {
    if g_display_prefs.displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"Note : asyncio is disabled (lack of multithreading support) \n\0"
                as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setPassThroughFlag(
    prefs: *mut FIO_prefs_t,
    mut value: libc::c_int,
) {
    (*prefs).passThrough = (value != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setMMapDict(
    prefs: *mut FIO_prefs_t,
    mut value: ZSTD_paramSwitch_e,
) {
    (*prefs).mmapDict = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setHasStdoutOutput(
    fCtx: *mut FIO_ctx_t,
    mut value: libc::c_int,
) {
    (*fCtx).hasStdoutOutput = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNbFilesTotal(
    fCtx: *mut FIO_ctx_t,
    mut value: libc::c_int,
) {
    (*fCtx).nbFilesTotal = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_determineHasStdinInput(
    fCtx: *mut FIO_ctx_t,
    filenames: *const FileNamesTable,
) {
    let mut i = 0 as libc::c_int as size_t;
    while i < (*filenames).tableSize {
        if strcmp(stdinmark.as_ptr(), *((*filenames).fileNames).offset(i as isize)) == 0
        {
            (*fCtx).hasStdinInput = 1 as libc::c_int;
            return;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn FIO_removeFile(mut path: *const libc::c_char) -> libc::c_int {
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(path, &mut statbuf) == 0 {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: Failed to stat %s while trying to remove it\n\0" as *const u8
                    as *const libc::c_char,
                path,
            );
        }
        return 0 as libc::c_int;
    }
    if UTIL_isRegularFileStat(&mut statbuf) == 0 {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: Refusing to remove non-regular file %s\n\0" as *const u8
                    as *const libc::c_char,
                path,
            );
        }
        return 0 as libc::c_int;
    }
    return remove(path);
}
unsafe extern "C" fn FIO_openSrcFile(
    prefs: *const FIO_prefs_t,
    mut srcFileName: *const libc::c_char,
    mut statbuf: *mut stat_t,
) -> *mut FILE {
    let mut allowBlockDevices = if !prefs.is_null() {
        (*prefs).allowBlockDevices
    } else {
        0 as libc::c_int
    };
    if !srcFileName.is_null() {} else {
        __assert_fail(
            b"srcFileName != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            546 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"FILE *FIO_openSrcFile(const FIO_prefs_t *const, const char *, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !statbuf.is_null() {} else {
        __assert_fail(
            b"statbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            547 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"FILE *FIO_openSrcFile(const FIO_prefs_t *const, const char *, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        if g_display_prefs.displayLevel >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"Using stdin for input \n\0" as *const u8 as *const libc::c_char,
            );
        }
        return stdin;
    }
    if UTIL_stat(srcFileName, statbuf) == 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: can't stat %s : %s -- ignored \n\0" as *const u8
                    as *const libc::c_char,
                srcFileName,
                strerror(*__errno_location()),
            );
        }
        return NULL as *mut FILE;
    }
    if UTIL_isRegularFileStat(statbuf) == 0 && UTIL_isFIFOStat(statbuf) == 0
        && !(allowBlockDevices != 0 && UTIL_isBlockDevStat(statbuf) != 0)
    {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s is not a regular file -- ignored \n\0" as *const u8
                    as *const libc::c_char,
                srcFileName,
            );
        }
        return NULL as *mut FILE;
    }
    let f = fopen(srcFileName, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s: %s \n\0" as *const u8 as *const libc::c_char,
                srcFileName,
                strerror(*__errno_location()),
            );
        }
    }
    return f;
}
unsafe extern "C" fn FIO_openDstFile(
    mut fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut srcFileName: *const libc::c_char,
    mut dstFileName: *const libc::c_char,
    mode: libc::c_int,
) -> *mut FILE {
    let mut isDstRegFile: libc::c_int = 0;
    if (*prefs).testMode != 0 {
        return NULL as *mut FILE;
    }
    if !dstFileName.is_null() {} else {
        __assert_fail(
            b"dstFileName != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            588 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"FILE *FIO_openDstFile(FIO_ctx_t *, FIO_prefs_t *const, const char *, const char *, const int)\0",
            ))
                .as_ptr(),
        );
    }
    if strcmp(dstFileName, stdoutmark.as_ptr()) == 0 {
        if g_display_prefs.displayLevel >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"Using stdout for output \n\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*prefs).sparseFileSupport == 1 as libc::c_int {
            (*prefs).sparseFileSupport = 0 as libc::c_int;
            if g_display_prefs.displayLevel >= 4 as libc::c_int {
                fprintf(
                    stderr,
                    b"Sparse File Support is automatically disabled on stdout ; try --sparse \n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        return stdout;
    }
    if !srcFileName.is_null() && UTIL_isSameFile(srcFileName, dstFileName) != 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: Refusing to open an output file which will overwrite the input file \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return NULL as *mut FILE;
    }
    isDstRegFile = UTIL_isRegularFile(dstFileName);
    if (*prefs).sparseFileSupport == 1 as libc::c_int {
        (*prefs).sparseFileSupport = ZSTD_SPARSE_DEFAULT;
        if isDstRegFile == 0 {
            (*prefs).sparseFileSupport = 0 as libc::c_int;
            if g_display_prefs.displayLevel >= 4 as libc::c_int {
                fprintf(
                    stderr,
                    b"Sparse File Support is disabled when output is not a file \n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if isDstRegFile != 0 {
        if strcmp(dstFileName, nulmark.as_ptr()) == 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    621 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    40 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s is unexpectedly categorized as a regular file\0" as *const u8
                        as *const libc::c_char,
                    dstFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(40 as libc::c_int);
        }
        if (*prefs).overwrite == 0 {
            if g_display_prefs.displayLevel <= 1 as libc::c_int {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s already exists; not overwritten  \n\0" as *const u8
                            as *const libc::c_char,
                        dstFileName,
                    );
                }
                return NULL as *mut FILE;
            }
            fprintf(
                stderr,
                b"zstd: %s already exists; \0" as *const u8 as *const libc::c_char,
                dstFileName,
            );
            if UTIL_requireUserConfirmation(
                b"overwrite (y/n) ? \0" as *const u8 as *const libc::c_char,
                b"Not overwritten  \n\0" as *const u8 as *const libc::c_char,
                b"yY\0" as *const u8 as *const libc::c_char,
                (*fCtx).hasStdinInput,
            ) != 0
            {
                return NULL as *mut FILE;
            }
        }
        FIO_removeFile(dstFileName);
    }
    let openflags = O_WRONLY | O_CREAT | O_TRUNC;
    let fd = open(dstFileName, openflags, mode);
    let mut f = NULL as *mut FILE;
    if fd != -(1 as libc::c_int) {
        f = fdopen(fd, b"wb\0" as *const u8 as *const libc::c_char);
    }
    if f.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s: %s\n\0" as *const u8 as *const libc::c_char,
                dstFileName,
                strerror(*__errno_location()),
            );
        }
    } else if setvbuf(
        f,
        NULL as *mut libc::c_char,
        _IOFBF,
        (1 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as size_t,
    ) != 0
    {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Warning: setvbuf failed for %s\n\0" as *const u8
                    as *const libc::c_char,
                dstFileName,
            );
        }
    }
    return f;
}
unsafe extern "C" fn FIO_getDictFileStat(
    mut fileName: *const libc::c_char,
    mut dictFileStat: *mut stat_t,
) {
    if !dictFileStat.is_null() {} else {
        __assert_fail(
            b"dictFileStat != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            685 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void FIO_getDictFileStat(const char *, stat_t *)\0"))
                .as_ptr(),
        );
    }
    if fileName.is_null() {
        return;
    }
    if UTIL_stat(fileName, dictFileStat) == 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                689 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Stat failed on dictionary file %s: %s\0" as *const u8
                    as *const libc::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(31 as libc::c_int);
    }
    if UTIL_isRegularFileStat(dictFileStat) == 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                693 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                32 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Dictionary %s must be a regular file.\0" as *const u8
                    as *const libc::c_char,
                fileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(32 as libc::c_int);
    }
}
unsafe extern "C" fn FIO_setDictBufferMalloc(
    mut dict: *mut FIO_Dict_t,
    mut fileName: *const libc::c_char,
    prefs: *mut FIO_prefs_t,
    mut dictFileStat: *mut stat_t,
) -> size_t {
    let mut fileHandle = 0 as *mut FILE;
    let mut fileSize: U64 = 0;
    let mut bufferPtr: *mut *mut libc::c_void = &mut (*dict).dictBuffer;
    if !bufferPtr.is_null() {} else {
        __assert_fail(
            b"bufferPtr != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            709 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"size_t FIO_setDictBufferMalloc(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !dictFileStat.is_null() {} else {
        __assert_fail(
            b"dictFileStat != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            710 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"size_t FIO_setDictBufferMalloc(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    *bufferPtr = NULL as *mut libc::c_void;
    if fileName.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if g_display_prefs.displayLevel >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"Loading %s as dictionary \n\0" as *const u8 as *const libc::c_char,
            fileName,
        );
    }
    fileHandle = fopen(fileName, b"rb\0" as *const u8 as *const libc::c_char);
    if fileHandle.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                719 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                33 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Couldn't open dictionary %s: %s\0" as *const u8 as *const libc::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(33 as libc::c_int);
    }
    fileSize = UTIL_getFileSizeStat(dictFileStat);
    let dictSizeMax = (if (*prefs).patchFromMode != 0 {
        (*prefs).memLimit
    } else {
        DICTSIZE_MAX as libc::c_uint
    }) as size_t;
    if fileSize > dictSizeMax {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                727 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                34 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Dictionary file %s is too large (> %u bytes)\0" as *const u8
                    as *const libc::c_char,
                fileName,
                dictSizeMax as libc::c_uint,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(34 as libc::c_int);
    }
    *bufferPtr = malloc(fileSize);
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                731 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                34 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(34 as libc::c_int);
    }
    let readSize = fread(
        *bufferPtr,
        1 as libc::c_int as libc::c_ulong,
        fileSize,
        fileHandle,
    );
    if readSize != fileSize {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                735 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                35 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error reading dictionary file %s : %s\0" as *const u8
                    as *const libc::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(35 as libc::c_int);
    }
    fclose(fileHandle);
    return fileSize;
}
pub const PROT_READ: libc::c_int = 0x1 as libc::c_int;
pub const MAP_PRIVATE: libc::c_int = 0x2 as libc::c_int;
unsafe extern "C" fn FIO_munmap(mut dict: *mut FIO_Dict_t) {
    munmap((*dict).dictBuffer, (*dict).dictBufferSize);
    (*dict).dictBuffer = NULL as *mut libc::c_void;
    (*dict).dictBufferSize = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FIO_setDictBufferMMap(
    mut dict: *mut FIO_Dict_t,
    mut fileName: *const libc::c_char,
    prefs: *mut FIO_prefs_t,
    mut dictFileStat: *mut stat_t,
) -> size_t {
    let mut fileHandle: libc::c_int = 0;
    let mut fileSize: U64 = 0;
    let mut bufferPtr: *mut *mut libc::c_void = &mut (*dict).dictBuffer;
    if !bufferPtr.is_null() {} else {
        __assert_fail(
            b"bufferPtr != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            756 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"size_t FIO_setDictBufferMMap(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !dictFileStat.is_null() {} else {
        __assert_fail(
            b"dictFileStat != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            757 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"size_t FIO_setDictBufferMMap(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    *bufferPtr = NULL as *mut libc::c_void;
    if fileName.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if g_display_prefs.displayLevel >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"Loading %s as dictionary \n\0" as *const u8 as *const libc::c_char,
            fileName,
        );
    }
    fileHandle = open(fileName, O_RDONLY);
    if fileHandle == -(1 as libc::c_int) {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                766 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                33 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Couldn't open dictionary %s: %s\0" as *const u8 as *const libc::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(33 as libc::c_int);
    }
    fileSize = UTIL_getFileSizeStat(dictFileStat);
    let dictSizeMax = (if (*prefs).patchFromMode != 0 {
        (*prefs).memLimit
    } else {
        DICTSIZE_MAX as libc::c_uint
    }) as size_t;
    if fileSize > dictSizeMax {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                774 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                34 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Dictionary file %s is too large (> %u bytes)\0" as *const u8
                    as *const libc::c_char,
                fileName,
                dictSizeMax as libc::c_uint,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(34 as libc::c_int);
    }
    *bufferPtr = mmap(
        NULL as *mut libc::c_void,
        fileSize,
        PROT_READ,
        MAP_PRIVATE,
        fileHandle,
        0 as libc::c_int as __off_t,
    );
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                779 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                34 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(34 as libc::c_int);
    }
    close(fileHandle);
    return fileSize;
}
unsafe extern "C" fn FIO_freeDict(mut dict: *mut FIO_Dict_t) {
    if (*dict).dictBufferType as libc::c_uint
        == FIO_mallocDict as libc::c_int as libc::c_uint
    {
        free((*dict).dictBuffer);
        (*dict).dictBuffer = NULL as *mut libc::c_void;
        (*dict).dictBufferSize = 0 as libc::c_int as size_t;
    } else if (*dict).dictBufferType as libc::c_uint
        == FIO_mmapDict as libc::c_int as libc::c_uint
    {
        FIO_munmap(dict);
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            852 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void FIO_freeDict(FIO_Dict_t *)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn FIO_initDict(
    mut dict: *mut FIO_Dict_t,
    mut fileName: *const libc::c_char,
    prefs: *mut FIO_prefs_t,
    mut dictFileStat: *mut stat_t,
    mut dictBufferType: FIO_dictBufferType_t,
) {
    (*dict).dictBufferType = dictBufferType;
    if (*dict).dictBufferType as libc::c_uint
        == FIO_mallocDict as libc::c_int as libc::c_uint
    {
        (*dict)
            .dictBufferSize = FIO_setDictBufferMalloc(
            dict,
            fileName,
            prefs,
            dictFileStat,
        );
    } else if (*dict).dictBufferType as libc::c_uint
        == FIO_mmapDict as libc::c_int as libc::c_uint
    {
        (*dict)
            .dictBufferSize = FIO_setDictBufferMMap(dict, fileName, prefs, dictFileStat);
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            863 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void FIO_initDict(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *, FIO_dictBufferType_t)\0",
            ))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn FIO_checkFilenameCollisions(
    mut filenameTable: *mut *const libc::c_char,
    mut nbFiles: libc::c_uint,
) -> libc::c_int {
    let mut filenameTableSorted = 0 as *mut *const libc::c_char;
    let mut prevElem = 0 as *const libc::c_char;
    let mut filename = 0 as *const libc::c_char;
    let mut u: libc::c_uint = 0;
    filenameTableSorted = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nbFiles as libc::c_ulong),
    ) as *mut *const libc::c_char;
    if filenameTableSorted.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Allocation error during filename collision checking \n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    u = 0 as libc::c_int as libc::c_uint;
    while u < nbFiles {
        filename = strrchr(*filenameTable.offset(u as isize), PATH_SEP);
        if filename.is_null() {
            let ref mut fresh0 = *filenameTableSorted.offset(u as isize);
            *fresh0 = *filenameTable.offset(u as isize);
        } else {
            let ref mut fresh1 = *filenameTableSorted.offset(u as isize);
            *fresh1 = filename.offset(1 as libc::c_int as isize);
        }
        u = u.wrapping_add(1);
    }
    qsort(
        filenameTableSorted as *mut libc::c_void,
        nbFiles as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            UTIL_compareStr
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    prevElem = *filenameTableSorted.offset(0 as libc::c_int as isize);
    u = 1 as libc::c_int as libc::c_uint;
    while u < nbFiles {
        if strcmp(prevElem, *filenameTableSorted.offset(u as isize)) == 0 as libc::c_int
        {
            if g_display_prefs.displayLevel >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"WARNING: Two files have same filename: %s\n\0" as *const u8
                        as *const libc::c_char,
                    prevElem,
                );
            }
        }
        prevElem = *filenameTableSorted.offset(u as isize);
        u = u.wrapping_add(1);
    }
    free(filenameTableSorted as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn extractFilename(
    mut path: *const libc::c_char,
    mut separator: libc::c_char,
) -> *const libc::c_char {
    let mut search: *const libc::c_char = strrchr(path, separator as libc::c_int);
    if search.is_null() {
        return path;
    }
    return search.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn FIO_createFilename_fromOutDir(
    mut path: *const libc::c_char,
    mut outDirName: *const libc::c_char,
    suffixLen: size_t,
) -> *mut libc::c_char {
    let mut filenameStart = 0 as *const libc::c_char;
    let mut separator: libc::c_char = 0;
    let mut result = 0 as *mut libc::c_char;
    separator = '/' as i32 as libc::c_char;
    filenameStart = extractFilename(path, separator);
    result = calloc(
        1 as libc::c_int as libc::c_ulong,
        (strlen(outDirName))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(filenameStart))
            .wrapping_add(suffixLen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if result.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                936 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                30 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: FIO_createFilename_fromOutDir: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(30 as libc::c_int);
    }
    memcpy(
        result as *mut libc::c_void,
        outDirName as *const libc::c_void,
        strlen(outDirName),
    );
    if *outDirName
        .offset(
            (strlen(outDirName)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == separator as libc::c_int
    {
        memcpy(
            result.offset(strlen(outDirName) as isize) as *mut libc::c_void,
            filenameStart as *const libc::c_void,
            strlen(filenameStart),
        );
    } else {
        memcpy(
            result.offset(strlen(outDirName) as isize) as *mut libc::c_void,
            &mut separator as *mut libc::c_char as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            result.offset(strlen(outDirName) as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            filenameStart as *const libc::c_void,
            strlen(filenameStart),
        );
    }
    return result;
}
unsafe extern "C" fn FIO_highbit64(mut v: libc::c_ulonglong) -> libc::c_uint {
    let mut count = 0 as libc::c_int as libc::c_uint;
    if v != 0 as libc::c_int as libc::c_ulonglong {} else {
        __assert_fail(
            b"v != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            957 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"unsigned int FIO_highbit64(unsigned long long)\0"))
                .as_ptr(),
        );
    }
    v >>= 1 as libc::c_int;
    while v != 0 {
        v >>= 1 as libc::c_int;
        count = count.wrapping_add(1);
    }
    return count;
}
unsafe extern "C" fn FIO_adjustMemLimitForPatchFromMode(
    prefs: *mut FIO_prefs_t,
    dictSize: libc::c_ulonglong,
    maxSrcFileSize: libc::c_ulonglong,
) {
    let mut maxSize = if (*prefs).memLimit as libc::c_ulonglong
        > (if dictSize > maxSrcFileSize { dictSize } else { maxSrcFileSize })
    {
        (*prefs).memLimit as libc::c_ulonglong
    } else if dictSize > maxSrcFileSize {
        dictSize
    } else {
        maxSrcFileSize
    };
    let maxWindowSize = (1 as libc::c_uint)
        << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        });
    if maxSize == UTIL_FILESIZE_UNKNOWN as libc::c_ulonglong {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                970 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                42 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Using --patch-from with stdin requires --stream-size\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(42 as libc::c_int);
    }
    if maxSize != -(1 as libc::c_int) as U64 as libc::c_ulonglong {} else {
        __assert_fail(
            b"maxSize != UTIL_FILESIZE_UNKNOWN\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            971 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"void FIO_adjustMemLimitForPatchFromMode(FIO_prefs_t *const, const unsigned long long, const unsigned long long)\0",
            ))
                .as_ptr(),
        );
    }
    if maxSize > maxWindowSize as libc::c_ulonglong {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                973 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                42 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Can't handle files larger than %u GB\n\0" as *const u8
                    as *const libc::c_char,
                maxWindowSize
                    .wrapping_div(
                        (1 as libc::c_int as libc::c_uint)
                            .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int),
                    ),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(42 as libc::c_int);
    }
    FIO_setMemLimit(prefs, maxSize as libc::c_uint);
}
unsafe extern "C" fn FIO_multiFilesConcatWarning(
    mut fCtx: *const FIO_ctx_t,
    mut prefs: *mut FIO_prefs_t,
    mut outFileName: *const libc::c_char,
    mut displayLevelCutoff: libc::c_int,
) -> libc::c_int {
    if (*fCtx).hasStdoutOutput != 0 {
        if (*prefs).removeSrcFile != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1000 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    43 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"It's not allowed to remove input files when processed output is piped to stdout. This scenario is not supposed to be possible. This is a programming error. File an issue for it to be fixed.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(43 as libc::c_int);
        }
    }
    if (*prefs).testMode != 0 {
        if (*prefs).removeSrcFile != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1008 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    43 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Test mode shall not remove input files! This scenario is not supposed to be possible. This is a programming error. File an issue for it to be fixed.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(43 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if (*fCtx).nbFilesTotal == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*fCtx).nbFilesTotal > 1 as libc::c_int {} else {
        __assert_fail(
            b"fCtx->nbFilesTotal > 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            1013 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"int FIO_multiFilesConcatWarning(const FIO_ctx_t *, FIO_prefs_t *, const char *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if outFileName.is_null() {
        return 0 as libc::c_int;
    }
    if (*fCtx).hasStdoutOutput != 0 {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: WARNING: all input files will be processed and concatenated into stdout. \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if g_display_prefs.displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"zstd: WARNING: all input files will be processed and concatenated into a single output file: %s \n\0"
                as *const u8 as *const libc::c_char,
            outFileName,
        );
    }
    if g_display_prefs.displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"The concatenated output CANNOT regenerate original file names nor directory structure. \n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if (*prefs).removeSrcFile != 0 {
        if g_display_prefs.displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Since it's a destructive operation, input files will not be removed. \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        (*prefs).removeSrcFile = 0 as libc::c_int;
    }
    if (*fCtx).hasStdoutOutput != 0 {
        return 0 as libc::c_int;
    }
    if (*prefs).overwrite != 0 {
        return 0 as libc::c_int;
    }
    if g_display_prefs.displayLevel <= displayLevelCutoff {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Concatenating multiple processed inputs into a single output loses file metadata. \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"Aborting. \n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int;
    }
    return UTIL_requireUserConfirmation(
        b"Proceed? (y/n): \0" as *const u8 as *const libc::c_char,
        b"Aborting...\0" as *const u8 as *const libc::c_char,
        b"yY\0" as *const u8 as *const libc::c_char,
        (*fCtx).hasStdinInput,
    );
}
unsafe extern "C" fn setInBuffer(
    mut buf: *const libc::c_void,
    mut s: size_t,
    mut pos: size_t,
) -> ZSTD_inBuffer {
    let mut i = ZSTD_inBuffer {
        src: 0 as *const libc::c_void,
        size: 0,
        pos: 0,
    };
    i.src = buf;
    i.size = s;
    i.pos = pos;
    return i;
}
unsafe extern "C" fn setOutBuffer(
    mut buf: *mut libc::c_void,
    mut s: size_t,
    mut pos: size_t,
) -> ZSTD_outBuffer {
    let mut o = ZSTD_outBuffer {
        dst: 0 as *mut libc::c_void,
        size: 0,
        pos: 0,
    };
    o.dst = buf;
    o.size = s;
    o.pos = pos;
    return o;
}
unsafe extern "C" fn ZSTD_cycleLog(mut hashLog: U32, mut strat: ZSTD_strategy) -> U32 {
    let btScale = (strat as U32 >= ZSTD_btlazy2 as libc::c_int as U32) as libc::c_int
        as U32;
    if hashLog > 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"hashLog > 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            1081 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"U32 ZSTD_cycleLog(U32, ZSTD_strategy)\0"))
                .as_ptr(),
        );
    }
    return hashLog.wrapping_sub(btScale);
}
unsafe extern "C" fn FIO_adjustParamsForPatchFromMode(
    prefs: *mut FIO_prefs_t,
    mut comprParams: *mut ZSTD_compressionParameters,
    dictSize: libc::c_ulonglong,
    maxSrcFileSize: libc::c_ulonglong,
    mut cLevel: libc::c_int,
) {
    let fileWindowLog = (FIO_highbit64(maxSrcFileSize))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let cParams = ZSTD_getCParams(
        cLevel,
        maxSrcFileSize as size_t as libc::c_ulonglong,
        dictSize as size_t,
    );
    FIO_adjustMemLimitForPatchFromMode(prefs, dictSize, maxSrcFileSize);
    if fileWindowLog
        > (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as libc::c_uint
    {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Max window log exceeded by file (compression ratio will suffer)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    (*comprParams)
        .windowLog = if 10 as libc::c_int as libc::c_uint
        > (if ((if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            30 as libc::c_int
        } else {
            31 as libc::c_int
        }) as libc::c_uint) < fileWindowLog
        {
            (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            }) as libc::c_uint
        } else {
            fileWindowLog
        })
    {
        10 as libc::c_int as libc::c_uint
    } else if ((if ::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
    {
        30 as libc::c_int
    } else {
        31 as libc::c_int
    }) as libc::c_uint) < fileWindowLog
    {
        (if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            30 as libc::c_int
        } else {
            31 as libc::c_int
        }) as libc::c_uint
    } else {
        fileWindowLog
    };
    if fileWindowLog > ZSTD_cycleLog(cParams.chainLog, cParams.strategy) {
        if (*prefs).ldmFlag == 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"long mode automatically triggered\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        FIO_setLdmFlag(prefs, 1 as libc::c_int as libc::c_uint);
    }
    if cParams.strategy as libc::c_uint >= ZSTD_btopt as libc::c_int as libc::c_uint {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[Optimal parser notes] Consider the following to improve patch size at the cost of speed:\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"- Use --single-thread mode in the zstd cli\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"- Set a larger targetLength (e.g. --zstd=targetLength=4096)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"- Set a larger chainLog (e.g. --zstd=chainLog=%u)\n\0" as *const u8
                    as *const libc::c_char,
                if ::core::mem::size_of::<size_t>() as libc::c_ulong
                    == 4 as libc::c_int as libc::c_ulong
                {
                    29 as libc::c_int
                } else {
                    30 as libc::c_int
                },
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Also consider playing around with searchLog and hashLog\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn FIO_createCResources(
    prefs: *mut FIO_prefs_t,
    mut dictFileName: *const libc::c_char,
    maxSrcFileSize: libc::c_ulonglong,
    mut cLevel: libc::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> cRess_t {
    let mut useMMap = ((*prefs).mmapDict as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint) as libc::c_int;
    let mut forceNoUseMMap = ((*prefs).mmapDict as libc::c_uint
        == ZSTD_ps_disable as libc::c_int as libc::c_uint) as libc::c_int;
    let mut dictBufferType = FIO_mallocDict;
    let mut ress = cRess_t {
        dict: FIO_Dict_t {
            dictBuffer: 0 as *mut libc::c_void,
            dictBufferSize: 0,
            dictBufferType: FIO_mallocDict,
        },
        dictFileName: 0 as *const libc::c_char,
        dictFileStat: stat_t {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        },
        cctx: 0 as *mut ZSTD_CStream,
        writeCtx: 0 as *mut WritePoolCtx_t,
        readCtx: 0 as *mut ReadPoolCtx_t,
    };
    memset(
        &mut ress as *mut cRess_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cRess_t>() as libc::c_ulong,
    );
    if g_display_prefs.displayLevel >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"FIO_createCResources \n\0" as *const u8 as *const libc::c_char,
        );
    }
    ress.cctx = ZSTD_createCCtx();
    if (ress.cctx).is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1124 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                30 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"allocation error (%s): can't create ZSTD_CCtx\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(30 as libc::c_int);
    }
    FIO_getDictFileStat(dictFileName, &mut ress.dictFileStat);
    if (*prefs).patchFromMode != 0 {
        let dictSize = UTIL_getFileSizeStat(&mut ress.dictFileStat);
        let ssSize = (*prefs).streamSrcSize as libc::c_ulonglong;
        useMMap |= (dictSize > (*prefs).memLimit as libc::c_ulong) as libc::c_int;
        FIO_adjustParamsForPatchFromMode(
            prefs,
            &mut comprParams,
            dictSize as libc::c_ulonglong,
            if ssSize > 0 as libc::c_int as libc::c_ulonglong {
                ssSize
            } else {
                maxSrcFileSize
            },
            cLevel,
        );
    }
    dictBufferType = (if useMMap != 0 && forceNoUseMMap == 0 {
        FIO_mmapDict as libc::c_int
    } else {
        FIO_mallocDict as libc::c_int
    }) as FIO_dictBufferType_t;
    FIO_initDict(
        &mut ress.dict,
        dictFileName,
        prefs,
        &mut ress.dictFileStat,
        dictBufferType,
    );
    ress.writeCtx = AIO_WritePool_create(prefs, ZSTD_CStreamOutSize());
    ress.readCtx = AIO_ReadPool_create(prefs, ZSTD_CStreamInSize());
    if !dictFileName.is_null() && (ress.dict.dictBuffer).is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1145 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                32 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"allocation error : can't create dictBuffer\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(32 as libc::c_int);
    }
    ress.dictFileName = dictFileName;
    if (*prefs).adaptiveMode != 0 && (*prefs).ldmFlag == 0 && comprParams.windowLog == 0
    {
        comprParams.windowLog = ADAPT_WINDOWLOG_DEFAULT as libc::c_uint;
    }
    let mut err: size_t = 0;
    err = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_contentSizeFlag,
        (*prefs).contentSize,
    );
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_contentSizeFlag, prefs->contentSize)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1151 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_0: size_t = 0;
    err_0 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_dictIDFlag, (*prefs).dictIDFlag);
    if ZSTD_isError(err_0) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_dictIDFlag, prefs->dictIDFlag)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1152 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_0),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_1: size_t = 0;
    err_1 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_checksumFlag,
        (*prefs).checksumFlag,
    );
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_checksumFlag, prefs->checksumFlag)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1153 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_2: size_t = 0;
    err_2 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_compressionLevel, cLevel);
    if ZSTD_isError(err_2) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_compressionLevel, cLevel)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1155 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_2),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_3: size_t = 0;
    err_3 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam6,
        (*prefs).targetCBlockSize as libc::c_int,
    );
    if ZSTD_isError(err_3) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam6, (int)prefs->targetCBlockSize)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1157 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_3),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_4: size_t = 0;
    err_4 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam7,
        (*prefs).srcSizeHint,
    );
    if ZSTD_isError(err_4) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam7, (int)prefs->srcSizeHint)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1159 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_4),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_5: size_t = 0;
    err_5 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_enableLongDistanceMatching,
        (*prefs).ldmFlag,
    );
    if ZSTD_isError(err_5) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_enableLongDistanceMatching, prefs->ldmFlag)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1161 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_5),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_6: size_t = 0;
    err_6 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashLog, (*prefs).ldmHashLog);
    if ZSTD_isError(err_6) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashLog, prefs->ldmHashLog)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1162 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_6),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_7: size_t = 0;
    err_7 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmMinMatch, (*prefs).ldmMinMatch);
    if ZSTD_isError(err_7) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmMinMatch, prefs->ldmMinMatch)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1163 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_7),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    if (*prefs).ldmBucketSizeLog != FIO_LDM_PARAM_NOTSET {
        let mut err_8: size_t = 0;
        err_8 = ZSTD_CCtx_setParameter(
            ress.cctx,
            ZSTD_c_ldmBucketSizeLog,
            (*prefs).ldmBucketSizeLog,
        );
        if ZSTD_isError(err_8) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmBucketSizeLog, prefs->ldmBucketSizeLog)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1165 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_8),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    }
    if (*prefs).ldmHashRateLog != FIO_LDM_PARAM_NOTSET {
        let mut err_9: size_t = 0;
        err_9 = ZSTD_CCtx_setParameter(
            ress.cctx,
            ZSTD_c_ldmHashRateLog,
            (*prefs).ldmHashRateLog,
        );
        if ZSTD_isError(err_9) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashRateLog, prefs->ldmHashRateLog)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1168 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_9),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    }
    let mut err_10: size_t = 0;
    err_10 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam14,
        (*prefs).useRowMatchFinder,
    );
    if ZSTD_isError(err_10) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam14, prefs->useRowMatchFinder)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1170 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_10),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_11: size_t = 0;
    err_11 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_windowLog,
        comprParams.windowLog as libc::c_int,
    );
    if ZSTD_isError(err_11) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_windowLog, (int)comprParams.windowLog)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1172 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_11),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_12: size_t = 0;
    err_12 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_chainLog,
        comprParams.chainLog as libc::c_int,
    );
    if ZSTD_isError(err_12) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_chainLog, (int)comprParams.chainLog)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1173 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_12),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_13: size_t = 0;
    err_13 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_hashLog,
        comprParams.hashLog as libc::c_int,
    );
    if ZSTD_isError(err_13) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_hashLog, (int)comprParams.hashLog)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1174 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_13),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_14: size_t = 0;
    err_14 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_searchLog,
        comprParams.searchLog as libc::c_int,
    );
    if ZSTD_isError(err_14) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_searchLog, (int)comprParams.searchLog)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1175 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_14),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_15: size_t = 0;
    err_15 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_minMatch,
        comprParams.minMatch as libc::c_int,
    );
    if ZSTD_isError(err_15) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_minMatch, (int)comprParams.minMatch)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1176 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_15),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_16: size_t = 0;
    err_16 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_targetLength,
        comprParams.targetLength as libc::c_int,
    );
    if ZSTD_isError(err_16) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_targetLength, (int)comprParams.targetLength)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1177 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_16),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_17: size_t = 0;
    err_17 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_strategy,
        comprParams.strategy as libc::c_int,
    );
    if ZSTD_isError(err_17) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_strategy, (int)comprParams.strategy)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1178 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_17),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_18: size_t = 0;
    err_18 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam5,
        (*prefs).literalCompressionMode as libc::c_int,
    );
    if ZSTD_isError(err_18) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam5, (int)prefs->literalCompressionMode)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1179 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_18),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_19: size_t = 0;
    err_19 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam8,
        1 as libc::c_int,
    );
    if ZSTD_isError(err_19) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam8, 1)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1180 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_19),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    if (*prefs).patchFromMode != 0 {
        let mut err_20: size_t = 0;
        err_20 = ZSTD_CCtx_refPrefix(
            ress.cctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_20) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_refPrefix(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1194 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_20),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    } else {
        let mut err_21: size_t = 0;
        err_21 = ZSTD_CCtx_loadDictionary_byReference(
            ress.cctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_21) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_loadDictionary_byReference(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1196 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_21),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    }
    return ress;
}
unsafe extern "C" fn FIO_freeCResources(ress: *mut cRess_t) {
    FIO_freeDict(&mut (*ress).dict);
    AIO_WritePool_free((*ress).writeCtx);
    AIO_ReadPool_free((*ress).readCtx);
    ZSTD_freeCStream((*ress).cctx);
}
unsafe extern "C" fn FIO_compressZstdFrame(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ressPtr: *const cRess_t,
    mut srcFileName: *const libc::c_char,
    mut fileSize: U64,
    mut compressionLevel: libc::c_int,
    mut readsize: *mut U64,
) -> libc::c_ulonglong {
    let ress = *ressPtr;
    let mut writeJob = AIO_WritePool_acquireJob((*ressPtr).writeCtx);
    let mut compressedfilesize = 0 as libc::c_int as U64;
    let mut directive = ZSTD_e_continue;
    let mut pledgedSrcSize = ZSTD_CONTENTSIZE_UNKNOWN as U64;
    let mut previous_zfp_update = {
        let mut init = ZSTD_frameProgression {
            ingested: 0 as libc::c_int as libc::c_ulonglong,
            consumed: 0 as libc::c_int as libc::c_ulonglong,
            produced: 0 as libc::c_int as libc::c_ulonglong,
            flushed: 0 as libc::c_int as libc::c_ulonglong,
            currentJobID: 0 as libc::c_int as libc::c_uint,
            nbActiveWorkers: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    let mut previous_zfp_correction = {
        let mut init = ZSTD_frameProgression {
            ingested: 0 as libc::c_int as libc::c_ulonglong,
            consumed: 0 as libc::c_int as libc::c_ulonglong,
            produced: 0 as libc::c_int as libc::c_ulonglong,
            flushed: 0 as libc::c_int as libc::c_ulonglong,
            currentJobID: 0 as libc::c_int as libc::c_uint,
            nbActiveWorkers: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    let mut speedChange = noChange;
    let mut flushWaiting = 0 as libc::c_int as libc::c_uint;
    let mut inputPresented = 0 as libc::c_int as libc::c_uint;
    let mut inputBlocked = 0 as libc::c_int as libc::c_uint;
    let mut lastJobID = 0 as libc::c_int as libc::c_uint;
    let mut lastAdaptTime = UTIL_getTime();
    let adaptEveryMicro = REFRESH_RATE;
    let file_hrs = UTIL_makeHumanReadableSize(fileSize);
    if g_display_prefs.displayLevel >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"compression using zstd format \n\0" as *const u8 as *const libc::c_char,
        );
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64 {
        pledgedSrcSize = fileSize;
        let mut err: size_t = 0;
        err = ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize as libc::c_ulonglong);
        if ZSTD_isError(err) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize)\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1522 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    } else if (*prefs).streamSrcSize > 0 as libc::c_int as libc::c_ulong {
        pledgedSrcSize = (*prefs).streamSrcSize;
        let mut err_0: size_t = 0;
        err_0 = ZSTD_CCtx_setPledgedSrcSize(
            ress.cctx,
            (*prefs).streamSrcSize as libc::c_ulonglong,
        );
        if ZSTD_isError(err_0) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, prefs->streamSrcSize)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1526 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_0),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    }
    let mut windowLog: libc::c_int = 0;
    let mut windowSize = UTIL_HumanReadableSize_t {
        value: 0.,
        precision: 0,
        suffix: 0 as *const libc::c_char,
    };
    let mut err_1: size_t = 0;
    err_1 = ZSTD_CCtx_getParameter(ress.cctx, ZSTD_c_windowLog, &mut windowLog);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_getParameter(ress.cctx, ZSTD_c_windowLog, &windowLog)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1532 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    if windowLog == 0 as libc::c_int {
        if (*prefs).ldmFlag != 0 {
            windowLog = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
        } else {
            let cParams = ZSTD_getCParams(
                compressionLevel,
                fileSize as libc::c_ulonglong,
                0 as libc::c_int as size_t,
            );
            windowLog = cParams.windowLog as libc::c_int;
        }
    }
    windowSize = UTIL_makeHumanReadableSize(
        (if 1 as libc::c_ulonglong
            > (if (1 as libc::c_ulonglong) << windowLog
                < pledgedSrcSize as libc::c_ulonglong
            {
                (1 as libc::c_ulonglong) << windowLog
            } else {
                pledgedSrcSize as libc::c_ulonglong
            })
        {
            1 as libc::c_ulonglong
        } else if (1 as libc::c_ulonglong) << windowLog
            < pledgedSrcSize as libc::c_ulonglong
        {
            (1 as libc::c_ulonglong) << windowLog
        } else {
            pledgedSrcSize as libc::c_ulonglong
        }) as U64,
    );
    if g_display_prefs.displayLevel >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"Decompression will require %.*f%s of memory\n\0" as *const u8
                as *const libc::c_char,
            windowSize.precision,
            windowSize.value,
            windowSize.suffix,
        );
    }
    loop {
        let mut stillToFlush: size_t = 0;
        let inSize = AIO_ReadPool_fillBuffer(ress.readCtx, ZSTD_CStreamInSize());
        let mut inBuff = setInBuffer(
            (*ress.readCtx).srcBuffer as *const libc::c_void,
            (*ress.readCtx).srcBufferLoaded,
            0 as libc::c_int as size_t,
        );
        if g_display_prefs.displayLevel >= 6 as libc::c_int {
            fprintf(
                stderr,
                b"fread %u bytes from source \n\0" as *const u8 as *const libc::c_char,
                inSize as libc::c_uint,
            );
        }
        *readsize = (*readsize as libc::c_ulong).wrapping_add(inSize) as U64 as U64;
        if (*ress.readCtx).srcBufferLoaded == 0 as libc::c_int as libc::c_ulong
            || *readsize == fileSize
        {
            directive = ZSTD_e_end;
        }
        stillToFlush = 1 as libc::c_int as size_t;
        while inBuff.pos != inBuff.size
            || directive as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
                && stillToFlush != 0 as libc::c_int as libc::c_ulong
        {
            let oldIPos = inBuff.pos;
            let mut outBuff = setOutBuffer(
                (*writeJob).buffer,
                (*writeJob).bufferSize,
                0 as libc::c_int as size_t,
            );
            let toFlushNow = ZSTD_toFlushNow(ress.cctx);
            stillToFlush = ZSTD_compressStream2(
                ress.cctx,
                &mut outBuff,
                &mut inBuff,
                directive,
            );
            if ZSTD_isError(stillToFlush) != 0 {
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s \n\0" as *const u8 as *const libc::c_char,
                        b"ZSTD_compressStream2(ress.cctx, &outBuff, &inBuff, directive)\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                            as *const libc::c_char,
                        1566 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        ZSTD_getErrorName(stillToFlush),
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(11 as libc::c_int);
            }
            AIO_ReadPool_consumeBytes(ress.readCtx, (inBuff.pos).wrapping_sub(oldIPos));
            inputPresented = inputPresented.wrapping_add(1);
            if oldIPos == inBuff.pos {
                inputBlocked = inputBlocked.wrapping_add(1);
            }
            if toFlushNow == 0 {
                flushWaiting = 1 as libc::c_int as libc::c_uint;
            }
            if g_display_prefs.displayLevel >= 6 as libc::c_int {
                fprintf(
                    stderr,
                    b"ZSTD_compress_generic(end:%u) => input pos(%u)<=(%u)size ; output generated %u bytes \n\0"
                        as *const u8 as *const libc::c_char,
                    directive as libc::c_uint,
                    inBuff.pos as libc::c_uint,
                    inBuff.size as libc::c_uint,
                    outBuff.pos as libc::c_uint,
                );
            }
            if outBuff.pos != 0 {
                (*writeJob).usedBufferSize = outBuff.pos;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                compressedfilesize = (compressedfilesize as libc::c_ulong)
                    .wrapping_add(outBuff.pos) as U64 as U64;
            }
            if (*prefs).adaptiveMode != 0
                && UTIL_clockSpanMicro(lastAdaptTime) > adaptEveryMicro
            {
                let zfp = ZSTD_getFrameProgression(ress.cctx);
                lastAdaptTime = UTIL_getTime();
                if zfp.currentJobID > 1 as libc::c_int as libc::c_uint {
                    let mut newlyProduced = (zfp.produced)
                        .wrapping_sub(previous_zfp_update.produced);
                    let mut newlyFlushed = (zfp.flushed)
                        .wrapping_sub(previous_zfp_update.flushed);
                    if zfp.produced >= previous_zfp_update.produced {} else {
                        __assert_fail(
                            b"zfp.produced >= previous_zfp_update.produced\0"
                                as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0"
                                as *const u8 as *const libc::c_char,
                            1594 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 127],
                                &[libc::c_char; 127],
                            >(
                                b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if (*prefs).nbWorkers >= 1 as libc::c_int {} else {
                        __assert_fail(
                            b"prefs->nbWorkers >= 1\0" as *const u8
                                as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0"
                                as *const u8 as *const libc::c_char,
                            1595 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 127],
                                &[libc::c_char; 127],
                            >(
                                b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if zfp.consumed == previous_zfp_update.consumed
                        && zfp.nbActiveWorkers == 0 as libc::c_int as libc::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as libc::c_int {
                            fprintf(
                                stderr,
                                b"all buffers full : compression stopped => slow down \n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        speedChange = slower;
                    }
                    previous_zfp_update = zfp;
                    if newlyProduced
                        > newlyFlushed
                            .wrapping_mul(9 as libc::c_int as libc::c_ulonglong)
                            .wrapping_div(8 as libc::c_int as libc::c_ulonglong)
                        && flushWaiting == 0 as libc::c_int as libc::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as libc::c_int {
                            fprintf(
                                stderr,
                                b"compression faster than flush (%llu > %llu), and flushed was never slowed down by lack of production => slow down \n\0"
                                    as *const u8 as *const libc::c_char,
                                newlyProduced,
                                newlyFlushed,
                            );
                        }
                        speedChange = slower;
                    }
                    flushWaiting = 0 as libc::c_int as libc::c_uint;
                }
                if zfp.currentJobID > lastJobID {
                    if g_display_prefs.displayLevel >= 6 as libc::c_int {
                        fprintf(
                            stderr,
                            b"compression level adaptation check \n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if zfp.currentJobID
                        > ((*prefs).nbWorkers + 1 as libc::c_int) as libc::c_uint
                    {
                        if inputBlocked <= 0 as libc::c_int as libc::c_uint {
                            if g_display_prefs.displayLevel >= 6 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"input is never blocked => input is slower than ingestion \n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            speedChange = slower;
                        } else if speedChange as libc::c_uint
                            == noChange as libc::c_int as libc::c_uint
                        {
                            let mut newlyIngested = (zfp.ingested)
                                .wrapping_sub(previous_zfp_correction.ingested);
                            let mut newlyConsumed = (zfp.consumed)
                                .wrapping_sub(previous_zfp_correction.consumed);
                            let mut newlyProduced_0 = (zfp.produced)
                                .wrapping_sub(previous_zfp_correction.produced);
                            let mut newlyFlushed_0 = (zfp.flushed)
                                .wrapping_sub(previous_zfp_correction.flushed);
                            previous_zfp_correction = zfp;
                            if inputPresented > 0 as libc::c_int as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"inputPresented > 0\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1634 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 127],
                                        &[libc::c_char; 127],
                                    >(
                                        b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if g_display_prefs.displayLevel >= 6 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"input blocked %u/%u(%.2f) - ingested:%u vs %u:consumed - flushed:%u vs %u:produced \n\0"
                                        as *const u8 as *const libc::c_char,
                                    inputBlocked,
                                    inputPresented,
                                    inputBlocked as libc::c_double
                                        / inputPresented as libc::c_double
                                        * 100 as libc::c_int as libc::c_double,
                                    newlyIngested as libc::c_uint,
                                    newlyConsumed as libc::c_uint,
                                    newlyFlushed_0 as libc::c_uint,
                                    newlyProduced_0 as libc::c_uint,
                                );
                            }
                            if inputBlocked
                                > inputPresented
                                    .wrapping_div(8 as libc::c_int as libc::c_uint)
                                && newlyFlushed_0
                                    .wrapping_mul(33 as libc::c_int as libc::c_ulonglong)
                                    .wrapping_div(32 as libc::c_int as libc::c_ulonglong)
                                    > newlyProduced_0
                                && newlyIngested
                                    .wrapping_mul(33 as libc::c_int as libc::c_ulonglong)
                                    .wrapping_div(32 as libc::c_int as libc::c_ulonglong)
                                    > newlyConsumed
                            {
                                if g_display_prefs.displayLevel >= 6 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"recommend faster as in(%llu) >= (%llu)comp(%llu) <= out(%llu) \n\0"
                                            as *const u8 as *const libc::c_char,
                                        newlyIngested,
                                        newlyConsumed,
                                        newlyProduced_0,
                                        newlyFlushed_0,
                                    );
                                }
                                speedChange = faster;
                            }
                        }
                        inputBlocked = 0 as libc::c_int as libc::c_uint;
                        inputPresented = 0 as libc::c_int as libc::c_uint;
                    }
                    if speedChange as libc::c_uint
                        == slower as libc::c_int as libc::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as libc::c_int {
                            fprintf(
                                stderr,
                                b"slower speed , higher compression \n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        compressionLevel += 1;
                        if compressionLevel > ZSTD_maxCLevel() {
                            compressionLevel = ZSTD_maxCLevel();
                        }
                        if compressionLevel > (*prefs).maxAdaptLevel {
                            compressionLevel = (*prefs).maxAdaptLevel;
                        }
                        compressionLevel
                            += (compressionLevel == 0 as libc::c_int) as libc::c_int;
                        ZSTD_CCtx_setParameter(
                            ress.cctx,
                            ZSTD_c_compressionLevel,
                            compressionLevel,
                        );
                    }
                    if speedChange as libc::c_uint
                        == faster as libc::c_int as libc::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as libc::c_int {
                            fprintf(
                                stderr,
                                b"faster speed , lighter compression \n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        compressionLevel -= 1;
                        if compressionLevel < (*prefs).minAdaptLevel {
                            compressionLevel = (*prefs).minAdaptLevel;
                        }
                        compressionLevel
                            -= (compressionLevel == 0 as libc::c_int) as libc::c_int;
                        ZSTD_CCtx_setParameter(
                            ress.cctx,
                            ZSTD_c_compressionLevel,
                            compressionLevel,
                        );
                    }
                    speedChange = noChange;
                    lastJobID = zfp.currentJobID;
                }
            }
            if g_display_prefs.progressSetting as libc::c_uint
                != FIO_ps_never as libc::c_int as libc::c_uint
                && (g_display_prefs.displayLevel >= 2 as libc::c_int
                    || g_display_prefs.progressSetting as libc::c_uint
                        == FIO_ps_always as libc::c_int as libc::c_uint)
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4 as libc::c_int)
            {
                let zfp_0 = ZSTD_getFrameProgression(ress.cctx);
                let cShare = zfp_0.produced as libc::c_double
                    / (zfp_0.consumed)
                        .wrapping_add(
                            (zfp_0.consumed == 0) as libc::c_int as libc::c_ulonglong,
                        ) as libc::c_double * 100 as libc::c_int as libc::c_double;
                let buffered_hrs = UTIL_makeHumanReadableSize(
                    (zfp_0.ingested).wrapping_sub(zfp_0.consumed) as U64,
                );
                let consumed_hrs = UTIL_makeHumanReadableSize(zfp_0.consumed as U64);
                let produced_hrs = UTIL_makeHumanReadableSize(zfp_0.produced as U64);
                g_displayClock = UTIL_getTime();
                if g_display_prefs.progressSetting as libc::c_uint
                    != FIO_ps_never as libc::c_int as libc::c_uint
                    && (g_display_prefs.displayLevel >= 2 as libc::c_int
                        || g_display_prefs.progressSetting as libc::c_uint
                            == FIO_ps_always as libc::c_int as libc::c_uint)
                {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"\r%79s\r\0" as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                if g_display_prefs.displayLevel >= 3 as libc::c_int {
                    if g_display_prefs.progressSetting as libc::c_uint
                        != FIO_ps_never as libc::c_int as libc::c_uint
                        && (g_display_prefs.displayLevel >= 2 as libc::c_int
                            || g_display_prefs.progressSetting as libc::c_uint
                                == FIO_ps_always as libc::c_int as libc::c_uint)
                    {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"(L%i) Buffered:%5.*f%s - Consumed:%5.*f%s - Compressed:%5.*f%s => %.2f%% \0"
                                    as *const u8 as *const libc::c_char,
                                compressionLevel,
                                buffered_hrs.precision,
                                buffered_hrs.value,
                                buffered_hrs.suffix,
                                consumed_hrs.precision,
                                consumed_hrs.value,
                                consumed_hrs.suffix,
                                produced_hrs.precision,
                                produced_hrs.value,
                                produced_hrs.suffix,
                                cShare,
                            );
                        }
                    }
                } else {
                    if (*fCtx).nbFilesTotal > 1 as libc::c_int {
                        let mut srcFileNameSize = strlen(srcFileName);
                        if srcFileNameSize > 18 as libc::c_int as libc::c_ulong {
                            let mut truncatedSrcFileName = srcFileName
                                .offset(srcFileNameSize as isize)
                                .offset(-(15 as libc::c_int as isize));
                            if g_display_prefs.progressSetting as libc::c_uint
                                != FIO_ps_never as libc::c_int as libc::c_uint
                                && (g_display_prefs.displayLevel >= 2 as libc::c_int
                                    || g_display_prefs.progressSetting as libc::c_uint
                                        == FIO_ps_always as libc::c_int as libc::c_uint)
                            {
                                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"Compress: %u/%u files. Current: ...%s \0" as *const u8
                                            as *const libc::c_char,
                                        (*fCtx).currFileIdx + 1 as libc::c_int,
                                        (*fCtx).nbFilesTotal,
                                        truncatedSrcFileName,
                                    );
                                }
                            }
                        } else if g_display_prefs.progressSetting as libc::c_uint
                            != FIO_ps_never as libc::c_int as libc::c_uint
                            && (g_display_prefs.displayLevel >= 2 as libc::c_int
                                || g_display_prefs.progressSetting as libc::c_uint
                                    == FIO_ps_always as libc::c_int as libc::c_uint)
                        {
                            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"Compress: %u/%u files. Current: %*s \0" as *const u8
                                        as *const libc::c_char,
                                    (*fCtx).currFileIdx + 1 as libc::c_int,
                                    (*fCtx).nbFilesTotal,
                                    (18 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(srcFileNameSize) as libc::c_int,
                                    srcFileName,
                                );
                            }
                        }
                    }
                    if g_display_prefs.progressSetting as libc::c_uint
                        != FIO_ps_never as libc::c_int as libc::c_uint
                        && (g_display_prefs.displayLevel >= 2 as libc::c_int
                            || g_display_prefs.progressSetting as libc::c_uint
                                == FIO_ps_always as libc::c_int as libc::c_uint)
                    {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Read:%6.*f%4s \0" as *const u8 as *const libc::c_char,
                                consumed_hrs.precision,
                                consumed_hrs.value,
                                consumed_hrs.suffix,
                            );
                        }
                    }
                    if fileSize != UTIL_FILESIZE_UNKNOWN as U64 {
                        if g_display_prefs.progressSetting as libc::c_uint
                            != FIO_ps_never as libc::c_int as libc::c_uint
                            && (g_display_prefs.displayLevel >= 2 as libc::c_int
                                || g_display_prefs.progressSetting as libc::c_uint
                                    == FIO_ps_always as libc::c_int as libc::c_uint)
                        {
                            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"/%6.*f%4s\0" as *const u8 as *const libc::c_char,
                                    file_hrs.precision,
                                    file_hrs.value,
                                    file_hrs.suffix,
                                );
                            }
                        }
                    }
                    if g_display_prefs.progressSetting as libc::c_uint
                        != FIO_ps_never as libc::c_int as libc::c_uint
                        && (g_display_prefs.displayLevel >= 2 as libc::c_int
                            || g_display_prefs.progressSetting as libc::c_uint
                                == FIO_ps_always as libc::c_int as libc::c_uint)
                    {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b" ==> %2.f%%\0" as *const u8 as *const libc::c_char,
                                cShare,
                            );
                        }
                    }
                }
            }
        }
        if !(directive as libc::c_uint != ZSTD_e_end as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64 && *readsize != fileSize {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                1719 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                27 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Read error : Incomplete read : %llu / %llu B\0" as *const u8
                    as *const libc::c_char,
                *readsize as libc::c_ulonglong,
                fileSize as libc::c_ulonglong,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(27 as libc::c_int);
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ressPtr).writeCtx);
    return compressedfilesize as libc::c_ulonglong;
}
unsafe extern "C" fn FIO_compressFilename_internal(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
    mut compressionLevel: libc::c_int,
) -> libc::c_int {
    let timeStart = UTIL_getTime();
    let cpuStart = clock();
    let mut readsize = 0 as libc::c_int as U64;
    let mut compressedfilesize = 0 as libc::c_int as U64;
    let fileSize = UTIL_getFileSize(srcFileName);
    if g_display_prefs.displayLevel >= 5 as libc::c_int {
        fprintf(
            stderr,
            b"%s: %llu bytes \n\0" as *const u8 as *const libc::c_char,
            srcFileName,
            fileSize as libc::c_ulonglong,
        );
    }
    match (*prefs).compressionType as libc::c_uint {
        1 => {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1760 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    20 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: file cannot be compressed as gzip (zstd compiled without ZSTD_GZCOMPRESS) -- ignored \n\0"
                        as *const u8 as *const libc::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(20 as libc::c_int);
        }
        2 | 3 => {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1771 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    20 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: file cannot be compressed as xz/lzma (zstd compiled without ZSTD_LZMACOMPRESS) -- ignored \n\0"
                        as *const u8 as *const libc::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(20 as libc::c_int);
        }
        4 => {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1781 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    20 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: file cannot be compressed as lz4 (zstd compiled without ZSTD_LZ4COMPRESS) -- ignored \n\0"
                        as *const u8 as *const libc::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(20 as libc::c_int);
        }
        0 | _ => {
            compressedfilesize = FIO_compressZstdFrame(
                fCtx,
                prefs,
                &mut ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
            ) as U64;
        }
    }
    (*fCtx)
        .totalBytesInput = ((*fCtx).totalBytesInput as libc::c_ulong)
        .wrapping_add(readsize) as size_t as size_t;
    (*fCtx)
        .totalBytesOutput = ((*fCtx).totalBytesOutput as libc::c_ulong)
        .wrapping_add(compressedfilesize) as size_t as size_t;
    if g_display_prefs.progressSetting as libc::c_uint
        != FIO_ps_never as libc::c_int as libc::c_uint
        && (g_display_prefs.displayLevel >= 2 as libc::c_int
            || g_display_prefs.progressSetting as libc::c_uint
                == FIO_ps_always as libc::c_int as libc::c_uint)
    {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"\r%79s\r\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if FIO_shouldDisplayFileSummary(fCtx) != 0 {
        let mut hr_isize = UTIL_makeHumanReadableSize(readsize);
        let mut hr_osize = UTIL_makeHumanReadableSize(compressedfilesize);
        if readsize == 0 as libc::c_int as libc::c_ulong {
            if g_display_prefs.displayLevel >= 2 as libc::c_int
                || g_display_prefs.progressSetting as libc::c_uint
                    == FIO_ps_always as libc::c_int as libc::c_uint
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%-20s :  (%6.*f%s => %6.*f%s, %s) \n\0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                        hr_isize.precision,
                        hr_isize.value,
                        hr_isize.suffix,
                        hr_osize.precision,
                        hr_osize.value,
                        hr_osize.suffix,
                        dstFileName,
                    );
                }
            }
        } else if g_display_prefs.displayLevel >= 2 as libc::c_int
            || g_display_prefs.progressSetting as libc::c_uint
                == FIO_ps_always as libc::c_int as libc::c_uint
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%-20s :%6.2f%%   (%6.*f%s => %6.*f%s, %s) \n\0" as *const u8
                        as *const libc::c_char,
                    srcFileName,
                    compressedfilesize as libc::c_double / readsize as libc::c_double
                        * 100 as libc::c_int as libc::c_double,
                    hr_isize.precision,
                    hr_isize.value,
                    hr_isize.suffix,
                    hr_osize.precision,
                    hr_osize.value,
                    hr_osize.suffix,
                    dstFileName,
                );
            }
        }
    }
    let cpuEnd = clock();
    let cpuLoad_s = (cpuEnd - cpuStart) as libc::c_double
        / CLOCKS_PER_SEC as libc::c_double;
    let timeLength_ns = UTIL_clockSpanNano(timeStart);
    let timeLength_s = timeLength_ns as libc::c_double
        / 1000000000 as libc::c_int as libc::c_double;
    let cpuLoad_pct = cpuLoad_s / timeLength_s * 100 as libc::c_int as libc::c_double;
    if g_display_prefs.displayLevel >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"%-20s : Completed in %.2f sec  (cpu load : %.0f%%)\n\0" as *const u8
                as *const libc::c_char,
            srcFileName,
            timeLength_s,
            cpuLoad_pct,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn FIO_compressFilename_dstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
    mut srcFileStat: *const stat_t,
    mut compressionLevel: libc::c_int,
) -> libc::c_int {
    let mut closeDstFile = 0 as libc::c_int;
    let mut result: libc::c_int = 0;
    let mut transferStat = 0 as libc::c_int;
    let mut dstFile = 0 as *mut FILE;
    let mut dstFd = -(1 as libc::c_int);
    if !(AIO_ReadPool_getFile(ress.readCtx)).is_null() {} else {
        __assert_fail(
            b"AIO_ReadPool_getFile(ress.readCtx) != NULL\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            1845 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 129],
                &[libc::c_char; 129],
            >(
                b"int FIO_compressFilename_dstFile(FIO_ctx_t *const, FIO_prefs_t *const, cRess_t, const char *, const char *, const stat_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (AIO_WritePool_getFile(ress.writeCtx)).is_null() {
        let mut dstFileInitialPermissions = DEFAULT_FILE_PERMISSIONS;
        if strcmp(srcFileName, stdinmark.as_ptr()) != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) != 0
            && UTIL_isRegularFileStat(srcFileStat) != 0
        {
            transferStat = 1 as libc::c_int;
            dstFileInitialPermissions = TEMPORARY_FILE_PERMISSIONS;
        }
        closeDstFile = 1 as libc::c_int;
        if g_display_prefs.displayLevel >= 6 as libc::c_int {
            fprintf(
                stderr,
                b"FIO_compressFilename_dstFile: opening dst: %s \n\0" as *const u8
                    as *const libc::c_char,
                dstFileName,
            );
        }
        dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            srcFileName,
            dstFileName,
            dstFileInitialPermissions,
        );
        if dstFile.is_null() {
            return 1 as libc::c_int;
        }
        dstFd = fileno(dstFile);
        AIO_WritePool_setFile(ress.writeCtx, dstFile);
        addHandler(dstFileName);
    }
    result = FIO_compressFilename_internal(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        compressionLevel,
    );
    if closeDstFile != 0 {
        clearHandler();
        if transferStat != 0 {
            UTIL_setFDStat(dstFd, dstFileName, srcFileStat);
        }
        if g_display_prefs.displayLevel >= 6 as libc::c_int {
            fprintf(
                stderr,
                b"FIO_compressFilename_dstFile: closing dst: %s \n\0" as *const u8
                    as *const libc::c_char,
                dstFileName,
            );
        }
        if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s \n\0" as *const u8 as *const libc::c_char,
                    dstFileName,
                    strerror(*__errno_location()),
                );
            }
            result = 1 as libc::c_int;
        }
        if transferStat != 0 {
            UTIL_utime(dstFileName, srcFileStat);
        }
        if result != 0 as libc::c_int && strcmp(dstFileName, stdoutmark.as_ptr()) != 0 {
            FIO_removeFile(dstFileName);
        }
    }
    return result;
}
static mut compressedFileExtensions: [*const libc::c_char; 10] = [
    ZSTD_EXTENSION.as_ptr(),
    TZSTD_EXTENSION.as_ptr(),
    GZ_EXTENSION.as_ptr(),
    TGZ_EXTENSION.as_ptr(),
    LZMA_EXTENSION.as_ptr(),
    XZ_EXTENSION.as_ptr(),
    TXZ_EXTENSION.as_ptr(),
    LZ4_EXTENSION.as_ptr(),
    TLZ4_EXTENSION.as_ptr(),
    NULL as *const libc::c_char,
];
unsafe extern "C" fn FIO_compressFilename_srcFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
    mut compressionLevel: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut srcFile = 0 as *mut FILE;
    let mut srcFileStat = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut fileSize = UTIL_FILESIZE_UNKNOWN as U64;
    if g_display_prefs.displayLevel >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"FIO_compressFilename_srcFile: %s \n\0" as *const u8 as *const libc::c_char,
            srcFileName,
        );
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        if UTIL_stat(srcFileName, &mut srcFileStat) != 0 {
            if UTIL_isDirectoryStat(&mut srcFileStat) != 0 {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s is a directory -- ignored \n\0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            }
            if !(ress.dictFileName).is_null()
                && UTIL_isSameFileStat(
                    srcFileName,
                    ress.dictFileName,
                    &mut srcFileStat,
                    &mut ress.dictFileStat,
                ) != 0
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: cannot use %s as an input file and dictionary \n\0"
                            as *const u8 as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            }
        }
    }
    if (*prefs).excludeCompressedFiles == 1 as libc::c_int
        && UTIL_isCompressedFile(srcFileName, compressedFileExtensions.as_mut_ptr()) != 0
    {
        if g_display_prefs.displayLevel >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"File is already compressed : %s \n\0" as *const u8
                    as *const libc::c_char,
                srcFileName,
            );
        }
        return 0 as libc::c_int;
    }
    srcFile = FIO_openSrcFile(prefs, srcFileName, &mut srcFileStat);
    if srcFile.is_null() {
        return 1 as libc::c_int;
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        fileSize = UTIL_getFileSizeStat(&mut srcFileStat);
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64
        && fileSize < (ZSTD_BLOCKSIZE_MAX * 3 as libc::c_int) as libc::c_ulong
    {
        AIO_ReadPool_setAsync(ress.readCtx, 0 as libc::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 0 as libc::c_int);
    } else {
        AIO_ReadPool_setAsync(ress.readCtx, 1 as libc::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 1 as libc::c_int);
    }
    AIO_ReadPool_setFile(ress.readCtx, srcFile);
    result = FIO_compressFilename_dstFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        &mut srcFileStat,
        compressionLevel,
    );
    AIO_ReadPool_closeFile(ress.readCtx);
    if (*prefs).removeSrcFile != 0 && result == 0 as libc::c_int
        && strcmp(srcFileName, stdinmark.as_ptr()) != 0
    {
        clearHandler();
        if FIO_removeFile(srcFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    1988 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s\0" as *const u8 as *const libc::c_char,
                    srcFileName,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(1 as libc::c_int);
        }
    }
    return result;
}
unsafe extern "C" fn checked_index(
    mut options: *mut *const libc::c_char,
    mut length: size_t,
    mut index: size_t,
) -> *const libc::c_char {
    if index < length {} else {
        __assert_fail(
            b"index < length\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            1995 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"const char *checked_index(const char **, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    return *options.offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_displayCompressionParameters(
    mut prefs: *const FIO_prefs_t,
) {
    static mut formatOptions: [*const libc::c_char; 5] = [
        ZSTD_EXTENSION.as_ptr(),
        GZ_EXTENSION.as_ptr(),
        XZ_EXTENSION.as_ptr(),
        LZMA_EXTENSION.as_ptr(),
        LZ4_EXTENSION.as_ptr(),
    ];
    static mut sparseOptions: [*const libc::c_char; 3] = [
        b" --no-sparse\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b" --sparse\0" as *const u8 as *const libc::c_char,
    ];
    static mut checkSumOptions: [*const libc::c_char; 3] = [
        b" --no-check\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b" --check\0" as *const u8 as *const libc::c_char,
    ];
    static mut rowMatchFinderOptions: [*const libc::c_char; 3] = [
        b"\0" as *const u8 as *const libc::c_char,
        b" --no-row-match-finder\0" as *const u8 as *const libc::c_char,
        b" --row-match-finder\0" as *const u8 as *const libc::c_char,
    ];
    static mut compressLiteralsOptions: [*const libc::c_char; 3] = [
        b"\0" as *const u8 as *const libc::c_char,
        b" --compress-literals\0" as *const u8 as *const libc::c_char,
        b" --no-compress-literals\0" as *const u8 as *const libc::c_char,
    ];
    if g_display_prefs.displayLevel >= 4 as libc::c_int {} else {
        __assert_fail(
            b"g_display_prefs.displayLevel >= 4\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            2012 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void FIO_displayCompressionParameters(const FIO_prefs_t *)\0"))
                .as_ptr(),
        );
    }
    fprintf(
        stderr,
        b"--format=%s\0" as *const u8 as *const libc::c_char,
        formatOptions[(*prefs).compressionType as usize],
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        checked_index(
            sparseOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
            (*prefs).sparseFileSupport as size_t,
        ),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        if (*prefs).dictIDFlag != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b" --no-dictID\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        checked_index(
            checkSumOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
            (*prefs).checksumFlag as size_t,
        ),
    );
    fprintf(
        stderr,
        b" --block-size=%d\0" as *const u8 as *const libc::c_char,
        (*prefs).blockSize,
    );
    if (*prefs).adaptiveMode != 0 {
        fprintf(
            stderr,
            b" --adapt=min=%d,max=%d\0" as *const u8 as *const libc::c_char,
            (*prefs).minAdaptLevel,
            (*prefs).maxAdaptLevel,
        );
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        checked_index(
            rowMatchFinderOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
            (*prefs).useRowMatchFinder as size_t,
        ),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        if (*prefs).rsyncable != 0 {
            b" --rsyncable\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    if (*prefs).streamSrcSize != 0 {
        fprintf(
            stderr,
            b" --stream-size=%u\0" as *const u8 as *const libc::c_char,
            (*prefs).streamSrcSize as libc::c_uint,
        );
    }
    if (*prefs).srcSizeHint != 0 {
        fprintf(
            stderr,
            b" --size-hint=%d\0" as *const u8 as *const libc::c_char,
            (*prefs).srcSizeHint,
        );
    }
    if (*prefs).targetCBlockSize != 0 {
        fprintf(
            stderr,
            b" --target-compressed-block-size=%u\0" as *const u8 as *const libc::c_char,
            (*prefs).targetCBlockSize as libc::c_uint,
        );
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        checked_index(
            compressLiteralsOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
            (*prefs).literalCompressionMode as size_t,
        ),
    );
    fprintf(
        stderr,
        b" --memory=%u\0" as *const u8 as *const libc::c_char,
        if (*prefs).memLimit != 0 {
            (*prefs).memLimit
        } else {
            (128 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int))
                as libc::c_uint
        },
    );
    fprintf(
        stderr,
        b" --threads=%d\0" as *const u8 as *const libc::c_char,
        (*prefs).nbWorkers,
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        if (*prefs).excludeCompressedFiles != 0 {
            b" --exclude-compressed\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        stderr,
        b" --%scontent-size\0" as *const u8 as *const libc::c_char,
        if (*prefs).contentSize != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"no-\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_compressFilename(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
    mut dictFileName: *const libc::c_char,
    mut compressionLevel: libc::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> libc::c_int {
    let mut ress = FIO_createCResources(
        prefs,
        dictFileName,
        UTIL_getFileSize(srcFileName) as libc::c_ulonglong,
        compressionLevel,
        comprParams,
    );
    let result = FIO_compressFilename_srcFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        compressionLevel,
    );
    FIO_freeCResources(&mut ress);
    return result;
}
unsafe extern "C" fn FIO_determineCompressedName(
    mut srcFileName: *const libc::c_char,
    mut outDirName: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> *const libc::c_char {
    static mut dfnbCapacity: size_t = 0 as libc::c_int as size_t;
    static mut dstFileNameBuffer: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut outDirFilename = NULL as *mut libc::c_char;
    let mut sfnSize = strlen(srcFileName);
    let srcSuffixLen = strlen(suffix);
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        return stdoutmark.as_ptr();
    }
    if !outDirName.is_null() {
        outDirFilename = FIO_createFilename_fromOutDir(
            srcFileName,
            outDirName,
            srcSuffixLen,
        );
        sfnSize = strlen(outDirFilename);
        if !outDirFilename.is_null() {} else {
            __assert_fail(
                b"outDirFilename != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2073 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"const char *FIO_determineCompressedName(const char *, const char *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if dfnbCapacity
        <= sfnSize
            .wrapping_add(srcSuffixLen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        free(dstFileNameBuffer as *mut libc::c_void);
        dfnbCapacity = sfnSize
            .wrapping_add(srcSuffixLen)
            .wrapping_add(30 as libc::c_int as libc::c_ulong);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut libc::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    2082 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    30 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(30 as libc::c_int);
        }
    }
    if !dstFileNameBuffer.is_null() {} else {
        __assert_fail(
            b"dstFileNameBuffer != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            2085 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"const char *FIO_determineCompressedName(const char *, const char *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if !outDirFilename.is_null() {
        memcpy(
            dstFileNameBuffer as *mut libc::c_void,
            outDirFilename as *const libc::c_void,
            sfnSize,
        );
        free(outDirFilename as *mut libc::c_void);
    } else {
        memcpy(
            dstFileNameBuffer as *mut libc::c_void,
            srcFileName as *const libc::c_void,
            sfnSize,
        );
    }
    memcpy(
        dstFileNameBuffer.offset(sfnSize as isize) as *mut libc::c_void,
        suffix as *const libc::c_void,
        srcSuffixLen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return dstFileNameBuffer;
}
unsafe extern "C" fn FIO_getLargestFileSize(
    mut inFileNames: *mut *const libc::c_char,
    mut nbFiles: libc::c_uint,
) -> libc::c_ulonglong {
    let mut i: size_t = 0;
    let mut fileSize: libc::c_ulonglong = 0;
    let mut maxFileSize = 0 as libc::c_int as libc::c_ulonglong;
    i = 0 as libc::c_int as size_t;
    while i < nbFiles as libc::c_ulong {
        fileSize = UTIL_getFileSize(*inFileNames.offset(i as isize))
            as libc::c_ulonglong;
        maxFileSize = if fileSize > maxFileSize { fileSize } else { maxFileSize };
        i = i.wrapping_add(1);
    }
    return maxFileSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_compressMultipleFilenames(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut inFileNamesTable: *mut *const libc::c_char,
    mut outMirroredRootDirName: *const libc::c_char,
    mut outDirName: *const libc::c_char,
    mut outFileName: *const libc::c_char,
    mut suffix: *const libc::c_char,
    mut dictFileName: *const libc::c_char,
    mut compressionLevel: libc::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut error = 0 as libc::c_int;
    let mut ress = FIO_createCResources(
        prefs,
        dictFileName,
        FIO_getLargestFileSize(inFileNamesTable, (*fCtx).nbFilesTotal as libc::c_uint),
        compressionLevel,
        comprParams,
    );
    if !outFileName.is_null() || !suffix.is_null() {} else {
        __assert_fail(
            b"outFileName != NULL || suffix != NULL\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            2130 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 190],
                &[libc::c_char; 190],
            >(
                b"int FIO_compressMultipleFilenames(FIO_ctx_t *const, FIO_prefs_t *const, const char **, const char *, const char *, const char *, const char *, const char *, int, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    if !outFileName.is_null() {
        let mut dstFile = 0 as *mut FILE;
        if FIO_multiFilesConcatWarning(fCtx, prefs, outFileName, 1 as libc::c_int) != 0 {
            FIO_freeCResources(&mut ress);
            return 1 as libc::c_int;
        }
        dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            NULL as *const libc::c_char,
            outFileName,
            DEFAULT_FILE_PERMISSIONS,
        );
        if dstFile.is_null() {
            error = 1 as libc::c_int;
        } else {
            AIO_WritePool_setFile(ress.writeCtx, dstFile);
            while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
                status = FIO_compressFilename_srcFile(
                    fCtx,
                    prefs,
                    ress,
                    outFileName,
                    *inFileNamesTable.offset((*fCtx).currFileIdx as isize),
                    compressionLevel,
                );
                if status == 0 {
                    (*fCtx).nbFilesProcessed += 1;
                }
                error |= status;
                (*fCtx).currFileIdx += 1;
            }
            if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                            as *const libc::c_char,
                        2149 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        29 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Write error (%s) : cannot properly close %s\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                        outFileName,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(29 as libc::c_int);
            }
        }
    } else {
        if !outMirroredRootDirName.is_null() {
            UTIL_mirrorSourceFilesDirectories(
                inFileNamesTable,
                (*fCtx).nbFilesTotal as libc::c_uint,
                outMirroredRootDirName,
            );
        }
        let mut current_block_63: u64;
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            let srcFileName = *inFileNamesTable.offset((*fCtx).currFileIdx as isize);
            let mut dstFileName = NULL as *const libc::c_char;
            if !outMirroredRootDirName.is_null() {
                let mut validMirroredDirName = UTIL_createMirroredDestDirName(
                    srcFileName,
                    outMirroredRootDirName,
                );
                if !validMirroredDirName.is_null() {
                    dstFileName = FIO_determineCompressedName(
                        srcFileName,
                        validMirroredDirName,
                        suffix,
                    );
                    free(validMirroredDirName as *mut libc::c_void);
                    current_block_63 = 5892776923941496671;
                } else {
                    if g_display_prefs.displayLevel >= 2 as libc::c_int {
                        fprintf(
                            stderr,
                            b"zstd: --output-dir-mirror cannot compress '%s' into '%s' \n\0"
                                as *const u8 as *const libc::c_char,
                            srcFileName,
                            outMirroredRootDirName,
                        );
                    }
                    error = 1 as libc::c_int;
                    current_block_63 = 15090052786889560393;
                }
            } else {
                dstFileName = FIO_determineCompressedName(
                    srcFileName,
                    outDirName,
                    suffix,
                );
                current_block_63 = 5892776923941496671;
            }
            match current_block_63 {
                5892776923941496671 => {
                    status = FIO_compressFilename_srcFile(
                        fCtx,
                        prefs,
                        ress,
                        dstFileName,
                        srcFileName,
                        compressionLevel,
                    );
                    if status == 0 {
                        (*fCtx).nbFilesProcessed += 1;
                    }
                    error |= status;
                }
                _ => {}
            }
            (*fCtx).currFileIdx += 1;
        }
        if !outDirName.is_null() {
            FIO_checkFilenameCollisions(
                inFileNamesTable,
                (*fCtx).nbFilesTotal as libc::c_uint,
            );
        }
    }
    if FIO_shouldDisplayMultipleFileSummary(fCtx) != 0 {
        let mut hr_isize = UTIL_makeHumanReadableSize((*fCtx).totalBytesInput);
        let mut hr_osize = UTIL_makeHumanReadableSize((*fCtx).totalBytesOutput);
        if g_display_prefs.progressSetting as libc::c_uint
            != FIO_ps_never as libc::c_int as libc::c_uint
            && (g_display_prefs.displayLevel >= 2 as libc::c_int
                || g_display_prefs.progressSetting as libc::c_uint
                    == FIO_ps_always as libc::c_int as libc::c_uint)
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"\r%79s\r\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if (*fCtx).totalBytesInput == 0 as libc::c_int as libc::c_ulong {
            if g_display_prefs.displayLevel >= 2 as libc::c_int
                || g_display_prefs.progressSetting as libc::c_uint
                    == FIO_ps_always as libc::c_int as libc::c_uint
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%3d files compressed : (%6.*f%4s => %6.*f%4s)\n\0" as *const u8
                            as *const libc::c_char,
                        (*fCtx).nbFilesProcessed,
                        hr_isize.precision,
                        hr_isize.value,
                        hr_isize.suffix,
                        hr_osize.precision,
                        hr_osize.value,
                        hr_osize.suffix,
                    );
                }
            }
        } else if g_display_prefs.displayLevel >= 2 as libc::c_int
            || g_display_prefs.progressSetting as libc::c_uint
                == FIO_ps_always as libc::c_int as libc::c_uint
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%3d files compressed : %.2f%% (%6.*f%4s => %6.*f%4s)\n\0"
                        as *const u8 as *const libc::c_char,
                    (*fCtx).nbFilesProcessed,
                    (*fCtx).totalBytesOutput as libc::c_double
                        / (*fCtx).totalBytesInput as libc::c_double
                        * 100 as libc::c_int as libc::c_double,
                    hr_isize.precision,
                    hr_isize.value,
                    hr_isize.suffix,
                    hr_osize.precision,
                    hr_osize.value,
                    hr_osize.suffix,
                );
            }
        }
    }
    FIO_freeCResources(&mut ress);
    return error;
}
unsafe extern "C" fn FIO_createDResources(
    prefs: *mut FIO_prefs_t,
    mut dictFileName: *const libc::c_char,
) -> dRess_t {
    let mut useMMap = ((*prefs).mmapDict as libc::c_uint
        == ZSTD_ps_enable as libc::c_int as libc::c_uint) as libc::c_int;
    let mut forceNoUseMMap = ((*prefs).mmapDict as libc::c_uint
        == ZSTD_ps_disable as libc::c_int as libc::c_uint) as libc::c_int;
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut ress = dRess_t {
        dict: FIO_Dict_t {
            dictBuffer: 0 as *mut libc::c_void,
            dictBufferSize: 0,
            dictBufferType: FIO_mallocDict,
        },
        dctx: 0 as *mut ZSTD_DStream,
        writeCtx: 0 as *mut WritePoolCtx_t,
        readCtx: 0 as *mut ReadPoolCtx_t,
    };
    memset(
        &mut ress as *mut dRess_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<dRess_t>() as libc::c_ulong,
    );
    FIO_getDictFileStat(dictFileName, &mut statbuf);
    if (*prefs).patchFromMode != 0 {
        let dictSize = UTIL_getFileSizeStat(&mut statbuf);
        useMMap |= (dictSize > (*prefs).memLimit as libc::c_ulong) as libc::c_int;
        FIO_adjustMemLimitForPatchFromMode(
            prefs,
            dictSize as libc::c_ulonglong,
            0 as libc::c_int as libc::c_ulonglong,
        );
    }
    ress.dctx = ZSTD_createDStream();
    if (ress.dctx).is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2238 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error: %s : can't create ZSTD_DStream\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(60 as libc::c_int);
    }
    let mut err: size_t = 0;
    err = ZSTD_DCtx_setMaxWindowSize(ress.dctx, (*prefs).memLimit as size_t);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_DCtx_setMaxWindowSize(ress.dctx, prefs->memLimit)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2239 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut err_0: size_t = 0;
    err_0 = ZSTD_DCtx_setParameter(
        ress.dctx,
        ZSTD_d_experimentalParam3,
        ((*prefs).checksumFlag == 0) as libc::c_int,
    );
    if ZSTD_isError(err_0) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_DCtx_setParameter(ress.dctx, ZSTD_d_experimentalParam3, !prefs->checksumFlag)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2240 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_0),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    let mut dictBufferType = (if useMMap != 0 && forceNoUseMMap == 0 {
        FIO_mmapDict as libc::c_int
    } else {
        FIO_mallocDict as libc::c_int
    }) as FIO_dictBufferType_t;
    FIO_initDict(&mut ress.dict, dictFileName, prefs, &mut statbuf, dictBufferType);
    let mut err_1: size_t = 0;
    err_1 = ZSTD_DCtx_reset(ress.dctx, ZSTD_reset_session_only);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_DCtx_reset(ress.dctx, ZSTD_reset_session_only)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2247 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    if (*prefs).patchFromMode != 0 {
        let mut err_2: size_t = 0;
        err_2 = ZSTD_DCtx_refPrefix(
            ress.dctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_2) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_DCtx_refPrefix(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    2250 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_2),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    } else {
        let mut err_3: size_t = 0;
        err_3 = ZSTD_DCtx_loadDictionary_byReference(
            ress.dctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_3) != 0 {
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const libc::c_char,
                    b"ZSTD_DCtx_loadDictionary_byReference(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    2252 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_3),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(11 as libc::c_int);
        }
    }
    ress.writeCtx = AIO_WritePool_create(prefs, ZSTD_DStreamOutSize());
    ress.readCtx = AIO_ReadPool_create(prefs, ZSTD_DStreamInSize());
    return ress;
}
unsafe extern "C" fn FIO_freeDResources(mut ress: dRess_t) {
    FIO_freeDict(&mut ress.dict);
    let mut err: size_t = 0;
    err = ZSTD_freeDStream(ress.dctx);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const libc::c_char,
                b"ZSTD_freeDStream(ress.dctx)\0" as *const u8 as *const libc::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2264 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        exit(11 as libc::c_int);
    }
    AIO_WritePool_free(ress.writeCtx);
    AIO_ReadPool_free(ress.readCtx);
}
unsafe extern "C" fn FIO_passThrough(mut ress: *mut dRess_t) -> libc::c_int {
    let blockSize = if (if ((64 as libc::c_int
        * ((1 as libc::c_int) << 10 as libc::c_int)) as libc::c_ulong)
        < ZSTD_DStreamInSize()
    {
        (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as libc::c_ulong
    } else {
        ZSTD_DStreamInSize()
    }) < ZSTD_DStreamOutSize()
    {
        if ((64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulong) < ZSTD_DStreamInSize()
        {
            (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong
        } else {
            ZSTD_DStreamInSize()
        }
    } else {
        ZSTD_DStreamOutSize()
    };
    let mut writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    AIO_ReadPool_fillBuffer((*ress).readCtx, blockSize);
    while (*(*ress).readCtx).srcBufferLoaded != 0 {
        let mut writeSize: size_t = 0;
        writeSize = if blockSize < (*(*ress).readCtx).srcBufferLoaded {
            blockSize
        } else {
            (*(*ress).readCtx).srcBufferLoaded
        };
        if writeSize <= (*writeJob).bufferSize {} else {
            __assert_fail(
                b"writeSize <= writeJob->bufferSize\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2280 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"int FIO_passThrough(dRess_t *)\0"))
                    .as_ptr(),
            );
        }
        memcpy(
            (*writeJob).buffer,
            (*(*ress).readCtx).srcBuffer as *const libc::c_void,
            writeSize,
        );
        (*writeJob).usedBufferSize = writeSize;
        AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
        AIO_ReadPool_consumeBytes((*ress).readCtx, writeSize);
        AIO_ReadPool_fillBuffer((*ress).readCtx, blockSize);
    }
    if (*(*ress).readCtx).reachedEof != 0 {} else {
        __assert_fail(
            b"ress->readCtx->reachedEof\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            2287 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"int FIO_passThrough(dRess_t *)\0"))
                .as_ptr(),
        );
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    return 0 as libc::c_int;
}
unsafe extern "C" fn FIO_zstdErrorHelp(
    prefs: *const FIO_prefs_t,
    mut ress: *const dRess_t,
    mut err: size_t,
    mut srcFileName: *const libc::c_char,
) {
    let mut header = ZSTD_frameHeader {
        frameContentSize: 0,
        windowSize: 0,
        blockSizeMax: 0,
        frameType: ZSTD_frame,
        headerSize: 0,
        dictID: 0,
        checksumFlag: 0,
        _reserved1: 0,
        _reserved2: 0,
    };
    if ZSTD_getErrorCode(err) as libc::c_uint
        != ZSTD_error_frameParameter_windowTooLarge as libc::c_int as libc::c_uint
    {
        return;
    }
    err = ZSTD_getFrameHeader(
        &mut header,
        (*(*ress).readCtx).srcBuffer as *const libc::c_void,
        (*(*ress).readCtx).srcBufferLoaded,
    );
    if err == 0 as libc::c_int as libc::c_ulong {
        let windowSize = header.windowSize;
        let windowLog = (FIO_highbit64(windowSize))
            .wrapping_add(
                (windowSize
                    & windowSize.wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                    != 0 as libc::c_int as libc::c_ulonglong) as libc::c_int
                    as libc::c_uint,
            );
        if (*prefs).memLimit > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"prefs->memLimit > 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2312 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"void FIO_zstdErrorHelp(const FIO_prefs_t *const, const dRess_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s : Window size larger than maximum : %llu > %u \n\0" as *const u8
                    as *const libc::c_char,
                srcFileName,
                windowSize,
                (*prefs).memLimit,
            );
        }
        if windowLog
            <= (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) as libc::c_uint
        {
            let windowMB = (windowSize >> 20 as libc::c_int)
                .wrapping_add(
                    (windowSize
                        & (1 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)
                            - 1 as libc::c_int) as libc::c_ulonglong
                        != 0 as libc::c_int as libc::c_ulonglong) as libc::c_int
                        as libc::c_ulonglong,
                ) as libc::c_uint;
            if windowSize
                < ((1 as libc::c_ulonglong) << 52 as libc::c_int) as U64
                    as libc::c_ulonglong
            {} else {
                __assert_fail(
                    b"windowSize < (U64)(1ULL << 52)\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    2317 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 88],
                        &[libc::c_char; 88],
                    >(
                        b"void FIO_zstdErrorHelp(const FIO_prefs_t *const, const dRess_t *, size_t, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s : Use --long=%u or --memory=%uMB \n\0" as *const u8
                        as *const libc::c_char,
                    srcFileName,
                    windowLog,
                    windowMB,
                );
            }
            return;
        }
    }
    if g_display_prefs.displayLevel >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s : Window log larger than ZSTD_WINDOWLOG_MAX=%u; not supported \n\0"
                as *const u8 as *const libc::c_char,
            srcFileName,
            if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            },
        );
    }
}
pub const FIO_ERROR_FRAME_DECODING: libc::c_int = -(2 as libc::c_int);
unsafe extern "C" fn FIO_decompressZstdFrame(
    fCtx: *mut FIO_ctx_t,
    mut ress: *mut dRess_t,
    prefs: *const FIO_prefs_t,
    mut srcFileName: *const libc::c_char,
    mut alreadyDecoded: U64,
) -> libc::c_ulonglong {
    let mut frameSize = 0 as libc::c_int as U64;
    let mut writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    let srcFileLength = strlen(srcFileName);
    if srcFileLength > 20 as libc::c_int as libc::c_ulong {
        srcFileName = srcFileName
            .offset(
                srcFileLength.wrapping_sub(20 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    ZSTD_DCtx_reset((*ress).dctx, ZSTD_reset_session_only);
    AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_FRAMEHEADERSIZE_MAX as size_t);
    loop {
        let mut inBuff = setInBuffer(
            (*(*ress).readCtx).srcBuffer as *const libc::c_void,
            (*(*ress).readCtx).srcBufferLoaded,
            0 as libc::c_int as size_t,
        );
        let mut outBuff = setOutBuffer(
            (*writeJob).buffer,
            (*writeJob).bufferSize,
            0 as libc::c_int as size_t,
        );
        let readSizeHint = ZSTD_decompressStream(
            (*ress).dctx,
            &mut outBuff,
            &mut inBuff,
        );
        let hrs = UTIL_makeHumanReadableSize(alreadyDecoded.wrapping_add(frameSize));
        if ZSTD_isError(readSizeHint) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s : Decoding error (36) : %s \n\0" as *const u8
                        as *const libc::c_char,
                    srcFileName,
                    ZSTD_getErrorName(readSizeHint),
                );
            }
            FIO_zstdErrorHelp(prefs, ress, readSizeHint, srcFileName);
            AIO_WritePool_releaseIoJob(writeJob);
            return FIO_ERROR_FRAME_DECODING as libc::c_ulonglong;
        }
        (*writeJob).usedBufferSize = outBuff.pos;
        AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
        frameSize = (frameSize as libc::c_ulong).wrapping_add(outBuff.pos) as U64 as U64;
        if (*fCtx).nbFilesTotal > 1 as libc::c_int {
            let mut srcFileNameSize = strlen(srcFileName);
            if srcFileNameSize > 18 as libc::c_int as libc::c_ulong {
                let mut truncatedSrcFileName = srcFileName
                    .offset(srcFileNameSize as isize)
                    .offset(-(15 as libc::c_int as isize));
                if g_display_prefs.progressSetting as libc::c_uint
                    != FIO_ps_never as libc::c_int as libc::c_uint
                    && (g_display_prefs.displayLevel >= 2 as libc::c_int
                        || g_display_prefs.progressSetting as libc::c_uint
                            == FIO_ps_always as libc::c_int as libc::c_uint)
                {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int
                        && g_display_prefs.progressSetting as libc::c_uint
                            != FIO_ps_never as libc::c_int as libc::c_uint
                    {
                        if UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                            || g_display_prefs.displayLevel >= 4 as libc::c_int
                        {
                            g_displayClock = UTIL_getTime();
                            fprintf(
                                stderr,
                                b"\rDecompress: %2u/%2u files. Current: ...%s : %.*f%s...    \0"
                                    as *const u8 as *const libc::c_char,
                                (*fCtx).currFileIdx + 1 as libc::c_int,
                                (*fCtx).nbFilesTotal,
                                truncatedSrcFileName,
                                hrs.precision,
                                hrs.value,
                                hrs.suffix,
                            );
                            if g_display_prefs.displayLevel >= 4 as libc::c_int {
                                fflush(stderr);
                            }
                        }
                    }
                }
            } else if g_display_prefs.progressSetting as libc::c_uint
                != FIO_ps_never as libc::c_int as libc::c_uint
                && (g_display_prefs.displayLevel >= 2 as libc::c_int
                    || g_display_prefs.progressSetting as libc::c_uint
                        == FIO_ps_always as libc::c_int as libc::c_uint)
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int
                    && g_display_prefs.progressSetting as libc::c_uint
                        != FIO_ps_never as libc::c_int as libc::c_uint
                {
                    if UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                        || g_display_prefs.displayLevel >= 4 as libc::c_int
                    {
                        g_displayClock = UTIL_getTime();
                        fprintf(
                            stderr,
                            b"\rDecompress: %2u/%2u files. Current: %s : %.*f%s...    \0"
                                as *const u8 as *const libc::c_char,
                            (*fCtx).currFileIdx + 1 as libc::c_int,
                            (*fCtx).nbFilesTotal,
                            srcFileName,
                            hrs.precision,
                            hrs.value,
                            hrs.suffix,
                        );
                        if g_display_prefs.displayLevel >= 4 as libc::c_int {
                            fflush(stderr);
                        }
                    }
                }
            }
        } else if g_display_prefs.progressSetting as libc::c_uint
            != FIO_ps_never as libc::c_int as libc::c_uint
            && (g_display_prefs.displayLevel >= 2 as libc::c_int
                || g_display_prefs.progressSetting as libc::c_uint
                    == FIO_ps_always as libc::c_int as libc::c_uint)
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int
                && g_display_prefs.progressSetting as libc::c_uint
                    != FIO_ps_never as libc::c_int as libc::c_uint
            {
                if UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4 as libc::c_int
                {
                    g_displayClock = UTIL_getTime();
                    fprintf(
                        stderr,
                        b"\r%-20.20s : %.*f%s...     \0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                        hrs.precision,
                        hrs.value,
                        hrs.suffix,
                    );
                    if g_display_prefs.displayLevel >= 4 as libc::c_int {
                        fflush(stderr);
                    }
                }
            }
        }
        AIO_ReadPool_consumeBytes((*ress).readCtx, inBuff.pos);
        if readSizeHint == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        let toDecode = if readSizeHint < ZSTD_DStreamInSize() {
            readSizeHint
        } else {
            ZSTD_DStreamInSize()
        };
        if (*(*ress).readCtx).srcBufferLoaded < toDecode {
            let readSize = AIO_ReadPool_fillBuffer((*ress).readCtx, toDecode);
            if readSize == 0 as libc::c_int as libc::c_ulong {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s : Read error (39) : premature end \n\0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                    );
                }
                AIO_WritePool_releaseIoJob(writeJob);
                return FIO_ERROR_FRAME_DECODING as libc::c_ulonglong;
            }
        }
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    return frameSize as libc::c_ulonglong;
}
unsafe extern "C" fn FIO_decompressFrames(
    fCtx: *mut FIO_ctx_t,
    mut ress: dRess_t,
    prefs: *const FIO_prefs_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
) -> libc::c_int {
    let mut readSomething = 0 as libc::c_int as libc::c_uint;
    let mut filesize = 0 as libc::c_int as libc::c_ulonglong;
    let mut passThrough = (*prefs).passThrough;
    if passThrough == -(1 as libc::c_int) {
        passThrough = ((*prefs).overwrite != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) == 0) as libc::c_int;
    }
    if passThrough == 0 as libc::c_int || passThrough == 1 as libc::c_int {} else {
        __assert_fail(
            b"passThrough == 0 || passThrough == 1\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            2637 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 106],
                &[libc::c_char; 106],
            >(
                b"int FIO_decompressFrames(FIO_ctx_t *const, dRess_t, const FIO_prefs_t *const, const char *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    loop {
        let toRead = 4 as libc::c_int as size_t;
        let mut buf = 0 as *const BYTE;
        AIO_ReadPool_fillBuffer(ress.readCtx, toRead);
        buf = (*ress.readCtx).srcBuffer as *const BYTE;
        if (*ress.readCtx).srcBufferLoaded == 0 as libc::c_int as libc::c_ulong {
            if readSomething == 0 as libc::c_int as libc::c_uint {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: unexpected end of file \n\0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            }
            break;
        } else {
            readSomething = 1 as libc::c_int as libc::c_uint;
            if (*ress.readCtx).srcBufferLoaded < toRead {
                if passThrough != 0 {
                    return FIO_passThrough(&mut ress);
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: unknown header \n\0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            }
            if ZSTD_isFrame(buf as *const libc::c_void, (*ress.readCtx).srcBufferLoaded)
                != 0
            {
                let frameSize = FIO_decompressZstdFrame(
                    fCtx,
                    &mut ress,
                    prefs,
                    srcFileName,
                    filesize as U64,
                );
                if frameSize == FIO_ERROR_FRAME_DECODING as libc::c_ulonglong {
                    return 1 as libc::c_int;
                }
                filesize = filesize.wrapping_add(frameSize);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                == 31 as libc::c_int
                && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                    == 139 as libc::c_int
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: gzip file cannot be uncompressed (zstd compiled without HAVE_ZLIB) -- ignored \n\0"
                            as *const u8 as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                == 0xfd as libc::c_int
                && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0x37 as libc::c_int
                || *buf.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0x5d as libc::c_int
                    && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: xz/lzma file cannot be uncompressed (zstd compiled without HAVE_LZMA) -- ignored \n\0"
                            as *const u8 as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            } else if MEM_readLE32(buf as *const libc::c_void)
                == LZ4_MAGICNUMBER as libc::c_uint
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: lz4 file cannot be uncompressed (zstd compiled without HAVE_LZ4) -- ignored \n\0"
                            as *const u8 as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            } else if passThrough != 0 {
                return FIO_passThrough(&mut ress)
            } else {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: unsupported format \n\0" as *const u8
                            as *const libc::c_char,
                        srcFileName,
                    );
                }
                return 1 as libc::c_int;
            }
        }
    }
    (*fCtx)
        .totalBytesOutput = ((*fCtx).totalBytesOutput as libc::c_ulong)
        .wrapping_add(filesize as size_t) as size_t as size_t;
    if g_display_prefs.progressSetting as libc::c_uint
        != FIO_ps_never as libc::c_int as libc::c_uint
        && (g_display_prefs.displayLevel >= 2 as libc::c_int
            || g_display_prefs.progressSetting as libc::c_uint
                == FIO_ps_always as libc::c_int as libc::c_uint)
    {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"\r%79s\r\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if FIO_shouldDisplayFileSummary(fCtx) != 0 {
        if g_display_prefs.displayLevel >= 2 as libc::c_int
            || g_display_prefs.progressSetting as libc::c_uint
                == FIO_ps_always as libc::c_int as libc::c_uint
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%-20s: %llu bytes \n\0" as *const u8 as *const libc::c_char,
                    srcFileName,
                    filesize,
                );
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn FIO_decompressDstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: dRess_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
    mut srcFileStat: *const stat_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut releaseDstFile = 0 as libc::c_int;
    let mut transferStat = 0 as libc::c_int;
    let mut dstFd = 0 as libc::c_int;
    if (AIO_WritePool_getFile(ress.writeCtx)).is_null()
        && (*prefs).testMode == 0 as libc::c_int
    {
        let mut dstFile = 0 as *mut FILE;
        let mut dstFilePermissions = DEFAULT_FILE_PERMISSIONS;
        if strcmp(srcFileName, stdinmark.as_ptr()) != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) != 0
            && UTIL_isRegularFileStat(srcFileStat) != 0
        {
            transferStat = 1 as libc::c_int;
            dstFilePermissions = TEMPORARY_FILE_PERMISSIONS;
        }
        releaseDstFile = 1 as libc::c_int;
        dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            srcFileName,
            dstFileName,
            dstFilePermissions,
        );
        if dstFile.is_null() {
            return 1 as libc::c_int;
        }
        dstFd = fileno(dstFile);
        AIO_WritePool_setFile(ress.writeCtx, dstFile);
        addHandler(dstFileName);
    }
    result = FIO_decompressFrames(fCtx, ress, prefs, dstFileName, srcFileName);
    if releaseDstFile != 0 {
        clearHandler();
        if transferStat != 0 {
            UTIL_setFDStat(dstFd, dstFileName, srcFileStat);
        }
        if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s \n\0" as *const u8 as *const libc::c_char,
                    dstFileName,
                    strerror(*__errno_location()),
                );
            }
            result = 1 as libc::c_int;
        }
        if transferStat != 0 {
            UTIL_utime(dstFileName, srcFileStat);
        }
        if result != 0 as libc::c_int && strcmp(dstFileName, stdoutmark.as_ptr()) != 0 {
            FIO_removeFile(dstFileName);
        }
    }
    return result;
}
unsafe extern "C" fn FIO_decompressSrcFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: dRess_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
) -> libc::c_int {
    let mut srcFile = 0 as *mut FILE;
    let mut srcFileStat = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut result: libc::c_int = 0;
    let mut fileSize = UTIL_FILESIZE_UNKNOWN as U64;
    if UTIL_isDirectory(srcFileName) != 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s is a directory -- ignored \n\0" as *const u8
                    as *const libc::c_char,
                srcFileName,
            );
        }
        return 1 as libc::c_int;
    }
    srcFile = FIO_openSrcFile(prefs, srcFileName, &mut srcFileStat);
    if srcFile.is_null() {
        return 1 as libc::c_int;
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        fileSize = UTIL_getFileSizeStat(&mut srcFileStat);
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64
        && fileSize < (ZSTD_BLOCKSIZE_MAX * 3 as libc::c_int) as libc::c_ulong
    {
        AIO_ReadPool_setAsync(ress.readCtx, 0 as libc::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 0 as libc::c_int);
    } else {
        AIO_ReadPool_setAsync(ress.readCtx, 1 as libc::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 1 as libc::c_int);
    }
    AIO_ReadPool_setFile(ress.readCtx, srcFile);
    result = FIO_decompressDstFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        &mut srcFileStat,
    );
    AIO_ReadPool_setFile(ress.readCtx, NULL as *mut FILE);
    if fclose(srcFile) != 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s: %s \n\0" as *const u8 as *const libc::c_char,
                srcFileName,
                strerror(*__errno_location()),
            );
        }
        return 1 as libc::c_int;
    }
    if (*prefs).removeSrcFile != 0 && result == 0 as libc::c_int
        && strcmp(srcFileName, stdinmark.as_ptr()) != 0
    {
        clearHandler();
        if FIO_removeFile(srcFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s \n\0" as *const u8 as *const libc::c_char,
                    srcFileName,
                    strerror(*__errno_location()),
                );
            }
            return 1 as libc::c_int;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressFilename(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut dstFileName: *const libc::c_char,
    mut srcFileName: *const libc::c_char,
    mut dictFileName: *const libc::c_char,
) -> libc::c_int {
    let ress = FIO_createDResources(prefs, dictFileName);
    let decodingError = FIO_decompressSrcFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
    );
    FIO_freeDResources(ress);
    return decodingError;
}
static mut suffixList: [*const libc::c_char; 4] = [
    ZSTD_EXTENSION.as_ptr(),
    TZSTD_EXTENSION.as_ptr(),
    ZSTD_ALT_EXTENSION.as_ptr(),
    NULL as *const libc::c_char,
];
static mut suffixListStr: *const libc::c_char = b".zst/.tzst\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn FIO_determineDstName(
    mut srcFileName: *const libc::c_char,
    mut outDirName: *const libc::c_char,
) -> *const libc::c_char {
    static mut dfnbCapacity: size_t = 0 as libc::c_int as size_t;
    static mut dstFileNameBuffer: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut dstFileNameEndPos: size_t = 0;
    let mut outDirFilename = NULL as *mut libc::c_char;
    let mut dstSuffix = b"\0" as *const u8 as *const libc::c_char;
    let mut dstSuffixLen = 0 as libc::c_int as size_t;
    let mut sfnSize = strlen(srcFileName);
    let mut srcSuffixLen: size_t = 0;
    let srcSuffix: *const libc::c_char = strrchr(srcFileName, '.' as i32);
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        return stdoutmark.as_ptr();
    }
    if srcSuffix.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s: unknown suffix (%s expected). Can't derive the output file name. Specify it with -o dstFileName. Ignoring.\n\0"
                    as *const u8 as *const libc::c_char,
                srcFileName,
                suffixListStr,
            );
        }
        return NULL as *const libc::c_char;
    }
    srcSuffixLen = strlen(srcSuffix);
    let mut matchedSuffixPtr = 0 as *mut *const libc::c_char;
    matchedSuffixPtr = suffixList.as_mut_ptr();
    while !(*matchedSuffixPtr).is_null() {
        if strcmp(*matchedSuffixPtr, srcSuffix) == 0 {
            break;
        }
        matchedSuffixPtr = matchedSuffixPtr.offset(1);
    }
    if sfnSize <= srcSuffixLen || (*matchedSuffixPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: %s: unknown suffix (%s expected). Can't derive the output file name. Specify it with -o dstFileName. Ignoring.\n\0"
                    as *const u8 as *const libc::c_char,
                srcFileName,
                suffixListStr,
            );
        }
        return NULL as *const libc::c_char;
    }
    if *(*matchedSuffixPtr).offset(1 as libc::c_int as isize) as libc::c_int
        == 't' as i32
    {
        dstSuffix = b".tar\0" as *const u8 as *const libc::c_char;
        dstSuffixLen = strlen(dstSuffix);
    }
    if !outDirName.is_null() {
        outDirFilename = FIO_createFilename_fromOutDir(
            srcFileName,
            outDirName,
            0 as libc::c_int as size_t,
        );
        sfnSize = strlen(outDirFilename);
        if !outDirFilename.is_null() {} else {
            __assert_fail(
                b"outDirFilename != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                    as *const libc::c_char,
                2948 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"const char *FIO_determineDstName(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    }
    if dfnbCapacity.wrapping_add(srcSuffixLen)
        <= sfnSize
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(dstSuffixLen)
    {
        free(dstFileNameBuffer as *mut libc::c_void);
        dfnbCapacity = sfnSize.wrapping_add(20 as libc::c_int as libc::c_ulong);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut libc::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    2958 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    74 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s : not enough memory for dstFileName\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(74 as libc::c_int);
        }
    }
    if !dstFileNameBuffer.is_null() {} else {
        __assert_fail(
            b"dstFileNameBuffer != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            2962 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"const char *FIO_determineDstName(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    dstFileNameEndPos = sfnSize.wrapping_sub(srcSuffixLen);
    if !outDirFilename.is_null() {
        memcpy(
            dstFileNameBuffer as *mut libc::c_void,
            outDirFilename as *const libc::c_void,
            dstFileNameEndPos,
        );
        free(outDirFilename as *mut libc::c_void);
    } else {
        memcpy(
            dstFileNameBuffer as *mut libc::c_void,
            srcFileName as *const libc::c_void,
            dstFileNameEndPos,
        );
    }
    strcpy(dstFileNameBuffer.offset(dstFileNameEndPos as isize), dstSuffix);
    return dstFileNameBuffer;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressMultipleFilenames(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut srcNamesTable: *mut *const libc::c_char,
    mut outMirroredRootDirName: *const libc::c_char,
    mut outDirName: *const libc::c_char,
    mut outFileName: *const libc::c_char,
    mut dictFileName: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut error = 0 as libc::c_int;
    let mut ress = FIO_createDResources(prefs, dictFileName);
    if !outFileName.is_null() {
        if FIO_multiFilesConcatWarning(fCtx, prefs, outFileName, 1 as libc::c_int) != 0 {
            FIO_freeDResources(ress);
            return 1 as libc::c_int;
        }
        if (*prefs).testMode == 0 {
            let mut dstFile = FIO_openDstFile(
                fCtx,
                prefs,
                NULL as *const libc::c_char,
                outFileName,
                DEFAULT_FILE_PERMISSIONS,
            );
            if dstFile.is_null() {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                            as *const libc::c_char,
                        2998 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        19 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"cannot open %s\0" as *const u8 as *const libc::c_char,
                        outFileName,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(19 as libc::c_int);
            }
            AIO_WritePool_setFile(ress.writeCtx, dstFile);
        }
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            status = FIO_decompressSrcFile(
                fCtx,
                prefs,
                ress,
                outFileName,
                *srcNamesTable.offset((*fCtx).currFileIdx as isize),
            );
            if status == 0 {
                (*fCtx).nbFilesProcessed += 1;
            }
            error |= status;
            (*fCtx).currFileIdx += 1;
        }
        if (*prefs).testMode == 0 && AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                        as *const libc::c_char,
                    3008 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    72 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Write error : %s : cannot properly close output file\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(72 as libc::c_int);
        }
    } else {
        if !outMirroredRootDirName.is_null() {
            UTIL_mirrorSourceFilesDirectories(
                srcNamesTable,
                (*fCtx).nbFilesTotal as libc::c_uint,
                outMirroredRootDirName,
            );
        }
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            let srcFileName = *srcNamesTable.offset((*fCtx).currFileIdx as isize);
            let mut dstFileName = NULL as *const libc::c_char;
            if !outMirroredRootDirName.is_null() {
                let mut validMirroredDirName = UTIL_createMirroredDestDirName(
                    srcFileName,
                    outMirroredRootDirName,
                );
                if !validMirroredDirName.is_null() {
                    dstFileName = FIO_determineDstName(
                        srcFileName,
                        validMirroredDirName,
                    );
                    free(validMirroredDirName as *mut libc::c_void);
                } else if g_display_prefs.displayLevel >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"zstd: --output-dir-mirror cannot decompress '%s' into '%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        srcFileName,
                        outMirroredRootDirName,
                    );
                }
            } else {
                dstFileName = FIO_determineDstName(srcFileName, outDirName);
            }
            if dstFileName.is_null() {
                error = 1 as libc::c_int;
            } else {
                status = FIO_decompressSrcFile(
                    fCtx,
                    prefs,
                    ress,
                    dstFileName,
                    srcFileName,
                );
                if status == 0 {
                    (*fCtx).nbFilesProcessed += 1;
                }
                error |= status;
            }
            (*fCtx).currFileIdx += 1;
        }
        if !outDirName.is_null() {
            FIO_checkFilenameCollisions(
                srcNamesTable,
                (*fCtx).nbFilesTotal as libc::c_uint,
            );
        }
    }
    if FIO_shouldDisplayMultipleFileSummary(fCtx) != 0 {
        if g_display_prefs.progressSetting as libc::c_uint
            != FIO_ps_never as libc::c_int as libc::c_uint
            && (g_display_prefs.displayLevel >= 2 as libc::c_int
                || g_display_prefs.progressSetting as libc::c_uint
                    == FIO_ps_always as libc::c_int as libc::c_uint)
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"\r%79s\r\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if g_display_prefs.displayLevel >= 2 as libc::c_int
            || g_display_prefs.progressSetting as libc::c_uint
                == FIO_ps_always as libc::c_int as libc::c_uint
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%d files decompressed : %6llu bytes total \n\0" as *const u8
                        as *const libc::c_char,
                    (*fCtx).nbFilesProcessed,
                    (*fCtx).totalBytesOutput as libc::c_ulonglong,
                );
            }
        }
    }
    FIO_freeDResources(ress);
    return error;
}
unsafe extern "C" fn FIO_analyzeFrames(
    mut info: *mut fileInfo_t,
    srcFile: *mut FILE,
) -> InfoError {
    loop {
        let mut headerBuffer: [BYTE; 18] = [0; 18];
        let numBytesRead = fread(
            headerBuffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<[BYTE; 18]>() as libc::c_ulong,
            srcFile,
        );
        if numBytesRead
            < (if ZSTD_f_zstd1 as libc::c_int == ZSTD_f_zstd1 as libc::c_int {
                6 as libc::c_int
            } else {
                2 as libc::c_int
            }) as libc::c_ulong
        {
            if feof(srcFile) != 0 && numBytesRead == 0 as libc::c_int as libc::c_ulong
                && (*info).compressedSize > 0 as libc::c_int as libc::c_ulong
                && (*info).compressedSize != UTIL_FILESIZE_UNKNOWN as U64
            {
                let mut file_position = ftell(srcFile) as libc::c_ulonglong;
                let mut file_size = (*info).compressedSize as libc::c_ulonglong;
                if file_position != file_size {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Error: seeked to position %llu, which is beyond file size of %llu\n\0"
                                as *const u8 as *const libc::c_char,
                            file_position,
                            file_size,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    }
                    return info_truncated_input;
                }
                break;
            } else {
                if feof(srcFile) != 0 {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Error: reached end of file with incomplete frame\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    }
                    return info_not_zstd;
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error: did not reach end of file but ran out of frames\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                return info_frame_error;
            }
        } else {
            let magicNumber = MEM_readLE32(
                headerBuffer.as_mut_ptr() as *const libc::c_void,
            );
            if magicNumber == ZSTD_MAGICNUMBER {
                let mut header = ZSTD_frameHeader {
                    frameContentSize: 0,
                    windowSize: 0,
                    blockSizeMax: 0,
                    frameType: ZSTD_frame,
                    headerSize: 0,
                    dictID: 0,
                    checksumFlag: 0,
                    _reserved1: 0,
                    _reserved2: 0,
                };
                let frameContentSize = ZSTD_getFrameContentSize(
                    headerBuffer.as_mut_ptr() as *const libc::c_void,
                    numBytesRead,
                ) as U64;
                if frameContentSize as libc::c_ulonglong == ZSTD_CONTENTSIZE_ERROR
                    || frameContentSize as libc::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
                {
                    (*info).decompUnavailable = 1 as libc::c_int;
                } else {
                    (*info)
                        .decompressedSize = ((*info).decompressedSize as libc::c_ulong)
                        .wrapping_add(frameContentSize) as U64 as U64;
                }
                if ZSTD_getFrameHeader(
                    &mut header,
                    headerBuffer.as_mut_ptr() as *const libc::c_void,
                    numBytesRead,
                ) != 0 as libc::c_int as libc::c_ulong
                {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not decode frame header\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    }
                    return info_frame_error;
                }
                if (*info).dictID != 0 as libc::c_int as libc::c_uint
                    && (*info).dictID != header.dictID
                {
                    fprintf(
                        stderr,
                        b"WARNING: File contains multiple frames with different dictionary IDs. Showing dictID 0 instead\0"
                            as *const u8 as *const libc::c_char,
                    );
                    (*info).dictID = 0 as libc::c_int as libc::c_uint;
                } else {
                    (*info).dictID = header.dictID;
                }
                (*info).windowSize = header.windowSize as U64;
                let headerSize = ZSTD_frameHeaderSize(
                    headerBuffer.as_mut_ptr() as *const libc::c_void,
                    numBytesRead,
                );
                if ZSTD_isError(headerSize) != 0 {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not determine frame header size\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    }
                    return info_frame_error;
                }
                if fseek(
                    srcFile,
                    headerSize as libc::c_long - numBytesRead as libc::c_long,
                    1 as libc::c_int,
                ) != 0 as libc::c_int
                {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not move to end of frame header\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    }
                    return info_frame_error;
                }
                let mut lastBlock = 0 as libc::c_int;
                loop {
                    let mut blockHeaderBuffer: [BYTE; 3] = [0; 3];
                    if fread(
                        blockHeaderBuffer.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        3 as libc::c_int as libc::c_ulong,
                        srcFile,
                    ) != 3 as libc::c_int as libc::c_ulong
                    {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Error while reading block header\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b" \n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        return info_frame_error;
                    }
                    let blockHeader = MEM_readLE24(
                        blockHeaderBuffer.as_mut_ptr() as *const libc::c_void,
                    );
                    let blockTypeID = blockHeader >> 1 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint;
                    let isRLE = (blockTypeID == 1 as libc::c_int as libc::c_uint)
                        as libc::c_int as U32;
                    let isWrongBlock = (blockTypeID == 3 as libc::c_int as libc::c_uint)
                        as libc::c_int as U32;
                    let blockSize = if isRLE != 0 {
                        1 as libc::c_int as libc::c_long
                    } else {
                        (blockHeader >> 3 as libc::c_int) as libc::c_long
                    };
                    if isWrongBlock != 0 {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Error: unsupported block type\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b" \n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        return info_frame_error;
                    }
                    lastBlock = (blockHeader & 1 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    if fseek(srcFile, blockSize, 1 as libc::c_int) != 0 as libc::c_int {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Error: could not skip to end of block\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b" \n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        return info_frame_error;
                    }
                    if !(lastBlock != 1 as libc::c_int) {
                        break;
                    }
                }
                let frameHeaderDescriptor = headerBuffer[4 as libc::c_int as usize];
                let contentChecksumFlag = (frameHeaderDescriptor as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int) >> 2 as libc::c_int;
                if contentChecksumFlag != 0 {
                    (*info).usesCheck = 1 as libc::c_int;
                    if fread(
                        ((*info).checksum).as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        4 as libc::c_int as libc::c_ulong,
                        srcFile,
                    ) != 4 as libc::c_int as libc::c_ulong
                    {
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Error: could not read checksum\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b" \n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        return info_frame_error;
                    }
                }
                (*info).numActualFrames += 1;
            } else if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK
                == ZSTD_MAGIC_SKIPPABLE_START as libc::c_uint
            {
                let frameSize = MEM_readLE32(
                    headerBuffer.as_mut_ptr().offset(4 as libc::c_int as isize)
                        as *const libc::c_void,
                );
                let seek = ((8 as libc::c_int as libc::c_uint).wrapping_add(frameSize)
                    as libc::c_ulong)
                    .wrapping_sub(numBytesRead) as libc::c_long;
                if fseek(srcFile, seek, 1 as libc::c_int) != 0 as libc::c_int {
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not find end of skippable frame\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as libc::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                    }
                    return info_frame_error;
                }
                (*info).numSkippableFrames += 1;
            } else {
                return info_not_zstd
            }
        }
    }
    return info_success;
}
unsafe extern "C" fn getFileInfo_fileConfirmed(
    mut info: *mut fileInfo_t,
    mut inFileName: *const libc::c_char,
) -> InfoError {
    let mut status = info_success;
    let mut srcFileStat = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let srcFile = FIO_openSrcFile(
        NULL as *const FIO_prefs_t,
        inFileName,
        &mut srcFileStat,
    );
    if srcFile.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error: could not open source file %s\0" as *const u8
                    as *const libc::c_char,
                inFileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        return info_file_error;
    }
    (*info).compressedSize = UTIL_getFileSizeStat(&mut srcFileStat);
    status = FIO_analyzeFrames(info, srcFile);
    fclose(srcFile);
    (*info).nbFiles = 1 as libc::c_int as U32;
    return status;
}
unsafe extern "C" fn getFileInfo(
    mut info: *mut fileInfo_t,
    mut srcFileName: *const libc::c_char,
) -> InfoError {
    if UTIL_isRegularFile(srcFileName) == 0 {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s is not a file\0" as *const u8 as *const libc::c_char,
                srcFileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
        }
        return info_file_error;
    }
    return getFileInfo_fileConfirmed(info, srcFileName);
}
unsafe extern "C" fn displayInfo(
    mut inFileName: *const libc::c_char,
    mut info: *const fileInfo_t,
    mut displayLevel: libc::c_int,
) {
    let window_hrs = UTIL_makeHumanReadableSize((*info).windowSize);
    let compressed_hrs = UTIL_makeHumanReadableSize((*info).compressedSize);
    let decompressed_hrs = UTIL_makeHumanReadableSize((*info).decompressedSize);
    let ratio = if (*info).compressedSize == 0 as libc::c_int as libc::c_ulong {
        0 as libc::c_int as libc::c_double
    } else {
        (*info).decompressedSize as libc::c_double
            / (*info).compressedSize as libc::c_double
    };
    let checkString = if (*info).usesCheck != 0 {
        b"XXH64\0" as *const u8 as *const libc::c_char
    } else {
        b"None\0" as *const u8 as *const libc::c_char
    };
    if displayLevel <= 2 as libc::c_int {
        if (*info).decompUnavailable == 0 {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s  %8.*f%4s  %5.3f  %5s  %s\n\0" as *const u8
                    as *const libc::c_char,
                (*info).numSkippableFrames + (*info).numActualFrames,
                (*info).numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                ratio,
                checkString,
                inFileName,
            );
        } else {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s                       %5s  %s\n\0" as *const u8
                    as *const libc::c_char,
                (*info).numSkippableFrames + (*info).numActualFrames,
                (*info).numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                checkString,
                inFileName,
            );
        }
    } else {
        fprintf(stdout, b"%s \n\0" as *const u8 as *const libc::c_char, inFileName);
        fprintf(
            stdout,
            b"# Zstandard Frames: %d\n\0" as *const u8 as *const libc::c_char,
            (*info).numActualFrames,
        );
        if (*info).numSkippableFrames != 0 {
            fprintf(
                stdout,
                b"# Skippable Frames: %d\n\0" as *const u8 as *const libc::c_char,
                (*info).numSkippableFrames,
            );
        }
        fprintf(
            stdout,
            b"DictID: %u\n\0" as *const u8 as *const libc::c_char,
            (*info).dictID,
        );
        fprintf(
            stdout,
            b"Window Size: %.*f%s (%llu B)\n\0" as *const u8 as *const libc::c_char,
            window_hrs.precision,
            window_hrs.value,
            window_hrs.suffix,
            (*info).windowSize as libc::c_ulonglong,
        );
        fprintf(
            stdout,
            b"Compressed Size: %.*f%s (%llu B)\n\0" as *const u8 as *const libc::c_char,
            compressed_hrs.precision,
            compressed_hrs.value,
            compressed_hrs.suffix,
            (*info).compressedSize as libc::c_ulonglong,
        );
        if (*info).decompUnavailable == 0 {
            fprintf(
                stdout,
                b"Decompressed Size: %.*f%s (%llu B)\n\0" as *const u8
                    as *const libc::c_char,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                (*info).decompressedSize as libc::c_ulonglong,
            );
            fprintf(
                stdout,
                b"Ratio: %.4f\n\0" as *const u8 as *const libc::c_char,
                ratio,
            );
        }
        if (*info).usesCheck != 0 && (*info).numActualFrames == 1 as libc::c_int {
            fprintf(
                stdout,
                b"Check: %s %02x%02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
                checkString,
                (*info).checksum[3 as libc::c_int as usize] as libc::c_int,
                (*info).checksum[2 as libc::c_int as usize] as libc::c_int,
                (*info).checksum[1 as libc::c_int as usize] as libc::c_int,
                (*info).checksum[0 as libc::c_int as usize] as libc::c_int,
            );
        } else {
            fprintf(
                stdout,
                b"Check: %s\n\0" as *const u8 as *const libc::c_char,
                checkString,
            );
        }
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn FIO_addFInfo(
    mut fi1: fileInfo_t,
    mut fi2: fileInfo_t,
) -> fileInfo_t {
    let mut total = fileInfo_t {
        decompressedSize: 0,
        compressedSize: 0,
        windowSize: 0,
        numActualFrames: 0,
        numSkippableFrames: 0,
        decompUnavailable: 0,
        usesCheck: 0,
        checksum: [0; 4],
        nbFiles: 0,
        dictID: 0,
    };
    memset(
        &mut total as *mut fileInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<fileInfo_t>() as libc::c_ulong,
    );
    total.numActualFrames = fi1.numActualFrames + fi2.numActualFrames;
    total.numSkippableFrames = fi1.numSkippableFrames + fi2.numSkippableFrames;
    total.compressedSize = (fi1.compressedSize).wrapping_add(fi2.compressedSize);
    total.decompressedSize = (fi1.decompressedSize).wrapping_add(fi2.decompressedSize);
    total.decompUnavailable = fi1.decompUnavailable | fi2.decompUnavailable;
    total.usesCheck = fi1.usesCheck & fi2.usesCheck;
    total.nbFiles = (fi1.nbFiles).wrapping_add(fi2.nbFiles);
    return total;
}
unsafe extern "C" fn FIO_listFile(
    mut total: *mut fileInfo_t,
    mut inFileName: *const libc::c_char,
    mut displayLevel: libc::c_int,
) -> libc::c_int {
    let mut info = fileInfo_t {
        decompressedSize: 0,
        compressedSize: 0,
        windowSize: 0,
        numActualFrames: 0,
        numSkippableFrames: 0,
        decompUnavailable: 0,
        usesCheck: 0,
        checksum: [0; 4],
        nbFiles: 0,
        dictID: 0,
    };
    memset(
        &mut info as *mut fileInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<fileInfo_t>() as libc::c_ulong,
    );
    let error = getFileInfo(&mut info, inFileName);
    match error as libc::c_uint {
        1 => {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error while parsing \"%s\" \n\0" as *const u8
                        as *const libc::c_char,
                    inFileName,
                );
            }
        }
        2 => {
            fprintf(
                stdout,
                b"File \"%s\" not compressed by zstd \n\0" as *const u8
                    as *const libc::c_char,
                inFileName,
            );
            if displayLevel > 2 as libc::c_int {
                fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int;
        }
        3 => {
            if displayLevel > 2 as libc::c_int {
                fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int;
        }
        4 => {
            fprintf(
                stdout,
                b"File \"%s\" is truncated \n\0" as *const u8 as *const libc::c_char,
                inFileName,
            );
            if displayLevel > 2 as libc::c_int {
                fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int;
        }
        0 | _ => {}
    }
    displayInfo(inFileName, &mut info, displayLevel);
    *total = FIO_addFInfo(*total, info);
    if error as libc::c_uint == info_success as libc::c_int as libc::c_uint
        || error as libc::c_uint == info_frame_error as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"error == info_success || error == info_frame_error\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio.c\0" as *const u8
                as *const libc::c_char,
            3305 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int FIO_listFile(fileInfo_t *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    return error as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_listMultipleFiles(
    mut numFiles: libc::c_uint,
    mut filenameTable: *mut *const libc::c_char,
    mut displayLevel: libc::c_int,
) -> libc::c_int {
    let mut u: libc::c_uint = 0;
    u = 0 as libc::c_int as libc::c_uint;
    while u < numFiles {
        if strcmp(
            *filenameTable.offset(u as isize),
            b"/*stdin*\\\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: --list does not support reading from standard input\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int;
        }
        u = u.wrapping_add(1);
    }
    if numFiles == 0 as libc::c_int as libc::c_uint {
        if UTIL_isConsole(stdin) == 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"zstd: --list does not support reading from standard input \n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"No files given \n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int;
    }
    if displayLevel <= 2 as libc::c_int {
        fprintf(
            stdout,
            b"Frames  Skips  Compressed  Uncompressed  Ratio  Check  Filename\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut error = 0 as libc::c_int;
    let mut total = fileInfo_t {
        decompressedSize: 0,
        compressedSize: 0,
        windowSize: 0,
        numActualFrames: 0,
        numSkippableFrames: 0,
        decompUnavailable: 0,
        usesCheck: 0,
        checksum: [0; 4],
        nbFiles: 0,
        dictID: 0,
    };
    memset(
        &mut total as *mut fileInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<fileInfo_t>() as libc::c_ulong,
    );
    total.usesCheck = 1 as libc::c_int;
    let mut u_0: libc::c_uint = 0;
    u_0 = 0 as libc::c_int as libc::c_uint;
    while u_0 < numFiles {
        error
            |= FIO_listFile(
                &mut total,
                *filenameTable.offset(u_0 as isize),
                displayLevel,
            );
        u_0 = u_0.wrapping_add(1);
    }
    if numFiles > 1 as libc::c_int as libc::c_uint && displayLevel <= 2 as libc::c_int {
        let compressed_hrs = UTIL_makeHumanReadableSize(total.compressedSize);
        let decompressed_hrs = UTIL_makeHumanReadableSize(total.decompressedSize);
        let ratio = if total.compressedSize == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int as libc::c_double
        } else {
            total.decompressedSize as libc::c_double
                / total.compressedSize as libc::c_double
        };
        let checkString = if total.usesCheck != 0 {
            b"XXH64\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        fprintf(
            stdout,
            b"----------------------------------------------------------------- \n\0"
                as *const u8 as *const libc::c_char,
        );
        if total.decompUnavailable != 0 {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s                       %5s  %u files\n\0"
                    as *const u8 as *const libc::c_char,
                total.numSkippableFrames + total.numActualFrames,
                total.numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                checkString,
                total.nbFiles,
            );
        } else {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s  %8.*f%4s  %5.3f  %5s  %u files\n\0" as *const u8
                    as *const libc::c_char,
                total.numSkippableFrames + total.numActualFrames,
                total.numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                ratio,
                checkString,
                total.nbFiles,
            );
        }
    }
    return error;
}
