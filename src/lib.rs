#![allow(warnings)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)] // now stable
#![feature(c_variadic)] // still unstable
#![feature(core_intrinsics)] // always unstable
#![feature(extern_types)] // still unstable
#![feature(label_break_value)] // now stable
#![feature(stdsimd)] // still unstable
#![feature(pointer_byte_offsets)] // now stable
#![feature(pointer_is_aligned)] // is_aligned_to still unstable
#![feature(ptr_const_cast)] // now stable

/// `__m128i` with unaligned memory access
pub type __m128i_u = [u64; 2];

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod zstd_h;

pub mod common {
pub mod allocations;
pub mod fse_h;
pub mod huf_h;
pub mod bitstream_h;
pub mod zstd_internal_h;
pub mod bits;
pub mod mem;
pub mod debug;
pub mod entropy_common;
pub mod error;
pub mod fse_decompress;
pub mod pool;
pub mod threading;
} // mod common
pub mod compress {
pub mod fse_compress;
pub mod hist;
pub mod huf_compress;
pub mod zstd_compress;
pub mod zstd_cwksp_h;
pub mod clevels;
pub mod zstd_compress_internal;
pub mod zstd_compress_literals;
pub mod zstd_compress_sequences;
pub mod zstd_compress_superblock;
pub mod zstd_double_fast;
pub mod zstd_fast;
pub mod zstd_lazy;
pub mod zstd_ldm;
pub mod zstd_opt;
pub mod zstd_preSplit;
pub mod zstdmt_compress;
} // mod compress
pub mod decompress {
pub mod huf_decompress;
pub mod zstd_ddict;
pub mod zstd_decompress;
pub mod zstd_decompress_block;
pub mod zstd_decompress_internal;
} // mod decompress
pub mod dict_builder {
pub mod cover;
pub mod divsufsort;
pub mod fastcover;
pub mod zdict;
} // mod dictBuilder
