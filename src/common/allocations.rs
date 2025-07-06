use std::ffi::c_void;

use crate::zstd_h::ZSTD_customMem;

/* custom memory allocation functions */

#[inline]
pub unsafe fn ZSTD_customMalloc(size: usize, customMem: ZSTD_customMem) -> *mut c_void {
    if let Some(customAlloc) = customMem.customAlloc {
        customAlloc(customMem.opaque, size)
    } else {
        libc::malloc(size)
    }
}

#[inline]
pub unsafe fn ZSTD_customCalloc(size: usize, customMem: ZSTD_customMem) -> *mut c_void {
    /* calloc implemented as malloc+memset;
     * not as efficient as calloc, but next best guess for custom malloc */
    if let Some(customAlloc) = customMem.customAlloc {
        let ptr = customAlloc(customMem.opaque, size);
        libc::memset(ptr, 0, size)
    } else {
        libc::calloc(1, size)
    }
}

#[inline]
pub unsafe fn ZSTD_customFree(ptr: *mut c_void, customMem: ZSTD_customMem) {
    if ptr.is_null() { return }

    if let Some(customFree) = customMem.customFree {
        customFree(customMem.opaque, ptr);
    } else {
        libc::free(ptr);
    }
}
