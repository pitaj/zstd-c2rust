use ::libc;
extern "C" {
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void,
        >,
        __arg: *mut std::ffi::c_void,
    ) -> std::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut std::ffi::c_void,
    ) -> std::ffi::c_int;
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
}
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
pub type pthread_t = std::ffi::c_ulong;
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
pub union pthread_attr_t {
    pub __size: [std::ffi::c_char; 56],
    pub __align: std::ffi::c_long,
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
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void,
>;
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut std::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POOL_ctx_s {
    pub customMem: ZSTD_customMem,
    pub threads: *mut pthread_t,
    pub threadCapacity: usize,
    pub threadLimit: usize,
    pub queue: *mut POOL_job,
    pub queueHead: usize,
    pub queueTail: usize,
    pub queueSize: usize,
    pub numThreadsBusy: usize,
    pub queueEmpty: std::ffi::c_int,
    pub queueMutex: pthread_mutex_t,
    pub queuePushCond: pthread_cond_t,
    pub queuePopCond: pthread_cond_t,
    pub shutdown: std::ffi::c_int,
}
pub type POOL_job = POOL_job_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POOL_job_s {
    pub function: POOL_function,
    pub opaque: *mut std::ffi::c_void,
}
pub type POOL_function = Option::<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
pub type ZSTD_threadPool = POOL_ctx_s;
pub type POOL_ctx = POOL_ctx_s;
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
            opaque: NULL as *mut std::ffi::c_void,
        };
        init
    }
};
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    mut size: usize,
    mut customMem: ZSTD_customMem,
) -> *mut std::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
        libc::memset(
            ZSTD_memset!(ptr, 0, size),
            ZSTD_memset!(ptr, 0, size),
            ZSTD_memset!(ptr, 0, size) as usize,
        );
        return ptr;
    }
    return ZSTD_calloc!(1, size);
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut std::ffi::c_void,
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
pub const NULL: std::ffi::c_int = 0;
pub const NULL_0: std::ffi::c_int = 0;
unsafe extern "C" fn POOL_thread(
    mut opaque: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    let ctx = opaque as *mut POOL_ctx;
    if ctx.is_null() {
        return NULL_0 as *mut std::ffi::c_void;
    }
    loop {
        ZSTD_pthread_mutex_lock!(
            & ctx -> queueMutex
        )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
        while (*ctx).queueEmpty != 0 || (*ctx).numThreadsBusy >= (*ctx).threadLimit {
            if (*ctx).shutdown != 0 {
                ZSTD_pthread_mutex_unlock!(
                    & ctx -> queueMutex
                )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
                return opaque;
            }
            ZSTD_pthread_cond_wait!(
                & ctx -> queuePopCond, & ctx -> queueMutex
            )(
                ZSTD_pthread_cond_wait!(& ctx -> queuePopCond, & ctx -> queueMutex),
                ZSTD_pthread_cond_wait!(& ctx -> queuePopCond, & ctx -> queueMutex),
            );
        }
        let job = *((*ctx).queue).offset((*ctx).queueHead as isize);
        (*ctx)
            .queueHead = ((*ctx).queueHead).wrapping_add(1)
            % (*ctx).queueSize;
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_add(1);
        (*ctx).numThreadsBusy;
        (*ctx).queueEmpty = ((*ctx).queueHead == (*ctx).queueTail) as std::ffi::c_int;
        ZSTD_pthread_cond_signal!(
            & ctx -> queuePushCond
        )(ZSTD_pthread_cond_signal!(& ctx -> queuePushCond));
        ZSTD_pthread_mutex_unlock!(
            & ctx -> queueMutex
        )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
        (job.function).expect("non-null function pointer")(job.opaque);
        ZSTD_pthread_mutex_lock!(
            & ctx -> queueMutex
        )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_sub(1);
        (*ctx).numThreadsBusy;
        ZSTD_pthread_cond_signal!(
            & ctx -> queuePushCond
        )(ZSTD_pthread_cond_signal!(& ctx -> queuePushCond));
        ZSTD_pthread_mutex_unlock!(
            & ctx -> queueMutex
        )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createThreadPool(
    mut numThreads: usize,
) -> *mut ZSTD_threadPool {
    return POOL_create(numThreads, 0 as std::ffi::c_int as usize);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_create(
    mut numThreads: usize,
    mut queueSize: usize,
) -> *mut POOL_ctx {
    return POOL_create_advanced(numThreads, queueSize, ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_create_advanced(
    mut numThreads: usize,
    mut queueSize: usize,
    mut customMem: ZSTD_customMem,
) -> *mut POOL_ctx {
    let mut ctx = 0 as *mut POOL_ctx;
    if numThreads == 0 {
        return NULL_0 as *mut POOL_ctx;
    }
    ctx = ZSTD_customCalloc(
        ::core::mem::size_of::<POOL_ctx>(),
        customMem,
    ) as *mut POOL_ctx;
    if ctx.is_null() {
        return NULL_0 as *mut POOL_ctx;
    }
    (*ctx).queueSize = queueSize.wrapping_add(1);
    (*ctx)
        .queue = ZSTD_customCalloc(
        ((*ctx).queueSize)
            .wrapping_mul(::core::mem::size_of::<POOL_job>()),
        customMem,
    ) as *mut POOL_job;
    (*ctx).queueHead = 0;
    (*ctx).queueTail = 0;
    (*ctx).numThreadsBusy = 0;
    (*ctx).queueEmpty = 1;
    let mut error: std::ffi::c_int = 0;
    error |= ZSTD_pthread_mutex_init!(& ctx -> queueMutex, NULL);
    error |= ZSTD_pthread_cond_init!(& ctx -> queuePushCond, NULL);
    error |= ZSTD_pthread_cond_init!(& ctx -> queuePopCond, NULL);
    if error != 0 {
        POOL_free(ctx);
        return NULL_0 as *mut POOL_ctx;
    }
    (*ctx).shutdown = 0;
    (*ctx)
        .threads = ZSTD_customCalloc(
        numThreads
            .wrapping_mul(::core::mem::size_of::<pthread_t>()),
        customMem,
    ) as *mut pthread_t;
    (*ctx).threadCapacity = 0;
    (*ctx).customMem = customMem;
    if ((*ctx).threads).is_null() || ((*ctx).queue).is_null() {
        POOL_free(ctx);
        return NULL_0 as *mut POOL_ctx;
    }
    let mut i: usize = 0;
    i = 0;
    while i < numThreads {
        if ZSTD_pthread_create!(& ctx -> threads[i], NULL, & POOL_thread, ctx) != 0 {
            (*ctx).threadCapacity = i;
            POOL_free(ctx);
            return NULL_0 as *mut POOL_ctx;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    return ctx;
}
unsafe extern "C" fn POOL_join(mut ctx: *mut POOL_ctx) {
    ZSTD_pthread_mutex_lock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
    (*ctx).shutdown = 1;
    ZSTD_pthread_mutex_unlock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
    ZSTD_pthread_cond_broadcast!(
        & ctx -> queuePushCond
    )(ZSTD_pthread_cond_broadcast!(& ctx -> queuePushCond));
    ZSTD_pthread_cond_broadcast!(
        & ctx -> queuePopCond
    )(ZSTD_pthread_cond_broadcast!(& ctx -> queuePopCond));
    let mut i: usize = 0;
    i = 0;
    while i < (*ctx).threadCapacity {
        ZSTD_pthread_join!(
            ctx -> threads[i]
        )(ZSTD_pthread_join!(ctx -> threads[i]), NULL_0 as *mut *mut std::ffi::c_void);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn POOL_free(mut ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(ctx);
    ZSTD_pthread_mutex_destroy!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_destroy!(& ctx -> queueMutex));
    ZSTD_pthread_cond_destroy!(
        & ctx -> queuePushCond
    )(ZSTD_pthread_cond_destroy!(& ctx -> queuePushCond));
    ZSTD_pthread_cond_destroy!(
        & ctx -> queuePopCond
    )(ZSTD_pthread_cond_destroy!(& ctx -> queuePopCond));
    ZSTD_customFree((*ctx).queue as *mut std::ffi::c_void, (*ctx).customMem);
    ZSTD_customFree((*ctx).threads as *mut std::ffi::c_void, (*ctx).customMem);
    ZSTD_customFree(ctx as *mut std::ffi::c_void, (*ctx).customMem);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_joinJobs(mut ctx: *mut POOL_ctx) {
    ZSTD_pthread_mutex_lock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
    while (*ctx).queueEmpty == 0
        || (*ctx).numThreadsBusy > 0
    {
        ZSTD_pthread_cond_wait!(
            & ctx -> queuePushCond, & ctx -> queueMutex
        )(
            ZSTD_pthread_cond_wait!(& ctx -> queuePushCond, & ctx -> queueMutex),
            ZSTD_pthread_cond_wait!(& ctx -> queuePushCond, & ctx -> queueMutex),
        );
    }
    ZSTD_pthread_mutex_unlock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeThreadPool(mut pool: *mut ZSTD_threadPool) {
    POOL_free(pool);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_sizeof(mut ctx: *const POOL_ctx) -> usize {
    if ctx.is_null() {
        return 0 as std::ffi::c_int as usize;
    }
    return (::core::mem::size_of::<POOL_ctx>())
        .wrapping_add(
            ((*ctx).queueSize)
                .wrapping_mul(::core::mem::size_of::<POOL_job>()),
        )
        .wrapping_add(
            ((*ctx).threadCapacity)
                .wrapping_mul(::core::mem::size_of::<pthread_t>()),
        );
}
unsafe extern "C" fn POOL_resize_internal(
    mut ctx: *mut POOL_ctx,
    mut numThreads: usize,
) -> std::ffi::c_int {
    if numThreads <= (*ctx).threadCapacity {
        if numThreads == 0 {
            return 1 as std::ffi::c_int;
        }
        (*ctx).threadLimit = numThreads;
        return 0 as std::ffi::c_int;
    }
    let threadPool = ZSTD_customCalloc(
        numThreads
            .wrapping_mul(::core::mem::size_of::<pthread_t>()),
        (*ctx).customMem,
    ) as *mut pthread_t;
    if threadPool.is_null() {
        return 1 as std::ffi::c_int;
    }
    libc::memcpy(
        threadPool as *mut std::ffi::c_void,
        (*ctx).threads as *const std::ffi::c_void,
        ((*ctx).threadCapacity)
            .wrapping_mul(::core::mem::size_of::<pthread_t>())
            as usize,
    );
    ZSTD_customFree((*ctx).threads as *mut std::ffi::c_void, (*ctx).customMem);
    (*ctx).threads = threadPool;
    let mut threadId: usize = 0;
    threadId = (*ctx).threadCapacity;
    while threadId < numThreads {
        if ZSTD_pthread_create!(& threadPool[threadId], NULL, & POOL_thread, ctx) != 0 {
            (*ctx).threadCapacity = threadId;
            return 1 as std::ffi::c_int;
        }
        threadId = threadId.wrapping_add(1);
        threadId;
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn POOL_resize(
    mut ctx: *mut POOL_ctx,
    mut numThreads: usize,
) -> std::ffi::c_int {
    let mut result: std::ffi::c_int = 0;
    if ctx.is_null() {
        return 1 as std::ffi::c_int;
    }
    ZSTD_pthread_mutex_lock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
    result = POOL_resize_internal(ctx, numThreads);
    ZSTD_pthread_cond_broadcast!(
        & ctx -> queuePopCond
    )(ZSTD_pthread_cond_broadcast!(& ctx -> queuePopCond));
    ZSTD_pthread_mutex_unlock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
    return result;
}
unsafe extern "C" fn isQueueFull(mut ctx: *const POOL_ctx) -> std::ffi::c_int {
    if (*ctx).queueSize > 1 {
        return ((*ctx).queueHead
            == ((*ctx).queueTail).wrapping_add(1)
                % (*ctx).queueSize) as std::ffi::c_int
    } else {
        return ((*ctx).numThreadsBusy == (*ctx).threadLimit || (*ctx).queueEmpty == 0)
            as std::ffi::c_int
    };
}
unsafe extern "C" fn POOL_add_internal(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut std::ffi::c_void,
) {
    let mut job = POOL_job_s {
        function: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    job.function = function;
    job.opaque = opaque;
    if (*ctx).shutdown != 0 {
        return;
    }
    (*ctx).queueEmpty = 0;
    *((*ctx).queue).offset((*ctx).queueTail as isize) = job;
    (*ctx)
        .queueTail = ((*ctx).queueTail).wrapping_add(1)
        % (*ctx).queueSize;
    ZSTD_pthread_cond_signal!(
        & ctx -> queuePopCond
    )(ZSTD_pthread_cond_signal!(& ctx -> queuePopCond));
}
#[no_mangle]
pub unsafe extern "C" fn POOL_add(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut std::ffi::c_void,
) {
    ZSTD_pthread_mutex_lock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
    while isQueueFull(ctx) != 0 && (*ctx).shutdown == 0 {
        ZSTD_pthread_cond_wait!(
            & ctx -> queuePushCond, & ctx -> queueMutex
        )(
            ZSTD_pthread_cond_wait!(& ctx -> queuePushCond, & ctx -> queueMutex),
            ZSTD_pthread_cond_wait!(& ctx -> queuePushCond, & ctx -> queueMutex),
        );
    }
    POOL_add_internal(ctx, function, opaque);
    ZSTD_pthread_mutex_unlock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
}
#[no_mangle]
pub unsafe extern "C" fn POOL_tryAdd(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    ZSTD_pthread_mutex_lock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_lock!(& ctx -> queueMutex));
    if isQueueFull(ctx) != 0 {
        ZSTD_pthread_mutex_unlock!(
            & ctx -> queueMutex
        )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
        return 0 as std::ffi::c_int;
    }
    POOL_add_internal(ctx, function, opaque);
    ZSTD_pthread_mutex_unlock!(
        & ctx -> queueMutex
    )(ZSTD_pthread_mutex_unlock!(& ctx -> queueMutex));
    return 1 as std::ffi::c_int;
}
