use ::libc;
extern "C" {
}
pub type XXH_errorcode = std::ffi::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type XXH32_hash_t = u32;
pub type xxh_u32 = XXH32_hash_t;
pub type XXH_alignment = std::ffi::c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type xxh_u8 = u8;
pub type xxh_unalign32 = xxh_u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len_32: XXH32_hash_t,
    pub large_len: XXH32_hash_t,
    pub v: [XXH32_hash_t; 4],
    pub mem32: [XXH32_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved: XXH32_hash_t,
}
pub type XXH32_state_t = XXH32_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [std::ffi::c_uchar; 4],
}
pub type XXH64_hash_t = u64;
pub type xxh_u64 = XXH64_hash_t;
pub type xxh_unalign64 = xxh_u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [std::ffi::c_uchar; 8],
}
pub const XXH_VERSION_MAJOR: std::ffi::c_int = 0;
pub const XXH_VERSION_MINOR: std::ffi::c_int = 8;
pub const XXH_VERSION_RELEASE: std::ffi::c_int = 2;
pub const XXH_VERSION_NUMBER: std::ffi::c_int = XXH_VERSION_MAJOR
    * 100 as std::ffi::c_int * 100 as std::ffi::c_int
    + XXH_VERSION_MINOR * 100 as std::ffi::c_int + XXH_VERSION_RELEASE;
pub const XXH_FORCE_ALIGN_CHECK: std::ffi::c_int = 0;
pub const XXH32_ENDJMP: std::ffi::c_int = 0;
unsafe extern "C" fn XXH_malloc(mut s: usize) -> *mut std::ffi::c_void {
    return libc::malloc(s);
}
unsafe extern "C" fn XXH_free(mut p: *mut std::ffi::c_void) {
    libc::free(p);
}
unsafe extern "C" fn XXH_memcpy(
    mut dest: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut size: usize,
) -> *mut std::ffi::c_void {
    return libc::memcpy(dest, src, size);
}
unsafe extern "C" fn XXH_read32(mut ptr: *const std::ffi::c_void) -> xxh_u32 {
    return *(ptr as *const xxh_unalign32);
}
pub const XXH_CPU_LITTLE_ENDIAN: std::ffi::c_int = 1;
pub const XXH_rotl32: unsafe extern "C" fn(
    std::ffi::c_uint,
    std::ffi::c_uint,
) -> std::ffi::c_uint = __builtin_rotateleft32;
pub const XXH_rotl64: unsafe extern "C" fn(
    std::ffi::c_ulong,
    std::ffi::c_ulong,
) -> std::ffi::c_ulong = __builtin_rotateleft64;
unsafe extern "C" fn XXH_swap32(mut x: xxh_u32) -> xxh_u32 {
    return x << 24 & 0xff000000 as std::ffi::c_uint
        | x << 8 & 0xff0000 as std::ffi::c_int as xxh_u32
        | x >> 8 & 0xff00 as std::ffi::c_int as xxh_u32
        | x >> 24 & 0xff as std::ffi::c_int as xxh_u32;
}
unsafe extern "C" fn XXH_readLE32(mut ptr: *const std::ffi::c_void) -> xxh_u32 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read32(ptr)
    } else {
        XXH_swap32(XXH_read32(ptr))
    };
}
unsafe extern "C" fn XXH_readBE32(mut ptr: *const std::ffi::c_void) -> xxh_u32 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_swap32(XXH_read32(ptr))
    } else {
        XXH_read32(ptr)
    };
}
unsafe extern "C" fn XXH_readLE32_align(
    mut ptr: *const std::ffi::c_void,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if align as std::ffi::c_uint == XXH_unaligned as std::ffi::c_int as std::ffi::c_uint
    {
        return XXH_readLE32(ptr)
    } else {
        return if XXH_CPU_LITTLE_ENDIAN != 0 {
            *(ptr as *const xxh_u32)
        } else {
            XXH_swap32(*(ptr as *const xxh_u32))
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH_versionNumber() -> std::ffi::c_uint {
    return XXH_VERSION_NUMBER as std::ffi::c_uint;
}
pub const XXH_PRIME32_1: std::ffi::c_uint = 0x9e3779b1 as std::ffi::c_uint;
pub const XXH_PRIME32_2: std::ffi::c_uint = 0x85ebca77 as std::ffi::c_uint;
pub const XXH_PRIME32_3: std::ffi::c_uint = 0xc2b2ae3d as std::ffi::c_uint;
pub const XXH_PRIME32_4: std::ffi::c_uint = 0x27d4eb2f as std::ffi::c_uint;
pub const XXH_PRIME32_5: std::ffi::c_uint = 0x165667b1 as std::ffi::c_uint;
unsafe extern "C" fn XXH32_round(mut acc: xxh_u32, mut input: xxh_u32) -> xxh_u32 {
    acc = (acc as std::ffi::c_uint).wrapping_add(input.wrapping_mul(XXH_PRIME32_2))
        as xxh_u32 as xxh_u32;
    acc = ::core::intrinsics::rotate_left(
        acc,
        13,
    );
    acc = (acc as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_1) as xxh_u32 as xxh_u32;
    return acc;
}
unsafe extern "C" fn XXH32_avalanche(mut hash: xxh_u32) -> xxh_u32 {
    hash ^= hash >> 15;
    hash = (hash as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_2) as xxh_u32 as xxh_u32;
    hash ^= hash >> 13;
    hash = (hash as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_3) as xxh_u32 as xxh_u32;
    hash ^= hash >> 16;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32(
    mut input: *const std::ffi::c_void,
    mut len: usize,
    mut seed: XXH32_hash_t,
) -> XXH32_hash_t {
    return XXH32_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_createState() -> *mut XXH32_state_t {
    return XXH_malloc(::core::mem::size_of::<XXH32_state_t>())
        as *mut XXH32_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_freeState(
    mut statePtr: *mut XXH32_state_t,
) -> XXH_errorcode {
    XXH_free(statePtr as *mut std::ffi::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_copyState(
    mut dstState: *mut XXH32_state_t,
    mut srcState: *const XXH32_state_t,
) {
    XXH_memcpy(
        dstState as *mut std::ffi::c_void,
        srcState as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH32_state_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_digest(
    mut state: *const XXH32_state_t,
) -> XXH32_hash_t {
    let mut h32: xxh_u32 = 0;
    if (*state).large_len != 0 {
        h32 = (::core::intrinsics::rotate_left(
            (*state).v[0],
            1,
        ))
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[1],
                    7,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[2],
                    12,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[3],
                    18,
                ),
            );
    } else {
        h32 = ((*state).v[2]).wrapping_add(XXH_PRIME32_5);
    }
    h32 = (h32 as XXH32_hash_t).wrapping_add((*state).total_len_32) as xxh_u32
        as xxh_u32;
    return XXH32_finalize(
        h32,
        ((*state).mem32).as_ptr() as *const xxh_u8,
        (*state).memsize as usize,
        XXH_aligned,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_canonicalFromHash(
    mut dst: *mut XXH32_canonical_t,
    mut hash: XXH32_hash_t,
) {
    hash = XXH_swap32(hash);
    XXH_memcpy(
        dst as *mut std::ffi::c_void,
        &mut hash as *mut XXH32_hash_t as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH32_canonical_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_hashFromCanonical(
    mut src: *const XXH32_canonical_t,
) -> XXH32_hash_t {
    return XXH_readBE32(src as *const std::ffi::c_void);
}
unsafe extern "C" fn XXH_read64(mut ptr: *const std::ffi::c_void) -> xxh_u64 {
    return *(ptr as *const xxh_unalign64);
}
unsafe extern "C" fn XXH_swap64(mut x: xxh_u64) -> xxh_u64 {
    return ((x << 56) as std::ffi::c_ulonglong
        & 0xff00000000000000 as std::ffi::c_ulonglong
        | (x << 40) as std::ffi::c_ulonglong
            & 0xff000000000000 as std::ffi::c_ulonglong
        | (x << 24) as std::ffi::c_ulonglong
            & 0xff0000000000 as std::ffi::c_ulonglong
        | (x << 8) as std::ffi::c_ulonglong
            & 0xff00000000 as std::ffi::c_ulonglong
        | (x >> 8) as std::ffi::c_ulonglong
            & 0xff000000 as std::ffi::c_ulonglong
        | (x >> 24) as std::ffi::c_ulonglong
            & 0xff0000 as std::ffi::c_ulonglong
        | (x >> 40) as std::ffi::c_ulonglong
            & 0xff00 as std::ffi::c_ulonglong
        | (x >> 56) as std::ffi::c_ulonglong
            & 0xff as std::ffi::c_ulonglong) as xxh_u64;
}
unsafe extern "C" fn XXH_readLE64(mut ptr: *const std::ffi::c_void) -> xxh_u64 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read64(ptr)
    } else {
        XXH_swap64(XXH_read64(ptr))
    };
}
unsafe extern "C" fn XXH_readBE64(mut ptr: *const std::ffi::c_void) -> xxh_u64 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_swap64(XXH_read64(ptr))
    } else {
        XXH_read64(ptr)
    };
}
unsafe extern "C" fn XXH_readLE64_align(
    mut ptr: *const std::ffi::c_void,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if align as std::ffi::c_uint == XXH_unaligned as std::ffi::c_int as std::ffi::c_uint
    {
        return XXH_readLE64(ptr)
    } else {
        return if XXH_CPU_LITTLE_ENDIAN != 0 {
            *(ptr as *const xxh_u64)
        } else {
            XXH_swap64(*(ptr as *const xxh_u64))
        }
    };
}
pub const XXH_PRIME64_1: std::ffi::c_ulonglong = 0x9e3779b185ebca87
    as std::ffi::c_ulonglong;
pub const XXH_PRIME64_2: std::ffi::c_ulonglong = 0xc2b2ae3d27d4eb4f
    as std::ffi::c_ulonglong;
pub const XXH_PRIME64_3: std::ffi::c_ulonglong = 0x165667b19e3779f9
    as std::ffi::c_ulonglong;
pub const XXH_PRIME64_4: std::ffi::c_ulonglong = 0x85ebca77c2b2ae63
    as std::ffi::c_ulonglong;
pub const XXH_PRIME64_5: std::ffi::c_ulonglong = 0x27d4eb2f165667c5
    as std::ffi::c_ulonglong;
unsafe extern "C" fn XXH64_round(mut acc: xxh_u64, mut input: xxh_u64) -> xxh_u64 {
    acc = (acc as std::ffi::c_ulonglong)
        .wrapping_add((input as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2))
        as xxh_u64 as xxh_u64;
    acc = ::core::intrinsics::rotate_left(
        acc,
        31,
    );
    acc = (acc as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64
        as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_mergeRound(mut acc: xxh_u64, mut val: xxh_u64) -> xxh_u64 {
    val = XXH64_round(0, val);
    acc ^= val;
    acc = (acc as std::ffi::c_ulonglong)
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_4) as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_avalanche(mut hash: xxh_u64) -> xxh_u64 {
    hash ^= hash >> 33;
    hash = (hash as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2) as xxh_u64
        as xxh_u64;
    hash ^= hash >> 29;
    hash = (hash as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_3) as xxh_u64
        as xxh_u64;
    hash ^= hash >> 32;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64(
    mut input: *const std::ffi::c_void,
    mut len: usize,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    return XXH64_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_createState() -> *mut XXH64_state_t {
    return XXH_malloc(::core::mem::size_of::<XXH64_state_t>())
        as *mut XXH64_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_freeState(
    mut statePtr: *mut XXH64_state_t,
) -> XXH_errorcode {
    XXH_free(statePtr as *mut std::ffi::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_copyState(
    mut dstState: *mut XXH64_state_t,
    mut srcState: *const XXH64_state_t,
) {
    XXH_memcpy(
        dstState as *mut std::ffi::c_void,
        srcState as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH64_state_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_digest(
    mut state: *const XXH64_state_t,
) -> XXH64_hash_t {
    let mut h64: xxh_u64 = 0;
    if (*state).total_len >= 32 {
        h64 = (::core::intrinsics::rotate_left(
            (*state).v[0],
            1,
        ))
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[1],
                    7,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[2],
                    12,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[3],
                    18,
                ),
            );
        h64 = XXH64_mergeRound(h64, (*state).v[0]);
        h64 = XXH64_mergeRound(h64, (*state).v[1]);
        h64 = XXH64_mergeRound(h64, (*state).v[2]);
        h64 = XXH64_mergeRound(h64, (*state).v[3]);
    } else {
        h64 = ((*state).v[2] as std::ffi::c_ulonglong)
            .wrapping_add(XXH_PRIME64_5) as xxh_u64;
    }
    h64 = h64.wrapping_add((*state).total_len);
    return XXH64_finalize(
        h64,
        ((*state).mem64).as_ptr() as *const xxh_u8,
        (*state).total_len,
        XXH_aligned,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_canonicalFromHash(
    mut dst: *mut XXH64_canonical_t,
    mut hash: XXH64_hash_t,
) {
    hash = XXH_swap64(hash);
    XXH_memcpy(
        dst as *mut std::ffi::c_void,
        &mut hash as *mut XXH64_hash_t as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH64_canonical_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_hashFromCanonical(
    mut src: *const XXH64_canonical_t,
) -> XXH64_hash_t {
    return XXH_readBE64(src as *const std::ffi::c_void);
}
pub const NULL: std::ffi::c_int = 0;
