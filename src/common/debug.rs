use ::libc;
pub const DEBUGLEVEL: i32 = 0;
#[no_mangle]
pub static mut g_debuglevel: i32 = DEBUGLEVEL;
