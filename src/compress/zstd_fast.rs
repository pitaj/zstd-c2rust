use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
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
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = libc::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
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
pub const CACHELINE_SIZE: libc::c_int = 64 as libc::c_int;
pub const kSearchStrength: libc::c_int = 8 as libc::c_int;
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
pub const HASH_READ_SIZE: libc::c_int = 8 as libc::c_int;
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
#[inline]
unsafe extern "C" fn ZSTD_writeTaggedIndex(
    hashTable: *mut U32,
    mut hashAndTag: size_t,
    mut index: U32,
) {
    let hash = hashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
    let tag = (hashAndTag & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as U32;
    if index >> 32 as libc::c_int - 8 as libc::c_int == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"index >> (32 - ZSTD_SHORT_CACHE_TAG_BITS) == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_internal.h\0"
                as *const u8 as *const libc::c_char,
            1377 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void ZSTD_writeTaggedIndex(U32 *const, size_t, U32)\0"))
                .as_ptr(),
        );
    }
    *hashTable.offset(hash as isize) = index << ZSTD_SHORT_CACHE_TAG_BITS | tag;
}
pub const ZSTD_SHORT_CACHE_TAG_BITS: libc::c_int = 8 as libc::c_int;
pub const ZSTD_SHORT_CACHE_TAG_MASK: libc::c_uint = ((1 as libc::c_uint)
    << ZSTD_SHORT_CACHE_TAG_BITS)
    .wrapping_sub(1 as libc::c_int as libc::c_uint);
#[inline]
unsafe extern "C" fn ZSTD_comparePackedTags(
    mut packedTag1: size_t,
    mut packedTag2: size_t,
) -> libc::c_int {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as U32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as U32;
    return (tag1 == tag2) as libc::c_int;
}
unsafe extern "C" fn ZSTD_fillHashTableForCDict(
    mut ms: *mut ZSTD_matchState_t,
    end: *const libc::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hBits = ((*cParams).hashLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let mls = (*cParams).minMatch;
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const BYTE).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3 as libc::c_int as U32;
    if dtlm as libc::c_uint == ZSTD_dtlm_full as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"dtlm == ZSTD_dtlm_full\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void ZSTD_fillHashTableForCDict(ZSTD_matchState_t *, const void *const, ZSTD_dictTableLoadMethod_e)\0",
            ))
                .as_ptr(),
        );
    }
    while ip.offset(fastHashFillStep as isize) < iend.offset(2 as libc::c_int as isize) {
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let hashAndTag = ZSTD_hashPtr(ip as *const libc::c_void, hBits, mls);
        ZSTD_writeTaggedIndex(hashTable, hashAndTag, curr);
        if !(dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint) {
            let mut p: U32 = 0;
            p = 1 as libc::c_int as U32;
            while p < fastHashFillStep {
                let hashAndTag_0 = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const libc::c_void,
                    hBits,
                    mls,
                );
                if *hashTable
                    .offset((hashAndTag_0 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize)
                    == 0 as libc::c_int as libc::c_uint
                {
                    ZSTD_writeTaggedIndex(hashTable, hashAndTag_0, curr.wrapping_add(p));
                }
                p = p.wrapping_add(1);
            }
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
unsafe extern "C" fn ZSTD_fillHashTableForCCtx(
    mut ms: *mut ZSTD_matchState_t,
    end: *const libc::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hBits = (*cParams).hashLog;
    let mls = (*cParams).minMatch;
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const BYTE).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3 as libc::c_int as U32;
    if dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"dtlm == ZSTD_dtlm_fast\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"void ZSTD_fillHashTableForCCtx(ZSTD_matchState_t *, const void *const, ZSTD_dictTableLoadMethod_e)\0",
            ))
                .as_ptr(),
        );
    }
    while ip.offset(fastHashFillStep as isize) < iend.offset(2 as libc::c_int as isize) {
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let hash0 = ZSTD_hashPtr(ip as *const libc::c_void, hBits, mls);
        *hashTable.offset(hash0 as isize) = curr;
        if !(dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint) {
            let mut p: U32 = 0;
            p = 1 as libc::c_int as U32;
            while p < fastHashFillStep {
                let hash = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const libc::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.offset(hash as isize) == 0 as libc::c_int as libc::c_uint {
                    *hashTable.offset(hash as isize) = curr.wrapping_add(p);
                }
                p = p.wrapping_add(1);
            }
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_fillHashTable(
    mut ms: *mut ZSTD_matchState_t,
    end: *const libc::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
) {
    if tfp as libc::c_uint == ZSTD_tfp_forCDict as libc::c_int as libc::c_uint {
        ZSTD_fillHashTableForCDict(ms, end, dtlm);
    } else {
        ZSTD_fillHashTableForCCtx(ms, end, dtlm);
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mls: U32,
    hasStep: U32,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = (if hasStep != 0 {
        ((*cParams).targetLength)
            .wrapping_add(((*cParams).targetLength == 0) as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    } else {
        2 as libc::c_int as libc::c_uint
    }) as size_t;
    let base = (*ms).window.base;
    let istart = src as *const BYTE;
    let endIndex = (istart.offset_from(base) as libc::c_long as size_t)
        .wrapping_add(srcSize) as U32;
    let prefixStartIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut anchor = istart;
    let mut ip0 = istart;
    let mut ip1 = 0 as *const BYTE;
    let mut ip2 = 0 as *const BYTE;
    let mut ip3 = 0 as *const BYTE;
    let mut current0: U32 = 0;
    let mut rep_offset1 = *rep.offset(0 as libc::c_int as isize);
    let mut rep_offset2 = *rep.offset(1 as libc::c_int as isize);
    let mut offsetSaved1 = 0 as libc::c_int as U32;
    let mut offsetSaved2 = 0 as libc::c_int as U32;
    let mut hash0: size_t = 0;
    let mut hash1: size_t = 0;
    let mut idx: U32 = 0;
    let mut mval: U32 = 0;
    let mut offcode: U32 = 0;
    let mut match0 = 0 as *const BYTE;
    let mut mLength: size_t = 0;
    let mut step: size_t = 0;
    let mut nextStep = 0 as *const BYTE;
    let kStepIncr = ((1 as libc::c_int) << kSearchStrength - 1 as libc::c_int) as size_t;
    ip0 = ip0.offset((ip0 == prefixStart) as libc::c_int as isize);
    let curr = ip0.offset_from(base) as libc::c_long as U32;
    let windowLow = ZSTD_getLowestPrefixIndex(ms, curr, (*cParams).windowLog);
    let maxRep = curr.wrapping_sub(windowLow);
    if rep_offset2 > maxRep {
        offsetSaved2 = rep_offset2;
        rep_offset2 = 0 as libc::c_int as U32;
    }
    if rep_offset1 > maxRep {
        offsetSaved1 = rep_offset1;
        rep_offset1 = 0 as libc::c_int as U32;
    }
    '__start: loop {
        step = stepSize;
        nextStep = ip0.offset(kStepIncr as isize);
        ip1 = ip0.offset(1 as libc::c_int as isize);
        ip2 = ip0.offset(step as isize);
        ip3 = ip2.offset(1 as libc::c_int as isize);
        if ip3 >= ilimit {
            break;
        }
        hash0 = ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const libc::c_void, hlog, mls);
        idx = *hashTable.offset(hash0 as isize);
        loop {
            let rval = MEM_read32(
                ip2.offset(-(rep_offset1 as isize)) as *const libc::c_void,
            );
            current0 = ip0.offset_from(base) as libc::c_long as U32;
            *hashTable.offset(hash0 as isize) = current0;
            if (MEM_read32(ip2 as *const libc::c_void) == rval) as libc::c_int
                & (rep_offset1 > 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
            {
                ip0 = ip2;
                match0 = ip0.offset(-(rep_offset1 as isize));
                mLength = (*ip0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == *match0.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    as libc::c_int as size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        233 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 132],
                            &[libc::c_char; 132],
                        >(
                            b"size_t ZSTD_compressBlock_fast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        233 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 132],
                            &[libc::c_char; 132],
                        >(
                            b"size_t ZSTD_compressBlock_fast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                offcode = 1 as libc::c_int as U32;
                mLength = (mLength as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
                *hashTable
                    .offset(
                        hash1 as isize,
                    ) = ip1.offset_from(base) as libc::c_long as U32;
                current_block = 6213199767695589360;
                break;
            } else {
                if idx >= prefixStartIndex {
                    mval = MEM_read32(base.offset(idx as isize) as *const libc::c_void);
                } else {
                    mval = MEM_read32(ip0 as *const libc::c_void)
                        ^ 1 as libc::c_int as libc::c_uint;
                }
                if MEM_read32(ip0 as *const libc::c_void) == mval {
                    *hashTable
                        .offset(
                            hash1 as isize,
                        ) = ip1.offset_from(base) as libc::c_long as U32;
                    current_block = 13355861697473861518;
                    break;
                } else {
                    idx = *hashTable.offset(hash1 as isize);
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const libc::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip3;
                    current0 = ip0.offset_from(base) as libc::c_long as U32;
                    *hashTable.offset(hash0 as isize) = current0;
                    if idx >= prefixStartIndex {
                        mval = MEM_read32(
                            base.offset(idx as isize) as *const libc::c_void,
                        );
                    } else {
                        mval = MEM_read32(ip0 as *const libc::c_void)
                            ^ 1 as libc::c_int as libc::c_uint;
                    }
                    if MEM_read32(ip0 as *const libc::c_void) == mval {
                        if step <= 4 as libc::c_int as libc::c_ulong {
                            *hashTable
                                .offset(
                                    hash1 as isize,
                                ) = ip1.offset_from(base) as libc::c_long as U32;
                        }
                        current_block = 13355861697473861518;
                        break;
                    } else {
                        idx = *hashTable.offset(hash1 as isize);
                        hash0 = hash1;
                        hash1 = ZSTD_hashPtr(ip2 as *const libc::c_void, hlog, mls);
                        ip0 = ip1;
                        ip1 = ip2;
                        ip2 = ip0.offset(step as isize);
                        ip3 = ip1.offset(step as isize);
                        if ip2 >= nextStep {
                            step = step.wrapping_add(1);
                            nextStep = nextStep.offset(kStepIncr as isize);
                        }
                        if !(ip3 < ilimit) {
                            break '__start;
                        }
                    }
                }
            }
        }
        match current_block {
            13355861697473861518 => {
                match0 = base.offset(idx as isize);
                rep_offset2 = rep_offset1;
                rep_offset1 = ip0.offset_from(match0) as libc::c_long as U32;
                if rep_offset1 > 0 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"(rep_offset1)>0\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        362 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 132],
                            &[libc::c_char; 132],
                        >(
                            b"size_t ZSTD_compressBlock_fast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                offcode = rep_offset1.wrapping_add(ZSTD_REP_NUM as libc::c_uint);
                mLength = 4 as libc::c_int as size_t;
                while (ip0 > anchor) as libc::c_int
                    & (match0 > prefixStart) as libc::c_int != 0
                    && *ip0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == *match0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                {
                    ip0 = ip0.offset(-1);
                    match0 = match0.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            }
            _ => {}
        }
        mLength = (mLength as libc::c_ulong)
            .wrapping_add(
                ZSTD_count(
                    ip0.offset(mLength as isize),
                    match0.offset(mLength as isize),
                    iend,
                ),
            ) as size_t as size_t;
        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from(anchor) as libc::c_long as size_t,
            anchor,
            iend,
            offcode,
            mLength,
        );
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip0 <= ilimit {
            if base.offset(current0 as isize).offset(2 as libc::c_int as isize) > istart
            {} else {
                __assert_fail(
                    b"base+current0+2 > istart\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    385 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 132],
                        &[libc::c_char; 132],
                    >(
                        b"size_t ZSTD_compressBlock_fast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        base.offset(current0 as isize).offset(2 as libc::c_int as isize)
                            as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = current0.wrapping_add(2 as libc::c_int as libc::c_uint);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        ip0.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = ip0.offset(-(2 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            if rep_offset2 > 0 as libc::c_int as libc::c_uint {
                while ip0 <= ilimit
                    && MEM_read32(ip0 as *const libc::c_void)
                        == MEM_read32(
                            ip0.offset(-(rep_offset2 as isize)) as *const libc::c_void,
                        )
                {
                    let rLength = (ZSTD_count(
                        ip0.offset(4 as libc::c_int as isize),
                        ip0
                            .offset(4 as libc::c_int as isize)
                            .offset(-(rep_offset2 as isize)),
                        iend,
                    ))
                        .wrapping_add(4 as libc::c_int as libc::c_ulong);
                    let tmpOff = rep_offset2;
                    rep_offset2 = rep_offset1;
                    rep_offset1 = tmpOff;
                    *hashTable
                        .offset(
                            ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls) as isize,
                        ) = ip0.offset_from(base) as libc::c_long as U32;
                    ip0 = ip0.offset(rLength as isize);
                    if 1 as libc::c_int >= 1 as libc::c_int {} else {
                        __assert_fail(
                            b"(1)>=1\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                                as *const u8 as *const libc::c_char,
                            396 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 132],
                                &[libc::c_char; 132],
                            >(
                                b"size_t ZSTD_compressBlock_fast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if 1 as libc::c_int <= 3 as libc::c_int {} else {
                        __assert_fail(
                            b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                                as *const u8 as *const libc::c_char,
                            396 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 132],
                                &[libc::c_char; 132],
                            >(
                                b"size_t ZSTD_compressBlock_fast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
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
                        rLength,
                    );
                    anchor = ip0;
                }
            }
        }
    }
    offsetSaved2 = if offsetSaved1 != 0 as libc::c_int as libc::c_uint
        && rep_offset1 != 0 as libc::c_int as libc::c_uint
    {
        offsetSaved1
    } else {
        offsetSaved2
    };
    *rep
        .offset(
            0 as libc::c_int as isize,
        ) = if rep_offset1 != 0 { rep_offset1 } else { offsetSaved1 };
    *rep
        .offset(
            1 as libc::c_int as isize,
        ) = if rep_offset2 != 0 { rep_offset2 } else { offsetSaved2 };
    return iend.offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_4_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
        1 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_5_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
        1 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_6_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
        1 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_7_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
        1 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_4_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_5_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_6_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_7_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    if ((*ms).dictMatchState).is_null() {} else {
        __assert_fail(
            b"ms->dictMatchState == NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            427 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"size_t ZSTD_compressBlock_fast(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*ms).cParams.targetLength > 1 as libc::c_int as libc::c_uint {
        match mls {
            5 => {
                return ZSTD_compressBlock_fast_noDict_5_1(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
            6 => {
                return ZSTD_compressBlock_fast_noDict_6_1(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
            7 => {
                return ZSTD_compressBlock_fast_noDict_7_1(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
            4 | _ => {
                return ZSTD_compressBlock_fast_noDict_4_1(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
        }
    } else {
        match mls {
            5 => {
                return ZSTD_compressBlock_fast_noDict_5_0(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
            6 => {
                return ZSTD_compressBlock_fast_noDict_6_0(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
            7 => {
                return ZSTD_compressBlock_fast_noDict_7_0(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
            4 | _ => {
                return ZSTD_compressBlock_fast_noDict_4_0(
                    ms,
                    seqStore,
                    rep,
                    src,
                    srcSize,
                );
            }
        }
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mls: U32,
    hasStep: U32,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as libc::c_int as libc::c_uint);
    let base = (*ms).window.base;
    let istart = src as *const BYTE;
    let mut ip0 = istart;
    let mut ip1 = ip0.offset(stepSize as isize);
    let mut anchor = istart;
    let prefixStartIndex = (*ms).window.dictLimit;
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    let dms = (*ms).dictMatchState;
    let dictCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const U32 = (*dms).hashTable;
    let dictStartIndex = (*dms).window.dictLimit;
    let dictBase = (*dms).window.base;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = (*dms).window.nextSrc;
    let dictIndexDelta = prefixStartIndex
        .wrapping_sub(dictEnd.offset_from(dictBase) as libc::c_long as U32);
    let dictAndPrefixLength = dictEnd
        .offset(istart.offset_from(prefixStart) as libc::c_long as isize)
        .offset_from(dictStart) as libc::c_long as U32;
    let dictHBits = ((*dictCParams).hashLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let maxDistance = (1 as libc::c_uint) << (*cParams).windowLog;
    let endIndex = (istart.offset_from(base) as libc::c_long as size_t)
        .wrapping_add(srcSize) as U32;
    if endIndex.wrapping_sub(prefixStartIndex) <= maxDistance {} else {
        __assert_fail(
            b"endIndex - prefixStartIndex <= maxDistance\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            494 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if prefixStartIndex >= dictEnd.offset_from(dictBase) as libc::c_long as U32 {} else {
        __assert_fail(
            b"prefixStartIndex >= (U32)(dictEnd - dictBase)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            501 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if (*ms).prefetchCDictTables != 0 {
        let hashTableBytes = ((1 as libc::c_int as size_t) << (*dictCParams).hashLog)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong);
        let _ptr = dictHashTable as *const libc::c_char;
        let _size = hashTableBytes;
        let mut _pos: size_t = 0;
        _pos = 0 as libc::c_int as size_t;
        while _pos < _size {
            _pos = (_pos as libc::c_ulong).wrapping_add(CACHELINE_SIZE as libc::c_ulong)
                as size_t as size_t;
        }
    }
    ip0 = ip0
        .offset(
            (dictAndPrefixLength == 0 as libc::c_int as libc::c_uint) as libc::c_int
                as isize,
        );
    if offset_1 <= dictAndPrefixLength {} else {
        __assert_fail(
            b"offset_1 <= dictAndPrefixLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            513 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if offset_2 <= dictAndPrefixLength {} else {
        __assert_fail(
            b"offset_2 <= dictAndPrefixLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            514 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if stepSize >= 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"stepSize >= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            517 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    's_126: while ip1 <= ilimit {
        let mut mLength: size_t = 0;
        let mut hash0 = ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls);
        let dictHashAndTag0 = ZSTD_hashPtr(ip0 as *const libc::c_void, dictHBits, mls);
        let mut dictMatchIndexAndTag = *dictHashTable
            .offset((dictHashAndTag0 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
        let mut dictTagsMatch = ZSTD_comparePackedTags(
            dictMatchIndexAndTag as size_t,
            dictHashAndTag0,
        );
        let mut matchIndex = *hashTable.offset(hash0 as isize);
        let mut curr = ip0.offset_from(base) as libc::c_long as U32;
        let mut step = stepSize as size_t;
        let kStepIncr = ((1 as libc::c_int) << kSearchStrength) as size_t;
        let mut nextStep = ip0.offset(kStepIncr as isize);
        loop {
            let mut match_0 = base.offset(matchIndex as isize);
            let repIndex = curr
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(offset_1);
            let mut repMatch = if repIndex < prefixStartIndex {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
            } else {
                base.offset(repIndex as isize)
            };
            let hash1 = ZSTD_hashPtr(ip1 as *const libc::c_void, hlog, mls);
            let dictHashAndTag1 = ZSTD_hashPtr(
                ip1 as *const libc::c_void,
                dictHBits,
                mls,
            );
            *hashTable.offset(hash0 as isize) = curr;
            if prefixStartIndex
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(repIndex) >= 3 as libc::c_int as libc::c_uint
                && MEM_read32(repMatch as *const libc::c_void)
                    == MEM_read32(
                        ip0.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    )
            {
                let repMatchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = (ZSTD_count_2segments(
                    ip0
                        .offset(1 as libc::c_int as isize)
                        .offset(4 as libc::c_int as isize),
                    repMatch.offset(4 as libc::c_int as isize),
                    iend,
                    repMatchEnd,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                ip0 = ip0.offset(1);
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        549 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 140],
                            &[libc::c_char; 140],
                        >(
                            b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        549 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 140],
                            &[libc::c_char; 140],
                        >(
                            b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ZSTD_storeSeq(
                    seqStore,
                    ip0.offset_from(anchor) as libc::c_long as size_t,
                    anchor,
                    iend,
                    1 as libc::c_int as U32,
                    mLength,
                );
                break;
            } else {
                if dictTagsMatch != 0 {
                    let dictMatchIndex = dictMatchIndexAndTag
                        >> ZSTD_SHORT_CACHE_TAG_BITS;
                    let mut dictMatch = dictBase.offset(dictMatchIndex as isize);
                    if dictMatchIndex > dictStartIndex
                        && MEM_read32(dictMatch as *const libc::c_void)
                            == MEM_read32(ip0 as *const libc::c_void)
                    {
                        if matchIndex <= prefixStartIndex {
                            let offset = curr
                                .wrapping_sub(dictMatchIndex)
                                .wrapping_sub(dictIndexDelta);
                            mLength = (ZSTD_count_2segments(
                                ip0.offset(4 as libc::c_int as isize),
                                dictMatch.offset(4 as libc::c_int as isize),
                                iend,
                                dictEnd,
                                prefixStart,
                            ))
                                .wrapping_add(4 as libc::c_int as libc::c_ulong);
                            while (ip0 > anchor) as libc::c_int
                                & (dictMatch > dictStart) as libc::c_int != 0
                                && *ip0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    == *dictMatch.offset(-(1 as libc::c_int) as isize)
                                        as libc::c_int
                            {
                                ip0 = ip0.offset(-1);
                                dictMatch = dictMatch.offset(-1);
                                mLength = mLength.wrapping_add(1);
                            }
                            offset_2 = offset_1;
                            offset_1 = offset;
                            if offset > 0 as libc::c_int as libc::c_uint {} else {
                                __assert_fail(
                                    b"(offset)>0\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                                        as *const u8 as *const libc::c_char,
                                    571 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 140],
                                        &[libc::c_char; 140],
                                    >(
                                        b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            ZSTD_storeSeq(
                                seqStore,
                                ip0.offset_from(anchor) as libc::c_long as size_t,
                                anchor,
                                iend,
                                offset.wrapping_add(ZSTD_REP_NUM as libc::c_uint),
                                mLength,
                            );
                            break;
                        }
                    }
                }
                if matchIndex > prefixStartIndex
                    && MEM_read32(match_0 as *const libc::c_void)
                        == MEM_read32(ip0 as *const libc::c_void)
                {
                    let offset_0 = ip0.offset_from(match_0) as libc::c_long as U32;
                    mLength = (ZSTD_count(
                        ip0.offset(4 as libc::c_int as isize),
                        match_0.offset(4 as libc::c_int as isize),
                        iend,
                    ))
                        .wrapping_add(4 as libc::c_int as libc::c_ulong);
                    while (ip0 > anchor) as libc::c_int
                        & (match_0 > prefixStart) as libc::c_int != 0
                        && *ip0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == *match_0.offset(-(1 as libc::c_int) as isize)
                                as libc::c_int
                    {
                        ip0 = ip0.offset(-1);
                        match_0 = match_0.offset(-1);
                        mLength = mLength.wrapping_add(1);
                    }
                    offset_2 = offset_1;
                    offset_1 = offset_0;
                    if offset_0 > 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"(offset)>0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                                as *const u8 as *const libc::c_char,
                            589 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 140],
                                &[libc::c_char; 140],
                            >(
                                b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    ZSTD_storeSeq(
                        seqStore,
                        ip0.offset_from(anchor) as libc::c_long as size_t,
                        anchor,
                        iend,
                        offset_0.wrapping_add(ZSTD_REP_NUM as libc::c_uint),
                        mLength,
                    );
                    break;
                } else {
                    dictMatchIndexAndTag = *dictHashTable
                        .offset((dictHashAndTag1 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
                    dictTagsMatch = ZSTD_comparePackedTags(
                        dictMatchIndexAndTag as size_t,
                        dictHashAndTag1,
                    );
                    matchIndex = *hashTable.offset(hash1 as isize);
                    if ip1 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.offset(kStepIncr as isize);
                    }
                    ip0 = ip1;
                    ip1 = ip1.offset(step as isize);
                    if ip1 > ilimit {
                        break 's_126;
                    }
                    curr = ip0.offset_from(base) as libc::c_long as U32;
                    hash0 = hash1;
                }
            }
        }
        if mLength != 0 {} else {
            __assert_fail(
                b"mLength\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                    as *const libc::c_char,
                611 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip0 <= ilimit {
            if base.offset(curr as isize).offset(2 as libc::c_int as isize) > istart
            {} else {
                __assert_fail(
                    b"base+curr+2 > istart\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    617 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 140],
                        &[libc::c_char; 140],
                    >(
                        b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        base.offset(curr as isize).offset(2 as libc::c_int as isize)
                            as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = curr.wrapping_add(2 as libc::c_int as libc::c_uint);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        ip0.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = ip0.offset(-(2 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            while ip0 <= ilimit {
                let current2 = ip0.offset_from(base) as libc::c_long as U32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let mut repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase
                        .offset(-(dictIndexDelta as isize))
                        .offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(prefixStartIndex
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(repIndex2) >= 3 as libc::c_int as libc::c_uint
                    && MEM_read32(repMatch2 as *const libc::c_void)
                        == MEM_read32(ip0 as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex { dictEnd } else { iend };
                let repLength2 = (ZSTD_count_2segments(
                    ip0.offset(4 as libc::c_int as isize),
                    repMatch2.offset(4 as libc::c_int as isize),
                    iend,
                    repEnd2,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                let mut tmpOffset = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        633 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 140],
                            &[libc::c_char; 140],
                        >(
                            b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        633 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 140],
                            &[libc::c_char; 140],
                        >(
                            b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
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
                    repLength2,
                );
                *hashTable
                    .offset(
                        ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls) as isize,
                    ) = current2;
                ip0 = ip0.offset(repLength2 as isize);
                anchor = ip0;
            }
        }
        if ip0 == anchor {} else {
            __assert_fail(
                b"ip0 == anchor\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                    as *const libc::c_char,
                644 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"size_t ZSTD_compressBlock_fast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        ip1 = ip0.offset(stepSize as isize);
    }
    *rep.offset(0 as libc::c_int as isize) = offset_1;
    *rep.offset(1 as libc::c_int as isize) = offset_2;
    return iend.offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_4_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_5_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_6_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_7_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    if !((*ms).dictMatchState).is_null() {} else {
        __assert_fail(
            b"ms->dictMatchState != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            668 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"size_t ZSTD_compressBlock_fast_dictMatchState(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    match mls {
        5 => {
            return ZSTD_compressBlock_fast_dictMatchState_5_0(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        6 => {
            return ZSTD_compressBlock_fast_dictMatchState_6_0(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        7 => {
            return ZSTD_compressBlock_fast_dictMatchState_7_0(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        4 | _ => {
            return ZSTD_compressBlock_fast_dictMatchState_4_0(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mls: U32,
    hasStep: U32,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let istart = src as *const BYTE;
    let mut anchor = istart;
    let endIndex = (istart.offset_from(base) as libc::c_long as size_t)
        .wrapping_add(srcSize) as U32;
    let lowLimit = ZSTD_getLowestMatchIndex(ms, endIndex, (*cParams).windowLog);
    let dictStartIndex = lowLimit;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictLimit = (*ms).window.dictLimit;
    let prefixStartIndex = if dictLimit < lowLimit { lowLimit } else { dictLimit };
    let prefixStart = base.offset(prefixStartIndex as isize);
    let dictEnd = dictBase.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(8 as libc::c_int as isize));
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    let mut offsetSaved1 = 0 as libc::c_int as U32;
    let mut offsetSaved2 = 0 as libc::c_int as U32;
    let mut ip0 = istart;
    let mut ip1 = 0 as *const BYTE;
    let mut ip2 = 0 as *const BYTE;
    let mut ip3 = 0 as *const BYTE;
    let mut current0: U32 = 0;
    let mut hash0: size_t = 0;
    let mut hash1: size_t = 0;
    let mut idx: U32 = 0;
    let mut idxBase = 0 as *const BYTE;
    let mut offcode: U32 = 0;
    let mut match0 = 0 as *const BYTE;
    let mut mLength: size_t = 0;
    let mut matchEnd = 0 as *const BYTE;
    let mut step: size_t = 0;
    let mut nextStep = 0 as *const BYTE;
    let kStepIncr = ((1 as libc::c_int) << kSearchStrength - 1 as libc::c_int) as size_t;
    if prefixStartIndex == dictStartIndex {
        return ZSTD_compressBlock_fast(ms, seqStore, rep, src, srcSize);
    }
    let curr = ip0.offset_from(base) as libc::c_long as U32;
    let maxRep = curr.wrapping_sub(dictStartIndex);
    if offset_2 >= maxRep {
        offsetSaved2 = offset_2;
        offset_2 = 0 as libc::c_int as U32;
    }
    if offset_1 >= maxRep {
        offsetSaved1 = offset_1;
        offset_1 = 0 as libc::c_int as U32;
    }
    '__start: loop {
        step = stepSize;
        nextStep = ip0.offset(kStepIncr as isize);
        ip1 = ip0.offset(1 as libc::c_int as isize);
        ip2 = ip0.offset(step as isize);
        ip3 = ip2.offset(1 as libc::c_int as isize);
        if ip3 >= ilimit {
            break;
        }
        hash0 = ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const libc::c_void, hlog, mls);
        idx = *hashTable.offset(hash0 as isize);
        idxBase = if idx < prefixStartIndex { dictBase } else { base };
        loop {
            let current2 = ip2.offset_from(base) as libc::c_long as U32;
            let repIndex = current2.wrapping_sub(offset_1);
            let repBase = if repIndex < prefixStartIndex { dictBase } else { base };
            let mut rval: U32 = 0;
            if (prefixStartIndex.wrapping_sub(repIndex)
                >= 4 as libc::c_int as libc::c_uint) as libc::c_int
                & (offset_1 > 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
            {
                rval = MEM_read32(
                    repBase.offset(repIndex as isize) as *const libc::c_void,
                );
            } else {
                rval = MEM_read32(ip2 as *const libc::c_void)
                    ^ 1 as libc::c_int as libc::c_uint;
            }
            current0 = ip0.offset_from(base) as libc::c_long as U32;
            *hashTable.offset(hash0 as isize) = current0;
            if MEM_read32(ip2 as *const libc::c_void) == rval {
                ip0 = ip2;
                match0 = repBase.offset(repIndex as isize);
                matchEnd = if repIndex < prefixStartIndex { dictEnd } else { iend };
                if (match0 != prefixStart) as libc::c_int
                    & (match0 != dictStart) as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"(match0 != prefixStart) & (match0 != dictStart)\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        788 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 133],
                            &[libc::c_char; 133],
                        >(
                            b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                mLength = (*ip0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == *match0.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    as libc::c_int as size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        792 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 133],
                            &[libc::c_char; 133],
                        >(
                            b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        792 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 133],
                            &[libc::c_char; 133],
                        >(
                            b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                offcode = 1 as libc::c_int as U32;
                mLength = (mLength as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
                current_block = 14866484591598397053;
                break;
            } else {
                let mval = if idx >= dictStartIndex {
                    MEM_read32(idxBase.offset(idx as isize) as *const libc::c_void)
                } else {
                    MEM_read32(ip0 as *const libc::c_void)
                        ^ 1 as libc::c_int as libc::c_uint
                };
                if MEM_read32(ip0 as *const libc::c_void) == mval {
                    current_block = 3062786670920734429;
                    break;
                } else {
                    idx = *hashTable.offset(hash1 as isize);
                    idxBase = if idx < prefixStartIndex { dictBase } else { base };
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const libc::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip3;
                    current0 = ip0.offset_from(base) as libc::c_long as U32;
                    *hashTable.offset(hash0 as isize) = current0;
                    let mval_0 = if idx >= dictStartIndex {
                        MEM_read32(idxBase.offset(idx as isize) as *const libc::c_void)
                    } else {
                        MEM_read32(ip0 as *const libc::c_void)
                            ^ 1 as libc::c_int as libc::c_uint
                    };
                    if MEM_read32(ip0 as *const libc::c_void) == mval_0 {
                        current_block = 3062786670920734429;
                        break;
                    }
                    idx = *hashTable.offset(hash1 as isize);
                    idxBase = if idx < prefixStartIndex { dictBase } else { base };
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const libc::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.offset(step as isize);
                    ip3 = ip1.offset(step as isize);
                    if ip2 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.offset(kStepIncr as isize);
                    }
                    if !(ip3 < ilimit) {
                        break '__start;
                    }
                }
            }
        }
        match current_block {
            3062786670920734429 => {
                let offset = current0.wrapping_sub(idx);
                let lowMatchPtr = if idx < prefixStartIndex {
                    dictStart
                } else {
                    prefixStart
                };
                matchEnd = if idx < prefixStartIndex { dictEnd } else { iend };
                match0 = idxBase.offset(idx as isize);
                offset_2 = offset_1;
                offset_1 = offset;
                if offset > 0 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"(offset)>0\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        884 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 133],
                            &[libc::c_char; 133],
                        >(
                            b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                offcode = offset.wrapping_add(ZSTD_REP_NUM as libc::c_uint);
                mLength = 4 as libc::c_int as size_t;
                while (ip0 > anchor) as libc::c_int
                    & (match0 > lowMatchPtr) as libc::c_int != 0
                    && *ip0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == *match0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                {
                    ip0 = ip0.offset(-1);
                    match0 = match0.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            }
            _ => {}
        }
        if !matchEnd.is_null() {} else {
            __assert_fail(
                b"matchEnd != 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                    as *const libc::c_char,
                897 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 133],
                    &[libc::c_char; 133],
                >(
                    b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                ))
                    .as_ptr(),
            );
        }
        mLength = (mLength as libc::c_ulong)
            .wrapping_add(
                ZSTD_count_2segments(
                    ip0.offset(mLength as isize),
                    match0.offset(mLength as isize),
                    iend,
                    matchEnd,
                    prefixStart,
                ),
            ) as size_t as size_t;
        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from(anchor) as libc::c_long as size_t,
            anchor,
            iend,
            offcode,
            mLength,
        );
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip1 < ip0 {
            *hashTable
                .offset(hash1 as isize) = ip1.offset_from(base) as libc::c_long as U32;
        }
        if ip0 <= ilimit {
            if base.offset(current0 as isize).offset(2 as libc::c_int as isize) > istart
            {} else {
                __assert_fail(
                    b"base+current0+2 > istart\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    913 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 133],
                        &[libc::c_char; 133],
                    >(
                        b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        base.offset(current0 as isize).offset(2 as libc::c_int as isize)
                            as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = current0.wrapping_add(2 as libc::c_int as libc::c_uint);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        ip0.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = ip0.offset(-(2 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            while ip0 <= ilimit {
                let repIndex2 = (ip0.offset_from(base) as libc::c_long as U32)
                    .wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !((prefixStartIndex
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(repIndex2) >= 3 as libc::c_int as libc::c_uint)
                    as libc::c_int
                    & (offset_2 > 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
                    && MEM_read32(repMatch2 as *const libc::c_void)
                        == MEM_read32(ip0 as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex { dictEnd } else { iend };
                let repLength2 = (ZSTD_count_2segments(
                    ip0.offset(4 as libc::c_int as isize),
                    repMatch2.offset(4 as libc::c_int as isize),
                    iend,
                    repEnd2,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                let tmpOffset = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        925 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 133],
                            &[libc::c_char; 133],
                        >(
                            b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        925 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 133],
                            &[libc::c_char; 133],
                        >(
                            b"size_t ZSTD_compressBlock_fast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32, const U32)\0",
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
                    repLength2,
                );
                *hashTable
                    .offset(
                        ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls) as isize,
                    ) = ip0.offset_from(base) as libc::c_long as U32;
                ip0 = ip0.offset(repLength2 as isize);
                anchor = ip0;
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
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_4_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_5_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_6_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_7_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
        0 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    if ((*ms).dictMatchState).is_null() {} else {
        __assert_fail(
            b"ms->dictMatchState == NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_fast.c\0" as *const u8
                as *const libc::c_char,
            947 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"size_t ZSTD_compressBlock_fast_extDict(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    match mls {
        5 => return ZSTD_compressBlock_fast_extDict_5_0(ms, seqStore, rep, src, srcSize),
        6 => return ZSTD_compressBlock_fast_extDict_6_0(ms, seqStore, rep, src, srcSize),
        7 => return ZSTD_compressBlock_fast_extDict_7_0(ms, seqStore, rep, src, srcSize),
        4 | _ => {
            return ZSTD_compressBlock_fast_extDict_4_0(ms, seqStore, rep, src, srcSize);
        }
    };
}
