#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(core_intrinsics)]

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

 // `__m128i` with unaligned memory access
pub type __m128i_u = [libc::c_longlong; 2];

mod common;
mod compress;
mod decompress;
mod dict_builder;
mod legacy;
