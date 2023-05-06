#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

fn main() {
    zstd_c2rust::programs::zstdcli::main();
}
