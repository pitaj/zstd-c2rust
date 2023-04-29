use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type POOL_ctx_s;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn clock() -> clock_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn POOL_create(numThreads: size_t, queueSize: size_t) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_add(ctx: *mut POOL_ctx, function: POOL_function, opaque: *mut libc::c_void);
    fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut libc::c_void,
        maxDictSize: size_t,
        dictContent: *const libc::c_void,
        dictContentSize: size_t,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const size_t,
        nbSamples: libc::c_uint,
        parameters: ZDICT_params_t,
    ) -> size_t;
    fn COVER_computeEpochs(
        maxDictSize: U32,
        nbDmers: U32,
        k: U32,
        passes: U32,
    ) -> COVER_epoch_info_t;
    fn COVER_warnOnSmallCorpus(
        maxDictSize: size_t,
        nbDmers: size_t,
        displayLevel: libc::c_int,
    );
    fn COVER_sum(samplesSizes: *const size_t, nbSamples: libc::c_uint) -> size_t;
    fn COVER_best_init(best: *mut COVER_best_t);
    fn COVER_best_wait(best: *mut COVER_best_t);
    fn COVER_best_destroy(best: *mut COVER_best_t);
    fn COVER_best_start(best: *mut COVER_best_t);
    fn COVER_best_finish(
        best: *mut COVER_best_t,
        parameters: ZDICT_cover_params_t,
        selection: COVER_dictSelection_t,
    );
    fn COVER_dictSelectionIsError(selection: COVER_dictSelection_t) -> libc::c_uint;
    fn COVER_dictSelectionError(error: size_t) -> COVER_dictSelection_t;
    fn COVER_dictSelectionFree(selection: COVER_dictSelection_t);
    fn COVER_selectDict(
        customDictContent: *mut BYTE,
        dictBufferCapacity: size_t,
        dictContentSize: size_t,
        samplesBuffer: *const BYTE,
        samplesSizes: *const size_t,
        nbFinalizeSamples: libc::c_uint,
        nbCheckSamples: size_t,
        nbSamples: size_t,
        params: ZDICT_cover_params_t,
        offsets: *mut size_t,
        totalCompressedSize: size_t,
    ) -> COVER_dictSelection_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign64 = U64;
pub type POOL_ctx = POOL_ctx_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: libc::c_double,
    pub shrinkDict: libc::c_uint,
    pub shrinkDictMaxRegression: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
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
pub struct FASTCOVER_accel_t {
    pub finalize: libc::c_uint,
    pub skip: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_ctx_t {
    pub samples: *const BYTE,
    pub offsets: *mut size_t,
    pub samplesSizes: *const size_t,
    pub nbSamples: size_t,
    pub nbTrainSamples: size_t,
    pub nbTestSamples: size_t,
    pub nbDmers: size_t,
    pub freqs: *mut U32,
    pub d: libc::c_uint,
    pub f: libc::c_uint,
    pub accelParams: FASTCOVER_accel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_segment_t {
    pub begin: U32,
    pub end: U32,
    pub score: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_epoch_info_t {
    pub num: U32,
    pub size: U32,
}
pub type COVER_best_t = COVER_best_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_best_s {
    pub mutex: *mut pthread_mutex_t,
    pub cond: *mut pthread_cond_t,
    pub liveJobs: size_t,
    pub dict: *mut libc::c_void,
    pub dictSize: size_t,
    pub parameters: ZDICT_cover_params_t,
    pub compressedSize: size_t,
}
pub type FASTCOVER_tryParameters_data_t = FASTCOVER_tryParameters_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_tryParameters_data_s {
    pub ctx: *const FASTCOVER_ctx_t,
    pub best: *mut COVER_best_t,
    pub dictBufferCapacity: size_t,
    pub parameters: ZDICT_cover_params_t,
}
pub type COVER_dictSelection_t = COVER_dictSelection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_dictSelection {
    pub dictContent: *mut BYTE,
    pub dictSize: size_t,
    pub totalCompressedSize: size_t,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const CLOCKS_PER_SEC: libc::c_int = 1000000 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        return MEM_swap64(MEM_read64(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0.swap_bytes();
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
static mut prime6bytes: U64 = 227718039650203 as libc::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    if h <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/../compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            814 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"size_t ZSTD_hash6(U64, U32, U64)\0"))
                .as_ptr(),
        );
    }
    return ((u << 64 as libc::c_int - 48 as libc::c_int).wrapping_mul(prime6bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h, 0 as libc::c_int as U64);
}
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463 as libc::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash8(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    if h <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/../compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            824 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"size_t ZSTD_hash8(U64, U32, U64)\0"))
                .as_ptr(),
        );
    }
    return (u.wrapping_mul(prime8bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash8(MEM_readLE64(p), h, 0 as libc::c_int as U64);
}
pub const ZDICT_DICTSIZE_MIN: libc::c_int = 256 as libc::c_int;
pub const FASTCOVER_MAX_F: libc::c_int = 31 as libc::c_int;
pub const FASTCOVER_MAX_ACCEL: libc::c_int = 10 as libc::c_int;
pub const FASTCOVER_DEFAULT_SPLITPOINT: libc::c_double = 0.75f64;
pub const DEFAULT_F: libc::c_int = 20 as libc::c_int;
pub const DEFAULT_ACCEL: libc::c_int = 1 as libc::c_int;
static mut g_displayLevel: libc::c_int = 0 as libc::c_int;
static mut g_refreshRate: clock_t = CLOCKS_PER_SEC as __clock_t
    * 15 as libc::c_int as libc::c_long / 100 as libc::c_int as libc::c_long;
static mut g_time: clock_t = 0 as libc::c_int as clock_t;
unsafe extern "C" fn FASTCOVER_hashPtrToIndex(
    mut p: *const libc::c_void,
    mut f: U32,
    mut d: libc::c_uint,
) -> size_t {
    if d == 6 as libc::c_int as libc::c_uint {
        return ZSTD_hash6Ptr(p, f);
    }
    return ZSTD_hash8Ptr(p, f);
}
static mut FASTCOVER_defaultAccelParameters: [FASTCOVER_accel_t; 11] = [
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 100 as libc::c_int as libc::c_uint,
            skip: 0 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 100 as libc::c_int as libc::c_uint,
            skip: 0 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 50 as libc::c_int as libc::c_uint,
            skip: 1 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 34 as libc::c_int as libc::c_uint,
            skip: 2 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 25 as libc::c_int as libc::c_uint,
            skip: 3 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 20 as libc::c_int as libc::c_uint,
            skip: 4 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 17 as libc::c_int as libc::c_uint,
            skip: 5 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 14 as libc::c_int as libc::c_uint,
            skip: 6 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 13 as libc::c_int as libc::c_uint,
            skip: 7 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 11 as libc::c_int as libc::c_uint,
            skip: 8 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 10 as libc::c_int as libc::c_uint,
            skip: 9 as libc::c_int as libc::c_uint,
        };
        init
    },
];
unsafe extern "C" fn FASTCOVER_selectSegment(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut U32,
    mut begin: U32,
    mut end: U32,
    mut parameters: ZDICT_cover_params_t,
    mut segmentFreqs: *mut U16,
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
    let f = (*ctx).f;
    let dmersInK = k.wrapping_sub(d).wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut bestSegment = {
        let mut init = COVER_segment_t {
            begin: 0 as libc::c_int as U32,
            end: 0 as libc::c_int as U32,
            score: 0 as libc::c_int as U32,
        };
        init
    };
    let mut activeSegment = COVER_segment_t {
        begin: 0,
        end: 0,
        score: 0,
    };
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0 as libc::c_int as U32;
    while activeSegment.end < end {
        let idx = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(activeSegment.end as isize) as *const libc::c_void,
            f,
            d,
        );
        if *segmentFreqs.offset(idx as isize) as libc::c_int == 0 as libc::c_int {
            activeSegment
                .score = (activeSegment.score as libc::c_uint)
                .wrapping_add(*freqs.offset(idx as isize)) as U32 as U32;
        }
        activeSegment
            .end = (activeSegment.end as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as U32 as U32;
        let ref mut fresh0 = *segmentFreqs.offset(idx as isize);
        *fresh0 = (*fresh0 as libc::c_int + 1 as libc::c_int) as U16;
        if (activeSegment.end).wrapping_sub(activeSegment.begin)
            == dmersInK.wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            let delIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(activeSegment.begin as isize)
                    as *const libc::c_void,
                f,
                d,
            );
            let ref mut fresh1 = *segmentFreqs.offset(delIndex as isize);
            *fresh1 = (*fresh1 as libc::c_int - 1 as libc::c_int) as U16;
            if *segmentFreqs.offset(delIndex as isize) as libc::c_int == 0 as libc::c_int
            {
                activeSegment
                    .score = (activeSegment.score as libc::c_uint)
                    .wrapping_sub(*freqs.offset(delIndex as isize)) as U32 as U32;
            }
            activeSegment
                .begin = (activeSegment.begin as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as U32 as U32;
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    while activeSegment.begin < end {
        let delIndex_0 = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(activeSegment.begin as isize) as *const libc::c_void,
            f,
            d,
        );
        let ref mut fresh2 = *segmentFreqs.offset(delIndex_0 as isize);
        *fresh2 = (*fresh2 as libc::c_int - 1 as libc::c_int) as U16;
        activeSegment
            .begin = (activeSegment.begin as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as U32 as U32;
    }
    let mut pos: U32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let i = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(pos as isize) as *const libc::c_void,
            f,
            d,
        );
        *freqs.offset(i as isize) = 0 as libc::c_int as U32;
        pos = pos.wrapping_add(1);
    }
    return bestSegment;
}
unsafe extern "C" fn FASTCOVER_checkParameters(
    mut parameters: ZDICT_cover_params_t,
    mut maxDictSize: size_t,
    mut f: libc::c_uint,
    mut accel: libc::c_uint,
) -> libc::c_int {
    if parameters.d == 0 as libc::c_int as libc::c_uint
        || parameters.k == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if parameters.d != 6 as libc::c_int as libc::c_uint
        && parameters.d != 8 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if parameters.k as libc::c_ulong > maxDictSize {
        return 0 as libc::c_int;
    }
    if parameters.d > parameters.k {
        return 0 as libc::c_int;
    }
    if f > FASTCOVER_MAX_F as libc::c_uint || f == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if parameters.splitPoint <= 0 as libc::c_int as libc::c_double
        || parameters.splitPoint > 1 as libc::c_int as libc::c_double
    {
        return 0 as libc::c_int;
    }
    if accel > 10 as libc::c_int as libc::c_uint
        || accel == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn FASTCOVER_ctx_destroy(mut ctx: *mut FASTCOVER_ctx_t) {
    if ctx.is_null() {
        return;
    }
    free((*ctx).freqs as *mut libc::c_void);
    (*ctx).freqs = NULL as *mut U32;
    free((*ctx).offsets as *mut libc::c_void);
    (*ctx).offsets = NULL as *mut size_t;
}
unsafe extern "C" fn FASTCOVER_computeFrequency(
    mut freqs: *mut U32,
    mut ctx: *const FASTCOVER_ctx_t,
) {
    let f = (*ctx).f;
    let d = (*ctx).d;
    let skip = (*ctx).accelParams.skip;
    let readLength = if d > 8 as libc::c_int as libc::c_uint {
        d
    } else {
        8 as libc::c_int as libc::c_uint
    };
    let mut i: size_t = 0;
    if (*ctx).nbTrainSamples >= 5 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ctx->nbTrainSamples >= 5\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/fastcover.c\0" as *const u8
                as *const libc::c_char,
            291 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void FASTCOVER_computeFrequency(U32 *, const FASTCOVER_ctx_t *)\0"))
                .as_ptr(),
        );
    }
    if (*ctx).nbTrainSamples <= (*ctx).nbSamples {} else {
        __assert_fail(
            b"ctx->nbTrainSamples <= ctx->nbSamples\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/fastcover.c\0" as *const u8
                as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void FASTCOVER_computeFrequency(U32 *, const FASTCOVER_ctx_t *)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int as size_t;
    while i < (*ctx).nbTrainSamples {
        let mut start = *((*ctx).offsets).offset(i as isize);
        let currSampleEnd = *((*ctx).offsets)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        while start.wrapping_add(readLength as libc::c_ulong) <= currSampleEnd {
            let dmerIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(start as isize) as *const libc::c_void,
                f,
                d,
            );
            let ref mut fresh3 = *freqs.offset(dmerIndex as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            start = start
                .wrapping_add(skip as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn FASTCOVER_ctx_init(
    mut ctx: *mut FASTCOVER_ctx_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: libc::c_uint,
    mut d: libc::c_uint,
    mut splitPoint: libc::c_double,
    mut f: libc::c_uint,
    mut accelParams: FASTCOVER_accel_t,
) -> size_t {
    let samples = samplesBuffer as *const BYTE;
    let totalSamplesSize = COVER_sum(samplesSizes, nbSamples);
    let nbTrainSamples = if splitPoint < 1.0f64 {
        (nbSamples as libc::c_double * splitPoint) as libc::c_uint
    } else {
        nbSamples
    };
    let nbTestSamples = if splitPoint < 1.0f64 {
        nbSamples.wrapping_sub(nbTrainSamples)
    } else {
        nbSamples
    };
    let trainingSamplesSize = if splitPoint < 1.0f64 {
        COVER_sum(samplesSizes, nbTrainSamples)
    } else {
        totalSamplesSize
    };
    let testSamplesSize = if splitPoint < 1.0f64 {
        COVER_sum(samplesSizes.offset(nbTrainSamples as isize), nbTestSamples)
    } else {
        totalSamplesSize
    };
    if totalSamplesSize
        < (if d as libc::c_ulong > ::core::mem::size_of::<U64>() as libc::c_ulong {
            d as libc::c_ulong
        } else {
            ::core::mem::size_of::<U64>() as libc::c_ulong
        })
        || totalSamplesSize
            >= (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 8 as libc::c_int as libc::c_ulong
            {
                -(1 as libc::c_int) as libc::c_uint
            } else {
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int)
            }) as size_t
    {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Total samples size is too large (%u MB), maximum size is %u MB\n\0"
                    as *const u8 as *const libc::c_char,
                (totalSamplesSize >> 20 as libc::c_int) as libc::c_uint,
                (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                    == 8 as libc::c_int as libc::c_ulong
                {
                    -(1 as libc::c_int) as libc::c_uint
                } else {
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((1 as libc::c_uint) << 30 as libc::c_int)
                }) >> 20 as libc::c_int,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if nbTrainSamples < 5 as libc::c_int as libc::c_uint {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Total number of training samples is %u and is invalid\n\0" as *const u8
                    as *const libc::c_char,
                nbTrainSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if nbTestSamples < 1 as libc::c_int as libc::c_uint {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Total number of testing samples is %u and is invalid.\n\0" as *const u8
                    as *const libc::c_char,
                nbTestSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<FASTCOVER_ctx_t>() as libc::c_ulong,
    );
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"Training on %u samples of total size %u\n\0" as *const u8
                as *const libc::c_char,
            nbTrainSamples,
            trainingSamplesSize as libc::c_uint,
        );
        fflush(stderr);
    }
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"Testing on %u samples of total size %u\n\0" as *const u8
                as *const libc::c_char,
            nbTestSamples,
            testSamplesSize as libc::c_uint,
        );
        fflush(stderr);
    }
    (*ctx).samples = samples;
    (*ctx).samplesSizes = samplesSizes;
    (*ctx).nbSamples = nbSamples as size_t;
    (*ctx).nbTrainSamples = nbTrainSamples as size_t;
    (*ctx).nbTestSamples = nbTestSamples as size_t;
    (*ctx)
        .nbDmers = trainingSamplesSize
        .wrapping_sub(
            (if d as libc::c_ulong > ::core::mem::size_of::<U64>() as libc::c_ulong {
                d as libc::c_ulong
            } else {
                ::core::mem::size_of::<U64>() as libc::c_ulong
            }),
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*ctx).d = d;
    (*ctx).f = f;
    (*ctx).accelParams = accelParams;
    (*ctx)
        .offsets = calloc(
        nbSamples.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
    ) as *mut size_t;
    if ((*ctx).offsets).is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Failed to allocate scratch buffers \n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    let mut i: U32 = 0;
    *((*ctx).offsets).offset(0 as libc::c_int as isize) = 0 as libc::c_int as size_t;
    if nbSamples >= 5 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"nbSamples >= 5\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/fastcover.c\0" as *const u8
                as *const libc::c_char,
            375 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t FASTCOVER_ctx_init(FASTCOVER_ctx_t *, const void *, const size_t *, unsigned int, unsigned int, double, unsigned int, FASTCOVER_accel_t)\0",
            ))
                .as_ptr(),
        );
    }
    i = 1 as libc::c_int as U32;
    while i <= nbSamples {
        *((*ctx).offsets)
            .offset(
                i as isize,
            ) = (*((*ctx).offsets)
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .wrapping_add(
                *samplesSizes
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize),
            );
        i = i.wrapping_add(1);
    }
    (*ctx)
        .freqs = calloc(
        (1 as libc::c_int as U64) << f,
        ::core::mem::size_of::<U32>() as libc::c_ulong,
    ) as *mut U32;
    if ((*ctx).freqs).is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Failed to allocate frequency table \n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
    }
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"Computing frequencies\n\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    FASTCOVER_computeFrequency((*ctx).freqs, ctx);
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FASTCOVER_buildDictionary(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut U32,
    mut dictBuffer: *mut libc::c_void,
    mut dictBufferCapacity: size_t,
    mut parameters: ZDICT_cover_params_t,
    mut segmentFreqs: *mut U16,
) -> size_t {
    let dict = dictBuffer as *mut BYTE;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as U32,
        (*ctx).nbDmers as U32,
        parameters.k,
        1 as libc::c_int as U32,
    );
    let maxZeroScoreRun = 10 as libc::c_int as size_t;
    let mut zeroScoreRun = 0 as libc::c_int as size_t;
    let mut epoch: size_t = 0;
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"Breaking content into %u epochs of size %u\n\0" as *const u8
                as *const libc::c_char,
            epochs.num,
            epochs.size,
        );
        fflush(stderr);
    }
    epoch = 0 as libc::c_int as size_t;
    while tail > 0 as libc::c_int as libc::c_ulong {
        let epochBegin = epoch.wrapping_mul(epochs.size as libc::c_ulong) as U32;
        let epochEnd = epochBegin.wrapping_add(epochs.size);
        let mut segmentSize: size_t = 0;
        let mut segment = FASTCOVER_selectSegment(
            ctx,
            freqs,
            epochBegin,
            epochEnd,
            parameters,
            segmentFreqs,
        );
        if segment.score == 0 as libc::c_int as libc::c_uint {
            zeroScoreRun = zeroScoreRun.wrapping_add(1);
            if zeroScoreRun >= maxZeroScoreRun {
                break;
            }
        } else {
            zeroScoreRun = 0 as libc::c_int as size_t;
            segmentSize = if ((segment.end)
                .wrapping_sub(segment.begin)
                .wrapping_add(parameters.d)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong) < tail
            {
                (segment.end)
                    .wrapping_sub(segment.begin)
                    .wrapping_add(parameters.d)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
            } else {
                tail
            };
            if segmentSize < parameters.d as libc::c_ulong {
                break;
            }
            tail = (tail as libc::c_ulong).wrapping_sub(segmentSize) as size_t as size_t;
            memcpy(
                dict.offset(tail as isize) as *mut libc::c_void,
                ((*ctx).samples).offset(segment.begin as isize) as *const libc::c_void,
                segmentSize,
            );
            if g_displayLevel >= 2 as libc::c_int {
                if clock() - g_time > g_refreshRate || g_displayLevel >= 4 as libc::c_int
                {
                    g_time = clock();
                    fprintf(
                        stderr,
                        b"\r%u%%       \0" as *const u8 as *const libc::c_char,
                        dictBufferCapacity
                            .wrapping_sub(tail)
                            .wrapping_mul(100 as libc::c_int as libc::c_ulong)
                            .wrapping_div(dictBufferCapacity) as libc::c_uint,
                    );
                    fflush(stderr);
                }
            }
        }
        epoch = epoch
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_rem(epochs.num as libc::c_ulong);
    }
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    return tail;
}
unsafe extern "C" fn FASTCOVER_tryParameters(mut opaque: *mut libc::c_void) {
    let data = opaque as *mut FASTCOVER_tryParameters_data_t;
    let ctx = (*data).ctx;
    let parameters = (*data).parameters;
    let mut dictBufferCapacity = (*data).dictBufferCapacity;
    let mut totalCompressedSize = -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    let mut segmentFreqs = calloc(
        (1 as libc::c_int as U64) << (*ctx).f,
        ::core::mem::size_of::<U16>() as libc::c_ulong,
    ) as *mut U16;
    let dict = malloc(dictBufferCapacity) as *mut BYTE;
    let mut selection = COVER_dictSelectionError(
        -(ZSTD_error_GENERIC as libc::c_int) as size_t,
    );
    let mut freqs = malloc(
        ((1 as libc::c_int as U64) << (*ctx).f)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
    ) as *mut U32;
    if segmentFreqs.is_null() || dict.is_null() || freqs.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Failed to allocate buffers: out of memory\n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
    } else {
        memcpy(
            freqs as *mut libc::c_void,
            (*ctx).freqs as *const libc::c_void,
            ((1 as libc::c_int as U64) << (*ctx).f)
                .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
        );
        let tail = FASTCOVER_buildDictionary(
            ctx,
            freqs,
            dict as *mut libc::c_void,
            dictBufferCapacity,
            parameters,
            segmentFreqs,
        );
        let nbFinalizeSamples = ((*ctx).nbTrainSamples)
            .wrapping_mul((*ctx).accelParams.finalize as libc::c_ulong)
            .wrapping_div(100 as libc::c_int as libc::c_ulong) as libc::c_uint;
        selection = COVER_selectDict(
            dict.offset(tail as isize),
            dictBufferCapacity,
            dictBufferCapacity.wrapping_sub(tail),
            (*ctx).samples,
            (*ctx).samplesSizes,
            nbFinalizeSamples,
            (*ctx).nbTrainSamples,
            (*ctx).nbSamples,
            parameters,
            (*ctx).offsets,
            totalCompressedSize,
        );
        if COVER_dictSelectionIsError(selection) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Failed to select dictionary\n\0" as *const u8
                        as *const libc::c_char,
                );
                fflush(stderr);
            }
        }
    }
    free(dict as *mut libc::c_void);
    COVER_best_finish((*data).best, parameters, selection);
    free(data as *mut libc::c_void);
    free(segmentFreqs as *mut libc::c_void);
    COVER_dictSelectionFree(selection);
    free(freqs as *mut libc::c_void);
}
unsafe extern "C" fn FASTCOVER_convertToCoverParams(
    mut fastCoverParams: ZDICT_fastCover_params_t,
    mut coverParams: *mut ZDICT_cover_params_t,
) {
    (*coverParams).k = fastCoverParams.k;
    (*coverParams).d = fastCoverParams.d;
    (*coverParams).steps = fastCoverParams.steps;
    (*coverParams).nbThreads = fastCoverParams.nbThreads;
    (*coverParams).splitPoint = fastCoverParams.splitPoint;
    (*coverParams).zParams = fastCoverParams.zParams;
    (*coverParams).shrinkDict = fastCoverParams.shrinkDict;
}
unsafe extern "C" fn FASTCOVER_convertToFastCoverParams(
    mut coverParams: ZDICT_cover_params_t,
    mut fastCoverParams: *mut ZDICT_fastCover_params_t,
    mut f: libc::c_uint,
    mut accel: libc::c_uint,
) {
    (*fastCoverParams).k = coverParams.k;
    (*fastCoverParams).d = coverParams.d;
    (*fastCoverParams).steps = coverParams.steps;
    (*fastCoverParams).nbThreads = coverParams.nbThreads;
    (*fastCoverParams).splitPoint = coverParams.splitPoint;
    (*fastCoverParams).f = f;
    (*fastCoverParams).accel = accel;
    (*fastCoverParams).zParams = coverParams.zParams;
    (*fastCoverParams).shrinkDict = coverParams.shrinkDict;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_fastCover(
    mut dictBuffer: *mut libc::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: libc::c_uint,
    mut parameters: ZDICT_fastCover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut BYTE;
    let mut ctx = FASTCOVER_ctx_t {
        samples: 0 as *const BYTE,
        offsets: 0 as *mut size_t,
        samplesSizes: 0 as *const size_t,
        nbSamples: 0,
        nbTrainSamples: 0,
        nbTestSamples: 0,
        nbDmers: 0,
        freqs: 0 as *mut U32,
        d: 0,
        f: 0,
        accelParams: FASTCOVER_accel_t {
            finalize: 0,
            skip: 0,
        },
    };
    let mut coverParams = ZDICT_cover_params_t {
        k: 0,
        d: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    let mut accelParams = FASTCOVER_accel_t {
        finalize: 0,
        skip: 0,
    };
    g_displayLevel = parameters.zParams.notificationLevel as libc::c_int;
    parameters.splitPoint = 1.0f64;
    parameters
        .f = if parameters.f == 0 as libc::c_int as libc::c_uint {
        DEFAULT_F as libc::c_uint
    } else {
        parameters.f
    };
    parameters
        .accel = if parameters.accel == 0 as libc::c_int as libc::c_uint {
        DEFAULT_ACCEL as libc::c_uint
    } else {
        parameters.accel
    };
    memset(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong,
    );
    FASTCOVER_convertToCoverParams(parameters, &mut coverParams);
    if FASTCOVER_checkParameters(
        coverParams,
        dictBufferCapacity,
        parameters.f,
        parameters.accel,
    ) == 0
    {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"FASTCOVER parameters incorrect\n\0" as *const u8 as *const libc::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if nbSamples == 0 as libc::c_int as libc::c_uint {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"FASTCOVER must have at least one input file\n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as libc::c_ulong {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const libc::c_char,
                256 as libc::c_int,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    accelParams = FASTCOVER_defaultAccelParameters[parameters.accel as usize];
    let initVal = FASTCOVER_ctx_init(
        &mut ctx,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        coverParams.d,
        parameters.splitPoint,
        parameters.f,
        accelParams,
    );
    if ERR_isError(initVal) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Failed to initialize context\n\0" as *const u8 as *const libc::c_char,
            );
            fflush(stderr);
        }
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, g_displayLevel);
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr, b"Building dictionary\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    let mut segmentFreqs = calloc(
        (1 as libc::c_int as U64) << parameters.f,
        ::core::mem::size_of::<U16>() as libc::c_ulong,
    ) as *mut U16;
    let tail = FASTCOVER_buildDictionary(
        &mut ctx,
        ctx.freqs,
        dictBuffer,
        dictBufferCapacity,
        coverParams,
        segmentFreqs,
    );
    let nbFinalizeSamples = (ctx.nbTrainSamples)
        .wrapping_mul(ctx.accelParams.finalize as libc::c_ulong)
        .wrapping_div(100 as libc::c_int as libc::c_ulong) as libc::c_uint;
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut libc::c_void,
        dictBufferCapacity,
        dict.offset(tail as isize) as *const libc::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes,
        nbFinalizeSamples,
        coverParams.zParams,
    );
    if ERR_isError(dictionarySize) == 0 {
        if g_displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Constructed dictionary of size %u\n\0" as *const u8
                    as *const libc::c_char,
                dictionarySize as libc::c_uint,
            );
            fflush(stderr);
        }
    }
    FASTCOVER_ctx_destroy(&mut ctx);
    free(segmentFreqs as *mut libc::c_void);
    return dictionarySize;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_fastCover(
    mut dictBuffer: *mut libc::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const libc::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: libc::c_uint,
    mut parameters: *mut ZDICT_fastCover_params_t,
) -> size_t {
    let mut coverParams = ZDICT_cover_params_t {
        k: 0,
        d: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    let mut accelParams = FASTCOVER_accel_t {
        finalize: 0,
        skip: 0,
    };
    let nbThreads = (*parameters).nbThreads;
    let splitPoint = if (*parameters).splitPoint <= 0.0f64 {
        FASTCOVER_DEFAULT_SPLITPOINT
    } else {
        (*parameters).splitPoint
    };
    let kMinD = if (*parameters).d == 0 as libc::c_int as libc::c_uint {
        6 as libc::c_int as libc::c_uint
    } else {
        (*parameters).d
    };
    let kMaxD = if (*parameters).d == 0 as libc::c_int as libc::c_uint {
        8 as libc::c_int as libc::c_uint
    } else {
        (*parameters).d
    };
    let kMinK = if (*parameters).k == 0 as libc::c_int as libc::c_uint {
        50 as libc::c_int as libc::c_uint
    } else {
        (*parameters).k
    };
    let kMaxK = if (*parameters).k == 0 as libc::c_int as libc::c_uint {
        2000 as libc::c_int as libc::c_uint
    } else {
        (*parameters).k
    };
    let kSteps = if (*parameters).steps == 0 as libc::c_int as libc::c_uint {
        40 as libc::c_int as libc::c_uint
    } else {
        (*parameters).steps
    };
    let kStepSize = if kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
        > 1 as libc::c_int as libc::c_uint
    {
        kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
    } else {
        1 as libc::c_int as libc::c_uint
    };
    let kIterations = (1 as libc::c_int as libc::c_uint)
        .wrapping_add(
            kMaxD.wrapping_sub(kMinD).wrapping_div(2 as libc::c_int as libc::c_uint),
        )
        .wrapping_mul(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)),
        );
    let f = if (*parameters).f == 0 as libc::c_int as libc::c_uint {
        DEFAULT_F as libc::c_uint
    } else {
        (*parameters).f
    };
    let accel = if (*parameters).accel == 0 as libc::c_int as libc::c_uint {
        DEFAULT_ACCEL as libc::c_uint
    } else {
        (*parameters).accel
    };
    let shrinkDict = 0 as libc::c_int as libc::c_uint;
    let displayLevel = (*parameters).zParams.notificationLevel as libc::c_int;
    let mut iteration = 1 as libc::c_int as libc::c_uint;
    let mut d: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut best = COVER_best_t {
        mutex: 0 as *mut pthread_mutex_t,
        cond: 0 as *mut pthread_cond_t,
        liveJobs: 0,
        dict: 0 as *mut libc::c_void,
        dictSize: 0,
        parameters: ZDICT_cover_params_t {
            k: 0,
            d: 0,
            steps: 0,
            nbThreads: 0,
            splitPoint: 0.,
            shrinkDict: 0,
            shrinkDictMaxRegression: 0,
            zParams: ZDICT_params_t {
                compressionLevel: 0,
                notificationLevel: 0,
                dictID: 0,
            },
        },
        compressedSize: 0,
    };
    let mut pool = NULL as *mut POOL_ctx;
    let mut warned = 0 as libc::c_int;
    if splitPoint <= 0 as libc::c_int as libc::c_double
        || splitPoint > 1 as libc::c_int as libc::c_double
    {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Incorrect splitPoint\n\0" as *const u8 as *const libc::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if accel == 0 as libc::c_int as libc::c_uint
        || accel > FASTCOVER_MAX_ACCEL as libc::c_uint
    {
        if displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"Incorrect accel\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b"Incorrect k\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t;
    }
    if nbSamples == 0 as libc::c_int as libc::c_uint {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"FASTCOVER must have at least one input file\n\0" as *const u8
                    as *const libc::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as libc::c_ulong {
        if displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const libc::c_char,
                256 as libc::c_int,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if nbThreads > 1 as libc::c_int as libc::c_uint {
        pool = POOL_create(nbThreads as size_t, 1 as libc::c_int as size_t);
        if pool.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        }
    }
    COVER_best_init(&mut best);
    memset(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong,
    );
    FASTCOVER_convertToCoverParams(*parameters, &mut coverParams);
    accelParams = FASTCOVER_defaultAccelParameters[accel as usize];
    g_displayLevel = if displayLevel == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        displayLevel - 1 as libc::c_int
    };
    if displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"Trying %u different sets of parameters\n\0" as *const u8
                as *const libc::c_char,
            kIterations,
        );
        fflush(stderr);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = FASTCOVER_ctx_t {
            samples: 0 as *const BYTE,
            offsets: 0 as *mut size_t,
            samplesSizes: 0 as *const size_t,
            nbSamples: 0,
            nbTrainSamples: 0,
            nbTestSamples: 0,
            nbDmers: 0,
            freqs: 0 as *mut U32,
            d: 0,
            f: 0,
            accelParams: FASTCOVER_accel_t {
                finalize: 0,
                skip: 0,
            },
        };
        if displayLevel >= 3 as libc::c_int {
            fprintf(stderr, b"d=%u\n\0" as *const u8 as *const libc::c_char, d);
            fflush(stderr);
        }
        let initVal = FASTCOVER_ctx_init(
            &mut ctx,
            samplesBuffer,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            f,
            accelParams,
        );
        if ERR_isError(initVal) != 0 {
            if displayLevel >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Failed to initialize context\n\0" as *const u8
                        as *const libc::c_char,
                );
                fflush(stderr);
            }
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
            warned = 1 as libc::c_int;
        }
        k = kMinK;
        while k <= kMaxK {
            let mut data = malloc(
                ::core::mem::size_of::<FASTCOVER_tryParameters_data_t>() as libc::c_ulong,
            ) as *mut FASTCOVER_tryParameters_data_t;
            if displayLevel >= 3 as libc::c_int {
                fprintf(stderr, b"k=%u\n\0" as *const u8 as *const libc::c_char, k);
                fflush(stderr);
            }
            if data.is_null() {
                if displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Failed to allocate parameters\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    fflush(stderr);
                }
                COVER_best_destroy(&mut best);
                FASTCOVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = coverParams;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.shrinkDict = shrinkDict;
            (*data)
                .parameters
                .zParams
                .notificationLevel = g_displayLevel as libc::c_uint;
            if FASTCOVER_checkParameters(
                (*data).parameters,
                dictBufferCapacity,
                (*(*data).ctx).f,
                accel,
            ) == 0
            {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"FASTCOVER parameters incorrect\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    fflush(stderr);
                }
                free(data as *mut libc::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        Some(
                            FASTCOVER_tryParameters
                                as unsafe extern "C" fn(*mut libc::c_void) -> (),
                        ),
                        data as *mut libc::c_void,
                    );
                } else {
                    FASTCOVER_tryParameters(data as *mut libc::c_void);
                }
                if displayLevel >= 2 as libc::c_int {
                    if clock() - g_time > g_refreshRate
                        || displayLevel >= 4 as libc::c_int
                    {
                        g_time = clock();
                        fprintf(
                            stderr,
                            b"\r%u%%       \0" as *const u8 as *const libc::c_char,
                            iteration
                                .wrapping_mul(100 as libc::c_int as libc::c_uint)
                                .wrapping_div(kIterations),
                        );
                        fflush(stderr);
                    }
                }
                iteration = iteration.wrapping_add(1);
            }
            k = k.wrapping_add(kStepSize);
        }
        COVER_best_wait(&mut best);
        FASTCOVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    if displayLevel >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
    }
    let dictSize = best.dictSize;
    if ERR_isError(best.compressedSize) != 0 {
        let compressedSize = best.compressedSize;
        COVER_best_destroy(&mut best);
        POOL_free(pool);
        return compressedSize;
    }
    FASTCOVER_convertToFastCoverParams(best.parameters, parameters, f, accel);
    memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    return dictSize;
}
