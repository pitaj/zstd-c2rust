use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn qsort_r(
        __base: *mut std::ffi::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_d_fn_t,
        __arg: *mut std::ffi::c_void,
    );
    fn memcmp(
        _: *const std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn clock() -> clock_t;
    fn ZSTD_compressBound(srcSize: usize) -> usize;
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> usize;
    fn ZSTD_createCDict(
        dictBuffer: *const std::ffi::c_void,
        dictSize: usize,
        compressionLevel: std::ffi::c_int,
    ) -> *mut ZSTD_CDict;
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> usize;
    fn ZSTD_compress_usingCDict(
        cctx: *mut ZSTD_CCtx,
        dst: *mut std::ffi::c_void,
        dstCapacity: usize,
        src: *const std::ffi::c_void,
        srcSize: usize,
        cdict: *const ZSTD_CDict,
    ) -> usize;
    fn POOL_create(numThreads: usize, queueSize: usize) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_add(
        ctx: *mut POOL_ctx,
        function: POOL_function,
        opaque: *mut std::ffi::c_void,
    );
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> std::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> std::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> std::ffi::c_int;
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
pub union pthread_mutexattr_t {
    pub __size: [std::ffi::c_char; 4],
    pub __align: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [std::ffi::c_char; 4],
    pub __align: std::ffi::c_int,
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
pub type __compar_d_fn_t = Option::<
    unsafe extern "C" fn(
        *const std::ffi::c_void,
        *const std::ffi::c_void,
        *mut std::ffi::c_void,
    ) -> std::ffi::c_int,
>;
pub type unalign64 = u64;
use crate::common::error::*;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_CDict = ZSTD_CDict_s;
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
pub type COVER_map_t = COVER_map_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_map_s {
    pub data: *mut COVER_map_pair_t,
    pub sizeLog: u32,
    pub size: u32,
    pub sizeMask: u32,
}
pub type COVER_map_pair_t = COVER_map_pair_t_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_map_pair_t_s {
    pub key: u32,
    pub value: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_ctx_t {
    pub samples: *const u8,
    pub offsets: *mut usize,
    pub samplesSizes: *const usize,
    pub nbSamples: usize,
    pub nbTrainSamples: usize,
    pub nbTestSamples: usize,
    pub suffix: *mut u32,
    pub suffixSize: usize,
    pub freqs: *mut u32,
    pub dmerAt: *mut u32,
    pub d: std::ffi::c_uint,
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
pub type COVER_tryParameters_data_t = COVER_tryParameters_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_tryParameters_data_s {
    pub ctx: *const COVER_ctx_t,
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
pub const CLOCKS_PER_SEC: std::ffi::c_int = 1000000;
use crate::common::mem::*;
use crate::common::bits::*;
pub const ZDICT_DICTSIZE_MIN: std::ffi::c_int = 256;
pub const NULL: std::ffi::c_int = 0;
pub const COVER_DEFAULT_SPLITPOINT: std::ffi::c_double = 1.0f64;
pub const MAP_EMPTY_VALUE: std::ffi::c_int = -(1 as std::ffi::c_int);
unsafe extern "C" fn COVER_map_clear(mut map: *mut COVER_map_t) {
    libc::memset(
        (*map).data as *mut std::ffi::c_void,
        MAP_EMPTY_VALUE,
        ((*map).size as std::ffi::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<COVER_map_pair_t>(),
            ),
    );
}
unsafe extern "C" fn COVER_map_init(
    mut map: *mut COVER_map_t,
    mut size: u32,
) -> std::ffi::c_int {
    (*map)
        .sizeLog = (ZSTD_highbit32(size))
        .wrapping_add(2);
    (*map).size = 1_u32 << (*map).sizeLog;
    (*map).sizeMask = ((*map).size).wrapping_sub(1);
    (*map)
        .data = libc::malloc(
        ((*map).size as std::ffi::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<COVER_map_pair_t>(),
            ),
    ) as *mut COVER_map_pair_t;
    if ((*map).data).is_null() {
        (*map).sizeLog = 0;
        (*map).size = 0;
        return 0;
    }
    COVER_map_clear(map);
    return 1;
}
static mut COVER_prime4bytes: u32 = 2654435761;
unsafe extern "C" fn COVER_map_hash(mut map: *mut COVER_map_t, mut key: u32) -> u32 {
    return key * COVER_prime4bytes
        >> 32_u32.wrapping_sub((*map).sizeLog);
}
unsafe extern "C" fn COVER_map_index(mut map: *mut COVER_map_t, mut key: u32) -> u32 {
    let hash = COVER_map_hash(map, key);
    let mut i: u32 = 0;
    i = hash;
    loop {
        let mut pos: *mut COVER_map_pair_t = &mut *((*map).data).offset(i as isize)
            as *mut COVER_map_pair_t;
        if (*pos).value == MAP_EMPTY_VALUE as u32 {
            return i;
        }
        if (*pos).key == key {
            return i;
        }
        i = i.wrapping_add(1) & (*map).sizeMask;
    };
}
unsafe extern "C" fn COVER_map_at(mut map: *mut COVER_map_t, mut key: u32) -> *mut u32 {
    let mut pos: *mut COVER_map_pair_t = &mut *((*map).data)
        .offset(
            (COVER_map_index
                as unsafe extern "C" fn(*mut COVER_map_t, u32) -> u32)(map, key) as isize,
        ) as *mut COVER_map_pair_t;
    if (*pos).value == MAP_EMPTY_VALUE as u32 {
        (*pos).key = key;
        (*pos).value = 0;
    }
    return &mut (*pos).value;
}
unsafe extern "C" fn COVER_map_remove(mut map: *mut COVER_map_t, mut key: u32) {
    let mut i = COVER_map_index(map, key);
    let mut del: *mut COVER_map_pair_t = &mut *((*map).data).offset(i as isize)
        as *mut COVER_map_pair_t;
    let mut shift: u32 = 1;
    if (*del).value == MAP_EMPTY_VALUE as u32 {
        return;
    }
    i = i.wrapping_add(1) & (*map).sizeMask;
    loop {
        let pos: *mut COVER_map_pair_t = &mut *((*map).data).offset(i as isize)
            as *mut COVER_map_pair_t;
        if (*pos).value == MAP_EMPTY_VALUE as u32 {
            (*del).value = MAP_EMPTY_VALUE as u32;
            return;
        }
        if i.wrapping_sub(COVER_map_hash(map, (*pos).key)) & (*map).sizeMask >= shift {
            (*del).key = (*pos).key;
            (*del).value = (*pos).value;
            del = pos;
            shift = 1;
        } else {
            shift = shift.wrapping_add(1);
            shift;
        }
        i = i.wrapping_add(1) & (*map).sizeMask;
    };
}
unsafe extern "C" fn COVER_map_destroy(mut map: *mut COVER_map_t) {
    if !((*map).data).is_null() {
        libc::free((*map).data as *mut std::ffi::c_void);
    }
    (*map).data = std::ptr::null_mut();
    (*map).size = 0;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_sum(
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
) -> usize {
    let mut sum: usize = 0;
    let mut i: std::ffi::c_uint = 0;
    i = 0;
    while i < nbSamples {
        sum = sum.wrapping_add(*samplesSizes.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
unsafe extern "C" fn COVER_cmp(
    mut ctx: *mut COVER_ctx_t,
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
) -> std::ffi::c_int {
    let lhs = *(lp as *const u32);
    let rhs = *(rp as *const u32);
    return memcmp(
        ((*ctx).samples).offset(lhs as isize) as *const std::ffi::c_void,
        ((*ctx).samples).offset(rhs as isize) as *const std::ffi::c_void,
        (*ctx).d as std::ffi::c_ulong,
    );
}
unsafe extern "C" fn COVER_cmp8(
    mut ctx: *mut COVER_ctx_t,
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
) -> std::ffi::c_int {
    let mask = if (*ctx).d == 8 {
        u64::MAX
    } else {
        (1_u64
            << (8 as std::ffi::c_uint).wrapping_mul((*ctx).d))
            .wrapping_sub(1)
    };
    let lhs = MEM_readLE64(
        ((*ctx).samples).offset(*(lp as *const u32) as isize) as *const std::ffi::c_void,
    ) & mask;
    let rhs = MEM_readLE64(
        ((*ctx).samples).offset(*(rp as *const u32) as isize) as *const std::ffi::c_void,
    ) & mask;
    if lhs < rhs {
        return -(1 as std::ffi::c_int);
    }
    return (lhs > rhs) as std::ffi::c_int;
}
unsafe extern "C" fn COVER_strict_cmp(
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
    mut g_coverCtx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut result = COVER_cmp(g_coverCtx as *mut COVER_ctx_t, lp, rp);
    if result == 0 {
        result = if lp < rp { -(1 as std::ffi::c_int) } else { 1 as std::ffi::c_int };
    }
    return result;
}
unsafe extern "C" fn COVER_strict_cmp8(
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
    mut g_coverCtx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut result = COVER_cmp8(g_coverCtx as *mut COVER_ctx_t, lp, rp);
    if result == 0 {
        result = if lp < rp { -(1 as std::ffi::c_int) } else { 1 as std::ffi::c_int };
    }
    return result;
}
unsafe extern "C" fn stableSort(mut ctx: *mut COVER_ctx_t) {
    qsort_r(
        (*ctx).suffix as *mut std::ffi::c_void,
        (*ctx).suffixSize,
        ::core::mem::size_of::<u32>(),
        if (*ctx).d <= 8 {
            Some(
                COVER_strict_cmp8
                    as unsafe extern "C" fn(
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                        *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        } else {
            Some(
                COVER_strict_cmp
                    as unsafe extern "C" fn(
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                        *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        },
        ctx as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn COVER_lower_bound(
    mut first: *const usize,
    mut last: *const usize,
    mut value: usize,
) -> *const usize {
    let mut count = last.offset_from(first) as std::ffi::c_long as usize;
    while count != 0 {
        let mut step = count / 2;
        let mut ptr = first;
        ptr = ptr.offset(step as isize);
        if *ptr < value {
            ptr = ptr.offset(1);
            first = ptr;
            count = count
                .wrapping_sub(step.wrapping_add(1));
        } else {
            count = step;
        }
    }
    return first;
}
unsafe extern "C" fn COVER_groupBy(
    mut data: *const std::ffi::c_void,
    mut count: usize,
    mut size: usize,
    mut ctx: *mut COVER_ctx_t,
    mut cmp: Option::<
        unsafe extern "C" fn(
            *mut COVER_ctx_t,
            *const std::ffi::c_void,
            *const std::ffi::c_void,
        ) -> std::ffi::c_int,
    >,
    mut grp: Option::<
        unsafe extern "C" fn(
            *mut COVER_ctx_t,
            *const std::ffi::c_void,
            *const std::ffi::c_void,
        ) -> (),
    >,
) {
    let mut ptr = data as *const u8;
    let mut num: usize = 0;
    while num < count {
        let mut grpEnd = ptr.offset(size as isize);
        num = num.wrapping_add(1);
        num;
        while num < count
            && cmp
                .expect(
                    "non-null function pointer",
                )(ctx, ptr as *const std::ffi::c_void, grpEnd as *const std::ffi::c_void)
                == 0
        {
            grpEnd = grpEnd.offset(size as isize);
            num = num.wrapping_add(1);
            num;
        }
        grp
            .expect(
                "non-null function pointer",
            )(ctx, ptr as *const std::ffi::c_void, grpEnd as *const std::ffi::c_void);
        ptr = grpEnd;
    }
}
unsafe extern "C" fn COVER_group(
    mut ctx: *mut COVER_ctx_t,
    mut group: *const std::ffi::c_void,
    mut groupEnd: *const std::ffi::c_void,
) {
    let mut grpPtr = group as *const u32;
    let mut grpEnd = groupEnd as *const u32;
    let dmerId = grpPtr.offset_from((*ctx).suffix) as std::ffi::c_long as u32;
    let mut freq: u32 = 0;
    let mut curOffsetPtr: *const usize = (*ctx).offsets;
    let mut offsetsEnd: *const usize = ((*ctx).offsets)
        .offset((*ctx).nbSamples as isize);
    let mut curSampleEnd = *((*ctx).offsets).offset(0);
    while grpPtr != grpEnd {
        *((*ctx).dmerAt).offset(*grpPtr as isize) = dmerId;
        if !((*grpPtr as usize) < curSampleEnd) {
            freq = freq.wrapping_add(1);
            if grpPtr.offset(1) != grpEnd {
                let mut sampleEndPtr = COVER_lower_bound(
                    curOffsetPtr,
                    offsetsEnd,
                    *grpPtr as usize,
                );
                curSampleEnd = *sampleEndPtr;
                curOffsetPtr = sampleEndPtr.offset(1);
            }
        }
        grpPtr = grpPtr.offset(1);
        grpPtr;
    }
    *((*ctx).suffix).offset(dmerId as isize) = freq;
}
unsafe extern "C" fn COVER_selectSegment(
    mut ctx: *const COVER_ctx_t,
    mut freqs: *mut u32,
    mut activeDmers: *mut COVER_map_t,
    mut begin: u32,
    mut end: u32,
    mut parameters: ZDICT_cover_params_t,
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
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
    COVER_map_clear(activeDmers);
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0;
    while activeSegment.end < end {
        let mut newDmer = *((*ctx).dmerAt).offset(activeSegment.end as isize);
        let mut newDmerOcc = COVER_map_at(activeDmers, newDmer);
        if *newDmerOcc == 0 {
            activeSegment
                .score = (activeSegment.score)
                .wrapping_add(*freqs.offset(newDmer as isize));
        }
        activeSegment
            .end = (activeSegment.end).wrapping_add(1);
        *newDmerOcc = (*newDmerOcc).wrapping_add(1);
        if (activeSegment.end).wrapping_sub(activeSegment.begin)
            == dmersInK.wrapping_add(1)
        {
            let mut delDmer = *((*ctx).dmerAt).offset(activeSegment.begin as isize);
            let mut delDmerOcc = COVER_map_at(activeDmers, delDmer);
            activeSegment
                .begin = (activeSegment.begin).wrapping_add(1);
            *delDmerOcc = (*delDmerOcc).wrapping_sub(1);
            if *delDmerOcc == 0 {
                COVER_map_remove(activeDmers, delDmer);
                activeSegment
                    .score = (activeSegment.score)
                    .wrapping_sub(*freqs.offset(delDmer as isize));
            }
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    let mut newBegin = bestSegment.end;
    let mut newEnd = bestSegment.begin;
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let mut freq = *freqs.offset(*((*ctx).dmerAt).offset(pos as isize) as isize);
        if freq != 0 {
            newBegin = std::cmp::min(newBegin, pos);
            newEnd = pos.wrapping_add(1);
        }
        pos = pos.wrapping_add(1);
        pos;
    }
    bestSegment.begin = newBegin;
    bestSegment.end = newEnd;
    let mut pos_0: u32 = 0;
    pos_0 = bestSegment.begin;
    while pos_0 != bestSegment.end {
        *freqs
            .offset(
                *((*ctx).dmerAt).offset(pos_0 as isize) as isize,
            ) = 0;
        pos_0 = pos_0.wrapping_add(1);
        pos_0;
    }
    return bestSegment;
}
unsafe extern "C" fn COVER_checkParameters(
    mut parameters: ZDICT_cover_params_t,
    mut maxDictSize: usize,
) -> std::ffi::c_int {
    if parameters.d == 0
        || parameters.k == 0
    {
        return 0;
    }
    if parameters.k as usize > maxDictSize {
        return 0;
    }
    if parameters.d > parameters.k {
        return 0;
    }
    if parameters.splitPoint <= 0.0
        || parameters.splitPoint > 1.0
    {
        return 0;
    }
    return 1;
}
unsafe extern "C" fn COVER_ctx_destroy(mut ctx: *mut COVER_ctx_t) {
    if ctx.is_null() {
        return;
    }
    if !((*ctx).suffix).is_null() {
        libc::free((*ctx).suffix as *mut std::ffi::c_void);
        (*ctx).suffix = std::ptr::null_mut();
    }
    if !((*ctx).freqs).is_null() {
        libc::free((*ctx).freqs as *mut std::ffi::c_void);
        (*ctx).freqs = std::ptr::null_mut();
    }
    if !((*ctx).dmerAt).is_null() {
        libc::free((*ctx).dmerAt as *mut std::ffi::c_void);
        (*ctx).dmerAt = std::ptr::null_mut();
    }
    if !((*ctx).offsets).is_null() {
        libc::free((*ctx).offsets as *mut std::ffi::c_void);
        (*ctx).offsets = std::ptr::null_mut();
    }
}
unsafe extern "C" fn COVER_ctx_init(
    mut ctx: *mut COVER_ctx_t,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
    mut d: std::ffi::c_uint,
    mut splitPoint: std::ffi::c_double,
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
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if nbTrainSamples < 5 {
        DISPLAYLEVEL!(1, "Total number of training samples is %u and is invalid.", nbTrainSamples);
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if nbTestSamples < 1 {
        DISPLAYLEVEL!(1, "Total number of testing samples is %u and is invalid.", nbTestSamples);
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    libc::memset(
        ctx as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<COVER_ctx_t>(),
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
        .suffixSize = trainingSamplesSize
        .wrapping_sub(
            (if d as std::ffi::c_ulong
                > ::core::mem::size_of::<u64>()
            {
                d as std::ffi::c_ulong
            } else {
                ::core::mem::size_of::<u64>()
            }),
        )
        .wrapping_add(1);
    (*ctx)
        .suffix = libc::malloc(
        ((*ctx).suffixSize)
            .wrapping_mul(::core::mem::size_of::<u32>()),
    ) as *mut u32;
    (*ctx)
        .dmerAt = libc::malloc(
        ((*ctx).suffixSize)
            .wrapping_mul(::core::mem::size_of::<u32>()),
    ) as *mut u32;
    (*ctx)
        .offsets = libc::malloc(
        (nbSamples.wrapping_add(1)
            as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<usize>()),
    ) as *mut usize;
    if ((*ctx).suffix).is_null() || ((*ctx).dmerAt).is_null()
        || ((*ctx).offsets).is_null()
    {
        DISPLAYLEVEL!(1, "Failed to allocate scratch buffers\n");
        COVER_ctx_destroy(ctx);
        return ERROR(ZSTD_error_memory_allocation);
    }
    (*ctx).freqs = std::ptr::null_mut();
    (*ctx).d = d;
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
    DISPLAYLEVEL!(2, "Constructing partial suffix array\n");
    let mut i_0: u32 = 0;
    i_0 = 0;
    while (i_0 as usize) < (*ctx).suffixSize {
        *((*ctx).suffix).offset(i_0 as isize) = i_0;
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    stableSort(ctx);
    DISPLAYLEVEL!(2, "Computing frequencies\n");
    COVER_groupBy(
        (*ctx).suffix as *const std::ffi::c_void,
        (*ctx).suffixSize,
        ::core::mem::size_of::<u32>(),
        ctx,
        if (*ctx).d <= 8 {
            Some(
                COVER_cmp8
                    as unsafe extern "C" fn(
                        *mut COVER_ctx_t,
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        } else {
            Some(
                COVER_cmp
                    as unsafe extern "C" fn(
                        *mut COVER_ctx_t,
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        },
        Some(
            COVER_group
                as unsafe extern "C" fn(
                    *mut COVER_ctx_t,
                    *const std::ffi::c_void,
                    *const std::ffi::c_void,
                ) -> (),
        ),
    );
    (*ctx).freqs = (*ctx).suffix;
    (*ctx).suffix = std::ptr::null_mut();
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_warnOnSmallCorpus(
    mut maxDictSize: usize,
    mut nbDmers: usize,
    mut displayLevel: std::ffi::c_int,
) {
    let ratio = nbDmers as std::ffi::c_double / maxDictSize as std::ffi::c_double;
    if ratio >= 10 {
        return;
    }
    DISPLAYLEVEL!(1, "WARNING: The maximum dictionary size %u is too large "
        "compared to the source size %u! "
        "size(source)/size(dictionary) = %f, but it should be >= "
        "10! This may lead to a subpar dictionary! We recommend "
        "training on sources at least 10x, and preferably 100x "
        "the size of the dictionary! \n", (u32) maxDictSize, (u32) nbDmers, ratio);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_computeEpochs(
    mut maxDictSize: u32,
    mut nbDmers: u32,
    mut k: u32,
    mut passes: u32,
) -> COVER_epoch_info_t {
    let minEpochSize = k * 10;
    let mut epochs = COVER_epoch_info_t {
        num: 0,
        size: 0,
    };
    epochs.num = std::cmp::max(1, maxDictSize / k / passes);
    epochs.size = nbDmers / epochs.num;
    if epochs.size >= minEpochSize {
        return epochs;
    }
    epochs.size = std::cmp::min(minEpochSize, nbDmers);
    epochs.num = nbDmers / epochs.size;
    return epochs;
}
unsafe extern "C" fn COVER_buildDictionary(
    mut ctx: *const COVER_ctx_t,
    mut freqs: *mut u32,
    mut activeDmers: *mut COVER_map_t,
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: usize,
    mut parameters: ZDICT_cover_params_t,
) -> usize {
    let dict = dictBuffer as *mut u8;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as u32,
        (*ctx).suffixSize as u32,
        parameters.k,
        4,
    );
    let maxZeroScoreRun = (if 10_u32
        > (if 100_u32 < epochs.num >> 3 {
            100_u32
        } else {
            epochs.num >> 3
        })
    {
        10_u32
    } else if 100_u32 < epochs.num >> 3 {
        100_u32
    } else {
        epochs.num >> 3
    }) as usize;
    let mut zeroScoreRun: usize = 0;
    let mut epoch: usize = 0;
    let mut lastUpdateTime: clock_t = 0;
    let displayLevel = (*ctx).displayLevel;
    DISPLAYLEVEL!(2, "Breaking content into %u epochs of size %u\n", (u32) epochs.num, (u32) epochs
        .size);
    epoch = 0;
    while tail > 0 {
        let epochBegin = (epoch * epochs.size as usize) as u32;
        let epochEnd = epochBegin.wrapping_add(epochs.size);
        let mut segmentSize: usize = 0;
        let mut segment = COVER_selectSegment(
            ctx,
            freqs,
            activeDmers,
            epochBegin,
            epochEnd,
            parameters,
        );
        if segment.score == 0 {
            zeroScoreRun = zeroScoreRun.wrapping_add(1);
            if zeroScoreRun >= maxZeroScoreRun {
                break;
            }
        } else {
            zeroScoreRun = 0;
            segmentSize = std::cmp::min(segment.end - segment.begin + parameters.d - 1, tail);
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
                            * 100_usize / dictBufferCapacity)
                            as std::ffi::c_uint,
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
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_cover(
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: usize,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
    mut parameters: ZDICT_cover_params_t,
) -> usize {
    let dict = dictBuffer as *mut u8;
    let mut ctx = COVER_ctx_t {
        samples: std::ptr::null(),
        offsets: std::ptr::null_mut(),
        samplesSizes: std::ptr::null(),
        nbSamples: 0,
        nbTrainSamples: 0,
        nbTestSamples: 0,
        suffix: std::ptr::null_mut(),
        suffixSize: 0,
        freqs: std::ptr::null_mut(),
        dmerAt: std::ptr::null_mut(),
        d: 0,
        displayLevel: 0,
    };
    let mut activeDmers = COVER_map_s {
        data: std::ptr::null_mut(),
        sizeLog: 0,
        size: 0,
        sizeMask: 0,
    };
    let displayLevel = parameters.zParams.notificationLevel as std::ffi::c_int;
    parameters.splitPoint = 1.0f64;
    if COVER_checkParameters(parameters, dictBufferCapacity) == 0 {
        DISPLAYLEVEL!(1, "Cover parameters incorrect\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if nbSamples == 0 {
        DISPLAYLEVEL!(1, "Cover must have at least one input file\n");
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as usize {
        DISPLAYLEVEL!(1, "dictBufferCapacity must be at least %u\n", ZDICT_DICTSIZE_MIN);
        return ERROR(ZSTD_error_dstSize_tooSmall);
    }
    let initVal = COVER_ctx_init(
        &mut ctx,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        parameters.d,
        parameters.splitPoint,
        displayLevel,
    );
    if ERR_isError(initVal) {
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.suffixSize, displayLevel);
    if COVER_map_init(
        &mut activeDmers,
        (parameters.k)
            .wrapping_sub(parameters.d)
            .wrapping_add(1),
    ) == 0
    {
        DISPLAYLEVEL!(1, "Failed to allocate dmer map: out of memory\n");
        COVER_ctx_destroy(&mut ctx);
        return ERROR(ZSTD_error_memory_allocation);
    }
    DISPLAYLEVEL!(2, "Building dictionary\n");
    let tail = COVER_buildDictionary(
        &mut ctx,
        ctx.freqs,
        &mut activeDmers,
        dictBuffer,
        dictBufferCapacity,
        parameters,
    );
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut std::ffi::c_void,
        dictBufferCapacity,
        dict.offset(tail as isize) as *const std::ffi::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes,
        nbSamples,
        parameters.zParams,
    );
    if ERR_isError(dictionarySize) == 0 {
        DISPLAYLEVEL!(2, "Constructed dictionary of size %u\n", (unsigned) dictionarySize);
    }
    COVER_ctx_destroy(&mut ctx);
    COVER_map_destroy(&mut activeDmers);
    return dictionarySize;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_checkTotalCompressedSize(
    parameters: ZDICT_cover_params_t,
    mut samplesSizes: *const usize,
    mut samples: *const u8,
    mut offsets: *mut usize,
    mut nbTrainSamples: usize,
    mut nbSamples: usize,
    dict: *mut u8,
    mut dictBufferCapacity: usize,
) -> usize {
    let mut totalCompressedSize = ERROR(ZSTD_error_GENERIC);
    let mut cctx = std::ptr::null_mut();
    let mut cdict = std::ptr::null_mut();
    let mut dst = std::ptr::null_mut();
    let mut dstCapacity: usize = 0;
    let mut i: usize = 0;
    let mut maxSampleSize: usize = 0;
    i = if parameters.splitPoint < 1.0f64 {
        nbTrainSamples
    } else {
        0_usize
    };
    while i < nbSamples {
        maxSampleSize = std::cmp::max((*samplesSizes.offset(i as isize)), maxSampleSize);
        i = i.wrapping_add(1);
        i;
    }
    dstCapacity = ZSTD_compressBound(maxSampleSize);
    dst = libc::malloc(dstCapacity);
    cctx = ZSTD_createCCtx();
    cdict = ZSTD_createCDict(
        dict as *const std::ffi::c_void,
        dictBufferCapacity,
        parameters.zParams.compressionLevel,
    );
    if !(dst.is_null() || cctx.is_null() || cdict.is_null()) {
        totalCompressedSize = dictBufferCapacity;
        i = if parameters.splitPoint < 1.0f64 {
            nbTrainSamples
        } else {
            0_usize
        };
        while i < nbSamples {
            let size = ZSTD_compress_usingCDict(
                cctx,
                dst,
                dstCapacity,
                samples.offset(*offsets.offset(i as isize) as isize)
                    as *const std::ffi::c_void,
                *samplesSizes.offset(i as isize),
                cdict,
            );
            if ERR_isError(size) {
                totalCompressedSize = size;
                break;
            } else {
                totalCompressedSize = totalCompressedSize.wrapping_add(size);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    ZSTD_freeCCtx(cctx);
    ZSTD_freeCDict(cdict);
    if !dst.is_null() {
        libc::free(dst);
    }
    return totalCompressedSize;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_init(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    (*best).liveJobs = 0;
    (*best).dict = std::ptr::null_mut();
    (*best).dictSize = 0;
    (*best).compressedSize = -1;
    libc::memset(
        &mut (*best).parameters as *mut ZDICT_cover_params_t as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_wait(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    ZSTD_pthread_mutex_lock!(& best -> mutex)(ZSTD_pthread_mutex_lock!(& best -> mutex));
    while (*best).liveJobs != 0 {
        ZSTD_pthread_cond_wait!(
            & best -> cond, & best -> mutex
        )(
            ZSTD_pthread_cond_wait!(& best -> cond, & best -> mutex),
            ZSTD_pthread_cond_wait!(& best -> cond, & best -> mutex),
        );
    }
    ZSTD_pthread_mutex_unlock!(
        & best -> mutex
    )(ZSTD_pthread_mutex_unlock!(& best -> mutex));
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_destroy(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    COVER_best_wait(best);
    if !((*best).dict).is_null() {
        libc::free((*best).dict);
    }
    ZSTD_pthread_mutex_destroy!(
        & best -> mutex
    )(ZSTD_pthread_mutex_destroy!(& best -> mutex));
    ZSTD_pthread_cond_destroy!(
        & best -> cond
    )(ZSTD_pthread_cond_destroy!(& best -> cond));
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_start(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    ZSTD_pthread_mutex_lock!(& best -> mutex)(ZSTD_pthread_mutex_lock!(& best -> mutex));
    (*best).liveJobs = ((*best).liveJobs).wrapping_add(1);
    (*best).liveJobs;
    ZSTD_pthread_mutex_unlock!(
        & best -> mutex
    )(ZSTD_pthread_mutex_unlock!(& best -> mutex));
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_finish(
    mut best: *mut COVER_best_t,
    mut parameters: ZDICT_cover_params_t,
    mut selection: COVER_dictSelection_t,
) {
    let mut dict = selection.dictContent as *mut std::ffi::c_void;
    let mut compressedSize = selection.totalCompressedSize;
    let mut dictSize = selection.dictSize;
    if best.is_null() {
        return;
    }
    let mut liveJobs: usize = 0;
    ZSTD_pthread_mutex_lock!(& best -> mutex)(ZSTD_pthread_mutex_lock!(& best -> mutex));
    (*best).liveJobs = ((*best).liveJobs).wrapping_sub(1);
    (*best).liveJobs;
    liveJobs = (*best).liveJobs;
    if compressedSize < (*best).compressedSize {
        if ((*best).dict).is_null() || (*best).dictSize < dictSize {
            if !((*best).dict).is_null() {
                libc::free((*best).dict);
            }
            (*best).dict = libc::malloc(dictSize);
            if ((*best).dict).is_null() {
                (*best).compressedSize = ERROR(ZSTD_error_GENERIC);
                (*best).dictSize = 0;
                ZSTD_pthread_cond_signal!(
                    & best -> cond
                )(ZSTD_pthread_cond_signal!(& best -> cond));
                ZSTD_pthread_mutex_unlock!(
                    & best -> mutex
                )(ZSTD_pthread_mutex_unlock!(& best -> mutex));
                return;
            }
        }
        if !dict.is_null() {
            libc::memcpy((*best).dict, dict, dictSize);
            (*best).dictSize = dictSize;
            (*best).parameters = parameters;
            (*best).compressedSize = compressedSize;
        }
    }
    if liveJobs == 0 {
        ZSTD_pthread_cond_broadcast!(
            & best -> cond
        )(ZSTD_pthread_cond_broadcast!(& best -> cond));
    }
    ZSTD_pthread_mutex_unlock!(
        & best -> mutex
    )(ZSTD_pthread_mutex_unlock!(& best -> mutex));
}
unsafe extern "C" fn setDictSelection(
    mut buf: *mut u8,
    mut s: usize,
    mut csz: usize,
) -> COVER_dictSelection_t {
    let mut ds = COVER_dictSelection {
        dictContent: std::ptr::null_mut(),
        dictSize: 0,
        totalCompressedSize: 0,
    };
    ds.dictContent = buf;
    ds.dictSize = s;
    ds.totalCompressedSize = csz;
    return ds;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_dictSelectionError(
    mut error: usize,
) -> COVER_dictSelection_t {
    return setDictSelection(std::ptr::null_mut(), 0, error);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_dictSelectionIsError(
    mut selection: COVER_dictSelection_t,
) -> std::ffi::c_uint {
    return (ERR_isError(selection.totalCompressedSize) != 0
        || (selection.dictContent).is_null()) as std::ffi::c_int as std::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_dictSelectionFree(mut selection: COVER_dictSelection_t) {
    libc::free(selection.dictContent as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_selectDict(
    mut customDictContent: *mut u8,
    mut dictBufferCapacity: usize,
    mut dictContentSize: usize,
    mut samplesBuffer: *const u8,
    mut samplesSizes: *const usize,
    mut nbFinalizeSamples: std::ffi::c_uint,
    mut nbCheckSamples: usize,
    mut nbSamples: usize,
    mut params: ZDICT_cover_params_t,
    mut offsets: *mut usize,
    mut totalCompressedSize: usize,
) -> COVER_dictSelection_t {
    let mut largestDict: usize = 0;
    let mut largestCompressed: usize = 0;
    let mut customDictContentEnd = customDictContent.offset(dictContentSize as isize);
    let mut largestDictbuffer = libc::malloc(dictBufferCapacity) as *mut u8;
    let mut candidateDictBuffer = libc::malloc(dictBufferCapacity) as *mut u8;
    let mut regressionTolerance = params.shrinkDictMaxRegression as std::ffi::c_double
        / 100.0f64 + 1.00f64;
    if largestDictbuffer.is_null() || candidateDictBuffer.is_null() {
        libc::free(largestDictbuffer as *mut std::ffi::c_void);
        libc::free(candidateDictBuffer as *mut std::ffi::c_void);
        return COVER_dictSelectionError(dictContentSize);
    }
    libc::memcpy(
        largestDictbuffer as *mut std::ffi::c_void,
        customDictContent as *const std::ffi::c_void,
        dictContentSize,
    );
    dictContentSize = ZDICT_finalizeDictionary(
        largestDictbuffer as *mut std::ffi::c_void,
        dictBufferCapacity,
        customDictContent as *const std::ffi::c_void,
        dictContentSize,
        samplesBuffer as *const std::ffi::c_void,
        samplesSizes,
        nbFinalizeSamples,
        params.zParams,
    );
    if ERR_isError(dictContentSize) {
        libc::free(largestDictbuffer as *mut std::ffi::c_void);
        libc::free(candidateDictBuffer as *mut std::ffi::c_void);
        return COVER_dictSelectionError(dictContentSize);
    }
    totalCompressedSize = COVER_checkTotalCompressedSize(
        params,
        samplesSizes,
        samplesBuffer,
        offsets,
        nbCheckSamples,
        nbSamples,
        largestDictbuffer,
        dictContentSize,
    );
    if ERR_isError(totalCompressedSize) {
        libc::free(largestDictbuffer as *mut std::ffi::c_void);
        libc::free(candidateDictBuffer as *mut std::ffi::c_void);
        return COVER_dictSelectionError(totalCompressedSize);
    }
    if params.shrinkDict == 0 {
        libc::free(candidateDictBuffer as *mut std::ffi::c_void);
        return setDictSelection(largestDictbuffer, dictContentSize, totalCompressedSize);
    }
    largestDict = dictContentSize;
    largestCompressed = totalCompressedSize;
    dictContentSize = ZDICT_DICTSIZE_MIN as usize;
    while dictContentSize < largestDict {
        libc::memcpy(
            candidateDictBuffer as *mut std::ffi::c_void,
            largestDictbuffer as *const std::ffi::c_void,
            largestDict,
        );
        dictContentSize = ZDICT_finalizeDictionary(
            candidateDictBuffer as *mut std::ffi::c_void,
            dictBufferCapacity,
            customDictContentEnd.offset(-(dictContentSize as isize))
                as *const std::ffi::c_void,
            dictContentSize,
            samplesBuffer as *const std::ffi::c_void,
            samplesSizes,
            nbFinalizeSamples,
            params.zParams,
        );
        if ERR_isError(dictContentSize) {
            libc::free(largestDictbuffer as *mut std::ffi::c_void);
            libc::free(candidateDictBuffer as *mut std::ffi::c_void);
            return COVER_dictSelectionError(dictContentSize);
        }
        totalCompressedSize = COVER_checkTotalCompressedSize(
            params,
            samplesSizes,
            samplesBuffer,
            offsets,
            nbCheckSamples,
            nbSamples,
            candidateDictBuffer,
            dictContentSize,
        );
        if ERR_isError(totalCompressedSize) {
            libc::free(largestDictbuffer as *mut std::ffi::c_void);
            libc::free(candidateDictBuffer as *mut std::ffi::c_void);
            return COVER_dictSelectionError(totalCompressedSize);
        }
        if totalCompressedSize as std::ffi::c_double
            <= largestCompressed as std::ffi::c_double * regressionTolerance
        {
            libc::free(largestDictbuffer as *mut std::ffi::c_void);
            return setDictSelection(
                candidateDictBuffer,
                dictContentSize,
                totalCompressedSize,
            );
        }
        dictContentSize = dictContentSize * 2;
    }
    dictContentSize = largestDict;
    totalCompressedSize = largestCompressed;
    libc::free(candidateDictBuffer as *mut std::ffi::c_void);
    return setDictSelection(largestDictbuffer, dictContentSize, totalCompressedSize);
}
unsafe extern "C" fn COVER_tryParameters(mut opaque: *mut std::ffi::c_void) {
    let data = opaque as *mut COVER_tryParameters_data_t;
    let ctx = (*data).ctx;
    let parameters = (*data).parameters;
    let mut dictBufferCapacity = (*data).dictBufferCapacity;
    let mut totalCompressedSize = ERROR(ZSTD_error_GENERIC);
    let mut activeDmers = COVER_map_s {
        data: std::ptr::null_mut(),
        sizeLog: 0,
        size: 0,
        sizeMask: 0,
    };
    let dict = libc::malloc(dictBufferCapacity) as *mut u8;
    let mut selection = COVER_dictSelectionError(ERROR(ZSTD_error_GENERIC));
    let freqs = libc::malloc(
        ((*ctx).suffixSize)
            .wrapping_mul(::core::mem::size_of::<u32>()),
    ) as *mut u32;
    let displayLevel = (*ctx).displayLevel;
    if COVER_map_init(
        &mut activeDmers,
        (parameters.k)
            .wrapping_sub(parameters.d)
            .wrapping_add(1),
    ) == 0
    {
        DISPLAYLEVEL!(1, "Failed to allocate dmer map: out of memory\n");
    } else if dict.is_null() || freqs.is_null() {
        DISPLAYLEVEL!(1, "Failed to allocate buffers: out of memory\n");
    } else {
        libc::memcpy(
            freqs as *mut std::ffi::c_void,
            (*ctx).freqs as *const std::ffi::c_void,
            ((*ctx).suffixSize)
                .wrapping_mul(::core::mem::size_of::<u32>()),
        );
        let tail = COVER_buildDictionary(
            ctx,
            freqs,
            &mut activeDmers,
            dict as *mut std::ffi::c_void,
            dictBufferCapacity,
            parameters,
        );
        selection = COVER_selectDict(
            dict.offset(tail as isize),
            dictBufferCapacity,
            dictBufferCapacity.wrapping_sub(tail),
            (*ctx).samples,
            (*ctx).samplesSizes,
            (*ctx).nbTrainSamples as std::ffi::c_uint,
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
    libc::free(dict as *mut std::ffi::c_void);
    COVER_best_finish((*data).best, parameters, selection);
    libc::free(data as *mut std::ffi::c_void);
    COVER_map_destroy(&mut activeDmers);
    COVER_dictSelectionFree(selection);
    libc::free(freqs as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_cover(
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: usize,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const usize,
    mut nbSamples: std::ffi::c_uint,
    mut parameters: *mut ZDICT_cover_params_t,
) -> usize {
    let nbThreads = (*parameters).nbThreads;
    let splitPoint = if (*parameters).splitPoint <= 0.0f64 {
        COVER_DEFAULT_SPLITPOINT
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
    let kStepSize = std::cmp::max((kMaxK - kMinK) / kSteps, 1);
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
    let shrinkDict = 0;
    let mut displayLevel = (*parameters).zParams.notificationLevel as std::ffi::c_int;
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
    let mut warned: std::ffi::c_int = 0;
    let mut lastUpdateTime: clock_t = 0;
    if splitPoint <= 0.0
        || splitPoint > 1.0
    {
        DISPLAYLEVEL!(1, "Incorrect parameters\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        DISPLAYLEVEL!(1, "Incorrect parameters\n");
        return ERROR(ZSTD_error_parameter_outOfBound);
    }
    if nbSamples == 0 {
        DISPLAYLEVEL!(1, "Cover must have at least one input file\n");
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
    DISPLAYLEVEL!(2, "Trying %u different sets of parameters\n", kIterations);
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = COVER_ctx_t {
            samples: std::ptr::null(),
            offsets: std::ptr::null_mut(),
            samplesSizes: std::ptr::null(),
            nbSamples: 0,
            nbTrainSamples: 0,
            nbTestSamples: 0,
            suffix: std::ptr::null_mut(),
            suffixSize: 0,
            freqs: std::ptr::null_mut(),
            dmerAt: std::ptr::null_mut(),
            d: 0,
            displayLevel: 0,
        };
        DISPLAYLEVEL!(3, "d=%u\n", d);
        let childDisplayLevel = if displayLevel == 0 {
            0 as std::ffi::c_int
        } else {
            displayLevel - 1 as std::ffi::c_int
        };
        let initVal = COVER_ctx_init(
            &mut ctx,
            samplesBuffer,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            childDisplayLevel,
        );
        if ERR_isError(initVal) {
            DISPLAYLEVEL!(1, "Failed to initialize context\n");
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.suffixSize, displayLevel);
            warned = 1;
        }
        k = kMinK;
        while k <= kMaxK {
            let mut data = libc::malloc(
                ::core::mem::size_of::<COVER_tryParameters_data_t>(),
            ) as *mut COVER_tryParameters_data_t;
            DISPLAYLEVEL!(3, "k=%u\n", k);
            if data.is_null() {
                DISPLAYLEVEL!(1, "Failed to allocate parameters\n");
                COVER_best_destroy(&mut best);
                COVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return ERROR(ZSTD_error_memory_allocation);
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = *parameters;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.shrinkDict = shrinkDict;
            (*data)
                .parameters
                .zParams
                .notificationLevel = ctx.displayLevel as std::ffi::c_uint;
            if COVER_checkParameters((*data).parameters, dictBufferCapacity) == 0 {
                DISPLAYLEVEL!(1, "Cover parameters incorrect\n");
                libc::free(data as *mut std::ffi::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        Some(
                            COVER_tryParameters
                                as unsafe extern "C" fn(*mut std::ffi::c_void) -> (),
                        ),
                        data as *mut std::ffi::c_void,
                    );
                } else {
                    COVER_tryParameters(data as *mut std::ffi::c_void);
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
        COVER_ctx_destroy(&mut ctx);
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
    *parameters = best.parameters;
    libc::memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    return dictSize;
}
