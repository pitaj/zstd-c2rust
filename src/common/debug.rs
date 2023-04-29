use ::libc;
pub const DEBUGLEVEL: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut g_debuglevel: libc::c_int = DEBUGLEVEL;
