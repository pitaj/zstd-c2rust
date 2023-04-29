use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type POOL_ctx_s;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn POOL_create(numThreads: size_t, queueSize: size_t) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_joinJobs(ctx: *mut POOL_ctx);
    fn POOL_add(ctx: *mut POOL_ctx, function: POOL_function, opaque: *mut libc::c_void);
    static mut g_display_prefs: FIO_display_prefs_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type U8 = uint8_t;
pub type U64 = uint64_t;
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
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
pub type POOL_ctx = POOL_ctx_s;
pub type POOL_function = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type ZSTD_pthread_mutex_t = libc::c_int;
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
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const MAX_IO_JOBS: libc::c_int = 10 as libc::c_int;
pub const LONG_SEEK: unsafe extern "C" fn(
    *mut FILE,
    libc::c_long,
    libc::c_int,
) -> libc::c_int = fseek;
static mut segmentSizeT: size_t = 0;
static mut maskT: size_t = 0;
unsafe extern "C" fn AIO_fwriteSparse(
    mut file: *mut FILE,
    mut buffer: *const libc::c_void,
    mut bufferSize: size_t,
    prefs: *const FIO_prefs_t,
    mut storedSkips: libc::c_uint,
) -> libc::c_uint {
    let bufferT = buffer as *const size_t;
    let mut bufferSizeT = bufferSize
        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong);
    let bufferTEnd = bufferT.offset(bufferSizeT as isize);
    let mut ptrT = bufferT;
    if (*prefs).testMode != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    if (*prefs).sparseFileSupport == 0 {
        let sizeCheck = fwrite(
            buffer,
            1 as libc::c_int as libc::c_ulong,
            bufferSize,
            file,
        );
        if sizeCheck != bufferSize {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    50 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Write error : cannot write block : %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(70 as libc::c_int);
        }
        return 0 as libc::c_int as libc::c_uint;
    }
    if storedSkips
        > (1 as libc::c_int as libc::c_uint)
            .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int)
    {
        if fseek(
            file,
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int) as libc::c_long,
            SEEK_CUR,
        ) != 0 as libc::c_int
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    57 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    91 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"1 GB skip error (sparse file support)\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(91 as libc::c_int);
        }
        storedSkips = storedSkips
            .wrapping_sub(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int),
            );
    }
    while ptrT < bufferTEnd {
        let mut nb0T: size_t = 0;
        let mut seg0SizeT = segmentSizeT;
        if seg0SizeT > bufferSizeT {
            seg0SizeT = bufferSizeT;
        }
        bufferSizeT = (bufferSizeT as libc::c_ulong).wrapping_sub(seg0SizeT) as size_t
            as size_t;
        nb0T = 0 as libc::c_int as size_t;
        while nb0T < seg0SizeT
            && *ptrT.offset(nb0T as isize) == 0 as libc::c_int as libc::c_ulong
        {
            nb0T = nb0T.wrapping_add(1);
        }
        storedSkips = storedSkips
            .wrapping_add(
                nb0T.wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    as libc::c_uint,
            );
        if nb0T != seg0SizeT {
            let nbNon0ST = seg0SizeT.wrapping_sub(nb0T);
            if fseek(file, storedSkips as libc::c_long, SEEK_CUR) != 0 as libc::c_int {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                            as *const u8 as *const libc::c_char,
                        77 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        92 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Sparse skip error ; try --no-sparse\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(92 as libc::c_int);
            }
            storedSkips = 0 as libc::c_int as libc::c_uint;
            if fwrite(
                ptrT.offset(nb0T as isize) as *const libc::c_void,
                ::core::mem::size_of::<size_t>() as libc::c_ulong,
                nbNon0ST,
                file,
            ) != nbNon0ST
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                            as *const u8 as *const libc::c_char,
                        82 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        93 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Write error : cannot write block : %s\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(93 as libc::c_int);
            }
        }
        ptrT = ptrT.offset(seg0SizeT as isize);
    }
    if bufferSize & maskT != 0 {
        let restStart = bufferTEnd as *const libc::c_char;
        let mut restPtr = restStart;
        let restEnd = (buffer as *const libc::c_char).offset(bufferSize as isize);
        if restEnd > restStart
            && restEnd
                < restStart
                    .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize)
        {} else {
            __assert_fail(
                b"restEnd > restStart && restEnd < restStart + sizeof(size_t)\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"unsigned int AIO_fwriteSparse(FILE *, const void *, size_t, const FIO_prefs_t *const, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        while restPtr < restEnd && *restPtr as libc::c_int == 0 as libc::c_int {
            restPtr = restPtr.offset(1);
        }
        storedSkips = storedSkips
            .wrapping_add(
                restPtr.offset_from(restStart) as libc::c_long as libc::c_uint,
            );
        if restPtr != restEnd {
            let restSize = restEnd.offset_from(restPtr) as libc::c_long as size_t;
            if fseek(file, storedSkips as libc::c_long, SEEK_CUR) != 0 as libc::c_int {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                            as *const u8 as *const libc::c_char,
                        100 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        92 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Sparse skip error ; try --no-sparse\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(92 as libc::c_int);
            }
            if fwrite(
                restPtr as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                restSize,
                file,
            ) != restSize
            {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                            as *const u8 as *const libc::c_char,
                        103 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        95 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Write error : cannot write end of decoded block : %s\0"
                            as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(95 as libc::c_int);
            }
            storedSkips = 0 as libc::c_int as libc::c_uint;
        }
    }
    return storedSkips;
}
unsafe extern "C" fn AIO_fwriteSparseEnd(
    prefs: *const FIO_prefs_t,
    mut file: *mut FILE,
    mut storedSkips: libc::c_uint,
) {
    if (*prefs).testMode != 0 {
        if storedSkips == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"storedSkips == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                113 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void AIO_fwriteSparseEnd(const FIO_prefs_t *const, FILE *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if storedSkips > 0 as libc::c_int as libc::c_uint {
        if (*prefs).sparseFileSupport > 0 as libc::c_int {} else {
            __assert_fail(
                b"prefs->sparseFileSupport > 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void AIO_fwriteSparseEnd(const FIO_prefs_t *const, FILE *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        if fseek(
            file,
            storedSkips.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_long,
            SEEK_CUR,
        ) != 0 as libc::c_int
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    118 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    69 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Final skip error (sparse file support)\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(69 as libc::c_int);
        }
        let lastZeroByte: [libc::c_char; 1] = [0 as libc::c_int as libc::c_char];
        if fwrite(
            lastZeroByte.as_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    123 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    69 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Write error : cannot write last zero : %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(69 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_supported() -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn AIO_IOPool_createIoJob(
    mut ctx: *mut IOPoolCtx_t,
    mut bufferSize: size_t,
) -> *mut IOJob_t {
    let job = malloc(::core::mem::size_of::<IOJob_t>() as libc::c_ulong) as *mut IOJob_t;
    let buffer = malloc(bufferSize);
    if job.is_null() || buffer.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                150 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                101 as libc::c_int,
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
        exit(101 as libc::c_int);
    }
    (*job).buffer = buffer;
    (*job).bufferSize = bufferSize;
    (*job).usedBufferSize = 0 as libc::c_int as size_t;
    (*job).file = NULL as *mut FILE;
    (*job).ctx = ctx as *mut libc::c_void;
    (*job).offset = 0 as libc::c_int as U64;
    return job;
}
unsafe extern "C" fn AIO_IOPool_createThreadPool(
    mut ctx: *mut IOPoolCtx_t,
    mut prefs: *const FIO_prefs_t,
) {
    (*ctx).threadPool = NULL as *mut POOL_ctx;
    (*ctx).threadPoolActive = 0 as libc::c_int;
    if (*prefs).asyncIO != 0 {
        if 0 as libc::c_int != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    169 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    102 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Failed creating ioJobsMutex mutex\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(102 as libc::c_int);
        }
        if 10 as libc::c_int >= 2 as libc::c_int {} else {
            __assert_fail(
                b"MAX_IO_JOBS >= 2\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void AIO_IOPool_createThreadPool(IOPoolCtx_t *, const FIO_prefs_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        (*ctx)
            .threadPool = POOL_create(
            1 as libc::c_int as size_t,
            (MAX_IO_JOBS - 2 as libc::c_int) as size_t,
        );
        (*ctx).threadPoolActive = 1 as libc::c_int;
        if ((*ctx).threadPool).is_null() {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    176 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    104 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Failed creating I/O thread pool\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(104 as libc::c_int);
        }
    }
}
unsafe extern "C" fn AIO_IOPool_init(
    mut ctx: *mut IOPoolCtx_t,
    mut prefs: *const FIO_prefs_t,
    mut poolFunction: POOL_function,
    mut bufferSize: size_t,
) {
    let mut i: libc::c_int = 0;
    AIO_IOPool_createThreadPool(ctx, prefs);
    (*ctx).prefs = prefs;
    (*ctx).poolFunction = poolFunction;
    (*ctx)
        .totalIoJobs = if !((*ctx).threadPool).is_null() {
        MAX_IO_JOBS
    } else {
        2 as libc::c_int
    };
    (*ctx).availableJobsCount = (*ctx).totalIoJobs;
    i = 0 as libc::c_int;
    while i < (*ctx).availableJobsCount {
        (*ctx)
            .availableJobs[i
            as usize] = AIO_IOPool_createIoJob(ctx, bufferSize) as *mut libc::c_void;
        i += 1;
    }
    (*ctx).jobBufferSize = bufferSize;
    (*ctx).file = NULL as *mut FILE;
}
unsafe extern "C" fn AIO_IOPool_threadPoolActive(
    mut ctx: *mut IOPoolCtx_t,
) -> libc::c_int {
    return (!((*ctx).threadPool).is_null() && (*ctx).threadPoolActive != 0)
        as libc::c_int;
}
unsafe extern "C" fn AIO_IOPool_lockJobsMutex(mut ctx: *mut IOPoolCtx_t) {
    AIO_IOPool_threadPoolActive(ctx) != 0;
}
unsafe extern "C" fn AIO_IOPool_unlockJobsMutex(mut ctx: *mut IOPoolCtx_t) {
    AIO_IOPool_threadPoolActive(ctx) != 0;
}
unsafe extern "C" fn AIO_IOPool_releaseIoJob(mut job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut IOPoolCtx_t;
    AIO_IOPool_lockJobsMutex(ctx);
    if (*ctx).availableJobsCount < (*ctx).totalIoJobs {} else {
        __assert_fail(
            b"ctx->availableJobsCount < ctx->totalIoJobs\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void AIO_IOPool_releaseIoJob(IOJob_t *)\0"))
                .as_ptr(),
        );
    }
    let fresh0 = (*ctx).availableJobsCount;
    (*ctx).availableJobsCount = (*ctx).availableJobsCount + 1;
    (*ctx).availableJobs[fresh0 as usize] = job as *mut libc::c_void;
    AIO_IOPool_unlockJobsMutex(ctx);
}
unsafe extern "C" fn AIO_IOPool_join(mut ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        POOL_joinJobs((*ctx).threadPool);
    }
}
unsafe extern "C" fn AIO_IOPool_setThreaded(
    mut ctx: *mut IOPoolCtx_t,
    mut threaded: libc::c_int,
) {
    if threaded == 0 as libc::c_int || threaded == 1 as libc::c_int {} else {
        __assert_fail(
            b"threaded == 0 || threaded == 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void AIO_IOPool_setThreaded(IOPoolCtx_t *, int)\0"))
                .as_ptr(),
        );
    }
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            241 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void AIO_IOPool_setThreaded(IOPoolCtx_t *, int)\0"))
                .as_ptr(),
        );
    }
    if (*ctx).threadPoolActive != threaded {
        AIO_IOPool_join(ctx);
        (*ctx).threadPoolActive = threaded;
    }
}
unsafe extern "C" fn AIO_IOPool_destroy(mut ctx: *mut IOPoolCtx_t) {
    let mut i: libc::c_int = 0;
    if !((*ctx).threadPool).is_null() {
        AIO_IOPool_join(ctx);
        if (*ctx).availableJobsCount == (*ctx).totalIoJobs {} else {
            __assert_fail(
                b"ctx->availableJobsCount == ctx->totalIoJobs\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                256 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void AIO_IOPool_destroy(IOPoolCtx_t *)\0"))
                    .as_ptr(),
            );
        }
        POOL_free((*ctx).threadPool);
    }
    if ((*ctx).file).is_null() {} else {
        __assert_fail(
            b"ctx->file == NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            260 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void AIO_IOPool_destroy(IOPoolCtx_t *)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < (*ctx).availableJobsCount {
        let mut job = (*ctx).availableJobs[i as usize] as *mut IOJob_t;
        free((*job).buffer);
        free(job as *mut libc::c_void);
        i += 1;
    }
}
unsafe extern "C" fn AIO_IOPool_acquireJob(mut ctx: *mut IOPoolCtx_t) -> *mut IOJob_t {
    let mut job = 0 as *mut IOJob_t;
    if !((*ctx).file).is_null() || (*(*ctx).prefs).testMode != 0 {} else {
        __assert_fail(
            b"ctx->file != NULL || ctx->prefs->testMode\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            272 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"IOJob_t *AIO_IOPool_acquireJob(IOPoolCtx_t *)\0"))
                .as_ptr(),
        );
    }
    AIO_IOPool_lockJobsMutex(ctx);
    if (*ctx).availableJobsCount > 0 as libc::c_int {} else {
        __assert_fail(
            b"ctx->availableJobsCount > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"IOJob_t *AIO_IOPool_acquireJob(IOPoolCtx_t *)\0"))
                .as_ptr(),
        );
    }
    (*ctx).availableJobsCount -= 1;
    job = (*ctx).availableJobs[(*ctx).availableJobsCount as usize] as *mut IOJob_t;
    AIO_IOPool_unlockJobsMutex(ctx);
    (*job).usedBufferSize = 0 as libc::c_int as size_t;
    (*job).file = (*ctx).file;
    (*job).offset = 0 as libc::c_int as U64;
    return job;
}
unsafe extern "C" fn AIO_IOPool_setFile(mut ctx: *mut IOPoolCtx_t, mut file: *mut FILE) {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx!=NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            288 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void AIO_IOPool_setFile(IOPoolCtx_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    AIO_IOPool_join(ctx);
    if (*ctx).availableJobsCount == (*ctx).totalIoJobs {} else {
        __assert_fail(
            b"ctx->availableJobsCount == ctx->totalIoJobs\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            290 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void AIO_IOPool_setFile(IOPoolCtx_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    (*ctx).file = file;
}
unsafe extern "C" fn AIO_IOPool_getFile(mut ctx: *const IOPoolCtx_t) -> *mut FILE {
    return (*ctx).file;
}
unsafe extern "C" fn AIO_IOPool_enqueueJob(mut job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut IOPoolCtx_t;
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        POOL_add((*ctx).threadPool, (*ctx).poolFunction, job as *mut libc::c_void);
    } else {
        ((*ctx).poolFunction)
            .expect("non-null function pointer")(job as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_acquireJob(
    mut ctx: *mut WritePoolCtx_t,
) -> *mut IOJob_t {
    return AIO_IOPool_acquireJob(&mut (*ctx).base);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_enqueueAndReacquireWriteJob(
    mut job: *mut *mut IOJob_t,
) {
    AIO_IOPool_enqueueJob(*job);
    *job = AIO_IOPool_acquireJob((**job).ctx as *mut IOPoolCtx_t);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_sparseWriteEnd(mut ctx: *mut WritePoolCtx_t) {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            333 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void AIO_WritePool_sparseWriteEnd(WritePoolCtx_t *)\0"))
                .as_ptr(),
        );
    }
    AIO_IOPool_join(&mut (*ctx).base);
    AIO_fwriteSparseEnd((*ctx).base.prefs, (*ctx).base.file, (*ctx).storedSkips);
    (*ctx).storedSkips = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_setFile(
    mut ctx: *mut WritePoolCtx_t,
    mut file: *mut FILE,
) {
    AIO_IOPool_setFile(&mut (*ctx).base, file);
    if (*ctx).storedSkips == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"ctx->storedSkips == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            345 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void AIO_WritePool_setFile(WritePoolCtx_t *, FILE *)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_getFile(
    mut ctx: *const WritePoolCtx_t,
) -> *mut FILE {
    return AIO_IOPool_getFile(&(*ctx).base);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_releaseIoJob(mut job: *mut IOJob_t) {
    AIO_IOPool_releaseIoJob(job);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_closeFile(
    mut ctx: *mut WritePoolCtx_t,
) -> libc::c_int {
    let dstFile = (*ctx).base.file;
    if !dstFile.is_null() || (*(*ctx).base.prefs).testMode != 0 as libc::c_int {} else {
        __assert_fail(
            b"dstFile!=NULL || ctx->base.prefs->testMode!=0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            365 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int AIO_WritePool_closeFile(WritePoolCtx_t *)\0"))
                .as_ptr(),
        );
    }
    AIO_WritePool_sparseWriteEnd(ctx);
    AIO_IOPool_setFile(&mut (*ctx).base, NULL as *mut FILE);
    return fclose(dstFile);
}
unsafe extern "C" fn AIO_WritePool_executeWriteJob(mut opaque: *mut libc::c_void) {
    let job = opaque as *mut IOJob_t;
    let ctx = (*job).ctx as *mut WritePoolCtx_t;
    (*ctx)
        .storedSkips = AIO_fwriteSparse(
        (*job).file,
        (*job).buffer,
        (*job).usedBufferSize,
        (*ctx).base.prefs,
        (*ctx).storedSkips,
    );
    AIO_IOPool_releaseIoJob(job);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_create(
    mut prefs: *const FIO_prefs_t,
    mut bufferSize: size_t,
) -> *mut WritePoolCtx_t {
    let ctx = malloc(::core::mem::size_of::<WritePoolCtx_t>() as libc::c_ulong)
        as *mut WritePoolCtx_t;
    if ctx.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                384 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
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
        exit(100 as libc::c_int);
    }
    AIO_IOPool_init(
        &mut (*ctx).base,
        prefs,
        Some(
            AIO_WritePool_executeWriteJob
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
        bufferSize,
    );
    (*ctx).storedSkips = 0 as libc::c_int as libc::c_uint;
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_free(mut ctx: *mut WritePoolCtx_t) {
    if !(AIO_WritePool_getFile(ctx)).is_null() {
        AIO_WritePool_closeFile(ctx);
    }
    AIO_IOPool_destroy(&mut (*ctx).base);
    if (*ctx).storedSkips == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"ctx->storedSkips==0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            397 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void AIO_WritePool_free(WritePoolCtx_t *)\0"))
                .as_ptr(),
        );
    }
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_setAsync(
    mut ctx: *mut WritePoolCtx_t,
    mut async_0: libc::c_int,
) {
    AIO_IOPool_setThreaded(&mut (*ctx).base, async_0);
}
unsafe extern "C" fn AIO_ReadPool_releaseAllCompletedJobs(mut ctx: *mut ReadPoolCtx_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ctx).completedJobsCount {
        let mut job = (*ctx).completedJobs[i as usize] as *mut IOJob_t;
        AIO_IOPool_releaseIoJob(job);
        i += 1;
    }
    (*ctx).completedJobsCount = 0 as libc::c_int;
}
unsafe extern "C" fn AIO_ReadPool_addJobToCompleted(mut job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut ReadPoolCtx_t;
    AIO_IOPool_lockJobsMutex(&mut (*ctx).base);
    if (*ctx).completedJobsCount < 10 as libc::c_int {} else {
        __assert_fail(
            b"ctx->completedJobsCount < MAX_IO_JOBS\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            424 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void AIO_ReadPool_addJobToCompleted(IOJob_t *)\0"))
                .as_ptr(),
        );
    }
    let fresh1 = (*ctx).completedJobsCount;
    (*ctx).completedJobsCount = (*ctx).completedJobsCount + 1;
    (*ctx).completedJobs[fresh1 as usize] = job as *mut libc::c_void;
    AIO_IOPool_threadPoolActive(&mut (*ctx).base) != 0;
    AIO_IOPool_unlockJobsMutex(&mut (*ctx).base);
}
unsafe extern "C" fn AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(
    mut ctx: *mut ReadPoolCtx_t,
) -> *mut IOJob_t {
    let mut job = NULL as *mut IOJob_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ctx).completedJobsCount {
        job = (*ctx).completedJobs[i as usize] as *mut IOJob_t;
        if (*job).offset == (*ctx).waitingOnOffset {
            (*ctx).completedJobsCount -= 1;
            (*ctx)
                .completedJobs[i
                as usize] = (*ctx).completedJobs[(*ctx).completedJobsCount as usize];
            return job;
        }
        i += 1;
    }
    return NULL as *mut IOJob_t;
}
unsafe extern "C" fn AIO_ReadPool_numReadsInFlight(
    mut ctx: *mut ReadPoolCtx_t,
) -> size_t {
    let jobsHeld = (if ((*ctx).currentJobHeld).is_null() {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as size_t;
    return ((*ctx).base.totalIoJobs as libc::c_ulong)
        .wrapping_sub(
            (((*ctx).base.availableJobsCount + (*ctx).completedJobsCount)
                as libc::c_ulong)
                .wrapping_add(jobsHeld),
        );
}
unsafe extern "C" fn AIO_ReadPool_getNextCompletedJob(
    mut ctx: *mut ReadPoolCtx_t,
) -> *mut IOJob_t {
    let mut job = NULL as *mut IOJob_t;
    AIO_IOPool_lockJobsMutex(&mut (*ctx).base);
    job = AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(ctx);
    while job.is_null()
        && AIO_ReadPool_numReadsInFlight(ctx) > 0 as libc::c_int as libc::c_ulong
    {
        if !((*ctx).base.threadPool).is_null() {} else {
            __assert_fail(
                b"ctx->base.threadPool != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                471 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"IOJob_t *AIO_ReadPool_getNextCompletedJob(ReadPoolCtx_t *)\0"))
                    .as_ptr(),
            );
        }
        job = AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(ctx);
    }
    if !job.is_null() {
        if (*job).offset == (*ctx).waitingOnOffset {} else {
            __assert_fail(
                b"job->offset == ctx->waitingOnOffset\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                477 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"IOJob_t *AIO_ReadPool_getNextCompletedJob(ReadPoolCtx_t *)\0"))
                    .as_ptr(),
            );
        }
        (*ctx)
            .waitingOnOffset = ((*ctx).waitingOnOffset as libc::c_ulong)
            .wrapping_add((*job).usedBufferSize) as U64 as U64;
    }
    AIO_IOPool_unlockJobsMutex(&mut (*ctx).base);
    return job;
}
unsafe extern "C" fn AIO_ReadPool_executeReadJob(mut opaque: *mut libc::c_void) {
    let job = opaque as *mut IOJob_t;
    let ctx = (*job).ctx as *mut ReadPoolCtx_t;
    if (*ctx).reachedEof != 0 {
        (*job).usedBufferSize = 0 as libc::c_int as size_t;
        AIO_ReadPool_addJobToCompleted(job);
        return;
    }
    (*job)
        .usedBufferSize = fread(
        (*job).buffer,
        1 as libc::c_int as libc::c_ulong,
        (*job).bufferSize,
        (*job).file,
    );
    if (*job).usedBufferSize < (*job).bufferSize {
        if ferror((*job).file) != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    499 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    37 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"Read error\0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(37 as libc::c_int);
        } else {
            if feof((*job).file) != 0 {
                (*ctx).reachedEof = 1 as libc::c_int;
            } else {
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                            as *const u8 as *const libc::c_char,
                        503 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const libc::c_char,
                        37 as libc::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Unexpected short read\0" as *const u8 as *const libc::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as libc::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
                }
                exit(37 as libc::c_int);
            }
        }
    }
    AIO_ReadPool_addJobToCompleted(job);
}
unsafe extern "C" fn AIO_ReadPool_enqueueRead(mut ctx: *mut ReadPoolCtx_t) {
    let job = AIO_IOPool_acquireJob(&mut (*ctx).base);
    (*job).offset = (*ctx).nextReadOffset;
    (*ctx)
        .nextReadOffset = ((*ctx).nextReadOffset as libc::c_ulong)
        .wrapping_add((*job).bufferSize) as U64 as U64;
    AIO_IOPool_enqueueJob(job);
}
unsafe extern "C" fn AIO_ReadPool_startReading(mut ctx: *mut ReadPoolCtx_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ctx).base.availableJobsCount {
        AIO_ReadPool_enqueueRead(ctx);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_setFile(
    mut ctx: *mut ReadPoolCtx_t,
    mut file: *mut FILE,
) {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx!=NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            527 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void AIO_ReadPool_setFile(ReadPoolCtx_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    AIO_IOPool_join(&mut (*ctx).base);
    AIO_ReadPool_releaseAllCompletedJobs(ctx);
    if !((*ctx).currentJobHeld).is_null() {
        AIO_IOPool_releaseIoJob((*ctx).currentJobHeld as *mut IOJob_t);
        (*ctx).currentJobHeld = NULL as *mut libc::c_void;
    }
    AIO_IOPool_setFile(&mut (*ctx).base, file);
    (*ctx).nextReadOffset = 0 as libc::c_int as U64;
    (*ctx).waitingOnOffset = 0 as libc::c_int as U64;
    (*ctx).srcBuffer = (*ctx).coalesceBuffer;
    (*ctx).srcBufferLoaded = 0 as libc::c_int as size_t;
    (*ctx).reachedEof = 0 as libc::c_int;
    if !file.is_null() {
        AIO_ReadPool_startReading(ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_create(
    mut prefs: *const FIO_prefs_t,
    mut bufferSize: size_t,
) -> *mut ReadPoolCtx_t {
    let ctx = malloc(::core::mem::size_of::<ReadPoolCtx_t>() as libc::c_ulong)
        as *mut ReadPoolCtx_t;
    if ctx.is_null() {
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as libc::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                550 as libc::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
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
        exit(100 as libc::c_int);
    }
    AIO_IOPool_init(
        &mut (*ctx).base,
        prefs,
        Some(
            AIO_ReadPool_executeReadJob as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
        bufferSize,
    );
    (*ctx)
        .coalesceBuffer = malloc(
        bufferSize.wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut U8;
    (*ctx).srcBuffer = (*ctx).coalesceBuffer;
    (*ctx).srcBufferLoaded = 0 as libc::c_int as size_t;
    (*ctx).completedJobsCount = 0 as libc::c_int;
    (*ctx).currentJobHeld = NULL as *mut libc::c_void;
    if !((*ctx).base.threadPool).is_null() {
        if 0 as libc::c_int != 0 {
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0"
                        as *const u8 as *const libc::c_char,
                    561 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const libc::c_char,
                    103 as libc::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Failed creating jobCompletedCond cond\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as libc::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
            }
            exit(103 as libc::c_int);
        }
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_free(mut ctx: *mut ReadPoolCtx_t) {
    if !(AIO_ReadPool_getFile(ctx)).is_null() {
        AIO_ReadPool_closeFile(ctx);
    }
    !((*ctx).base.threadPool).is_null();
    AIO_IOPool_destroy(&mut (*ctx).base);
    free((*ctx).coalesceBuffer as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_consumeBytes(
    mut ctx: *mut ReadPoolCtx_t,
    mut n: size_t,
) {
    if n <= (*ctx).srcBufferLoaded {} else {
        __assert_fail(
            b"n <= ctx->srcBufferLoaded\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                as *const libc::c_char,
            581 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void AIO_ReadPool_consumeBytes(ReadPoolCtx_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    (*ctx)
        .srcBufferLoaded = ((*ctx).srcBufferLoaded as libc::c_ulong).wrapping_sub(n)
        as size_t as size_t;
    (*ctx).srcBuffer = ((*ctx).srcBuffer).offset(n as isize);
}
unsafe extern "C" fn AIO_ReadPool_releaseCurrentHeldAndGetNext(
    mut ctx: *mut ReadPoolCtx_t,
) -> *mut IOJob_t {
    if !((*ctx).currentJobHeld).is_null() {
        AIO_IOPool_releaseIoJob((*ctx).currentJobHeld as *mut IOJob_t);
        (*ctx).currentJobHeld = NULL as *mut libc::c_void;
        AIO_ReadPool_enqueueRead(ctx);
    }
    (*ctx).currentJobHeld = AIO_ReadPool_getNextCompletedJob(ctx) as *mut libc::c_void;
    return (*ctx).currentJobHeld as *mut IOJob_t;
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_fillBuffer(
    mut ctx: *mut ReadPoolCtx_t,
    mut n: size_t,
) -> size_t {
    let mut job = 0 as *mut IOJob_t;
    let mut useCoalesce = 0 as libc::c_int;
    if n > (*ctx).base.jobBufferSize {
        n = (*ctx).base.jobBufferSize;
    }
    if (*ctx).srcBufferLoaded >= n {
        return 0 as libc::c_int as size_t;
    }
    if (*ctx).srcBufferLoaded > 0 as libc::c_int as libc::c_ulong {
        useCoalesce = 1 as libc::c_int;
        memcpy(
            (*ctx).coalesceBuffer as *mut libc::c_void,
            (*ctx).srcBuffer as *const libc::c_void,
            (*ctx).srcBufferLoaded,
        );
        (*ctx).srcBuffer = (*ctx).coalesceBuffer;
    }
    job = AIO_ReadPool_releaseCurrentHeldAndGetNext(ctx);
    if job.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if useCoalesce != 0 {
        if ((*ctx).srcBufferLoaded).wrapping_add((*job).usedBufferSize)
            <= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*ctx).base.jobBufferSize)
        {} else {
            __assert_fail(
                b"ctx->srcBufferLoaded + job->usedBufferSize <= 2*ctx->base.jobBufferSize\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/fileio_asyncio.c\0" as *const u8
                    as *const libc::c_char,
                626 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"size_t AIO_ReadPool_fillBuffer(ReadPoolCtx_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
        memcpy(
            ((*ctx).coalesceBuffer).offset((*ctx).srcBufferLoaded as isize)
                as *mut libc::c_void,
            (*job).buffer,
            (*job).usedBufferSize,
        );
        (*ctx)
            .srcBufferLoaded = ((*ctx).srcBufferLoaded as libc::c_ulong)
            .wrapping_add((*job).usedBufferSize) as size_t as size_t;
    } else {
        (*ctx).srcBuffer = (*job).buffer as *mut U8;
        (*ctx).srcBufferLoaded = (*job).usedBufferSize;
    }
    return (*job).usedBufferSize;
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_consumeAndRefill(
    mut ctx: *mut ReadPoolCtx_t,
) -> size_t {
    AIO_ReadPool_consumeBytes(ctx, (*ctx).srcBufferLoaded);
    return AIO_ReadPool_fillBuffer(ctx, (*ctx).base.jobBufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_getFile(
    mut ctx: *const ReadPoolCtx_t,
) -> *mut FILE {
    return AIO_IOPool_getFile(&(*ctx).base);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_closeFile(
    mut ctx: *mut ReadPoolCtx_t,
) -> libc::c_int {
    let file = AIO_ReadPool_getFile(ctx);
    AIO_ReadPool_setFile(ctx, NULL as *mut FILE);
    return fclose(file);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_setAsync(
    mut ctx: *mut ReadPoolCtx_t,
    mut async_0: libc::c_int,
) {
    AIO_IOPool_setThreaded(&mut (*ctx).base, async_0);
}
unsafe extern "C" fn run_static_initializers() {
    segmentSizeT = ((32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong);
    maskT = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
