use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn ZSTD_resetSeqStore(ssPtr: *mut seqStore_t);
    fn HUF_getNbBitsFromCTable(symbolTable: *const HUF_CElt, symbolValue: u32) -> u32;
    fn HIST_count_simple(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::c_uint;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: u32,
}
pub type ZSTD_overlap_e = libc::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [u32; 3],
}
pub type repcodes_t = repcodes_s;
pub type ZSTD_getAllMatchesFn = Option::<
    unsafe extern "C" fn(
        *mut ZSTD_match_t,
        *mut ZSTD_matchState_t,
        *mut u32,
        *const u8,
        *const u8,
        *const u32,
        u32,
        u32,
    ) -> u32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optLdm_t {
    pub seqStore: rawSeqStore_t,
    pub startPosInBlock: u32,
    pub endPosInBlock: u32,
    pub offset: u32,
}
pub type base_directive_e = libc::c_uint;
pub const base_1guaranteed: base_directive_e = 1;
pub const base_0possible: base_directive_e = 0;
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.leading_zeros() as i32 as libc::c_uint;
}
pub const MaxLit: libc::c_int = ((1 as libc::c_int) << Litbits) - 1 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
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
pub const Litbits: libc::c_int = 8 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const MaxML: libc::c_int = 52 as libc::c_int;
pub const MaxOff: libc::c_int = 31 as libc::c_int;
pub const UINT_MAX: libc::c_uint = (__INT_MAX__ as libc::c_uint)
    .wrapping_mul(2)
    .wrapping_add(1);
pub const ZSTD_OPT_NUM: libc::c_int = (1 as libc::c_int) << 12 as libc::c_int;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1 as libc::c_int) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_getLowestMatchIndex(
    mut ms: *const ZSTD_matchState_t,
    mut curr: u32,
    mut windowLog: libc::c_uint,
) -> u32 {
    let maxDistance = (1 as libc::c_uint) << windowLog;
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
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.trailing_zeros() as i32 as libc::c_uint;
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
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return (31 as libc::c_int as libc::c_uint)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
#[inline]
unsafe extern "C" fn FSE_initCState(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
) {
    let mut ptr = ct as *const libc::c_void;
    let mut u16ptr = ptr as *const u16;
    let tableLog = MEM_read16(ptr) as u32;
    (*statePtr).value = (1 as libc::c_int as ptrdiff_t) << tableLog;
    (*statePtr)
        .stateTable = u16ptr.offset(2 as libc::c_int as isize) as *const libc::c_void;
    (*statePtr)
        .symbolTT = ct
        .offset(1 as libc::c_int as isize)
        .offset(
            (if tableLog != 0 {
                (1 as libc::c_int)
                    << tableLog.wrapping_sub(1)
            } else {
                1 as libc::c_int
            }) as isize,
        ) as *const libc::c_void;
    (*statePtr).stateLog = tableLog;
}
#[inline]
unsafe extern "C" fn FSE_getMaxNbBits(
    mut symbolTTPtr: *const libc::c_void,
    mut symbolValue: u32,
) -> u32 {
    let mut symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    return ((*symbolTT.offset(symbolValue as isize)).deltaNbBits)
        .wrapping_add(
            (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint,
        ) >> 16 as libc::c_int;
}
static mut LL_bits: [u8; 36] = [
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    2 as libc::c_int as u8,
    2 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    4 as libc::c_int as u8,
    6 as libc::c_int as u8,
    7 as libc::c_int as u8,
    8 as libc::c_int as u8,
    9 as libc::c_int as u8,
    10 as libc::c_int as u8,
    11 as libc::c_int as u8,
    12 as libc::c_int as u8,
    13 as libc::c_int as u8,
    14 as libc::c_int as u8,
    15 as libc::c_int as u8,
    16 as libc::c_int as u8,
];
static mut ML_bits: [u8; 53] = [
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    1 as libc::c_int as u8,
    2 as libc::c_int as u8,
    2 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    5 as libc::c_int as u8,
    7 as libc::c_int as u8,
    8 as libc::c_int as u8,
    9 as libc::c_int as u8,
    10 as libc::c_int as u8,
    11 as libc::c_int as u8,
    12 as libc::c_int as u8,
    13 as libc::c_int as u8,
    14 as libc::c_int as u8,
    15 as libc::c_int as u8,
    16 as libc::c_int as u8,
];
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
            op = op.offset(8 as libc::c_int as isize);
            ip = ip.offset(8 as libc::c_int as isize);
            if !(op < oend) {
                break;
            }
        }
    } else {
        debug_assert!(diff >= 16 as libc::c_int as libc::c_long
            || diff <= -(16 as libc::c_int) as libc::c_long);
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
static mut kNullRawSeqStore: rawSeqStore_t = {
    let mut init = rawSeqStore_t {
        seq: NULL as *mut rawSeq,
        pos: 0 as libc::c_int as libc::size_t,
        posInSequence: 0 as libc::c_int as libc::size_t,
        size: 0 as libc::c_int as libc::size_t,
        capacity: 0 as libc::c_int as libc::size_t,
    };
    init
};
#[inline]
unsafe extern "C" fn ZSTD_LLcode(mut litLength: u32) -> u32 {
    static mut LL_Code: [u8; 64] = [
        0 as libc::c_int as u8,
        1 as libc::c_int as u8,
        2 as libc::c_int as u8,
        3 as libc::c_int as u8,
        4 as libc::c_int as u8,
        5 as libc::c_int as u8,
        6 as libc::c_int as u8,
        7 as libc::c_int as u8,
        8 as libc::c_int as u8,
        9 as libc::c_int as u8,
        10 as libc::c_int as u8,
        11 as libc::c_int as u8,
        12 as libc::c_int as u8,
        13 as libc::c_int as u8,
        14 as libc::c_int as u8,
        15 as libc::c_int as u8,
        16 as libc::c_int as u8,
        16 as libc::c_int as u8,
        17 as libc::c_int as u8,
        17 as libc::c_int as u8,
        18 as libc::c_int as u8,
        18 as libc::c_int as u8,
        19 as libc::c_int as u8,
        19 as libc::c_int as u8,
        20 as libc::c_int as u8,
        20 as libc::c_int as u8,
        20 as libc::c_int as u8,
        20 as libc::c_int as u8,
        21 as libc::c_int as u8,
        21 as libc::c_int as u8,
        21 as libc::c_int as u8,
        21 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        22 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        23 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
        24 as libc::c_int as u8,
    ];
    static mut LL_deltaCode: u32 = 19 as libc::c_int as u32;
    return if litLength > 63 as libc::c_int as libc::c_uint {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        LL_Code[litLength as usize] as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: u32) -> u32 {
    static mut ML_Code: [u8; 128] = [
        0 as libc::c_int as u8,
        1 as libc::c_int as u8,
        2 as libc::c_int as u8,
        3 as libc::c_int as u8,
        4 as libc::c_int as u8,
        5 as libc::c_int as u8,
        6 as libc::c_int as u8,
        7 as libc::c_int as u8,
        8 as libc::c_int as u8,
        9 as libc::c_int as u8,
        10 as libc::c_int as u8,
        11 as libc::c_int as u8,
        12 as libc::c_int as u8,
        13 as libc::c_int as u8,
        14 as libc::c_int as u8,
        15 as libc::c_int as u8,
        16 as libc::c_int as u8,
        17 as libc::c_int as u8,
        18 as libc::c_int as u8,
        19 as libc::c_int as u8,
        20 as libc::c_int as u8,
        21 as libc::c_int as u8,
        22 as libc::c_int as u8,
        23 as libc::c_int as u8,
        24 as libc::c_int as u8,
        25 as libc::c_int as u8,
        26 as libc::c_int as u8,
        27 as libc::c_int as u8,
        28 as libc::c_int as u8,
        29 as libc::c_int as u8,
        30 as libc::c_int as u8,
        31 as libc::c_int as u8,
        32 as libc::c_int as u8,
        32 as libc::c_int as u8,
        33 as libc::c_int as u8,
        33 as libc::c_int as u8,
        34 as libc::c_int as u8,
        34 as libc::c_int as u8,
        35 as libc::c_int as u8,
        35 as libc::c_int as u8,
        36 as libc::c_int as u8,
        36 as libc::c_int as u8,
        36 as libc::c_int as u8,
        36 as libc::c_int as u8,
        37 as libc::c_int as u8,
        37 as libc::c_int as u8,
        37 as libc::c_int as u8,
        37 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        38 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        39 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        40 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        41 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
        42 as libc::c_int as u8,
    ];
    static mut ML_deltaCode: u32 = 36 as libc::c_int as u32;
    return if mlBase > 127 as libc::c_int as libc::c_uint {
        (ZSTD_highbit32(mlBase)).wrapping_add(ML_deltaCode)
    } else {
        ML_Code[mlBase as usize] as libc::c_uint
    };
}
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
        <= (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
            as libc::c_ulong);
    debug_assert!(((*seqStorePtr).lit).offset(litLength as isize)
        <= ((*seqStorePtr).litStart).offset((*seqStorePtr).maxNbLit as isize));
    debug_assert!(literals.offset(litLength as isize) <= litLimit);
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
        debug_assert!((*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint);
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize))
        .litLength = litLength as u16;
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize)).offBase = offBase;
    debug_assert!(matchLength >= 3 as libc::c_int as libc::c_ulong);
    let mlBase = matchLength.wrapping_sub(MINMATCH as libc::c_ulong);
    if mlBase > 0xffff as libc::c_int as libc::c_ulong {
        debug_assert!((*seqStorePtr).longLengthType as libc::c_uint
            == ZSTD_llt_none as libc::c_int as libc::c_uint);
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr)
            .longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart) as libc::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0 as libc::c_int as isize))
        .mlBase = mlBase as u16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
}
pub const WILDCOPY_OVERLENGTH: libc::c_int = 32 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_updateRep(mut rep: *mut u32, offBase: u32, ll0: u32) {
    if offBase > ZSTD_REP_NUM as libc::c_uint {
        *rep.offset(2 as libc::c_int as isize) = *rep.offset(1 as libc::c_int as isize);
        *rep.offset(1 as libc::c_int as isize) = *rep.offset(0 as libc::c_int as isize);
        debug_assert!(offBase > 3 as libc::c_int as libc::c_uint);
        *rep
            .offset(
                0 as libc::c_int as isize,
            ) = offBase.wrapping_sub(ZSTD_REP_NUM as libc::c_uint);
    } else {
        debug_assert!(1 as libc::c_int as libc::c_uint <= offBase
            && offBase <= 3 as libc::c_int as libc::c_uint);
        let repCode = offBase
            .wrapping_sub(1)
            .wrapping_add(ll0);
        if repCode > 0 as libc::c_int as libc::c_uint {
            let currentOffset = if repCode == ZSTD_REP_NUM as libc::c_uint {
                (*rep.offset(0 as libc::c_int as isize))
                    .wrapping_sub(1)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep
                .offset(
                    2 as libc::c_int as isize,
                ) = if repCode >= 2 as libc::c_int as libc::c_uint {
                *rep.offset(1 as libc::c_int as isize)
            } else {
                *rep.offset(2 as libc::c_int as isize)
            };
            *rep
                .offset(
                    1 as libc::c_int as isize,
                ) = *rep.offset(0 as libc::c_int as isize);
            *rep.offset(0 as libc::c_int as isize) = currentOffset;
        }
    };
}
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_newRep(
    mut rep: *const u32,
    offBase: u32,
    ll0: u32,
) -> repcodes_t {
    let mut newReps = repcodes_t { rep: [0; 3] };
    libc::memcpy(
        &mut newReps as *mut repcodes_t as *mut libc::c_void,
        rep as *const libc::c_void,
        ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
    );
    ZSTD_updateRep((newReps.rep).as_mut_ptr(), offBase, ll0);
    return newReps;
}
#[inline]
unsafe extern "C" fn ZSTD_count(
    mut pIn: *const u8,
    mut pMatch: *const u8,
    pInLimit: *const u8,
) -> libc::size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit
        .offset(
            -((::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_sub(1) as isize),
        );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const libc::c_void)
            ^ MEM_readST(pIn as *const libc::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as libc::size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<libc::size_t>() as libc::c_ulong as isize);
        pMatch = pMatch
            .offset(::core::mem::size_of::<libc::size_t>() as libc::c_ulong as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const libc::c_void)
                ^ MEM_readST(pIn as *const libc::c_void);
            if diff_0 == 0 {
                pIn = pIn
                    .offset(::core::mem::size_of::<libc::size_t>() as libc::c_ulong as isize);
                pMatch = pMatch
                    .offset(::core::mem::size_of::<libc::size_t>() as libc::c_ulong as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as libc::c_long as libc::size_t;
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
static mut prime3bytes: u32 = 506832829 as libc::c_uint;
unsafe extern "C" fn ZSTD_hash3(mut u: u32, mut h: u32, mut s: u32) -> u32 {
    debug_assert!(h <= 32 as libc::c_int as libc::c_uint);
    return ((u << 32 as libc::c_int - 24 as libc::c_int).wrapping_mul(prime3bytes) ^ s)
        >> (32 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
#[inline]
unsafe extern "C" fn ZSTD_hash3Ptr(mut ptr: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash3(MEM_readLE32(ptr), h, 0 as libc::c_int as u32) as libc::size_t;
}
static mut prime4bytes: u32 = 2654435761 as libc::c_uint;
unsafe extern "C" fn ZSTD_hash4(mut u: u32, mut h: u32, mut s: u32) -> u32 {
    debug_assert!(h <= 32 as libc::c_int as libc::c_uint);
    return (u.wrapping_mul(prime4bytes) ^ s)
        >> (32 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash4(MEM_readLE32(ptr), h, 0 as libc::c_int as u32) as libc::size_t;
}
static mut prime5bytes: u64 = 889523592379 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash5(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64 as libc::c_int as libc::c_uint);
    return ((u << 64 as libc::c_int - 40 as libc::c_int).wrapping_mul(prime5bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash5(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
static mut prime6bytes: u64 = 227718039650203 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash6(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64 as libc::c_int as libc::c_uint);
    return ((u << 64 as libc::c_int - 48 as libc::c_int).wrapping_mul(prime6bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash6(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
static mut prime7bytes: u64 = 58295818150454627 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash7(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64 as libc::c_int as libc::c_uint);
    return ((u << 64 as libc::c_int - 56 as libc::c_int).wrapping_mul(prime7bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const libc::c_void, mut h: u32) -> libc::size_t {
    return ZSTD_hash7(MEM_readLE64(p), h, 0 as libc::c_int as u64);
}
static mut prime8bytes: u64 = 0xcf1bbcdcb7a56463 as libc::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash8(mut u: u64, mut h: u32, mut s: u64) -> libc::size_t {
    debug_assert!(h <= 64 as libc::c_int as libc::c_uint);
    return (u.wrapping_mul(prime8bytes) ^ s)
        >> (64 as libc::c_int as libc::c_uint).wrapping_sub(h);
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
    debug_assert!(hBits <= 32 as libc::c_int as libc::c_uint);
    match mls {
        5 => return ZSTD_hash5Ptr(p, hBits),
        6 => return ZSTD_hash6Ptr(p, hBits),
        7 => return ZSTD_hash7Ptr(p, hBits),
        8 => return ZSTD_hash8Ptr(p, hBits),
        4 | _ => return ZSTD_hash4Ptr(p, hBits),
    };
}
pub const ZSTD_LITFREQ_ADD: libc::c_int = 2 as libc::c_int;
pub const ZSTD_MAX_PRICE: libc::c_int = (1 as libc::c_int) << 30 as libc::c_int;
pub const ZSTD_PREDEF_THRESHOLD: libc::c_int = 8 as libc::c_int;
pub const BITCOST_ACCURACY: libc::c_int = 8 as libc::c_int;
pub const BITCOST_MULTIPLIER: libc::c_int = (1 as libc::c_int) << BITCOST_ACCURACY;
#[inline]
unsafe extern "C" fn ZSTD_bitWeight(mut stat: u32) -> u32 {
    return (ZSTD_highbit32(stat.wrapping_add(1)))
        .wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint);
}
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_fracWeight(mut rawStat: u32) -> u32 {
    let stat = rawStat.wrapping_add(1);
    let hb = ZSTD_highbit32(stat);
    let BWeight = hb.wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint);
    let FWeight = stat << BITCOST_ACCURACY >> hb;
    let weight = BWeight.wrapping_add(FWeight);
    debug_assert!(hb.wrapping_add(8)
        < 31 as libc::c_int as libc::c_uint);
    return weight;
}
unsafe extern "C" fn ZSTD_compressedLiterals(optPtr: *const optState_t) -> libc::c_int {
    return ((*optPtr).literalCompressionMode as libc::c_uint
        != ZSTD_ps_disable as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_setBasePrices(
    mut optPtr: *mut optState_t,
    mut optLevel: libc::c_int,
) {
    if ZSTD_compressedLiterals(optPtr) != 0 {
        (*optPtr)
            .litSumBasePrice = if optLevel != 0 {
            ZSTD_fracWeight((*optPtr).litSum)
        } else {
            ZSTD_bitWeight((*optPtr).litSum)
        };
    }
    (*optPtr)
        .litLengthSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).litLengthSum)
    } else {
        ZSTD_bitWeight((*optPtr).litLengthSum)
    };
    (*optPtr)
        .matchLengthSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).matchLengthSum)
    } else {
        ZSTD_bitWeight((*optPtr).matchLengthSum)
    };
    (*optPtr)
        .offCodeSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).offCodeSum)
    } else {
        ZSTD_bitWeight((*optPtr).offCodeSum)
    };
}
unsafe extern "C" fn sum_u32(mut table: *const libc::c_uint, mut nbElts: libc::size_t) -> u32 {
    let mut n: libc::size_t = 0;
    let mut total = 0 as libc::c_int as u32;
    n = 0 as libc::c_int as libc::size_t;
    while n < nbElts {
        total = (total as libc::c_uint).wrapping_add(*table.offset(n as isize)) as u32
            as u32;
        n = n.wrapping_add(1);
    }
    return total;
}
unsafe extern "C" fn ZSTD_downscaleStats(
    mut table: *mut libc::c_uint,
    mut lastEltIndex: u32,
    mut shift: u32,
    mut base1: base_directive_e,
) -> u32 {
    let mut s: u32 = 0;
    let mut sum = 0 as libc::c_int as u32;
    debug_assert!(shift < 30 as libc::c_int as libc::c_uint);
    s = 0 as libc::c_int as u32;
    while s < lastEltIndex.wrapping_add(1) {
        let base = (if base1 as libc::c_uint != 0 {
            1 as libc::c_int
        } else {
            (*table.offset(s as isize) > 0 as libc::c_int as libc::c_uint) as libc::c_int
        }) as libc::c_uint;
        let newStat = base.wrapping_add(*table.offset(s as isize) >> shift);
        sum = (sum as libc::c_uint).wrapping_add(newStat) ;
        *table.offset(s as isize) = newStat;
        s = s.wrapping_add(1);
    }
    return sum;
}
unsafe extern "C" fn ZSTD_scaleStats(
    mut table: *mut libc::c_uint,
    mut lastEltIndex: u32,
    mut logTarget: u32,
) -> u32 {
    let prevsum = sum_u32(
        table as *const libc::c_uint,
        lastEltIndex.wrapping_add(1) as libc::size_t,
    );
    let factor = prevsum >> logTarget;
    debug_assert!(logTarget < 30 as libc::c_int as libc::c_uint);
    if factor <= 1 as libc::c_int as libc::c_uint {
        return prevsum;
    }
    return ZSTD_downscaleStats(
        table,
        lastEltIndex,
        ZSTD_highbit32(factor),
        base_1guaranteed,
    );
}
unsafe extern "C" fn ZSTD_rescaleFreqs(
    optPtr: *mut optState_t,
    src: *const u8,
    srcSize: libc::size_t,
    optLevel: libc::c_int,
) {
    let compressedLiterals = ZSTD_compressedLiterals(optPtr);
    (*optPtr).priceType = zop_dynamic;
    if (*optPtr).litLengthSum == 0 as libc::c_int as libc::c_uint {
        if srcSize <= ZSTD_PREDEF_THRESHOLD as libc::c_ulong {
            (*optPtr).priceType = zop_predef;
        }
        debug_assert!(!((*optPtr).symbolCosts).is_null());
        if (*(*optPtr).symbolCosts).huf.repeatMode as libc::c_uint
            == HUF_repeat_valid as libc::c_int as libc::c_uint
        {
            (*optPtr).priceType = zop_dynamic;
            if compressedLiterals != 0 {
                let mut lit: libc::c_uint = 0;
                debug_assert!(!((*optPtr).litFreq).is_null());
                (*optPtr).litSum = 0 as libc::c_int as u32;
                lit = 0 as libc::c_int as libc::c_uint;
                while lit <= MaxLit as libc::c_uint {
                    let scaleLog = 11 as libc::c_int as u32;
                    let bitCost = HUF_getNbBitsFromCTable(
                        ((*(*optPtr).symbolCosts).huf.CTable).as_ptr(),
                        lit,
                    );
                    debug_assert!(bitCost <= scaleLog);
                    *((*optPtr).litFreq)
                        .offset(
                            lit as isize,
                        ) = (if bitCost != 0 {
                        (1 as libc::c_int) << scaleLog.wrapping_sub(bitCost)
                    } else {
                        1 as libc::c_int
                    }) as libc::c_uint;
                    (*optPtr)
                        .litSum = ((*optPtr).litSum as libc::c_uint)
                        .wrapping_add(*((*optPtr).litFreq).offset(lit as isize)) as u32
                        as u32;
                    lit = lit.wrapping_add(1);
                }
            }
            let mut ll: libc::c_uint = 0;
            let mut llstate = FSE_CState_t {
                value: 0,
                stateTable: 0 as *const libc::c_void,
                symbolTT: 0 as *const libc::c_void,
                stateLog: 0,
            };
            FSE_initCState(
                &mut llstate,
                ((*(*optPtr).symbolCosts).fse.litlengthCTable).as_ptr(),
            );
            (*optPtr).litLengthSum = 0 as libc::c_int as u32;
            ll = 0 as libc::c_int as libc::c_uint;
            while ll <= MaxLL as libc::c_uint {
                let scaleLog_0 = 10 as libc::c_int as u32;
                let bitCost_0 = FSE_getMaxNbBits(llstate.symbolTT, ll);
                debug_assert!(bitCost_0 < scaleLog_0);
                *((*optPtr).litLengthFreq)
                    .offset(
                        ll as isize,
                    ) = (if bitCost_0 != 0 {
                    (1 as libc::c_int) << scaleLog_0.wrapping_sub(bitCost_0)
                } else {
                    1 as libc::c_int
                }) as libc::c_uint;
                (*optPtr)
                    .litLengthSum = ((*optPtr).litLengthSum as libc::c_uint)
                    .wrapping_add(*((*optPtr).litLengthFreq).offset(ll as isize)) as u32
                    as u32;
                ll = ll.wrapping_add(1);
            }
            let mut ml: libc::c_uint = 0;
            let mut mlstate = FSE_CState_t {
                value: 0,
                stateTable: 0 as *const libc::c_void,
                symbolTT: 0 as *const libc::c_void,
                stateLog: 0,
            };
            FSE_initCState(
                &mut mlstate,
                ((*(*optPtr).symbolCosts).fse.matchlengthCTable).as_ptr(),
            );
            (*optPtr).matchLengthSum = 0 as libc::c_int as u32;
            ml = 0 as libc::c_int as libc::c_uint;
            while ml <= MaxML as libc::c_uint {
                let scaleLog_1 = 10 as libc::c_int as u32;
                let bitCost_1 = FSE_getMaxNbBits(mlstate.symbolTT, ml);
                debug_assert!(bitCost_1 < scaleLog_1);
                *((*optPtr).matchLengthFreq)
                    .offset(
                        ml as isize,
                    ) = (if bitCost_1 != 0 {
                    (1 as libc::c_int) << scaleLog_1.wrapping_sub(bitCost_1)
                } else {
                    1 as libc::c_int
                }) as libc::c_uint;
                (*optPtr)
                    .matchLengthSum = ((*optPtr).matchLengthSum as libc::c_uint)
                    .wrapping_add(*((*optPtr).matchLengthFreq).offset(ml as isize))
                    ;
                ml = ml.wrapping_add(1);
            }
            let mut of: libc::c_uint = 0;
            let mut ofstate = FSE_CState_t {
                value: 0,
                stateTable: 0 as *const libc::c_void,
                symbolTT: 0 as *const libc::c_void,
                stateLog: 0,
            };
            FSE_initCState(
                &mut ofstate,
                ((*(*optPtr).symbolCosts).fse.offcodeCTable).as_ptr(),
            );
            (*optPtr).offCodeSum = 0 as libc::c_int as u32;
            of = 0 as libc::c_int as libc::c_uint;
            while of <= MaxOff as libc::c_uint {
                let scaleLog_2 = 10 as libc::c_int as u32;
                let bitCost_2 = FSE_getMaxNbBits(ofstate.symbolTT, of);
                debug_assert!(bitCost_2 < scaleLog_2);
                *((*optPtr).offCodeFreq)
                    .offset(
                        of as isize,
                    ) = (if bitCost_2 != 0 {
                    (1 as libc::c_int) << scaleLog_2.wrapping_sub(bitCost_2)
                } else {
                    1 as libc::c_int
                }) as libc::c_uint;
                (*optPtr)
                    .offCodeSum = ((*optPtr).offCodeSum as libc::c_uint)
                    .wrapping_add(*((*optPtr).offCodeFreq).offset(of as isize)) as u32
                    as u32;
                of = of.wrapping_add(1);
            }
        } else {
            debug_assert!(!((*optPtr).litFreq).is_null());
            if compressedLiterals != 0 {
                let mut lit_0 = MaxLit as libc::c_uint;
                HIST_count_simple(
                    (*optPtr).litFreq,
                    &mut lit_0,
                    src as *const libc::c_void,
                    srcSize,
                );
                (*optPtr)
                    .litSum = ZSTD_downscaleStats(
                    (*optPtr).litFreq,
                    MaxLit as u32,
                    8 as libc::c_int as u32,
                    base_0possible,
                );
            }
            let baseLLfreqs: [libc::c_uint; 36] = [
                4 as libc::c_int as libc::c_uint,
                2 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ];
            libc::memcpy(
                (*optPtr).litLengthFreq as *mut libc::c_void,
                baseLLfreqs.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uint; 36]>() as libc::c_ulong
                    as libc::size_t,
            );
            (*optPtr)
                .litLengthSum = sum_u32(
                baseLLfreqs.as_ptr(),
                (MaxLL + 1 as libc::c_int) as libc::size_t,
            );
            let mut ml_0: libc::c_uint = 0;
            ml_0 = 0 as libc::c_int as libc::c_uint;
            while ml_0 <= MaxML as libc::c_uint {
                *((*optPtr).matchLengthFreq)
                    .offset(ml_0 as isize) = 1 as libc::c_int as libc::c_uint;
                ml_0 = ml_0.wrapping_add(1);
            }
            (*optPtr).matchLengthSum = (MaxML + 1 as libc::c_int) as u32;
            let baseOFCfreqs: [libc::c_uint; 32] = [
                6 as libc::c_int as libc::c_uint,
                2 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                2 as libc::c_int as libc::c_uint,
                3 as libc::c_int as libc::c_uint,
                4 as libc::c_int as libc::c_uint,
                4 as libc::c_int as libc::c_uint,
                4 as libc::c_int as libc::c_uint,
                3 as libc::c_int as libc::c_uint,
                2 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ];
            libc::memcpy(
                (*optPtr).offCodeFreq as *mut libc::c_void,
                baseOFCfreqs.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong
                    as libc::size_t,
            );
            (*optPtr)
                .offCodeSum = sum_u32(
                baseOFCfreqs.as_ptr(),
                (MaxOff + 1 as libc::c_int) as libc::size_t,
            );
        }
    } else {
        if compressedLiterals != 0 {
            (*optPtr)
                .litSum = ZSTD_scaleStats(
                (*optPtr).litFreq,
                MaxLit as u32,
                12 as libc::c_int as u32,
            );
        }
        (*optPtr)
            .litLengthSum = ZSTD_scaleStats(
            (*optPtr).litLengthFreq,
            MaxLL as u32,
            11 as libc::c_int as u32,
        );
        (*optPtr)
            .matchLengthSum = ZSTD_scaleStats(
            (*optPtr).matchLengthFreq,
            MaxML as u32,
            11 as libc::c_int as u32,
        );
        (*optPtr)
            .offCodeSum = ZSTD_scaleStats(
            (*optPtr).offCodeFreq,
            MaxOff as u32,
            11 as libc::c_int as u32,
        );
    }
    ZSTD_setBasePrices(optPtr, optLevel);
}
unsafe extern "C" fn ZSTD_rawLiteralsCost(
    literals: *const u8,
    litLength: u32,
    optPtr: *const optState_t,
    mut optLevel: libc::c_int,
) -> u32 {
    if litLength == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as u32;
    }
    if ZSTD_compressedLiterals(optPtr) == 0 {
        return (litLength << 3 as libc::c_int)
            .wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint);
    }
    if (*optPtr).priceType as libc::c_uint == zop_predef as libc::c_int as libc::c_uint {
        return litLength
            .wrapping_mul(6)
            .wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint);
    }
    let mut price = ((*optPtr).litSumBasePrice).wrapping_mul(litLength);
    let litPriceMax = ((*optPtr).litSumBasePrice)
        .wrapping_sub(BITCOST_MULTIPLIER as libc::c_uint);
    let mut u: u32 = 0;
    debug_assert!((*optPtr).litSumBasePrice
        >= ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint);
    u = 0 as libc::c_int as u32;
    while u < litLength {
        let mut litPrice = if optLevel != 0 {
            ZSTD_fracWeight(
                *((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize),
            )
        } else {
            ZSTD_bitWeight(
                *((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize),
            )
        };
        if (litPrice > litPriceMax) as libc::c_int as libc::c_long != 0 {
            litPrice = litPriceMax;
        }
        price = (price as libc::c_uint).wrapping_sub(litPrice) ;
        u = u.wrapping_add(1);
    }
    return price;
}
unsafe extern "C" fn ZSTD_litLengthPrice(
    litLength: u32,
    optPtr: *const optState_t,
    mut optLevel: libc::c_int,
) -> u32 {
    debug_assert!(litLength <= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint);
    if (*optPtr).priceType as libc::c_uint == zop_predef as libc::c_int as libc::c_uint {
        return if optLevel != 0 {
            ZSTD_fracWeight(litLength)
        } else {
            ZSTD_bitWeight(litLength)
        };
    }
    if litLength == ZSTD_BLOCKSIZE_MAX as libc::c_uint {
        return (BITCOST_MULTIPLIER as libc::c_uint)
            .wrapping_add(
                ZSTD_litLengthPrice(
                    (ZSTD_BLOCKSIZE_MAX - 1 as libc::c_int) as u32,
                    optPtr,
                    optLevel,
                ),
            );
    }
    let llCode = ZSTD_LLcode(litLength);
    return ((LL_bits[llCode as usize] as libc::c_int * BITCOST_MULTIPLIER)
        as libc::c_uint)
        .wrapping_add((*optPtr).litLengthSumBasePrice)
        .wrapping_sub(
            (if optLevel != 0 {
                ZSTD_fracWeight(*((*optPtr).litLengthFreq).offset(llCode as isize))
            } else {
                ZSTD_bitWeight(*((*optPtr).litLengthFreq).offset(llCode as isize))
            }),
        );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_getMatchPrice(
    offBase: u32,
    matchLength: u32,
    optPtr: *const optState_t,
    optLevel: libc::c_int,
) -> u32 {
    let mut price: u32 = 0;
    let offCode = ZSTD_highbit32(offBase);
    let mlBase = matchLength.wrapping_sub(MINMATCH as libc::c_uint);
    debug_assert!(matchLength >= 3 as libc::c_int as libc::c_uint);
    if (*optPtr).priceType as libc::c_uint == zop_predef as libc::c_int as libc::c_uint {
        return (if optLevel != 0 {
            ZSTD_fracWeight(mlBase)
        } else {
            ZSTD_bitWeight(mlBase)
        })
            .wrapping_add(
                (16 as libc::c_int as libc::c_uint)
                    .wrapping_add(offCode)
                    .wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint),
            );
    }
    price = offCode
        .wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint)
        .wrapping_add(
            ((*optPtr).offCodeSumBasePrice)
                .wrapping_sub(
                    (if optLevel != 0 {
                        ZSTD_fracWeight(
                            *((*optPtr).offCodeFreq).offset(offCode as isize),
                        )
                    } else {
                        ZSTD_bitWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
                    }),
                ),
        );
    if optLevel < 2 as libc::c_int && offCode >= 20 as libc::c_int as libc::c_uint {
        price = (price as libc::c_uint)
            .wrapping_add(
                offCode
                    .wrapping_sub(19)
                    .wrapping_mul(2)
                    .wrapping_mul(BITCOST_MULTIPLIER as libc::c_uint),
            ) ;
    }
    let mlCode = ZSTD_MLcode(mlBase);
    price = (price as libc::c_uint)
        .wrapping_add(
            ((ML_bits[mlCode as usize] as libc::c_int * BITCOST_MULTIPLIER)
                as libc::c_uint)
                .wrapping_add(
                    ((*optPtr).matchLengthSumBasePrice)
                        .wrapping_sub(
                            (if optLevel != 0 {
                                ZSTD_fracWeight(
                                    *((*optPtr).matchLengthFreq).offset(mlCode as isize),
                                )
                            } else {
                                ZSTD_bitWeight(
                                    *((*optPtr).matchLengthFreq).offset(mlCode as isize),
                                )
                            }),
                        ),
                ),
        ) ;
    price = (price as libc::c_uint)
        .wrapping_add((BITCOST_MULTIPLIER / 5 as libc::c_int) as libc::c_uint) as u32
        as u32;
    return price;
}
unsafe extern "C" fn ZSTD_updateStats(
    optPtr: *mut optState_t,
    mut litLength: u32,
    mut literals: *const u8,
    mut offBase: u32,
    mut matchLength: u32,
) {
    if ZSTD_compressedLiterals(optPtr) != 0 {
        let mut u: u32 = 0;
        u = 0 as libc::c_int as u32;
        while u < litLength {
            let ref mut fresh2 = *((*optPtr).litFreq)
                .offset(*literals.offset(u as isize) as isize);
            *fresh2 = (*fresh2).wrapping_add(ZSTD_LITFREQ_ADD as libc::c_uint);
            u = u.wrapping_add(1);
        }
        (*optPtr)
            .litSum = ((*optPtr).litSum as libc::c_uint)
            .wrapping_add(litLength.wrapping_mul(ZSTD_LITFREQ_ADD as libc::c_uint))
            ;
    }
    let llCode = ZSTD_LLcode(litLength);
    let ref mut fresh3 = *((*optPtr).litLengthFreq).offset(llCode as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    (*optPtr).litLengthSum = ((*optPtr).litLengthSum).wrapping_add(1);
    let offCode = ZSTD_highbit32(offBase);
    debug_assert!(offCode <= 31 as libc::c_int as libc::c_uint);
    let ref mut fresh4 = *((*optPtr).offCodeFreq).offset(offCode as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    (*optPtr).offCodeSum = ((*optPtr).offCodeSum).wrapping_add(1);
    let mlBase = matchLength.wrapping_sub(MINMATCH as libc::c_uint);
    let mlCode = ZSTD_MLcode(mlBase);
    let ref mut fresh5 = *((*optPtr).matchLengthFreq).offset(mlCode as isize);
    *fresh5 = (*fresh5).wrapping_add(1);
    (*optPtr).matchLengthSum = ((*optPtr).matchLengthSum).wrapping_add(1);
}
#[inline]
unsafe extern "C" fn ZSTD_readMINMATCH(
    mut memPtr: *const libc::c_void,
    mut length: u32,
) -> u32 {
    match length {
        3 => {
            if MEM_isLittleEndian() != 0 {
                return MEM_read32(memPtr) << 8 as libc::c_int
            } else {
                return MEM_read32(memPtr) >> 8 as libc::c_int
            }
        }
        4 | _ => return MEM_read32(memPtr),
    };
}
unsafe extern "C" fn ZSTD_insertAndFindFirstIndexHash3(
    mut ms: *const ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    ip: *const u8,
) -> u32 {
    let hashTable3 = (*ms).hashTable3;
    let hashLog3 = (*ms).hashLog3;
    let base = (*ms).window.base;
    let mut idx = *nextToUpdate3;
    let target = ip.offset_from(base) as libc::c_long as u32;
    let hash3 = ZSTD_hash3Ptr(ip as *const libc::c_void, hashLog3);
    debug_assert!(hashLog3 > 0 as libc::c_int as libc::c_uint);
    while idx < target {
        *hashTable3
            .offset(
                ZSTD_hash3Ptr(base.offset(idx as isize) as *const libc::c_void, hashLog3)
                    as isize,
            ) = idx;
        idx = idx.wrapping_add(1);
    }
    *nextToUpdate3 = target;
    return *hashTable3.offset(hash3 as isize);
}
unsafe extern "C" fn ZSTD_insertBt1(
    mut ms: *const ZSTD_matchState_t,
    ip: *const u8,
    iend: *const u8,
    target: u32,
    mls: u32,
    extDict: libc::c_int,
) -> u32 {
    let cParams: *const ZSTD_compressionParameters = &(*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1);
    let btMask = (((1 as libc::c_int) << btLog) - 1 as libc::c_int) as u32;
    let mut matchIndex = *hashTable.offset(h as isize);
    let mut commonLengthSmaller = 0 as libc::c_int as libc::size_t;
    let mut commonLengthLarger = 0 as libc::c_int as libc::size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut match_0 = 0 as *const u8;
    let curr = ip.offset_from(base) as libc::c_long as u32;
    let btLow = if btMask >= curr {
        0 as libc::c_int as libc::c_uint
    } else {
        curr.wrapping_sub(btMask)
    };
    let mut smallerPtr = bt
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(curr & btMask) as isize);
    let mut largerPtr = smallerPtr.offset(1 as libc::c_int as isize);
    let mut dummy32: u32 = 0;
    let windowLow = ZSTD_getLowestMatchIndex(ms, target, (*cParams).windowLog);
    let mut matchEndIdx = curr
        .wrapping_add(8)
        .wrapping_add(1);
    let mut bestLength = 8 as libc::c_int as libc::size_t;
    let mut nbCompares = (1 as libc::c_uint) << (*cParams).searchLog;
    debug_assert!(curr <= target);
    debug_assert!(ip <= iend.offset(-(8 as libc::c_int as isize)));
    *hashTable.offset(h as isize) = curr;
    debug_assert!(windowLow > 0 as libc::c_int as libc::c_uint);
    while nbCompares != 0 && matchIndex >= windowLow {
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
        debug_assert!(matchIndex < curr);
        if extDict == 0
            || (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
        {
            debug_assert!((matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong);
            match_0 = base.offset(matchIndex as isize);
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count(
                        ip.offset(matchLength as isize),
                        match_0.offset(matchLength as isize),
                        iend,
                    ),
                ) ;
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
                ) ;
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
            {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            bestLength = matchLength;
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
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
    *largerPtr = 0 as libc::c_int as u32;
    *smallerPtr = *largerPtr;
    let mut positions = 0 as libc::c_int as u32;
    if bestLength > 384 as libc::c_int as libc::c_ulong {
        positions = if (192 as libc::c_int as libc::c_uint)
            < bestLength.wrapping_sub(384) as u32
        {
            192 as libc::c_int as libc::c_uint
        } else {
            bestLength.wrapping_sub(384) as u32
        };
    }
    debug_assert!(matchEndIdx > curr.wrapping_add(8));
    return if positions
        > matchEndIdx.wrapping_sub(curr.wrapping_add(8))
    {
        positions
    } else {
        matchEndIdx.wrapping_sub(curr.wrapping_add(8))
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_updateTree_internal(
    mut ms: *mut ZSTD_matchState_t,
    ip: *const u8,
    iend: *const u8,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) {
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as libc::c_long as u32;
    let mut idx = (*ms).nextToUpdate;
    while idx < target {
        let forward = ZSTD_insertBt1(
            ms,
            base.offset(idx as isize),
            iend,
            target,
            mls,
            (dictMode as libc::c_uint == ZSTD_extDict as libc::c_int as libc::c_uint)
                as libc::c_int,
        );
        debug_assert!(idx < idx.wrapping_add(forward));
        idx = (idx as libc::c_uint).wrapping_add(forward) ;
    }
    debug_assert!(ip.offset_from(base) as libc::c_long as libc::size_t
        <= -(1 as libc::c_int) as u32 as libc::size_t);
    debug_assert!(iend.offset_from(base) as libc::c_long as libc::size_t
        <= -(1 as libc::c_int) as u32 as libc::size_t);
    (*ms).nextToUpdate = target;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_updateTree(
    mut ms: *mut ZSTD_matchState_t,
    mut ip: *const u8,
    mut iend: *const u8,
) {
    ZSTD_updateTree_internal(ms, ip, iend, (*ms).cParams.minMatch, ZSTD_noDict);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_insertBtAndGetAllMatches(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    ip: *const u8,
    iLimit: *const u8,
    dictMode: ZSTD_dictMode_e,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
    mls: u32,
) -> u32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let sufficient_len = if (*cParams).targetLength
        < (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int) as libc::c_uint
    {
        (*cParams).targetLength
    } else {
        (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int) as libc::c_uint
    };
    let base = (*ms).window.base;
    let curr = ip.offset_from(base) as libc::c_long as u32;
    let hashLog = (*cParams).hashLog;
    let minMatch = (if mls == 3 as libc::c_int as libc::c_uint {
        3 as libc::c_int
    } else {
        4 as libc::c_int
    }) as u32;
    let hashTable = (*ms).hashTable;
    let h = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.offset(h as isize);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1);
    let btMask = ((1 as libc::c_uint) << btLog)
        .wrapping_sub(1);
    let mut commonLengthSmaller = 0 as libc::c_int as libc::size_t;
    let mut commonLengthLarger = 0 as libc::c_int as libc::size_t;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let btLow = if btMask >= curr {
        0 as libc::c_int as libc::c_uint
    } else {
        curr.wrapping_sub(btMask)
    };
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, (*cParams).windowLog);
    let matchLow = if windowLow != 0 {
        windowLow
    } else {
        1 as libc::c_int as libc::c_uint
    };
    let mut smallerPtr = bt
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(curr & btMask) as isize);
    let mut largerPtr = bt
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(curr & btMask) as isize)
        .offset(1 as libc::c_int as isize);
    let mut matchEndIdx = curr
        .wrapping_add(8)
        .wrapping_add(1);
    let mut dummy32: u32 = 0;
    let mut mnum = 0 as libc::c_int as u32;
    let mut nbCompares = (1 as libc::c_uint) << (*cParams).searchLog;
    let mut dms = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        (*ms).dictMatchState
    } else {
        NULL as *const ZSTD_matchState_t
    };
    let dmsCParams = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        &(*dms).cParams
    } else {
        NULL as *const ZSTD_compressionParameters
    };
    let dmsBase = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        (*dms).window.base
    } else {
        NULL as *const u8
    };
    let dmsEnd = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        (*dms).window.nextSrc
    } else {
        NULL as *const u8
    };
    let dmsHighLimit = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        dmsEnd.offset_from(dmsBase) as libc::c_long as u32
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let dmsLowLimit = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        (*dms).window.lowLimit
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let dmsIndexDelta = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        windowLow.wrapping_sub(dmsHighLimit)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let dmsHashLog = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        (*dmsCParams).hashLog
    } else {
        hashLog
    };
    let dmsBtLog = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        ((*dmsCParams).chainLog).wrapping_sub(1)
    } else {
        btLog
    };
    let dmsBtMask = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
    {
        ((1 as libc::c_uint) << dmsBtLog).wrapping_sub(1)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let dmsBtLow = if dictMode as libc::c_uint
        == ZSTD_dictMatchState as libc::c_int as libc::c_uint
        && dmsBtMask < dmsHighLimit.wrapping_sub(dmsLowLimit)
    {
        dmsHighLimit.wrapping_sub(dmsBtMask)
    } else {
        dmsLowLimit
    };
    let mut bestLength = lengthToBeat.wrapping_sub(1)
        as libc::size_t;
    debug_assert!(ll0 <= 1 as libc::c_int as libc::c_uint);
    let lastR = (ZSTD_REP_NUM as libc::c_uint).wrapping_add(ll0);
    let mut repCode: u32 = 0;
    repCode = ll0;
    while repCode < lastR {
        let repOffset = if repCode == ZSTD_REP_NUM as libc::c_uint {
            (*rep.offset(0 as libc::c_int as isize))
                .wrapping_sub(1)
        } else {
            *rep.offset(repCode as isize)
        };
        let repIndex = curr.wrapping_sub(repOffset);
        let mut repLen = 0 as libc::c_int as u32;
        debug_assert!(curr >= dictLimit);
        if repOffset.wrapping_sub(1)
            < curr.wrapping_sub(dictLimit)
        {
            if (repIndex >= windowLow) as libc::c_int
                & (ZSTD_readMINMATCH(ip as *const libc::c_void, minMatch)
                    == ZSTD_readMINMATCH(
                        ip.offset(-(repOffset as isize)) as *const libc::c_void,
                        minMatch,
                    )) as libc::c_int != 0
            {
                repLen = (ZSTD_count(
                    ip.offset(minMatch as isize),
                    ip.offset(minMatch as isize).offset(-(repOffset as isize)),
                    iLimit,
                ) as u32)
                    .wrapping_add(minMatch);
            }
        } else {
            let repMatch = if dictMode as libc::c_uint
                == ZSTD_dictMatchState as libc::c_int as libc::c_uint
            {
                dmsBase.offset(repIndex as isize).offset(-(dmsIndexDelta as isize))
            } else {
                dictBase.offset(repIndex as isize)
            };
            debug_assert!(curr >= windowLow);
            if dictMode as libc::c_uint == ZSTD_extDict as libc::c_int as libc::c_uint
                && (repOffset.wrapping_sub(1)
                    < curr.wrapping_sub(windowLow)) as libc::c_int
                    & (dictLimit
                        .wrapping_sub(1)
                        .wrapping_sub(repIndex) >= 3 as libc::c_int as libc::c_uint)
                        as libc::c_int != 0
                && ZSTD_readMINMATCH(ip as *const libc::c_void, minMatch)
                    == ZSTD_readMINMATCH(repMatch as *const libc::c_void, minMatch)
            {
                repLen = (ZSTD_count_2segments(
                    ip.offset(minMatch as isize),
                    repMatch.offset(minMatch as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ) as u32)
                    .wrapping_add(minMatch);
            }
            if dictMode as libc::c_uint
                == ZSTD_dictMatchState as libc::c_int as libc::c_uint
                && (repOffset.wrapping_sub(1)
                    < curr.wrapping_sub(dmsLowLimit.wrapping_add(dmsIndexDelta)))
                    as libc::c_int
                    & (dictLimit
                        .wrapping_sub(1)
                        .wrapping_sub(repIndex) >= 3 as libc::c_int as libc::c_uint)
                        as libc::c_int != 0
                && ZSTD_readMINMATCH(ip as *const libc::c_void, minMatch)
                    == ZSTD_readMINMATCH(repMatch as *const libc::c_void, minMatch)
            {
                repLen = (ZSTD_count_2segments(
                    ip.offset(minMatch as isize),
                    repMatch.offset(minMatch as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ) as u32)
                    .wrapping_add(minMatch);
            }
        }
        if repLen as libc::c_ulong > bestLength {
            bestLength = repLen as libc::size_t;
            debug_assert!(repCode.wrapping_sub(ll0).wrapping_add(1)
                >= 1 as libc::c_int as libc::c_uint);
            debug_assert!(repCode.wrapping_sub(ll0).wrapping_add(1)
                <= 3 as libc::c_int as libc::c_uint);
            (*matches.offset(mnum as isize))
                .off = repCode
                .wrapping_sub(ll0)
                .wrapping_add(1);
            (*matches.offset(mnum as isize)).len = repLen;
            mnum = mnum.wrapping_add(1);
            if (repLen > sufficient_len) as libc::c_int
                | (ip.offset(repLen as isize) == iLimit) as libc::c_int != 0
            {
                return mnum;
            }
        }
        repCode = repCode.wrapping_add(1);
    }
    if mls == 3 as libc::c_int as libc::c_uint && bestLength < mls as libc::c_ulong {
        let matchIndex3 = ZSTD_insertAndFindFirstIndexHash3(ms, nextToUpdate3, ip);
        if (matchIndex3 >= matchLow) as libc::c_int
            & (curr.wrapping_sub(matchIndex3)
                < ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_uint)
                as libc::c_int != 0
        {
            let mut mlen: libc::size_t = 0;
            if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint
                || dictMode as libc::c_uint
                    == ZSTD_dictMatchState as libc::c_int as libc::c_uint
                || matchIndex3 >= dictLimit
            {
                let match_0 = base.offset(matchIndex3 as isize);
                mlen = ZSTD_count(ip, match_0, iLimit);
            } else {
                let match_1 = dictBase.offset(matchIndex3 as isize);
                mlen = ZSTD_count_2segments(ip, match_1, iLimit, dictEnd, prefixStart);
            }
            if mlen >= mls as libc::c_ulong {
                bestLength = mlen;
                debug_assert!(curr > matchIndex3);
                debug_assert!(mnum == 0 as libc::c_int as libc::c_uint);
                debug_assert!(curr.wrapping_sub(matchIndex3) > 0 as libc::c_int as libc::c_uint);
                (*matches.offset(0 as libc::c_int as isize))
                    .off = curr
                    .wrapping_sub(matchIndex3)
                    .wrapping_add(ZSTD_REP_NUM as libc::c_uint);
                (*matches.offset(0 as libc::c_int as isize)).len = mlen as u32;
                mnum = 1 as libc::c_int as u32;
                if (mlen > sufficient_len as libc::c_ulong) as libc::c_int
                    | (ip.offset(mlen as isize) == iLimit) as libc::c_int != 0
                {
                    (*ms)
                        .nextToUpdate = curr
                        .wrapping_add(1);
                    return 1 as libc::c_int as u32;
                }
            }
        }
    }
    *hashTable.offset(h as isize) = curr;
    while nbCompares != 0 && matchIndex >= matchLow {
        let nextPtr = bt
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(matchIndex & btMask)
                    as isize,
            );
        let mut match_2 = 0 as *const u8;
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        debug_assert!(curr > matchIndex);
        if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint
            || dictMode as libc::c_uint
                == ZSTD_dictMatchState as libc::c_int as libc::c_uint
            || (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
        {
            debug_assert!((matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong);
            match_2 = base.offset(matchIndex as isize);
            if matchIndex >= dictLimit {
                debug_assert!(memcmp(
                    match_2 as *const libc::c_void,
                    ip as *const libc::c_void,
                    matchLength,
                ) == 0 as libc::c_int);
            }
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count(
                        ip.offset(matchLength as isize),
                        match_2.offset(matchLength as isize),
                        iLimit,
                    ),
                ) ;
        } else {
            match_2 = dictBase.offset(matchIndex as isize);
            debug_assert!(memcmp(
                match_2 as *const libc::c_void,
                ip as *const libc::c_void,
                matchLength,
            ) == 0 as libc::c_int);
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count_2segments(
                        ip.offset(matchLength as isize),
                        match_2.offset(matchLength as isize),
                        iLimit,
                        dictEnd,
                        prefixStart,
                    ),
                ) ;
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                >= dictLimit as libc::c_ulong
            {
                match_2 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            debug_assert!(matchEndIdx > matchIndex);
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
            }
            bestLength = matchLength;
            debug_assert!(curr.wrapping_sub(matchIndex) > 0 as libc::c_int as libc::c_uint);
            (*matches.offset(mnum as isize))
                .off = curr
                .wrapping_sub(matchIndex)
                .wrapping_add(ZSTD_REP_NUM as libc::c_uint);
            (*matches.offset(mnum as isize)).len = matchLength as u32;
            mnum = mnum.wrapping_add(1);
            if (matchLength > ZSTD_OPT_NUM as libc::c_ulong) as libc::c_int
                | (ip.offset(matchLength as isize) == iLimit) as libc::c_int != 0
            {
                if dictMode as libc::c_uint
                    == ZSTD_dictMatchState as libc::c_int as libc::c_uint
                {
                    nbCompares = 0 as libc::c_int as u32;
                }
                break;
            }
        }
        if (*match_2.offset(matchLength as isize) as libc::c_int)
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
    *largerPtr = 0 as libc::c_int as u32;
    *smallerPtr = *largerPtr;
    debug_assert!(nbCompares
        <= (1 as libc::c_uint)
            << (if ::core::mem::size_of::<libc::size_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
            {
                30 as libc::c_int
            } else {
                31 as libc::c_int
            }) - 1 as libc::c_int);
    if dictMode as libc::c_uint == ZSTD_dictMatchState as libc::c_int as libc::c_uint
        && nbCompares != 0
    {
        let dmsH = ZSTD_hashPtr(ip as *const libc::c_void, dmsHashLog, mls);
        let mut dictMatchIndex = *((*dms).hashTable).offset(dmsH as isize);
        let dmsBt: *const u32 = (*dms).chainTable;
        commonLengthLarger = 0 as libc::c_int as libc::size_t;
        commonLengthSmaller = commonLengthLarger;
        while nbCompares != 0 && dictMatchIndex > dmsLowLimit {
            let nextPtr_0 = dmsBt
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(dictMatchIndex & dmsBtMask) as isize,
                );
            let mut matchLength_0 = if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else {
                commonLengthLarger
            };
            let mut match_3 = dmsBase.offset(dictMatchIndex as isize);
            matchLength_0 = (matchLength_0 as libc::c_ulong)
                .wrapping_add(
                    ZSTD_count_2segments(
                        ip.offset(matchLength_0 as isize),
                        match_3.offset(matchLength_0 as isize),
                        iLimit,
                        dmsEnd,
                        prefixStart,
                    ),
                ) ;
            if (dictMatchIndex as libc::c_ulong).wrapping_add(matchLength_0)
                >= dmsHighLimit as libc::c_ulong
            {
                match_3 = base
                    .offset(dictMatchIndex as isize)
                    .offset(dmsIndexDelta as isize);
            }
            if matchLength_0 > bestLength {
                matchIndex = dictMatchIndex.wrapping_add(dmsIndexDelta);
                if matchLength_0 > matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong
                {
                    matchEndIdx = matchIndex.wrapping_add(matchLength_0 as u32);
                }
                bestLength = matchLength_0;
                debug_assert!(curr.wrapping_sub(matchIndex) > 0 as libc::c_int as libc::c_uint);
                (*matches.offset(mnum as isize))
                    .off = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as libc::c_uint);
                (*matches.offset(mnum as isize)).len = matchLength_0 as u32;
                mnum = mnum.wrapping_add(1);
                if (matchLength_0 > ZSTD_OPT_NUM as libc::c_ulong) as libc::c_int
                    | (ip.offset(matchLength_0 as isize) == iLimit) as libc::c_int != 0
                {
                    break;
                }
            }
            if dictMatchIndex <= dmsBtLow {
                break;
            }
            if (*match_3.offset(matchLength_0 as isize) as libc::c_int)
                < *ip.offset(matchLength_0 as isize) as libc::c_int
            {
                commonLengthSmaller = matchLength_0;
                dictMatchIndex = *nextPtr_0.offset(1 as libc::c_int as isize);
            } else {
                commonLengthLarger = matchLength_0;
                dictMatchIndex = *nextPtr_0.offset(0 as libc::c_int as isize);
            }
            nbCompares = nbCompares.wrapping_sub(1);
        }
    }
    debug_assert!(matchEndIdx > curr.wrapping_add(8));
    (*ms).nextToUpdate = matchEndIdx.wrapping_sub(8);
    return mnum;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_btGetAllMatches_internal(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
    dictMode: ZSTD_dictMode_e,
    mls: u32,
) -> u32 {
    debug_assert!((if 3 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        3 as libc::c_int as libc::c_uint
    } else {
        (if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as libc::c_int as libc::c_uint
        })
    }) == mls);
    if ip < ((*ms).window.base).offset((*ms).nextToUpdate as isize) {
        return 0 as libc::c_int as u32;
    }
    ZSTD_updateTree_internal(ms, ip, iHighLimit, mls, dictMode);
    return ZSTD_insertBtAndGetAllMatches(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        dictMode,
        rep,
        ll0,
        lengthToBeat,
        mls,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_5(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        5 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_3(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        3 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_6(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        6 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_4(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        4 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_4(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        4 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_6(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        6 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_3(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        3 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_5(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        5 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_3(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        3 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_6(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        6 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_4(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        4 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_5(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_matchState_t,
    mut nextToUpdate3: *mut u32,
    mut ip: *const u8,
    iHighLimit: *const u8,
    mut rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    return ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        5 as libc::c_int as u32,
    );
}
unsafe extern "C" fn ZSTD_selectBtGetAllMatches(
    mut ms: *const ZSTD_matchState_t,
    dictMode: ZSTD_dictMode_e,
) -> ZSTD_getAllMatchesFn {
    let getAllMatchesFns: [[ZSTD_getAllMatchesFn; 4]; 3] = [
        [
            Some(
                ZSTD_btGetAllMatches_noDict_3
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_4
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_5
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_6
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
        ],
        [
            Some(
                ZSTD_btGetAllMatches_extDict_3
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_4
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_5
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_6
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
        ],
        [
            Some(
                ZSTD_btGetAllMatches_dictMatchState_3
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_4
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_5
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_6
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_matchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
        ],
    ];
    let mls = if 3 as libc::c_int as libc::c_uint
        > (if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as libc::c_int as libc::c_uint
        })
    {
        3 as libc::c_int as libc::c_uint
    } else if (*ms).cParams.minMatch < 6 as libc::c_int as libc::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as libc::c_int as libc::c_uint
    };
    debug_assert!((dictMode as u32) < 3 as libc::c_int as libc::c_uint);
    debug_assert!(mls.wrapping_sub(3)
        < 4 as libc::c_int as libc::c_uint);
    return getAllMatchesFns[dictMode as libc::c_int
        as usize][mls.wrapping_sub(3) as usize];
}
unsafe extern "C" fn ZSTD_optLdm_skipRawSeqStoreBytes(
    mut rawSeqStore: *mut rawSeqStore_t,
    mut nbBytes: libc::size_t,
) {
    let mut currPos = ((*rawSeqStore).posInSequence).wrapping_add(nbBytes) as u32;
    while currPos != 0 && (*rawSeqStore).pos < (*rawSeqStore).size {
        let mut currSeq = *((*rawSeqStore).seq).offset((*rawSeqStore).pos as isize);
        if currPos >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            currPos = (currPos as libc::c_uint)
                .wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength))
                ;
            (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
        } else {
            (*rawSeqStore).posInSequence = currPos as libc::size_t;
            break;
        }
    }
    if currPos == 0 as libc::c_int as libc::c_uint
        || (*rawSeqStore).pos == (*rawSeqStore).size
    {
        (*rawSeqStore).posInSequence = 0 as libc::c_int as libc::size_t;
    }
}
unsafe extern "C" fn ZSTD_opt_getNextMatchAndUpdateSeqStore(
    mut optLdm: *mut ZSTD_optLdm_t,
    mut currPosInBlock: u32,
    mut blockBytesRemaining: u32,
) {
    let mut currSeq = rawSeq {
        offset: 0,
        litLength: 0,
        matchLength: 0,
    };
    let mut currBlockEndPos: u32 = 0;
    let mut literalsBytesRemaining: u32 = 0;
    let mut matchBytesRemaining: u32 = 0;
    if (*optLdm).seqStore.size == 0 as libc::c_int as libc::c_ulong
        || (*optLdm).seqStore.pos >= (*optLdm).seqStore.size
    {
        (*optLdm).startPosInBlock = UINT_MAX;
        (*optLdm).endPosInBlock = UINT_MAX;
        return;
    }
    currSeq = *((*optLdm).seqStore.seq).offset((*optLdm).seqStore.pos as isize);
    debug_assert!((*optLdm).seqStore.posInSequence
        <= (currSeq.litLength).wrapping_add(currSeq.matchLength) as libc::c_ulong);
    currBlockEndPos = currPosInBlock.wrapping_add(blockBytesRemaining);
    literalsBytesRemaining = if (*optLdm).seqStore.posInSequence
        < currSeq.litLength as libc::c_ulong
    {
        (currSeq.litLength).wrapping_sub((*optLdm).seqStore.posInSequence as u32)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    matchBytesRemaining = if literalsBytesRemaining == 0 as libc::c_int as libc::c_uint {
        (currSeq.matchLength)
            .wrapping_sub(
                ((*optLdm).seqStore.posInSequence as u32).wrapping_sub(currSeq.litLength),
            )
    } else {
        currSeq.matchLength
    };
    if literalsBytesRemaining >= blockBytesRemaining {
        (*optLdm).startPosInBlock = UINT_MAX;
        (*optLdm).endPosInBlock = UINT_MAX;
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            blockBytesRemaining as libc::size_t,
        );
        return;
    }
    (*optLdm).startPosInBlock = currPosInBlock.wrapping_add(literalsBytesRemaining);
    (*optLdm)
        .endPosInBlock = ((*optLdm).startPosInBlock).wrapping_add(matchBytesRemaining);
    (*optLdm).offset = currSeq.offset;
    if (*optLdm).endPosInBlock > currBlockEndPos {
        (*optLdm).endPosInBlock = currBlockEndPos;
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            currBlockEndPos.wrapping_sub(currPosInBlock) as libc::size_t,
        );
    } else {
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            literalsBytesRemaining.wrapping_add(matchBytesRemaining) as libc::size_t,
        );
    };
}
unsafe extern "C" fn ZSTD_optLdm_maybeAddMatch(
    mut matches: *mut ZSTD_match_t,
    mut nbMatches: *mut u32,
    mut optLdm: *const ZSTD_optLdm_t,
    mut currPosInBlock: u32,
) {
    let posDiff = currPosInBlock.wrapping_sub((*optLdm).startPosInBlock);
    let candidateMatchLength = ((*optLdm).endPosInBlock)
        .wrapping_sub((*optLdm).startPosInBlock)
        .wrapping_sub(posDiff);
    if currPosInBlock < (*optLdm).startPosInBlock
        || currPosInBlock >= (*optLdm).endPosInBlock
        || candidateMatchLength < MINMATCH as libc::c_uint
    {
        return;
    }
    if *nbMatches == 0 as libc::c_int as libc::c_uint
        || candidateMatchLength
            > (*matches
                .offset(
                    (*nbMatches).wrapping_sub(1) as isize,
                ))
                .len && *nbMatches < ZSTD_OPT_NUM as libc::c_uint
    {
        debug_assert!((*optLdm).offset > 0 as libc::c_int as libc::c_uint);
        let candidateOffBase = ((*optLdm).offset)
            .wrapping_add(ZSTD_REP_NUM as libc::c_uint);
        (*matches.offset(*nbMatches as isize)).len = candidateMatchLength;
        (*matches.offset(*nbMatches as isize)).off = candidateOffBase;
        *nbMatches = (*nbMatches).wrapping_add(1);
    }
}
unsafe extern "C" fn ZSTD_optLdm_processMatchCandidate(
    mut optLdm: *mut ZSTD_optLdm_t,
    mut matches: *mut ZSTD_match_t,
    mut nbMatches: *mut u32,
    mut currPosInBlock: u32,
    mut remainingBytes: u32,
) {
    if (*optLdm).seqStore.size == 0 as libc::c_int as libc::c_ulong
        || (*optLdm).seqStore.pos >= (*optLdm).seqStore.size
    {
        return;
    }
    if currPosInBlock >= (*optLdm).endPosInBlock {
        if currPosInBlock > (*optLdm).endPosInBlock {
            let posOvershoot = currPosInBlock.wrapping_sub((*optLdm).endPosInBlock);
            ZSTD_optLdm_skipRawSeqStoreBytes(
                &mut (*optLdm).seqStore,
                posOvershoot as libc::size_t,
            );
        }
        ZSTD_opt_getNextMatchAndUpdateSeqStore(optLdm, currPosInBlock, remainingBytes);
    }
    ZSTD_optLdm_maybeAddMatch(matches, nbMatches, optLdm, currPosInBlock);
}
unsafe extern "C" fn ZSTD_totalLen(mut sol: ZSTD_optimal_t) -> u32 {
    return (sol.litlen).wrapping_add(sol.mlen);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_opt_generic(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    optLevel: libc::c_int,
    dictMode: ZSTD_dictMode_e,
) -> libc::size_t {
    let mut current_block: u64;
    let optStatePtr: *mut optState_t = &mut (*ms).opt;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(8 as libc::c_int as isize));
    let base = (*ms).window.base;
    let prefixStart = base.offset((*ms).window.dictLimit as isize);
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let mut getAllMatches = ZSTD_selectBtGetAllMatches(ms, dictMode);
    let sufficient_len = if (*cParams).targetLength
        < (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int) as libc::c_uint
    {
        (*cParams).targetLength
    } else {
        (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int) as libc::c_uint
    };
    let minMatch = (if (*cParams).minMatch == 3 as libc::c_int as libc::c_uint {
        3 as libc::c_int
    } else {
        4 as libc::c_int
    }) as u32;
    let mut nextToUpdate3 = (*ms).nextToUpdate;
    let opt = (*optStatePtr).priceTable;
    let matches = (*optStatePtr).matchTable;
    let mut lastSequence = ZSTD_optimal_t {
        price: 0,
        off: 0,
        mlen: 0,
        litlen: 0,
        rep: [0; 3],
    };
    let mut optLdm = ZSTD_optLdm_t {
        seqStore: rawSeqStore_t {
            seq: 0 as *mut rawSeq,
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        },
        startPosInBlock: 0,
        endPosInBlock: 0,
        offset: 0,
    };
    libc::memset(
        &mut lastSequence as *mut ZSTD_optimal_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_optimal_t>() as libc::c_ulong as libc::size_t,
    );
    optLdm
        .seqStore = if !((*ms).ldmSeqStore).is_null() {
        *(*ms).ldmSeqStore
    } else {
        kNullRawSeqStore
    };
    optLdm.offset = 0 as libc::c_int as u32;
    optLdm.startPosInBlock = optLdm.offset;
    optLdm.endPosInBlock = optLdm.startPosInBlock;
    ZSTD_opt_getNextMatchAndUpdateSeqStore(
        &mut optLdm,
        ip.offset_from(istart) as libc::c_long as u32,
        iend.offset_from(ip) as libc::c_long as u32,
    );
    debug_assert!(optLevel <= 2 as libc::c_int);
    ZSTD_rescaleFreqs(optStatePtr, src as *const u8, srcSize, optLevel);
    ip = ip.offset((ip == prefixStart) as libc::c_int as isize);
    while ip < ilimit {
        let mut cur: u32 = 0;
        let mut last_pos = 0 as libc::c_int as u32;
        let litlen = ip.offset_from(anchor) as libc::c_long as u32;
        let ll0 = (litlen == 0) as libc::c_int as u32;
        let mut nbMatches = getAllMatches
            .expect(
                "non-null function pointer",
            )(
            matches,
            ms,
            &mut nextToUpdate3,
            ip,
            iend,
            rep as *const u32,
            ll0,
            minMatch,
        );
        ZSTD_optLdm_processMatchCandidate(
            &mut optLdm,
            matches,
            &mut nbMatches,
            ip.offset_from(istart) as libc::c_long as u32,
            iend.offset_from(ip) as libc::c_long as u32,
        );
        if nbMatches == 0 {
            ip = ip.offset(1);
        } else {
            let mut i: u32 = 0;
            i = 0 as libc::c_int as u32;
            while i < ZSTD_REP_NUM as libc::c_uint {
                (*opt.offset(0 as libc::c_int as isize))
                    .rep[i as usize] = *rep.offset(i as isize);
                i = i.wrapping_add(1);
            }
            (*opt.offset(0 as libc::c_int as isize)).mlen = 0 as libc::c_int as u32;
            (*opt.offset(0 as libc::c_int as isize)).litlen = litlen;
            (*opt.offset(0 as libc::c_int as isize))
                .price = ZSTD_litLengthPrice(litlen, optStatePtr, optLevel)
                as libc::c_int;
            let maxML = (*matches
                .offset(
                    nbMatches.wrapping_sub(1) as isize,
                ))
                .len;
            let maxOffBase = (*matches
                .offset(
                    nbMatches.wrapping_sub(1) as isize,
                ))
                .off;
            if maxML > sufficient_len {
                lastSequence.litlen = litlen;
                lastSequence.mlen = maxML;
                lastSequence.off = maxOffBase;
                cur = 0 as libc::c_int as u32;
                last_pos = ZSTD_totalLen(lastSequence);
            } else {
                debug_assert!((*opt.offset(0 as libc::c_int as isize)).price >= 0 as libc::c_int);
                let literalsPrice = ((*opt.offset(0 as libc::c_int as isize)).price
                    as u32)
                    .wrapping_add(
                        ZSTD_litLengthPrice(
                            0 as libc::c_int as u32,
                            optStatePtr,
                            optLevel,
                        ),
                    );
                let mut pos: u32 = 0;
                let mut matchNb: u32 = 0;
                pos = 1 as libc::c_int as u32;
                while pos < minMatch {
                    (*opt.offset(pos as isize)).price = ZSTD_MAX_PRICE;
                    pos = pos.wrapping_add(1);
                }
                matchNb = 0 as libc::c_int as u32;
                while matchNb < nbMatches {
                    let offBase = (*matches.offset(matchNb as isize)).off;
                    let end = (*matches.offset(matchNb as isize)).len;
                    while pos <= end {
                        let matchPrice = ZSTD_getMatchPrice(
                            offBase,
                            pos,
                            optStatePtr,
                            optLevel,
                        );
                        let sequencePrice = literalsPrice.wrapping_add(matchPrice);
                        (*opt.offset(pos as isize)).mlen = pos;
                        (*opt.offset(pos as isize)).off = offBase;
                        (*opt.offset(pos as isize)).litlen = litlen;
                        (*opt.offset(pos as isize)).price = sequencePrice as libc::c_int;
                        pos = pos.wrapping_add(1);
                    }
                    matchNb = matchNb.wrapping_add(1);
                }
                last_pos = pos.wrapping_sub(1);
                cur = 1 as libc::c_int as u32;
                loop {
                    if !(cur <= last_pos) {
                        current_block = 10109057886293123569;
                        break;
                    }
                    let inr = ip.offset(cur as isize);
                    debug_assert!(cur < ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint);
                    let litlen_0 = if (*opt
                        .offset(
                            cur.wrapping_sub(1) as isize,
                        ))
                        .mlen == 0 as libc::c_int as libc::c_uint
                    {
                        ((*opt
                            .offset(
                                cur.wrapping_sub(1) as isize,
                            ))
                            .litlen)
                            .wrapping_add(1)
                    } else {
                        1 as libc::c_int as libc::c_uint
                    };
                    let price = (*opt
                        .offset(
                            cur.wrapping_sub(1) as isize,
                        ))
                        .price
                        + ZSTD_rawLiteralsCost(
                            ip.offset(cur as isize).offset(-(1 as libc::c_int as isize)),
                            1 as libc::c_int as u32,
                            optStatePtr,
                            optLevel,
                        ) as libc::c_int
                        + ZSTD_litLengthPrice(litlen_0, optStatePtr, optLevel)
                            as libc::c_int
                        - ZSTD_litLengthPrice(
                            litlen_0.wrapping_sub(1),
                            optStatePtr,
                            optLevel,
                        ) as libc::c_int;
                    debug_assert!(price < 1000000000 as libc::c_int);
                    if price <= (*opt.offset(cur as isize)).price {
                        (*opt.offset(cur as isize)).mlen = 0 as libc::c_int as u32;
                        (*opt.offset(cur as isize)).off = 0 as libc::c_int as u32;
                        (*opt.offset(cur as isize)).litlen = litlen_0;
                        (*opt.offset(cur as isize)).price = price;
                    }
                    debug_assert!(cur >= (*opt.offset(cur as isize)).mlen);
                    if (*opt.offset(cur as isize)).mlen
                        != 0 as libc::c_int as libc::c_uint
                    {
                        let prev = cur.wrapping_sub((*opt.offset(cur as isize)).mlen);
                        let newReps = ZSTD_newRep(
                            ((*opt.offset(prev as isize)).rep).as_mut_ptr()
                                as *const u32,
                            (*opt.offset(cur as isize)).off,
                            ((*opt.offset(cur as isize)).litlen
                                == 0 as libc::c_int as libc::c_uint) as libc::c_int as u32,
                        );
                        libc::memcpy(
                            ((*opt.offset(cur as isize)).rep).as_mut_ptr()
                                as *mut libc::c_void,
                            &newReps as *const repcodes_t as *const libc::c_void,
                            ::core::mem::size_of::<repcodes_t>() as libc::c_ulong
                                as libc::size_t,
                        );
                    } else {
                        libc::memcpy(
                            ((*opt.offset(cur as isize)).rep).as_mut_ptr()
                                as *mut libc::c_void,
                            ((*opt
                                .offset(
                                    cur.wrapping_sub(1) as isize,
                                ))
                                .rep)
                                .as_mut_ptr() as *const libc::c_void,
                            ::core::mem::size_of::<repcodes_t>() as libc::c_ulong
                                as libc::size_t,
                        );
                    }
                    if !(inr > ilimit) {
                        if cur == last_pos {
                            current_block = 10109057886293123569;
                            break;
                        }
                        if !(optLevel == 0 as libc::c_int
                            && (*opt
                                .offset(
                                    cur.wrapping_add(1) as isize,
                                ))
                                .price
                                <= (*opt.offset(cur as isize)).price
                                    + BITCOST_MULTIPLIER / 2 as libc::c_int)
                        {
                            debug_assert!((*opt.offset(cur as isize)).price >= 0 as libc::c_int);
                            let ll0_0 = ((*opt.offset(cur as isize)).mlen
                                != 0 as libc::c_int as libc::c_uint) as libc::c_int as u32;
                            let litlen_1 = if (*opt.offset(cur as isize)).mlen
                                == 0 as libc::c_int as libc::c_uint
                            {
                                (*opt.offset(cur as isize)).litlen
                            } else {
                                0 as libc::c_int as libc::c_uint
                            };
                            let previousPrice = (*opt.offset(cur as isize)).price as u32;
                            let basePrice = previousPrice
                                .wrapping_add(
                                    ZSTD_litLengthPrice(
                                        0 as libc::c_int as u32,
                                        optStatePtr,
                                        optLevel,
                                    ),
                                );
                            let mut nbMatches_0 = getAllMatches
                                .expect(
                                    "non-null function pointer",
                                )(
                                matches,
                                ms,
                                &mut nextToUpdate3,
                                inr,
                                iend,
                                ((*opt.offset(cur as isize)).rep).as_mut_ptr()
                                    as *const u32,
                                ll0_0,
                                minMatch,
                            );
                            let mut matchNb_0: u32 = 0;
                            ZSTD_optLdm_processMatchCandidate(
                                &mut optLdm,
                                matches,
                                &mut nbMatches_0,
                                inr.offset_from(istart) as libc::c_long as u32,
                                iend.offset_from(inr) as libc::c_long as u32,
                            );
                            if !(nbMatches_0 == 0) {
                                let maxML_0 = (*matches
                                    .offset(
                                        nbMatches_0.wrapping_sub(1)
                                            as isize,
                                    ))
                                    .len;
                                if maxML_0 > sufficient_len
                                    || cur.wrapping_add(maxML_0) >= ZSTD_OPT_NUM as libc::c_uint
                                {
                                    lastSequence.mlen = maxML_0;
                                    lastSequence
                                        .off = (*matches
                                        .offset(
                                            nbMatches_0.wrapping_sub(1)
                                                as isize,
                                        ))
                                        .off;
                                    lastSequence.litlen = litlen_1;
                                    cur = (cur as libc::c_uint)
                                        .wrapping_sub(
                                            if (*opt.offset(cur as isize)).mlen
                                                == 0 as libc::c_int as libc::c_uint
                                            {
                                                (*opt.offset(cur as isize)).litlen
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            },
                                        ) ;
                                    last_pos = cur.wrapping_add(ZSTD_totalLen(lastSequence));
                                    if cur > ZSTD_OPT_NUM as libc::c_uint {
                                        cur = 0 as libc::c_int as u32;
                                    }
                                    current_block = 4910109294474246627;
                                    break;
                                } else {
                                    matchNb_0 = 0 as libc::c_int as u32;
                                    while matchNb_0 < nbMatches_0 {
                                        let offset = (*matches.offset(matchNb_0 as isize)).off;
                                        let lastML = (*matches.offset(matchNb_0 as isize)).len;
                                        let startML = if matchNb_0
                                            > 0 as libc::c_int as libc::c_uint
                                        {
                                            ((*matches
                                                .offset(
                                                    matchNb_0.wrapping_sub(1)
                                                        as isize,
                                                ))
                                                .len)
                                                .wrapping_add(1)
                                        } else {
                                            minMatch
                                        };
                                        let mut mlen: u32 = 0;
                                        mlen = lastML;
                                        while mlen >= startML {
                                            let pos_0 = cur.wrapping_add(mlen);
                                            let price_0 = basePrice as libc::c_int
                                                + ZSTD_getMatchPrice(offset, mlen, optStatePtr, optLevel)
                                                    as libc::c_int;
                                            if pos_0 > last_pos
                                                || price_0 < (*opt.offset(pos_0 as isize)).price
                                            {
                                                while last_pos < pos_0 {
                                                    (*opt
                                                        .offset(
                                                            last_pos.wrapping_add(1)
                                                                as isize,
                                                        ))
                                                        .price = ZSTD_MAX_PRICE;
                                                    last_pos = last_pos.wrapping_add(1);
                                                }
                                                (*opt.offset(pos_0 as isize)).mlen = mlen;
                                                (*opt.offset(pos_0 as isize)).off = offset;
                                                (*opt.offset(pos_0 as isize)).litlen = litlen_1;
                                                (*opt.offset(pos_0 as isize)).price = price_0;
                                            } else if optLevel == 0 as libc::c_int {
                                                break;
                                            }
                                            mlen = mlen.wrapping_sub(1);
                                        }
                                        matchNb_0 = matchNb_0.wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                    cur = cur.wrapping_add(1);
                }
                match current_block {
                    4910109294474246627 => {}
                    _ => {
                        lastSequence = *opt.offset(last_pos as isize);
                        cur = if last_pos > ZSTD_totalLen(lastSequence) {
                            last_pos.wrapping_sub(ZSTD_totalLen(lastSequence))
                        } else {
                            0 as libc::c_int as libc::c_uint
                        };
                        debug_assert!(cur
                            < ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint);
                    }
                }
            }
            debug_assert!((*opt.offset(0 as libc::c_int as isize)).mlen
                == 0 as libc::c_int as libc::c_uint);
            if lastSequence.mlen != 0 as libc::c_int as libc::c_uint {
                let reps = ZSTD_newRep(
                    ((*opt.offset(cur as isize)).rep).as_mut_ptr() as *const u32,
                    lastSequence.off,
                    (lastSequence.litlen == 0 as libc::c_int as libc::c_uint)
                        as libc::c_int as u32,
                );
                libc::memcpy(
                    rep as *mut libc::c_void,
                    &reps as *const repcodes_t as *const libc::c_void,
                    ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
                );
            } else {
                libc::memcpy(
                    rep as *mut libc::c_void,
                    ((*opt.offset(cur as isize)).rep).as_mut_ptr()
                        as *const libc::c_void,
                    ::core::mem::size_of::<repcodes_t>() as libc::c_ulong as libc::size_t,
                );
            }
            let storeEnd = cur.wrapping_add(1);
            let mut storeStart = storeEnd;
            let mut seqPos = cur;
            debug_assert!(storeEnd < ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint);
            *opt.offset(storeEnd as isize) = lastSequence;
            while seqPos > 0 as libc::c_int as libc::c_uint {
                let backDist = ZSTD_totalLen(*opt.offset(seqPos as isize));
                storeStart = storeStart.wrapping_sub(1);
                *opt.offset(storeStart as isize) = *opt.offset(seqPos as isize);
                seqPos = if seqPos > backDist {
                    seqPos.wrapping_sub(backDist)
                } else {
                    0 as libc::c_int as libc::c_uint
                };
            }
            let mut storePos: u32 = 0;
            storePos = storeStart;
            while storePos <= storeEnd {
                let llen = (*opt.offset(storePos as isize)).litlen;
                let mlen_0 = (*opt.offset(storePos as isize)).mlen;
                let offBase_0 = (*opt.offset(storePos as isize)).off;
                let advance = llen.wrapping_add(mlen_0);
                if mlen_0 == 0 as libc::c_int as libc::c_uint {
                    debug_assert!(storePos == storeEnd);
                    ip = anchor.offset(llen as isize);
                } else {
                    debug_assert!(anchor.offset(llen as isize) <= iend);
                    ZSTD_updateStats(optStatePtr, llen, anchor, offBase_0, mlen_0);
                    ZSTD_storeSeq(
                        seqStore,
                        llen as libc::size_t,
                        anchor,
                        iend,
                        offBase_0,
                        mlen_0 as libc::size_t,
                    );
                    anchor = anchor.offset(advance as isize);
                    ip = anchor;
                }
                storePos = storePos.wrapping_add(1);
            }
            ZSTD_setBasePrices(optStatePtr, optLevel);
        }
    }
    return iend.offset_from(anchor) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_compressBlock_opt0(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    dictMode: ZSTD_dictMode_e,
) -> libc::size_t {
    return ZSTD_compressBlock_opt_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        0 as libc::c_int,
        dictMode,
    );
}
unsafe extern "C" fn ZSTD_compressBlock_opt2(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    dictMode: ZSTD_dictMode_e,
) -> libc::size_t {
    return ZSTD_compressBlock_opt_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        2 as libc::c_int,
        dictMode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_noDict);
}
unsafe extern "C" fn ZSTD_initStats_ultra(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) {
    let mut tmpRep: [u32; 3] = [0; 3];
    libc::memcpy(
        tmpRep.as_mut_ptr() as *mut libc::c_void,
        rep as *const libc::c_void,
        ::core::mem::size_of::<[u32; 3]>() as libc::c_ulong as libc::size_t,
    );
    debug_assert!((*ms).opt.litLengthSum == 0 as libc::c_int as libc::c_uint);
    debug_assert!((*seqStore).sequences == (*seqStore).sequencesStart);
    debug_assert!((*ms).window.dictLimit == (*ms).window.lowLimit);
    debug_assert!(((*ms).window.dictLimit).wrapping_sub((*ms).nextToUpdate)
        <= 1 as libc::c_int as libc::c_uint);
    ZSTD_compressBlock_opt2(
        ms,
        seqStore,
        tmpRep.as_mut_ptr(),
        src,
        srcSize,
        ZSTD_noDict,
    );
    ZSTD_resetSeqStore(seqStore);
    (*ms).window.base = ((*ms).window.base).offset(-(srcSize as isize));
    (*ms)
        .window
        .dictLimit = ((*ms).window.dictLimit as libc::c_uint)
        .wrapping_add(srcSize as u32) ;
    (*ms).window.lowLimit = (*ms).window.dictLimit;
    (*ms).nextToUpdate = (*ms).window.dictLimit;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra2(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let curr = (src as *const u8).offset_from((*ms).window.base) as libc::c_long
        as u32;
    debug_assert!(srcSize <= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong);
    if (*ms).opt.litLengthSum == 0 as libc::c_int as libc::c_uint
        && (*seqStore).sequences == (*seqStore).sequencesStart
        && (*ms).window.dictLimit == (*ms).window.lowLimit
        && curr == (*ms).window.dictLimit
        && srcSize > ZSTD_PREDEF_THRESHOLD as libc::c_ulong
    {
        ZSTD_initStats_ultra(ms, seqStore, rep, src, srcSize);
    }
    return ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_dictMatchState(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_extDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_extDict(
    mut ms: *mut ZSTD_matchState_t,
    mut seqStore: *mut seqStore_t,
    mut rep: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_extDict);
}
