use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn FSE_readNCount_bmi2(
        normalizedCounter: *mut libc::c_short,
        maxSymbolValuePtr: *mut libc::c_uint,
        tableLogPtr: *mut libc::c_uint,
        rBuffer: *const libc::c_void,
        rBuffSize: size_t,
        bmi2: libc::c_int,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
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
pub type U16 = uint16_t;
pub type S16 = int16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
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
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub type BIT_DStream_status = libc::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type FSE_DTable = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DecompressWksp {
    pub ncount: [libc::c_short; 256],
    pub dtable: [FSE_DTable; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let regMask = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32;
    if nbBits >= 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"nbBits >= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            347 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"size_t BIT_lookBitsFast(const BIT_DStream_t *, U32)\0"))
                .as_ptr(),
        );
    }
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask.wrapping_add(1 as libc::c_int as libc::c_uint).wrapping_sub(nbBits)
            & regMask);
}
#[inline]
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: libc::c_uint,
) -> size_t {
    let value = BIT_lookBitsFast(bitD, nbBits);
    if nbBits >= 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"nbBits >= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            372 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"size_t BIT_readBitsFast(BIT_DStream_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline]
unsafe extern "C" fn BIT_reloadDStreamFast(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if ((*bitD).ptr < (*bitD).limitPtr) as libc::c_int as libc::c_long != 0 {
        return BIT_DStream_overflow;
    }
    if (*bitD).bitsConsumed as libc::c_ulong
        <= (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"bitD->bitsConsumed <= sizeof(bitD->bitContainer)*8\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            387 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"BIT_DStream_status BIT_reloadDStreamFast(BIT_DStream_t *)\0"))
                .as_ptr(),
        );
    }
    (*bitD)
        .ptr = ((*bitD).ptr)
        .offset(-(((*bitD).bitsConsumed >> 3 as libc::c_int) as isize));
    (*bitD).bitsConsumed &= 7 as libc::c_int as libc::c_uint;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return BIT_DStream_unfinished;
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong
        > (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStreamFast(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong)
            < (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as libc::c_int;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as libc::c_long as U32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD)
        .bitsConsumed = ((*bitD).bitsConsumed)
        .wrapping_sub(nbBytes.wrapping_mul(8 as libc::c_int as libc::c_uint));
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return result;
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline(always)]
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: size_t,
    start: U32,
    nbBits: U32,
) -> size_t {
    let regMask = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32;
    if (nbBits as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbBits < BIT_MASK_SIZE\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bitstream.h\0"
                as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"size_t BIT_getMiddleBits(size_t, const U32, const U32)\0"))
                .as_ptr(),
        );
    }
    return bitContainer >> (start & regMask)
        & ((1 as libc::c_int as U64) << nbBits)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: U32,
) -> size_t {
    return BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub((*bitD).bitsConsumed as libc::c_ulong)
            .wrapping_sub(nbBits as libc::c_ulong) as U32,
        nbBits,
    );
}
#[inline(always)]
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: libc::c_uint,
) -> size_t {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline]
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 as libc::c_int as libc::c_ulong {
        libc::memset(
            bitD as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<BIT_DStream_t>() as libc::c_ulong as libc::size_t,
        );
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    (*bitD).start = srcBuffer as *const libc::c_char;
    (*bitD)
        .limitPtr = ((*bitD).start)
        .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
    if srcSize >= ::core::mem::size_of::<size_t>() as libc::c_ulong {
        (*bitD)
            .ptr = (srcBuffer as *const libc::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<size_t>() as libc::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        let lastByte = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        (*bitD)
            .bitsConsumed = if lastByte as libc::c_int != 0 {
            (8 as libc::c_int as libc::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte as U32))
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if lastByte as libc::c_int == 0 as libc::c_int {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const BYTE) as size_t;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(6 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(16 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 4958702415736314236;
            }
            6 => {
                current_block_32 = 4958702415736314236;
            }
            5 => {
                current_block_32 = 14069466093522752717;
            }
            4 => {
                current_block_32 = 3702419396666302431;
            }
            3 => {
                current_block_32 = 11927906626876583792;
            }
            2 => {
                current_block_32 = 13596834209139043397;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        match current_block_32 {
            4958702415736314236 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(5 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(24 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 14069466093522752717;
            }
            _ => {}
        }
        match current_block_32 {
            14069466093522752717 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(4 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(32 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 3702419396666302431;
            }
            _ => {}
        }
        match current_block_32 {
            3702419396666302431 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(3 as libc::c_int as isize)
                            as size_t) << 24 as libc::c_int,
                    ) as size_t as size_t;
                current_block_32 = 11927906626876583792;
            }
            _ => {}
        }
        match current_block_32 {
            11927906626876583792 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(2 as libc::c_int as isize)
                            as size_t) << 16 as libc::c_int,
                    ) as size_t as size_t;
                current_block_32 = 13596834209139043397;
            }
            _ => {}
        }
        match current_block_32 {
            13596834209139043397 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(1 as libc::c_int as isize)
                            as size_t) << 8 as libc::c_int,
                    ) as size_t as size_t;
            }
            _ => {}
        }
        let lastByte_0 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        (*bitD)
            .bitsConsumed = if lastByte_0 as libc::c_int != 0 {
            (8 as libc::c_int as libc::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as U32))
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if lastByte_0 as libc::c_int == 0 as libc::c_int {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                ((::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(srcSize) as U32)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint),
            );
    }
    return srcSize;
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bits.h\0"
                as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bits.h\0"
                as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as size_t
    } else {
        return MEM_readLE64(memPtr)
    };
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
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
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
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
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return *(ptr as *const unalign32);
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
unsafe extern "C" fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut ptr = dt as *const libc::c_void;
    let DTableH = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog as libc::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as libc::c_int as isize) as *const libc::c_void;
}
#[inline]
unsafe extern "C" fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> libc::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
#[inline]
unsafe extern "C" fn FSE_decodeSymbolFast(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> libc::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
pub const FSE_MAX_MEMORY_USAGE: libc::c_int = 14 as libc::c_int;
pub const FSE_MAX_SYMBOL_VALUE: libc::c_int = 255 as libc::c_int;
pub const FSE_MAX_TABLELOG: libc::c_int = FSE_MAX_MEMORY_USAGE - 2 as libc::c_int;
pub const FSE_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
unsafe extern "C" fn FSE_buildDTable_internal(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let tdPtr = dt.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let tableDecode = tdPtr as *mut FSE_decode_t;
    let mut symbolNext = workSpace as *mut U16;
    let mut spread = symbolNext
        .offset(maxSymbolValue as isize)
        .offset(1 as libc::c_int as isize) as *mut BYTE;
    let maxSV1 = maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint);
    let tableSize = ((1 as libc::c_int) << tableLog) as U32;
    let mut highThreshold = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if ((::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
        .wrapping_mul(
            maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
        ) as libc::c_ulonglong)
        .wrapping_add((1 as libc::c_ulonglong) << tableLog)
        .wrapping_add(8 as libc::c_int as libc::c_ulonglong)
        > wkspSize as libc::c_ulonglong
    {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    if maxSymbolValue > FSE_MAX_SYMBOL_VALUE as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    if tableLog > FSE_MAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as U16;
    DTableH.fastMode = 1 as libc::c_int as U16;
    let largeLimit = ((1 as libc::c_int)
        << tableLog.wrapping_sub(1 as libc::c_int as libc::c_uint)) as S16;
    let mut s: U32 = 0;
    s = 0 as libc::c_int as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -(1 as libc::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as BYTE;
            *symbolNext.offset(s as isize) = 1 as libc::c_int as U16;
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int
                >= largeLimit as libc::c_int
            {
                DTableH.fastMode = 0 as libc::c_int as U16;
            }
            *symbolNext
                .offset(s as isize) = *normalizedCounter.offset(s as isize) as U16;
        }
        s = s.wrapping_add(1);
    }
    libc::memcpy(
        dt as *mut libc::c_void,
        &mut DTableH as *mut FSE_DTableHeader as *const libc::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong as libc::size_t,
    );
    if highThreshold == tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        let tableMask = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint)
            as size_t;
        let step = (tableSize >> 1 as libc::c_int)
            .wrapping_add(tableSize >> 3 as libc::c_int)
            .wrapping_add(3 as libc::c_int as libc::c_uint) as size_t;
        let add = 0x101010101010101 as libc::c_ulonglong as U64;
        let mut pos = 0 as libc::c_int as size_t;
        let mut sv = 0 as libc::c_int as U64;
        let mut s_0: U32 = 0;
        s_0 = 0 as libc::c_int as U32;
        while s_0 < maxSV1 {
            let mut i: libc::c_int = 0;
            let n = *normalizedCounter.offset(s_0 as isize) as libc::c_int;
            MEM_write64(spread.offset(pos as isize) as *mut libc::c_void, sv);
            i = 8 as libc::c_int;
            while i < n {
                MEM_write64(
                    spread.offset(pos as isize).offset(i as isize) as *mut libc::c_void,
                    sv,
                );
                i += 8 as libc::c_int;
            }
            pos = (pos as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                as size_t;
            s_0 = s_0.wrapping_add(1);
            sv = (sv as libc::c_ulong).wrapping_add(add) as U64 as U64;
        }
        let mut position = 0 as libc::c_int as size_t;
        let mut s_1: size_t = 0;
        let unroll = 2 as libc::c_int as size_t;
        if (tableSize as libc::c_ulong).wrapping_rem(unroll)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"tableSize % unroll == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/common/fse_decompress.c\0" as *const u8
                    as *const libc::c_char,
                127 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"size_t FSE_buildDTable_internal(FSE_DTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        s_1 = 0 as libc::c_int as size_t;
        while s_1 < tableSize as size_t {
            let mut u: size_t = 0;
            u = 0 as libc::c_int as size_t;
            while u < unroll {
                let uPosition = position.wrapping_add(u.wrapping_mul(step)) & tableMask;
                (*tableDecode.offset(uPosition as isize))
                    .symbol = *spread.offset(s_1.wrapping_add(u) as isize);
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll.wrapping_mul(step)) & tableMask;
            s_1 = (s_1 as libc::c_ulong).wrapping_add(unroll) as size_t as size_t;
        }
        if position == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"position == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/common/fse_decompress.c\0" as *const u8
                    as *const libc::c_char,
                136 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"size_t FSE_buildDTable_internal(FSE_DTable *, const short *, unsigned int, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
        let step_0 = (tableSize >> 1 as libc::c_int)
            .wrapping_add(tableSize >> 3 as libc::c_int)
            .wrapping_add(3 as libc::c_int as libc::c_uint);
        let mut s_2: U32 = 0;
        let mut position_0 = 0 as libc::c_int as U32;
        s_2 = 0 as libc::c_int as U32;
        while s_2 < maxSV1 {
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < *normalizedCounter.offset(s_2 as isize) as libc::c_int {
                (*tableDecode.offset(position_0 as isize)).symbol = s_2 as BYTE;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
            }
            s_2 = s_2.wrapping_add(1);
        }
        if position_0 != 0 as libc::c_int as libc::c_uint {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
    }
    let mut u_0: U32 = 0;
    u_0 = 0 as libc::c_int as U32;
    while u_0 < tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).symbol;
        let ref mut fresh1 = *symbolNext.offset(symbol as isize);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as U32;
        (*tableDecode.offset(u_0 as isize))
            .nbBits = tableLog.wrapping_sub(ZSTD_highbit32(nextState)) as BYTE;
        (*tableDecode.offset(u_0 as isize))
            .newState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as libc::c_int)
            .wrapping_sub(tableSize) as U16;
        u_0 = u_0.wrapping_add(1);
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable_wksp(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    return FSE_buildDTable_internal(
        dt,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    );
}
#[inline(always)]
unsafe extern "C" fn FSE_decompress_usingDTable_generic(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSE_DTable,
    fast: libc::c_uint,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3 as libc::c_int as isize));
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut state1 = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let mut state2 = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);
    while (BIT_reloadDStream(&mut bitD) as libc::c_uint
        == BIT_DStream_unfinished as libc::c_int as libc::c_uint) as libc::c_int
        & (op < olimit) as libc::c_int != 0
    {
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op
            .offset(
                1 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 4 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            if BIT_reloadDStream(&mut bitD) as libc::c_uint
                > BIT_DStream_unfinished as libc::c_int as libc::c_uint
            {
                op = op.offset(2 as libc::c_int as isize);
                break;
            }
        }
        *op
            .offset(
                2 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op
            .offset(
                3 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as BYTE;
        op = op.offset(4 as libc::c_int as isize);
    }
    loop {
        if op > omax.offset(-(2 as libc::c_int as isize)) {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        let fresh3 = op;
        op = op.offset(1);
        *fresh3 = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if BIT_reloadDStream(&mut bitD) as libc::c_uint
            == BIT_DStream_overflow as libc::c_int as libc::c_uint
        {
            let fresh4 = op;
            op = op.offset(1);
            *fresh4 = (if fast != 0 {
                FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
            } else {
                FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
            }) as BYTE;
            break;
        } else {
            if op > omax.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            let fresh5 = op;
            op = op.offset(1);
            *fresh5 = (if fast != 0 {
                FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
            } else {
                FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
            }) as BYTE;
            if !(BIT_reloadDStream(&mut bitD) as libc::c_uint
                == BIT_DStream_overflow as libc::c_int as libc::c_uint)
            {
                continue;
            }
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = (if fast != 0 {
                FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
            } else {
                FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
            }) as BYTE;
            break;
        }
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn FSE_decompress_wksp_body(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut maxLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut bmi2: libc::c_int,
) -> size_t {
    let istart = cSrc as *const BYTE;
    let mut ip = istart;
    let mut tableLog: libc::c_uint = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as libc::c_uint;
    let wksp = workSpace as *mut FSE_DecompressWksp;
    if wkspSize < ::core::mem::size_of::<FSE_DecompressWksp>() as libc::c_ulong {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    let NCountLength = FSE_readNCount_bmi2(
        ((*wksp).ncount).as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const libc::c_void,
        cSrcSize,
        bmi2,
    );
    if ERR_isError(NCountLength) != 0 {
        return NCountLength;
    }
    if tableLog > maxLog {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if NCountLength <= cSrcSize {} else {
        __assert_fail(
            b"NCountLength <= cSrcSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/common/fse_decompress.c\0" as *const u8
                as *const libc::c_char,
            264 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t FSE_decompress_wksp_body(void *, size_t, const void *, size_t, unsigned int, void *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    ip = ip.offset(NCountLength as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(NCountLength) as size_t
        as size_t;
    if ((1 as libc::c_int + ((1 as libc::c_int) << tableLog) + 1 as libc::c_int)
        as libc::c_ulonglong)
        .wrapping_add(
            ((::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                .wrapping_mul(
                    maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong,
                ) as libc::c_ulonglong)
                .wrapping_add((1 as libc::c_ulonglong) << tableLog)
                .wrapping_add(8 as libc::c_int as libc::c_ulonglong)
                .wrapping_add(
                    ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
                        as libc::c_ulonglong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                .wrapping_div(
                    ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
                        as libc::c_ulonglong,
                ),
        )
        .wrapping_add(
            ((FSE_MAX_SYMBOL_VALUE + 1 as libc::c_int) / 2 as libc::c_int)
                as libc::c_ulonglong,
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        .wrapping_mul(
            ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_ulonglong,
        ) > wkspSize as libc::c_ulonglong
    {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if (::core::mem::size_of::<FSE_DecompressWksp>() as libc::c_ulong)
        .wrapping_add(
            ((1 as libc::c_int + ((1 as libc::c_int) << tableLog)) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<FSE_DTable>() as libc::c_ulong),
        ) <= wkspSize
    {} else {
        __assert_fail(
            b"sizeof(*wksp) + FSE_DTABLE_SIZE(tableLog) <= wkspSize\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/common/fse_decompress.c\0" as *const u8
                as *const libc::c_char,
            270 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"size_t FSE_decompress_wksp_body(void *, size_t, const void *, size_t, unsigned int, void *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    workSpace = (workSpace as *mut BYTE)
        .offset(::core::mem::size_of::<FSE_DecompressWksp>() as libc::c_ulong as isize)
        .offset(
            ((1 as libc::c_int + ((1 as libc::c_int) << tableLog)) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<FSE_DTable>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_void;
    wkspSize = (wkspSize as libc::c_ulong)
        .wrapping_sub(
            (::core::mem::size_of::<FSE_DecompressWksp>() as libc::c_ulong)
                .wrapping_add(
                    ((1 as libc::c_int + ((1 as libc::c_int) << tableLog))
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<FSE_DTable>() as libc::c_ulong,
                        ),
                ),
        ) as size_t as size_t;
    let _var_err__ = FSE_buildDTable_internal(
        ((*wksp).dtable).as_mut_ptr(),
        ((*wksp).ncount).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let mut ptr = ((*wksp).dtable).as_mut_ptr() as *const libc::c_void;
    let mut DTableH = ptr as *const FSE_DTableHeader;
    let fastMode = (*DTableH).fastMode as U32;
    if fastMode != 0 {
        return FSE_decompress_usingDTable_generic(
            dst,
            dstCapacity,
            ip as *const libc::c_void,
            cSrcSize,
            ((*wksp).dtable).as_mut_ptr(),
            1 as libc::c_int as libc::c_uint,
        );
    }
    return FSE_decompress_usingDTable_generic(
        dst,
        dstCapacity,
        ip as *const libc::c_void,
        cSrcSize,
        ((*wksp).dtable).as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn FSE_decompress_wksp_body_default(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut maxLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    return FSE_decompress_wksp_body(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn FSE_decompress_wksp_body_bmi2(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut maxLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    return FSE_decompress_wksp_body(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FSE_decompress_wksp_bmi2(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut maxLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut bmi2: libc::c_int,
) -> size_t {
    if bmi2 != 0 {
        return FSE_decompress_wksp_body_bmi2(
            dst,
            dstCapacity,
            cSrc,
            cSrcSize,
            maxLog,
            workSpace,
            wkspSize,
        );
    }
    return FSE_decompress_wksp_body_default(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
    );
}
