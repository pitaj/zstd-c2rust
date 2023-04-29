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
pub const WILDCOPY_OVERLENGTH: libc::c_int = 32 as libc::c_int;
pub const CACHELINE_SIZE: libc::c_int = 64 as libc::c_int;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
pub const kSearchStrength: libc::c_int = 8 as libc::c_int;
pub const HASH_READ_SIZE: libc::c_int = 8 as libc::c_int;
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
static mut prime4bytes: U32 = 2654435761 as libc::c_uint;
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
#[inline]
unsafe extern "C" fn ZSTD_comparePackedTags(
    mut packedTag1: size_t,
    mut packedTag2: size_t,
) -> libc::c_int {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as U32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as U32;
    return (tag1 == tag2) as libc::c_int;
}
pub const ZSTD_SHORT_CACHE_TAG_MASK: libc::c_uint = ((1 as libc::c_uint)
    << ZSTD_SHORT_CACHE_TAG_BITS)
    .wrapping_sub(1 as libc::c_int as libc::c_uint);
unsafe extern "C" fn ZSTD_fillDoubleHashTableForCDict(
    mut ms: *mut ZSTD_matchState_t,
    mut end: *const libc::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLarge = (*ms).hashTable;
    let hBitsL = ((*cParams).hashLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let mls = (*cParams).minMatch;
    let hashSmall = (*ms).chainTable;
    let hBitsS = ((*cParams).chainLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const BYTE).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3 as libc::c_int as U32;
    while ip.offset(fastHashFillStep as isize).offset(-(1 as libc::c_int as isize))
        <= iend
    {
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let mut i: U32 = 0;
        i = 0 as libc::c_int as U32;
        while i < fastHashFillStep {
            let smHashAndTag = ZSTD_hashPtr(
                ip.offset(i as isize) as *const libc::c_void,
                hBitsS,
                mls,
            );
            let lgHashAndTag = ZSTD_hashPtr(
                ip.offset(i as isize) as *const libc::c_void,
                hBitsL,
                8 as libc::c_int as U32,
            );
            if i == 0 as libc::c_int as libc::c_uint {
                ZSTD_writeTaggedIndex(hashSmall, smHashAndTag, curr.wrapping_add(i));
            }
            if i == 0 as libc::c_int as libc::c_uint
                || *hashLarge
                    .offset((lgHashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS) as isize)
                    == 0 as libc::c_int as libc::c_uint
            {
                ZSTD_writeTaggedIndex(hashLarge, lgHashAndTag, curr.wrapping_add(i));
            }
            if dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint {
                break;
            }
            i = i.wrapping_add(1);
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
unsafe extern "C" fn ZSTD_fillDoubleHashTableForCCtx(
    mut ms: *mut ZSTD_matchState_t,
    mut end: *const libc::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLarge = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let mls = (*cParams).minMatch;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const BYTE).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3 as libc::c_int as U32;
    while ip.offset(fastHashFillStep as isize).offset(-(1 as libc::c_int as isize))
        <= iend
    {
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let mut i: U32 = 0;
        i = 0 as libc::c_int as U32;
        while i < fastHashFillStep {
            let smHash = ZSTD_hashPtr(
                ip.offset(i as isize) as *const libc::c_void,
                hBitsS,
                mls,
            );
            let lgHash = ZSTD_hashPtr(
                ip.offset(i as isize) as *const libc::c_void,
                hBitsL,
                8 as libc::c_int as U32,
            );
            if i == 0 as libc::c_int as libc::c_uint {
                *hashSmall.offset(smHash as isize) = curr.wrapping_add(i);
            }
            if i == 0 as libc::c_int as libc::c_uint
                || *hashLarge.offset(lgHash as isize) == 0 as libc::c_int as libc::c_uint
            {
                *hashLarge.offset(lgHash as isize) = curr.wrapping_add(i);
            }
            if dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint {
                break;
            }
            i = i.wrapping_add(1);
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_fillDoubleHashTable(
    mut ms: *mut ZSTD_matchState_t,
    end: *const libc::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
) {
    if tfp as libc::c_uint == ZSTD_tfp_forCDict as libc::c_int as libc::c_uint {
        ZSTD_fillDoubleHashTableForCDict(ms, end, dtlm);
    } else {
        ZSTD_fillDoubleHashTableForCCtx(ms, end, dtlm);
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_noDict_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mls: U32,
) -> size_t {
    let mut current_block: u64;
    let mut cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLong = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let base = (*ms).window.base;
    let istart = src as *const BYTE;
    let mut anchor = istart;
    let endIndex = (istart.offset_from(base) as libc::c_long as size_t)
        .wrapping_add(srcSize) as U32;
    let prefixLowestIndex = ZSTD_getLowestPrefixIndex(
        ms,
        endIndex,
        (*cParams).windowLog,
    );
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    let mut offsetSaved1 = 0 as libc::c_int as U32;
    let mut offsetSaved2 = 0 as libc::c_int as U32;
    let mut mLength: size_t = 0;
    let mut offset: U32 = 0;
    let mut curr: U32 = 0;
    let kStepIncr = ((1 as libc::c_int) << kSearchStrength) as size_t;
    let mut nextStep = 0 as *const BYTE;
    let mut step: size_t = 0;
    let mut hl0: size_t = 0;
    let mut hl1: size_t = 0;
    let mut idxl0: U32 = 0;
    let mut idxl1: U32 = 0;
    let mut matchl0 = 0 as *const BYTE;
    let mut matchs0 = 0 as *const BYTE;
    let mut matchl1 = 0 as *const BYTE;
    let mut ip = istart;
    let mut ip1 = 0 as *const BYTE;
    ip = ip
        .offset(
            (ip.offset_from(prefixLowest) as libc::c_long
                == 0 as libc::c_int as libc::c_long) as libc::c_int as isize,
        );
    let current = ip.offset_from(base) as libc::c_long as U32;
    let windowLow = ZSTD_getLowestPrefixIndex(ms, current, (*cParams).windowLog);
    let maxRep = current.wrapping_sub(windowLow);
    if offset_2 > maxRep {
        offsetSaved2 = offset_2;
        offset_2 = 0 as libc::c_int as U32;
    }
    if offset_1 > maxRep {
        offsetSaved1 = offset_1;
        offset_1 = 0 as libc::c_int as U32;
    }
    loop {
        step = 1 as libc::c_int as size_t;
        nextStep = ip.offset(kStepIncr as isize);
        ip1 = ip.offset(step as isize);
        if !(ip1 > ilimit) {
            hl0 = ZSTD_hashPtr(
                ip as *const libc::c_void,
                hBitsL,
                8 as libc::c_int as U32,
            );
            idxl0 = *hashLong.offset(hl0 as isize);
            matchl0 = base.offset(idxl0 as isize);
            loop {
                let hs0 = ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls);
                let idxs0 = *hashSmall.offset(hs0 as isize);
                curr = ip.offset_from(base) as libc::c_long as U32;
                matchs0 = base.offset(idxs0 as isize);
                let ref mut fresh2 = *hashSmall.offset(hs0 as isize);
                *fresh2 = curr;
                *hashLong.offset(hl0 as isize) = *fresh2;
                if (offset_1 > 0 as libc::c_int as libc::c_uint) as libc::c_int
                    & (MEM_read32(
                        ip.offset(1 as libc::c_int as isize).offset(-(offset_1 as isize))
                            as *const libc::c_void,
                    )
                        == MEM_read32(
                            ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        )) as libc::c_int != 0
                {
                    mLength = (ZSTD_count(
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
                    ip = ip.offset(1);
                    if 1 as libc::c_int >= 1 as libc::c_int {} else {
                        __assert_fail(
                            b"(1)>=1\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                                as *const u8 as *const libc::c_char,
                            181 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 127],
                                &[libc::c_char; 127],
                            >(
                                b"size_t ZSTD_compressBlock_doubleFast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if 1 as libc::c_int <= 3 as libc::c_int {} else {
                        __assert_fail(
                            b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                                as *const u8 as *const libc::c_char,
                            181 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 127],
                                &[libc::c_char; 127],
                            >(
                                b"size_t ZSTD_compressBlock_doubleFast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    ZSTD_storeSeq(
                        seqStore,
                        ip.offset_from(anchor) as libc::c_long as size_t,
                        anchor,
                        iend,
                        1 as libc::c_int as U32,
                        mLength,
                    );
                    current_block = 7055686759104843839;
                    break;
                } else {
                    hl1 = ZSTD_hashPtr(
                        ip1 as *const libc::c_void,
                        hBitsL,
                        8 as libc::c_int as U32,
                    );
                    if idxl0 > prefixLowestIndex {
                        if MEM_read64(matchl0 as *const libc::c_void)
                            == MEM_read64(ip as *const libc::c_void)
                        {
                            mLength = (ZSTD_count(
                                ip.offset(8 as libc::c_int as isize),
                                matchl0.offset(8 as libc::c_int as isize),
                                iend,
                            ))
                                .wrapping_add(8 as libc::c_int as libc::c_ulong);
                            offset = ip.offset_from(matchl0) as libc::c_long as U32;
                            while (ip > anchor) as libc::c_int
                                & (matchl0 > prefixLowest) as libc::c_int != 0
                                && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    == *matchl0.offset(-(1 as libc::c_int) as isize)
                                        as libc::c_int
                            {
                                ip = ip.offset(-1);
                                matchl0 = matchl0.offset(-1);
                                mLength = mLength.wrapping_add(1);
                            }
                            current_block = 1549319127794666967;
                            break;
                        }
                    }
                    idxl1 = *hashLong.offset(hl1 as isize);
                    matchl1 = base.offset(idxl1 as isize);
                    if idxs0 > prefixLowestIndex {
                        if MEM_read32(matchs0 as *const libc::c_void)
                            == MEM_read32(ip as *const libc::c_void)
                        {
                            current_block = 10227688948194353775;
                            break;
                        }
                    }
                    if ip1 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.offset(kStepIncr as isize);
                    }
                    ip = ip1;
                    ip1 = ip1.offset(step as isize);
                    hl0 = hl1;
                    idxl0 = idxl1;
                    matchl0 = matchl1;
                    if !(ip1 <= ilimit) {
                        current_block = 17028889961375082305;
                        break;
                    }
                }
            }
            match current_block {
                17028889961375082305 => {}
                _ => {
                    match current_block {
                        10227688948194353775 => {
                            if idxl1 > prefixLowestIndex {
                                if MEM_read64(matchl1 as *const libc::c_void)
                                    == MEM_read64(ip1 as *const libc::c_void)
                                {
                                    ip = ip1;
                                    mLength = (ZSTD_count(
                                        ip.offset(8 as libc::c_int as isize),
                                        matchl1.offset(8 as libc::c_int as isize),
                                        iend,
                                    ))
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong);
                                    offset = ip.offset_from(matchl1) as libc::c_long as U32;
                                    while (ip > anchor) as libc::c_int
                                        & (matchl1 > prefixLowest) as libc::c_int != 0
                                        && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                            == *matchl1.offset(-(1 as libc::c_int) as isize)
                                                as libc::c_int
                                    {
                                        ip = ip.offset(-1);
                                        matchl1 = matchl1.offset(-1);
                                        mLength = mLength.wrapping_add(1);
                                    }
                                    current_block = 1549319127794666967;
                                } else {
                                    current_block = 14001958660280927786;
                                }
                            } else {
                                current_block = 14001958660280927786;
                            }
                            match current_block {
                                1549319127794666967 => {}
                                _ => {
                                    mLength = (ZSTD_count(
                                        ip.offset(4 as libc::c_int as isize),
                                        matchs0.offset(4 as libc::c_int as isize),
                                        iend,
                                    ))
                                        .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                    offset = ip.offset_from(matchs0) as libc::c_long as U32;
                                    while (ip > anchor) as libc::c_int
                                        & (matchs0 > prefixLowest) as libc::c_int != 0
                                        && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                            == *matchs0.offset(-(1 as libc::c_int) as isize)
                                                as libc::c_int
                                    {
                                        ip = ip.offset(-1);
                                        matchs0 = matchs0.offset(-1);
                                        mLength = mLength.wrapping_add(1);
                                    }
                                    current_block = 1549319127794666967;
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        1549319127794666967 => {
                            offset_2 = offset_1;
                            offset_1 = offset;
                            if step < 4 as libc::c_int as libc::c_ulong {
                                *hashLong
                                    .offset(
                                        hl1 as isize,
                                    ) = ip1.offset_from(base) as libc::c_long as U32;
                            }
                            if offset > 0 as libc::c_int as libc::c_uint {} else {
                                __assert_fail(
                                    b"(offset)>0\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                                        as *const u8 as *const libc::c_char,
                                    271 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 127],
                                        &[libc::c_char; 127],
                                    >(
                                        b"size_t ZSTD_compressBlock_doubleFast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            ZSTD_storeSeq(
                                seqStore,
                                ip.offset_from(anchor) as libc::c_long as size_t,
                                anchor,
                                iend,
                                offset.wrapping_add(ZSTD_REP_NUM as libc::c_uint),
                                mLength,
                            );
                        }
                        _ => {}
                    }
                    ip = ip.offset(mLength as isize);
                    anchor = ip;
                    if ip <= ilimit {
                        let indexToInsert = curr
                            .wrapping_add(2 as libc::c_int as libc::c_uint);
                        *hashLong
                            .offset(
                                ZSTD_hashPtr(
                                    base.offset(indexToInsert as isize) as *const libc::c_void,
                                    hBitsL,
                                    8 as libc::c_int as U32,
                                ) as isize,
                            ) = indexToInsert;
                        *hashLong
                            .offset(
                                ZSTD_hashPtr(
                                    ip.offset(-(2 as libc::c_int as isize))
                                        as *const libc::c_void,
                                    hBitsL,
                                    8 as libc::c_int as U32,
                                ) as isize,
                            ) = ip.offset(-(2 as libc::c_int as isize)).offset_from(base)
                            as libc::c_long as U32;
                        *hashSmall
                            .offset(
                                ZSTD_hashPtr(
                                    base.offset(indexToInsert as isize) as *const libc::c_void,
                                    hBitsS,
                                    mls,
                                ) as isize,
                            ) = indexToInsert;
                        *hashSmall
                            .offset(
                                ZSTD_hashPtr(
                                    ip.offset(-(1 as libc::c_int as isize))
                                        as *const libc::c_void,
                                    hBitsS,
                                    mls,
                                ) as isize,
                            ) = ip.offset(-(1 as libc::c_int as isize)).offset_from(base)
                            as libc::c_long as U32;
                        while ip <= ilimit
                            && (offset_2 > 0 as libc::c_int as libc::c_uint)
                                as libc::c_int
                                & (MEM_read32(ip as *const libc::c_void)
                                    == MEM_read32(
                                        ip.offset(-(offset_2 as isize)) as *const libc::c_void,
                                    )) as libc::c_int != 0
                        {
                            let rLength = (ZSTD_count(
                                ip.offset(4 as libc::c_int as isize),
                                ip
                                    .offset(4 as libc::c_int as isize)
                                    .offset(-(offset_2 as isize)),
                                iend,
                            ))
                                .wrapping_add(4 as libc::c_int as libc::c_ulong);
                            let tmpOff = offset_2;
                            offset_2 = offset_1;
                            offset_1 = tmpOff;
                            *hashSmall
                                .offset(
                                    ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls)
                                        as isize,
                                ) = ip.offset_from(base) as libc::c_long as U32;
                            *hashLong
                                .offset(
                                    ZSTD_hashPtr(
                                        ip as *const libc::c_void,
                                        hBitsL,
                                        8 as libc::c_int as U32,
                                    ) as isize,
                                ) = ip.offset_from(base) as libc::c_long as U32;
                            if 1 as libc::c_int >= 1 as libc::c_int {} else {
                                __assert_fail(
                                    b"(1)>=1\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                                        as *const u8 as *const libc::c_char,
                                    297 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 127],
                                        &[libc::c_char; 127],
                                    >(
                                        b"size_t ZSTD_compressBlock_doubleFast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if 1 as libc::c_int <= 3 as libc::c_int {} else {
                                __assert_fail(
                                    b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                                        as *const u8 as *const libc::c_char,
                                    297 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 127],
                                        &[libc::c_char; 127],
                                    >(
                                        b"size_t ZSTD_compressBlock_doubleFast_noDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
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
                            ip = ip.offset(rLength as isize);
                            anchor = ip;
                        }
                    }
                    continue;
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
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mls: U32,
) -> size_t {
    let mut current_block: u64;
    let mut cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLong = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let base = (*ms).window.base;
    let istart = src as *const BYTE;
    let mut ip = istart;
    let mut anchor = istart;
    let endIndex = (istart.offset_from(base) as libc::c_long as size_t)
        .wrapping_add(srcSize) as U32;
    let prefixLowestIndex = ZSTD_getLowestPrefixIndex(
        ms,
        endIndex,
        (*cParams).windowLog,
    );
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    let dms = (*ms).dictMatchState;
    let dictCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashLong: *const U32 = (*dms).hashTable;
    let dictHashSmall: *const U32 = (*dms).chainTable;
    let dictStartIndex = (*dms).window.dictLimit;
    let dictBase = (*dms).window.base;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = (*dms).window.nextSrc;
    let dictIndexDelta = prefixLowestIndex
        .wrapping_sub(dictEnd.offset_from(dictBase) as libc::c_long as U32);
    let dictHBitsL = ((*dictCParams).hashLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let dictHBitsS = ((*dictCParams).chainLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let dictAndPrefixLength = (ip.offset_from(prefixLowest) as libc::c_long
        + dictEnd.offset_from(dictStart) as libc::c_long) as U32;
    if ((*ms).window.dictLimit).wrapping_add((1 as libc::c_uint) << (*cParams).windowLog)
        >= endIndex
    {} else {
        __assert_fail(
            b"ms->window.dictLimit + (1U << cParams->windowLog) >= endIndex\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0" as *const u8
                as *const libc::c_char,
            346 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if (*ms).prefetchCDictTables != 0 {
        let hashTableBytes = ((1 as libc::c_int as size_t) << (*dictCParams).hashLog)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong);
        let chainTableBytes = ((1 as libc::c_int as size_t) << (*dictCParams).chainLog)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong);
        let _ptr = dictHashLong as *const libc::c_char;
        let _size = hashTableBytes;
        let mut _pos: size_t = 0;
        _pos = 0 as libc::c_int as size_t;
        while _pos < _size {
            _pos = (_pos as libc::c_ulong).wrapping_add(CACHELINE_SIZE as libc::c_ulong)
                as size_t as size_t;
        }
        let _ptr_0 = dictHashSmall as *const libc::c_char;
        let _size_0 = chainTableBytes;
        let mut _pos_0: size_t = 0;
        _pos_0 = 0 as libc::c_int as size_t;
        while _pos_0 < _size_0 {
            _pos_0 = (_pos_0 as libc::c_ulong)
                .wrapping_add(CACHELINE_SIZE as libc::c_ulong) as size_t as size_t;
        }
    }
    ip = ip
        .offset(
            (dictAndPrefixLength == 0 as libc::c_int as libc::c_uint) as libc::c_int
                as isize,
        );
    if offset_1 <= dictAndPrefixLength {} else {
        __assert_fail(
            b"offset_1 <= dictAndPrefixLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0" as *const u8
                as *const libc::c_char,
            360 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    if offset_2 <= dictAndPrefixLength {} else {
        __assert_fail(
            b"offset_2 <= dictAndPrefixLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0" as *const u8
                as *const libc::c_char,
            361 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
            ))
                .as_ptr(),
        );
    }
    while ip < ilimit {
        let mut mLength: size_t = 0;
        let mut offset: U32 = 0;
        let h2 = ZSTD_hashPtr(
            ip as *const libc::c_void,
            hBitsL,
            8 as libc::c_int as U32,
        );
        let h = ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls);
        let dictHashAndTagL = ZSTD_hashPtr(
            ip as *const libc::c_void,
            dictHBitsL,
            8 as libc::c_int as U32,
        );
        let dictHashAndTagS = ZSTD_hashPtr(ip as *const libc::c_void, dictHBitsS, mls);
        let dictMatchIndexAndTagL = *dictHashLong
            .offset((dictHashAndTagL >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
        let dictMatchIndexAndTagS = *dictHashSmall
            .offset((dictHashAndTagS >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
        let dictTagsMatchL = ZSTD_comparePackedTags(
            dictMatchIndexAndTagL as size_t,
            dictHashAndTagL,
        );
        let dictTagsMatchS = ZSTD_comparePackedTags(
            dictMatchIndexAndTagS as size_t,
            dictHashAndTagS,
        );
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let matchIndexL = *hashLong.offset(h2 as isize);
        let mut matchIndexS = *hashSmall.offset(h as isize);
        let mut matchLong = base.offset(matchIndexL as isize);
        let mut match_0 = base.offset(matchIndexS as isize);
        let repIndex = curr
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(offset_1);
        let mut repMatch = if repIndex < prefixLowestIndex {
            dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
        } else {
            base.offset(repIndex as isize)
        };
        let ref mut fresh3 = *hashSmall.offset(h as isize);
        *fresh3 = curr;
        *hashLong.offset(h2 as isize) = *fresh3;
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
            mLength = (ZSTD_count_2segments(
                ip.offset(1 as libc::c_int as isize).offset(4 as libc::c_int as isize),
                repMatch.offset(4 as libc::c_int as isize),
                iend,
                repMatchEnd,
                prefixLowest,
            ))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            ip = ip.offset(1);
            if 1 as libc::c_int >= 1 as libc::c_int {} else {
                __assert_fail(
                    b"(1)>=1\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    392 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 135],
                        &[libc::c_char; 135],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if 1 as libc::c_int <= 3 as libc::c_int {} else {
                __assert_fail(
                    b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    392 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 135],
                        &[libc::c_char; 135],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as libc::c_long as size_t,
                anchor,
                iend,
                1 as libc::c_int as U32,
                mLength,
            );
        } else {
            if matchIndexL > prefixLowestIndex {
                if MEM_read64(matchLong as *const libc::c_void)
                    == MEM_read64(ip as *const libc::c_void)
                {
                    mLength = (ZSTD_count(
                        ip.offset(8 as libc::c_int as isize),
                        matchLong.offset(8 as libc::c_int as isize),
                        iend,
                    ))
                        .wrapping_add(8 as libc::c_int as libc::c_ulong);
                    offset = ip.offset_from(matchLong) as libc::c_long as U32;
                    while (ip > anchor) as libc::c_int
                        & (matchLong > prefixLowest) as libc::c_int != 0
                        && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == *matchLong.offset(-(1 as libc::c_int) as isize)
                                as libc::c_int
                    {
                        ip = ip.offset(-1);
                        matchLong = matchLong.offset(-1);
                        mLength = mLength.wrapping_add(1);
                    }
                    current_block = 7636448771861776548;
                } else {
                    current_block = 6072622540298447352;
                }
            } else if dictTagsMatchL != 0 {
                let dictMatchIndexL = dictMatchIndexAndTagL >> ZSTD_SHORT_CACHE_TAG_BITS;
                let mut dictMatchL = dictBase.offset(dictMatchIndexL as isize);
                if dictMatchL < dictEnd {} else {
                    __assert_fail(
                        b"dictMatchL < dictEnd\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        408 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 135],
                            &[libc::c_char; 135],
                        >(
                            b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if dictMatchL > dictStart
                    && MEM_read64(dictMatchL as *const libc::c_void)
                        == MEM_read64(ip as *const libc::c_void)
                {
                    mLength = (ZSTD_count_2segments(
                        ip.offset(8 as libc::c_int as isize),
                        dictMatchL.offset(8 as libc::c_int as isize),
                        iend,
                        dictEnd,
                        prefixLowest,
                    ))
                        .wrapping_add(8 as libc::c_int as libc::c_ulong);
                    offset = curr
                        .wrapping_sub(dictMatchIndexL)
                        .wrapping_sub(dictIndexDelta);
                    while (ip > anchor) as libc::c_int
                        & (dictMatchL > dictStart) as libc::c_int != 0
                        && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == *dictMatchL.offset(-(1 as libc::c_int) as isize)
                                as libc::c_int
                    {
                        ip = ip.offset(-1);
                        dictMatchL = dictMatchL.offset(-1);
                        mLength = mLength.wrapping_add(1);
                    }
                    current_block = 7636448771861776548;
                } else {
                    current_block = 6072622540298447352;
                }
            } else {
                current_block = 6072622540298447352;
            }
            match current_block {
                6072622540298447352 => {
                    if matchIndexS > prefixLowestIndex {
                        if MEM_read32(match_0 as *const libc::c_void)
                            == MEM_read32(ip as *const libc::c_void)
                        {
                            current_block = 6243635450180130569;
                        } else {
                            current_block = 15594839951440953787;
                        }
                    } else if dictTagsMatchS != 0 {
                        let dictMatchIndexS = dictMatchIndexAndTagS
                            >> ZSTD_SHORT_CACHE_TAG_BITS;
                        match_0 = dictBase.offset(dictMatchIndexS as isize);
                        matchIndexS = dictMatchIndexS.wrapping_add(dictIndexDelta);
                        if match_0 > dictStart
                            && MEM_read32(match_0 as *const libc::c_void)
                                == MEM_read32(ip as *const libc::c_void)
                        {
                            current_block = 6243635450180130569;
                        } else {
                            current_block = 15594839951440953787;
                        }
                    } else {
                        current_block = 15594839951440953787;
                    }
                    match current_block {
                        15594839951440953787 => {
                            ip = ip
                                .offset(
                                    ((ip.offset_from(anchor) as libc::c_long >> kSearchStrength)
                                        + 1 as libc::c_int as libc::c_long) as isize,
                                );
                            continue;
                        }
                        _ => {
                            let hl3 = ZSTD_hashPtr(
                                ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                hBitsL,
                                8 as libc::c_int as U32,
                            );
                            let dictHashAndTagL3 = ZSTD_hashPtr(
                                ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                dictHBitsL,
                                8 as libc::c_int as U32,
                            );
                            let matchIndexL3 = *hashLong.offset(hl3 as isize);
                            let dictMatchIndexAndTagL3 = *dictHashLong
                                .offset(
                                    (dictHashAndTagL3 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize,
                                );
                            let dictTagsMatchL3 = ZSTD_comparePackedTags(
                                dictMatchIndexAndTagL3 as size_t,
                                dictHashAndTagL3,
                            );
                            let mut matchL3 = base.offset(matchIndexL3 as isize);
                            *hashLong
                                .offset(
                                    hl3 as isize,
                                ) = curr.wrapping_add(1 as libc::c_int as libc::c_uint);
                            if matchIndexL3 > prefixLowestIndex {
                                if MEM_read64(matchL3 as *const libc::c_void)
                                    == MEM_read64(
                                        ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                    )
                                {
                                    mLength = (ZSTD_count(
                                        ip.offset(9 as libc::c_int as isize),
                                        matchL3.offset(8 as libc::c_int as isize),
                                        iend,
                                    ))
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong);
                                    ip = ip.offset(1);
                                    offset = ip.offset_from(matchL3) as libc::c_long as U32;
                                    while (ip > anchor) as libc::c_int
                                        & (matchL3 > prefixLowest) as libc::c_int != 0
                                        && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                            == *matchL3.offset(-(1 as libc::c_int) as isize)
                                                as libc::c_int
                                    {
                                        ip = ip.offset(-1);
                                        matchL3 = matchL3.offset(-1);
                                        mLength = mLength.wrapping_add(1);
                                    }
                                    current_block = 7636448771861776548;
                                } else {
                                    current_block = 10859911333150192671;
                                }
                            } else if dictTagsMatchL3 != 0 {
                                let dictMatchIndexL3 = dictMatchIndexAndTagL3
                                    >> ZSTD_SHORT_CACHE_TAG_BITS;
                                let mut dictMatchL3 = dictBase
                                    .offset(dictMatchIndexL3 as isize);
                                if dictMatchL3 < dictEnd {} else {
                                    __assert_fail(
                                        b"dictMatchL3 < dictEnd\0" as *const u8
                                            as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                                            as *const u8 as *const libc::c_char,
                                        460 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 135],
                                            &[libc::c_char; 135],
                                        >(
                                            b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                if dictMatchL3 > dictStart
                                    && MEM_read64(dictMatchL3 as *const libc::c_void)
                                        == MEM_read64(
                                            ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        )
                                {
                                    mLength = (ZSTD_count_2segments(
                                        ip
                                            .offset(1 as libc::c_int as isize)
                                            .offset(8 as libc::c_int as isize),
                                        dictMatchL3.offset(8 as libc::c_int as isize),
                                        iend,
                                        dictEnd,
                                        prefixLowest,
                                    ))
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong);
                                    ip = ip.offset(1);
                                    offset = curr
                                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(dictMatchIndexL3)
                                        .wrapping_sub(dictIndexDelta);
                                    while (ip > anchor) as libc::c_int
                                        & (dictMatchL3 > dictStart) as libc::c_int != 0
                                        && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                            == *dictMatchL3.offset(-(1 as libc::c_int) as isize)
                                                as libc::c_int
                                    {
                                        ip = ip.offset(-1);
                                        dictMatchL3 = dictMatchL3.offset(-1);
                                        mLength = mLength.wrapping_add(1);
                                    }
                                    current_block = 7636448771861776548;
                                } else {
                                    current_block = 10859911333150192671;
                                }
                            } else {
                                current_block = 10859911333150192671;
                            }
                            match current_block {
                                7636448771861776548 => {}
                                _ => {
                                    if matchIndexS < prefixLowestIndex {
                                        mLength = (ZSTD_count_2segments(
                                            ip.offset(4 as libc::c_int as isize),
                                            match_0.offset(4 as libc::c_int as isize),
                                            iend,
                                            dictEnd,
                                            prefixLowest,
                                        ))
                                            .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                        offset = curr.wrapping_sub(matchIndexS);
                                        while (ip > anchor) as libc::c_int
                                            & (match_0 > dictStart) as libc::c_int != 0
                                            && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == *match_0.offset(-(1 as libc::c_int) as isize)
                                                    as libc::c_int
                                        {
                                            ip = ip.offset(-1);
                                            match_0 = match_0.offset(-1);
                                            mLength = mLength.wrapping_add(1);
                                        }
                                    } else {
                                        mLength = (ZSTD_count(
                                            ip.offset(4 as libc::c_int as isize),
                                            match_0.offset(4 as libc::c_int as isize),
                                            iend,
                                        ))
                                            .wrapping_add(4 as libc::c_int as libc::c_ulong);
                                        offset = ip.offset_from(match_0) as libc::c_long as U32;
                                        while (ip > anchor) as libc::c_int
                                            & (match_0 > prefixLowest) as libc::c_int != 0
                                            && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == *match_0.offset(-(1 as libc::c_int) as isize)
                                                    as libc::c_int
                                        {
                                            ip = ip.offset(-1);
                                            match_0 = match_0.offset(-1);
                                            mLength = mLength.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            offset_2 = offset_1;
            offset_1 = offset;
            if offset > 0 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"(offset)>0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    484 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 135],
                        &[libc::c_char; 135],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as libc::c_long as size_t,
                anchor,
                iend,
                offset.wrapping_add(ZSTD_REP_NUM as libc::c_uint),
                mLength,
            );
        }
        ip = ip.offset(mLength as isize);
        anchor = ip;
        if ip <= ilimit {
            let indexToInsert = curr.wrapping_add(2 as libc::c_int as libc::c_uint);
            *hashLong
                .offset(
                    ZSTD_hashPtr(
                        base.offset(indexToInsert as isize) as *const libc::c_void,
                        hBitsL,
                        8 as libc::c_int as U32,
                    ) as isize,
                ) = indexToInsert;
            *hashLong
                .offset(
                    ZSTD_hashPtr(
                        ip.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                        hBitsL,
                        8 as libc::c_int as U32,
                    ) as isize,
                ) = ip.offset(-(2 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            *hashSmall
                .offset(
                    ZSTD_hashPtr(
                        base.offset(indexToInsert as isize) as *const libc::c_void,
                        hBitsS,
                        mls,
                    ) as isize,
                ) = indexToInsert;
            *hashSmall
                .offset(
                    ZSTD_hashPtr(
                        ip.offset(-(1 as libc::c_int as isize)) as *const libc::c_void,
                        hBitsS,
                        mls,
                    ) as isize,
                ) = ip.offset(-(1 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as libc::c_long as U32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let mut repMatch2 = if repIndex2 < prefixLowestIndex {
                    dictBase
                        .offset(repIndex2 as isize)
                        .offset(-(dictIndexDelta as isize))
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(prefixLowestIndex
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(repIndex2) >= 3 as libc::c_int as libc::c_uint
                    && MEM_read32(repMatch2 as *const libc::c_void)
                        == MEM_read32(ip as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixLowestIndex { dictEnd } else { iend };
                let repLength2 = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    repMatch2.offset(4 as libc::c_int as isize),
                    iend,
                    repEnd2,
                    prefixLowest,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                let mut tmpOffset = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                if 1 as libc::c_int >= 1 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)>=1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        513 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 135],
                            &[libc::c_char; 135],
                        >(
                            b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        513 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 135],
                            &[libc::c_char; 135],
                        >(
                            b"size_t ZSTD_compressBlock_doubleFast_dictMatchState_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
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
                *hashSmall
                    .offset(
                        ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls) as isize,
                    ) = current2;
                *hashLong
                    .offset(
                        ZSTD_hashPtr(
                            ip as *const libc::c_void,
                            hBitsL,
                            8 as libc::c_int as U32,
                        ) as isize,
                    ) = current2;
                ip = ip.offset(repLength2 as isize);
                anchor = ip;
            }
        }
    }
    *rep.offset(0 as libc::c_int as isize) = offset_1;
    *rep.offset(1 as libc::c_int as isize) = offset_2;
    return iend.offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_noDict_4(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_noDict_5(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_noDict_6(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_noDict_7(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState_4(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState_5(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState_6(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState_7(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_doubleFast(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => {
            return ZSTD_compressBlock_doubleFast_noDict_5(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        6 => {
            return ZSTD_compressBlock_doubleFast_noDict_6(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        7 => {
            return ZSTD_compressBlock_doubleFast_noDict_7(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        4 | _ => {
            return ZSTD_compressBlock_doubleFast_noDict_4(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => {
            return ZSTD_compressBlock_doubleFast_dictMatchState_5(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        6 => {
            return ZSTD_compressBlock_doubleFast_dictMatchState_6(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        7 => {
            return ZSTD_compressBlock_doubleFast_dictMatchState_7(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        4 | _ => {
            return ZSTD_compressBlock_doubleFast_dictMatchState_4(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mls: U32,
) -> size_t {
    let mut cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLong = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let istart = src as *const BYTE;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(8 as libc::c_int as isize));
    let base = (*ms).window.base;
    let endIndex = (istart.offset_from(base) as libc::c_long as size_t)
        .wrapping_add(srcSize) as U32;
    let lowLimit = ZSTD_getLowestMatchIndex(ms, endIndex, (*cParams).windowLog);
    let dictStartIndex = lowLimit;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStartIndex = if dictLimit > lowLimit { dictLimit } else { lowLimit };
    let prefixStart = base.offset(prefixStartIndex as isize);
    let dictBase = (*ms).window.dictBase;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = dictBase.offset(prefixStartIndex as isize);
    let mut offset_1 = *rep.offset(0 as libc::c_int as isize);
    let mut offset_2 = *rep.offset(1 as libc::c_int as isize);
    if prefixStartIndex == dictStartIndex {
        return ZSTD_compressBlock_doubleFast(ms, seqStore, rep, src, srcSize);
    }
    while ip < ilimit {
        let hSmall = ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls);
        let matchIndex = *hashSmall.offset(hSmall as isize);
        let matchBase = if matchIndex < prefixStartIndex { dictBase } else { base };
        let mut match_0 = matchBase.offset(matchIndex as isize);
        let hLong = ZSTD_hashPtr(
            ip as *const libc::c_void,
            hBitsL,
            8 as libc::c_int as U32,
        );
        let matchLongIndex = *hashLong.offset(hLong as isize);
        let matchLongBase = if matchLongIndex < prefixStartIndex {
            dictBase
        } else {
            base
        };
        let mut matchLong = matchLongBase.offset(matchLongIndex as isize);
        let curr = ip.offset_from(base) as libc::c_long as U32;
        let repIndex = curr
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(offset_1);
        let repBase = if repIndex < prefixStartIndex { dictBase } else { base };
        let repMatch = repBase.offset(repIndex as isize);
        let mut mLength: size_t = 0;
        let ref mut fresh4 = *hashLong.offset(hLong as isize);
        *fresh4 = curr;
        *hashSmall.offset(hSmall as isize) = *fresh4;
        if (prefixStartIndex
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(repIndex) >= 3 as libc::c_int as libc::c_uint) as libc::c_int
            & (offset_1
                <= curr
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(dictStartIndex)) as libc::c_int != 0
            && MEM_read32(repMatch as *const libc::c_void)
                == MEM_read32(
                    ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                )
        {
            let mut repMatchEnd = if repIndex < prefixStartIndex {
                dictEnd
            } else {
                iend
            };
            mLength = (ZSTD_count_2segments(
                ip.offset(1 as libc::c_int as isize).offset(4 as libc::c_int as isize),
                repMatch.offset(4 as libc::c_int as isize),
                iend,
                repMatchEnd,
                prefixStart,
            ))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            ip = ip.offset(1);
            if 1 as libc::c_int >= 1 as libc::c_int {} else {
                __assert_fail(
                    b"(1)>=1\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    650 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 128],
                        &[libc::c_char; 128],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            if 1 as libc::c_int <= 3 as libc::c_int {} else {
                __assert_fail(
                    b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    650 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 128],
                        &[libc::c_char; 128],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as libc::c_long as size_t,
                anchor,
                iend,
                1 as libc::c_int as U32,
                mLength,
            );
        } else if matchLongIndex > dictStartIndex
            && MEM_read64(matchLong as *const libc::c_void)
                == MEM_read64(ip as *const libc::c_void)
        {
            let matchEnd = if matchLongIndex < prefixStartIndex {
                dictEnd
            } else {
                iend
            };
            let lowMatchPtr = if matchLongIndex < prefixStartIndex {
                dictStart
            } else {
                prefixStart
            };
            let mut offset: U32 = 0;
            mLength = (ZSTD_count_2segments(
                ip.offset(8 as libc::c_int as isize),
                matchLong.offset(8 as libc::c_int as isize),
                iend,
                matchEnd,
                prefixStart,
            ))
                .wrapping_add(8 as libc::c_int as libc::c_ulong);
            offset = curr.wrapping_sub(matchLongIndex);
            while (ip > anchor) as libc::c_int & (matchLong > lowMatchPtr) as libc::c_int
                != 0
                && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == *matchLong.offset(-(1 as libc::c_int) as isize) as libc::c_int
            {
                ip = ip.offset(-1);
                matchLong = matchLong.offset(-1);
                mLength = mLength.wrapping_add(1);
            }
            offset_2 = offset_1;
            offset_1 = offset;
            if offset > 0 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"(offset)>0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    661 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 128],
                        &[libc::c_char; 128],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as libc::c_long as size_t,
                anchor,
                iend,
                offset.wrapping_add(ZSTD_REP_NUM as libc::c_uint),
                mLength,
            );
        } else if matchIndex > dictStartIndex
            && MEM_read32(match_0 as *const libc::c_void)
                == MEM_read32(ip as *const libc::c_void)
        {
            let h3 = ZSTD_hashPtr(
                ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                hBitsL,
                8 as libc::c_int as U32,
            );
            let matchIndex3 = *hashLong.offset(h3 as isize);
            let match3Base = if matchIndex3 < prefixStartIndex {
                dictBase
            } else {
                base
            };
            let mut match3 = match3Base.offset(matchIndex3 as isize);
            let mut offset_0: U32 = 0;
            *hashLong
                .offset(
                    h3 as isize,
                ) = curr.wrapping_add(1 as libc::c_int as libc::c_uint);
            if matchIndex3 > dictStartIndex
                && MEM_read64(match3 as *const libc::c_void)
                    == MEM_read64(
                        ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    )
            {
                let matchEnd_0 = if matchIndex3 < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let lowMatchPtr_0 = if matchIndex3 < prefixStartIndex {
                    dictStart
                } else {
                    prefixStart
                };
                mLength = (ZSTD_count_2segments(
                    ip.offset(9 as libc::c_int as isize),
                    match3.offset(8 as libc::c_int as isize),
                    iend,
                    matchEnd_0,
                    prefixStart,
                ))
                    .wrapping_add(8 as libc::c_int as libc::c_ulong);
                ip = ip.offset(1);
                offset_0 = curr
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(matchIndex3);
                while (ip > anchor) as libc::c_int
                    & (match3 > lowMatchPtr_0) as libc::c_int != 0
                    && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == *match3.offset(-(1 as libc::c_int) as isize) as libc::c_int
                {
                    ip = ip.offset(-1);
                    match3 = match3.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            } else {
                let matchEnd_1 = if matchIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let lowMatchPtr_1 = if matchIndex < prefixStartIndex {
                    dictStart
                } else {
                    prefixStart
                };
                mLength = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
                    match_0.offset(4 as libc::c_int as isize),
                    iend,
                    matchEnd_1,
                    prefixStart,
                ))
                    .wrapping_add(4 as libc::c_int as libc::c_ulong);
                offset_0 = curr.wrapping_sub(matchIndex);
                while (ip > anchor) as libc::c_int
                    & (match_0 > lowMatchPtr_1) as libc::c_int != 0
                    && *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == *match_0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                {
                    ip = ip.offset(-1);
                    match_0 = match_0.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            }
            offset_2 = offset_1;
            offset_1 = offset_0;
            if offset_0 > 0 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"(offset)>0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                        as *const u8 as *const libc::c_char,
                    686 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 128],
                        &[libc::c_char; 128],
                    >(
                        b"size_t ZSTD_compressBlock_doubleFast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                    ))
                        .as_ptr(),
                );
            }
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as libc::c_long as size_t,
                anchor,
                iend,
                offset_0.wrapping_add(ZSTD_REP_NUM as libc::c_uint),
                mLength,
            );
        } else {
            ip = ip
                .offset(
                    ((ip.offset_from(anchor) as libc::c_long >> kSearchStrength)
                        + 1 as libc::c_int as libc::c_long) as isize,
                );
            continue;
        }
        ip = ip.offset(mLength as isize);
        anchor = ip;
        if ip <= ilimit {
            let indexToInsert = curr.wrapping_add(2 as libc::c_int as libc::c_uint);
            *hashLong
                .offset(
                    ZSTD_hashPtr(
                        base.offset(indexToInsert as isize) as *const libc::c_void,
                        hBitsL,
                        8 as libc::c_int as U32,
                    ) as isize,
                ) = indexToInsert;
            *hashLong
                .offset(
                    ZSTD_hashPtr(
                        ip.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                        hBitsL,
                        8 as libc::c_int as U32,
                    ) as isize,
                ) = ip.offset(-(2 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            *hashSmall
                .offset(
                    ZSTD_hashPtr(
                        base.offset(indexToInsert as isize) as *const libc::c_void,
                        hBitsS,
                        mls,
                    ) as isize,
                ) = indexToInsert;
            *hashSmall
                .offset(
                    ZSTD_hashPtr(
                        ip.offset(-(1 as libc::c_int as isize)) as *const libc::c_void,
                        hBitsS,
                        mls,
                    ) as isize,
                ) = ip.offset(-(1 as libc::c_int as isize)).offset_from(base)
                as libc::c_long as U32;
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as libc::c_long as U32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let mut repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !((prefixStartIndex
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(repIndex2) >= 3 as libc::c_int as libc::c_uint)
                    as libc::c_int
                    & (offset_2 <= current2.wrapping_sub(dictStartIndex)) as libc::c_int
                    != 0
                    && MEM_read32(repMatch2 as *const libc::c_void)
                        == MEM_read32(ip as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex { dictEnd } else { iend };
                let repLength2 = (ZSTD_count_2segments(
                    ip.offset(4 as libc::c_int as isize),
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
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        718 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 128],
                            &[libc::c_char; 128],
                        >(
                            b"size_t ZSTD_compressBlock_doubleFast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if 1 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(1)<=ZSTD_REP_NUM\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_double_fast.c\0"
                            as *const u8 as *const libc::c_char,
                        718 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 128],
                            &[libc::c_char; 128],
                        >(
                            b"size_t ZSTD_compressBlock_doubleFast_extDict_generic(ZSTD_matchState_t *, seqStore_t *, U32 *, const void *, size_t, const U32)\0",
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
                *hashSmall
                    .offset(
                        ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls) as isize,
                    ) = current2;
                *hashLong
                    .offset(
                        ZSTD_hashPtr(
                            ip as *const libc::c_void,
                            hBitsL,
                            8 as libc::c_int as U32,
                        ) as isize,
                    ) = current2;
                ip = ip.offset(repLength2 as isize);
                anchor = ip;
            }
        }
    }
    *rep.offset(0 as libc::c_int as isize) = offset_1;
    *rep.offset(1 as libc::c_int as isize) = offset_2;
    return iend.offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict_4(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict_5(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict_6(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as U32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict_7(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_compressBlock_doubleFast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => {
            return ZSTD_compressBlock_doubleFast_extDict_5(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        6 => {
            return ZSTD_compressBlock_doubleFast_extDict_6(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        7 => {
            return ZSTD_compressBlock_doubleFast_extDict_7(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
        4 | _ => {
            return ZSTD_compressBlock_doubleFast_extDict_4(
                ms,
                seqStore,
                rep,
                src,
                srcSize,
            );
        }
    };
}
