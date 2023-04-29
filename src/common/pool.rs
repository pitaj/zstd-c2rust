use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn ZSTD_pthread_mutex_init(
        mutex: *mut *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn ZSTD_pthread_cond_destroy(cond: *mut *mut pthread_cond_t) -> libc::c_int;
    fn ZSTD_pthread_mutex_destroy(mutex: *mut *mut pthread_mutex_t) -> libc::c_int;
    fn ZSTD_pthread_cond_init(
        cond: *mut *mut pthread_cond_t,
        attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type pthread_t = libc::c_ulong;
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
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POOL_ctx_s {
    pub customMem: ZSTD_customMem,
    pub threads: *mut pthread_t,
    pub threadCapacity: size_t,
    pub threadLimit: size_t,
    pub queue: *mut POOL_job,
    pub queueHead: size_t,
    pub queueTail: size_t,
    pub queueSize: size_t,
    pub numThreadsBusy: size_t,
    pub queueEmpty: libc::c_int,
    pub queueMutex: *mut pthread_mutex_t,
    pub queuePushCond: *mut pthread_cond_t,
    pub queuePopCond: *mut pthread_cond_t,
    pub shutdown: libc::c_int,
}
pub type POOL_job = POOL_job_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POOL_job_s {
    pub function: POOL_function,
    pub opaque: *mut libc::c_void,
}
pub type POOL_function = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
            opaque: NULL as *mut libc::c_void,
        };
        init
    }
};
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const NULL_0: libc::c_int = 0 as libc::c_int;
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
unsafe extern "C" fn POOL_thread(mut opaque: *mut libc::c_void) -> *mut libc::c_void {
    let ctx = opaque as *mut POOL_ctx;
    if ctx.is_null() {
        return NULL_0 as *mut libc::c_void;
    }
    loop {
        pthread_mutex_lock((*ctx).queueMutex);
        while (*ctx).queueEmpty != 0 || (*ctx).numThreadsBusy >= (*ctx).threadLimit {
            if (*ctx).shutdown != 0 {
                pthread_mutex_unlock((*ctx).queueMutex);
                return opaque;
            }
            pthread_cond_wait((*ctx).queuePopCond, (*ctx).queueMutex);
        }
        let job = *((*ctx).queue).offset((*ctx).queueHead as isize);
        (*ctx)
            .queueHead = ((*ctx).queueHead)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_rem((*ctx).queueSize);
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_add(1);
        (*ctx).queueEmpty = ((*ctx).queueHead == (*ctx).queueTail) as libc::c_int;
        pthread_cond_signal((*ctx).queuePushCond);
        pthread_mutex_unlock((*ctx).queueMutex);
        (job.function).expect("non-null function pointer")(job.opaque);
        pthread_mutex_lock((*ctx).queueMutex);
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_sub(1);
        pthread_cond_signal((*ctx).queuePushCond);
        pthread_mutex_unlock((*ctx).queueMutex);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createThreadPool(
    mut numThreads: size_t,
) -> *mut ZSTD_threadPool {
    return POOL_create(numThreads, 0 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_create(
    mut numThreads: size_t,
    mut queueSize: size_t,
) -> *mut POOL_ctx {
    return POOL_create_advanced(numThreads, queueSize, ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_create_advanced(
    mut numThreads: size_t,
    mut queueSize: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut POOL_ctx {
    let mut ctx = 0 as *mut POOL_ctx;
    if numThreads == 0 {
        return NULL_0 as *mut POOL_ctx;
    }
    ctx = ZSTD_customCalloc(
        ::core::mem::size_of::<POOL_ctx>() as libc::c_ulong,
        customMem,
    ) as *mut POOL_ctx;
    if ctx.is_null() {
        return NULL_0 as *mut POOL_ctx;
    }
    (*ctx).queueSize = queueSize.wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*ctx)
        .queue = ZSTD_customCalloc(
        ((*ctx).queueSize)
            .wrapping_mul(::core::mem::size_of::<POOL_job>() as libc::c_ulong),
        customMem,
    ) as *mut POOL_job;
    (*ctx).queueHead = 0 as libc::c_int as size_t;
    (*ctx).queueTail = 0 as libc::c_int as size_t;
    (*ctx).numThreadsBusy = 0 as libc::c_int as size_t;
    (*ctx).queueEmpty = 1 as libc::c_int;
    let mut error = 0 as libc::c_int;
    error
        |= ZSTD_pthread_mutex_init(
            &mut (*ctx).queueMutex,
            NULL_0 as *const pthread_mutexattr_t,
        );
    error
        |= ZSTD_pthread_cond_init(
            &mut (*ctx).queuePushCond,
            NULL_0 as *const pthread_condattr_t,
        );
    error
        |= ZSTD_pthread_cond_init(
            &mut (*ctx).queuePopCond,
            NULL_0 as *const pthread_condattr_t,
        );
    if error != 0 {
        POOL_free(ctx);
        return NULL_0 as *mut POOL_ctx;
    }
    (*ctx).shutdown = 0 as libc::c_int;
    (*ctx)
        .threads = ZSTD_customCalloc(
        numThreads.wrapping_mul(::core::mem::size_of::<pthread_t>() as libc::c_ulong),
        customMem,
    ) as *mut pthread_t;
    (*ctx).threadCapacity = 0 as libc::c_int as size_t;
    (*ctx).customMem = customMem;
    if ((*ctx).threads).is_null() || ((*ctx).queue).is_null() {
        POOL_free(ctx);
        return NULL_0 as *mut POOL_ctx;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numThreads {
        if pthread_create(
            &mut *((*ctx).threads).offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                POOL_thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            ctx as *mut libc::c_void,
        ) != 0
        {
            (*ctx).threadCapacity = i;
            POOL_free(ctx);
            return NULL_0 as *mut POOL_ctx;
        }
        i = i.wrapping_add(1);
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    return ctx;
}
unsafe extern "C" fn POOL_join(mut ctx: *mut POOL_ctx) {
    pthread_mutex_lock((*ctx).queueMutex);
    (*ctx).shutdown = 1 as libc::c_int;
    pthread_mutex_unlock((*ctx).queueMutex);
    pthread_cond_broadcast((*ctx).queuePushCond);
    pthread_cond_broadcast((*ctx).queuePopCond);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*ctx).threadCapacity {
        pthread_join(
            *((*ctx).threads).offset(i as isize),
            NULL_0 as *mut *mut libc::c_void,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn POOL_free(mut ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(ctx);
    ZSTD_pthread_mutex_destroy(&mut (*ctx).queueMutex);
    ZSTD_pthread_cond_destroy(&mut (*ctx).queuePushCond);
    ZSTD_pthread_cond_destroy(&mut (*ctx).queuePopCond);
    ZSTD_customFree((*ctx).queue as *mut libc::c_void, (*ctx).customMem);
    ZSTD_customFree((*ctx).threads as *mut libc::c_void, (*ctx).customMem);
    ZSTD_customFree(ctx as *mut libc::c_void, (*ctx).customMem);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_joinJobs(mut ctx: *mut POOL_ctx) {
    pthread_mutex_lock((*ctx).queueMutex);
    while (*ctx).queueEmpty == 0
        || (*ctx).numThreadsBusy > 0 as libc::c_int as libc::c_ulong
    {
        pthread_cond_wait((*ctx).queuePushCond, (*ctx).queueMutex);
    }
    pthread_mutex_unlock((*ctx).queueMutex);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeThreadPool(mut pool: *mut ZSTD_threadPool) {
    POOL_free(pool);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_sizeof(mut ctx: *const POOL_ctx) -> size_t {
    if ctx.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return (::core::mem::size_of::<POOL_ctx>() as libc::c_ulong)
        .wrapping_add(
            ((*ctx).queueSize)
                .wrapping_mul(::core::mem::size_of::<POOL_job>() as libc::c_ulong),
        )
        .wrapping_add(
            ((*ctx).threadCapacity)
                .wrapping_mul(::core::mem::size_of::<pthread_t>() as libc::c_ulong),
        );
}
unsafe extern "C" fn POOL_resize_internal(
    mut ctx: *mut POOL_ctx,
    mut numThreads: size_t,
) -> libc::c_int {
    if numThreads <= (*ctx).threadCapacity {
        if numThreads == 0 {
            return 1 as libc::c_int;
        }
        (*ctx).threadLimit = numThreads;
        return 0 as libc::c_int;
    }
    let threadPool = ZSTD_customCalloc(
        numThreads.wrapping_mul(::core::mem::size_of::<pthread_t>() as libc::c_ulong),
        (*ctx).customMem,
    ) as *mut pthread_t;
    if threadPool.is_null() {
        return 1 as libc::c_int;
    }
    libc::memcpy(
        threadPool as *mut libc::c_void,
        (*ctx).threads as *const libc::c_void,
        ((*ctx).threadCapacity)
            .wrapping_mul(::core::mem::size_of::<pthread_t>() as libc::c_ulong)
            as libc::size_t,
    );
    ZSTD_customFree((*ctx).threads as *mut libc::c_void, (*ctx).customMem);
    (*ctx).threads = threadPool;
    let mut threadId: size_t = 0;
    threadId = (*ctx).threadCapacity;
    while threadId < numThreads {
        if pthread_create(
            &mut *threadPool.offset(threadId as isize),
            0 as *const pthread_attr_t,
            Some(
                POOL_thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            ctx as *mut libc::c_void,
        ) != 0
        {
            (*ctx).threadCapacity = threadId;
            return 1 as libc::c_int;
        }
        threadId = threadId.wrapping_add(1);
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn POOL_resize(
    mut ctx: *mut POOL_ctx,
    mut numThreads: size_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if ctx.is_null() {
        return 1 as libc::c_int;
    }
    pthread_mutex_lock((*ctx).queueMutex);
    result = POOL_resize_internal(ctx, numThreads);
    pthread_cond_broadcast((*ctx).queuePopCond);
    pthread_mutex_unlock((*ctx).queueMutex);
    return result;
}
unsafe extern "C" fn isQueueFull(mut ctx: *const POOL_ctx) -> libc::c_int {
    if (*ctx).queueSize > 1 as libc::c_int as libc::c_ulong {
        return ((*ctx).queueHead
            == ((*ctx).queueTail)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_rem((*ctx).queueSize)) as libc::c_int
    } else {
        return ((*ctx).numThreadsBusy == (*ctx).threadLimit || (*ctx).queueEmpty == 0)
            as libc::c_int
    };
}
unsafe extern "C" fn POOL_add_internal(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut libc::c_void,
) {
    let mut job = POOL_job {
        function: None,
        opaque: 0 as *mut libc::c_void,
    };
    job.function = function;
    job.opaque = opaque;
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/common/pool.c\0" as *const u8
                as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void POOL_add_internal(POOL_ctx *, POOL_function, void *)\0"))
                .as_ptr(),
        );
    }
    if (*ctx).shutdown != 0 {
        return;
    }
    (*ctx).queueEmpty = 0 as libc::c_int;
    *((*ctx).queue).offset((*ctx).queueTail as isize) = job;
    (*ctx)
        .queueTail = ((*ctx).queueTail)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_rem((*ctx).queueSize);
    pthread_cond_signal((*ctx).queuePopCond);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_add(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut libc::c_void,
) {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/common/pool.c\0" as *const u8
                as *const libc::c_char,
            288 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void POOL_add(POOL_ctx *, POOL_function, void *)\0"))
                .as_ptr(),
        );
    }
    pthread_mutex_lock((*ctx).queueMutex);
    while isQueueFull(ctx) != 0 && (*ctx).shutdown == 0 {
        pthread_cond_wait((*ctx).queuePushCond, (*ctx).queueMutex);
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock((*ctx).queueMutex);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_tryAdd(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/common/pool.c\0" as *const u8
                as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int POOL_tryAdd(POOL_ctx *, POOL_function, void *)\0"))
                .as_ptr(),
        );
    }
    pthread_mutex_lock((*ctx).queueMutex);
    if isQueueFull(ctx) != 0 {
        pthread_mutex_unlock((*ctx).queueMutex);
        return 0 as libc::c_int;
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock((*ctx).queueMutex);
    return 1 as libc::c_int;
}
