use ::libc;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type FSE_CTable = libc::c_uint;
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
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return *(ptr as *const unalign16);
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: U32) {
    *(memPtr as *mut unalign32) = value;
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
unsafe extern "C" fn BIT_flushBitsFast(mut bitC: *mut BIT_CStream_t) {
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
            206 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void BIT_flushBitsFast(BIT_CStream_t *)\0"))
                .as_ptr(),
        );
    }
    if (*bitC).ptr <= (*bitC).endPtr {} else {
        __assert_fail(
            b"bitC->ptr <= bitC->endPtr\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void BIT_flushBitsFast(BIT_CStream_t *)\0"))
                .as_ptr(),
        );
    }
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    (*bitC).bitPos &= 7 as libc::c_int as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8 as libc::c_int as libc::c_ulong);
}
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
pub const FSE_NCOUNTBOUND: libc::c_int = 512 as libc::c_int;
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
pub const FSE_MAX_MEMORY_USAGE: libc::c_int = 14 as libc::c_int;
pub const FSE_DEFAULT_MEMORY_USAGE: libc::c_int = 13 as libc::c_int;
pub const FSE_MAX_TABLELOG: libc::c_int = FSE_MAX_MEMORY_USAGE - 2 as libc::c_int;
pub const FSE_DEFAULT_TABLELOG: libc::c_int = FSE_DEFAULT_MEMORY_USAGE
    - 2 as libc::c_int;
pub const FSE_MIN_TABLELOG: libc::c_int = 5 as libc::c_int;
pub const FSE_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_wksp(
    mut ct: *mut FSE_CTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let tableSize = ((1 as libc::c_int) << tableLog) as U32;
    let tableMask = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let ptr = ct as *mut libc::c_void;
    let tableU16 = (ptr as *mut U16).offset(2 as libc::c_int as isize);
    let FSCT = (ptr as *mut U32)
        .offset(1 as libc::c_int as isize)
        .offset(
            (if tableLog != 0 {
                tableSize >> 1 as libc::c_int
            } else {
                1 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut libc::c_void;
    let symbolTT = FSCT as *mut FSE_symbolCompressionTransform;
    let step = (tableSize >> 1 as libc::c_int)
        .wrapping_add(tableSize >> 3 as libc::c_int)
        .wrapping_add(3 as libc::c_int as libc::c_uint);
    let maxSV1 = maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut cumul = workSpace as *mut U16;
    let tableSymbol = cumul
        .offset(maxSV1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
        as *mut BYTE;
    let mut highThreshold = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if workSpace as size_t & 1 as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"((size_t)workSpace & 1) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_ulonglong)
        .wrapping_mul(
            (maxSymbolValue.wrapping_add(2 as libc::c_int as libc::c_uint)
                as libc::c_ulonglong)
                .wrapping_add((1 as libc::c_ulonglong) << tableLog)
                .wrapping_div(2 as libc::c_int as libc::c_ulonglong)
                .wrapping_add(
                    (::core::mem::size_of::<U64>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<U32>() as libc::c_ulong)
                        as libc::c_ulonglong,
                ),
        ) > wkspSize as libc::c_ulonglong
    {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    *tableU16.offset(-(2 as libc::c_int) as isize) = tableLog as U16;
    *tableU16.offset(-(1 as libc::c_int) as isize) = maxSymbolValue as U16;
    if tableLog < 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tableLog < 16\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mut u: U32 = 0;
    *cumul.offset(0 as libc::c_int as isize) = 0 as libc::c_int as U16;
    u = 1 as libc::c_int as U32;
    while u <= maxSV1 {
        if *normalizedCounter
            .offset(u.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == -(1 as libc::c_int)
        {
            *cumul
                .offset(
                    u as isize,
                ) = (*cumul
                .offset(u.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int + 1 as libc::c_int) as U16;
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            *tableSymbol
                .offset(
                    fresh0 as isize,
                ) = u.wrapping_sub(1 as libc::c_int as libc::c_uint) as BYTE;
        } else {
            if *normalizedCounter
                .offset(u.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int >= 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"normalizedCounter[u-1] >= 0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    108 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            *cumul
                .offset(
                    u as isize,
                ) = (*cumul
                .offset(u.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int
                + *normalizedCounter
                    .offset(u.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                    as U16 as libc::c_int) as U16;
            if *cumul.offset(u as isize) as libc::c_int
                >= *cumul
                    .offset(u.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
            {} else {
                __assert_fail(
                    b"cumul[u] >= cumul[u-1]\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    110 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        u = u.wrapping_add(1);
    }
    *cumul
        .offset(
            maxSV1 as isize,
        ) = tableSize.wrapping_add(1 as libc::c_int as libc::c_uint) as U16;
    if highThreshold == tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        let spread = tableSymbol.offset(tableSize as isize);
        let add = 0x101010101010101 as libc::c_ulonglong as U64;
        let mut pos = 0 as libc::c_int as size_t;
        let mut sv = 0 as libc::c_int as U64;
        let mut s: U32 = 0;
        s = 0 as libc::c_int as U32;
        while s < maxSV1 {
            let mut i: libc::c_int = 0;
            let n = *normalizedCounter.offset(s as isize) as libc::c_int;
            MEM_write64(spread.offset(pos as isize) as *mut libc::c_void, sv);
            i = 8 as libc::c_int;
            while i < n {
                MEM_write64(
                    spread.offset(pos as isize).offset(i as isize) as *mut libc::c_void,
                    sv,
                );
                i += 8 as libc::c_int;
            }
            if n >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"n>=0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    132 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            pos = (pos as libc::c_ulong).wrapping_add(n as size_t) as size_t as size_t;
            s = s.wrapping_add(1);
            sv = (sv as libc::c_ulong).wrapping_add(add) as U64 as U64;
        }
        let mut position = 0 as libc::c_int as size_t;
        let mut s_0: size_t = 0;
        let unroll = 2 as libc::c_int as size_t;
        if (tableSize as libc::c_ulong).wrapping_rem(unroll)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"tableSize % unroll == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                    as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        s_0 = 0 as libc::c_int as size_t;
        while s_0 < tableSize as size_t {
            let mut u_0: size_t = 0;
            u_0 = 0 as libc::c_int as size_t;
            while u_0 < unroll {
                let uPosition = position
                    .wrapping_add(u_0.wrapping_mul(step as libc::c_ulong))
                    & tableMask as libc::c_ulong;
                *tableSymbol
                    .offset(
                        uPosition as isize,
                    ) = *spread.offset(s_0.wrapping_add(u_0) as isize);
                u_0 = u_0.wrapping_add(1);
            }
            position = position.wrapping_add(unroll.wrapping_mul(step as libc::c_ulong))
                & tableMask as libc::c_ulong;
            s_0 = (s_0 as libc::c_ulong).wrapping_add(unroll) as size_t as size_t;
        }
        if position == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"position == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                    as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        let mut position_0 = 0 as libc::c_int as U32;
        let mut symbol: U32 = 0;
        symbol = 0 as libc::c_int as U32;
        while symbol < maxSV1 {
            let mut nbOccurrences: libc::c_int = 0;
            let freq = *normalizedCounter.offset(symbol as isize) as libc::c_int;
            nbOccurrences = 0 as libc::c_int;
            while nbOccurrences < freq {
                *tableSymbol.offset(position_0 as isize) = symbol as BYTE;
                position_0 = position_0.wrapping_add(step) & tableMask;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step) & tableMask;
                }
                nbOccurrences += 1;
            }
            symbol = symbol.wrapping_add(1);
        }
        if position_0 == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"position==0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                    as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    let mut u_1: U32 = 0;
    u_1 = 0 as libc::c_int as U32;
    while u_1 < tableSize {
        let mut s_1 = *tableSymbol.offset(u_1 as isize);
        let ref mut fresh1 = *cumul.offset(s_1 as isize);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        *tableU16.offset(fresh2 as isize) = tableSize.wrapping_add(u_1) as U16;
        u_1 = u_1.wrapping_add(1);
    }
    let mut total = 0 as libc::c_int as libc::c_uint;
    let mut s_2: libc::c_uint = 0;
    s_2 = 0 as libc::c_int as libc::c_uint;
    while s_2 <= maxSymbolValue {
        match *normalizedCounter.offset(s_2 as isize) as libc::c_int {
            0 => {
                (*symbolTT.offset(s_2 as isize))
                    .deltaNbBits = (tableLog
                    .wrapping_add(1 as libc::c_int as libc::c_uint) << 16 as libc::c_int)
                    .wrapping_sub(((1 as libc::c_int) << tableLog) as libc::c_uint);
            }
            -1 | 1 => {
                (*symbolTT.offset(s_2 as isize))
                    .deltaNbBits = (tableLog << 16 as libc::c_int)
                    .wrapping_sub(((1 as libc::c_int) << tableLog) as libc::c_uint);
                if total <= 2147483647 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"total <= INT_MAX\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        189 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                (*symbolTT.offset(s_2 as isize))
                    .deltaFindState = total
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                total = total.wrapping_add(1);
            }
            _ => {
                if *normalizedCounter.offset(s_2 as isize) as libc::c_int
                    > 1 as libc::c_int
                {} else {
                    __assert_fail(
                        b"normalizedCounter[s] > 1\0" as *const u8
                            as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0"
                            as *const u8 as *const libc::c_char,
                        194 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"size_t FSE_buildCTable_wksp(FSE_CTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                let maxBitsOut = tableLog
                    .wrapping_sub(
                        ZSTD_highbit32(
                            (*normalizedCounter.offset(s_2 as isize) as U32)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ),
                    );
                let minStatePlus = (*normalizedCounter.offset(s_2 as isize) as U32)
                    << maxBitsOut;
                (*symbolTT.offset(s_2 as isize))
                    .deltaNbBits = (maxBitsOut << 16 as libc::c_int)
                    .wrapping_sub(minStatePlus);
                (*symbolTT.offset(s_2 as isize))
                    .deltaFindState = total
                    .wrapping_sub(
                        *normalizedCounter.offset(s_2 as isize) as libc::c_uint,
                    ) as libc::c_int;
                total = total
                    .wrapping_add(
                        *normalizedCounter.offset(s_2 as isize) as libc::c_uint,
                    );
            }
        }
        s_2 = s_2.wrapping_add(1);
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_NCountWriteBound(
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    let maxHeaderSize = maxSymbolValue
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_mul(tableLog)
        .wrapping_add(4 as libc::c_int as libc::c_uint)
        .wrapping_add(2 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_add(2 as libc::c_int as libc::c_uint) as size_t;
    return if maxSymbolValue != 0 {
        maxHeaderSize
    } else {
        FSE_NCOUNTBOUND as libc::c_ulong
    };
}
unsafe extern "C" fn FSE_writeNCount_generic(
    mut header: *mut libc::c_void,
    mut headerBufferSize: size_t,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
    mut writeIsSafe: libc::c_uint,
) -> size_t {
    let ostart = header as *mut BYTE;
    let mut out = ostart;
    let oend = ostart.offset(headerBufferSize as isize);
    let mut nbBits: libc::c_int = 0;
    let tableSize = (1 as libc::c_int) << tableLog;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream = 0 as libc::c_int as U32;
    let mut bitCount = 0 as libc::c_int;
    let mut symbol = 0 as libc::c_int as libc::c_uint;
    let alphabetSize = maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut previousIs0 = 0 as libc::c_int;
    bitStream = (bitStream as libc::c_uint)
        .wrapping_add(
            tableLog.wrapping_sub(FSE_MIN_TABLELOG as libc::c_uint) << bitCount,
        ) as U32 as U32;
    bitCount += 4 as libc::c_int;
    remaining = tableSize + 1 as libc::c_int;
    threshold = tableSize;
    nbBits = tableLog.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    while symbol < alphabetSize && remaining > 1 as libc::c_int {
        if previousIs0 != 0 {
            let mut start = symbol;
            while symbol < alphabetSize
                && *normalizedCounter.offset(symbol as isize) == 0
            {
                symbol = symbol.wrapping_add(1);
            }
            if symbol == alphabetSize {
                break;
            }
            while symbol >= start.wrapping_add(24 as libc::c_int as libc::c_uint) {
                start = start.wrapping_add(24 as libc::c_int as libc::c_uint);
                bitStream = (bitStream as libc::c_uint)
                    .wrapping_add((0xffff as libc::c_uint) << bitCount) as U32 as U32;
                if writeIsSafe == 0 && out > oend.offset(-(2 as libc::c_int as isize)) {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
                }
                *out.offset(0 as libc::c_int as isize) = bitStream as BYTE;
                *out
                    .offset(
                        1 as libc::c_int as isize,
                    ) = (bitStream >> 8 as libc::c_int) as BYTE;
                out = out.offset(2 as libc::c_int as isize);
                bitStream >>= 16 as libc::c_int;
            }
            while symbol >= start.wrapping_add(3 as libc::c_int as libc::c_uint) {
                start = start.wrapping_add(3 as libc::c_int as libc::c_uint);
                bitStream = (bitStream as libc::c_uint)
                    .wrapping_add(((3 as libc::c_int) << bitCount) as libc::c_uint)
                    as U32 as U32;
                bitCount += 2 as libc::c_int;
            }
            bitStream = (bitStream as libc::c_uint)
                .wrapping_add(symbol.wrapping_sub(start) << bitCount) as U32 as U32;
            bitCount += 2 as libc::c_int;
            if bitCount > 16 as libc::c_int {
                if writeIsSafe == 0 && out > oend.offset(-(2 as libc::c_int as isize)) {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
                }
                *out.offset(0 as libc::c_int as isize) = bitStream as BYTE;
                *out
                    .offset(
                        1 as libc::c_int as isize,
                    ) = (bitStream >> 8 as libc::c_int) as BYTE;
                out = out.offset(2 as libc::c_int as isize);
                bitStream >>= 16 as libc::c_int;
                bitCount -= 16 as libc::c_int;
            }
        }
        let fresh3 = symbol;
        symbol = symbol.wrapping_add(1);
        let mut count = *normalizedCounter.offset(fresh3 as isize) as libc::c_int;
        let max = 2 as libc::c_int * threshold - 1 as libc::c_int - remaining;
        remaining -= if count < 0 as libc::c_int { -count } else { count };
        count += 1;
        if count >= threshold {
            count += max;
        }
        bitStream = (bitStream as libc::c_uint)
            .wrapping_add((count << bitCount) as libc::c_uint) as U32 as U32;
        bitCount += nbBits;
        bitCount -= (count < max) as libc::c_int;
        previousIs0 = (count == 1 as libc::c_int) as libc::c_int;
        if remaining < 1 as libc::c_int {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1 as libc::c_int;
        }
        if bitCount > 16 as libc::c_int {
            if writeIsSafe == 0 && out > oend.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            *out.offset(0 as libc::c_int as isize) = bitStream as BYTE;
            *out
                .offset(
                    1 as libc::c_int as isize,
                ) = (bitStream >> 8 as libc::c_int) as BYTE;
            out = out.offset(2 as libc::c_int as isize);
            bitStream >>= 16 as libc::c_int;
            bitCount -= 16 as libc::c_int;
        }
    }
    if remaining != 1 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if symbol <= alphabetSize {} else {
        __assert_fail(
            b"symbol <= alphabetSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                as *const libc::c_char,
            316 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t FSE_writeNCount_generic(void *, size_t, const short *, unsigned int, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    if writeIsSafe == 0 && out > oend.offset(-(2 as libc::c_int as isize)) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    *out.offset(0 as libc::c_int as isize) = bitStream as BYTE;
    *out.offset(1 as libc::c_int as isize) = (bitStream >> 8 as libc::c_int) as BYTE;
    out = out.offset(((bitCount + 7 as libc::c_int) / 8 as libc::c_int) as isize);
    return out.offset_from(ostart) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_writeNCount(
    mut buffer: *mut libc::c_void,
    mut bufferSize: size_t,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    if tableLog > FSE_MAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if tableLog < FSE_MIN_TABLELOG as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if bufferSize < FSE_NCountWriteBound(maxSymbolValue, tableLog) {
        return FSE_writeNCount_generic(
            buffer,
            bufferSize,
            normalizedCounter,
            maxSymbolValue,
            tableLog,
            0 as libc::c_int as libc::c_uint,
        );
    }
    return FSE_writeNCount_generic(
        buffer,
        bufferSize,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        1 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn FSE_minTableLog(
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
) -> libc::c_uint {
    let mut minBitsSrc = (ZSTD_highbit32(srcSize as U32))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut minBitsSymbols = (ZSTD_highbit32(maxSymbolValue))
        .wrapping_add(2 as libc::c_int as libc::c_uint);
    let mut minBits = if minBitsSrc < minBitsSymbols {
        minBitsSrc
    } else {
        minBitsSymbols
    };
    if srcSize > 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"srcSize > 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                as *const libc::c_char,
            352 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"unsigned int FSE_minTableLog(size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    return minBits;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_optimalTableLog_internal(
    mut maxTableLog: libc::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut minus: libc::c_uint,
) -> libc::c_uint {
    let mut maxBitsSrc = (ZSTD_highbit32(
        srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32,
    ))
        .wrapping_sub(minus);
    let mut tableLog = maxTableLog;
    let mut minBits = FSE_minTableLog(srcSize, maxSymbolValue);
    if srcSize > 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"srcSize > 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/fse_compress.c\0" as *const u8
                as *const libc::c_char,
            361 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"unsigned int FSE_optimalTableLog_internal(unsigned int, size_t, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    if tableLog == 0 as libc::c_int as libc::c_uint {
        tableLog = FSE_DEFAULT_TABLELOG as U32;
    }
    if maxBitsSrc < tableLog {
        tableLog = maxBitsSrc;
    }
    if minBits > tableLog {
        tableLog = minBits;
    }
    if tableLog < FSE_MIN_TABLELOG as libc::c_uint {
        tableLog = FSE_MIN_TABLELOG as U32;
    }
    if tableLog > FSE_MAX_TABLELOG as libc::c_uint {
        tableLog = FSE_MAX_TABLELOG as U32;
    }
    return tableLog;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_optimalTableLog(
    mut maxTableLog: libc::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
) -> libc::c_uint {
    return FSE_optimalTableLog_internal(
        maxTableLog,
        srcSize,
        maxSymbolValue,
        2 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn FSE_normalizeM2(
    mut norm: *mut libc::c_short,
    mut tableLog: U32,
    mut count: *const libc::c_uint,
    mut total: size_t,
    mut maxSymbolValue: U32,
    mut lowProbCount: libc::c_short,
) -> size_t {
    let NOT_YET_ASSIGNED = -(2 as libc::c_int) as libc::c_short;
    let mut s: U32 = 0;
    let mut distributed = 0 as libc::c_int as U32;
    let mut ToDistribute: U32 = 0;
    let lowThreshold = (total >> tableLog) as U32;
    let mut lowOne = (total.wrapping_mul(3 as libc::c_int as libc::c_ulong)
        >> tableLog.wrapping_add(1 as libc::c_int as libc::c_uint)) as U32;
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) == 0 as libc::c_int as libc::c_uint {
            *norm.offset(s as isize) = 0 as libc::c_int as libc::c_short;
        } else if *count.offset(s as isize) <= lowThreshold {
            *norm.offset(s as isize) = lowProbCount;
            distributed = distributed.wrapping_add(1);
            total = (total as libc::c_ulong)
                .wrapping_sub(*count.offset(s as isize) as libc::c_ulong) as size_t
                as size_t;
        } else if *count.offset(s as isize) <= lowOne {
            *norm.offset(s as isize) = 1 as libc::c_int as libc::c_short;
            distributed = distributed.wrapping_add(1);
            total = (total as libc::c_ulong)
                .wrapping_sub(*count.offset(s as isize) as libc::c_ulong) as size_t
                as size_t;
        } else {
            *norm.offset(s as isize) = NOT_YET_ASSIGNED;
        }
        s = s.wrapping_add(1);
    }
    ToDistribute = (((1 as libc::c_int) << tableLog) as libc::c_uint)
        .wrapping_sub(distributed);
    if ToDistribute == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as size_t;
    }
    if total.wrapping_div(ToDistribute as libc::c_ulong) > lowOne as libc::c_ulong {
        lowOne = total
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            .wrapping_div(
                ToDistribute.wrapping_mul(2 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            ) as U32;
        s = 0 as libc::c_int as U32;
        while s <= maxSymbolValue {
            if *norm.offset(s as isize) as libc::c_int == NOT_YET_ASSIGNED as libc::c_int
                && *count.offset(s as isize) <= lowOne
            {
                *norm.offset(s as isize) = 1 as libc::c_int as libc::c_short;
                distributed = distributed.wrapping_add(1);
                total = (total as libc::c_ulong)
                    .wrapping_sub(*count.offset(s as isize) as libc::c_ulong) as size_t
                    as size_t;
            }
            s = s.wrapping_add(1);
        }
        ToDistribute = (((1 as libc::c_int) << tableLog) as libc::c_uint)
            .wrapping_sub(distributed);
    }
    if distributed == maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint) {
        let mut maxV = 0 as libc::c_int as U32;
        let mut maxC = 0 as libc::c_int as U32;
        s = 0 as libc::c_int as U32;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) > maxC {
                maxV = s;
                maxC = *count.offset(s as isize);
            }
            s = s.wrapping_add(1);
        }
        let ref mut fresh4 = *norm.offset(maxV as isize);
        *fresh4 = (*fresh4 as libc::c_int + ToDistribute as libc::c_short as libc::c_int)
            as libc::c_short;
        return 0 as libc::c_int as size_t;
    }
    if total == 0 as libc::c_int as libc::c_ulong {
        s = 0 as libc::c_int as U32;
        while ToDistribute > 0 as libc::c_int as libc::c_uint {
            if *norm.offset(s as isize) as libc::c_int > 0 as libc::c_int {
                ToDistribute = ToDistribute.wrapping_sub(1);
                let ref mut fresh5 = *norm.offset(s as isize);
                *fresh5 += 1;
            }
            s = s
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(
                    maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint),
                );
        }
        return 0 as libc::c_int as size_t;
    }
    let vStepLog = (62 as libc::c_int as libc::c_uint).wrapping_sub(tableLog) as U64;
    let mid = ((1 as libc::c_ulonglong)
        << vStepLog.wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as U64;
    let rStep = ((1 as libc::c_int as U64) << vStepLog)
        .wrapping_mul(ToDistribute as libc::c_ulong)
        .wrapping_add(mid)
        .wrapping_div(total as U32 as libc::c_ulong);
    let mut tmpTotal = mid;
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        if *norm.offset(s as isize) as libc::c_int == NOT_YET_ASSIGNED as libc::c_int {
            let end = tmpTotal
                .wrapping_add(
                    (*count.offset(s as isize) as libc::c_ulong).wrapping_mul(rStep),
                );
            let sStart = (tmpTotal >> vStepLog) as U32;
            let sEnd = (end >> vStepLog) as U32;
            let weight = sEnd.wrapping_sub(sStart);
            if weight < 1 as libc::c_int as libc::c_uint {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            *norm.offset(s as isize) = weight as libc::c_short;
            tmpTotal = end;
        }
        s = s.wrapping_add(1);
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_normalizeCount(
    mut normalizedCounter: *mut libc::c_short,
    mut tableLog: libc::c_uint,
    mut count: *const libc::c_uint,
    mut total: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut useLowProbCount: libc::c_uint,
) -> size_t {
    if tableLog == 0 as libc::c_int as libc::c_uint {
        tableLog = FSE_DEFAULT_TABLELOG as libc::c_uint;
    }
    if tableLog < FSE_MIN_TABLELOG as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if tableLog > FSE_MAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if tableLog < FSE_minTableLog(total, maxSymbolValue) {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    static mut rtbTable: [U32; 8] = [
        0 as libc::c_int as U32,
        473195 as libc::c_int as U32,
        504333 as libc::c_int as U32,
        520860 as libc::c_int as U32,
        550000 as libc::c_int as U32,
        700000 as libc::c_int as U32,
        750000 as libc::c_int as U32,
        830000 as libc::c_int as U32,
    ];
    let lowProbCount = (if useLowProbCount != 0 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as libc::c_short;
    let scale = (62 as libc::c_int as libc::c_uint).wrapping_sub(tableLog) as U64;
    let step = ((1 as libc::c_int as U64) << 62 as libc::c_int)
        .wrapping_div(total as U32 as libc::c_ulong);
    let vStep = ((1 as libc::c_ulonglong)
        << scale.wrapping_sub(20 as libc::c_int as libc::c_ulong)) as U64;
    let mut stillToDistribute = (1 as libc::c_int) << tableLog;
    let mut s: libc::c_uint = 0;
    let mut largest = 0 as libc::c_int as libc::c_uint;
    let mut largestP = 0 as libc::c_int as libc::c_short;
    let mut lowThreshold = (total >> tableLog) as U32;
    s = 0 as libc::c_int as libc::c_uint;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) as libc::c_ulong == total {
            return 0 as libc::c_int as size_t;
        }
        if *count.offset(s as isize) == 0 as libc::c_int as libc::c_uint {
            *normalizedCounter.offset(s as isize) = 0 as libc::c_int as libc::c_short;
        } else if *count.offset(s as isize) <= lowThreshold {
            *normalizedCounter.offset(s as isize) = lowProbCount;
            stillToDistribute -= 1;
        } else {
            let mut proba = ((*count.offset(s as isize) as libc::c_ulong)
                .wrapping_mul(step) >> scale) as libc::c_short;
            if (proba as libc::c_int) < 8 as libc::c_int {
                let mut restToBeat = vStep
                    .wrapping_mul(rtbTable[proba as usize] as libc::c_ulong);
                proba = (proba as libc::c_int
                    + ((*count.offset(s as isize) as libc::c_ulong)
                        .wrapping_mul(step)
                        .wrapping_sub((proba as U64) << scale) > restToBeat)
                        as libc::c_int) as libc::c_short;
            }
            if proba as libc::c_int > largestP as libc::c_int {
                largestP = proba;
                largest = s;
            }
            *normalizedCounter.offset(s as isize) = proba;
            stillToDistribute -= proba as libc::c_int;
        }
        s = s.wrapping_add(1);
    }
    if -stillToDistribute
        >= *normalizedCounter.offset(largest as isize) as libc::c_int >> 1 as libc::c_int
    {
        let errorCode = FSE_normalizeM2(
            normalizedCounter,
            tableLog,
            count,
            total,
            maxSymbolValue,
            lowProbCount,
        );
        if ERR_isError(errorCode) != 0 {
            return errorCode;
        }
    } else {
        let ref mut fresh6 = *normalizedCounter.offset(largest as isize);
        *fresh6 = (*fresh6 as libc::c_int
            + stillToDistribute as libc::c_short as libc::c_int) as libc::c_short;
    }
    return tableLog as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_rle(
    mut ct: *mut FSE_CTable,
    mut symbolValue: BYTE,
) -> size_t {
    let mut ptr = ct as *mut libc::c_void;
    let mut tableU16 = (ptr as *mut U16).offset(2 as libc::c_int as isize);
    let mut FSCTptr = (ptr as *mut U32).offset(2 as libc::c_int as isize)
        as *mut libc::c_void;
    let mut symbolTT = FSCTptr as *mut FSE_symbolCompressionTransform;
    *tableU16.offset(-(2 as libc::c_int) as isize) = 0 as libc::c_int as U16;
    *tableU16.offset(-(1 as libc::c_int) as isize) = symbolValue as U16;
    *tableU16.offset(0 as libc::c_int as isize) = 0 as libc::c_int as U16;
    *tableU16.offset(1 as libc::c_int as isize) = 0 as libc::c_int as U16;
    (*symbolTT.offset(symbolValue as isize)).deltaNbBits = 0 as libc::c_int as U32;
    (*symbolTT.offset(symbolValue as isize)).deltaFindState = 0 as libc::c_int;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FSE_compress_usingCTable_generic(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut ct: *const FSE_CTable,
    fast: libc::c_uint,
) -> size_t {
    let istart = src as *const BYTE;
    let iend = istart.offset(srcSize as isize);
    let mut ip = iend;
    let mut bitC = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        endPtr: 0 as *mut libc::c_char,
    };
    let mut CState1 = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    let mut CState2 = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    if srcSize <= 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    let initError = BIT_initCStream(&mut bitC, dst, dstSize);
    if ERR_isError(initError) != 0 {
        return 0 as libc::c_int as size_t;
    }
    if srcSize & 1 as libc::c_int as libc::c_ulong != 0 {
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState1, ct, *ip as U32);
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState2, ct, *ip as U32);
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    } else {
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState2, ct, *ip as U32);
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState1, ct, *ip as U32);
    }
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > (FSE_MAX_TABLELOG * 4 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
        && srcSize & 2 as libc::c_int as libc::c_ulong != 0
    {
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as libc::c_uint);
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    }
    while ip > istart {
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as libc::c_uint);
        if (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            < (FSE_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
        {
            if fast != 0 {
                BIT_flushBitsFast(&mut bitC);
            } else {
                BIT_flushBits(&mut bitC);
            };
        }
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        if (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            > (FSE_MAX_TABLELOG * 4 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
        {
            ip = ip.offset(-1);
            FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as libc::c_uint);
            ip = ip.offset(-1);
            FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        }
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    }
    FSE_flushCState(&mut bitC, &mut CState2);
    FSE_flushCState(&mut bitC, &mut CState1);
    return BIT_closeCStream(&mut bitC);
}
#[no_mangle]
pub unsafe extern "C" fn FSE_compress_usingCTable(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut ct: *const FSE_CTable,
) -> size_t {
    let fast = (dstSize
        >= srcSize
            .wrapping_add(srcSize >> 7 as libc::c_int)
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong))
        as libc::c_int as libc::c_uint;
    if fast != 0 {
        return FSE_compress_usingCTable_generic(
            dst,
            dstSize,
            src,
            srcSize,
            ct,
            1 as libc::c_int as libc::c_uint,
        )
    } else {
        return FSE_compress_usingCTable_generic(
            dst,
            dstSize,
            src,
            srcSize,
            ct,
            0 as libc::c_int as libc::c_uint,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn FSE_compressBound(mut size: size_t) -> size_t {
    return (FSE_NCOUNTBOUND as libc::c_ulong)
        .wrapping_add(
            size
                .wrapping_add(size >> 7 as libc::c_int)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong),
        );
}
