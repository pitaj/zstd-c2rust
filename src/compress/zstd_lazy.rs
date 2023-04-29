use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_set_epi8, _mm_set1_epi8,
    _mm_storeu_si128, _mm_movemask_epi8, _mm_setzero_si128,
};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_set_epi8, _mm_set1_epi8,
    _mm_storeu_si128, _mm_movemask_epi8, _mm_setzero_si128,
};
use core::arch::asm;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __loadu_si128 {
    pub __v: __m128i_u,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si128 {
    pub __v: __m128i_u,
}
pub type ptrdiff_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type unalign64 = U64;
pub type unalignArch = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut BYTE,
    pub lit: *mut BYTE,
    pub llCode: *mut BYTE,
    pub mlCode: *mut BYTE,
    pub ofCode: *mut BYTE,
    pub maxNbSeq: size_t,
    pub maxNbLit: size_t,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: U32,
}
pub type ZSTD_longLengthType_e = libc::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type seqDef = seqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqDef_s {
    pub offBase: U32,
    pub litLength: U16,
    pub mlBase: U16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_matchState_t {
    pub window: ZSTD_window_t,
    pub loadedDictEnd: U32,
    pub nextToUpdate: U32,
    pub hashLog3: U32,
    pub rowHashLog: U32,
    pub tagTable: *mut BYTE,
    pub hashCache: [U32; 8],
    pub hashSalt: U64,
    pub hashSaltEntropy: U32,
    pub hashTable: *mut U32,
    pub hashTable3: *mut U32,
    pub chainTable: *mut U32,
    pub forceNonContiguous: U32,
    pub dedicatedDictSearch: libc::c_int,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_matchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const rawSeqStore_t,
    pub prefetchCDictTables: libc::c_int,
    pub lazySkipping: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub posInSequence: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub minMatch: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optState_t {
    pub litFreq: *mut libc::c_uint,
    pub litLengthFreq: *mut libc::c_uint,
    pub matchLengthFreq: *mut libc::c_uint,
    pub offCodeFreq: *mut libc::c_uint,
    pub matchTable: *mut ZSTD_match_t,
    pub priceTable: *mut ZSTD_optimal_t,
    pub litSum: U32,
    pub litLengthSum: U32,
    pub matchLengthSum: U32,
    pub offCodeSum: U32,
    pub litSumBasePrice: U32,
    pub litLengthSumBasePrice: U32,
    pub matchLengthSumBasePrice: U32,
    pub offCodeSumBasePrice: U32,
    pub priceType: ZSTD_OptPrice_e,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
}
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
pub type FSE_repeat = libc::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = libc::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = size_t;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
    pub nbOverflowCorrections: U32,
}
pub type ZSTD_overlap_e = libc::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type searchMethod_e = libc::c_uint;
pub const search_rowHash: searchMethod_e = 2;
pub const search_binaryTree: searchMethod_e = 1;
pub const search_hashChain: searchMethod_e = 0;
pub type ZSTD_VecMask = U64;
pub const kSearchStrength: libc::c_int = 8 as libc::c_int;
pub const ZSTD_DUBT_UNSORTED_MARK: libc::c_int = 1 as libc::c_int;
pub const ZSTD_ROW_HASH_CACHE_SIZE: libc::c_int = 8 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return *(ptr as *const unalign16);
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_readST(mut ptr: *const libc::c_void) -> size_t {
    return *(ptr as *const unalignArch);
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr)
    } else {
        return MEM_swap32(MEM_read32(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        return MEM_swap64(MEM_read64(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0.swap_bytes();
}
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"unsigned int ZSTD_countTrailingZeros32(U32)\0"))
                .as_ptr(),
        );
    }
    return val.trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"unsigned int ZSTD_countLeadingZeros32(U32)\0"))
                .as_ptr(),
        );
    }
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: U64) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"unsigned int ZSTD_countTrailingZeros64(U64)\0"))
                .as_ptr(),
        );
    }
    return (val as libc::c_ulonglong).trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros64(mut val: U64) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"unsigned int ZSTD_countLeadingZeros64(U64)\0"))
                .as_ptr(),
        );
    }
    return (val as libc::c_ulonglong).leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: size_t) -> libc::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            return ZSTD_countTrailingZeros64(val) >> 3 as libc::c_int
        } else {
            return ZSTD_countTrailingZeros32(val as U32) >> 3 as libc::c_int
        }
    } else if MEM_64bits() != 0 {
        return ZSTD_countLeadingZeros64(val) >> 3 as libc::c_int
    } else {
        return ZSTD_countLeadingZeros32(val as U32) >> 3 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"unsigned int ZSTD_highbit32(U32)\0"))
                .as_ptr(),
        );
    }
    return (31 as libc::c_int as libc::c_uint)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U64(value: U64, mut count: U32) -> U64 {
    if count < 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"count < 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"U64 ZSTD_rotateRight_U64(const U64, U32)\0"))
                .as_ptr(),
        );
    }
    count &= 0x3f as libc::c_int as libc::c_uint;
    return value >> count
        | value
            << ((0 as libc::c_uint).wrapping_sub(count)
                & 0x3f as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U32(value: U32, mut count: U32) -> U32 {
    if count < 32 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"count < 32\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            188 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"U32 ZSTD_rotateRight_U32(const U32, U32)\0"))
                .as_ptr(),
        );
    }
    count &= 0x1f as libc::c_int as libc::c_uint;
    return value >> count
        | value
            << ((0 as libc::c_uint).wrapping_sub(count)
                & 0x1f as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U16(value: U16, mut count: U32) -> U16 {
    if count < 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"count < 16\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"U16 ZSTD_rotateRight_U16(const U16, U32)\0"))
                .as_ptr(),
        );
    }
    count &= 0xf as libc::c_int as libc::c_uint;
    return (value as libc::c_int >> count
        | ((value as libc::c_int)
            << ((0 as libc::c_uint).wrapping_sub(count)
                & 0xf as libc::c_int as libc::c_uint)) as U16 as libc::c_int) as U16;
}
unsafe extern "C" fn ZSTD_copy8(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    libc::memcpy(dst, src, 8 as libc::c_int as libc::c_ulong as libc::size_t);
}
unsafe extern "C" fn ZSTD_copy16(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
#[inline(always)]
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut length: ptrdiff_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut BYTE).offset_from(src as *const BYTE) as libc::c_long;
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let oend = op.offset(length as isize);
    if ovtype as libc::c_uint
        == ZSTD_overlap_src_before_dst as libc::c_int as libc::c_uint
        && diff < WILDCOPY_VECLEN as libc::c_long
    {
        loop {
            ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(8 as libc::c_int as isize);
            ip = ip.offset(8 as libc::c_int as isize);
            if !(op < oend) {
                break;
            }
        }
    } else {
        if diff >= 16 as libc::c_int as libc::c_long
            || diff <= -(16 as libc::c_int) as libc::c_long
        {} else {
            __assert_fail(
                b"diff >= WILDCOPY_VECLEN || diff <= -WILDCOPY_VECLEN\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/zstd_internal.h\0"
                    as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void ZSTD_wildcopy(void *, const void *, ptrdiff_t, const ZSTD_overlap_e)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
        if 16 as libc::c_int as libc::c_long >= length {
            return;
        }
        op = op.offset(16 as libc::c_int as isize);
        ip = ip.offset(16 as libc::c_int as isize);
        loop {
            ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(16 as libc::c_int as isize);
            ip = ip.offset(16 as libc::c_int as isize);
            ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(16 as libc::c_int as isize);
            ip = ip.offset(16 as libc::c_int as isize);
            if !(op < oend) {
                break;
            }
        }
    };
}
pub const WILDCOPY_VECLEN: libc::c_int = 16 as libc::c_int;
pub const MINMATCH: libc::c_int = 3 as libc::c_int;
unsafe extern "C" fn ZSTD_safecopyLiterals(
    mut op: *mut BYTE,
    mut ip: *const BYTE,
    iend: *const BYTE,
    mut ilimit_w: *const BYTE,
) {
    if iend > ilimit_w {} else {
        __assert_fail(
            b"iend > ilimit_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            627 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_safecopyLiterals(BYTE *, const BYTE *, const BYTE *const, const BYTE *)\0",
            ))
                .as_ptr(),
        );
    }
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            ip as *const libc::c_void,
            ilimit_w.offset_from(ip) as libc::c_long,
            ZSTD_no_overlap,
        );
        op = op.offset(ilimit_w.offset_from(ip) as libc::c_long as isize);
        ip = ilimit_w;
    }
    while ip < iend {
        let fresh0 = ip;
        ip = ip.offset(1);
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = *fresh0;
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_storeSeq(
    mut seqStorePtr: *mut seqStore_t,
    mut litLength: size_t,
    mut literals: *const BYTE,
    mut litLimit: *const BYTE,
    mut offBase: U32,
    mut matchLength: size_t,
) {
    let litLimit_w = litLimit.offset(-(WILDCOPY_OVERLENGTH as isize));
    let litEnd = literals.offset(litLength as isize);
    if (((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as libc::c_long as size_t) < (*seqStorePtr).maxNbSeq
    {} else {
        __assert_fail(
            b"(size_t)(seqStorePtr->sequences - seqStorePtr->sequencesStart) < seqStorePtr->maxNbSeq\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            669 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*seqStorePtr).maxNbLit
        <= (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulong
    {} else {
        __assert_fail(
            b"seqStorePtr->maxNbLit <= 128 KB\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            671 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*seqStorePtr).lit).offset(litLength as isize)
        <= ((*seqStorePtr).litStart).offset((*seqStorePtr).maxNbLit as isize)
    {} else {
        __assert_fail(
            b"seqStorePtr->lit + litLength <= seqStorePtr->litStart + seqStorePtr->maxNbLit\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            672 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if literals.offset(litLength as isize) <= litLimit {} else {
        __assert_fail(
            b"literals + litLength <= litLimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            673 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if litEnd <= litLimit_w {
        ZSTD_copy16(
            (*seqStorePtr).lit as *mut libc::c_void,
            literals as *const libc::c_void,
        );
        if litLength > 16 as libc::c_int as libc::c_ulong {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16 as libc::c_int as isize)
                    as *mut libc::c_void,
                literals.offset(16 as libc::c_int as isize) as *const libc::c_void,
                litLength as ptrdiff_t - 16 as libc::c_int as libc::c_long,
                ZSTD_no_overlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(litLength as isize);
    if litLength > 0xffff as libc::c_int as libc::c_ulong {
        if (*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"seqStorePtr->longLengthType == ZSTD_llt_none\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                690 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as U32;
    }
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize))
        .litLength = litLength as U16;
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize)).offBase = offBase;
    if matchLength >= 3 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"matchLength >= MINMATCH\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            700 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mlBase = matchLength.wrapping_sub(MINMATCH as libc::c_ulong);
    if mlBase > 0xffff as libc::c_int as libc::c_ulong {
        if (*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"seqStorePtr->longLengthType == ZSTD_llt_none\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                    as *const u8 as *const libc::c_char,
                703 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void ZSTD_storeSeq(seqStore_t *, size_t, const BYTE *, const BYTE *, U32, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as U32;
    }
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize))
        .mlBase = mlBase as U16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
}
pub const WILDCOPY_OVERLENGTH: libc::c_int = 32 as libc::c_int;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_count(
    mut pIn: *const BYTE,
    mut pMatch: *const BYTE,
    pInLimit: *const BYTE,
) -> size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit
        .offset(
            -((::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
        );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const libc::c_void)
            ^ MEM_readST(pIn as *const libc::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
        pMatch = pMatch
            .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const libc::c_void)
                ^ MEM_readST(pIn as *const libc::c_void);
            if diff_0 == 0 {
                pIn = pIn
                    .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
                pMatch = pMatch
                    .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as libc::c_long as size_t;
            }
        }
    }
    if MEM_64bits() != 0 && pIn < pInLimit.offset(-(3 as libc::c_int as isize))
        && MEM_read32(pMatch as *const libc::c_void)
            == MEM_read32(pIn as *const libc::c_void)
    {
        pIn = pIn.offset(4 as libc::c_int as isize);
        pMatch = pMatch.offset(4 as libc::c_int as isize);
    }
    if pIn < pInLimit.offset(-(1 as libc::c_int as isize))
        && MEM_read16(pMatch as *const libc::c_void) as libc::c_int
            == MEM_read16(pIn as *const libc::c_void) as libc::c_int
    {
        pIn = pIn.offset(2 as libc::c_int as isize);
        pMatch = pMatch.offset(2 as libc::c_int as isize);
    }
    if pIn < pInLimit && *pMatch as libc::c_int == *pIn as libc::c_int {
        pIn = pIn.offset(1);
    }
    return pIn.offset_from(pStart) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_count_2segments(
    mut ip: *const BYTE,
    mut match_0: *const BYTE,
    mut iEnd: *const BYTE,
    mut mEnd: *const BYTE,
    mut iStart: *const BYTE,
) -> size_t {
    let vEnd = if ip.offset(mEnd.offset_from(match_0) as libc::c_long as isize) < iEnd {
        ip.offset(mEnd.offset_from(match_0) as libc::c_long as isize)
    } else {
        iEnd
    };
    let matchLength = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength;
    }
    return matchLength
        .wrapping_add(ZSTD_count(ip.offset(matchLength as isize), iStart, iEnd));
}
static mut prime4bytes: U32 = 2654435761 as libc::c_uint;
unsafe extern "C" fn ZSTD_hash4(mut u: U32, mut h: U32, mut s: U32) -> U32 {
    if h <= 32 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 32\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            804 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"U32 ZSTD_hash4(U32, U32, U32)\0"))
                .as_ptr(),
        );
    }
    return (u.wrapping_mul(prime4bytes) ^ s)
        >> (32 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash4(MEM_readLE32(ptr), h, 0 as libc::c_int as U32) as size_t;
}
unsafe extern "C" fn ZSTD_hash4PtrS(
    mut ptr: *const libc::c_void,
    mut h: U32,
    mut s: U32,
) -> size_t {
    return ZSTD_hash4(MEM_readLE32(ptr), h, s) as size_t;
}
static mut prime5bytes: U64 = 889523592379 as libc::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash5(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    if h <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            809 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"size_t ZSTD_hash5(U64, U32, U64)\0"))
                .as_ptr(),
        );
    }
    return ((u << 64 as libc::c_int - 40 as libc::c_int).wrapping_mul(prime5bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash5(MEM_readLE64(p), h, 0 as libc::c_int as U64);
}
unsafe extern "C" fn ZSTD_hash5PtrS(
    mut p: *const libc::c_void,
    mut h: U32,
    mut s: U64,
) -> size_t {
    return ZSTD_hash5(MEM_readLE64(p), h, s);
}
static mut prime6bytes: U64 = 227718039650203 as libc::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    if h <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            814 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"size_t ZSTD_hash6(U64, U32, U64)\0"))
                .as_ptr(),
        );
    }
    return ((u << 64 as libc::c_int - 48 as libc::c_int).wrapping_mul(prime6bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h, 0 as libc::c_int as U64);
}
unsafe extern "C" fn ZSTD_hash6PtrS(
    mut p: *const libc::c_void,
    mut h: U32,
    mut s: U64,
) -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h, s);
}
static mut prime7bytes: U64 = 58295818150454627 as libc::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash7(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    if h <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            819 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"size_t ZSTD_hash7(U64, U32, U64)\0"))
                .as_ptr(),
        );
    }
    return ((u << 64 as libc::c_int - 56 as libc::c_int).wrapping_mul(prime7bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash7(MEM_readLE64(p), h, 0 as libc::c_int as U64);
}
unsafe extern "C" fn ZSTD_hash7PtrS(
    mut p: *const libc::c_void,
    mut h: U32,
    mut s: U64,
) -> size_t {
    return ZSTD_hash7(MEM_readLE64(p), h, s);
}
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463 as libc::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash8(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    if h <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h <= 64\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            824 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"size_t ZSTD_hash8(U64, U32, U64)\0"))
                .as_ptr(),
        );
    }
    return (u.wrapping_mul(prime8bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const libc::c_void, mut h: U32) -> size_t {
    return ZSTD_hash8(MEM_readLE64(p), h, 0 as libc::c_int as U64);
}
unsafe extern "C" fn ZSTD_hash8PtrS(
    mut p: *const libc::c_void,
    mut h: U32,
    mut s: U64,
) -> size_t {
    return ZSTD_hash8(MEM_readLE64(p), h, s);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_hashPtr(
    mut p: *const libc::c_void,
    mut hBits: U32,
    mut mls: U32,
) -> size_t {
    if hBits <= 32 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"hBits <= 32\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            834 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"size_t ZSTD_hashPtr(const void *, U32, U32)\0"))
                .as_ptr(),
        );
    }
    match mls {
        5 => return ZSTD_hash5Ptr(p, hBits),
        6 => return ZSTD_hash6Ptr(p, hBits),
        7 => return ZSTD_hash7Ptr(p, hBits),
        8 => return ZSTD_hash8Ptr(p, hBits),
        4 | _ => return ZSTD_hash4Ptr(p, hBits),
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_hashPtrSalted(
    mut p: *const libc::c_void,
    mut hBits: U32,
    mut mls: U32,
    hashSalt: U64,
) -> size_t {
    if hBits <= 32 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"hBits <= 32\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            851 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"size_t ZSTD_hashPtrSalted(const void *, U32, U32, const U64)\0"))
                .as_ptr(),
        );
    }
    match mls {
        5 => return ZSTD_hash5PtrS(p, hBits, hashSalt),
        6 => return ZSTD_hash6PtrS(p, hBits, hashSalt),
        7 => return ZSTD_hash7PtrS(p, hBits, hashSalt),
        8 => return ZSTD_hash8PtrS(p, hBits, hashSalt),
        4 | _ => return ZSTD_hash4PtrS(p, hBits, hashSalt as U32),
    };
}
#[inline]
unsafe extern "C" fn ZSTD_getLowestMatchIndex(
    mut ms: *const ZSTD_matchState_t,
    mut curr: U32,
    mut windowLog: libc::c_uint,
) -> U32 {
    let maxDistance = (1 as libc::c_uint) << windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as U32;
    let matchLowest = if isDictionary != 0 { lowestValid } else { withinWindow };
    return matchLowest;
}
#[inline]
unsafe extern "C" fn ZSTD_getLowestPrefixIndex(
    mut ms: *const ZSTD_matchState_t,
    mut curr: U32,
    mut windowLog: libc::c_uint,
) -> U32 {
    let maxDistance = (1 as libc::c_uint) << windowLog;
    let lowestValid = (*ms).window.dictLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as U32;
    let matchLowest = if isDictionary != 0 { lowestValid } else { withinWindow };
    return matchLowest;
}
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: libc::c_int = 2 as libc::c_int;
pub const ZSTD_ROW_HASH_TAG_BITS: libc::c_int = 8 as libc::c_int;
pub const kLazySkippingStep: libc::c_int = 8 as libc::c_int;
unsafe extern "C" fn ZSTD_updateDUBT(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    mut iend: *const BYTE,
    mut mls: U32,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let btMask = (((1 as libc::c_int) << btLog) - 1 as libc::c_int) as U32;
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as libc::c_long as U32;
    let mut idx = (*ms).nextToUpdate;
    idx != target;
    if ip.offset(8 as libc::c_int as isize) <= iend {} else {
        __assert_fail(
            b"ip + 8 <= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void ZSTD_updateDUBT(ZSTD_matchState_t *, const BYTE *, const BYTE *, U32)\0",
            ))
                .as_ptr(),
        );
    }
    if idx >= (*ms).window.dictLimit {} else {
        __assert_fail(
            b"idx >= ms->window.dictLimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void ZSTD_updateDUBT(ZSTD_matchState_t *, const BYTE *, const BYTE *, U32)\0",
            ))
                .as_ptr(),
        );
    }
    while idx < target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const libc::c_void,
            hashLog,
            mls,
        );
        let matchIndex = *hashTable.offset(h as isize);
        let nextCandidatePtr = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(idx & btMask) as isize,
            );
        let sortMarkPtr = nextCandidatePtr.offset(1 as libc::c_int as isize);
        *hashTable.offset(h as isize) = idx;
        *nextCandidatePtr = matchIndex;
        *sortMarkPtr = ZSTD_DUBT_UNSORTED_MARK as U32;
        idx = idx.wrapping_add(1);
    }
    (*ms).nextToUpdate = target;
}
unsafe extern "C" fn ZSTD_insertDUBT1(
    mut ms: *const ZSTD_matchState_t,
    mut curr: U32,
    mut inputEnd: *const BYTE,
    mut nbCompares: U32,
    mut btLow: U32,
    dictMode: ZSTD_dictMode_e,
) {
    let cParams: *const ZSTD_compressionParameters = &(*ms).cParams;
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let btMask = (((1 as libc::c_int) << btLog) - 1 as libc::c_int) as U32;
    let mut commonLengthSmaller = 0 as libc::c_int as size_t;
    let mut commonLengthLarger = 0 as libc::c_int as size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let ip = if curr >= dictLimit {
        base.offset(curr as isize)
    } else {
        dictBase.offset(curr as isize)
    };
    let iend = if curr >= dictLimit {
        inputEnd
    } else {
        dictBase.offset(dictLimit as isize)
    };
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut match_0 = 0 as *const BYTE;
    let mut smallerPtr = bt
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(curr & btMask) as isize);
    let mut largerPtr = smallerPtr.offset(1 as libc::c_int as isize);
    let mut matchIndex = *smallerPtr;
    let mut dummy32: U32 = 0;
    let windowValid = (*ms).window.lowLimit;
    let maxDistance = (1 as libc::c_uint) << (*cParams).windowLog;
    let windowLow = if curr.wrapping_sub(windowValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        windowValid
    };
    if curr >= btLow {} else {
        __assert_fail(
            b"curr >= btLow\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"void ZSTD_insertDUBT1(const ZSTD_matchState_t *, U32, const BYTE *, U32, U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    if ip < iend {} else {
        __assert_fail(
            b"ip < iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"void ZSTD_insertDUBT1(const ZSTD_matchState_t *, U32, const BYTE *, U32, U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    while nbCompares != 0 && matchIndex > windowLow {
        let nextPtr = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask)
                    as isize,
            );
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        if matchIndex < curr {} else {
            __assert_fail(
                b"matchIndex < curr\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                102 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"void ZSTD_insertDUBT1(const ZSTD_matchState_t *, U32, const BYTE *, U32, U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if dictMode as libc::c_uint != ZSTD_extDict as libc::c_int as libc::c_uint
            || (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong || curr < dictLimit
        {
            let mBase = if dictMode as libc::c_uint
                != ZSTD_extDict as libc::c_int as libc::c_uint
                || (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                    >= dictLimit as libc::c_ulong
            {
                base
            } else {
                dictBase
            };
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong || curr < dictLimit
            {} else {
                __assert_fail(
                    b"(matchIndex+matchLength >= dictLimit) || (curr < dictLimit)\0"
                        as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    114 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"void ZSTD_insertDUBT1(const ZSTD_matchState_t *, U32, const BYTE *, U32, U32, const ZSTD_dictMode_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            match_0 = mBase.offset(matchIndex as isize);
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count(
                        ip.offset(matchLength as isize),
                        match_0.offset(matchLength as isize),
                        iend,
                    ),
                ) as size_t as size_t;
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count_2segments(
                        ip.offset(matchLength as isize),
                        match_0.offset(matchLength as isize),
                        iend,
                        dictEnd,
                        prefixStart,
                    ),
                ) as size_t as size_t;
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
            {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if ip.offset(matchLength as isize) == iend {
            break;
        } else {
            if (*match_0.offset(matchLength as isize) as libc::c_int)
                < *ip.offset(matchLength as isize) as libc::c_int
            {
                *smallerPtr = matchIndex;
                commonLengthSmaller = matchLength;
                if matchIndex <= btLow {
                    smallerPtr = &mut dummy32;
                    break;
                } else {
                    smallerPtr = nextPtr.offset(1 as libc::c_int as isize);
                    matchIndex = *nextPtr.offset(1 as libc::c_int as isize);
                }
            } else {
                *largerPtr = matchIndex;
                commonLengthLarger = matchLength;
                if matchIndex <= btLow {
                    largerPtr = &mut dummy32;
                    break;
                } else {
                    largerPtr = nextPtr;
                    matchIndex = *nextPtr.offset(0 as libc::c_int as isize);
                }
            }
            nbCompares = nbCompares.wrapping_sub(1);
        }
    }
    *largerPtr = 0 as libc::c_int as U32;
    *smallerPtr = *largerPtr;
}
unsafe extern "C" fn ZSTD_DUBT_findBetterDictMatch(
    mut ms: *const ZSTD_matchState_t,
    ip: *const BYTE,
    iend: *const BYTE,
    mut offsetPtr: *mut size_t,
    mut bestLength: size_t,
    mut nbCompares: U32,
    mls: U32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let dms = (*ms).dictMatchState;
    let dmsCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const U32 = (*dms).hashTable;
    let hashLog = (*dmsCParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let mut dictMatchIndex = *dictHashTable.offset(h as isize);
    let base = (*ms).window.base;
    let prefixStart = base.offset((*ms).window.dictLimit as isize);
    let curr = ip.offset_from(base) as libc::c_long as U32;
    let dictBase = (*dms).window.base;
    let dictEnd = (*dms).window.nextSrc;
    let dictHighLimit = ((*dms).window.nextSrc).offset_from((*dms).window.base)
        as libc::c_long as U32;
    let dictLowLimit = (*dms).window.lowLimit;
    let dictIndexDelta = ((*ms).window.lowLimit).wrapping_sub(dictHighLimit);
    let dictBt = (*dms).chainTable;
    let btLog = ((*dmsCParams).chainLog).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let btMask = (((1 as libc::c_int) << btLog) - 1 as libc::c_int) as U32;
    let btLow = if btMask >= dictHighLimit.wrapping_sub(dictLowLimit) {
        dictLowLimit
    } else {
        dictHighLimit.wrapping_sub(btMask)
    };
    let mut commonLengthSmaller = 0 as libc::c_int as size_t;
    let mut commonLengthLarger = 0 as libc::c_int as size_t;
    if dictMode as libc::c_uint == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"dictMode == ZSTD_dictMatchState\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 159],
                &[libc::c_char; 159],
            >(
                b"size_t ZSTD_DUBT_findBetterDictMatch(const ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, size_t, U32, const U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    while nbCompares != 0 && dictMatchIndex > dictLowLimit {
        let nextPtr = dictBt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(dictMatchIndex & btMask)
                    as isize,
            );
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        let mut match_0 = dictBase.offset(dictMatchIndex as isize);
        matchLength = (matchLength as libc::c_ulong)
            .wrapping_add(
                ZSTD_count_2segments(
                    ip.offset(matchLength as isize),
                    match_0.offset(matchLength as isize),
                    iend,
                    dictEnd,
                    prefixStart,
                ),
            ) as size_t as size_t;
        if (dictMatchIndex as libc::c_ulong).wrapping_add(matchLength)
            >= dictHighLimit as libc::c_ulong
        {
            match_0 = base
                .offset(dictMatchIndex as isize)
                .offset(dictIndexDelta as isize);
        }
        if matchLength > bestLength {
            let mut matchIndex = dictMatchIndex.wrapping_add(dictIndexDelta);
            if 4 as libc::c_int * matchLength.wrapping_sub(bestLength) as libc::c_int
                > (ZSTD_highbit32(
                    curr
                        .wrapping_sub(matchIndex)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                ))
                    .wrapping_sub(
                        ZSTD_highbit32(
                            (*offsetPtr.offset(0 as libc::c_int as isize) as U32)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ),
                    ) as libc::c_int
            {
                bestLength = matchLength;
                if curr.wrapping_sub(matchIndex) > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"(curr - matchIndex)>0\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        204 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 159],
                            &[libc::c_char; 159],
                        >(
                            b"size_t ZSTD_DUBT_findBetterDictMatch(const ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, size_t, U32, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *offsetPtr = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
            }
            if ip.offset(matchLength as isize) == iend {
                break;
            }
        }
        if (*match_0.offset(matchLength as isize) as libc::c_int)
            < *ip.offset(matchLength as isize) as libc::c_int
        {
            if dictMatchIndex <= btLow {
                break;
            }
            commonLengthSmaller = matchLength;
            dictMatchIndex = *nextPtr.offset(1 as libc::c_int as isize);
        } else {
            if dictMatchIndex <= btLow {
                break;
            }
            commonLengthLarger = matchLength;
            dictMatchIndex = *nextPtr.offset(0 as libc::c_int as isize);
        }
        nbCompares = nbCompares.wrapping_sub(1);
    }
    if bestLength >= MINMATCH as libc::c_ulong {
        if *offsetPtr > 3 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"OFFBASE_IS_OFFSET(*offsetPtr)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 159],
                    &[libc::c_char; 159],
                >(
                    b"size_t ZSTD_DUBT_findBetterDictMatch(const ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, size_t, U32, const U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        let mIndex = curr
            .wrapping_sub(
                (*offsetPtr).wrapping_sub(ZSTD_REP_NUM as libc::c_ulong) as U32,
            );
    }
    return bestLength;
}
unsafe extern "C" fn ZSTD_DUBT_findBestMatch(
    mut ms: *mut ZSTD_matchState_t,
    ip: *const BYTE,
    iend: *const BYTE,
    mut offBasePtr: *mut size_t,
    mls: U32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.offset(h as isize);
    let base = (*ms).window.base;
    let curr = ip.offset_from(base) as libc::c_long as U32;
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, (*cParams).windowLog);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let btMask = (((1 as libc::c_int) << btLog) - 1 as libc::c_int) as U32;
    let btLow = if btMask >= curr {
        0 as libc::c_int as libc::c_uint
    } else {
        curr.wrapping_sub(btMask)
    };
    let unsortLimit = if btLow > windowLow { btLow } else { windowLow };
    let mut nextCandidate = bt
        .offset(
            (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask) as isize,
        );
    let mut unsortedMark = bt
        .offset(
            (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask) as isize,
        )
        .offset(1 as libc::c_int as isize);
    let mut nbCompares = (1 as libc::c_uint) << (*cParams).searchLog;
    let mut nbCandidates = nbCompares;
    let mut previousCandidate = 0 as libc::c_int as U32;
    if ip <= iend.offset(-(8 as libc::c_int as isize)) {} else {
        __assert_fail(
            b"ip <= iend-8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"size_t ZSTD_DUBT_findBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    if dictMode as libc::c_uint
        != ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"dictMode != ZSTD_dedicatedDictSearch\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            264 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"size_t ZSTD_DUBT_findBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    while matchIndex > unsortLimit
        && *unsortedMark == ZSTD_DUBT_UNSORTED_MARK as libc::c_uint
        && nbCandidates > 1 as libc::c_int as libc::c_uint
    {
        *unsortedMark = previousCandidate;
        previousCandidate = matchIndex;
        matchIndex = *nextCandidate;
        nextCandidate = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask)
                    as isize,
            );
        unsortedMark = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask)
                    as isize,
            )
            .offset(1 as libc::c_int as isize);
        nbCandidates = nbCandidates.wrapping_sub(1);
    }
    if matchIndex > unsortLimit
        && *unsortedMark == ZSTD_DUBT_UNSORTED_MARK as libc::c_uint
    {
        *unsortedMark = 0 as libc::c_int as U32;
        *nextCandidate = *unsortedMark;
    }
    matchIndex = previousCandidate;
    while matchIndex != 0 {
        let nextCandidateIdxPtr = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask)
                    as isize,
            )
            .offset(1 as libc::c_int as isize);
        let nextCandidateIdx = *nextCandidateIdxPtr;
        ZSTD_insertDUBT1(ms, matchIndex, iend, nbCandidates, unsortLimit, dictMode);
        matchIndex = nextCandidateIdx;
        nbCandidates = nbCandidates.wrapping_add(1);
    }
    let mut commonLengthSmaller = 0 as libc::c_int as size_t;
    let mut commonLengthLarger = 0 as libc::c_int as size_t;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut smallerPtr = bt
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(curr & btMask) as isize);
    let mut largerPtr = bt
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(curr & btMask) as isize)
        .offset(1 as libc::c_int as isize);
    let mut matchEndIdx = curr
        .wrapping_add(8 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut dummy32: U32 = 0;
    let mut bestLength = 0 as libc::c_int as size_t;
    matchIndex = *hashTable.offset(h as isize);
    *hashTable.offset(h as isize) = curr;
    while nbCompares != 0 && matchIndex > windowLow {
        let nextPtr = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask)
                    as isize,
            );
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        let mut match_0 = 0 as *const BYTE;
        if dictMode as libc::c_uint != ZSTD_extDict as libc::c_int as libc::c_uint
            || (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
        {
            match_0 = base.offset(matchIndex as isize);
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count(
                        ip.offset(matchLength as isize),
                        match_0.offset(matchLength as isize),
                        iend,
                    ),
                ) as size_t as size_t;
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count_2segments(
                        ip.offset(matchLength as isize),
                        match_0.offset(matchLength as isize),
                        iend,
                        dictEnd,
                        prefixStart,
                    ),
                ) as size_t as size_t;
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
            {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                matchEndIdx = matchIndex.wrapping_add(matchLength as U32);
            }
            if 4 as libc::c_int * matchLength.wrapping_sub(bestLength) as libc::c_int
                > (ZSTD_highbit32(
                    curr
                        .wrapping_sub(matchIndex)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                ))
                    .wrapping_sub(ZSTD_highbit32(*offBasePtr as U32)) as libc::c_int
            {
                bestLength = matchLength;
                if curr.wrapping_sub(matchIndex) > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"(curr - matchIndex)>0\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        334 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 134],
                            &[libc::c_char; 134],
                        >(
                            b"size_t ZSTD_DUBT_findBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *offBasePtr = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
            }
            if ip.offset(matchLength as isize) == iend {
                if dictMode as libc::c_uint
                    == ZSTD_dictMatchState as libc::c_int as libc::c_uint
                {
                    nbCompares = 0 as libc::c_int as U32;
                }
                break;
            }
        }
        if (*match_0.offset(matchLength as isize) as libc::c_int)
            < *ip.offset(matchLength as isize) as libc::c_int
        {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                break;
            } else {
                smallerPtr = nextPtr.offset(1 as libc::c_int as isize);
                matchIndex = *nextPtr.offset(1 as libc::c_int as isize);
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                break;
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr.offset(0 as libc::c_int as isize);
            }
        }
        nbCompares = nbCompares.wrapping_sub(1);
    }
    *largerPtr = 0 as libc::c_int as U32;
    *smallerPtr = *largerPtr;
    if nbCompares
        <= (1 as libc::c_uint)
            << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            }) - 1 as libc::c_int
    {} else {
        __assert_fail(
            b"nbCompares <= (1U << ZSTD_SEARCHLOG_MAX)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            363 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"size_t ZSTD_DUBT_findBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    if dictMode as libc::c_uint == ZSTD_dictMatchState as libc::c_int as libc::c_uint
        && nbCompares != 0
    {
        bestLength = ZSTD_DUBT_findBetterDictMatch(
            ms,
            ip,
            iend,
            offBasePtr,
            bestLength,
            nbCompares,
            mls,
            dictMode,
        );
    }
    if matchEndIdx > curr.wrapping_add(8 as libc::c_int as libc::c_uint) {} else {
        __assert_fail(
            b"matchEndIdx > curr+8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            371 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"size_t ZSTD_DUBT_findBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    (*ms).nextToUpdate = matchEndIdx.wrapping_sub(8 as libc::c_int as libc::c_uint);
    if bestLength >= MINMATCH as libc::c_ulong {
        if *offBasePtr > 3 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"OFFBASE_IS_OFFSET(*offBasePtr)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                374 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 134],
                    &[libc::c_char; 134],
                >(
                    b"size_t ZSTD_DUBT_findBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        let mIndex = curr
            .wrapping_sub(
                (*offBasePtr).wrapping_sub(ZSTD_REP_NUM as libc::c_ulong) as U32,
            );
    }
    return bestLength;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_BtFindBestMatch(
    mut ms: *mut ZSTD_matchState_t,
    ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
    mls: U32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    if ip < ((*ms).window.base).offset((*ms).nextToUpdate as isize) {
        return 0 as libc::c_int as size_t;
    }
    ZSTD_updateDUBT(ms, ip, iLimit, mls);
    return ZSTD_DUBT_findBestMatch(ms, ip, iLimit, offBasePtr, mls, dictMode);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_dedicatedDictSearch_lazy_loadDictionary(
    mut ms: *mut ZSTD_matchState_t,
    ip: *const BYTE,
) {
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as libc::c_long as U32;
    let hashTable = (*ms).hashTable;
    let chainTable = (*ms).chainTable;
    let chainSize = ((1 as libc::c_int) << (*ms).cParams.chainLog) as U32;
    let mut idx = (*ms).nextToUpdate;
    let minChain = if chainSize < target.wrapping_sub(idx) {
        target.wrapping_sub(chainSize)
    } else {
        idx
    };
    let bucketSize = ((1 as libc::c_int) << ZSTD_LAZY_DDSS_BUCKET_LOG) as U32;
    let cacheSize = bucketSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let chainAttempts = (((1 as libc::c_int) << (*ms).cParams.searchLog) as libc::c_uint)
        .wrapping_sub(cacheSize);
    let chainLimit = if chainAttempts > 255 as libc::c_int as libc::c_uint {
        255 as libc::c_int as libc::c_uint
    } else {
        chainAttempts
    };
    let hashLog = ((*ms).cParams.hashLog)
        .wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as libc::c_uint);
    let tmpHashTable = hashTable;
    let tmpChainTable = hashTable
        .offset(((1 as libc::c_int as size_t) << hashLog) as isize);
    let tmpChainSize = ((((1 as libc::c_int) << ZSTD_LAZY_DDSS_BUCKET_LOG)
        - 1 as libc::c_int) as U32) << hashLog;
    let tmpMinChain = if tmpChainSize < target {
        target.wrapping_sub(tmpChainSize)
    } else {
        idx
    };
    let mut hashIdx: U32 = 0;
    if (*ms).cParams.chainLog <= 24 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"ms->cParams.chainLog <= 24\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            427 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void ZSTD_dedicatedDictSearch_lazy_loadDictionary(ZSTD_matchState_t *, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if (*ms).cParams.hashLog > (*ms).cParams.chainLog {} else {
        __assert_fail(
            b"ms->cParams.hashLog > ms->cParams.chainLog\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            428 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void ZSTD_dedicatedDictSearch_lazy_loadDictionary(ZSTD_matchState_t *, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if idx != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"idx != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            429 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void ZSTD_dedicatedDictSearch_lazy_loadDictionary(ZSTD_matchState_t *, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if tmpMinChain <= minChain {} else {
        __assert_fail(
            b"tmpMinChain <= minChain\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            430 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void ZSTD_dedicatedDictSearch_lazy_loadDictionary(ZSTD_matchState_t *, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    while idx < target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const libc::c_void,
            hashLog,
            (*ms).cParams.minMatch,
        ) as U32;
        if idx >= tmpMinChain {
            *tmpChainTable
                .offset(
                    idx.wrapping_sub(tmpMinChain) as isize,
                ) = *hashTable.offset(h as isize);
        }
        *tmpHashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1);
    }
    let mut chainPos = 0 as libc::c_int as U32;
    hashIdx = 0 as libc::c_int as U32;
    while hashIdx < (1 as libc::c_uint) << hashLog {
        let mut count: U32 = 0;
        let mut countBeyondMinChain = 0 as libc::c_int as U32;
        let mut i = *tmpHashTable.offset(hashIdx as isize);
        count = 0 as libc::c_int as U32;
        while i >= tmpMinChain && count < cacheSize {
            if i < minChain {
                countBeyondMinChain = countBeyondMinChain.wrapping_add(1);
            }
            i = *tmpChainTable.offset(i.wrapping_sub(tmpMinChain) as isize);
            count = count.wrapping_add(1);
        }
        if count == cacheSize {
            count = 0 as libc::c_int as U32;
            while count < chainLimit {
                if i < minChain {
                    if i == 0
                        || {
                            countBeyondMinChain = countBeyondMinChain.wrapping_add(1);
                            countBeyondMinChain > cacheSize
                        }
                    {
                        break;
                    }
                }
                let fresh2 = chainPos;
                chainPos = chainPos.wrapping_add(1);
                *chainTable.offset(fresh2 as isize) = i;
                count = count.wrapping_add(1);
                if i < tmpMinChain {
                    break;
                }
                i = *tmpChainTable.offset(i.wrapping_sub(tmpMinChain) as isize);
            }
        } else {
            count = 0 as libc::c_int as U32;
        }
        if count != 0 {
            *tmpHashTable
                .offset(
                    hashIdx as isize,
                ) = (chainPos.wrapping_sub(count) << 8 as libc::c_int)
                .wrapping_add(count);
        } else {
            *tmpHashTable.offset(hashIdx as isize) = 0 as libc::c_int as U32;
        }
        hashIdx = hashIdx.wrapping_add(1);
    }
    if chainPos <= chainSize {} else {
        __assert_fail(
            b"chainPos <= chainSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            487 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void ZSTD_dedicatedDictSearch_lazy_loadDictionary(ZSTD_matchState_t *, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    hashIdx = ((1 as libc::c_int) << hashLog) as U32;
    while hashIdx != 0 {
        hashIdx = hashIdx.wrapping_sub(1);
        let bucketIdx = hashIdx << ZSTD_LAZY_DDSS_BUCKET_LOG;
        let chainPackedPointer = *tmpHashTable.offset(hashIdx as isize);
        let mut i_0: U32 = 0;
        i_0 = 0 as libc::c_int as U32;
        while i_0 < cacheSize {
            *hashTable
                .offset(bucketIdx.wrapping_add(i_0) as isize) = 0 as libc::c_int as U32;
            i_0 = i_0.wrapping_add(1);
        }
        *hashTable
            .offset(
                bucketIdx
                    .wrapping_add(bucketSize)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = chainPackedPointer;
    }
    idx = (*ms).nextToUpdate;
    while idx < target {
        let h_0 = (ZSTD_hashPtr(
            base.offset(idx as isize) as *const libc::c_void,
            hashLog,
            (*ms).cParams.minMatch,
        ) as U32) << ZSTD_LAZY_DDSS_BUCKET_LOG;
        let mut i_1: U32 = 0;
        i_1 = cacheSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while i_1 != 0 {
            *hashTable
                .offset(
                    h_0.wrapping_add(i_1) as isize,
                ) = *hashTable
                .offset(
                    h_0.wrapping_add(i_1).wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as isize,
                );
            i_1 = i_1.wrapping_sub(1);
        }
        *hashTable.offset(h_0 as isize) = idx;
        idx = idx.wrapping_add(1);
    }
    (*ms).nextToUpdate = target;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_dedicatedDictSearch_lazy_search(
    mut offsetPtr: *mut size_t,
    mut ml: size_t,
    mut nbAttempts: U32,
    dms: *const ZSTD_matchState_t,
    ip: *const BYTE,
    iLimit: *const BYTE,
    prefixStart: *const BYTE,
    curr: U32,
    dictLimit: U32,
    ddsIdx: size_t,
) -> size_t {
    let ddsLowestIndex = (*dms).window.dictLimit;
    let ddsBase = (*dms).window.base;
    let ddsEnd = (*dms).window.nextSrc;
    let ddsSize = ddsEnd.offset_from(ddsBase) as libc::c_long as U32;
    let ddsIndexDelta = dictLimit.wrapping_sub(ddsSize);
    let bucketSize = ((1 as libc::c_int) << ZSTD_LAZY_DDSS_BUCKET_LOG) as U32;
    let bucketLimit = if nbAttempts
        < bucketSize.wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        nbAttempts
    } else {
        bucketSize.wrapping_sub(1 as libc::c_int as libc::c_uint)
    };
    let mut ddsAttempt: U32 = 0;
    let mut matchIndex: U32 = 0;
    ddsAttempt = 0 as libc::c_int as U32;
    while ddsAttempt < bucketSize.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        ddsAttempt = ddsAttempt.wrapping_add(1);
    }
    let chainPackedPointer = *((*dms).hashTable)
        .offset(
            ddsIdx
                .wrapping_add(bucketSize as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    let chainIndex = chainPackedPointer >> 8 as libc::c_int;
    ddsAttempt = 0 as libc::c_int as U32;
    while ddsAttempt < bucketLimit {
        let mut currentMl = 0 as libc::c_int as size_t;
        let mut match_0 = 0 as *const BYTE;
        matchIndex = *((*dms).hashTable)
            .offset(ddsIdx.wrapping_add(ddsAttempt as libc::c_ulong) as isize);
        match_0 = ddsBase.offset(matchIndex as isize);
        if matchIndex == 0 {
            return ml;
        }
        if matchIndex >= ddsLowestIndex {} else {
            __assert_fail(
                b"matchIndex >= ddsLowestIndex\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                557 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"size_t ZSTD_dedicatedDictSearch_lazy_search(size_t *, size_t, U32, const ZSTD_matchState_t *const, const BYTE *const, const BYTE *const, const BYTE *const, const U32, const U32, const size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if match_0.offset(4 as libc::c_int as isize) <= ddsEnd {} else {
            __assert_fail(
                b"match+4 <= ddsEnd\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                558 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"size_t ZSTD_dedicatedDictSearch_lazy_search(size_t *, size_t, U32, const ZSTD_matchState_t *const, const BYTE *const, const BYTE *const, const BYTE *const, const U32, const U32, const size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if MEM_read32(match_0 as *const libc::c_void)
            == MEM_read32(ip as *const libc::c_void)
        {
            currentMl = (ZSTD_count_2segments(
                ip.offset(4 as libc::c_int as isize),
                match_0.offset(4 as libc::c_int as isize),
                iLimit,
                ddsEnd,
                prefixStart,
            ))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
        }
        if currentMl > ml {
            ml = currentMl;
            if curr.wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"(curr - (matchIndex + ddsIndexDelta))>0\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    567 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 192],
                        &[libc::c_char; 192],
                    >(
                        b"size_t ZSTD_dedicatedDictSearch_lazy_search(size_t *, size_t, U32, const ZSTD_matchState_t *const, const BYTE *const, const BYTE *const, const BYTE *const, const U32, const U32, const size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            *offsetPtr = curr
                .wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                return ml;
            }
        }
        ddsAttempt = ddsAttempt.wrapping_add(1);
    }
    let chainPackedPointer_0 = *((*dms).hashTable)
        .offset(
            ddsIdx
                .wrapping_add(bucketSize as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    let mut chainIndex_0 = chainPackedPointer_0 >> 8 as libc::c_int;
    let chainLength = chainPackedPointer_0 & 0xff as libc::c_int as libc::c_uint;
    let chainAttempts = nbAttempts.wrapping_sub(ddsAttempt);
    let chainLimit = if chainAttempts > chainLength {
        chainLength
    } else {
        chainAttempts
    };
    let mut chainAttempt: U32 = 0;
    chainAttempt = 0 as libc::c_int as U32;
    while chainAttempt < chainLimit {
        chainAttempt = chainAttempt.wrapping_add(1);
    }
    chainAttempt = 0 as libc::c_int as U32;
    while chainAttempt < chainLimit {
        let mut currentMl_0 = 0 as libc::c_int as size_t;
        let mut match_1 = 0 as *const BYTE;
        matchIndex = *((*dms).chainTable).offset(chainIndex_0 as isize);
        match_1 = ddsBase.offset(matchIndex as isize);
        if matchIndex >= ddsLowestIndex {} else {
            __assert_fail(
                b"matchIndex >= ddsLowestIndex\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                594 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"size_t ZSTD_dedicatedDictSearch_lazy_search(size_t *, size_t, U32, const ZSTD_matchState_t *const, const BYTE *const, const BYTE *const, const BYTE *const, const U32, const U32, const size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if match_1.offset(4 as libc::c_int as isize) <= ddsEnd {} else {
            __assert_fail(
                b"match+4 <= ddsEnd\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                595 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"size_t ZSTD_dedicatedDictSearch_lazy_search(size_t *, size_t, U32, const ZSTD_matchState_t *const, const BYTE *const, const BYTE *const, const BYTE *const, const U32, const U32, const size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if MEM_read32(match_1 as *const libc::c_void)
            == MEM_read32(ip as *const libc::c_void)
        {
            currentMl_0 = (ZSTD_count_2segments(
                ip.offset(4 as libc::c_int as isize),
                match_1.offset(4 as libc::c_int as isize),
                iLimit,
                ddsEnd,
                prefixStart,
            ))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
        }
        if currentMl_0 > ml {
            ml = currentMl_0;
            if curr.wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"(curr - (matchIndex + ddsIndexDelta))>0\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    604 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 192],
                        &[libc::c_char; 192],
                    >(
                        b"size_t ZSTD_dedicatedDictSearch_lazy_search(size_t *, size_t, U32, const ZSTD_matchState_t *const, const BYTE *const, const BYTE *const, const BYTE *const, const U32, const U32, const size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            *offsetPtr = curr
                .wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
            if ip.offset(currentMl_0 as isize) == iLimit {
                break;
            }
        }
        chainAttempt = chainAttempt.wrapping_add(1);
        chainIndex_0 = chainIndex_0.wrapping_add(1);
    }
    return ml;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_insertAndFindFirstIndex_internal(
    mut ms: *mut ZSTD_matchState_t,
    cParams: *const ZSTD_compressionParameters,
    mut ip: *const BYTE,
    mls: U32,
    lazySkipping: U32,
) -> U32 {
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let chainTable = (*ms).chainTable;
    let chainMask = (((1 as libc::c_int) << (*cParams).chainLog) - 1 as libc::c_int)
        as U32;
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as libc::c_long as U32;
    let mut idx = (*ms).nextToUpdate;
    while idx < target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const libc::c_void,
            hashLog,
            mls,
        );
        *chainTable.offset((idx & chainMask) as isize) = *hashTable.offset(h as isize);
        *hashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1);
        if lazySkipping != 0 {
            break;
        }
    }
    (*ms).nextToUpdate = target;
    return *hashTable
        .offset(ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_insertAndFindFirstIndex(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
) -> U32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    return ZSTD_insertAndFindFirstIndex_internal(
        ms,
        cParams,
        ip,
        (*ms).cParams.minMatch,
        0 as libc::c_int as U32,
    );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_HcFindBestMatch(
    mut ms: *mut ZSTD_matchState_t,
    ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
    mls: U32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let chainTable = (*ms).chainTable;
    let chainSize = ((1 as libc::c_int) << (*cParams).chainLog) as U32;
    let chainMask = chainSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictEnd = dictBase.offset(dictLimit as isize);
    let curr = ip.offset_from(base) as libc::c_long as U32;
    let maxDistance = (1 as libc::c_uint) << (*cParams).windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinMaxDistance = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as U32;
    let lowLimit = if isDictionary != 0 { lowestValid } else { withinMaxDistance };
    let minChain = if curr > chainSize {
        curr.wrapping_sub(chainSize)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let mut nbAttempts = (1 as libc::c_uint) << (*cParams).searchLog;
    let mut ml = (4 as libc::c_int - 1 as libc::c_int) as size_t;
    let dms = (*ms).dictMatchState;
    let ddsHashLog = if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        ((*dms).cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as libc::c_uint)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let ddsIdx = if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        ZSTD_hashPtr(ip as *const libc::c_void, ddsHashLog, mls)
            << ZSTD_LAZY_DDSS_BUCKET_LOG
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut matchIndex: U32 = 0;
    if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        let mut entry: *const U32 = &mut *((*dms).hashTable).offset(ddsIdx as isize)
            as *mut U32;
    }
    matchIndex = ZSTD_insertAndFindFirstIndex_internal(
        ms,
        cParams,
        ip,
        mls,
        (*ms).lazySkipping as U32,
    );
    while (matchIndex >= lowLimit) as libc::c_int
        & (nbAttempts > 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
    {
        let mut currentMl = 0 as libc::c_int as size_t;
        if dictMode as libc::c_uint != ZSTD_extDict as libc::c_int as libc::c_uint
            || matchIndex >= dictLimit
        {
            let match_0 = base.offset(matchIndex as isize);
            if matchIndex >= dictLimit {} else {
                __assert_fail(
                    b"matchIndex >= dictLimit\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    699 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            if MEM_read32(
                match_0.offset(ml as isize).offset(-(3 as libc::c_int as isize))
                    as *const libc::c_void,
            )
                == MEM_read32(
                    ip.offset(ml as isize).offset(-(3 as libc::c_int as isize))
                        as *const libc::c_void,
                )
            {
                currentMl = ZSTD_count(ip, match_0, iLimit);
            }
        } else {
            let match_1 = dictBase.offset(matchIndex as isize);
            if match_1.offset(4 as libc::c_int as isize) <= dictEnd {} else {
                __assert_fail(
                    b"match+4 <= dictEnd\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    705 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            if MEM_read32(match_1 as *const libc::c_void)
                == MEM_read32(ip as *const libc::c_void)
            {
                currentMl = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    match_1.offset(4 as libc::c_int as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
            }
        }
        if currentMl > ml {
            ml = currentMl;
            if curr.wrapping_sub(matchIndex) > 0 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"(curr - matchIndex)>0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    713 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            *offsetPtr = curr
                .wrapping_sub(matchIndex)
                .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                break;
            }
        }
        if matchIndex <= minChain {
            break;
        }
        matchIndex = *chainTable.offset((matchIndex & chainMask) as isize);
        nbAttempts = nbAttempts.wrapping_sub(1);
    }
    if nbAttempts
        <= (1 as libc::c_uint)
            << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            }) - 1 as libc::c_int
    {} else {
        __assert_fail(
            b"nbAttempts <= (1U << ZSTD_SEARCHLOG_MAX)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            721 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 131],
                &[libc::c_char; 131],
            >(
                b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
    }
    if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        ml = ZSTD_dedicatedDictSearch_lazy_search(
            offsetPtr,
            ml,
            nbAttempts,
            dms,
            ip,
            iLimit,
            prefixStart,
            curr,
            dictLimit,
            ddsIdx,
        );
    } else if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        let dmsChainTable: *const U32 = (*dms).chainTable;
        let dmsChainSize = ((1 as libc::c_int) << (*dms).cParams.chainLog) as U32;
        let dmsChainMask = dmsChainSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
        let dmsLowestIndex = (*dms).window.dictLimit;
        let dmsBase = (*dms).window.base;
        let dmsEnd = (*dms).window.nextSrc;
        let dmsSize = dmsEnd.offset_from(dmsBase) as libc::c_long as U32;
        let dmsIndexDelta = dictLimit.wrapping_sub(dmsSize);
        let dmsMinChain = if dmsSize > dmsChainSize {
            dmsSize.wrapping_sub(dmsChainSize)
        } else {
            0 as libc::c_int as libc::c_uint
        };
        matchIndex = *((*dms).hashTable)
            .offset(
                ZSTD_hashPtr(ip as *const libc::c_void, (*dms).cParams.hashLog, mls)
                    as isize,
            );
        while (matchIndex >= dmsLowestIndex) as libc::c_int
            & (nbAttempts > 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
        {
            let mut currentMl_0 = 0 as libc::c_int as size_t;
            let match_2 = dmsBase.offset(matchIndex as isize);
            if match_2.offset(4 as libc::c_int as isize) <= dmsEnd {} else {
                __assert_fail(
                    b"match+4 <= dmsEnd\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    741 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                    ))
                        .as_ptr(),
                );
            }
            if MEM_read32(match_2 as *const libc::c_void)
                == MEM_read32(ip as *const libc::c_void)
            {
                currentMl_0 = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    match_2.offset(4 as libc::c_int as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
            }
            if currentMl_0 > ml {
                ml = currentMl_0;
                if curr > matchIndex.wrapping_add(dmsIndexDelta) {} else {
                    __assert_fail(
                        b"curr > matchIndex + dmsIndexDelta\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        748 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 131],
                            &[libc::c_char; 131],
                        >(
                            b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if curr.wrapping_sub(matchIndex.wrapping_add(dmsIndexDelta))
                    > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"(curr - (matchIndex + dmsIndexDelta))>0\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        749 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 131],
                            &[libc::c_char; 131],
                        >(
                            b"size_t ZSTD_HcFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *offsetPtr = curr
                    .wrapping_sub(matchIndex.wrapping_add(dmsIndexDelta))
                    .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
                if ip.offset(currentMl_0 as isize) == iLimit {
                    break;
                }
            }
            if matchIndex <= dmsMinChain {
                break;
            }
            matchIndex = *dmsChainTable.offset((matchIndex & dmsChainMask) as isize);
            nbAttempts = nbAttempts.wrapping_sub(1);
        }
    }
    return ml;
}
pub const ZSTD_ROW_HASH_TAG_MASK: libc::c_uint = ((1 as libc::c_uint)
    << ZSTD_ROW_HASH_TAG_BITS)
    .wrapping_sub(1 as libc::c_int as libc::c_uint);
pub const ZSTD_ROW_HASH_CACHE_MASK: libc::c_int = ZSTD_ROW_HASH_CACHE_SIZE
    - 1 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_VecMask_next(mut val: ZSTD_VecMask) -> U32 {
    return ZSTD_countTrailingZeros64(val);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_nextIndex(tagRow: *mut BYTE, rowMask: U32) -> U32 {
    let mut next = (*tagRow as libc::c_int - 1 as libc::c_int) as libc::c_uint & rowMask;
    next = (next as libc::c_uint)
        .wrapping_add(
            if next == 0 as libc::c_int as libc::c_uint {
                rowMask
            } else {
                0 as libc::c_int as libc::c_uint
            },
        ) as U32 as U32;
    *tagRow = next as BYTE;
    return next;
}
#[inline]
unsafe extern "C" fn ZSTD_isAligned(
    mut ptr: *const libc::c_void,
    mut align: size_t,
) -> libc::c_int {
    if align & align.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(align & (align - 1)) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            796 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int ZSTD_isAligned(const void *, size_t)\0"))
                .as_ptr(),
        );
    }
    return (ptr as size_t & align.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_prefetch(
    mut hashTable: *const U32,
    mut tagTable: *const BYTE,
    relRow: U32,
    rowLog: U32,
) {
    rowLog >= 5 as libc::c_int as libc::c_uint;
    rowLog == 6 as libc::c_int as libc::c_uint;
    if rowLog == 4 as libc::c_int as libc::c_uint
        || rowLog == 5 as libc::c_int as libc::c_uint
        || rowLog == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"rowLog == 4 || rowLog == 5 || rowLog == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            813 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void ZSTD_row_prefetch(const U32 *, const BYTE *, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if ZSTD_isAligned(
        hashTable.offset(relRow as isize) as *const libc::c_void,
        64 as libc::c_int as size_t,
    ) != 0
    {} else {
        __assert_fail(
            b"ZSTD_isAligned(hashTable + relRow, 64)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            814 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void ZSTD_row_prefetch(const U32 *, const BYTE *, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if ZSTD_isAligned(
        tagTable.offset(relRow as isize) as *const libc::c_void,
        (1 as libc::c_int as size_t) << rowLog,
    ) != 0
    {} else {
        __assert_fail(
            b"ZSTD_isAligned(tagTable + relRow, (size_t)1 << rowLog)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            815 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void ZSTD_row_prefetch(const U32 *, const BYTE *, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_fillHashCache(
    mut ms: *mut ZSTD_matchState_t,
    mut base: *const BYTE,
    rowLog: U32,
    mls: U32,
    mut idx: U32,
    iLimit: *const BYTE,
) {
    let hashTable: *const U32 = (*ms).hashTable;
    let tagTable: *const BYTE = (*ms).tagTable;
    let hashLog = (*ms).rowHashLog;
    let maxElemsToPrefetch = if base.offset(idx as isize) > iLimit {
        0 as libc::c_int as libc::c_uint
    } else {
        (iLimit.offset_from(base.offset(idx as isize)) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as U32
    };
    let lim = idx
        .wrapping_add(
            (if (8 as libc::c_int as libc::c_uint) < maxElemsToPrefetch {
                8 as libc::c_int as libc::c_uint
            } else {
                maxElemsToPrefetch
            }),
        );
    while idx < lim {
        let hash = ZSTD_hashPtrSalted(
            base.offset(idx as isize) as *const libc::c_void,
            hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as libc::c_uint),
            mls,
            (*ms).hashSalt,
        ) as U32;
        let row = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        ZSTD_row_prefetch(hashTable, tagTable, row, rowLog);
        (*ms)
            .hashCache[(idx & ZSTD_ROW_HASH_CACHE_MASK as libc::c_uint) as usize] = hash;
        idx = idx.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_nextCachedHash(
    mut cache: *mut U32,
    mut hashTable: *const U32,
    mut tagTable: *const BYTE,
    mut base: *const BYTE,
    mut idx: U32,
    hashLog: U32,
    rowLog: U32,
    mls: U32,
    hashSalt: U64,
) -> U32 {
    let newHash = ZSTD_hashPtrSalted(
        base.offset(idx as isize).offset(ZSTD_ROW_HASH_CACHE_SIZE as isize)
            as *const libc::c_void,
        hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as libc::c_uint),
        mls,
        hashSalt,
    ) as U32;
    let row = newHash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
    ZSTD_row_prefetch(hashTable, tagTable, row, rowLog);
    let hash = *cache.offset((idx & ZSTD_ROW_HASH_CACHE_MASK as libc::c_uint) as isize);
    *cache.offset((idx & ZSTD_ROW_HASH_CACHE_MASK as libc::c_uint) as isize) = newHash;
    return hash;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_update_internalImpl(
    mut ms: *mut ZSTD_matchState_t,
    mut updateStartIdx: U32,
    updateEndIdx: U32,
    mls: U32,
    rowLog: U32,
    rowMask: U32,
    useCache: U32,
) {
    let hashTable = (*ms).hashTable;
    let tagTable = (*ms).tagTable;
    let hashLog = (*ms).rowHashLog;
    let base = (*ms).window.base;
    while updateStartIdx < updateEndIdx {
        let hash = if useCache != 0 {
            ZSTD_row_nextCachedHash(
                ((*ms).hashCache).as_mut_ptr(),
                hashTable,
                tagTable,
                base,
                updateStartIdx,
                hashLog,
                rowLog,
                mls,
                (*ms).hashSalt,
            )
        } else {
            ZSTD_hashPtrSalted(
                base.offset(updateStartIdx as isize) as *const libc::c_void,
                hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as libc::c_uint),
                mls,
                (*ms).hashSalt,
            ) as U32
        };
        let relRow = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        let row = hashTable.offset(relRow as isize);
        let mut tagRow = tagTable.offset(relRow as isize);
        let pos = ZSTD_row_nextIndex(tagRow, rowMask);
        if hash as libc::c_ulong
            == ZSTD_hashPtrSalted(
                base.offset(updateStartIdx as isize) as *const libc::c_void,
                hashLog.wrapping_add(8 as libc::c_int as libc::c_uint),
                mls,
                (*ms).hashSalt,
            )
        {} else {
            __assert_fail(
                b"hash == ZSTD_hashPtrSalted(base + updateStartIdx, hashLog + ZSTD_ROW_HASH_TAG_BITS, mls, ms->hashSalt)\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                885 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"void ZSTD_row_update_internalImpl(ZSTD_matchState_t *, U32, const U32, const U32, const U32, const U32, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        *tagRow.offset(pos as isize) = (hash & ZSTD_ROW_HASH_TAG_MASK) as BYTE;
        *row.offset(pos as isize) = updateStartIdx;
        updateStartIdx = updateStartIdx.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_update_internal(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    mls: U32,
    rowLog: U32,
    rowMask: U32,
    useCache: U32,
) {
    let mut idx = (*ms).nextToUpdate;
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as libc::c_long as U32;
    let kSkipThreshold = 384 as libc::c_int as U32;
    let kMaxMatchStartPositionsToUpdate = 96 as libc::c_int as U32;
    let kMaxMatchEndPositionsToUpdate = 32 as libc::c_int as U32;
    if useCache != 0 {
        if (target.wrapping_sub(idx) > kSkipThreshold) as libc::c_int as libc::c_long
            != 0
        {
            let bound = idx.wrapping_add(kMaxMatchStartPositionsToUpdate);
            ZSTD_row_update_internalImpl(ms, idx, bound, mls, rowLog, rowMask, useCache);
            idx = target.wrapping_sub(kMaxMatchEndPositionsToUpdate);
            ZSTD_row_fillHashCache(
                ms,
                base,
                rowLog,
                mls,
                idx,
                ip.offset(1 as libc::c_int as isize),
            );
        }
    }
    if target >= idx {} else {
        __assert_fail(
            b"target >= idx\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            919 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"void ZSTD_row_update_internal(ZSTD_matchState_t *, const BYTE *, const U32, const U32, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_row_update_internalImpl(ms, idx, target, mls, rowLog, rowMask, useCache);
    (*ms).nextToUpdate = target;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_row_update(
    ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
) {
    let rowLog = if 4 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.searchLog < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.searchLog
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        4 as libc::c_int as libc::c_uint
    } else if (*ms).cParams.searchLog < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.searchLog
    } else {
        6 as libc::c_int as libc::c_uint
    };
    let rowMask = ((1 as libc::c_uint) << rowLog)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mls = if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as libc::c_int as libc::c_uint
    };
    ZSTD_row_update_internal(ms, ip, mls, rowLog, rowMask, 0 as libc::c_int as U32);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_matchMaskGroupWidth(rowEntries: U32) -> U32 {
    if rowEntries == 16 as libc::c_int as libc::c_uint
        || rowEntries == 32 as libc::c_int as libc::c_uint
        || rowEntries == 64 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"(rowEntries == 16) || (rowEntries == 32) || rowEntries == 64\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            944 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"U32 ZSTD_row_matchMaskGroupWidth(const U32)\0"))
                .as_ptr(),
        );
    }
    if rowEntries <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"rowEntries <= ZSTD_ROW_HASH_MAX_ENTRIES\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            945 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"U32 ZSTD_row_matchMaskGroupWidth(const U32)\0"))
                .as_ptr(),
        );
    }
    return 1 as libc::c_int as U32;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_getSSEMask(
    mut nbChunks: libc::c_int,
    src: *const BYTE,
    tag: BYTE,
    head: U32,
) -> ZSTD_VecMask {
    let comparisonMask = _mm_set1_epi8(tag as libc::c_char);
    let mut matches: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
    let mut i: libc::c_int = 0;
    if nbChunks == 1 as libc::c_int || nbChunks == 2 as libc::c_int
        || nbChunks == 4 as libc::c_int
    {} else {
        __assert_fail(
            b"nbChunks == 1 || nbChunks == 2 || nbChunks == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            972 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"ZSTD_VecMask ZSTD_row_getSSEMask(int, const BYTE *const, const BYTE, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < nbChunks {
        let chunk = _mm_loadu_si128(
            src.offset((16 as libc::c_int * i) as isize) as *const libc::c_void
                as *const __m128i,
        );
        let equalMask = _mm_cmpeq_epi8(chunk, comparisonMask);
        matches[i as usize] = _mm_movemask_epi8(equalMask);
        i += 1;
    }
    if nbChunks == 1 as libc::c_int {
        return ZSTD_rotateRight_U16(matches[0 as libc::c_int as usize] as U16, head)
            as ZSTD_VecMask;
    }
    if nbChunks == 2 as libc::c_int {
        return ZSTD_rotateRight_U32(
            (matches[1 as libc::c_int as usize] as U32) << 16 as libc::c_int
                | matches[0 as libc::c_int as usize] as U32,
            head,
        ) as ZSTD_VecMask;
    }
    if nbChunks == 4 as libc::c_int {} else {
        __assert_fail(
            b"nbChunks == 4\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            980 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"ZSTD_VecMask ZSTD_row_getSSEMask(int, const BYTE *const, const BYTE, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_rotateRight_U64(
        (matches[3 as libc::c_int as usize] as U64) << 48 as libc::c_int
            | (matches[2 as libc::c_int as usize] as U64) << 32 as libc::c_int
            | (matches[1 as libc::c_int as usize] as U64) << 16 as libc::c_int
            | matches[0 as libc::c_int as usize] as U64,
        head,
    );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_getMatchMask(
    tagRow: *const BYTE,
    tag: BYTE,
    headGrouped: U32,
    rowEntries: U32,
) -> ZSTD_VecMask {
    let src = tagRow;
    if rowEntries == 16 as libc::c_int as libc::c_uint
        || rowEntries == 32 as libc::c_int as libc::c_uint
        || rowEntries == 64 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"(rowEntries == 16) || (rowEntries == 32) || rowEntries == 64\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1043 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"ZSTD_VecMask ZSTD_row_getMatchMask(const BYTE *const, const BYTE, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if rowEntries <= 64 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"rowEntries <= ZSTD_ROW_HASH_MAX_ENTRIES\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1044 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"ZSTD_VecMask ZSTD_row_getMatchMask(const BYTE *const, const BYTE, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if (ZSTD_row_matchMaskGroupWidth(rowEntries)).wrapping_mul(rowEntries)
        as libc::c_ulong
        <= (::core::mem::size_of::<ZSTD_VecMask>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"ZSTD_row_matchMaskGroupWidth(rowEntries) * rowEntries <= sizeof(ZSTD_VecMask) * 8\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1045 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"ZSTD_VecMask ZSTD_row_getMatchMask(const BYTE *const, const BYTE, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_row_getSSEMask(
        rowEntries.wrapping_div(16 as libc::c_int as libc::c_uint) as libc::c_int,
        src,
        tag,
        headGrouped,
    );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_RowFindBestMatch(
    mut ms: *mut ZSTD_matchState_t,
    ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
    mls: U32,
    dictMode: ZSTD_dictMode_e,
    rowLog: U32,
) -> size_t {
    let hashTable = (*ms).hashTable;
    let tagTable = (*ms).tagTable;
    let hashCache = ((*ms).hashCache).as_mut_ptr();
    let hashLog = (*ms).rowHashLog;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictEnd = dictBase.offset(dictLimit as isize);
    let curr = ip.offset_from(base) as libc::c_long as U32;
    let maxDistance = (1 as libc::c_uint) << (*cParams).windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinMaxDistance = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as U32;
    let lowLimit = if isDictionary != 0 { lowestValid } else { withinMaxDistance };
    let rowEntries = (1 as libc::c_uint) << rowLog;
    let rowMask = rowEntries.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let cappedSearchLog = if (*cParams).searchLog < rowLog {
        (*cParams).searchLog
    } else {
        rowLog
    };
    let groupWidth = ZSTD_row_matchMaskGroupWidth(rowEntries);
    let hashSalt = (*ms).hashSalt;
    let mut nbAttempts = (1 as libc::c_uint) << cappedSearchLog;
    let mut ml = (4 as libc::c_int - 1 as libc::c_int) as size_t;
    let mut hash: U32 = 0;
    let dms = (*ms).dictMatchState;
    let mut ddsIdx = 0 as libc::c_int as size_t;
    let mut ddsExtraAttempts = 0 as libc::c_int as U32;
    let mut dmsTag = 0 as libc::c_int as U32;
    let mut dmsRow = NULL as *mut U32;
    let mut dmsTagRow = NULL as *mut BYTE;
    if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        let ddsHashLog = ((*dms).cParams.hashLog)
            .wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as libc::c_uint);
        ddsIdx = ZSTD_hashPtr(ip as *const libc::c_void, ddsHashLog, mls)
            << ZSTD_LAZY_DDSS_BUCKET_LOG;
        ddsExtraAttempts = if (*cParams).searchLog > rowLog {
            (1 as libc::c_uint) << ((*cParams).searchLog).wrapping_sub(rowLog)
        } else {
            0 as libc::c_int as libc::c_uint
        };
    }
    if dictMode as libc::c_uint == ZSTD_dictMatchState as libc::c_int as libc::c_uint {
        let dmsHashTable = (*dms).hashTable;
        let dmsTagTable = (*dms).tagTable;
        let dmsHash = ZSTD_hashPtr(
            ip as *const libc::c_void,
            ((*dms).rowHashLog).wrapping_add(ZSTD_ROW_HASH_TAG_BITS as libc::c_uint),
            mls,
        ) as U32;
        let dmsRelRow = dmsHash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        dmsTag = dmsHash & ZSTD_ROW_HASH_TAG_MASK;
        dmsTagRow = dmsTagTable.offset(dmsRelRow as isize);
        dmsRow = dmsHashTable.offset(dmsRelRow as isize);
        ZSTD_row_prefetch(dmsHashTable, dmsTagTable, dmsRelRow, rowLog);
    }
    if (*ms).lazySkipping == 0 {
        ZSTD_row_update_internal(ms, ip, mls, rowLog, rowMask, 1 as libc::c_int as U32);
        hash = ZSTD_row_nextCachedHash(
            hashCache,
            hashTable,
            tagTable,
            base,
            curr,
            hashLog,
            rowLog,
            mls,
            hashSalt,
        );
    } else {
        hash = ZSTD_hashPtrSalted(
            ip as *const libc::c_void,
            hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as libc::c_uint),
            mls,
            hashSalt,
        ) as U32;
        (*ms).nextToUpdate = curr;
    }
    (*ms)
        .hashSaltEntropy = ((*ms).hashSaltEntropy as libc::c_uint).wrapping_add(hash)
        as U32 as U32;
    let relRow = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
    let tag = hash & ZSTD_ROW_HASH_TAG_MASK;
    let row = hashTable.offset(relRow as isize);
    let mut tagRow = tagTable.offset(relRow as isize);
    let headGrouped = (*tagRow as libc::c_uint & rowMask).wrapping_mul(groupWidth);
    let mut matchBuffer: [U32; 64] = [0; 64];
    let mut numMatches = 0 as libc::c_int as size_t;
    let mut currMatch = 0 as libc::c_int as size_t;
    let mut matches = ZSTD_row_getMatchMask(
        tagRow,
        tag as BYTE,
        headGrouped,
        rowEntries,
    );
    while matches > 0 as libc::c_int as libc::c_ulong
        && nbAttempts > 0 as libc::c_int as libc::c_uint
    {
        let matchPos = headGrouped
            .wrapping_add(ZSTD_VecMask_next(matches))
            .wrapping_div(groupWidth) & rowMask;
        let matchIndex = *row.offset(matchPos as isize);
        if !(matchPos == 0 as libc::c_int as libc::c_uint) {
            if numMatches < rowEntries as libc::c_ulong {} else {
                __assert_fail(
                    b"numMatches < rowEntries\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1211 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if matchIndex < lowLimit {
                break;
            }
            dictMode as libc::c_uint != ZSTD_extDict as libc::c_int as libc::c_uint
                || matchIndex >= dictLimit;
            let fresh3 = numMatches;
            numMatches = numMatches.wrapping_add(1);
            matchBuffer[fresh3 as usize] = matchIndex;
            nbAttempts = nbAttempts.wrapping_sub(1);
        }
        matches &= matches.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    let pos = ZSTD_row_nextIndex(tagRow, rowMask);
    *tagRow.offset(pos as isize) = tag as BYTE;
    let fresh4 = (*ms).nextToUpdate;
    (*ms).nextToUpdate = ((*ms).nextToUpdate).wrapping_add(1);
    *row.offset(pos as isize) = fresh4;
    while currMatch < numMatches {
        let matchIndex_0 = matchBuffer[currMatch as usize];
        let mut currentMl = 0 as libc::c_int as size_t;
        if matchIndex_0 < curr {} else {
            __assert_fail(
                b"matchIndex < curr\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1235 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 143],
                    &[libc::c_char; 143],
                >(
                    b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        if matchIndex_0 >= lowLimit {} else {
            __assert_fail(
                b"matchIndex >= lowLimit\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1236 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 143],
                    &[libc::c_char; 143],
                >(
                    b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        if dictMode as libc::c_uint != ZSTD_extDict as libc::c_int as libc::c_uint
            || matchIndex_0 >= dictLimit
        {
            let match_0 = base.offset(matchIndex_0 as isize);
            if matchIndex_0 >= dictLimit {} else {
                __assert_fail(
                    b"matchIndex >= dictLimit\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1240 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if MEM_read32(
                match_0.offset(ml as isize).offset(-(3 as libc::c_int as isize))
                    as *const libc::c_void,
            )
                == MEM_read32(
                    ip.offset(ml as isize).offset(-(3 as libc::c_int as isize))
                        as *const libc::c_void,
                )
            {
                currentMl = ZSTD_count(ip, match_0, iLimit);
            }
        } else {
            let match_1 = dictBase.offset(matchIndex_0 as isize);
            if match_1.offset(4 as libc::c_int as isize) <= dictEnd {} else {
                __assert_fail(
                    b"match+4 <= dictEnd\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1246 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if MEM_read32(match_1 as *const libc::c_void)
                == MEM_read32(ip as *const libc::c_void)
            {
                currentMl = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    match_1.offset(4 as libc::c_int as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
            }
        }
        if currentMl > ml {
            ml = currentMl;
            if curr.wrapping_sub(matchIndex_0) > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"(curr - matchIndex)>0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1254 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            *offsetPtr = curr
                .wrapping_sub(matchIndex_0)
                .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                break;
            }
        }
        currMatch = currMatch.wrapping_add(1);
    }
    if nbAttempts
        <= (1 as libc::c_uint)
            << (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            }) - 1 as libc::c_int
    {} else {
        __assert_fail(
            b"nbAttempts <= (1U << ZSTD_SEARCHLOG_MAX)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1260 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 143],
                &[libc::c_char; 143],
            >(
                b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        ml = ZSTD_dedicatedDictSearch_lazy_search(
            offsetPtr,
            ml,
            nbAttempts.wrapping_add(ddsExtraAttempts),
            dms,
            ip,
            iLimit,
            prefixStart,
            curr,
            dictLimit,
            ddsIdx,
        );
    } else if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        let dmsLowestIndex = (*dms).window.dictLimit;
        let dmsBase = (*dms).window.base;
        let dmsEnd = (*dms).window.nextSrc;
        let dmsSize = dmsEnd.offset_from(dmsBase) as libc::c_long as U32;
        let dmsIndexDelta = dictLimit.wrapping_sub(dmsSize);
        let headGrouped_0 = (*dmsTagRow as libc::c_uint & rowMask)
            .wrapping_mul(groupWidth);
        let mut matchBuffer_0: [U32; 64] = [0; 64];
        let mut numMatches_0 = 0 as libc::c_int as size_t;
        let mut currMatch_0 = 0 as libc::c_int as size_t;
        let mut matches_0 = ZSTD_row_getMatchMask(
            dmsTagRow,
            dmsTag as BYTE,
            headGrouped_0,
            rowEntries,
        );
        while matches_0 > 0 as libc::c_int as libc::c_ulong
            && nbAttempts > 0 as libc::c_int as libc::c_uint
        {
            let matchPos_0 = headGrouped_0
                .wrapping_add(ZSTD_VecMask_next(matches_0))
                .wrapping_div(groupWidth) & rowMask;
            let matchIndex_1 = *dmsRow.offset(matchPos_0 as isize);
            if !(matchPos_0 == 0 as libc::c_int as libc::c_uint) {
                if matchIndex_1 < dmsLowestIndex {
                    break;
                }
                let fresh5 = numMatches_0;
                numMatches_0 = numMatches_0.wrapping_add(1);
                matchBuffer_0[fresh5 as usize] = matchIndex_1;
                nbAttempts = nbAttempts.wrapping_sub(1);
            }
            matches_0 &= matches_0.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        while currMatch_0 < numMatches_0 {
            let matchIndex_2 = matchBuffer_0[currMatch_0 as usize];
            let mut currentMl_0 = 0 as libc::c_int as size_t;
            if matchIndex_2 >= dmsLowestIndex {} else {
                __assert_fail(
                    b"matchIndex >= dmsLowestIndex\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1293 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if matchIndex_2 < curr {} else {
                __assert_fail(
                    b"matchIndex < curr\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1294 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            let match_2 = dmsBase.offset(matchIndex_2 as isize);
            if match_2.offset(4 as libc::c_int as isize) <= dmsEnd {} else {
                __assert_fail(
                    b"match+4 <= dmsEnd\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    1297 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if MEM_read32(match_2 as *const libc::c_void)
                == MEM_read32(ip as *const libc::c_void)
            {
                currentMl_0 = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    match_2.offset(4 as libc::c_int as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
            }
            if currentMl_0 > ml {
                ml = currentMl_0;
                if curr > matchIndex_2.wrapping_add(dmsIndexDelta) {} else {
                    __assert_fail(
                        b"curr > matchIndex + dmsIndexDelta\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        1304 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 143],
                            &[libc::c_char; 143],
                        >(
                            b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if curr.wrapping_sub(matchIndex_2.wrapping_add(dmsIndexDelta))
                    > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"(curr - (matchIndex + dmsIndexDelta))>0\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        1305 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 143],
                            &[libc::c_char; 143],
                        >(
                            b"size_t ZSTD_RowFindBestMatch(ZSTD_matchState_t *, const BYTE *const, const BYTE *const, size_t *, const U32, const ZSTD_dictMode_e, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *offsetPtr = curr
                    .wrapping_sub(matchIndex_2.wrapping_add(dmsIndexDelta))
                    .wrapping_add(ZSTD_REP_NUM as libc::c_uint) as size_t;
                if ip.offset(currentMl_0 as isize) == iLimit {
                    break;
                }
            }
            currMatch_0 = currMatch_0.wrapping_add(1);
        }
    }
    return ml;
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_4_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_noDict,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_6_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_extDict,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_6_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_extDict,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_6_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_extDict,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_5_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_extDict,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_5_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_extDict,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_5_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_extDict,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_4_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_extDict,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_4_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_extDict,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_4_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t ZSTD_RowFindBestMatch_extDict_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_extDict,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_6_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dictMatchState,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_6_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dictMatchState,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_6_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dictMatchState,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_6_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_noDict,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_6_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_noDict,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_6_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_noDict,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_5_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_noDict,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_5_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_noDict,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_5_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_noDict,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_4_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_noDict,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_5_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_5_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dictMatchState,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_4_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t ZSTD_RowFindBestMatch_noDict_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_noDict,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_5_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dictMatchState,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_5_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dictMatchState,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_4_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_4_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dictMatchState,
        6 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_4_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_4_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dictMatchState,
        5 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_4_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.searchLog
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.searchLog)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"size_t ZSTD_RowFindBestMatch_dictMatchState_4_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dictMatchState,
        4 as libc::c_int as U32,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_BtFindBestMatch_dictMatchState_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dedicatedDictSearch_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"size_t ZSTD_BtFindBestMatch_dedicatedDictSearch_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_BtFindBestMatch_dictMatchState_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_noDict_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t ZSTD_BtFindBestMatch_noDict_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_noDict_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t ZSTD_BtFindBestMatch_noDict_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_noDict_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t ZSTD_BtFindBestMatch_noDict_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_BtFindBestMatch_dictMatchState_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"size_t ZSTD_BtFindBestMatch_extDict_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as libc::c_int as U32,
        ZSTD_extDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"size_t ZSTD_BtFindBestMatch_extDict_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as libc::c_int as U32,
        ZSTD_extDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"size_t ZSTD_BtFindBestMatch_extDict_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as libc::c_int as U32,
        ZSTD_extDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dedicatedDictSearch_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"size_t ZSTD_BtFindBestMatch_dedicatedDictSearch_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dedicatedDictSearch_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offBasePtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"size_t ZSTD_BtFindBestMatch_dedicatedDictSearch_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_HcFindBestMatch_dictMatchState_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"size_t ZSTD_HcFindBestMatch_extDict_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_extDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dedicatedDictSearch_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"size_t ZSTD_HcFindBestMatch_dedicatedDictSearch_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dedicatedDictSearch_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"size_t ZSTD_HcFindBestMatch_dedicatedDictSearch_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dedicatedDictSearch_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"size_t ZSTD_HcFindBestMatch_dedicatedDictSearch_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_HcFindBestMatch_dictMatchState_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"size_t ZSTD_HcFindBestMatch_dictMatchState_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"size_t ZSTD_HcFindBestMatch_extDict_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_extDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_noDict_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t ZSTD_HcFindBestMatch_noDict_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_noDict_5(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 5 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 5\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t ZSTD_HcFindBestMatch_noDict_5(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_noDict_4(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 4\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t ZSTD_HcFindBestMatch_noDict_4(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_6(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    iLimit: *const BYTE,
    mut offsetPtr: *mut size_t,
) -> size_t {
    if (if 4 as libc::c_int as libc::c_uint
        > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    {
        4 as libc::c_int as libc::c_uint
    } else {
        (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
            6 as libc::c_int as libc::c_uint
        } else {
            (*ms).cParams.minMatch
        })
    }) == 6 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"MAX(4, MIN(6, ms->cParams.minMatch)) == 6\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1400 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"size_t ZSTD_HcFindBestMatch_extDict_6(ZSTD_matchState_t *, const BYTE *, const BYTE *const, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as libc::c_int as U32,
        ZSTD_extDict,
    );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_searchMax(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const BYTE,
    mut iend: *const BYTE,
    mut offsetPtr: *mut size_t,
    mls: U32,
    rowLog: U32,
    searchMethod: searchMethod_e,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint {
        match searchMethod as libc::c_uint {
            0 => {
                match mls {
                    4 => return ZSTD_HcFindBestMatch_noDict_4(ms, ip, iend, offsetPtr),
                    5 => return ZSTD_HcFindBestMatch_noDict_5(ms, ip, iend, offsetPtr),
                    6 => return ZSTD_HcFindBestMatch_noDict_6(ms, ip, iend, offsetPtr),
                    _ => {}
                }
            }
            1 => {
                match mls {
                    4 => return ZSTD_BtFindBestMatch_noDict_4(ms, ip, iend, offsetPtr),
                    5 => return ZSTD_BtFindBestMatch_noDict_5(ms, ip, iend, offsetPtr),
                    6 => return ZSTD_BtFindBestMatch_noDict_6(ms, ip, iend, offsetPtr),
                    _ => {}
                }
            }
            2 => {
                match mls {
                    4 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_noDict_4_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_noDict_4_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_noDict_4_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1476 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    5 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_noDict_5_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_noDict_5_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_noDict_5_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1476 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    6 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_noDict_6_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_noDict_6_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_noDict_6_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1476 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1476 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
        unreachable!();
    } else if dictMode as libc::c_uint == ZSTD_extDict as libc::c_int as libc::c_uint {
        match searchMethod as libc::c_uint {
            0 => {
                match mls {
                    4 => return ZSTD_HcFindBestMatch_extDict_4(ms, ip, iend, offsetPtr),
                    5 => return ZSTD_HcFindBestMatch_extDict_5(ms, ip, iend, offsetPtr),
                    6 => return ZSTD_HcFindBestMatch_extDict_6(ms, ip, iend, offsetPtr),
                    _ => {}
                }
            }
            1 => {
                match mls {
                    4 => return ZSTD_BtFindBestMatch_extDict_4(ms, ip, iend, offsetPtr),
                    5 => return ZSTD_BtFindBestMatch_extDict_5(ms, ip, iend, offsetPtr),
                    6 => return ZSTD_BtFindBestMatch_extDict_6(ms, ip, iend, offsetPtr),
                    _ => {}
                }
            }
            2 => {
                match mls {
                    4 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_extDict_4_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_extDict_4_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_extDict_4_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1478 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    5 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_extDict_5_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_extDict_5_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_extDict_5_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1478 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    6 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_extDict_6_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_extDict_6_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_extDict_6_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1478 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1478 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
        unreachable!();
    } else if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        match searchMethod as libc::c_uint {
            0 => {
                match mls {
                    4 => {
                        return ZSTD_HcFindBestMatch_dictMatchState_4(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    5 => {
                        return ZSTD_HcFindBestMatch_dictMatchState_5(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    6 => {
                        return ZSTD_HcFindBestMatch_dictMatchState_6(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    _ => {}
                }
            }
            1 => {
                match mls {
                    4 => {
                        return ZSTD_BtFindBestMatch_dictMatchState_4(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    5 => {
                        return ZSTD_BtFindBestMatch_dictMatchState_5(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    6 => {
                        return ZSTD_BtFindBestMatch_dictMatchState_6(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    _ => {}
                }
            }
            2 => {
                match mls {
                    4 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_4_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_4_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_4_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1480 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    5 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_5_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_5_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_5_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1480 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    6 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_6_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_6_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_dictMatchState_6_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1480 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1480 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
        unreachable!();
    } else if dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint
    {
        match searchMethod as libc::c_uint {
            0 => {
                match mls {
                    4 => {
                        return ZSTD_HcFindBestMatch_dedicatedDictSearch_4(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    5 => {
                        return ZSTD_HcFindBestMatch_dedicatedDictSearch_5(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    6 => {
                        return ZSTD_HcFindBestMatch_dedicatedDictSearch_6(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    _ => {}
                }
            }
            1 => {
                match mls {
                    4 => {
                        return ZSTD_BtFindBestMatch_dedicatedDictSearch_4(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    5 => {
                        return ZSTD_BtFindBestMatch_dedicatedDictSearch_5(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    6 => {
                        return ZSTD_BtFindBestMatch_dedicatedDictSearch_6(
                            ms,
                            ip,
                            iend,
                            offsetPtr,
                        );
                    }
                    _ => {}
                }
            }
            2 => {
                match mls {
                    4 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1482 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    5 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1482 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    6 => {
                        match rowLog {
                            4 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            5 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            6 => {
                                return ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(
                                    ms,
                                    ip,
                                    iend,
                                    offsetPtr,
                                );
                            }
                            _ => {}
                        }
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                as *const u8 as *const libc::c_char,
                            1482 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 148],
                                &[libc::c_char; 148],
                            >(
                                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
                            ))
                                .as_ptr(),
                        );
                        unreachable!();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                as *const libc::c_char,
            1482 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
            ))
                .as_ptr(),
        );
        unreachable!();
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
            as *const libc::c_char,
        1484 as libc::c_int as libc::c_uint,
        (*::core::mem::transmute::<
            &[u8; 148],
            &[libc::c_char; 148],
        >(
            b"size_t ZSTD_searchMax(ZSTD_matchState_t *, const BYTE *, const BYTE *, size_t *, const U32, const U32, const searchMethod_e, const ZSTD_dictMode_e)\0",
        ))
            .as_ptr(),
    );
    unreachable!();
    return 0 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_lazy_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    searchMethod: searchMethod_e,
    depth: U32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let mut current_block: u64;
    let istart = src as *const BYTE;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = if searchMethod as libc::c_uint
        == search_rowHash as libc::c_int as libc::c_uint
    {
        iend.offset(-(8 as libc::c_int as isize))
            .offset(-(ZSTD_ROW_HASH_CACHE_SIZE as isize))
    } else {
        iend.offset(-(8 as libc::c_int as isize))
    };
    let base = (*ms).window.base;
    let prefixLowestIndex = (*ms).window.dictLimit;
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let mls = if 4 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        4 as libc::c_int as libc::c_uint
    } else if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as libc::c_int as libc::c_uint
    };
    let rowLog = if 4 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.searchLog < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.searchLog
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        4 as libc::c_int as libc::c_uint
    } else if (*ms).cParams.searchLog < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.searchLog
    } else {
        6 as libc::c_int as libc::c_uint
    };
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    let mut offsetSaved1 = 0 as libc::c_int as U32;
    let mut offsetSaved2 = 0 as libc::c_int as U32;
    let isDMS = (dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint) as libc::c_int;
    let isDDS = (dictMode as libc::c_uint
        == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint) as libc::c_int;
    let isDxS = (isDMS != 0 || isDDS != 0) as libc::c_int;
    let dms = (*ms).dictMatchState;
    let dictLowestIndex = if isDxS != 0 {
        (*dms).window.dictLimit
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let dictBase = if isDxS != 0 { (*dms).window.base } else { NULL as *const BYTE };
    let dictLowest = if isDxS != 0 {
        dictBase.offset(dictLowestIndex as isize)
    } else {
        NULL as *const BYTE
    };
    let dictEnd = if isDxS != 0 { (*dms).window.nextSrc } else { NULL as *const BYTE };
    let dictIndexDelta = if isDxS != 0 {
        prefixLowestIndex
            .wrapping_sub(dictEnd.offset_from(dictBase) as libc::c_long as U32)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let dictAndPrefixLength = (ip.offset_from(prefixLowest) as libc::c_long
        + dictEnd.offset_from(dictLowest) as libc::c_long) as U32;
    ip = ip
        .offset(
            (dictAndPrefixLength == 0 as libc::c_int as libc::c_uint) as libc::c_int
                as isize,
        );
    if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint {
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let windowLow = ZSTD_getLowestPrefixIndex(ms, curr, (*ms).cParams.windowLog);
        let maxRep = curr.wrapping_sub(windowLow);
        if offset_2 > maxRep {
            offsetSaved2 = offset_2;
            offset_2 = 0 as libc::c_int as U32;
        }
        if offset_1 > maxRep {
            offsetSaved1 = offset_1;
            offset_1 = 0 as libc::c_int as U32;
        }
    }
    if isDxS != 0 {
        if offset_1 <= dictAndPrefixLength {} else {
            __assert_fail(
                b"offset_1 <= dictAndPrefixLength\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1539 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 159],
                    &[libc::c_char; 159],
                >(
                    b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if offset_2 <= dictAndPrefixLength {} else {
            __assert_fail(
                b"offset_2 <= dictAndPrefixLength\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1540 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 159],
                    &[libc::c_char; 159],
                >(
                    b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    (*ms).lazySkipping = 0 as libc::c_int;
    if searchMethod as libc::c_uint == search_rowHash as libc::c_int as libc::c_uint {
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, (*ms).nextToUpdate, ilimit);
    }
    asm!(".p2align 5", options(preserves_flags, att_syntax));
    while ip < ilimit {
        let mut matchLength = 0 as libc::c_int as size_t;
        if 1 as libc::c_int >= 1 as libc::c_int {} else {
            __assert_fail(
                b"(1)>=1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1559 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 159],
                    &[libc::c_char; 159],
                >(
                    b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if 1 as libc::c_int <= 3 as libc::c_int {} else {
            __assert_fail(
                b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1559 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 159],
                    &[libc::c_char; 159],
                >(
                    b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                ))
                    .as_ptr(),
            );
        }
        let mut offBase = 1 as libc::c_int as size_t;
        let mut start = ip.offset(1 as libc::c_int as isize);
        if isDxS != 0 {
            let repIndex = (ip.offset_from(base) as libc::c_long as U32)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(offset_1);
            let mut repMatch = if (dictMode as libc::c_uint
                == ZSTD_dictMatchState as libc::c_int as libc::c_uint
                || dictMode as libc::c_uint
                    == ZSTD_dedicatedDictSearch as libc::c_int as libc::c_uint)
                && repIndex < prefixLowestIndex
            {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
            } else {
                base.offset(repIndex as isize)
            };
            if prefixLowestIndex
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(repIndex) >= 3 as libc::c_int as libc::c_uint
                && MEM_read32(repMatch as *const libc::c_void)
                    == MEM_read32(
                        ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    )
            {
                let mut repMatchEnd = if repIndex < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                matchLength = (ZSTD_count_2segments(
                    ip
                        .offset(1 as libc::c_int as isize)
                        .offset(4 as libc::c_int as isize),
                    repMatch.offset(4 as libc::c_int as isize),
                    iend,
                    repMatchEnd,
                    prefixLowest,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                if depth == 0 as libc::c_int as libc::c_uint {
                    current_block = 647665664658786213;
                } else {
                    current_block = 3275366147856559585;
                }
            } else {
                current_block = 3275366147856559585;
            }
        } else {
            current_block = 3275366147856559585;
        }
        match current_block {
            3275366147856559585 => {
                if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint
                    && (offset_1 > 0 as libc::c_int as libc::c_uint) as libc::c_int
                        & (MEM_read32(
                            ip
                                .offset(1 as libc::c_int as isize)
                                .offset(-(offset_1 as isize)) as *const libc::c_void,
                        )
                            == MEM_read32(
                                ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            )) as libc::c_int != 0
                {
                    matchLength = (ZSTD_count(
                        ip
                            .offset(1 as libc::c_int as isize)
                            .offset(4 as libc::c_int as isize),
                        ip
                            .offset(1 as libc::c_int as isize)
                            .offset(4 as libc::c_int as isize)
                            .offset(-(offset_1 as isize)),
                        iend,
                    ))
                        .wrapping_add(4 as libc::c_int as libc::c_ulong);
                    if depth == 0 as libc::c_int as libc::c_uint {
                        current_block = 647665664658786213;
                    } else {
                        current_block = 7245201122033322888;
                    }
                } else {
                    current_block = 7245201122033322888;
                }
                match current_block {
                    647665664658786213 => {}
                    _ => {
                        let mut offbaseFound = 999999999 as libc::c_int as size_t;
                        let ml2 = ZSTD_searchMax(
                            ms,
                            ip,
                            iend,
                            &mut offbaseFound,
                            mls,
                            rowLog,
                            searchMethod,
                            dictMode,
                        );
                        if ml2 > matchLength {
                            matchLength = ml2;
                            start = ip;
                            offBase = offbaseFound;
                        }
                        if matchLength < 4 as libc::c_int as libc::c_ulong {
                            let step = (ip.offset_from(anchor) as libc::c_long as size_t
                                >> kSearchStrength)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            ip = ip.offset(step as isize);
                            (*ms)
                                .lazySkipping = (step > kLazySkippingStep as libc::c_ulong)
                                as libc::c_int;
                            continue;
                        } else {
                            if depth >= 1 as libc::c_int as libc::c_uint {
                                while ip < ilimit {
                                    ip = ip.offset(1);
                                    if dictMode as libc::c_uint
                                        == ZSTD_noDict as libc::c_int as libc::c_uint
                                        && offBase != 0
                                        && (offset_1 > 0 as libc::c_int as libc::c_uint)
                                            as libc::c_int
                                            & (MEM_read32(ip as *const libc::c_void)
                                                == MEM_read32(
                                                    ip.offset(-(offset_1 as isize)) as *const libc::c_void,
                                                )) as libc::c_int != 0
                                    {
                                        let mlRep = (ZSTD_count(
                                            ip.offset(4 as libc::c_int as isize),
                                            ip
                                                .offset(4 as libc::c_int as isize)
                                                .offset(-(offset_1 as isize)),
                                            iend,
                                        ))
                                            .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                        let gain2 = mlRep
                                            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                            as libc::c_int;
                                        let gain1 = matchLength
                                            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(
                                                ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                            )
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as libc::c_int;
                                        if mlRep >= 4 as libc::c_int as libc::c_ulong
                                            && gain2 > gain1
                                        {
                                            matchLength = mlRep;
                                            if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                                __assert_fail(
                                                    b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                        as *const u8 as *const libc::c_char,
                                                    1615 as libc::c_int as libc::c_uint,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 159],
                                                        &[libc::c_char; 159],
                                                    >(
                                                        b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                    ))
                                                        .as_ptr(),
                                                );
                                            }
                                            if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                                __assert_fail(
                                                    b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                        as *const u8 as *const libc::c_char,
                                                    1615 as libc::c_int as libc::c_uint,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 159],
                                                        &[libc::c_char; 159],
                                                    >(
                                                        b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                    ))
                                                        .as_ptr(),
                                                );
                                            }
                                            offBase = 1 as libc::c_int as size_t;
                                            start = ip;
                                        }
                                    }
                                    if isDxS != 0 {
                                        let repIndex_0 = (ip.offset_from(base) as libc::c_long
                                            as U32)
                                            .wrapping_sub(offset_1);
                                        let mut repMatch_0 = if repIndex_0 < prefixLowestIndex {
                                            dictBase
                                                .offset(repIndex_0.wrapping_sub(dictIndexDelta) as isize)
                                        } else {
                                            base.offset(repIndex_0 as isize)
                                        };
                                        if prefixLowestIndex
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            .wrapping_sub(repIndex_0)
                                            >= 3 as libc::c_int as libc::c_uint
                                            && MEM_read32(repMatch_0 as *const libc::c_void)
                                                == MEM_read32(ip as *const libc::c_void)
                                        {
                                            let mut repMatchEnd_0 = if repIndex_0 < prefixLowestIndex {
                                                dictEnd
                                            } else {
                                                iend
                                            };
                                            let mlRep_0 = (ZSTD_count_2segments(
                                                ip.offset(4 as libc::c_int as isize),
                                                repMatch_0.offset(4 as libc::c_int as isize),
                                                iend,
                                                repMatchEnd_0,
                                                prefixLowest,
                                            ))
                                                .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                            let gain2_0 = mlRep_0
                                                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            let gain1_0 = matchLength
                                                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(
                                                    ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                                )
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            if mlRep_0 >= 4 as libc::c_int as libc::c_ulong
                                                && gain2_0 > gain1_0
                                            {
                                                matchLength = mlRep_0;
                                                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                                    __assert_fail(
                                                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1629 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 159],
                                                            &[libc::c_char; 159],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                                    __assert_fail(
                                                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1629 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 159],
                                                            &[libc::c_char; 159],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                offBase = 1 as libc::c_int as size_t;
                                                start = ip;
                                            }
                                        }
                                    }
                                    let mut ofbCandidate = 999999999 as libc::c_int as size_t;
                                    let ml2_0 = ZSTD_searchMax(
                                        ms,
                                        ip,
                                        iend,
                                        &mut ofbCandidate,
                                        mls,
                                        rowLog,
                                        searchMethod,
                                        dictMode,
                                    );
                                    let gain2_1 = ml2_0
                                        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(
                                            ZSTD_highbit32(ofbCandidate as U32) as libc::c_ulong,
                                        ) as libc::c_int;
                                    let gain1_1 = matchLength
                                        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(
                                            ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                        )
                                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                        as libc::c_int;
                                    if ml2_0 >= 4 as libc::c_int as libc::c_ulong
                                        && gain2_1 > gain1_1
                                    {
                                        matchLength = ml2_0;
                                        offBase = ofbCandidate;
                                        start = ip;
                                    } else {
                                        if !(depth == 2 as libc::c_int as libc::c_uint
                                            && ip < ilimit)
                                        {
                                            break;
                                        }
                                        ip = ip.offset(1);
                                        if dictMode as libc::c_uint
                                            == ZSTD_noDict as libc::c_int as libc::c_uint
                                            && offBase != 0
                                            && (offset_1 > 0 as libc::c_int as libc::c_uint)
                                                as libc::c_int
                                                & (MEM_read32(ip as *const libc::c_void)
                                                    == MEM_read32(
                                                        ip.offset(-(offset_1 as isize)) as *const libc::c_void,
                                                    )) as libc::c_int != 0
                                        {
                                            let mlRep_1 = (ZSTD_count(
                                                ip.offset(4 as libc::c_int as isize),
                                                ip
                                                    .offset(4 as libc::c_int as isize)
                                                    .offset(-(offset_1 as isize)),
                                                iend,
                                            ))
                                                .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                            let gain2_2 = mlRep_1
                                                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            let gain1_2 = matchLength
                                                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(
                                                    ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                                )
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            if mlRep_1 >= 4 as libc::c_int as libc::c_ulong
                                                && gain2_2 > gain1_2
                                            {
                                                matchLength = mlRep_1;
                                                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                                    __assert_fail(
                                                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1651 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 159],
                                                            &[libc::c_char; 159],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                                    __assert_fail(
                                                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1651 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 159],
                                                            &[libc::c_char; 159],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                offBase = 1 as libc::c_int as size_t;
                                                start = ip;
                                            }
                                        }
                                        if isDxS != 0 {
                                            let repIndex_1 = (ip.offset_from(base) as libc::c_long
                                                as U32)
                                                .wrapping_sub(offset_1);
                                            let mut repMatch_1 = if repIndex_1 < prefixLowestIndex {
                                                dictBase
                                                    .offset(repIndex_1.wrapping_sub(dictIndexDelta) as isize)
                                            } else {
                                                base.offset(repIndex_1 as isize)
                                            };
                                            if prefixLowestIndex
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                .wrapping_sub(repIndex_1)
                                                >= 3 as libc::c_int as libc::c_uint
                                                && MEM_read32(repMatch_1 as *const libc::c_void)
                                                    == MEM_read32(ip as *const libc::c_void)
                                            {
                                                let mut repMatchEnd_1 = if repIndex_1 < prefixLowestIndex {
                                                    dictEnd
                                                } else {
                                                    iend
                                                };
                                                let mlRep_2 = (ZSTD_count_2segments(
                                                    ip.offset(4 as libc::c_int as isize),
                                                    repMatch_1.offset(4 as libc::c_int as isize),
                                                    iend,
                                                    repMatchEnd_1,
                                                    prefixLowest,
                                                ))
                                                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                                let gain2_3 = mlRep_2
                                                    .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                                    as libc::c_int;
                                                let gain1_3 = matchLength
                                                    .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(
                                                        ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    as libc::c_int;
                                                if mlRep_2 >= 4 as libc::c_int as libc::c_ulong
                                                    && gain2_3 > gain1_3
                                                {
                                                    matchLength = mlRep_2;
                                                    if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                                        __assert_fail(
                                                            b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                                as *const u8 as *const libc::c_char,
                                                            1665 as libc::c_int as libc::c_uint,
                                                            (*::core::mem::transmute::<
                                                                &[u8; 159],
                                                                &[libc::c_char; 159],
                                                            >(
                                                                b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                            ))
                                                                .as_ptr(),
                                                        );
                                                    }
                                                    if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                                        __assert_fail(
                                                            b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                                as *const u8 as *const libc::c_char,
                                                            1665 as libc::c_int as libc::c_uint,
                                                            (*::core::mem::transmute::<
                                                                &[u8; 159],
                                                                &[libc::c_char; 159],
                                                            >(
                                                                b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                            ))
                                                                .as_ptr(),
                                                        );
                                                    }
                                                    offBase = 1 as libc::c_int as size_t;
                                                    start = ip;
                                                }
                                            }
                                        }
                                        let mut ofbCandidate_0 = 999999999 as libc::c_int as size_t;
                                        let ml2_1 = ZSTD_searchMax(
                                            ms,
                                            ip,
                                            iend,
                                            &mut ofbCandidate_0,
                                            mls,
                                            rowLog,
                                            searchMethod,
                                            dictMode,
                                        );
                                        let gain2_4 = ml2_1
                                            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(
                                                ZSTD_highbit32(ofbCandidate_0 as U32) as libc::c_ulong,
                                            ) as libc::c_int;
                                        let gain1_4 = matchLength
                                            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(
                                                ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                            )
                                            .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                            as libc::c_int;
                                        if !(ml2_1 >= 4 as libc::c_int as libc::c_ulong
                                            && gain2_4 > gain1_4)
                                        {
                                            break;
                                        }
                                        matchLength = ml2_1;
                                        offBase = ofbCandidate_0;
                                        start = ip;
                                    }
                                }
                            }
                            if offBase > ZSTD_REP_NUM as libc::c_ulong {
                                if dictMode as libc::c_uint
                                    == ZSTD_noDict as libc::c_int as libc::c_uint
                                {
                                    loop {
                                        if offBase > 3 as libc::c_int as libc::c_ulong {} else {
                                            __assert_fail(
                                                b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8
                                                    as *const libc::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                    as *const u8 as *const libc::c_char,
                                                1686 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 159],
                                                    &[libc::c_char; 159],
                                                >(
                                                    b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                        if !((start > anchor) as libc::c_int
                                            & (start
                                                .offset(
                                                    -(offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong)
                                                        as isize),
                                                ) > prefixLowest) as libc::c_int != 0
                                            && {
                                                if offBase > 3 as libc::c_int as libc::c_ulong {} else {
                                                    __assert_fail(
                                                        b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8
                                                            as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1687 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 159],
                                                            &[libc::c_char; 159],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                *start.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == *start
                                                        .offset(
                                                            -(offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong)
                                                                as isize),
                                                        )
                                                        .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                            })
                                        {
                                            break;
                                        }
                                        start = start.offset(-1);
                                        matchLength = matchLength.wrapping_add(1);
                                    }
                                }
                                if isDxS != 0 {
                                    if offBase > 3 as libc::c_int as libc::c_ulong {} else {
                                        __assert_fail(
                                            b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8
                                                as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1691 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 159],
                                                &[libc::c_char; 159],
                                            >(
                                                b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let matchIndex = (start.offset_from(base) as libc::c_long
                                        as size_t)
                                        .wrapping_sub(
                                            offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong),
                                        ) as U32;
                                    let mut match_0 = if matchIndex < prefixLowestIndex {
                                        dictBase
                                            .offset(matchIndex as isize)
                                            .offset(-(dictIndexDelta as isize))
                                    } else {
                                        base.offset(matchIndex as isize)
                                    };
                                    let mStart = if matchIndex < prefixLowestIndex {
                                        dictLowest
                                    } else {
                                        prefixLowest
                                    };
                                    while start > anchor && match_0 > mStart
                                        && *start.offset(-(1 as libc::c_int) as isize)
                                            as libc::c_int
                                            == *match_0.offset(-(1 as libc::c_int) as isize)
                                                as libc::c_int
                                    {
                                        start = start.offset(-1);
                                        match_0 = match_0.offset(-1);
                                        matchLength = matchLength.wrapping_add(1);
                                    }
                                }
                                offset_2 = offset_1;
                                if offBase > 3 as libc::c_int as libc::c_ulong {} else {
                                    __assert_fail(
                                        b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8
                                            as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1696 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 159],
                                            &[libc::c_char; 159],
                                        >(
                                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                offset_1 = offBase
                                    .wrapping_sub(ZSTD_REP_NUM as libc::c_ulong) as U32;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        let litLength = start.offset_from(anchor) as libc::c_long as size_t;
        ZSTD_storeSeq(seqStore, litLength, anchor, iend, offBase as U32, matchLength);
        ip = start.offset(matchLength as isize);
        anchor = ip;
        if (*ms).lazySkipping != 0 {
            if searchMethod as libc::c_uint
                == search_rowHash as libc::c_int as libc::c_uint
            {
                ZSTD_row_fillHashCache(
                    ms,
                    base,
                    rowLog,
                    mls,
                    (*ms).nextToUpdate,
                    ilimit,
                );
            }
            (*ms).lazySkipping = 0 as libc::c_int;
        }
        if isDxS != 0 {
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as libc::c_long as U32;
                let repIndex_2 = current2.wrapping_sub(offset_2);
                let mut repMatch_2 = if repIndex_2 < prefixLowestIndex {
                    dictBase
                        .offset(-(dictIndexDelta as isize))
                        .offset(repIndex_2 as isize)
                } else {
                    base.offset(repIndex_2 as isize)
                };
                if !(prefixLowestIndex
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(repIndex_2) >= 3 as libc::c_int as libc::c_uint
                    && MEM_read32(repMatch_2 as *const libc::c_void)
                        == MEM_read32(ip as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex_2 < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                matchLength = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    repMatch_2.offset(4 as libc::c_int as isize),
                    iend,
                    repEnd2,
                    prefixLowest,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                offBase = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offBase as U32;
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        1725 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 159],
                            &[libc::c_char; 159],
                        >(
                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        1725 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 159],
                            &[libc::c_char; 159],
                        >(
                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ZSTD_storeSeq(
                    seqStore,
                    0 as libc::c_int as size_t,
                    anchor,
                    iend,
                    1 as libc::c_int as U32,
                    matchLength,
                );
                ip = ip.offset(matchLength as isize);
                anchor = ip;
            }
        }
        if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint {
            while (ip <= ilimit) as libc::c_int
                & (offset_2 > 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
                && MEM_read32(ip as *const libc::c_void)
                    == MEM_read32(ip.offset(-(offset_2 as isize)) as *const libc::c_void)
            {
                matchLength = (ZSTD_count(
                    ip.offset(4 as libc::c_int as isize),
                    ip.offset(4 as libc::c_int as isize).offset(-(offset_2 as isize)),
                    iend,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                offBase = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offBase as U32;
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        1740 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 159],
                            &[libc::c_char; 159],
                        >(
                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                            as *const u8 as *const libc::c_char,
                        1740 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 159],
                            &[libc::c_char; 159],
                        >(
                            b"size_t ZSTD_compressBlock_lazy_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32, const ZSTD_dictMode_e)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ZSTD_storeSeq(
                    seqStore,
                    0 as libc::c_int as size_t,
                    anchor,
                    iend,
                    1 as libc::c_int as U32,
                    matchLength,
                );
                ip = ip.offset(matchLength as isize);
                anchor = ip;
            }
        }
    }
    offsetSaved2 = if offsetSaved1 != 0 as libc::c_int as libc::c_uint
        && offset_1 != 0 as libc::c_int as libc::c_uint
    {
        offsetSaved1
    } else {
        offsetSaved2
    };
    *rep
        .offset(
            0 as libc::c_int as isize,
        ) = if offset_1 != 0 { offset_1 } else { offsetSaved1 };
    *rep
        .offset(
            1 as libc::c_int as isize,
        ) = if offset_2 != 0 { offset_2 } else { offsetSaved2 };
    return iend.offset_from(anchor) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dedicatedDictSearch(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dedicatedDictSearch(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dedicatedDictSearch(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as libc::c_int as U32,
        ZSTD_noDict,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dictMatchState_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dictMatchState_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dictMatchState_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as libc::c_int as U32,
        ZSTD_dictMatchState,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dedicatedDictSearch_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dedicatedDictSearch_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dedicatedDictSearch_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as libc::c_int as U32,
        ZSTD_dedicatedDictSearch,
    );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    searchMethod: searchMethod_e,
    depth: U32,
) -> size_t {
    let istart = src as *const BYTE;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = if searchMethod as libc::c_uint
        == search_rowHash as libc::c_int as libc::c_uint
    {
        iend.offset(-(8 as libc::c_int as isize))
            .offset(-(ZSTD_ROW_HASH_CACHE_SIZE as isize))
    } else {
        iend.offset(-(8 as libc::c_int as isize))
    };
    let base = (*ms).window.base;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictBase = (*ms).window.dictBase;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let dictStart = dictBase.offset((*ms).window.lowLimit as isize);
    let windowLog = (*ms).cParams.windowLog;
    let mls = if 4 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        4 as libc::c_int as libc::c_uint
    } else if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as libc::c_int as libc::c_uint
    };
    let rowLog = if 4 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.searchLog < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.searchLog
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        4 as libc::c_int as libc::c_uint
    } else if (*ms).cParams.searchLog < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.searchLog
    } else {
        6 as libc::c_int as libc::c_uint
    };
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    (*ms).lazySkipping = 0 as libc::c_int;
    ip = ip.offset((ip == prefixStart) as libc::c_int as isize);
    if searchMethod as libc::c_uint == search_rowHash as libc::c_int as libc::c_uint {
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, (*ms).nextToUpdate, ilimit);
    }
    asm!(".p2align 5", options(preserves_flags, att_syntax));
    let mut current_block_61: u64;
    while ip < ilimit {
        let mut matchLength = 0 as libc::c_int as size_t;
        if 1 as libc::c_int >= 1 as libc::c_int {} else {
            __assert_fail(
                b"(1)>=1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1946 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 144],
                    &[libc::c_char; 144],
                >(
                    b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        if 1 as libc::c_int <= 3 as libc::c_int {} else {
            __assert_fail(
                b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0" as *const u8
                    as *const libc::c_char,
                1946 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 144],
                    &[libc::c_char; 144],
                >(
                    b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        let mut offBase = 1 as libc::c_int as size_t;
        let mut start = ip.offset(1 as libc::c_int as isize);
        let mut curr = ip.offset_from(base) as libc::c_long as U32;
        let windowLow = ZSTD_getLowestMatchIndex(
            ms,
            curr.wrapping_add(1 as libc::c_int as libc::c_uint),
            windowLog,
        );
        let repIndex = curr
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(offset_1);
        let repBase = if repIndex < dictLimit { dictBase } else { base };
        let repMatch = repBase.offset(repIndex as isize);
        if (dictLimit
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(repIndex) >= 3 as libc::c_int as libc::c_uint) as libc::c_int
            & (offset_1
                <= curr
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(windowLow)) as libc::c_int != 0
        {
            if MEM_read32(ip.offset(1 as libc::c_int as isize) as *const libc::c_void)
                == MEM_read32(repMatch as *const libc::c_void)
            {
                let repEnd = if repIndex < dictLimit { dictEnd } else { iend };
                matchLength = (ZSTD_count_2segments(
                    ip
                        .offset(1 as libc::c_int as isize)
                        .offset(4 as libc::c_int as isize),
                    repMatch.offset(4 as libc::c_int as isize),
                    iend,
                    repEnd,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                if depth == 0 as libc::c_int as libc::c_uint {
                    current_block_61 = 13008900890699373198;
                } else {
                    current_block_61 = 4808432441040389987;
                }
            } else {
                current_block_61 = 4808432441040389987;
            }
        } else {
            current_block_61 = 4808432441040389987;
        }
        match current_block_61 {
            4808432441040389987 => {
                let mut ofbCandidate = 999999999 as libc::c_int as size_t;
                let ml2 = ZSTD_searchMax(
                    ms,
                    ip,
                    iend,
                    &mut ofbCandidate,
                    mls,
                    rowLog,
                    searchMethod,
                    ZSTD_extDict,
                );
                if ml2 > matchLength {
                    matchLength = ml2;
                    start = ip;
                    offBase = ofbCandidate;
                }
                if matchLength < 4 as libc::c_int as libc::c_ulong {
                    let step = ip.offset_from(anchor) as libc::c_long as size_t
                        >> kSearchStrength;
                    ip = ip
                        .offset(
                            step.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    (*ms)
                        .lazySkipping = (step > kLazySkippingStep as libc::c_ulong)
                        as libc::c_int;
                    continue;
                } else {
                    if depth >= 1 as libc::c_int as libc::c_uint {
                        while ip < ilimit {
                            ip = ip.offset(1);
                            curr = curr.wrapping_add(1);
                            if offBase != 0 {
                                let windowLow_0 = ZSTD_getLowestMatchIndex(
                                    ms,
                                    curr,
                                    windowLog,
                                );
                                let repIndex_0 = curr.wrapping_sub(offset_1);
                                let repBase_0 = if repIndex_0 < dictLimit {
                                    dictBase
                                } else {
                                    base
                                };
                                let repMatch_0 = repBase_0.offset(repIndex_0 as isize);
                                if (dictLimit
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    .wrapping_sub(repIndex_0)
                                    >= 3 as libc::c_int as libc::c_uint) as libc::c_int
                                    & (offset_1 <= curr.wrapping_sub(windowLow_0))
                                        as libc::c_int != 0
                                {
                                    if MEM_read32(ip as *const libc::c_void)
                                        == MEM_read32(repMatch_0 as *const libc::c_void)
                                    {
                                        let repEnd_0 = if repIndex_0 < dictLimit {
                                            dictEnd
                                        } else {
                                            iend
                                        };
                                        let repLength = (ZSTD_count_2segments(
                                            ip.offset(4 as libc::c_int as isize),
                                            repMatch_0.offset(4 as libc::c_int as isize),
                                            iend,
                                            repEnd_0,
                                            prefixStart,
                                        ))
                                            .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                        let gain2 = repLength
                                            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                            as libc::c_int;
                                        let gain1 = matchLength
                                            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(
                                                ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                            )
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as libc::c_int;
                                        if repLength >= 4 as libc::c_int as libc::c_ulong
                                            && gain2 > gain1
                                        {
                                            matchLength = repLength;
                                            if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                                __assert_fail(
                                                    b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                        as *const u8 as *const libc::c_char,
                                                    2005 as libc::c_int as libc::c_uint,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 144],
                                                        &[libc::c_char; 144],
                                                    >(
                                                        b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                                                    ))
                                                        .as_ptr(),
                                                );
                                            }
                                            if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                                __assert_fail(
                                                    b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                        as *const u8 as *const libc::c_char,
                                                    2005 as libc::c_int as libc::c_uint,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 144],
                                                        &[libc::c_char; 144],
                                                    >(
                                                        b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                                                    ))
                                                        .as_ptr(),
                                                );
                                            }
                                            offBase = 1 as libc::c_int as size_t;
                                            start = ip;
                                        }
                                    }
                                }
                            }
                            let mut ofbCandidate_0 = 999999999 as libc::c_int as size_t;
                            let ml2_0 = ZSTD_searchMax(
                                ms,
                                ip,
                                iend,
                                &mut ofbCandidate_0,
                                mls,
                                rowLog,
                                searchMethod,
                                ZSTD_extDict,
                            );
                            let gain2_0 = ml2_0
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(
                                    ZSTD_highbit32(ofbCandidate_0 as U32) as libc::c_ulong,
                                ) as libc::c_int;
                            let gain1_0 = matchLength
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(
                                    ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                )
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                as libc::c_int;
                            if ml2_0 >= 4 as libc::c_int as libc::c_ulong
                                && gain2_0 > gain1_0
                            {
                                matchLength = ml2_0;
                                offBase = ofbCandidate_0;
                                start = ip;
                            } else {
                                if !(depth == 2 as libc::c_int as libc::c_uint
                                    && ip < ilimit)
                                {
                                    break;
                                }
                                ip = ip.offset(1);
                                curr = curr.wrapping_add(1);
                                if offBase != 0 {
                                    let windowLow_1 = ZSTD_getLowestMatchIndex(
                                        ms,
                                        curr,
                                        windowLog,
                                    );
                                    let repIndex_1 = curr.wrapping_sub(offset_1);
                                    let repBase_1 = if repIndex_1 < dictLimit {
                                        dictBase
                                    } else {
                                        base
                                    };
                                    let repMatch_1 = repBase_1.offset(repIndex_1 as isize);
                                    if (dictLimit
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(repIndex_1)
                                        >= 3 as libc::c_int as libc::c_uint) as libc::c_int
                                        & (offset_1 <= curr.wrapping_sub(windowLow_1))
                                            as libc::c_int != 0
                                    {
                                        if MEM_read32(ip as *const libc::c_void)
                                            == MEM_read32(repMatch_1 as *const libc::c_void)
                                        {
                                            let repEnd_1 = if repIndex_1 < dictLimit {
                                                dictEnd
                                            } else {
                                                iend
                                            };
                                            let repLength_0 = (ZSTD_count_2segments(
                                                ip.offset(4 as libc::c_int as isize),
                                                repMatch_1.offset(4 as libc::c_int as isize),
                                                iend,
                                                repEnd_1,
                                                prefixStart,
                                            ))
                                                .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                            let gain2_1 = repLength_0
                                                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            let gain1_1 = matchLength
                                                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(
                                                    ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                                )
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            if repLength_0 >= 4 as libc::c_int as libc::c_ulong
                                                && gain2_1 > gain1_1
                                            {
                                                matchLength = repLength_0;
                                                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                                    __assert_fail(
                                                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        2037 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 144],
                                                            &[libc::c_char; 144],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                                    __assert_fail(
                                                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        2037 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 144],
                                                            &[libc::c_char; 144],
                                                        >(
                                                            b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                                offBase = 1 as libc::c_int as size_t;
                                                start = ip;
                                            }
                                        }
                                    }
                                }
                                let mut ofbCandidate_1 = 999999999 as libc::c_int as size_t;
                                let ml2_1 = ZSTD_searchMax(
                                    ms,
                                    ip,
                                    iend,
                                    &mut ofbCandidate_1,
                                    mls,
                                    rowLog,
                                    searchMethod,
                                    ZSTD_extDict,
                                );
                                let gain2_2 = ml2_1
                                    .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(
                                        ZSTD_highbit32(ofbCandidate_1 as U32) as libc::c_ulong,
                                    ) as libc::c_int;
                                let gain1_2 = matchLength
                                    .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(
                                        ZSTD_highbit32(offBase as U32) as libc::c_ulong,
                                    )
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as libc::c_int;
                                if !(ml2_1 >= 4 as libc::c_int as libc::c_ulong
                                    && gain2_2 > gain1_2)
                                {
                                    break;
                                }
                                matchLength = ml2_1;
                                offBase = ofbCandidate_1;
                                start = ip;
                            }
                        }
                    }
                    if offBase > ZSTD_REP_NUM as libc::c_ulong {
                        if offBase > 3 as libc::c_int as libc::c_ulong {} else {
                            __assert_fail(
                                b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8
                                    as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                    as *const u8 as *const libc::c_char,
                                2054 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 144],
                                    &[libc::c_char; 144],
                                >(
                                    b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let matchIndex = (start.offset_from(base) as libc::c_long
                            as size_t)
                            .wrapping_sub(
                                offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong),
                            ) as U32;
                        let mut match_0 = if matchIndex < dictLimit {
                            dictBase.offset(matchIndex as isize)
                        } else {
                            base.offset(matchIndex as isize)
                        };
                        let mStart = if matchIndex < dictLimit {
                            dictStart
                        } else {
                            prefixStart
                        };
                        while start > anchor && match_0 > mStart
                            && *start.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                == *match_0.offset(-(1 as libc::c_int) as isize)
                                    as libc::c_int
                        {
                            start = start.offset(-1);
                            match_0 = match_0.offset(-1);
                            matchLength = matchLength.wrapping_add(1);
                        }
                        offset_2 = offset_1;
                        if offBase > 3 as libc::c_int as libc::c_ulong {} else {
                            __assert_fail(
                                b"OFFBASE_IS_OFFSET(offBase)\0" as *const u8
                                    as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                                    as *const u8 as *const libc::c_char,
                                2058 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 144],
                                    &[libc::c_char; 144],
                                >(
                                    b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        offset_1 = offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong)
                            as U32;
                    }
                }
            }
            _ => {}
        }
        let litLength = start.offset_from(anchor) as libc::c_long as size_t;
        ZSTD_storeSeq(seqStore, litLength, anchor, iend, offBase as U32, matchLength);
        ip = start.offset(matchLength as isize);
        anchor = ip;
        if (*ms).lazySkipping != 0 {
            if searchMethod as libc::c_uint
                == search_rowHash as libc::c_int as libc::c_uint
            {
                ZSTD_row_fillHashCache(
                    ms,
                    base,
                    rowLog,
                    mls,
                    (*ms).nextToUpdate,
                    ilimit,
                );
            }
            (*ms).lazySkipping = 0 as libc::c_int;
        }
        while ip <= ilimit {
            let repCurrent = ip.offset_from(base) as libc::c_long as U32;
            let windowLow_2 = ZSTD_getLowestMatchIndex(ms, repCurrent, windowLog);
            let repIndex_2 = repCurrent.wrapping_sub(offset_2);
            let repBase_2 = if repIndex_2 < dictLimit { dictBase } else { base };
            let repMatch_2 = repBase_2.offset(repIndex_2 as isize);
            if !((dictLimit
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(repIndex_2) >= 3 as libc::c_int as libc::c_uint)
                as libc::c_int
                & (offset_2 <= repCurrent.wrapping_sub(windowLow_2)) as libc::c_int != 0)
            {
                break;
            }
            if !(MEM_read32(ip as *const libc::c_void)
                == MEM_read32(repMatch_2 as *const libc::c_void))
            {
                break;
            }
            let repEnd_2 = if repIndex_2 < dictLimit { dictEnd } else { iend };
            matchLength = (ZSTD_count_2segments(
                ip.offset(4 as libc::c_int as isize),
                repMatch_2.offset(4 as libc::c_int as isize),
                iend,
                repEnd_2,
                prefixStart,
            ))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            offBase = offset_2 as size_t;
            offset_2 = offset_1;
            offset_1 = offBase as U32;
            if 1 as libc::c_int >= 1 as libc::c_int {} else {
                __assert_fail(
                    b"(1)>=1\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    2089 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 144],
                        &[libc::c_char; 144],
                    >(
                        b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if 1 as libc::c_int <= 3 as libc::c_int {} else {
                __assert_fail(
                    b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_lazy.c\0"
                        as *const u8 as *const libc::c_char,
                    2089 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 144],
                        &[libc::c_char; 144],
                    >(
                        b"size_t ZSTD_compressBlock_lazy_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const searchMethod_e, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_storeSeq(
                seqStore,
                0 as libc::c_int as size_t,
                anchor,
                iend,
                1 as libc::c_int as U32,
                matchLength,
            );
            ip = ip.offset(matchLength as isize);
            anchor = ip;
        }
    }
    *rep.offset(0 as libc::c_int as isize) = offset_1;
    *rep.offset(1 as libc::c_int as isize) = offset_2;
    return iend.offset_from(anchor) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_extDict_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_extDict_row(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as libc::c_int as U32,
    );
}
