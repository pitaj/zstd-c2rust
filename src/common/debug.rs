use ::libc;
pub const DEBUGLEVEL: std::ffi::c_int = 0;
#[no_mangle]
pub static mut g_debuglevel: std::ffi::c_int = DEBUGLEVEL;
