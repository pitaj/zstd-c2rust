use std::ffi::{c_char, c_void};
use crate::common::zstd_internal_h::*;

/*-*************************************
*  Constants
***************************************/
/**
* There are 32bit indexes used to ref samples, so limit samples size to 4GB
* on 64bit builds.
* For 32bit builds we choose 1 GB.
* Most 32bit platforms have 2GB user-mode addressable space and we allocate a large
* contiguous buffer, so 1GB is already a high limit.
*/
const FASTCOVER_MAX_SAMPLES_SIZE: usize = if ::core::mem::size_of::<usize>() == 8 {
    u32::MAX
} else {
    GB(1)
};
const FASTCOVER_MAX_F: u32 = 31;
const FASTCOVER_MAX_ACCEL: u32 = 10;
const FASTCOVER_DEFAULT_SPLITPOINT: f64 = 0.75;
const DEFAULT_F: u32 = 20;
const DEFAULT_ACCEL: u32 = 1;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type POOL_ctx_s;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const c_char, _: ...) -> i32;
    fn clock() -> clock_t;
    fn POOL_create(numThreads: usize, queueSize: usize) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_add(
        ctx: *mut POOL_ctx,
        function: POOL_function,
        opaque: *mut c_void,
    );
    fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut c_void,
        maxDictSize: usize,
        dictContent: *const c_void,
        dictContentSize: usize,
        samplesBuffer: *const c_void,
        samplesSizes: *const usize,
        nbSamples: u32,
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
        displayLevel: i32,
    );
    fn COVER_sum(samplesSizes: *const usize, nbSamples: u32) -> usize;
    fn COVER_best_init(best: *mut COVER_best_t);
    fn COVER_best_wait(best: *mut COVER_best_t);
    fn COVER_best_destroy(best: *mut COVER_best_t);
    fn COVER_best_start(best: *mut COVER_best_t);
    fn COVER_best_finish(
        best: *mut COVER_best_t,
        parameters: ZDICT_cover_params_t,
        selection: COVER_dictSelection_t,
    );
    fn COVER_dictSelectionIsError(selection: COVER_dictSelection_t) -> u32;
    fn COVER_dictSelectionError(error: usize) -> COVER_dictSelection_t;
    fn COVER_dictSelectionFree(selection: COVER_dictSelection_t);
    fn COVER_selectDict(
        customDictContent: *mut u8,
        dictBufferCapacity: usize,
        dictContentSize: usize,
        samplesBuffer: *const u8,
        samplesSizes: *const usize,
        nbFinalizeSamples: u32,
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
    pub _flags: i32,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    #[bitfield(name = "_flags2", ty = "i32", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: i32,
    pub _unused2: [c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
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
pub type unalign64 = u64;
use crate::common::error::*;
pub type POOL_ctx = POOL_ctx_s;
pub type POOL_function = Option::<unsafe extern "C" fn(*mut c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: i32,
    pub notificationLevel: u32,
    pub dictID: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: u32,
    pub d: u32,
    pub steps: u32,
    pub nbThreads: u32,
    pub splitPoint: std::ffi::c_double,
    pub shrinkDict: u32,
    pub shrinkDictMaxRegression: u32,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: u32,
    pub d: u32,
    pub f: u32,
    pub steps: u32,
    pub nbThreads: u32,
    pub splitPoint: std::ffi::c_double,
    pub accel: u32,
    pub shrinkDict: u32,
    pub shrinkDictMaxRegression: u32,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_accel_t {
    pub finalize: u32,
    pub skip: u32,
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
    pub d: u32,
    pub f: u32,
    pub accelParams: FASTCOVER_accel_t,
    pub displayLevel: i32,
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
    pub dict: *mut c_void,
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
use crate::common::mem::*;
use crate::compress::zstd_compress_internal::*;
pub const ZDICT_DICTSIZE_MIN: i32 = 256;
pub const CLOCKS_PER_SEC: i32 = 1000000;
pub const NULL: i32 = 0;
unsafe extern "C" fn FASTCOVER_hashPtrToIndex(
    mut p: *const c_void,
    mut f: u32,
    mut d: u32,
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
                as *const c_void,
            f,
            d,
        );
        if *segmentFreqs.offset(idx as isize) as i32 == 0
        {
            activeSegment
                .score = (activeSegment.score).wrapping_add(*freqs.offset(idx as isize));
        }
        activeSegment
            .end = (activeSegment.end).wrapping_add(1);
        let ref mut fresh0 = *segmentFreqs.offset(idx as isize);
        *fresh0 = (*fresh0 as i32 + 1 as i32) as u16;
        if (activeSegment.end).wrapping_sub(activeSegment.begin)
            == dmersInK.wrapping_add(1)
        {
            let delIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(activeSegment.begin as isize)
                    as *const c_void,
                f,
                d,
            );
            let ref mut fresh1 = *segmentFreqs.offset(delIndex as isize);
            *fresh1 = (*fresh1 as i32 - 1 as i32) as u16;
            if *segmentFreqs.offset(delIndex as isize) as i32
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
                as *const c_void,
            f,
            d,
        );
        let ref mut fresh2 = *segmentFreqs.offset(delIndex_0 as isize);
        *fresh2 = (*fresh2 as i32 - 1 as i32) as u16;
        activeSegment
            .begin = (activeSegment.begin).wrapping_add(1);
    }
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let i = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(pos as isize) as *const c_void,
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
    mut f: u32,
    mut accel: u32,
) -> i32 {
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
    if f > FASTCOVER_MAX_F
        || f == 0
    {
        return 0;
    }
    if parameters.splitPoint <= 0.0
        || parameters.splitPoint > 1.0
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
    libc::free((*ctx).freqs as *mut c_void);
    (*ctx).freqs = std::ptr::null_mut();
    libc::free((*ctx).offsets as *mut c_void);
    (*ctx).offsets = std::ptr::null_mut();
}
unsafe extern "C" fn FASTCOVER_computeFrequency(
    mut freqs: *mut u32,
    mut ctx: *const FASTCOVER_ctx_t,
) {
    let f = (*ctx).f;
    let d = (*ctx).d;
    let skip = (*ctx).accelParams.skip;
    let readLength = std::cmp::max(d, 8);
    let mut i: usize = 0;
    i = 0;
    while i < (*ctx).nbTrainSamples {
        let mut start = *((*ctx).offsets).offset(i as isize);
        let currSampleEnd = *((*ctx).offsets)
            .offset(i.wrapping_add(1) as isize);
        while start.wrapping_add(readLength as usize) <= currSampleEnd {
            let dmerIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(start as isize) as *const c_void,
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
    mut samplesBuffer: *const c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: u32,
    mut d: u32,
    mut splitPoint: std::ffi::c_double,
    mut f: u32,
    mut accelParams: FASTCOVER_accel_t,
    mut displayLevel: i32,
) -> usize {
    let samples = samplesBuffer as *const u8;
    let totalSamplesSize = COVER_sum(samplesSizes, nbSamples);
    let nbTrainSamples = if splitPoint < 1.0f64 {
        (nbSamples as std::ffi::c_double * splitPoint) as u32
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
        < std::cmp::max(d as usize, ::core::mem::size_of::<u64>())
        || totalSamplesSize >= FASTCOVER_MAX_SAMPLES_SIZE
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total samples size is too large (%u MB), maximum size is %u MB\n\0"
                    as *const u8 as *const c_char,
                (totalSamplesSize >> 20) as u32,
                FASTCOVER_MAX_SAMPLES_SIZE >> 20,
            );
            fflush(stderr);
        }
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if nbTrainSamples < 5 {
        DISPLAYLEVEL!(1, "Total number of training samples is %u and is invalid\n", nbTrainSamples);
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if nbTestSamples < 1 {
        DISPLAYLEVEL!(1, "Total number of testing samples is %u and is invalid.\n", nbTestSamples);
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    libc::memset(
        ctx as *mut c_void,
        0,
        ::core::mem::size_of::<FASTCOVER_ctx_t>(),
    );
    DISPLAYLEVEL!(2, "Training on %u samples of total size %u\n", nbTrainSamples, (unsigned)
        trainingSamplesSize);
    DISPLAYLEVEL!(2, "Testing on %u samples of total size %u\n", nbTestSamples, (unsigned)
        testSamplesSize);
    (*ctx).samples = samples;
    (*ctx).samplesSizes = samplesSizes;
    (*ctx).nbSamples = nbSamples as usize;
    (*ctx).nbTrainSamples = nbTrainSamples as usize;
    (*ctx).nbTestSamples = nbTestSamples as usize;
    (*ctx)
        .nbDmers = trainingSamplesSize
        .wrapping_sub(std::cmp::max(d as usize, ::core::mem::size_of::<u64>()))
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
        DISPLAYLEVEL!(1, "Failed to allocate scratch buffers \n");
        FASTCOVER_ctx_destroy(ctx);
        return ERROR(ZSTD_error_memory_allocation);
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
        1_usize << f,
        ::core::mem::size_of::<u32>(),
    ) as *mut u32;
    if ((*ctx).freqs).is_null() {
        DISPLAYLEVEL!(1, "Failed to allocate frequency table \n");
        FASTCOVER_ctx_destroy(ctx);
        return ERROR(ZSTD_error_memory_allocation);
    }
    DISPLAYLEVEL!(2, "Computing frequencies\n");
    FASTCOVER_computeFrequency((*ctx).freqs, ctx);
    return 0;
}
unsafe extern "C" fn FASTCOVER_buildDictionary(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut u32,
    mut dictBuffer: *mut c_void,
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
    DISPLAYLEVEL!(2, "Breaking content into %u epochs of size %u\n", (u32) epochs.num, (u32) epochs
        .size);
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
            segmentSize = std::cmp::min((segment.end - segment.begin + parameters.d - 1) as usize, tail);
            if segmentSize < parameters.d as usize {
                break;
            }
            tail = tail.wrapping_sub(segmentSize);
            libc::memcpy(
                dict.offset(tail as isize) as *mut c_void,
                ((*ctx).samples).offset(segment.begin as isize)
                    as *const c_void,
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
                        b"\r%u%%       \0" as *const u8 as *const c_char,
                        (dictBufferCapacity.wrapping_sub(tail)
                            * 100_usize / dictBufferCapacity)
                            as u32,
                    );
                    fflush(stderr);
                }
            }
        }
        epoch = epoch.wrapping_add(1)
            % epochs.num as usize;
    }
    DISPLAYLEVEL!(2, "\r%79s\r", "");
    return tail;
}
unsafe extern "C" fn FASTCOVER_tryParameters(mut opaque: *mut c_void) {
    let data = opaque as *mut FASTCOVER_tryParameters_data_t;
    let ctx = (*data).ctx;
    let parameters = (*data).parameters;
    let mut dictBufferCapacity = (*data).dictBufferCapacity;
    let mut totalCompressedSize = ERROR(ZSTD_error_GENERIC);
    let mut segmentFreqs = libc::calloc(
        1_usize << (*ctx).f,
        ::core::mem::size_of::<u16>(),
    ) as *mut u16;
    let dict = libc::malloc(dictBufferCapacity) as *mut u8;
    let mut selection = COVER_dictSelectionError(ERROR(ZSTD_error_GENERIC));
    let mut freqs = libc::malloc(
        (1_usize << (*ctx).f)
            .wrapping_mul(::core::mem::size_of::<u32>()),
    ) as *mut u32;
    let displayLevel = (*ctx).displayLevel;
    if segmentFreqs.is_null() || dict.is_null() || freqs.is_null() {
        DISPLAYLEVEL!(1, "Failed to allocate buffers: out of memory\n");
    } else {
        libc::memcpy(
            freqs as *mut c_void,
            (*ctx).freqs as *const c_void,
            (1_usize << (*ctx).f)
                .wrapping_mul(::core::mem::size_of::<u32>()),
        );
        let tail = FASTCOVER_buildDictionary(
            ctx,
            freqs,
            dict as *mut c_void,
            dictBufferCapacity,
            parameters,
            segmentFreqs,
        );
        let nbFinalizeSamples = ((*ctx).nbTrainSamples
            * (*ctx).accelParams.finalize as usize / 100_usize)
            as u32;
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
            DISPLAYLEVEL!(1, "Failed to select dictionary\n");
        }
    }
    libc::free(dict as *mut c_void);
    COVER_best_finish((*data).best, parameters, selection);
    libc::free(data as *mut c_void);
    libc::free(segmentFreqs as *mut c_void);
    COVER_dictSelectionFree(selection);
    libc::free(freqs as *mut c_void);
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
    mut f: u32,
    mut accel: u32,
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
    mut dictBuffer: *mut c_void,
    mut dictBufferCapacity: usize,
    mut samplesBuffer: *const c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: u32,
    mut parameters: ZDICT_fastCover_params_t,
) -> usize {
    let dict = dictBuffer as *mut u8;
    let mut ctx = FASTCOVER_ctx_t {
        samples: std::ptr::null(),
        offsets: std::ptr::null_mut(),
        samplesSizes: std::ptr::null(),
        nbSamples: 0,
        nbTrainSamples: 0,
        nbTestSamples: 0,
        nbDmers: 0,
        freqs: std::ptr::null_mut(),
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
    let displayLevel = parameters.zParams.notificationLevel as i32;
    parameters.splitPoint = 1.0f64;
    parameters
        .f = if parameters.f == 0 {
        DEFAULT_F
    } else {
        parameters.f
    };
    parameters
        .accel = if parameters.accel == 0 {
        DEFAULT_ACCEL
    } else {
        parameters.accel
    };
    libc::memset(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut c_void,
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
        DISPLAYLEVEL!(1, "FASTCOVER parameters incorrect\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if nbSamples == 0 {
        DISPLAYLEVEL!(1, "FASTCOVER must have at least one input file\n");
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as usize {
        DISPLAYLEVEL!(1, "dictBufferCapacity must be at least %u\n", ZDICT_DICTSIZE_MIN);
        return ERROR(ZSTD_error_dstSize_tooSmall);
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
    if ERR_isError(initVal) {
        DISPLAYLEVEL!(1, "Failed to initialize context\n");
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
    DISPLAYLEVEL!(2, "Building dictionary\n");
    let mut segmentFreqs = libc::calloc(
        1_usize << parameters.f,
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
        / 100_usize) as u32;
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut c_void,
        dictBufferCapacity,
        dict.offset(tail as isize) as *const c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes,
        nbFinalizeSamples,
        coverParams.zParams,
    );
    if !ERR_isError(dictionarySize) {
        DISPLAYLEVEL!(2, "Constructed dictionary of size %u\n", (unsigned) dictionarySize);
    }
    FASTCOVER_ctx_destroy(&mut ctx);
    libc::free(segmentFreqs as *mut c_void);
    return dictionarySize;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_fastCover(
    mut dictBuffer: *mut c_void,
    mut dictBufferCapacity: usize,
    mut samplesBuffer: *const c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: u32,
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
        6 as u32
    } else {
        (*parameters).d
    };
    let kMaxD = if (*parameters).d == 0 {
        8 as u32
    } else {
        (*parameters).d
    };
    let kMinK = if (*parameters).k == 0 {
        50 as u32
    } else {
        (*parameters).k
    };
    let kMaxK = if (*parameters).k == 0 {
        2000 as u32
    } else {
        (*parameters).k
    };
    let kSteps = if (*parameters).steps == 0 {
        40 as u32
    } else {
        (*parameters).steps
    };
    let kStepSize = std::cmp::max((kMaxK - kMinK) / kSteps, 1);
    let kIterations = (1 as u32)
        .wrapping_add(
            kMaxD
                .wrapping_sub(kMinD)
                .wrapping_div(2),
        )
        .wrapping_mul(
            (1 as u32)
                .wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)),
        );
    let f = if (*parameters).f == 0 {
        DEFAULT_F
    } else {
        (*parameters).f
    };
    let accel = if (*parameters).accel == 0 {
        DEFAULT_ACCEL
    } else {
        (*parameters).accel
    };
    let shrinkDict = 0;
    let displayLevel = (*parameters).zParams.notificationLevel as i32;
    let mut iteration: u32 = 1;
    let mut d: u32 = 0;
    let mut k: u32 = 0;
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
                    __prev: std::ptr::null_mut(),
                    __next: std::ptr::null_mut(),
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
        dict: std::ptr::null_mut(),
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
    let mut pool = std::ptr::null_mut();
    let mut warned: i32 = 0;
    let mut lastUpdateTime: clock_t = 0;
    if splitPoint <= 0.0
        || splitPoint > 1.0
    {
        DISPLAYLEVEL!(1, "Incorrect splitPoint\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if accel == 0
        || accel > FASTCOVER_MAX_ACCEL
    {
        DISPLAYLEVEL!(1, "Incorrect accel\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        DISPLAYLEVEL!(1, "Incorrect k\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if nbSamples == 0 {
        DISPLAYLEVEL!(1, "FASTCOVER must have at least one input file\n");
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as usize {
        DISPLAYLEVEL!(1, "dictBufferCapacity must be at least %u\n", ZDICT_DICTSIZE_MIN);
        return ERROR(ZSTD_error_dstSize_tooSmall);
    }
    if nbThreads > 1 {
        pool = POOL_create(nbThreads as usize, 1);
        RETURN_ERROR_IF!(pool.is_null(), ZSTD_error_memory_allocation);
    }
    COVER_best_init(&mut best);
    libc::memset(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut c_void,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    FASTCOVER_convertToCoverParams(*parameters, &mut coverParams);
    accelParams = FASTCOVER_defaultAccelParameters[accel as usize];
    DISPLAYLEVEL!(2, "Trying %u different sets of parameters\n", kIterations);
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = FASTCOVER_ctx_t {
            samples: std::ptr::null(),
            offsets: std::ptr::null_mut(),
            samplesSizes: std::ptr::null(),
            nbSamples: 0,
            nbTrainSamples: 0,
            nbTestSamples: 0,
            nbDmers: 0,
            freqs: std::ptr::null_mut(),
            d: 0,
            f: 0,
            accelParams: FASTCOVER_accel_t {
                finalize: 0,
                skip: 0,
            },
            displayLevel: 0,
        };
        DISPLAYLEVEL!(3, "d=%u\n", d);
        let childDisplayLevel = if displayLevel == 0 {
            0 as i32
        } else {
            displayLevel - 1 as i32
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
        if ERR_isError(initVal) {
            DISPLAYLEVEL!(1, "Failed to initialize context\n");
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
                ::core::mem::size_of::<FASTCOVER_tryParameters_data_t>()
            ) as *mut FASTCOVER_tryParameters_data_t;
            DISPLAYLEVEL!(3, "k=%u\n", k);
            if data.is_null() {
                DISPLAYLEVEL!(1, "Failed to allocate parameters\n");
                COVER_best_destroy(&mut best);
                FASTCOVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return ERROR(ZSTD_error_memory_allocation);
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
                .notificationLevel = ctx.displayLevel as u32;
            if FASTCOVER_checkParameters(
                (*data).parameters,
                dictBufferCapacity,
                (*(*data).ctx).f,
                accel,
            ) == 0
            {
                DISPLAYLEVEL!(1, "FASTCOVER parameters incorrect\n");
                libc::free(data as *mut c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        Some(
                            FASTCOVER_tryParameters
                                as unsafe extern "C" fn(*mut c_void) -> (),
                        ),
                        data as *mut c_void,
                    );
                } else {
                    FASTCOVER_tryParameters(data as *mut c_void);
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
                            b"\r%u%%       \0" as *const u8 as *const c_char,
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
    DISPLAYLEVEL!(2, "\r%79s\r", "");
    let dictSize = best.dictSize;
    if ERR_isError(best.compressedSize) {
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
