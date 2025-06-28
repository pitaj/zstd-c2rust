use ::libc;
extern "C" {
}
use crate::common::error::*;

pub const ZSTD_VERSION_MAJOR: std::ffi::c_int = 1;
pub const ZSTD_VERSION_MINOR: std::ffi::c_int = 5;
pub const ZSTD_VERSION_RELEASE: std::ffi::c_int = 8;
pub const ZSTD_VERSION_NUMBER: std::ffi::c_int = ZSTD_VERSION_MAJOR
    * 100 as std::ffi::c_int * 100 as std::ffi::c_int
    + ZSTD_VERSION_MINOR * 100 as std::ffi::c_int + ZSTD_VERSION_RELEASE;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_versionNumber() -> std::ffi::c_uint {
    return ZSTD_VERSION_NUMBER as std::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_versionString() -> *const std::ffi::c_char {
    return b"1.5.8\0" as *const u8 as *const std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isDeterministicBuild() -> std::ffi::c_int {
    return 1;
}
