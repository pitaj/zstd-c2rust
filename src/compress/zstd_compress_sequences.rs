use ::libc;
extern "C" {
    fn FSE_writeNCount(
        buffer: *mut libc::c_void,
        bufferSize: size_t,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
    ) -> size_t;
    fn FSE_normalizeCount(
        normalizedCounter: *mut libc::c_short,
        tableLog: libc::c_uint,
        count: *const libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
        useLowProbCount: libc::c_uint,
    ) -> size_t;
    fn FSE_optimalTableLog(
        maxTableLog: libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
    ) -> libc::c_uint;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn FSE_buildCTable_rle(ct: *mut FSE_CTable, symbolValue: libc::c_uchar) -> size_t;
    fn FSE_buildCTable_wksp(
        ct: *mut FSE_CTable,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
    ) -> size_t;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type FSE_CTable = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U8 = uint8_t;
pub type U16 = uint16_t;
pub type S16 = int16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type unalign64 = U64;
pub type C2RustUnnamed = libc::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed = 10;
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
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
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type FSE_repeat = libc::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
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
    pub deltaNbBits: U32,
}
pub type symbolEncodingType_e = libc::c_uint;
pub const set_repeat: symbolEncodingType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub const set_rle: symbolEncodingType_e = 1;
pub const set_basic: symbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqDef_s {
    pub offBase: U32,
    pub litLength: U16,
    pub mlBase: U16,
}
pub type seqDef = seqDef_s;
pub type ZSTD_defaultPolicy_e = libc::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_defaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_defaultPolicy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_BuildCTableWksp {
    pub norm: [S16; 53],
    pub wksp: [U32; 285],
}
pub const LLFSELog: libc::c_int = 9 as libc::c_int;
pub const MLFSELog: libc::c_int = 9 as libc::c_int;
pub const OffFSELog: libc::c_int = 8 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
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
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: U32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: U32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void, mut val64: U64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void, mut val: size_t) {
    if MEM_32bits() != 0 {
        MEM_writeLE32(memPtr, val as U32);
    } else {
        MEM_writeLE64(memPtr, val);
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
#[inline]
unsafe extern "C" fn BIT_initCStream(
    mut bitC: *mut BIT_CStream_t,
    mut startPtr: *mut libc::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    (*bitC).bitContainer = 0 as libc::c_int as size_t;
    (*bitC).bitPos = 0 as libc::c_int as libc::c_uint;
    (*bitC).startPtr = startPtr as *mut libc::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC)
        .endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<size_t>() as libc::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<size_t>() as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn BIT_addBits(
    mut bitC: *mut BIT_CStream_t,
    mut value: size_t,
    mut nbBits: libc::c_uint,
) {
    if (nbBits as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbBits < BIT_MASK_SIZE\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void BIT_addBits(BIT_CStream_t *, size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    if (nbBits.wrapping_add((*bitC).bitPos) as libc::c_ulong)
        < (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbBits + bitC->bitPos < sizeof(bitC->bitContainer) * 8\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void BIT_addBits(BIT_CStream_t *, size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    (*bitC).bitContainer |= BIT_getLowerBits(value, nbBits) << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}
#[inline(always)]
unsafe extern "C" fn BIT_getLowerBits(mut bitContainer: size_t, nbBits: U32) -> size_t {
    if (nbBits as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbBits < BIT_MASK_SIZE\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"size_t BIT_getLowerBits(size_t, const U32)\0"))
                .as_ptr(),
        );
    }
    return bitContainer & BIT_mask[nbBits as usize] as libc::c_ulong;
}
static mut BIT_mask: [libc::c_uint; 32] = [
    0 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    3 as libc::c_int as libc::c_uint,
    7 as libc::c_int as libc::c_uint,
    0xf as libc::c_int as libc::c_uint,
    0x1f as libc::c_int as libc::c_uint,
    0x3f as libc::c_int as libc::c_uint,
    0x7f as libc::c_int as libc::c_uint,
    0xff as libc::c_int as libc::c_uint,
    0x1ff as libc::c_int as libc::c_uint,
    0x3ff as libc::c_int as libc::c_uint,
    0x7ff as libc::c_int as libc::c_uint,
    0xfff as libc::c_int as libc::c_uint,
    0x1fff as libc::c_int as libc::c_uint,
    0x3fff as libc::c_int as libc::c_uint,
    0x7fff as libc::c_int as libc::c_uint,
    0xffff as libc::c_int as libc::c_uint,
    0x1ffff as libc::c_int as libc::c_uint,
    0x3ffff as libc::c_int as libc::c_uint,
    0x7ffff as libc::c_int as libc::c_uint,
    0xfffff as libc::c_int as libc::c_uint,
    0x1fffff as libc::c_int as libc::c_uint,
    0x3fffff as libc::c_int as libc::c_uint,
    0x7fffff as libc::c_int as libc::c_uint,
    0xffffff as libc::c_int as libc::c_uint,
    0x1ffffff as libc::c_int as libc::c_uint,
    0x3ffffff as libc::c_int as libc::c_uint,
    0x7ffffff as libc::c_int as libc::c_uint,
    0xfffffff as libc::c_int as libc::c_uint,
    0x1fffffff as libc::c_int as libc::c_uint,
    0x3fffffff as libc::c_int as libc::c_uint,
    0x7fffffff as libc::c_int as libc::c_uint,
];
#[inline]
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3 as libc::c_int) as size_t;
    if ((*bitC).bitPos as libc::c_ulong)
        < (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"bitC->bitPos < sizeof(bitC->bitContainer) * 8\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void BIT_flushBits(BIT_CStream_t *)\0"))
                .as_ptr(),
        );
    }
    if (*bitC).ptr <= (*bitC).endPtr {} else {
        __assert_fail(
            b"bitC->ptr <= bitC->endPtr\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            223 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void BIT_flushBits(BIT_CStream_t *)\0"))
                .as_ptr(),
        );
    }
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    if (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
    (*bitC).bitPos &= 7 as libc::c_int as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8 as libc::c_int as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t) -> size_t {
    BIT_addBitsFast(bitC, 1 as libc::c_int as size_t, 1 as libc::c_int as libc::c_uint);
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0 as libc::c_int as size_t;
    }
    return (((*bitC).ptr).offset_from((*bitC).startPtr) as libc::c_long
        + ((*bitC).bitPos > 0 as libc::c_int as libc::c_uint) as libc::c_int
            as libc::c_long) as size_t;
}
#[inline]
unsafe extern "C" fn BIT_addBitsFast(
    mut bitC: *mut BIT_CStream_t,
    mut value: size_t,
    mut nbBits: libc::c_uint,
) {
    if value >> nbBits == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"(value>>nbBits) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void BIT_addBitsFast(BIT_CStream_t *, size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    if (nbBits.wrapping_add((*bitC).bitPos) as libc::c_ulong)
        < (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbBits + bitC->bitPos < sizeof(bitC->bitContainer) * 8\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void BIT_addBitsFast(BIT_CStream_t *, size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn FSE_initCState(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
) {
    let mut ptr = ct as *const libc::c_void;
    let mut u16ptr = ptr as *const U16;
    let tableLog = MEM_read16(ptr) as U32;
    (*statePtr).value = (1 as libc::c_int as ptrdiff_t) << tableLog;
    (*statePtr)
        .stateTable = u16ptr.offset(2 as libc::c_int as isize) as *const libc::c_void;
    (*statePtr)
        .symbolTT = ct
        .offset(1 as libc::c_int as isize)
        .offset(
            (if tableLog != 0 {
                (1 as libc::c_int)
                    << tableLog.wrapping_sub(1 as libc::c_int as libc::c_uint)
            } else {
                1 as libc::c_int
            }) as isize,
        ) as *const libc::c_void;
    (*statePtr).stateLog = tableLog;
}
#[inline]
unsafe extern "C" fn FSE_encodeSymbol(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *mut FSE_CState_t,
    mut symbol: libc::c_uint,
) {
    let symbolTT = *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform)
        .offset(symbol as isize);
    let stateTable = (*statePtr).stateTable as *const U16;
    let nbBitsOut = ((*statePtr).value + symbolTT.deltaNbBits as libc::c_long
        >> 16 as libc::c_int) as U32;
    BIT_addBits(bitC, (*statePtr).value as size_t, nbBitsOut);
    (*statePtr)
        .value = *stateTable
        .offset(
            (((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as libc::c_long)
                as isize,
        ) as ptrdiff_t;
}
#[inline]
unsafe extern "C" fn FSE_flushCState(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *const FSE_CState_t,
) {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}
#[inline]
unsafe extern "C" fn FSE_initCState2(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
    mut symbol: U32,
) {
    FSE_initCState(statePtr, ct);
    let symbolTT = *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform)
        .offset(symbol as isize);
    let mut stateTable = (*statePtr).stateTable as *const U16;
    let mut nbBitsOut = (symbolTT.deltaNbBits)
        .wrapping_add(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
        >> 16 as libc::c_int;
    (*statePtr)
        .value = (nbBitsOut << 16 as libc::c_int).wrapping_sub(symbolTT.deltaNbBits)
        as ptrdiff_t;
    (*statePtr)
        .value = *stateTable
        .offset(
            (((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as libc::c_long)
                as isize,
        ) as ptrdiff_t;
}
#[inline]
unsafe extern "C" fn FSE_bitCost(
    mut symbolTTPtr: *const libc::c_void,
    mut tableLog: U32,
    mut symbolValue: U32,
    mut accuracyLog: U32,
) -> U32 {
    let mut symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits = (*symbolTT.offset(symbolValue as isize)).deltaNbBits
        >> 16 as libc::c_int;
    let threshold = minNbBits.wrapping_add(1 as libc::c_int as libc::c_uint)
        << 16 as libc::c_int;
    if tableLog < 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tableLog < 16\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/fse.h\0"
                as *const u8 as *const libc::c_char,
            498 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"U32 FSE_bitCost(const void *, U32, U32, U32)\0"))
                .as_ptr(),
        );
    }
    if accuracyLog < (31 as libc::c_int as libc::c_uint).wrapping_sub(tableLog) {} else {
        __assert_fail(
            b"accuracyLog < 31-tableLog\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/fse.h\0"
                as *const u8 as *const libc::c_char,
            499 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"U32 FSE_bitCost(const void *, U32, U32, U32)\0"))
                .as_ptr(),
        );
    }
    let tableSize = ((1 as libc::c_int) << tableLog) as U32;
    let deltaFromThreshold = threshold
        .wrapping_sub(
            ((*symbolTT.offset(symbolValue as isize)).deltaNbBits)
                .wrapping_add(tableSize),
        );
    let normalizedDeltaFromThreshold = deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier = ((1 as libc::c_int) << accuracyLog) as U32;
    if ((*symbolTT.offset(symbolValue as isize)).deltaNbBits).wrapping_add(tableSize)
        <= threshold
    {} else {
        __assert_fail(
            b"symbolTT[symbolValue].deltaNbBits + tableSize <= threshold\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/fse.h\0"
                as *const u8 as *const libc::c_char,
            504 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"U32 FSE_bitCost(const void *, U32, U32, U32)\0"))
                .as_ptr(),
        );
    }
    if normalizedDeltaFromThreshold <= bitMultiplier {} else {
        __assert_fail(
            b"normalizedDeltaFromThreshold <= bitMultiplier\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/fse.h\0"
                as *const u8 as *const libc::c_char,
            505 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"U32 FSE_bitCost(const void *, U32, U32, U32)\0"))
                .as_ptr(),
        );
    }
    return minNbBits
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_mul(bitMultiplier)
        .wrapping_sub(normalizedDeltaFromThreshold);
}
static mut LL_bits: [U8; 36] = [
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    2 as libc::c_int as U8,
    2 as libc::c_int as U8,
    3 as libc::c_int as U8,
    3 as libc::c_int as U8,
    4 as libc::c_int as U8,
    6 as libc::c_int as U8,
    7 as libc::c_int as U8,
    8 as libc::c_int as U8,
    9 as libc::c_int as U8,
    10 as libc::c_int as U8,
    11 as libc::c_int as U8,
    12 as libc::c_int as U8,
    13 as libc::c_int as U8,
    14 as libc::c_int as U8,
    15 as libc::c_int as U8,
    16 as libc::c_int as U8,
];
static mut ML_bits: [U8; 53] = [
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    0 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    1 as libc::c_int as U8,
    2 as libc::c_int as U8,
    2 as libc::c_int as U8,
    3 as libc::c_int as U8,
    3 as libc::c_int as U8,
    4 as libc::c_int as U8,
    4 as libc::c_int as U8,
    5 as libc::c_int as U8,
    7 as libc::c_int as U8,
    8 as libc::c_int as U8,
    9 as libc::c_int as U8,
    10 as libc::c_int as U8,
    11 as libc::c_int as U8,
    12 as libc::c_int as U8,
    13 as libc::c_int as U8,
    14 as libc::c_int as U8,
    15 as libc::c_int as U8,
    16 as libc::c_int as U8,
];
static mut kInverseProbabilityLog256: [libc::c_uint; 256] = [
    0 as libc::c_int as libc::c_uint,
    2048 as libc::c_int as libc::c_uint,
    1792 as libc::c_int as libc::c_uint,
    1642 as libc::c_int as libc::c_uint,
    1536 as libc::c_int as libc::c_uint,
    1453 as libc::c_int as libc::c_uint,
    1386 as libc::c_int as libc::c_uint,
    1329 as libc::c_int as libc::c_uint,
    1280 as libc::c_int as libc::c_uint,
    1236 as libc::c_int as libc::c_uint,
    1197 as libc::c_int as libc::c_uint,
    1162 as libc::c_int as libc::c_uint,
    1130 as libc::c_int as libc::c_uint,
    1100 as libc::c_int as libc::c_uint,
    1073 as libc::c_int as libc::c_uint,
    1047 as libc::c_int as libc::c_uint,
    1024 as libc::c_int as libc::c_uint,
    1001 as libc::c_int as libc::c_uint,
    980 as libc::c_int as libc::c_uint,
    960 as libc::c_int as libc::c_uint,
    941 as libc::c_int as libc::c_uint,
    923 as libc::c_int as libc::c_uint,
    906 as libc::c_int as libc::c_uint,
    889 as libc::c_int as libc::c_uint,
    874 as libc::c_int as libc::c_uint,
    859 as libc::c_int as libc::c_uint,
    844 as libc::c_int as libc::c_uint,
    830 as libc::c_int as libc::c_uint,
    817 as libc::c_int as libc::c_uint,
    804 as libc::c_int as libc::c_uint,
    791 as libc::c_int as libc::c_uint,
    779 as libc::c_int as libc::c_uint,
    768 as libc::c_int as libc::c_uint,
    756 as libc::c_int as libc::c_uint,
    745 as libc::c_int as libc::c_uint,
    734 as libc::c_int as libc::c_uint,
    724 as libc::c_int as libc::c_uint,
    714 as libc::c_int as libc::c_uint,
    704 as libc::c_int as libc::c_uint,
    694 as libc::c_int as libc::c_uint,
    685 as libc::c_int as libc::c_uint,
    676 as libc::c_int as libc::c_uint,
    667 as libc::c_int as libc::c_uint,
    658 as libc::c_int as libc::c_uint,
    650 as libc::c_int as libc::c_uint,
    642 as libc::c_int as libc::c_uint,
    633 as libc::c_int as libc::c_uint,
    626 as libc::c_int as libc::c_uint,
    618 as libc::c_int as libc::c_uint,
    610 as libc::c_int as libc::c_uint,
    603 as libc::c_int as libc::c_uint,
    595 as libc::c_int as libc::c_uint,
    588 as libc::c_int as libc::c_uint,
    581 as libc::c_int as libc::c_uint,
    574 as libc::c_int as libc::c_uint,
    567 as libc::c_int as libc::c_uint,
    561 as libc::c_int as libc::c_uint,
    554 as libc::c_int as libc::c_uint,
    548 as libc::c_int as libc::c_uint,
    542 as libc::c_int as libc::c_uint,
    535 as libc::c_int as libc::c_uint,
    529 as libc::c_int as libc::c_uint,
    523 as libc::c_int as libc::c_uint,
    517 as libc::c_int as libc::c_uint,
    512 as libc::c_int as libc::c_uint,
    506 as libc::c_int as libc::c_uint,
    500 as libc::c_int as libc::c_uint,
    495 as libc::c_int as libc::c_uint,
    489 as libc::c_int as libc::c_uint,
    484 as libc::c_int as libc::c_uint,
    478 as libc::c_int as libc::c_uint,
    473 as libc::c_int as libc::c_uint,
    468 as libc::c_int as libc::c_uint,
    463 as libc::c_int as libc::c_uint,
    458 as libc::c_int as libc::c_uint,
    453 as libc::c_int as libc::c_uint,
    448 as libc::c_int as libc::c_uint,
    443 as libc::c_int as libc::c_uint,
    438 as libc::c_int as libc::c_uint,
    434 as libc::c_int as libc::c_uint,
    429 as libc::c_int as libc::c_uint,
    424 as libc::c_int as libc::c_uint,
    420 as libc::c_int as libc::c_uint,
    415 as libc::c_int as libc::c_uint,
    411 as libc::c_int as libc::c_uint,
    407 as libc::c_int as libc::c_uint,
    402 as libc::c_int as libc::c_uint,
    398 as libc::c_int as libc::c_uint,
    394 as libc::c_int as libc::c_uint,
    390 as libc::c_int as libc::c_uint,
    386 as libc::c_int as libc::c_uint,
    382 as libc::c_int as libc::c_uint,
    377 as libc::c_int as libc::c_uint,
    373 as libc::c_int as libc::c_uint,
    370 as libc::c_int as libc::c_uint,
    366 as libc::c_int as libc::c_uint,
    362 as libc::c_int as libc::c_uint,
    358 as libc::c_int as libc::c_uint,
    354 as libc::c_int as libc::c_uint,
    350 as libc::c_int as libc::c_uint,
    347 as libc::c_int as libc::c_uint,
    343 as libc::c_int as libc::c_uint,
    339 as libc::c_int as libc::c_uint,
    336 as libc::c_int as libc::c_uint,
    332 as libc::c_int as libc::c_uint,
    329 as libc::c_int as libc::c_uint,
    325 as libc::c_int as libc::c_uint,
    322 as libc::c_int as libc::c_uint,
    318 as libc::c_int as libc::c_uint,
    315 as libc::c_int as libc::c_uint,
    311 as libc::c_int as libc::c_uint,
    308 as libc::c_int as libc::c_uint,
    305 as libc::c_int as libc::c_uint,
    302 as libc::c_int as libc::c_uint,
    298 as libc::c_int as libc::c_uint,
    295 as libc::c_int as libc::c_uint,
    292 as libc::c_int as libc::c_uint,
    289 as libc::c_int as libc::c_uint,
    286 as libc::c_int as libc::c_uint,
    282 as libc::c_int as libc::c_uint,
    279 as libc::c_int as libc::c_uint,
    276 as libc::c_int as libc::c_uint,
    273 as libc::c_int as libc::c_uint,
    270 as libc::c_int as libc::c_uint,
    267 as libc::c_int as libc::c_uint,
    264 as libc::c_int as libc::c_uint,
    261 as libc::c_int as libc::c_uint,
    258 as libc::c_int as libc::c_uint,
    256 as libc::c_int as libc::c_uint,
    253 as libc::c_int as libc::c_uint,
    250 as libc::c_int as libc::c_uint,
    247 as libc::c_int as libc::c_uint,
    244 as libc::c_int as libc::c_uint,
    241 as libc::c_int as libc::c_uint,
    239 as libc::c_int as libc::c_uint,
    236 as libc::c_int as libc::c_uint,
    233 as libc::c_int as libc::c_uint,
    230 as libc::c_int as libc::c_uint,
    228 as libc::c_int as libc::c_uint,
    225 as libc::c_int as libc::c_uint,
    222 as libc::c_int as libc::c_uint,
    220 as libc::c_int as libc::c_uint,
    217 as libc::c_int as libc::c_uint,
    215 as libc::c_int as libc::c_uint,
    212 as libc::c_int as libc::c_uint,
    209 as libc::c_int as libc::c_uint,
    207 as libc::c_int as libc::c_uint,
    204 as libc::c_int as libc::c_uint,
    202 as libc::c_int as libc::c_uint,
    199 as libc::c_int as libc::c_uint,
    197 as libc::c_int as libc::c_uint,
    194 as libc::c_int as libc::c_uint,
    192 as libc::c_int as libc::c_uint,
    190 as libc::c_int as libc::c_uint,
    187 as libc::c_int as libc::c_uint,
    185 as libc::c_int as libc::c_uint,
    182 as libc::c_int as libc::c_uint,
    180 as libc::c_int as libc::c_uint,
    178 as libc::c_int as libc::c_uint,
    175 as libc::c_int as libc::c_uint,
    173 as libc::c_int as libc::c_uint,
    171 as libc::c_int as libc::c_uint,
    168 as libc::c_int as libc::c_uint,
    166 as libc::c_int as libc::c_uint,
    164 as libc::c_int as libc::c_uint,
    162 as libc::c_int as libc::c_uint,
    159 as libc::c_int as libc::c_uint,
    157 as libc::c_int as libc::c_uint,
    155 as libc::c_int as libc::c_uint,
    153 as libc::c_int as libc::c_uint,
    151 as libc::c_int as libc::c_uint,
    149 as libc::c_int as libc::c_uint,
    146 as libc::c_int as libc::c_uint,
    144 as libc::c_int as libc::c_uint,
    142 as libc::c_int as libc::c_uint,
    140 as libc::c_int as libc::c_uint,
    138 as libc::c_int as libc::c_uint,
    136 as libc::c_int as libc::c_uint,
    134 as libc::c_int as libc::c_uint,
    132 as libc::c_int as libc::c_uint,
    130 as libc::c_int as libc::c_uint,
    128 as libc::c_int as libc::c_uint,
    126 as libc::c_int as libc::c_uint,
    123 as libc::c_int as libc::c_uint,
    121 as libc::c_int as libc::c_uint,
    119 as libc::c_int as libc::c_uint,
    117 as libc::c_int as libc::c_uint,
    115 as libc::c_int as libc::c_uint,
    114 as libc::c_int as libc::c_uint,
    112 as libc::c_int as libc::c_uint,
    110 as libc::c_int as libc::c_uint,
    108 as libc::c_int as libc::c_uint,
    106 as libc::c_int as libc::c_uint,
    104 as libc::c_int as libc::c_uint,
    102 as libc::c_int as libc::c_uint,
    100 as libc::c_int as libc::c_uint,
    98 as libc::c_int as libc::c_uint,
    96 as libc::c_int as libc::c_uint,
    94 as libc::c_int as libc::c_uint,
    93 as libc::c_int as libc::c_uint,
    91 as libc::c_int as libc::c_uint,
    89 as libc::c_int as libc::c_uint,
    87 as libc::c_int as libc::c_uint,
    85 as libc::c_int as libc::c_uint,
    83 as libc::c_int as libc::c_uint,
    82 as libc::c_int as libc::c_uint,
    80 as libc::c_int as libc::c_uint,
    78 as libc::c_int as libc::c_uint,
    76 as libc::c_int as libc::c_uint,
    74 as libc::c_int as libc::c_uint,
    73 as libc::c_int as libc::c_uint,
    71 as libc::c_int as libc::c_uint,
    69 as libc::c_int as libc::c_uint,
    67 as libc::c_int as libc::c_uint,
    66 as libc::c_int as libc::c_uint,
    64 as libc::c_int as libc::c_uint,
    62 as libc::c_int as libc::c_uint,
    61 as libc::c_int as libc::c_uint,
    59 as libc::c_int as libc::c_uint,
    57 as libc::c_int as libc::c_uint,
    55 as libc::c_int as libc::c_uint,
    54 as libc::c_int as libc::c_uint,
    52 as libc::c_int as libc::c_uint,
    50 as libc::c_int as libc::c_uint,
    49 as libc::c_int as libc::c_uint,
    47 as libc::c_int as libc::c_uint,
    46 as libc::c_int as libc::c_uint,
    44 as libc::c_int as libc::c_uint,
    42 as libc::c_int as libc::c_uint,
    41 as libc::c_int as libc::c_uint,
    39 as libc::c_int as libc::c_uint,
    37 as libc::c_int as libc::c_uint,
    36 as libc::c_int as libc::c_uint,
    34 as libc::c_int as libc::c_uint,
    33 as libc::c_int as libc::c_uint,
    31 as libc::c_int as libc::c_uint,
    30 as libc::c_int as libc::c_uint,
    28 as libc::c_int as libc::c_uint,
    26 as libc::c_int as libc::c_uint,
    25 as libc::c_int as libc::c_uint,
    23 as libc::c_int as libc::c_uint,
    22 as libc::c_int as libc::c_uint,
    20 as libc::c_int as libc::c_uint,
    19 as libc::c_int as libc::c_uint,
    17 as libc::c_int as libc::c_uint,
    16 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    13 as libc::c_int as libc::c_uint,
    11 as libc::c_int as libc::c_uint,
    10 as libc::c_int as libc::c_uint,
    8 as libc::c_int as libc::c_uint,
    7 as libc::c_int as libc::c_uint,
    5 as libc::c_int as libc::c_uint,
    4 as libc::c_int as libc::c_uint,
    2 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
];
unsafe extern "C" fn ZSTD_getFSEMaxSymbolValue(
    mut ctable: *const FSE_CTable,
) -> libc::c_uint {
    let mut ptr = ctable as *const libc::c_void;
    let mut u16ptr = ptr as *const U16;
    let maxSymbolValue = MEM_read16(
        u16ptr.offset(1 as libc::c_int as isize) as *const libc::c_void,
    ) as U32;
    return maxSymbolValue;
}
unsafe extern "C" fn ZSTD_useLowProbCount(nbSeq: size_t) -> libc::c_uint {
    return (nbSeq >= 2048 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn ZSTD_NCountCost(
    mut count: *const libc::c_uint,
    max: libc::c_uint,
    nbSeq: size_t,
    FSELog: libc::c_uint,
) -> size_t {
    let mut wksp: [BYTE; 512] = [0; 512];
    let mut norm: [S16; 53] = [0; 53];
    let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
    let err_code = FSE_normalizeCount(
        norm.as_mut_ptr(),
        tableLog,
        count,
        nbSeq,
        max,
        ZSTD_useLowProbCount(nbSeq),
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return FSE_writeNCount(
        wksp.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[BYTE; 512]>() as libc::c_ulong,
        norm.as_mut_ptr(),
        max,
        tableLog,
    );
}
unsafe extern "C" fn ZSTD_entropyCost(
    mut count: *const libc::c_uint,
    max: libc::c_uint,
    total: size_t,
) -> size_t {
    let mut cost = 0 as libc::c_int as libc::c_uint;
    let mut s: libc::c_uint = 0;
    if total > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"total > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"size_t ZSTD_entropyCost(const unsigned int *, const unsigned int, const size_t)\0",
            ))
                .as_ptr(),
        );
    }
    s = 0 as libc::c_int as libc::c_uint;
    while s <= max {
        let mut norm = ((256 as libc::c_int as libc::c_uint)
            .wrapping_mul(*count.offset(s as isize)) as libc::c_ulong)
            .wrapping_div(total) as libc::c_uint;
        if *count.offset(s as isize) != 0 as libc::c_int as libc::c_uint
            && norm == 0 as libc::c_int as libc::c_uint
        {
            norm = 1 as libc::c_int as libc::c_uint;
        }
        if (*count.offset(s as isize) as libc::c_ulong) < total {} else {
            __assert_fail(
                b"count[s] < total\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"size_t ZSTD_entropyCost(const unsigned int *, const unsigned int, const size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        cost = cost
            .wrapping_add(
                (*count.offset(s as isize))
                    .wrapping_mul(kInverseProbabilityLog256[norm as usize]),
            );
        s = s.wrapping_add(1);
    }
    return (cost >> 8 as libc::c_int) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_fseBitCost(
    mut ctable: *const FSE_CTable,
    mut count: *const libc::c_uint,
    max: libc::c_uint,
) -> size_t {
    let kAccuracyLog = 8 as libc::c_int as libc::c_uint;
    let mut cost = 0 as libc::c_int as size_t;
    let mut s: libc::c_uint = 0;
    let mut cstate = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    s = 0 as libc::c_int as libc::c_uint;
    while s <= max {
        let tableLog = cstate.stateLog;
        let badCost = tableLog.wrapping_add(1 as libc::c_int as libc::c_uint)
            << kAccuracyLog;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s, kAccuracyLog);
        if !(*count.offset(s as isize) == 0 as libc::c_int as libc::c_uint) {
            if bitCost >= badCost {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            cost = (cost as libc::c_ulong)
                .wrapping_add(
                    (*count.offset(s as isize) as size_t)
                        .wrapping_mul(bitCost as libc::c_ulong),
                ) as size_t as size_t;
        }
        s = s.wrapping_add(1);
    }
    return cost >> kAccuracyLog;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_crossEntropyCost(
    mut norm: *const libc::c_short,
    mut accuracyLog: libc::c_uint,
    mut count: *const libc::c_uint,
    max: libc::c_uint,
) -> size_t {
    let shift = (8 as libc::c_int as libc::c_uint).wrapping_sub(accuracyLog);
    let mut cost = 0 as libc::c_int as size_t;
    let mut s: libc::c_uint = 0;
    if accuracyLog <= 8 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"accuracyLog <= 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"size_t ZSTD_crossEntropyCost(const short *, unsigned int, const unsigned int *, const unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    s = 0 as libc::c_int as libc::c_uint;
    while s <= max {
        let normAcc = if *norm.offset(s as isize) as libc::c_int != -(1 as libc::c_int) {
            *norm.offset(s as isize) as libc::c_uint
        } else {
            1 as libc::c_int as libc::c_uint
        };
        let norm256 = normAcc << shift;
        if norm256 > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"norm256 > 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"size_t ZSTD_crossEntropyCost(const short *, unsigned int, const unsigned int *, const unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        if norm256 < 256 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"norm256 < 256\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                150 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"size_t ZSTD_crossEntropyCost(const short *, unsigned int, const unsigned int *, const unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        cost = (cost as libc::c_ulong)
            .wrapping_add(
                (*count.offset(s as isize))
                    .wrapping_mul(kInverseProbabilityLog256[norm256 as usize])
                    as libc::c_ulong,
            ) as size_t as size_t;
        s = s.wrapping_add(1);
    }
    return cost >> 8 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_selectEncodingType(
    mut repeatMode: *mut FSE_repeat,
    mut count: *const libc::c_uint,
    max: libc::c_uint,
    mostFrequent: size_t,
    mut nbSeq: size_t,
    FSELog: libc::c_uint,
    mut prevCTable: *const FSE_CTable,
    mut defaultNorm: *const libc::c_short,
    mut defaultNormLog: U32,
    isDefaultAllowed: ZSTD_defaultPolicy_e,
    strategy: ZSTD_strategy,
) -> symbolEncodingType_e {
    if mostFrequent == nbSeq {
        *repeatMode = FSE_repeat_none;
        if isDefaultAllowed as libc::c_uint != 0
            && nbSeq <= 2 as libc::c_int as libc::c_ulong
        {
            return set_basic;
        }
        return set_rle;
    }
    if (strategy as libc::c_uint) < ZSTD_lazy as libc::c_int as libc::c_uint {
        if isDefaultAllowed as u64 != 0 {
            let staticFse_nbSeq_max = 1000 as libc::c_int as size_t;
            let mult = (10 as libc::c_int as libc::c_uint)
                .wrapping_sub(strategy as libc::c_uint) as size_t;
            let baseLog = 3 as libc::c_int as size_t;
            let dynamicFse_nbSeq_min = ((1 as libc::c_int as size_t) << defaultNormLog)
                .wrapping_mul(mult) >> baseLog;
            if defaultNormLog >= 5 as libc::c_int as libc::c_uint
                && defaultNormLog <= 6 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"defaultNormLog >= 5 && defaultNormLog <= 6\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    185 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 232],
                        &[libc::c_char; 232],
                    >(
                        b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                    ))
                        .as_ptr(),
                );
            }
            if mult <= 9 as libc::c_int as libc::c_ulong
                && mult >= 7 as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"mult <= 9 && mult >= 7\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    186 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 232],
                        &[libc::c_char; 232],
                    >(
                        b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                    ))
                        .as_ptr(),
                );
            }
            if *repeatMode as libc::c_uint
                == FSE_repeat_valid as libc::c_int as libc::c_uint
                && nbSeq < staticFse_nbSeq_max
            {
                return set_repeat;
            }
            if nbSeq < dynamicFse_nbSeq_min
                || mostFrequent
                    < nbSeq
                        >> defaultNormLog.wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                *repeatMode = FSE_repeat_none;
                return set_basic;
            }
        }
    } else {
        let basicCost = if isDefaultAllowed as libc::c_uint != 0 {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count, max)
        } else {
            -(ZSTD_error_GENERIC as libc::c_int) as size_t
        };
        let repeatCost = if *repeatMode as libc::c_uint
            != FSE_repeat_none as libc::c_int as libc::c_uint
        {
            ZSTD_fseBitCost(prevCTable, count, max)
        } else {
            -(ZSTD_error_GENERIC as libc::c_int) as size_t
        };
        let NCountCost = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost = (NCountCost << 3 as libc::c_int)
            .wrapping_add(ZSTD_entropyCost(count, max, nbSeq));
        if isDefaultAllowed as u64 != 0 {
            if ERR_isError(basicCost) == 0 {} else {
                __assert_fail(
                    b"!ZSTD_isError(basicCost)\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    212 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 232],
                        &[libc::c_char; 232],
                    >(
                        b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                    ))
                        .as_ptr(),
                );
            }
            if !(*repeatMode as libc::c_uint
                == FSE_repeat_valid as libc::c_int as libc::c_uint
                && ERR_isError(repeatCost) != 0)
            {} else {
                __assert_fail(
                    b"!(*repeatMode == FSE_repeat_valid && ZSTD_isError(repeatCost))\0"
                        as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    213 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 232],
                        &[libc::c_char; 232],
                    >(
                        b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        if ERR_isError(NCountCost) == 0 {} else {
            __assert_fail(
                b"!ZSTD_isError(NCountCost)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 232],
                    &[libc::c_char; 232],
                >(
                    b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                ))
                    .as_ptr(),
            );
        }
        if compressedCost < -(ZSTD_error_maxCode as libc::c_int) as size_t {} else {
            __assert_fail(
                b"compressedCost < ERROR(maxCode)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                216 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 232],
                    &[libc::c_char; 232],
                >(
                    b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                ))
                    .as_ptr(),
            );
        }
        if basicCost <= repeatCost && basicCost <= compressedCost {
            if isDefaultAllowed as u64 != 0 {} else {
                __assert_fail(
                    b"isDefaultAllowed\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    221 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 232],
                        &[libc::c_char; 232],
                    >(
                        b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                    ))
                        .as_ptr(),
                );
            }
            *repeatMode = FSE_repeat_none;
            return set_basic;
        }
        if repeatCost <= compressedCost {
            if ERR_isError(repeatCost) == 0 {} else {
                __assert_fail(
                    b"!ZSTD_isError(repeatCost)\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    227 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 232],
                        &[libc::c_char; 232],
                    >(
                        b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                    ))
                        .as_ptr(),
                );
            }
            return set_repeat;
        }
        if compressedCost < basicCost && compressedCost < repeatCost {} else {
            __assert_fail(
                b"compressedCost < basicCost && compressedCost < repeatCost\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 232],
                    &[libc::c_char; 232],
                >(
                    b"symbolEncodingType_e ZSTD_selectEncodingType(FSE_repeat *, const unsigned int *, const unsigned int, const size_t, size_t, const unsigned int, const FSE_CTable *, const short *, U32, const ZSTD_defaultPolicy_e, const ZSTD_strategy)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    *repeatMode = FSE_repeat_check;
    return set_compressed;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_buildCTable(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut nextCTable: *mut FSE_CTable,
    mut FSELog: U32,
    mut type_0: symbolEncodingType_e,
    mut count: *mut libc::c_uint,
    mut max: U32,
    mut codeTable: *const BYTE,
    mut nbSeq: size_t,
    mut defaultNorm: *const S16,
    mut defaultNormLog: U32,
    mut defaultMax: U32,
    mut prevCTable: *const FSE_CTable,
    mut prevCTableSize: size_t,
    mut entropyWorkspace: *mut libc::c_void,
    mut entropyWorkspaceSize: size_t,
) -> size_t {
    let mut op = dst as *mut BYTE;
    let oend: *const BYTE = op.offset(dstCapacity as isize);
    match type_0 as libc::c_uint {
        1 => {
            let err_code = FSE_buildCTable_rle(nextCTable, max as BYTE);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if dstCapacity == 0 as libc::c_int as libc::c_ulong {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            *op = *codeTable.offset(0 as libc::c_int as isize);
            return 1 as libc::c_int as size_t;
        }
        3 => {
            libc::memcpy(
                nextCTable as *mut libc::c_void,
                prevCTable as *const libc::c_void,
                prevCTableSize as libc::size_t,
            );
            return 0 as libc::c_int as size_t;
        }
        0 => {
            let err_code_0 = FSE_buildCTable_wksp(
                nextCTable,
                defaultNorm,
                defaultMax,
                defaultNormLog,
                entropyWorkspace,
                entropyWorkspaceSize,
            );
            if ERR_isError(err_code_0) != 0 {
                return err_code_0;
            }
            return 0 as libc::c_int as size_t;
        }
        2 => {
            let mut wksp = entropyWorkspace as *mut ZSTD_BuildCTableWksp;
            let mut nbSeq_1 = nbSeq;
            let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
            if *count
                .offset(
                    *codeTable
                        .offset(
                            nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as isize,
                ) > 1 as libc::c_int as libc::c_uint
            {
                let ref mut fresh0 = *count
                    .offset(
                        *codeTable
                            .offset(
                                nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as isize,
                    );
                *fresh0 = (*fresh0).wrapping_sub(1);
                nbSeq_1 = nbSeq_1.wrapping_sub(1);
            }
            if nbSeq_1 > 1 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"nbSeq_1 > 1\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    275 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 191],
                        &[libc::c_char; 191],
                    >(
                        b"size_t ZSTD_buildCTable(void *, size_t, FSE_CTable *, U32, symbolEncodingType_e, unsigned int *, U32, const BYTE *, size_t, const S16 *, U32, U32, const FSE_CTable *, size_t, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            if entropyWorkspaceSize
                >= ::core::mem::size_of::<ZSTD_BuildCTableWksp>() as libc::c_ulong
            {} else {
                __assert_fail(
                    b"entropyWorkspaceSize >= sizeof(ZSTD_BuildCTableWksp)\0"
                        as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    276 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 191],
                        &[libc::c_char; 191],
                    >(
                        b"size_t ZSTD_buildCTable(void *, size_t, FSE_CTable *, U32, symbolEncodingType_e, unsigned int *, U32, const BYTE *, size_t, const S16 *, U32, U32, const FSE_CTable *, size_t, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            let err_code_1 = FSE_normalizeCount(
                ((*wksp).norm).as_mut_ptr(),
                tableLog,
                count,
                nbSeq_1,
                max,
                ZSTD_useLowProbCount(nbSeq_1),
            );
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            if oend >= op as *const BYTE {} else {
                __assert_fail(
                    b"oend >= op\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                        as *const u8 as *const libc::c_char,
                    279 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 191],
                        &[libc::c_char; 191],
                    >(
                        b"size_t ZSTD_buildCTable(void *, size_t, FSE_CTable *, U32, symbolEncodingType_e, unsigned int *, U32, const BYTE *, size_t, const S16 *, U32, U32, const FSE_CTable *, size_t, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            let NCountSize = FSE_writeNCount(
                op as *mut libc::c_void,
                oend.offset_from(op) as libc::c_long as size_t,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
            );
            let err_code_2 = NCountSize;
            if ERR_isError(err_code_2) != 0 {
                return err_code_2;
            }
            let err_code_3 = FSE_buildCTable_wksp(
                nextCTable,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
                ((*wksp).wksp).as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[U32; 285]>() as libc::c_ulong,
            );
            if ERR_isError(err_code_3) != 0 {
                return err_code_3;
            }
            return NCountSize;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/zstd_compress_sequences.c\0"
                    as *const u8 as *const libc::c_char,
                286 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 191],
                    &[libc::c_char; 191],
                >(
                    b"size_t ZSTD_buildCTable(void *, size_t, FSE_CTable *, U32, symbolEncodingType_e, unsigned int *, U32, const BYTE *, size_t, const S16 *, U32, U32, const FSE_CTable *, size_t, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn ZSTD_encodeSequences_body(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const BYTE,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const BYTE,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const BYTE,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut longOffsets: libc::c_int,
) -> size_t {
    let mut blockStream = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        endPtr: 0 as *mut libc::c_char,
    };
    let mut stateMatchLength = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    let mut stateOffsetBits = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    let mut stateLitLength = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    if ERR_isError(BIT_initCStream(&mut blockStream, dst, dstCapacity)) != 0 {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    FSE_initCState2(
        &mut stateMatchLength,
        CTable_MatchLength,
        *mlCodeTable
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32,
    );
    FSE_initCState2(
        &mut stateOffsetBits,
        CTable_OffsetBits,
        *ofCodeTable
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32,
    );
    FSE_initCState2(
        &mut stateLitLength,
        CTable_LitLength,
        *llCodeTable
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32,
    );
    BIT_addBits(
        &mut blockStream,
        (*sequences
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .litLength as size_t,
        LL_bits[*llCodeTable
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as usize] as libc::c_uint,
    );
    if MEM_32bits() != 0 {
        BIT_flushBits(&mut blockStream);
    }
    BIT_addBits(
        &mut blockStream,
        (*sequences
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .mlBase as size_t,
        ML_bits[*mlCodeTable
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as usize] as libc::c_uint,
    );
    if MEM_32bits() != 0 {
        BIT_flushBits(&mut blockStream);
    }
    if longOffsets != 0 {
        let ofBits = *ofCodeTable
            .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32;
        let extraBits = ofBits
            .wrapping_sub(
                (if ofBits
                    < ((if MEM_32bits() != 0 {
                        25 as libc::c_int
                    } else {
                        57 as libc::c_int
                    }) as U32)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    ofBits
                } else {
                    ((if MEM_32bits() != 0 {
                        25 as libc::c_int
                    } else {
                        57 as libc::c_int
                    }) as U32)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                }),
            );
        if extraBits != 0 {
            BIT_addBits(
                &mut blockStream,
                (*sequences
                    .offset(
                        nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .offBase as size_t,
                extraBits,
            );
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            ((*sequences
                .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .offBase >> extraBits) as size_t,
            ofBits.wrapping_sub(extraBits),
        );
    } else {
        BIT_addBits(
            &mut blockStream,
            (*sequences
                .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .offBase as size_t,
            *ofCodeTable
                .offset(nbSeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_uint,
        );
    }
    BIT_flushBits(&mut blockStream);
    let mut n: size_t = 0;
    n = nbSeq.wrapping_sub(2 as libc::c_int as libc::c_ulong);
    while n < nbSeq {
        let llCode = *llCodeTable.offset(n as isize);
        let ofCode = *ofCodeTable.offset(n as isize);
        let mlCode = *mlCodeTable.offset(n as isize);
        let llBits = LL_bits[llCode as usize] as U32;
        let ofBits_0 = ofCode as U32;
        let mlBits = ML_bits[mlCode as usize] as U32;
        FSE_encodeSymbol(&mut blockStream, &mut stateOffsetBits, ofCode as libc::c_uint);
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateMatchLength,
            mlCode as libc::c_uint,
        );
        if MEM_32bits() != 0 {
            BIT_flushBits(&mut blockStream);
        }
        FSE_encodeSymbol(&mut blockStream, &mut stateLitLength, llCode as libc::c_uint);
        if MEM_32bits() != 0
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits)
                >= (64 as libc::c_int - 7 as libc::c_int
                    - (LLFSELog + MLFSELog + OffFSELog)) as libc::c_uint
        {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.offset(n as isize)).litLength as size_t,
            llBits,
        );
        if MEM_32bits() != 0
            && llBits.wrapping_add(mlBits) > 24 as libc::c_int as libc::c_uint
        {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.offset(n as isize)).mlBase as size_t,
            mlBits,
        );
        if MEM_32bits() != 0
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits)
                > 56 as libc::c_int as libc::c_uint
        {
            BIT_flushBits(&mut blockStream);
        }
        if longOffsets != 0 {
            let extraBits_0 = ofBits_0
                .wrapping_sub(
                    (if ofBits_0
                        < ((if MEM_32bits() != 0 {
                            25 as libc::c_int
                        } else {
                            57 as libc::c_int
                        }) as U32)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    {
                        ofBits_0
                    } else {
                        ((if MEM_32bits() != 0 {
                            25 as libc::c_int
                        } else {
                            57 as libc::c_int
                        }) as U32)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    }),
                );
            if extraBits_0 != 0 {
                BIT_addBits(
                    &mut blockStream,
                    (*sequences.offset(n as isize)).offBase as size_t,
                    extraBits_0,
                );
                BIT_flushBits(&mut blockStream);
            }
            BIT_addBits(
                &mut blockStream,
                ((*sequences.offset(n as isize)).offBase >> extraBits_0) as size_t,
                ofBits_0.wrapping_sub(extraBits_0),
            );
        } else {
            BIT_addBits(
                &mut blockStream,
                (*sequences.offset(n as isize)).offBase as size_t,
                ofBits_0,
            );
        }
        BIT_flushBits(&mut blockStream);
        n = n.wrapping_sub(1);
    }
    FSE_flushCState(&mut blockStream, &mut stateMatchLength);
    FSE_flushCState(&mut blockStream, &mut stateOffsetBits);
    FSE_flushCState(&mut blockStream, &mut stateLitLength);
    let streamSize = BIT_closeCStream(&mut blockStream);
    if streamSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return streamSize;
}
unsafe extern "C" fn ZSTD_encodeSequences_default(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const BYTE,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const BYTE,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const BYTE,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut longOffsets: libc::c_int,
) -> size_t {
    return ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    );
}
unsafe extern "C" fn ZSTD_encodeSequences_bmi2(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const BYTE,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const BYTE,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const BYTE,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut longOffsets: libc::c_int,
) -> size_t {
    return ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_encodeSequences(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const BYTE,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const BYTE,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const BYTE,
    mut sequences: *const seqDef,
    mut nbSeq: size_t,
    mut longOffsets: libc::c_int,
    mut bmi2: libc::c_int,
) -> size_t {
    if bmi2 != 0 {
        return ZSTD_encodeSequences_bmi2(
            dst,
            dstCapacity,
            CTable_MatchLength,
            mlCodeTable,
            CTable_OffsetBits,
            ofCodeTable,
            CTable_LitLength,
            llCodeTable,
            sequences,
            nbSeq,
            longOffsets,
        );
    }
    return ZSTD_encodeSequences_default(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    );
}
