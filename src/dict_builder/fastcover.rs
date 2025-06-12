use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type POOL_ctx_s;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn clock() -> clock_t;
    fn POOL_create(numThreads: usize, queueSize: usize) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_add(
        ctx: *mut POOL_ctx,
        function: POOL_function,
        opaque: *mut std::ffi::c_void,
    );
    fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut std::ffi::c_void,
        maxDictSize: usize,
        dictContent: *const std::ffi::c_void,
        dictContentSize: usize,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: std::ffi::c_uint,
        parameters: ZDICT_params_t,
    ) -> usize;
    fn COVER_computeEpochs(
        maxDictSize: u32,
        nbDmers: u32,
        k: u32,
        passes: u32,
    ) -> COVER_epoch_info_t;
    fn COVER_warnOnSmallCorpus(
        maxDictSize: usize,
        nbDmers: usize,
        displayLevel: std::ffi::c_int,
    );
    fn COVER_sum(samplesSizes: *const usize, nbSamples: std::ffi::c_uint) -> usize;
    fn COVER_best_init(best: *mut COVER_best_t);
    fn COVER_best_wait(best: *mut COVER_best_t);
    fn COVER_best_destroy(best: *mut COVER_best_t);
    fn COVER_best_start(best: *mut COVER_best_t);
    fn COVER_best_finish(
        best: *mut COVER_best_t,
        parameters: ZDICT_cover_params_t,
        selection: COVER_dictSelection_t,
    );
    fn COVER_dictSelectionIsError(selection: COVER_dictSelection_t) -> std::ffi::c_uint;
    fn COVER_dictSelectionError(error: usize) -> COVER_dictSelection_t;
    fn COVER_dictSelectionFree(selection: COVER_dictSelection_t);
    fn COVER_selectDict(
        customDictContent: *mut u8,
        dictBufferCapacity: usize,
        dictContentSize: usize,
        samplesBuffer: *const u8,
        samplesSizes: *const usize,
        nbFinalizeSamples: std::ffi::c_uint,
        nbCheckSamples: usize,
        nbSamples: usize,
        params: ZDICT_cover_params_t,
        offsets: *mut usize,
        totalCompressedSize: usize,
    ) -> COVER_dictSelection_t;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __clock_t = std::ffi::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [std::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: std::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: std::ffi::c_uint,
    pub __high: std::ffi::c_uint,
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
    pub __lock: std::ffi::c_int,
    pub __count: std::ffi::c_uint,
    pub __owner: std::ffi::c_int,
    pub __nusers: std::ffi::c_uint,
    pub __kind: std::ffi::c_int,
    pub __spins: std::ffi::c_short,
    pub __elision: std::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [std::ffi::c_uint; 2],
    pub __g1_orig_size: std::ffi::c_uint,
    pub __wrefs: std::ffi::c_uint,
    pub __g_signals: [std::ffi::c_uint; 2],
    pub __unused_initialized_1: std::ffi::c_uint,
    pub __unused_initialized_2: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [std::ffi::c_char; 40],
    pub __align: std::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [std::ffi::c_char; 48],
    pub __align: std::ffi::c_longlong,
}
pub type unalign64 = u64;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
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
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed_0 = 49;
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
pub type POOL_ctx = POOL_ctx_s;
pub type POOL_function = Option::<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: std::ffi::c_int,
    pub notificationLevel: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: std::ffi::c_uint,
    pub d: std::ffi::c_uint,
    pub steps: std::ffi::c_uint,
    pub nbThreads: std::ffi::c_uint,
    pub splitPoint: std::ffi::c_double,
    pub shrinkDict: std::ffi::c_uint,
    pub shrinkDictMaxRegression: std::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: std::ffi::c_uint,
    pub d: std::ffi::c_uint,
    pub f: std::ffi::c_uint,
    pub steps: std::ffi::c_uint,
    pub nbThreads: std::ffi::c_uint,
    pub splitPoint: std::ffi::c_double,
    pub accel: std::ffi::c_uint,
    pub shrinkDict: std::ffi::c_uint,
    pub shrinkDictMaxRegression: std::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_accel_t {
    pub finalize: std::ffi::c_uint,
    pub skip: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_ctx_t {
    pub samples: *const u8,
    pub offsets: *mut usize,
    pub samplesSizes: *const usize,
    pub nbSamples: usize,
    pub nbTrainSamples: usize,
    pub nbTestSamples: usize,
    pub nbDmers: usize,
    pub freqs: *mut u32,
    pub d: std::ffi::c_uint,
    pub f: std::ffi::c_uint,
    pub accelParams: FASTCOVER_accel_t,
    pub displayLevel: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_segment_t {
    pub begin: u32,
    pub end: u32,
    pub score: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_epoch_info_t {
    pub num: u32,
    pub size: u32,
}
pub type COVER_best_t = COVER_best_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_best_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub liveJobs: usize,
    pub dict: *mut std::ffi::c_void,
    pub dictSize: usize,
    pub parameters: ZDICT_cover_params_t,
    pub compressedSize: usize,
}
pub type FASTCOVER_tryParameters_data_t = FASTCOVER_tryParameters_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_tryParameters_data_s {
    pub ctx: *const FASTCOVER_ctx_t,
    pub best: *mut COVER_best_t,
    pub dictBufferCapacity: usize,
    pub parameters: ZDICT_cover_params_t,
}
pub type COVER_dictSelection_t = COVER_dictSelection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_dictSelection {
    pub dictContent: *mut u8,
    pub dictSize: usize,
    pub totalCompressedSize: usize,
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    return 1;
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const std::ffi::c_void) -> u64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const std::ffi::c_void) -> u64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        return MEM_swap64(MEM_read64(memPtr))
    };
}
static mut prime6bytes: u64 = 227718039650203;
unsafe extern "C" fn ZSTD_hash6(mut u: u64, mut h: u32, mut s: u64) -> usize {
    return ((u << 64 - 48 as std::ffi::c_int) * prime6bytes ^ s)
        >> 64_u32.wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(
    mut p: *const std::ffi::c_void,
    mut h: u32,
) -> usize {
    return ZSTD_hash6(MEM_readLE64(p), h, 0);
}
static mut prime8bytes: u64 = 0xcf1bbcdcb7a56463 as std::ffi::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash8(mut u: u64, mut h: u32, mut s: u64) -> usize {
    return (u * prime8bytes ^ s) >> 64_u32.wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash8Ptr(
    mut p: *const std::ffi::c_void,
    mut h: u32,
) -> usize {
    return ZSTD_hash8(MEM_readLE64(p), h, 0);
}
pub const ZSTD_isError: unsafe extern "C" fn(usize) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn ERR_isError(mut code: usize) -> std::ffi::c_uint {
    return (code > -(ZSTD_error_maxCode as std::ffi::c_int) as usize) as std::ffi::c_int
        as std::ffi::c_uint;
}
pub const ZDICT_DICTSIZE_MIN: std::ffi::c_int = 256;
pub const CLOCKS_PER_SEC: std::ffi::c_int = 1000000;
pub const NULL: std::ffi::c_int = 0;
pub const FASTCOVER_MAX_F: std::ffi::c_int = 31;
pub const FASTCOVER_MAX_ACCEL: std::ffi::c_int = 10;
pub const FASTCOVER_DEFAULT_SPLITPOINT: std::ffi::c_double = 0.75f64;
pub const DEFAULT_F: std::ffi::c_int = 20;
pub const DEFAULT_ACCEL: std::ffi::c_int = 1;
unsafe extern "C" fn FASTCOVER_hashPtrToIndex(
    mut p: *const std::ffi::c_void,
    mut f: u32,
    mut d: std::ffi::c_uint,
) -> usize {
    if d == 6 {
        return ZSTD_hash6Ptr(p, f);
    }
    return ZSTD_hash8Ptr(p, f);
}
static mut FASTCOVER_defaultAccelParameters: [FASTCOVER_accel_t; 11] = [
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 100,
            skip: 0,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 100,
            skip: 0,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 50,
            skip: 1,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 34,
            skip: 2,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 25,
            skip: 3,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 20,
            skip: 4,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 17,
            skip: 5,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 14,
            skip: 6,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 13,
            skip: 7,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 11,
            skip: 8,
        };
        init
    },
    {
        let mut init = FASTCOVER_accel_t {
            finalize: 10,
            skip: 9,
        };
        init
    },
];
unsafe extern "C" fn FASTCOVER_selectSegment(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut u32,
    mut begin: u32,
    mut end: u32,
    mut parameters: ZDICT_cover_params_t,
    mut segmentFreqs: *mut u16,
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
    let f = (*ctx).f;
    let dmersInK = k.wrapping_sub(d).wrapping_add(1);
    let mut bestSegment = {
        let mut init = COVER_segment_t {
            begin: 0,
            end: 0,
            score: 0,
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
    activeSegment.score = 0;
    while activeSegment.end < end {
        let idx = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(activeSegment.end as isize)
                as *const std::ffi::c_void,
            f,
            d,
        );
        if *segmentFreqs.offset(idx as isize) as std::ffi::c_int == 0
        {
            activeSegment
                .score = (activeSegment.score).wrapping_add(*freqs.offset(idx as isize));
        }
        activeSegment
            .end = (activeSegment.end).wrapping_add(1);
        let ref mut fresh0 = *segmentFreqs.offset(idx as isize);
        *fresh0 = (*fresh0 as std::ffi::c_int + 1 as std::ffi::c_int) as u16;
        if (activeSegment.end).wrapping_sub(activeSegment.begin)
            == dmersInK.wrapping_add(1)
        {
            let delIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(activeSegment.begin as isize)
                    as *const std::ffi::c_void,
                f,
                d,
            );
            let ref mut fresh1 = *segmentFreqs.offset(delIndex as isize);
            *fresh1 = (*fresh1 as std::ffi::c_int - 1 as std::ffi::c_int) as u16;
            if *segmentFreqs.offset(delIndex as isize) as std::ffi::c_int
                == 0
            {
                activeSegment
                    .score = (activeSegment.score)
                    .wrapping_sub(*freqs.offset(delIndex as isize));
            }
            activeSegment
                .begin = (activeSegment.begin).wrapping_add(1);
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    while activeSegment.begin < end {
        let delIndex_0 = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(activeSegment.begin as isize)
                as *const std::ffi::c_void,
            f,
            d,
        );
        let ref mut fresh2 = *segmentFreqs.offset(delIndex_0 as isize);
        *fresh2 = (*fresh2 as std::ffi::c_int - 1 as std::ffi::c_int) as u16;
        activeSegment
            .begin = (activeSegment.begin).wrapping_add(1);
    }
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let i = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(pos as isize) as *const std::ffi::c_void,
            f,
            d,
        );
        *freqs.offset(i as isize) = 0;
        pos = pos.wrapping_add(1);
        pos;
    }
    return bestSegment;
}
unsafe extern "C" fn FASTCOVER_checkParameters(
    mut parameters: ZDICT_cover_params_t,
    mut maxDictSize: usize,
    mut f: std::ffi::c_uint,
    mut accel: std::ffi::c_uint,
) -> std::ffi::c_int {
    if parameters.d == 0
        || parameters.k == 0
    {
        return 0;
    }
    if parameters.d != 6
        && parameters.d != 8
    {
        return 0;
    }
    if parameters.k as usize > maxDictSize {
        return 0;
    }
    if parameters.d > parameters.k {
        return 0;
    }
    if f > FASTCOVER_MAX_F as std::ffi::c_uint
        || f == 0
    {
        return 0;
    }
    if parameters.splitPoint <= 0
        || parameters.splitPoint > 1
    {
        return 0;
    }
    if accel > 10
        || accel == 0
    {
        return 0;
    }
    return 1;
}
unsafe extern "C" fn FASTCOVER_ctx_destroy(mut ctx: *mut FASTCOVER_ctx_t) {
    if ctx.is_null() {
        return;
    }
    free((*ctx).freqs as *mut std::ffi::c_void);
    (*ctx).freqs = NULL as *mut u32;
    free((*ctx).offsets as *mut std::ffi::c_void);
    (*ctx).offsets = NULL as *mut usize;
}
unsafe extern "C" fn FASTCOVER_computeFrequency(
    mut freqs: *mut u32,
    mut ctx: *const FASTCOVER_ctx_t,
) {
    let f = (*ctx).f;
    let d = (*ctx).d;
    let skip = (*ctx).accelParams.skip;
    let readLength = if d > 8 {
        d
    } else {
        8 as std::ffi::c_uint
    };
    let mut i: usize = 0;
    i = 0;
    while i < (*ctx).nbTrainSamples {
        let mut start = *((*ctx).offsets).offset(i as isize);
        let currSampleEnd = *((*ctx).offsets)
            .offset(i.wrapping_add(1) as isize);
        while start.wrapping_add(readLength as usize) <= currSampleEnd {
            let dmerIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(start as isize) as *const std::ffi::c_void,
                f,
                d,
            );
            let ref mut fresh3 = *freqs.offset(dmerIndex as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            *fresh3;
            start = start
                .wrapping_add(skip as usize)
                .wrapping_add(1);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn FASTCOVER_ctx_init(
    mut ctx: *mut FASTCOVER_ctx_t,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
    mut d: std::ffi::c_uint,
    mut splitPoint: std::ffi::c_double,
    mut f: std::ffi::c_uint,
    mut accelParams: FASTCOVER_accel_t,
    mut displayLevel: std::ffi::c_int,
) -> usize {
    let samples = samplesBuffer as *const u8;
    let totalSamplesSize = COVER_sum(samplesSizes, nbSamples);
    let nbTrainSamples = if splitPoint < 1.0f64 {
        (nbSamples as std::ffi::c_double * splitPoint) as std::ffi::c_uint
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
    (*ctx).displayLevel = displayLevel;
    if totalSamplesSize
        < (if d as std::ffi::c_ulong > ::core::mem::size_of::<u64>()
        {
            d as std::ffi::c_ulong
        } else {
            ::core::mem::size_of::<u64>()
        })
        || totalSamplesSize
            >= (if ::core::mem::size_of::<usize>()
                == 8
            {
                -(1 as std::ffi::c_int) as std::ffi::c_uint
            } else {
                (1 as std::ffi::c_uint)
                    .wrapping_mul((1 as std::ffi::c_uint) << 30)
            }) as usize
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total samples size is too large (%u MB), maximum size is %u MB\n\0"
                    as *const u8 as *const std::ffi::c_char,
                (totalSamplesSize >> 20) as std::ffi::c_uint,
                (if ::core::mem::size_of::<usize>()
                    == 8
                {
                    -(1 as std::ffi::c_int) as std::ffi::c_uint
                } else {
                    (1 as std::ffi::c_uint)
                        .wrapping_mul((1 as std::ffi::c_uint) << 30)
                }) >> 20,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as usize;
    }
    if nbTrainSamples < 5 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total number of training samples is %u and is invalid\n\0" as *const u8
                    as *const std::ffi::c_char,
                nbTrainSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as usize;
    }
    if nbTestSamples < 1 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total number of testing samples is %u and is invalid.\n\0" as *const u8
                    as *const std::ffi::c_char,
                nbTestSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as usize;
    }
    libc::memset(
        ctx as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<FASTCOVER_ctx_t>(),
    );
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Training on %u samples of total size %u\n\0" as *const u8
                as *const std::ffi::c_char,
            nbTrainSamples,
            trainingSamplesSize as std::ffi::c_uint,
        );
        fflush(stderr);
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Testing on %u samples of total size %u\n\0" as *const u8
                as *const std::ffi::c_char,
            nbTestSamples,
            testSamplesSize as std::ffi::c_uint,
        );
        fflush(stderr);
    }
    (*ctx).samples = samples;
    (*ctx).samplesSizes = samplesSizes;
    (*ctx).nbSamples = nbSamples as usize;
    (*ctx).nbTrainSamples = nbTrainSamples as usize;
    (*ctx).nbTestSamples = nbTestSamples as usize;
    (*ctx)
        .nbDmers = trainingSamplesSize
        .wrapping_sub(
            (if d as usize
                > ::core::mem::size_of::<u64>()
            {
                d
            } else {
                ::core::mem::size_of::<u64>()
            }),
        )
        .wrapping_add(1);
    (*ctx).d = d;
    (*ctx).f = f;
    (*ctx).accelParams = accelParams;
    (*ctx)
        .offsets = libc::calloc(
        nbSamples.wrapping_add(1)
            as usize,
        ::core::mem::size_of::<usize>(),
    ) as *mut usize;
    if ((*ctx).offsets).is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to allocate scratch buffers \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as usize;
    }
    let mut i: u32 = 0;
    *((*ctx).offsets)
        .offset(0) = 0;
    i = 1;
    while i <= nbSamples {
        *((*ctx).offsets)
            .offset(
                i as isize,
            ) = (*((*ctx).offsets)
            .offset(i.wrapping_sub(1) as isize))
            .wrapping_add(
                *samplesSizes
                    .offset(i.wrapping_sub(1) as isize),
            );
        i = i.wrapping_add(1);
        i;
    }
    (*ctx)
        .freqs = libc::calloc(
        (1_usize) << f,
        ::core::mem::size_of::<u32>(),
    ) as *mut u32;
    if ((*ctx).freqs).is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to allocate frequency table \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as usize;
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Computing frequencies\n\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    FASTCOVER_computeFrequency((*ctx).freqs, ctx);
    return 0;
}
unsafe extern "C" fn FASTCOVER_buildDictionary(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut u32,
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: usize,
    mut parameters: ZDICT_cover_params_t,
    mut segmentFreqs: *mut u16,
) -> usize {
    let dict = dictBuffer as *mut u8;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as u32,
        (*ctx).nbDmers as u32,
        parameters.k,
        1,
    );
    let maxZeroScoreRun = 10;
    let displayLevel = (*ctx).displayLevel;
    let mut zeroScoreRun: usize = 0;
    let mut lastUpdateTime: clock_t = 0;
    let mut epoch: usize = 0;
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Breaking content into %u epochs of size %u\n\0" as *const u8
                as *const std::ffi::c_char,
            epochs.num,
            epochs.size,
        );
        fflush(stderr);
    }
    epoch = 0;
    while tail > 0 {
        let epochBegin = (epoch * epochs.size as usize) as u32;
        let epochEnd = epochBegin.wrapping_add(epochs.size);
        let mut segmentSize: usize = 0;
        let mut segment = FASTCOVER_selectSegment(
            ctx,
            freqs,
            epochBegin,
            epochEnd,
            parameters,
            segmentFreqs,
        );
        if segment.score == 0 {
            zeroScoreRun = zeroScoreRun.wrapping_add(1);
            if zeroScoreRun >= maxZeroScoreRun {
                break;
            }
        } else {
            zeroScoreRun = 0;
            segmentSize = if ((segment.end)
                .wrapping_sub(segment.begin)
                .wrapping_add(parameters.d)
                .wrapping_sub(1) as usize) < tail
            {
                (segment.end)
                    .wrapping_sub(segment.begin)
                    .wrapping_add(parameters.d)
                    .wrapping_sub(1) as usize
            } else {
                tail
            };
            if segmentSize < parameters.d as usize {
                break;
            }
            tail = tail.wrapping_sub(segmentSize);
            libc::memcpy(
                dict.offset(tail as isize) as *mut std::ffi::c_void,
                ((*ctx).samples).offset(segment.begin as isize)
                    as *const std::ffi::c_void,
                segmentSize,
            );
            if displayLevel >= 2 {
                let refreshRate = CLOCKS_PER_SEC as __clock_t
                    * 15 as __clock_t
                    / 100;
                if clock() - lastUpdateTime > refreshRate
                    || displayLevel >= 4
                {
                    lastUpdateTime = clock();
                    fprintf(
                        stderr,
                        b"\r%u%%       \0" as *const u8 as *const std::ffi::c_char,
                        (dictBufferCapacity.wrapping_sub(tail)
                            * 100 as usize / dictBufferCapacity)
                            as std::ffi::c_uint,
                    );
                    fflush(stderr);
                }
            }
        }
        epoch = epoch.wrapping_add(1)
            % epochs.num as usize;
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    return tail;
}
unsafe extern "C" fn FASTCOVER_tryParameters(mut opaque: *mut std::ffi::c_void) {
    let data = opaque as *mut FASTCOVER_tryParameters_data_t;
    let ctx = (*data).ctx;
    let parameters = (*data).parameters;
    let mut dictBufferCapacity = (*data).dictBufferCapacity;
    let mut totalCompressedSize = -(ZSTD_error_GENERIC as std::ffi::c_int) as usize;
    let mut segmentFreqs = libc::calloc(
        (1_usize) << (*ctx).f,
        ::core::mem::size_of::<u16>(),
    ) as *mut u16;
    let dict = libc::malloc(dictBufferCapacity) as *mut u8;
    let mut selection = COVER_dictSelectionError(
        -(ZSTD_error_GENERIC as std::ffi::c_int) as usize,
    );
    let mut freqs = libc::malloc(
        (1_usize << (*ctx).f)
            .wrapping_mul(::core::mem::size_of::<u32>()),
    ) as *mut u32;
    let displayLevel = (*ctx).displayLevel;
    if segmentFreqs.is_null() || dict.is_null() || freqs.is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to allocate buffers: out of memory\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
    } else {
        libc::memcpy(
            freqs as *mut std::ffi::c_void,
            (*ctx).freqs as *const std::ffi::c_void,
            (1_usize << (*ctx).f)
                .wrapping_mul(::core::mem::size_of::<u32>()),
        );
        let tail = FASTCOVER_buildDictionary(
            ctx,
            freqs,
            dict as *mut std::ffi::c_void,
            dictBufferCapacity,
            parameters,
            segmentFreqs,
        );
        let nbFinalizeSamples = ((*ctx).nbTrainSamples
            * (*ctx).accelParams.finalize as usize / 100 as usize)
            as std::ffi::c_uint;
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
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Failed to select dictionary\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
                fflush(stderr);
            }
        }
    }
    free(dict as *mut std::ffi::c_void);
    COVER_best_finish((*data).best, parameters, selection);
    free(data as *mut std::ffi::c_void);
    free(segmentFreqs as *mut std::ffi::c_void);
    COVER_dictSelectionFree(selection);
    free(freqs as *mut std::ffi::c_void);
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
    mut f: std::ffi::c_uint,
    mut accel: std::ffi::c_uint,
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
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: usize,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
    mut parameters: ZDICT_fastCover_params_t,
) -> usize {
    let dict = dictBuffer as *mut u8;
    let mut ctx = FASTCOVER_ctx_t {
        samples: 0 as *const u8,
        offsets: 0 as *mut usize,
        samplesSizes: 0 as *const usize,
        nbSamples: 0,
        nbTrainSamples: 0,
        nbTestSamples: 0,
        nbDmers: 0,
        freqs: 0 as *mut u32,
        d: 0,
        f: 0,
        accelParams: FASTCOVER_accel_t {
            finalize: 0,
            skip: 0,
        },
        displayLevel: 0,
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
    let displayLevel = parameters.zParams.notificationLevel as std::ffi::c_int;
    parameters.splitPoint = 1.0f64;
    parameters
        .f = if parameters.f == 0 {
        DEFAULT_F as std::ffi::c_uint
    } else {
        parameters.f
    };
    parameters
        .accel = if parameters.accel == 0 {
        DEFAULT_ACCEL as std::ffi::c_uint
    } else {
        parameters.accel
    };
    libc::memset(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    FASTCOVER_convertToCoverParams(parameters, &mut coverParams);
    if FASTCOVER_checkParameters(
        coverParams,
        dictBufferCapacity,
        parameters.f,
        parameters.accel,
    ) == 0
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"FASTCOVER parameters incorrect\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as usize;
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"FASTCOVER must have at least one input file\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as usize;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as usize {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const std::ffi::c_char,
                256,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as usize;
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
        displayLevel,
    );
    if ERR_isError(initVal) != 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to initialize context\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Building dictionary\n\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    let mut segmentFreqs = libc::calloc(
        (1_usize) << parameters.f,
        ::core::mem::size_of::<u16>(),
    ) as *mut u16;
    let tail = FASTCOVER_buildDictionary(
        &mut ctx,
        ctx.freqs,
        dictBuffer,
        dictBufferCapacity,
        coverParams,
        segmentFreqs,
    );
    let nbFinalizeSamples = (ctx.nbTrainSamples * ctx.accelParams.finalize as usize
        / 100 as usize) as std::ffi::c_uint;
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut std::ffi::c_void,
        dictBufferCapacity,
        dict.offset(tail as isize) as *const std::ffi::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes,
        nbFinalizeSamples,
        coverParams.zParams,
    );
    if ERR_isError(dictionarySize) == 0 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"Constructed dictionary of size %u\n\0" as *const u8
                    as *const std::ffi::c_char,
                dictionarySize as std::ffi::c_uint,
            );
            fflush(stderr);
        }
    }
    FASTCOVER_ctx_destroy(&mut ctx);
    free(segmentFreqs as *mut std::ffi::c_void);
    return dictionarySize;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_fastCover(
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: usize,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
    mut parameters: *mut ZDICT_fastCover_params_t,
) -> usize {
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
    let kMinD = if (*parameters).d == 0 {
        6 as std::ffi::c_uint
    } else {
        (*parameters).d
    };
    let kMaxD = if (*parameters).d == 0 {
        8 as std::ffi::c_uint
    } else {
        (*parameters).d
    };
    let kMinK = if (*parameters).k == 0 {
        50 as std::ffi::c_uint
    } else {
        (*parameters).k
    };
    let kMaxK = if (*parameters).k == 0 {
        2000 as std::ffi::c_uint
    } else {
        (*parameters).k
    };
    let kSteps = if (*parameters).steps == 0 {
        40 as std::ffi::c_uint
    } else {
        (*parameters).steps
    };
    let kStepSize = if kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
        > 1
    {
        kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
    } else {
        1 as std::ffi::c_uint
    };
    let kIterations = (1 as std::ffi::c_uint)
        .wrapping_add(
            kMaxD
                .wrapping_sub(kMinD)
                .wrapping_div(2),
        )
        .wrapping_mul(
            (1 as std::ffi::c_uint)
                .wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)),
        );
    let f = if (*parameters).f == 0 {
        DEFAULT_F as std::ffi::c_uint
    } else {
        (*parameters).f
    };
    let accel = if (*parameters).accel == 0 {
        DEFAULT_ACCEL as std::ffi::c_uint
    } else {
        (*parameters).accel
    };
    let shrinkDict = 0;
    let displayLevel = (*parameters).zParams.notificationLevel as std::ffi::c_int;
    let mut iteration: std::ffi::c_uint = 1;
    let mut d: std::ffi::c_uint = 0;
    let mut k: std::ffi::c_uint = 0;
    let mut best = COVER_best_s {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
        cond: pthread_cond_t {
            __data: __pthread_cond_s {
                __wseq: __atomic_wide_counter {
                    __value64: 0,
                },
                __g1_start: __atomic_wide_counter {
                    __value64: 0,
                },
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
                __unused_initialized_1: 0,
                __unused_initialized_2: 0,
            },
        },
        liveJobs: 0,
        dict: 0 as *mut std::ffi::c_void,
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
    let mut warned: std::ffi::c_int = 0;
    let mut lastUpdateTime: clock_t = 0;
    if splitPoint <= 0
        || splitPoint > 1
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Incorrect splitPoint\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as usize;
    }
    if accel == 0
        || accel > FASTCOVER_MAX_ACCEL as std::ffi::c_uint
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Incorrect accel\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as usize;
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1 {
            fprintf(stderr, b"Incorrect k\n\0" as *const u8 as *const std::ffi::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as usize;
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"FASTCOVER must have at least one input file\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as usize;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as usize {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const std::ffi::c_char,
                256,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as usize;
    }
    if nbThreads > 1 {
        pool = POOL_create(nbThreads as usize, 1);
        if pool.is_null() {
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as usize;
        }
    }
    COVER_best_init(&mut best);
    libc::memset(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    FASTCOVER_convertToCoverParams(*parameters, &mut coverParams);
    accelParams = FASTCOVER_defaultAccelParameters[accel as usize];
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Trying %u different sets of parameters\n\0" as *const u8
                as *const std::ffi::c_char,
            kIterations,
        );
        fflush(stderr);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = FASTCOVER_ctx_t {
            samples: 0 as *const u8,
            offsets: 0 as *mut usize,
            samplesSizes: 0 as *const usize,
            nbSamples: 0,
            nbTrainSamples: 0,
            nbTestSamples: 0,
            nbDmers: 0,
            freqs: 0 as *mut u32,
            d: 0,
            f: 0,
            accelParams: FASTCOVER_accel_t {
                finalize: 0,
                skip: 0,
            },
            displayLevel: 0,
        };
        if displayLevel >= 3 {
            fprintf(stderr, b"d=%u\n\0" as *const u8 as *const std::ffi::c_char, d);
            fflush(stderr);
        }
        let childDisplayLevel = if displayLevel == 0 {
            0 as std::ffi::c_int
        } else {
            displayLevel - 1 as std::ffi::c_int
        };
        let initVal = FASTCOVER_ctx_init(
            &mut ctx,
            samplesBuffer,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            f,
            accelParams,
            childDisplayLevel,
        );
        if ERR_isError(initVal) != 0 {
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Failed to initialize context\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
                fflush(stderr);
            }
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
            warned = 1;
        }
        k = kMinK;
        while k <= kMaxK {
            let mut data = libc::malloc(
                ::core::mem::size_of::<FASTCOVER_tryParameters_data_t>(),
            ) as *mut FASTCOVER_tryParameters_data_t;
            if displayLevel >= 3 {
                fprintf(stderr, b"k=%u\n\0" as *const u8 as *const std::ffi::c_char, k);
                fflush(stderr);
            }
            if data.is_null() {
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Failed to allocate parameters\n\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                    fflush(stderr);
                }
                COVER_best_destroy(&mut best);
                FASTCOVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as usize;
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
                .notificationLevel = ctx.displayLevel as std::ffi::c_uint;
            if FASTCOVER_checkParameters(
                (*data).parameters,
                dictBufferCapacity,
                (*(*data).ctx).f,
                accel,
            ) == 0
            {
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"FASTCOVER parameters incorrect\n\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                    fflush(stderr);
                }
                free(data as *mut std::ffi::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        Some(
                            FASTCOVER_tryParameters
                                as unsafe extern "C" fn(*mut std::ffi::c_void) -> (),
                        ),
                        data as *mut std::ffi::c_void,
                    );
                } else {
                    FASTCOVER_tryParameters(data as *mut std::ffi::c_void);
                }
                if displayLevel >= 2 {
                    let refreshRate = CLOCKS_PER_SEC as __clock_t
                        * 15 as __clock_t
                        / 100;
                    if clock() - lastUpdateTime > refreshRate
                        || displayLevel >= 4
                    {
                        lastUpdateTime = clock();
                        fprintf(
                            stderr,
                            b"\r%u%%       \0" as *const u8 as *const std::ffi::c_char,
                            iteration
                                .wrapping_mul(100)
                                .wrapping_div(kIterations),
                        );
                        fflush(stderr);
                    }
                }
                iteration = iteration.wrapping_add(1);
                iteration;
            }
            k = k.wrapping_add(kStepSize);
        }
        COVER_best_wait(&mut best);
        FASTCOVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2);
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
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
    libc::memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    return dictSize;
}
