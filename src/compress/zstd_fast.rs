use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

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
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
pub type unalignArch = libc::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut u8,
    pub lit: *mut u8,
    pub llCode: *mut u8,
    pub mlCode: *mut u8,
    pub ofCode: *mut u8,
    pub maxNbSeq: libc::size_t,
    pub maxNbLit: libc::size_t,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: u32,
}
pub type ZSTD_longLengthType_e = libc::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type seqDef = seqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqDef_s {
    pub offBase: u32,
    pub litLength: u16,
    pub mlBase: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_matchState_t {
    pub window: ZSTD_window_t,
    pub loadedDictEnd: u32,
    pub nextToUpdate: u32,
    pub hashLog3: u32,
    pub rowHashLog: u32,
    pub tagTable: *mut u8,
    pub hashCache: [u32; 8],
    pub hashSalt: u64,
    pub hashSaltEntropy: u32,
    pub hashTable: *mut u32,
    pub hashTable3: *mut u32,
    pub chainTable: *mut u32,
    pub forceNonContiguous: u32,
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
    pub pos: libc::size_t,
    pub posInSequence: libc::size_t,
    pub size: libc::size_t,
    pub capacity: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: u32,
    pub litLength: u32,
    pub matchLength: u32,
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
    pub litSum: u32,
    pub litLengthSum: u32,
    pub matchLengthSum: u32,
    pub offCodeSum: u32,
    pub litSumBasePrice: u32,
    pub litLengthSumBasePrice: u32,
    pub matchLengthSumBasePrice: u32,
    pub offCodeSumBasePrice: u32,
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
pub type HUF_CElt = libc::size_t;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: u32,
    pub mlen: u32,
    pub litlen: u32,
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: u32,
    pub len: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: u32,
    pub lowLimit: u32,
    pub nbOverflowCorrections: u32,
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
    return (::core::mem::size_of::<libc::size_t>()
        == 8) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> u16 {
    return *(ptr as *const unalign16);
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> u32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> u64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_readST(mut ptr: *const libc::c_void) -> libc::size_t {
    return *(ptr as *const unalignArch);
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> u32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr)
    } else {
        return MEM_swap32(MEM_read32(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> u64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        return MEM_swap64(MEM_read64(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: u64) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_ulong);
    return (val as libc::c_ulonglong).trailing_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros64(mut val: u64) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_ulong);
    return (val as libc::c_ulonglong).leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: libc::size_t) -> libc::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            return ZSTD_countTrailingZeros64(val) >> 3 as libc::c_int
        } else {
            return ZSTD_countTrailingZeros32(val as u32) >> 3 as libc::c_int
        }
    } else if MEM_64bits() != 0 {
        return ZSTD_countLeadingZeros64(val) >> 3 as libc::c_int
    } else {
        return ZSTD_countLeadingZeros32(val as u32) >> 3 as libc::c_int
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
    let mut diff = (dst as *mut u8).offset_from(src as *const u8) as libc::c_long;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    if ovtype as libc::c_uint
        == ZSTD_overlap_src_before_dst as libc::c_int as libc::c_uint
        && diff < WILDCOPY_VECLEN as libc::c_long
    {
        loop {
            ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(8);
            ip = ip.offset(8);
            if !(op < oend) {
                break;
            }
        }
    } else {
        debug_assert!(diff >= 16
            || diff <= -(16) as libc::c_long);
        ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
        if 16 as libc::c_int as libc::c_long >= length {
            return;
        }
        op = op.offset(16);
        ip = ip.offset(16);
        loop {
            ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            ZSTD_copy16(op as *mut libc::c_void, ip as *const libc::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            if !(op < oend) {
                break;
            }
        }
    };
}
pub const WILDCOPY_VECLEN: libc::c_int = 16 as libc::c_int;
pub const MINMATCH: libc::c_int = 3 as libc::c_int;
unsafe extern "C" fn ZSTD_safecopyLiterals(
    mut op: *mut u8,
    mut ip: *const u8,
    iend: *const u8,
    mut ilimit_w: *const u8,
) {
    debug_assert!(iend > ilimit_w);
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
    mut litLength: libc::size_t,
    mut literals: *const u8,
    mut litLimit: *const u8,
    mut offBase: u32,
    mut matchLength: libc::size_t,
) {
    let litLimit_w = litLimit.offset(-(WILDCOPY_OVERLENGTH as isize));
    let litEnd = literals.offset(litLength as isize);
    debug_assert!((((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as libc::c_long as libc::size_t) < (*seqStorePtr).maxNbSeq);
    debug_assert!((*seqStorePtr).maxNbLit
        <= (128 as libc::c_int * ((1) << 10 as libc::c_int))
            as libc::c_ulong);
    debug_assert!(((*seqStorePtr).lit).offset(litLength as isize)
        <= ((*seqStorePtr).litStart).offset((*seqStorePtr).maxNbLit as isize));
    debug_assert!(literals.offset(litLength as isize) <= litLimit);
    if litEnd <= litLimit_w {
        ZSTD_copy16(
            (*seqStorePtr).lit as *mut libc::c_void,
            literals as *const libc::c_void,
        );
        if litLength > 16 {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16)
                    as *mut libc::c_void,
                literals.offset(16) as *const libc::c_void,
                litLength as ptrdiff_t - 16 as libc::c_int as libc::c_long,
                ZSTD_no_overlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(litLength as isize);
    if litLength > 0xffff as libc::c_int as libc::c_ulong {
        debug_assert!((*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint);
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0))
        .litLength = litLength as u16;
    (*((*seqStorePtr).sequences).offset(0)).offBase = offBase;
    debug_assert!(matchLength >= 3);
    let mlBase = matchLength.wrapping_sub(MINMATCH as libc::c_ulong);
    if mlBase > 0xffff as libc::c_int as libc::c_ulong {
        debug_assert!((*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint);
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0))
        .mlBase = mlBase as u16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
}
pub const WILDCOPY_OVERLENGTH: libc::c_int = 32 as libc::c_int;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_count(
    mut pIn: *const u8,
    mut pMatch: *const u8,
    pInLimit: *const u8,
) -> libc::size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit
        .offset(
            -((::core::mem::size_of::<libc::size_t>())
                .wrapping_sub(1) as isize),
        );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const libc::c_void)
            ^ MEM_readST(pIn as *const libc::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as libc::size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<libc::size_t>() as isize);
        pMatch = pMatch
            .offset(::core::mem::size_of::<libc::size_t>() as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const libc::c_void)
                ^ MEM_readST(pIn as *const libc::c_void);
            if diff_0 == 0 {
                pIn = pIn
                    .offset(::core::mem::size_of::<libc::size_t>() as isize);
                pMatch = pMatch
                    .offset(::core::mem::size_of::<libc::size_t>() as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as libc::c_long as libc::size_t;
            }
        }
    }
    if MEM_64bits() != 0 && pIn < pInLimit.offset(-(3))
        && MEM_read32(pMatch as *const libc::c_void)
            == MEM_read32(pIn as *const libc::c_void)
    {
        pIn = pIn.offset(4);
        pMatch = pMatch.offset(4);
    }
    if pIn < pInLimit.offset(-(1))
        && MEM_read16(pMatch as *const libc::c_void) as libc::c_int
            == MEM_read16(pIn as *const libc::c_void) as libc::c_int
    {
        pIn = pIn.offset(2);
        pMatch = pMatch.offset(2);
    }
    if pIn < pInLimit && *pMatch as libc::c_int == *pIn as libc::c_int {
        pIn = pIn.offset(1);
    }
    return pIn.offset_from(pStart) as libc::c_long as libc::size_t;
}
#[inline]
unsafe extern "C" fn ZSTD_count_2segments(
    mut ip: *const u8,
    mut match_0: *const u8,
    mut iEnd: *const u8,
    mut mEnd: *const u8,
    mut iStart: *const u8,
) -> libc::size_t {
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
static mut prime4bytes: u32 = 2654435761 as libc::c_uint;
unsafe extern "C" fn ZSTD_hash4(mut u: u32, mut h: u32, mut s: u32) -> u32 {
    debug_assert!(h <= 32);
    return (u.wrapping_mul(prime4bytes) ^ s)
        >> (32).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash4(MEM_readLE32(ptr), h, 0 as libc::c_int as u32) as libc::size_t;
}
static mut prime5bytes: u64 = 889523592379 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash5(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64);
    return ((u << 64 as libc::c_int - 40 as libc::c_int).wrapping_mul(prime5bytes) ^ s)
        >> (64).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash5(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
static mut prime6bytes: u64 = 227718039650203 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash6(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64);
    return ((u << 64 as libc::c_int - 48 as libc::c_int).wrapping_mul(prime6bytes) ^ s)
        >> (64).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash6(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
static mut prime7bytes: u64 = 58295818150454627 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash7(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64);
    return ((u << 64 as libc::c_int - 56 as libc::c_int).wrapping_mul(prime7bytes) ^ s)
        >> (64).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash7(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
static mut prime8bytes: u64 = 0xcf1bbcdcb7a56463 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash8(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64);
    return (u.wrapping_mul(prime8bytes) ^ s)
        >> (64).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash8(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_hashPtr(
    mut p: *const libc::c_void,
    mut hBits: u32,
    mut mls: u32,
) -> libc::size_t {
    debug_assert!(hBits <= 32);
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
    mut curr: u32,
    mut windowLog: libc::c_uint,
) -> u32 {
    let maxDistance = (1) << windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as u32;
    let matchLowest = if isDictionary != 0 { lowestValid } else { withinWindow };
    return matchLowest;
}
#[inline]
unsafe extern "C" fn ZSTD_getLowestPrefixIndex(
    mut ms: *const ZSTD_matchState_t,
    mut curr: u32,
    mut windowLog: libc::c_uint,
) -> u32 {
    let maxDistance = (1) << windowLog;
    let lowestValid = (*ms).window.dictLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as u32;
    let matchLowest = if isDictionary != 0 { lowestValid } else { withinWindow };
    return matchLowest;
}
#[inline]
unsafe extern "C" fn ZSTD_writeTaggedIndex(
    hashTable: *mut u32,
    mut hashAndTag: libc::size_t,
    mut index: u32,
) {
    let hash = hashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
    let tag = (hashAndTag & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as u32;
    debug_assert!(index >> 32 as libc::c_int - 8 as libc::c_int == 0);
    *hashTable.offset(hash as isize) = index << ZSTD_SHORT_CACHE_TAG_BITS | tag;
}
pub const ZSTD_SHORT_CACHE_TAG_BITS: libc::c_int = 8 as libc::c_int;
pub const ZSTD_SHORT_CACHE_TAG_MASK: libc::c_uint = ((1)
    << ZSTD_SHORT_CACHE_TAG_BITS)
    .wrapping_sub(1);
#[inline]
unsafe extern "C" fn ZSTD_comparePackedTags(
    mut packedTag1: libc::size_t,
    mut packedTag2: libc::size_t,
) -> libc::c_int {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as u32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK as libc::c_ulong) as u32;
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
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3 as libc::c_int as u32;
    debug_assert!(dtlm as libc::c_uint == ZSTD_dtlm_full as libc::c_int as libc::c_uint);
    while ip.offset(fastHashFillStep as isize) < iend.offset(2) {
        let curr = ip.offset_from(base) as libc::c_long as u32;
        let hashAndTag = ZSTD_hashPtr(ip as *const libc::c_void, hBits, mls);
        ZSTD_writeTaggedIndex(hashTable, hashAndTag, curr);
        if !(dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint) {
            let mut p: u32 = 0;
            p = 1 as libc::c_int as u32;
            while p < fastHashFillStep {
                let hashAndTag_0 = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const libc::c_void,
                    hBits,
                    mls,
                );
                if *hashTable
                    .offset((hashAndTag_0 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize)
                    == 0
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
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3 as libc::c_int as u32;
    debug_assert!(dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint);
    while ip.offset(fastHashFillStep as isize) < iend.offset(2) {
        let curr = ip.offset_from(base) as libc::c_long as u32;
        let hash0 = ZSTD_hashPtr(ip as *const libc::c_void, hBits, mls);
        *hashTable.offset(hash0 as isize) = curr;
        if !(dtlm as libc::c_uint == ZSTD_dtlm_fast as libc::c_int as libc::c_uint) {
            let mut p: u32 = 0;
            p = 1 as libc::c_int as u32;
            while p < fastHashFillStep {
                let hash = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const libc::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.offset(hash as isize) == 0 {
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
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mls: u32,
    hasStep: u32,
) -> libc::size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = (if hasStep != 0 {
        ((*cParams).targetLength)
            .wrapping_add(((*cParams).targetLength == 0) as libc::c_int as libc::c_uint)
            .wrapping_add(1)
    } else {
        2 as libc::c_int as libc::c_uint
    }) as libc::size_t;
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let endIndex = (istart.offset_from(base) as libc::c_long as libc::size_t)
        .wrapping_add(srcSize) as u32;
    let prefixStartIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut anchor = istart;
    let mut ip0 = istart;
    let mut ip1 = 0 as *const u8;
    let mut ip2 = 0 as *const u8;
    let mut ip3 = 0 as *const u8;
    let mut current0: u32 = 0;
    let mut rep_offset1 = *rep.offset(0);
    let mut rep_offset2 = *rep.offset(1);
    let mut offsetSaved1 = 0 as libc::c_int as u32;
    let mut offsetSaved2 = 0 as libc::c_int as u32;
    let mut hash0: libc::size_t = 0;
    let mut hash1: libc::size_t = 0;
    let mut idx: u32 = 0;
    let mut mval: u32 = 0;
    let mut offcode: u32 = 0;
    let mut match0 = 0 as *const u8;
    let mut mLength: libc::size_t = 0;
    let mut step: libc::size_t = 0;
    let mut nextStep = 0 as *const u8;
    let kStepIncr = ((1) << kSearchStrength - 1 as libc::c_int) as libc::size_t;
    ip0 = ip0.offset((ip0 == prefixStart) as libc::c_int as isize);
    let curr = ip0.offset_from(base) as libc::c_long as u32;
    let windowLow = ZSTD_getLowestPrefixIndex(ms, curr, (*cParams).windowLog);
    let maxRep = curr.wrapping_sub(windowLow);
    if rep_offset2 > maxRep {
        offsetSaved2 = rep_offset2;
        rep_offset2 = 0 as libc::c_int as u32;
    }
    if rep_offset1 > maxRep {
        offsetSaved1 = rep_offset1;
        rep_offset1 = 0 as libc::c_int as u32;
    }
    '__start: loop {
        step = stepSize;
        nextStep = ip0.offset(kStepIncr as isize);
        ip1 = ip0.offset(1);
        ip2 = ip0.offset(step as isize);
        ip3 = ip2.offset(1);
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
            current0 = ip0.offset_from(base) as libc::c_long as u32;
            *hashTable.offset(hash0 as isize) = current0;
            if (MEM_read32(ip2 as *const libc::c_void) == rval) as libc::c_int
                & (rep_offset1 > 0) as libc::c_int != 0
            {
                ip0 = ip2;
                match0 = ip0.offset(-(rep_offset1 as isize));
                mLength = (*ip0.offset(-(1) as isize) as libc::c_int
                    == *match0.offset(-(1) as isize) as libc::c_int)
                    as libc::c_int as libc::size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                debug_assert!(1 as libc::c_int >= 1);
                debug_assert!(1 as libc::c_int <= 3);
                offcode = 1 as libc::c_int as u32;
                mLength = (mLength as libc::c_ulong)
                    .wrapping_add(4) ;
                *hashTable
                    .offset(
                        hash1 as isize,
                    ) = ip1.offset_from(base) as libc::c_long as u32;
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
                        ) = ip1.offset_from(base) as libc::c_long as u32;
                    current_block = 13355861697473861518;
                    break;
                } else {
                    idx = *hashTable.offset(hash1 as isize);
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const libc::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip3;
                    current0 = ip0.offset_from(base) as libc::c_long as u32;
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
                        if step <= 4 {
                            *hashTable
                                .offset(
                                    hash1 as isize,
                                ) = ip1.offset_from(base) as libc::c_long as u32;
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
                rep_offset1 = ip0.offset_from(match0) as libc::c_long as u32;
                debug_assert!(rep_offset1 > 0);
                offcode = rep_offset1.wrapping_add(ZSTD_REP_NUM as libc::c_uint);
                mLength = 4 as libc::c_int as libc::size_t;
                while (ip0 > anchor) as libc::c_int
                    & (match0 > prefixStart) as libc::c_int != 0
                    && *ip0.offset(-(1) as isize) as libc::c_int
                        == *match0.offset(-(1) as isize) as libc::c_int
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
            ) ;
        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from(anchor) as libc::c_long as libc::size_t,
            anchor,
            iend,
            offcode,
            mLength,
        );
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip0 <= ilimit {
            debug_assert!(base.offset(current0 as isize).offset(2) > istart);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        base.offset(current0 as isize).offset(2)
                            as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = current0.wrapping_add(2);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        ip0.offset(-(2)) as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = ip0.offset(-(2)).offset_from(base)
                as libc::c_long as u32;
            if rep_offset2 > 0 {
                while ip0 <= ilimit
                    && MEM_read32(ip0 as *const libc::c_void)
                        == MEM_read32(
                            ip0.offset(-(rep_offset2 as isize)) as *const libc::c_void,
                        )
                {
                    let rLength = (ZSTD_count(
                        ip0.offset(4),
                        ip0
                            .offset(4)
                            .offset(-(rep_offset2 as isize)),
                        iend,
                    ))
                        .wrapping_add(4);
                    let tmpOff = rep_offset2;
                    rep_offset2 = rep_offset1;
                    rep_offset1 = tmpOff;
                    *hashTable
                        .offset(
                            ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls) as isize,
                        ) = ip0.offset_from(base) as libc::c_long as u32;
                    ip0 = ip0.offset(rLength as isize);
                    debug_assert!(1 as libc::c_int >= 1);
                    debug_assert!(1 as libc::c_int <= 3);
                    ZSTD_storeSeq(
                        seqStore,
                        0 as libc::c_int as libc::size_t,
                        anchor,
                        iend,
                        1 as libc::c_int as u32,
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
    return iend.offset_from(anchor) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_4_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as u32,
        1 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_5_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as u32,
        1 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_6_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as u32,
        1 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_7_1(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as u32,
        1 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_4_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_5_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_6_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_noDict_7_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_noDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mls = (*ms).cParams.minMatch;
    debug_assert!(((*ms).dictMatchState).is_null());
    if (*ms).cParams.targetLength > 1 {
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
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mls: u32,
    hasStep: u32,
) -> libc::size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as libc::c_int as libc::c_uint);
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let mut ip0 = istart;
    let mut ip1 = ip0.offset(stepSize as isize);
    let mut anchor = istart;
    let prefixStartIndex = (*ms).window.dictLimit;
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
    let dms = (*ms).dictMatchState;
    let dictCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const u32 = (*dms).hashTable;
    let dictStartIndex = (*dms).window.dictLimit;
    let dictBase = (*dms).window.base;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = (*dms).window.nextSrc;
    let dictIndexDelta = prefixStartIndex
        .wrapping_sub(dictEnd.offset_from(dictBase) as libc::c_long as u32);
    let dictAndPrefixLength = dictEnd
        .offset(istart.offset_from(prefixStart) as libc::c_long as isize)
        .offset_from(dictStart) as libc::c_long as u32;
    let dictHBits = ((*dictCParams).hashLog)
        .wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as libc::c_uint);
    let maxDistance = (1) << (*cParams).windowLog;
    let endIndex = (istart.offset_from(base) as libc::c_long as libc::size_t)
        .wrapping_add(srcSize) as u32;
    debug_assert!(endIndex.wrapping_sub(prefixStartIndex) <= maxDistance);
    debug_assert!(prefixStartIndex >= dictEnd.offset_from(dictBase) as libc::c_long as u32);
    if (*ms).prefetchCDictTables != 0 {
        let hashTableBytes = ((1) << (*dictCParams).hashLog)
            .wrapping_mul(::core::mem::size_of::<u32>());
        let _ptr = dictHashTable as *const libc::c_char;
        let _size = hashTableBytes;
        let mut _pos: libc::size_t = 0;
        _pos = 0 as libc::c_int as libc::size_t;
        while _pos < _size {
            _pos = (_pos as libc::c_ulong).wrapping_add(CACHELINE_SIZE as libc::c_ulong)
                ;
        }
    }
    ip0 = ip0
        .offset(
            (dictAndPrefixLength == 0) as libc::c_int
                as isize,
        );
    debug_assert!(offset_1 <= dictAndPrefixLength);
    debug_assert!(offset_2 <= dictAndPrefixLength);
    debug_assert!(stepSize >= 1);
    's_126: while ip1 <= ilimit {
        let mut mLength: libc::size_t = 0;
        let mut hash0 = ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls);
        let dictHashAndTag0 = ZSTD_hashPtr(ip0 as *const libc::c_void, dictHBits, mls);
        let mut dictMatchIndexAndTag = *dictHashTable
            .offset((dictHashAndTag0 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
        let mut dictTagsMatch = ZSTD_comparePackedTags(
            dictMatchIndexAndTag as libc::size_t,
            dictHashAndTag0,
        );
        let mut matchIndex = *hashTable.offset(hash0 as isize);
        let mut curr = ip0.offset_from(base) as libc::c_long as u32;
        let mut step = stepSize as libc::size_t;
        let kStepIncr = ((1) << kSearchStrength) as libc::size_t;
        let mut nextStep = ip0.offset(kStepIncr as isize);
        loop {
            let mut match_0 = base.offset(matchIndex as isize);
            let repIndex = curr
                .wrapping_add(1)
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
                .wrapping_sub(1)
                .wrapping_sub(repIndex) >= 3
                && MEM_read32(repMatch as *const libc::c_void)
                    == MEM_read32(
                        ip0.offset(1) as *const libc::c_void,
                    )
            {
                let repMatchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = (ZSTD_count_2segments(
                    ip0
                        .offset(1)
                        .offset(4),
                    repMatch.offset(4),
                    iend,
                    repMatchEnd,
                    prefixStart,
                ))
                    .wrapping_add(4);
                ip0 = ip0.offset(1);
                debug_assert!(1 as libc::c_int >= 1);
                debug_assert!(1 as libc::c_int <= 3);
                ZSTD_storeSeq(
                    seqStore,
                    ip0.offset_from(anchor) as libc::c_long as libc::size_t,
                    anchor,
                    iend,
                    1 as libc::c_int as u32,
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
                                ip0.offset(4),
                                dictMatch.offset(4),
                                iend,
                                dictEnd,
                                prefixStart,
                            ))
                                .wrapping_add(4);
                            while (ip0 > anchor) as libc::c_int
                                & (dictMatch > dictStart) as libc::c_int != 0
                                && *ip0.offset(-(1) as isize) as libc::c_int
                                    == *dictMatch.offset(-(1) as isize)
                                        as libc::c_int
                            {
                                ip0 = ip0.offset(-1);
                                dictMatch = dictMatch.offset(-1);
                                mLength = mLength.wrapping_add(1);
                            }
                            offset_2 = offset_1;
                            offset_1 = offset;
                            debug_assert!(offset > 0);
                            ZSTD_storeSeq(
                                seqStore,
                                ip0.offset_from(anchor) as libc::c_long as libc::size_t,
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
                    let offset_0 = ip0.offset_from(match_0) as libc::c_long as u32;
                    mLength = (ZSTD_count(
                        ip0.offset(4),
                        match_0.offset(4),
                        iend,
                    ))
                        .wrapping_add(4);
                    while (ip0 > anchor) as libc::c_int
                        & (match_0 > prefixStart) as libc::c_int != 0
                        && *ip0.offset(-(1) as isize) as libc::c_int
                            == *match_0.offset(-(1) as isize)
                                as libc::c_int
                    {
                        ip0 = ip0.offset(-1);
                        match_0 = match_0.offset(-1);
                        mLength = mLength.wrapping_add(1);
                    }
                    offset_2 = offset_1;
                    offset_1 = offset_0;
                    debug_assert!(offset_0 > 0);
                    ZSTD_storeSeq(
                        seqStore,
                        ip0.offset_from(anchor) as libc::c_long as libc::size_t,
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
                        dictMatchIndexAndTag as libc::size_t,
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
                    curr = ip0.offset_from(base) as libc::c_long as u32;
                    hash0 = hash1;
                }
            }
        }
        debug_assert!(mLength != 0);
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip0 <= ilimit {
            debug_assert!(base.offset(curr as isize).offset(2) > istart);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        base.offset(curr as isize).offset(2)
                            as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = curr.wrapping_add(2);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        ip0.offset(-(2)) as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = ip0.offset(-(2)).offset_from(base)
                as libc::c_long as u32;
            while ip0 <= ilimit {
                let current2 = ip0.offset_from(base) as libc::c_long as u32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let mut repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase
                        .offset(-(dictIndexDelta as isize))
                        .offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(prefixStartIndex
                    .wrapping_sub(1)
                    .wrapping_sub(repIndex2) >= 3
                    && MEM_read32(repMatch2 as *const libc::c_void)
                        == MEM_read32(ip0 as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex { dictEnd } else { iend };
                let repLength2 = (ZSTD_count_2segments(
                    ip0.offset(4),
                    repMatch2.offset(4),
                    iend,
                    repEnd2,
                    prefixStart,
                ))
                    .wrapping_add(4);
                let mut tmpOffset = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                debug_assert!(1 as libc::c_int >= 1);
                debug_assert!(1 as libc::c_int <= 3);
                ZSTD_storeSeq(
                    seqStore,
                    0 as libc::c_int as libc::size_t,
                    anchor,
                    iend,
                    1 as libc::c_int as u32,
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
        debug_assert!(ip0 == anchor);
        ip1 = ip0.offset(stepSize as isize);
    }
    *rep.offset(0) = offset_1;
    *rep.offset(1) = offset_2;
    return iend.offset_from(anchor) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_4_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_5_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_6_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState_7_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_dictMatchState_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mls = (*ms).cParams.minMatch;
    debug_assert!(!((*ms).dictMatchState).is_null());
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
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mls: u32,
    hasStep: u32,
) -> libc::size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as libc::c_int as libc::c_uint)
        .wrapping_add(1) as libc::size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let istart = src as *const u8;
    let mut anchor = istart;
    let endIndex = (istart.offset_from(base) as libc::c_long as libc::size_t)
        .wrapping_add(srcSize) as u32;
    let lowLimit = ZSTD_getLowestMatchIndex(ms, endIndex, (*cParams).windowLog);
    let dictStartIndex = lowLimit;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictLimit = (*ms).window.dictLimit;
    let prefixStartIndex = if dictLimit < lowLimit { lowLimit } else { dictLimit };
    let prefixStart = base.offset(prefixStartIndex as isize);
    let dictEnd = dictBase.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(8));
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
    let mut offsetSaved1 = 0 as libc::c_int as u32;
    let mut offsetSaved2 = 0 as libc::c_int as u32;
    let mut ip0 = istart;
    let mut ip1 = 0 as *const u8;
    let mut ip2 = 0 as *const u8;
    let mut ip3 = 0 as *const u8;
    let mut current0: u32 = 0;
    let mut hash0: libc::size_t = 0;
    let mut hash1: libc::size_t = 0;
    let mut idx: u32 = 0;
    let mut idxBase = 0 as *const u8;
    let mut offcode: u32 = 0;
    let mut match0 = 0 as *const u8;
    let mut mLength: libc::size_t = 0;
    let mut matchEnd = 0 as *const u8;
    let mut step: libc::size_t = 0;
    let mut nextStep = 0 as *const u8;
    let kStepIncr = ((1) << kSearchStrength - 1 as libc::c_int) as libc::size_t;
    if prefixStartIndex == dictStartIndex {
        return ZSTD_compressBlock_fast(ms, seqStore, rep, src, srcSize);
    }
    let curr = ip0.offset_from(base) as libc::c_long as u32;
    let maxRep = curr.wrapping_sub(dictStartIndex);
    if offset_2 >= maxRep {
        offsetSaved2 = offset_2;
        offset_2 = 0 as libc::c_int as u32;
    }
    if offset_1 >= maxRep {
        offsetSaved1 = offset_1;
        offset_1 = 0 as libc::c_int as u32;
    }
    '__start: loop {
        step = stepSize;
        nextStep = ip0.offset(kStepIncr as isize);
        ip1 = ip0.offset(1);
        ip2 = ip0.offset(step as isize);
        ip3 = ip2.offset(1);
        if ip3 >= ilimit {
            break;
        }
        hash0 = ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const libc::c_void, hlog, mls);
        idx = *hashTable.offset(hash0 as isize);
        idxBase = if idx < prefixStartIndex { dictBase } else { base };
        loop {
            let current2 = ip2.offset_from(base) as libc::c_long as u32;
            let repIndex = current2.wrapping_sub(offset_1);
            let repBase = if repIndex < prefixStartIndex { dictBase } else { base };
            let mut rval: u32 = 0;
            if (prefixStartIndex.wrapping_sub(repIndex)
                >= 4) as libc::c_int
                & (offset_1 > 0) as libc::c_int != 0
            {
                rval = MEM_read32(
                    repBase.offset(repIndex as isize) as *const libc::c_void,
                );
            } else {
                rval = MEM_read32(ip2 as *const libc::c_void)
                    ^ 1 as libc::c_int as libc::c_uint;
            }
            current0 = ip0.offset_from(base) as libc::c_long as u32;
            *hashTable.offset(hash0 as isize) = current0;
            if MEM_read32(ip2 as *const libc::c_void) == rval {
                ip0 = ip2;
                match0 = repBase.offset(repIndex as isize);
                matchEnd = if repIndex < prefixStartIndex { dictEnd } else { iend };
                debug_assert!((match0 != prefixStart) as libc::c_int
                    & (match0 != dictStart) as libc::c_int != 0);
                mLength = (*ip0.offset(-(1) as isize) as libc::c_int
                    == *match0.offset(-(1) as isize) as libc::c_int)
                    as libc::c_int as libc::size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                debug_assert!(1 as libc::c_int >= 1);
                debug_assert!(1 as libc::c_int <= 3);
                offcode = 1 as libc::c_int as u32;
                mLength = (mLength as libc::c_ulong)
                    .wrapping_add(4) ;
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
                    current0 = ip0.offset_from(base) as libc::c_long as u32;
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
                debug_assert!(offset > 0);
                offcode = offset.wrapping_add(ZSTD_REP_NUM as libc::c_uint);
                mLength = 4 as libc::c_int as libc::size_t;
                while (ip0 > anchor) as libc::c_int
                    & (match0 > lowMatchPtr) as libc::c_int != 0
                    && *ip0.offset(-(1) as isize) as libc::c_int
                        == *match0.offset(-(1) as isize) as libc::c_int
                {
                    ip0 = ip0.offset(-1);
                    match0 = match0.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            }
            _ => {}
        }
        debug_assert!(!matchEnd.is_null());
        mLength = (mLength as libc::c_ulong)
            .wrapping_add(
                ZSTD_count_2segments(
                    ip0.offset(mLength as isize),
                    match0.offset(mLength as isize),
                    iend,
                    matchEnd,
                    prefixStart,
                ),
            ) ;
        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from(anchor) as libc::c_long as libc::size_t,
            anchor,
            iend,
            offcode,
            mLength,
        );
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip1 < ip0 {
            *hashTable
                .offset(hash1 as isize) = ip1.offset_from(base) as libc::c_long as u32;
        }
        if ip0 <= ilimit {
            debug_assert!(base.offset(current0 as isize).offset(2) > istart);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        base.offset(current0 as isize).offset(2)
                            as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = current0.wrapping_add(2);
            *hashTable
                .offset(
                    ZSTD_hashPtr(
                        ip0.offset(-(2)) as *const libc::c_void,
                        hlog,
                        mls,
                    ) as isize,
                ) = ip0.offset(-(2)).offset_from(base)
                as libc::c_long as u32;
            while ip0 <= ilimit {
                let repIndex2 = (ip0.offset_from(base) as libc::c_long as u32)
                    .wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !((prefixStartIndex
                    .wrapping_sub(1)
                    .wrapping_sub(repIndex2) >= 3)
                    as libc::c_int
                    & (offset_2 > 0) as libc::c_int != 0
                    && MEM_read32(repMatch2 as *const libc::c_void)
                        == MEM_read32(ip0 as *const libc::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex { dictEnd } else { iend };
                let repLength2 = (ZSTD_count_2segments(
                    ip0.offset(4),
                    repMatch2.offset(4),
                    iend,
                    repEnd2,
                    prefixStart,
                ))
                    .wrapping_add(4);
                let tmpOffset = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                debug_assert!(1 as libc::c_int >= 1);
                debug_assert!(1 as libc::c_int <= 3);
                ZSTD_storeSeq(
                    seqStore,
                    0 as libc::c_int as libc::size_t,
                    anchor,
                    iend,
                    1 as libc::c_int as u32,
                    repLength2,
                );
                *hashTable
                    .offset(
                        ZSTD_hashPtr(ip0 as *const libc::c_void, hlog, mls) as isize,
                    ) = ip0.offset_from(base) as libc::c_long as u32;
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
    return iend.offset_from(anchor) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_4_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        4 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_5_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        5 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_6_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        6 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_7_0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_fast_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        7 as libc::c_int as u32,
        0 as libc::c_int as u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mls = (*ms).cParams.minMatch;
    debug_assert!(((*ms).dictMatchState).is_null());
    match mls {
        5 => return ZSTD_compressBlock_fast_extDict_5_0(ms, seqStore, rep, src, srcSize),
        6 => return ZSTD_compressBlock_fast_extDict_6_0(ms, seqStore, rep, src, srcSize),
        7 => return ZSTD_compressBlock_fast_extDict_7_0(ms, seqStore, rep, src, srcSize),
        4 | _ => {
            return ZSTD_compressBlock_fast_extDict_4_0(ms, seqStore, rep, src, srcSize);
        }
    };
}
