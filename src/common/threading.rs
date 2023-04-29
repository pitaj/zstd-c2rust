use ::libc;
extern "C" {
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
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
#[no_mangle]
pub static mut g_ZSTD_threading_useless_symbol: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_pthread_mutex_init(
    mut mutex: *mut *mut pthread_mutex_t,
    mut attr: *const pthread_mutexattr_t,
) -> libc::c_int {
    *mutex = malloc(::core::mem::size_of::<pthread_mutex_t>() as libc::c_ulong)
        as *mut pthread_mutex_t;
    if (*mutex).is_null() {
        return 1 as libc::c_int;
    }
    return pthread_mutex_init(*mutex, attr);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_pthread_mutex_destroy(
    mut mutex: *mut *mut pthread_mutex_t,
) -> libc::c_int {
    if (*mutex).is_null() {
        return 0 as libc::c_int;
    }
    let ret = pthread_mutex_destroy(*mutex);
    free(*mutex as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_pthread_cond_init(
    mut cond: *mut *mut pthread_cond_t,
    mut attr: *const pthread_condattr_t,
) -> libc::c_int {
    *cond = malloc(::core::mem::size_of::<pthread_cond_t>() as libc::c_ulong)
        as *mut pthread_cond_t;
    if (*cond).is_null() {
        return 1 as libc::c_int;
    }
    return pthread_cond_init(*cond, attr);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_pthread_cond_destroy(
    mut cond: *mut *mut pthread_cond_t,
) -> libc::c_int {
    if (*cond).is_null() {
        return 0 as libc::c_int;
    }
    let ret = pthread_cond_destroy(*cond);
    free(*cond as *mut libc::c_void);
    return ret;
}
