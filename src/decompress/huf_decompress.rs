use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn HUF_readStats_wksp(
        huffWeight: *mut BYTE,
        hwSize: size_t,
        rankStats: *mut U32,
        nbSymbolsPtr: *mut U32,
        tableLogPtr: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
        workspace: *mut libc::c_void,
        wkspSize: size_t,
        flags: libc::c_int,
    ) -> size_t;
    fn HUF_decompress4X1_usingDTable_internal_fast_asm_loop(
        args: *mut HUF_DecompressFastArgs,
    );
    fn HUF_decompress4X2_usingDTable_internal_fast_asm_loop(
        args: *mut HUF_DecompressFastArgs,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type HUF_DTable = U32;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: U32,
    pub decode256Time: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DTableDesc {
    pub maxTableLog: BYTE,
    pub tableType: BYTE,
    pub tableLog: BYTE,
    pub reserved: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX1 {
    pub nbBits: BYTE,
    pub byte: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_ReadDTableX1_Workspace {
    pub rankVal: [U32; 13],
    pub rankStart: [U32; 13],
    pub statsWksp: [U32; 219],
    pub symbols: [BYTE; 256],
    pub huffWeight: [BYTE; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX2 {
    pub sequence: U16,
    pub nbBits: BYTE,
    pub length: BYTE,
}
pub type rankValCol_t = [U32; 13];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_ReadDTableX2_Workspace {
    pub rankVal: [rankValCol_t; 12],
    pub rankStats: [U32; 13],
    pub rankStart0: [U32; 15],
    pub sortedSymbol: [sortedSymbol_t; 256],
    pub weightList: [BYTE; 256],
    pub calleeWksp: [U32; 219],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: BYTE,
}
pub type HUF_DecompressUsingDTableFn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        size_t,
        *const libc::c_void,
        size_t,
        *const HUF_DTable,
    ) -> size_t,
>;
pub type HUF_DecompressFastLoopFn = Option::<
    unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DecompressFastArgs {
    pub ip: [*const BYTE; 4],
    pub op: [*mut BYTE; 4],
    pub bits: [U64; 4],
    pub dt: *const libc::c_void,
    pub ilimit: *const BYTE,
    pub oend: *mut BYTE,
    pub iend: [*const BYTE; 4],
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/../common/bits.h\0" as *const u8
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
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/../common/bits.h\0" as *const u8
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/../common/bits.h\0" as *const u8
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
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
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
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: U16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as libc::c_int
            + ((*p.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as U16;
    };
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
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as size_t
    } else {
        return MEM_readLE64(memPtr)
    };
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return *(ptr as *const unalign16);
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
                current_block_32 = 4374783160472978455;
            }
            6 => {
                current_block_32 = 4374783160472978455;
            }
            5 => {
                current_block_32 = 508082133153388528;
            }
            4 => {
                current_block_32 = 11010276459331751112;
            }
            3 => {
                current_block_32 = 6819980346489144569;
            }
            2 => {
                current_block_32 = 8159937075156795181;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        match current_block_32 {
            4374783160472978455 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(5 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(24 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 508082133153388528;
            }
            _ => {}
        }
        match current_block_32 {
            508082133153388528 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(4 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(32 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 11010276459331751112;
            }
            _ => {}
        }
        match current_block_32 {
            11010276459331751112 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(3 as libc::c_int as isize)
                            as size_t) << 24 as libc::c_int,
                    ) as size_t as size_t;
                current_block_32 = 6819980346489144569;
            }
            _ => {}
        }
        match current_block_32 {
            6819980346489144569 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(2 as libc::c_int as isize)
                            as size_t) << 16 as libc::c_int,
                    ) as size_t as size_t;
                current_block_32 = 8159937075156795181;
            }
            _ => {}
        }
        match current_block_32 {
            8159937075156795181 => {
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
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
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
#[inline]
unsafe extern "C" fn BIT_endOfDStream(
    mut DStream: *const BIT_DStream_t,
) -> libc::c_uint {
    return ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as libc::c_ulong
            == (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)) as libc::c_int
        as libc::c_uint;
}
pub const HUF_TABLELOG_MAX: libc::c_int = 12 as libc::c_int;
pub const HUF_SYMBOLVALUE_MAX: libc::c_int = 255 as libc::c_int;
pub const HUF_DECODER_FAST_TABLELOG: libc::c_int = 11 as libc::c_int;
pub const HUF_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
unsafe extern "C" fn HUF_getDTableDesc(mut table: *const HUF_DTable) -> DTableDesc {
    let mut dtd = DTableDesc {
        maxTableLog: 0,
        tableType: 0,
        tableLog: 0,
        reserved: 0,
    };
    libc::memcpy(
        &mut dtd as *mut DTableDesc as *mut libc::c_void,
        table as *const libc::c_void,
        ::core::mem::size_of::<DTableDesc>() as libc::c_ulong as libc::size_t,
    );
    return dtd;
}
unsafe extern "C" fn HUF_initFastDStream(mut ip: *const BYTE) -> size_t {
    let lastByte = *ip.offset(7 as libc::c_int as isize);
    let bitsConsumed = (if lastByte as libc::c_int != 0 {
        (8 as libc::c_int as libc::c_uint).wrapping_sub(ZSTD_highbit32(lastByte as U32))
    } else {
        0 as libc::c_int as libc::c_uint
    }) as size_t;
    let value = MEM_readLEST(ip as *const libc::c_void)
        | 1 as libc::c_int as libc::c_ulong;
    if bitsConsumed <= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"bitsConsumed <= 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"size_t HUF_initFastDStream(const BYTE *)\0"))
                .as_ptr(),
        );
    }
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"sizeof(size_t) == 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"size_t HUF_initFastDStream(const BYTE *)\0"))
                .as_ptr(),
        );
    }
    return value << bitsConsumed;
}
unsafe extern "C" fn HUF_DecompressFastArgs_init(
    mut args: *mut HUF_DecompressFastArgs,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    let mut dt = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dtLog = (HUF_getDTableDesc(DTable)).tableLog as U32;
    let ilimit = (src as *const BYTE)
        .offset(6 as libc::c_int as isize)
        .offset(8 as libc::c_int as isize);
    let oend = (dst as *mut BYTE).offset(dstSize as isize);
    if MEM_isLittleEndian() == 0 || MEM_32bits() != 0 {
        return 0 as libc::c_int as size_t;
    }
    if srcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if dtLog != HUF_DECODER_FAST_TABLELOG as libc::c_uint {
        return 0 as libc::c_int as size_t;
    }
    let istart = src as *const BYTE;
    let length1 = MEM_readLE16(istart as *const libc::c_void) as size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length4 = srcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6 as libc::c_int as libc::c_ulong),
        );
    (*args).iend[0 as libc::c_int as usize] = istart.offset(6 as libc::c_int as isize);
    (*args)
        .iend[1 as libc::c_int
        as usize] = ((*args).iend[0 as libc::c_int as usize]).offset(length1 as isize);
    (*args)
        .iend[2 as libc::c_int
        as usize] = ((*args).iend[1 as libc::c_int as usize]).offset(length2 as isize);
    (*args)
        .iend[3 as libc::c_int
        as usize] = ((*args).iend[2 as libc::c_int as usize]).offset(length3 as isize);
    if length1 < 16 as libc::c_int as libc::c_ulong
        || length2 < 8 as libc::c_int as libc::c_ulong
        || length3 < 8 as libc::c_int as libc::c_ulong
        || length4 < 8 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    if length4 > srcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    (*args)
        .ip[0 as libc::c_int
        as usize] = ((*args).iend[1 as libc::c_int as usize])
        .offset(-(::core::mem::size_of::<U64>() as libc::c_ulong as isize));
    (*args)
        .ip[1 as libc::c_int
        as usize] = ((*args).iend[2 as libc::c_int as usize])
        .offset(-(::core::mem::size_of::<U64>() as libc::c_ulong as isize));
    (*args)
        .ip[2 as libc::c_int
        as usize] = ((*args).iend[3 as libc::c_int as usize])
        .offset(-(::core::mem::size_of::<U64>() as libc::c_ulong as isize));
    (*args)
        .ip[3 as libc::c_int
        as usize] = (src as *const BYTE)
        .offset(srcSize as isize)
        .offset(-(::core::mem::size_of::<U64>() as libc::c_ulong as isize));
    (*args).op[0 as libc::c_int as usize] = dst as *mut BYTE;
    (*args)
        .op[1 as libc::c_int
        as usize] = ((*args).op[0 as libc::c_int as usize])
        .offset(
            dstSize
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong) as isize,
        );
    (*args)
        .op[2 as libc::c_int
        as usize] = ((*args).op[1 as libc::c_int as usize])
        .offset(
            dstSize
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong) as isize,
        );
    (*args)
        .op[3 as libc::c_int
        as usize] = ((*args).op[2 as libc::c_int as usize])
        .offset(
            dstSize
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong) as isize,
        );
    if (*args).op[3 as libc::c_int as usize] >= oend {
        return 0 as libc::c_int as size_t;
    }
    (*args)
        .bits[0 as libc::c_int
        as usize] = HUF_initFastDStream((*args).ip[0 as libc::c_int as usize]);
    (*args)
        .bits[1 as libc::c_int
        as usize] = HUF_initFastDStream((*args).ip[1 as libc::c_int as usize]);
    (*args)
        .bits[2 as libc::c_int
        as usize] = HUF_initFastDStream((*args).ip[2 as libc::c_int as usize]);
    (*args)
        .bits[3 as libc::c_int
        as usize] = HUF_initFastDStream((*args).ip[3 as libc::c_int as usize]);
    (*args).ilimit = ilimit;
    (*args).oend = oend;
    (*args).dt = dt;
    return 1 as libc::c_int as size_t;
}
unsafe extern "C" fn HUF_initRemainingDStream(
    mut bit: *mut BIT_DStream_t,
    mut args: *const HUF_DecompressFastArgs,
    mut stream: libc::c_int,
    mut segmentEnd: *mut BYTE,
) -> size_t {
    if (*args).op[stream as usize] > segmentEnd {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if (*args).ip[stream as usize]
        < ((*args).iend[stream as usize]).offset(-(8 as libc::c_int as isize))
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"sizeof(size_t) == 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            285 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"size_t HUF_initRemainingDStream(BIT_DStream_t *, const HUF_DecompressFastArgs *, int, BYTE *)\0",
            ))
                .as_ptr(),
        );
    }
    (*bit)
        .bitContainer = MEM_readLEST((*args).ip[stream as usize] as *const libc::c_void);
    (*bit).bitsConsumed = ZSTD_countTrailingZeros64((*args).bits[stream as usize]);
    (*bit).start = (*args).iend[0 as libc::c_int as usize] as *const libc::c_char;
    (*bit)
        .limitPtr = ((*bit).start)
        .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
    (*bit).ptr = (*args).ip[stream as usize] as *const libc::c_char;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn HUF_DEltX1_set4(mut symbol: BYTE, mut nbBits: BYTE) -> U64 {
    let mut D4: U64 = 0;
    if MEM_isLittleEndian() != 0 {
        D4 = (((symbol as libc::c_int) << 8 as libc::c_int) + nbBits as libc::c_int)
            as U64;
    } else {
        D4 = (symbol as libc::c_int + ((nbBits as libc::c_int) << 8 as libc::c_int))
            as U64;
    }
    if D4 < ((1 as libc::c_uint) << 16 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"D4 < (1U << 16)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            314 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"U64 HUF_DEltX1_set4(BYTE, BYTE)\0"))
                .as_ptr(),
        );
    }
    D4 = (D4 as libc::c_ulonglong).wrapping_mul(0x1000100010001 as libc::c_ulonglong)
        as U64 as U64;
    return D4;
}
unsafe extern "C" fn HUF_rescaleStats(
    mut huffWeight: *mut BYTE,
    mut rankVal: *mut U32,
    mut nbSymbols: U32,
    mut tableLog: U32,
    mut targetTableLog: U32,
) -> U32 {
    if tableLog > targetTableLog {
        return tableLog;
    }
    if tableLog < targetTableLog {
        let scale = targetTableLog.wrapping_sub(tableLog);
        let mut s: U32 = 0;
        s = 0 as libc::c_int as U32;
        while s < nbSymbols {
            let ref mut fresh0 = *huffWeight.offset(s as isize);
            *fresh0 = (*fresh0 as libc::c_int
                + (if *huffWeight.offset(s as isize) as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    scale
                }) as BYTE as libc::c_int) as BYTE;
            s = s.wrapping_add(1);
        }
        s = targetTableLog;
        while s > scale {
            *rankVal
                .offset(s as isize) = *rankVal.offset(s.wrapping_sub(scale) as isize);
            s = s.wrapping_sub(1);
        }
        s = scale;
        while s > 0 as libc::c_int as libc::c_uint {
            *rankVal.offset(s as isize) = 0 as libc::c_int as U32;
            s = s.wrapping_sub(1);
        }
    }
    return targetTableLog;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX1_wksp(
    mut DTable: *mut HUF_DTable,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut tableLog = 0 as libc::c_int as U32;
    let mut nbSymbols = 0 as libc::c_int as U32;
    let mut iSize: size_t = 0;
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dt = dtPtr as *mut HUF_DEltX1;
    let mut wksp = workSpace as *mut HUF_ReadDTableX1_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX1_Workspace>() as libc::c_ulong > wkspSize {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    iSize = HUF_readStats_wksp(
        ((*wksp).huffWeight).as_mut_ptr(),
        (HUF_SYMBOLVALUE_MAX + 1 as libc::c_int) as size_t,
        ((*wksp).rankVal).as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
        ((*wksp).statsWksp).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[U32; 219]>() as libc::c_ulong,
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }
    let mut dtd = HUF_getDTableDesc(DTable);
    let maxTableLog = (dtd.maxTableLog as libc::c_int + 1 as libc::c_int) as U32;
    let targetTableLog = if maxTableLog < 11 as libc::c_int as libc::c_uint {
        maxTableLog
    } else {
        11 as libc::c_int as libc::c_uint
    };
    tableLog = HUF_rescaleStats(
        ((*wksp).huffWeight).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        nbSymbols,
        tableLog,
        targetTableLog,
    );
    if tableLog > (dtd.maxTableLog as libc::c_int + 1 as libc::c_int) as U32 {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    dtd.tableType = 0 as libc::c_int as BYTE;
    dtd.tableLog = tableLog as BYTE;
    libc::memcpy(
        DTable as *mut libc::c_void,
        &mut dtd as *mut DTableDesc as *const libc::c_void,
        ::core::mem::size_of::<DTableDesc>() as libc::c_ulong as libc::size_t,
    );
    let mut n: libc::c_int = 0;
    let mut nextRankStart = 0 as libc::c_int as U32;
    let unroll = 4 as libc::c_int;
    let nLimit = nbSymbols as libc::c_int - unroll + 1 as libc::c_int;
    n = 0 as libc::c_int;
    while n < tableLog as libc::c_int + 1 as libc::c_int {
        let curr = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add((*wksp).rankVal[n as usize]) as U32 as U32;
        (*wksp).rankStart[n as usize] = curr;
        n += 1;
    }
    n = 0 as libc::c_int;
    while n < nLimit {
        let mut u: libc::c_int = 0;
        u = 0 as libc::c_int;
        while u < unroll {
            let w = (*wksp).huffWeight[(n + u) as usize] as size_t;
            let fresh1 = (*wksp).rankStart[w as usize];
            (*wksp)
                .rankStart[w as usize] = ((*wksp).rankStart[w as usize]).wrapping_add(1);
            (*wksp).symbols[fresh1 as usize] = (n + u) as BYTE;
            u += 1;
        }
        n += unroll;
    }
    while n < nbSymbols as libc::c_int {
        let w_0 = (*wksp).huffWeight[n as usize] as size_t;
        let fresh2 = (*wksp).rankStart[w_0 as usize];
        (*wksp)
            .rankStart[w_0 as usize] = ((*wksp).rankStart[w_0 as usize]).wrapping_add(1);
        (*wksp).symbols[fresh2 as usize] = n as BYTE;
        n += 1;
    }
    let mut w_1: U32 = 0;
    let mut symbol = (*wksp).rankVal[0 as libc::c_int as usize] as libc::c_int;
    let mut rankStart = 0 as libc::c_int;
    w_1 = 1 as libc::c_int as U32;
    while w_1 < tableLog.wrapping_add(1 as libc::c_int as libc::c_uint) {
        let symbolCount = (*wksp).rankVal[w_1 as usize] as libc::c_int;
        let length = (1 as libc::c_int) << w_1 >> 1 as libc::c_int;
        let mut uStart = rankStart;
        let nbBits = tableLog
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(w_1) as BYTE;
        let mut s: libc::c_int = 0;
        let mut u_0: libc::c_int = 0;
        match length {
            1 => {
                s = 0 as libc::c_int;
                while s < symbolCount {
                    let mut D = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D.byte = (*wksp).symbols[(symbol + s) as usize];
                    D.nbBits = nbBits;
                    *dt.offset(uStart as isize) = D;
                    uStart += 1 as libc::c_int;
                    s += 1;
                }
            }
            2 => {
                s = 0 as libc::c_int;
                while s < symbolCount {
                    let mut D_0 = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D_0.byte = (*wksp).symbols[(symbol + s) as usize];
                    D_0.nbBits = nbBits;
                    *dt.offset((uStart + 0 as libc::c_int) as isize) = D_0;
                    *dt.offset((uStart + 1 as libc::c_int) as isize) = D_0;
                    uStart += 2 as libc::c_int;
                    s += 1;
                }
            }
            4 => {
                s = 0 as libc::c_int;
                while s < symbolCount {
                    let D4 = HUF_DEltX1_set4(
                        (*wksp).symbols[(symbol + s) as usize],
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut libc::c_void, D4);
                    uStart += 4 as libc::c_int;
                    s += 1;
                }
            }
            8 => {
                s = 0 as libc::c_int;
                while s < symbolCount {
                    let D4_0 = HUF_DEltX1_set4(
                        (*wksp).symbols[(symbol + s) as usize],
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut libc::c_void, D4_0);
                    MEM_write64(
                        dt.offset(uStart as isize).offset(4 as libc::c_int as isize)
                            as *mut libc::c_void,
                        D4_0,
                    );
                    uStart += 8 as libc::c_int;
                    s += 1;
                }
            }
            _ => {
                s = 0 as libc::c_int;
                while s < symbolCount {
                    let D4_1 = HUF_DEltX1_set4(
                        (*wksp).symbols[(symbol + s) as usize],
                        nbBits,
                    );
                    u_0 = 0 as libc::c_int;
                    while u_0 < length {
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(0 as libc::c_int as isize) as *mut libc::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(4 as libc::c_int as isize) as *mut libc::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(8 as libc::c_int as isize) as *mut libc::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(12 as libc::c_int as isize) as *mut libc::c_void,
                            D4_1,
                        );
                        u_0 += 16 as libc::c_int;
                    }
                    if u_0 == length {} else {
                        __assert_fail(
                            b"u == length\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                                as *const u8 as *const libc::c_char,
                            481 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 86],
                                &[libc::c_char; 86],
                            >(
                                b"size_t HUF_readDTableX1_wksp(HUF_DTable *, const void *, size_t, void *, size_t, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    uStart += length;
                    s += 1;
                }
            }
        }
        symbol += symbolCount;
        rankStart += symbolCount * length;
        w_1 = w_1.wrapping_add(1);
    }
    return iSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeSymbolX1(
    mut Dstream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX1,
    dtLog: U32,
) -> BYTE {
    let val = BIT_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BIT_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as U32);
    return c;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeStreamX1(
    mut p: *mut BYTE,
    bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut BYTE,
    dt: *const HUF_DEltX1,
    dtLog: U32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as libc::c_long > 3 as libc::c_int as libc::c_long {
        while (BIT_reloadDStream(bitDPtr) as libc::c_uint
            == BIT_DStream_unfinished as libc::c_int as libc::c_uint) as libc::c_int
            & (p < pEnd.offset(-(3 as libc::c_int as isize))) as libc::c_int != 0
        {
            if MEM_64bits() != 0 {
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            let fresh6 = p;
            p = p.offset(1);
            *fresh6 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
        }
    } else {
        BIT_reloadDStream(bitDPtr);
    }
    if MEM_32bits() != 0 {
        while (BIT_reloadDStream(bitDPtr) as libc::c_uint
            == BIT_DStream_unfinished as libc::c_int as libc::c_uint) as libc::c_int
            & (p < pEnd) as libc::c_int != 0
        {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
        }
    }
    while p < pEnd {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
    }
    return pEnd.offset_from(pStart) as libc::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_body(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    let mut op = dst as *mut BYTE;
    let oend = op.offset(dstSize as isize);
    let mut dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as U32;
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    HUF_decodeStreamX1(op, &mut bitD, oend, dt, dtLog);
    if BIT_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return dstSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_body(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    if cSrcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(-(3 as libc::c_int as isize));
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let length1 = MEM_readLE16(istart as *const libc::c_void) as size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6 as libc::c_int as libc::c_ulong),
        );
    let istart1 = istart.offset(6 as libc::c_int as isize);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(4 as libc::c_int as libc::c_ulong);
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as U32;
    let mut endSignal = 1 as libc::c_int as U32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if dstSize < 6 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let _var_err__ = BIT_initDStream(
        &mut bitD1,
        istart1 as *const libc::c_void,
        length1,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let _var_err___0 = BIT_initDStream(
        &mut bitD2,
        istart2 as *const libc::c_void,
        length2,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let _var_err___1 = BIT_initDStream(
        &mut bitD3,
        istart3 as *const libc::c_void,
        length3,
    );
    if ERR_isError(_var_err___1) != 0 {
        return _var_err___1;
    }
    let _var_err___2 = BIT_initDStream(
        &mut bitD4,
        istart4 as *const libc::c_void,
        length4,
    );
    if ERR_isError(_var_err___2) != 0 {
        return _var_err___2;
    }
    if oend.offset_from(op4) as libc::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
    {
        while endSignal & (op4 < olimit) as libc::c_int as libc::c_uint != 0 {
            if MEM_64bits() != 0 {
                let fresh9 = op1;
                op1 = op1.offset(1);
                *fresh9 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh10 = op2;
                op2 = op2.offset(1);
                *fresh10 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh11 = op3;
                op3 = op3.offset(1);
                *fresh11 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh12 = op4;
                op4 = op4.offset(1);
                *fresh12 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                let fresh13 = op1;
                op1 = op1.offset(1);
                *fresh13 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                let fresh14 = op2;
                op2 = op2.offset(1);
                *fresh14 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                let fresh15 = op3;
                op3 = op3.offset(1);
                *fresh15 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                let fresh16 = op4;
                op4 = op4.offset(1);
                *fresh16 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh17 = op1;
                op1 = op1.offset(1);
                *fresh17 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh18 = op2;
                op2 = op2.offset(1);
                *fresh18 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh19 = op3;
                op3 = op3.offset(1);
                *fresh19 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh20 = op4;
                op4 = op4.offset(1);
                *fresh20 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            let fresh21 = op1;
            op1 = op1.offset(1);
            *fresh21 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            let fresh22 = op2;
            op2 = op2.offset(1);
            *fresh22 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            let fresh23 = op3;
            op3 = op3.offset(1);
            *fresh23 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            let fresh24 = op4;
            op4 = op4.offset(1);
            *fresh24 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD1) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD2) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD3) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD4) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
        }
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    HUF_decodeStreamX1(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX1(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX1(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX1(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BIT_endOfDStream(&mut bitD1) & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3) & BIT_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_bmi2(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress4X1_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_default(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress4X1_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast_c_loop(
    mut args: *mut HUF_DecompressFastArgs,
) {
    let mut bits: [U64; 4] = [0; 4];
    let mut ip: [*const BYTE; 4] = [0 as *const BYTE; 4];
    let mut op: [*mut BYTE; 4] = [0 as *mut BYTE; 4];
    let dtable = (*args).dt as *const U16;
    let oend = (*args).oend;
    let ilimit = (*args).ilimit;
    libc::memcpy(
        &mut bits as *mut [U64; 4] as *mut libc::c_void,
        &mut (*args).bits as *mut [U64; 4] as *const libc::c_void,
        ::core::mem::size_of::<[U64; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut ip as *mut [*const BYTE; 4] as *mut libc::c_void,
        &mut (*args).ip as *mut [*const BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*const BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut op as *mut [*mut BYTE; 4] as *mut libc::c_void,
        &mut (*args).op as *mut [*mut BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*mut BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
    if MEM_isLittleEndian() != 0 {} else {
        __assert_fail(
            b"MEM_isLittleEndian()\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            702 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void HUF_decompress4X1_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
            ))
                .as_ptr(),
        );
    }
    if MEM_32bits() == 0 {} else {
        __assert_fail(
            b"!MEM_32bits()\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            703 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void HUF_decompress4X1_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
            ))
                .as_ptr(),
        );
    }
    's_33: loop {
        let mut olimit = 0 as *mut BYTE;
        let mut stream: libc::c_int = 0;
        let mut symbol: libc::c_int = 0;
        stream = 0 as libc::c_int;
        while stream < 4 as libc::c_int {
            if op[stream as usize]
                <= (if stream == 3 as libc::c_int {
                    oend
                } else {
                    op[(stream + 1 as libc::c_int) as usize]
                })
            {} else {
                __assert_fail(
                    b"op[stream] <= (stream == 3 ? oend : op[stream + 1])\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                        as *const u8 as *const libc::c_char,
                    713 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"void HUF_decompress4X1_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ip[stream as usize] >= ilimit {} else {
                __assert_fail(
                    b"ip[stream] >= ilimit\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                        as *const u8 as *const libc::c_char,
                    714 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"void HUF_decompress4X1_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
                    ))
                        .as_ptr(),
                );
            }
            stream += 1;
        }
        let oiters = (oend.offset_from(op[3 as libc::c_int as usize]) as libc::c_long
            as size_t)
            .wrapping_div(5 as libc::c_int as libc::c_ulong);
        let iiters = ((ip[0 as libc::c_int as usize]).offset_from(ilimit) as libc::c_long
            as size_t)
            .wrapping_div(7 as libc::c_int as libc::c_ulong);
        let iters = if oiters < iiters { oiters } else { iiters };
        let symbols = iters.wrapping_mul(5 as libc::c_int as libc::c_ulong);
        olimit = (op[3 as libc::c_int as usize]).offset(symbols as isize);
        if (op[3 as libc::c_int as usize]).offset(20 as libc::c_int as isize) > olimit {
            break;
        }
        stream = 1 as libc::c_int;
        while stream < 4 as libc::c_int {
            if ip[stream as usize] < ip[(stream - 1 as libc::c_int) as usize] {
                break 's_33;
            }
            stream += 1;
        }
        stream = 1 as libc::c_int;
        while stream < 4 as libc::c_int {
            if ip[stream as usize] >= ip[(stream - 1 as libc::c_int) as usize] {} else {
                __assert_fail(
                    b"ip[stream] >= ip[stream - 1]\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                        as *const u8 as *const libc::c_char,
                    751 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"void HUF_decompress4X1_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
                    ))
                        .as_ptr(),
                );
            }
            stream += 1;
        }
        loop {
            symbol = 0 as libc::c_int;
            while symbol < 5 as libc::c_int {
                stream = 0 as libc::c_int;
                while stream < 4 as libc::c_int {
                    let index = (bits[stream as usize] >> 53 as libc::c_int)
                        as libc::c_int;
                    let entry = *dtable.offset(index as isize) as libc::c_int;
                    bits[stream as usize] <<= entry & 63 as libc::c_int;
                    *(op[stream as usize])
                        .offset(
                            symbol as isize,
                        ) = (entry >> 8 as libc::c_int & 0xff as libc::c_int) as BYTE;
                    stream += 1;
                }
                symbol += 1;
            }
            stream = 0 as libc::c_int;
            while stream < 4 as libc::c_int {
                let ctz = ZSTD_countTrailingZeros64(bits[stream as usize])
                    as libc::c_int;
                let nbBits = ctz & 7 as libc::c_int;
                let nbBytes = ctz >> 3 as libc::c_int;
                op[stream
                    as usize] = (op[stream as usize]).offset(5 as libc::c_int as isize);
                ip[stream as usize] = (ip[stream as usize]).offset(-(nbBytes as isize));
                bits[stream
                    as usize] = MEM_read64(ip[stream as usize] as *const libc::c_void)
                    | 1 as libc::c_int as libc::c_ulong;
                bits[stream as usize] <<= nbBits;
                stream += 1;
            }
            if !(op[3 as libc::c_int as usize] < olimit) {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [U64; 4] as *mut libc::c_void,
        &mut bits as *mut [U64; 4] as *const libc::c_void,
        ::core::mem::size_of::<[U64; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const BYTE; 4] as *mut libc::c_void,
        &mut ip as *mut [*const BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*const BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut BYTE; 4] as *mut libc::c_void,
        &mut op as *mut [*mut BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*mut BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let mut dt = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let iend = (cSrc as *const BYTE).offset(6 as libc::c_int as isize);
    let oend = (dst as *mut BYTE).offset(dstSize as isize);
    let mut args = HUF_DecompressFastArgs {
        ip: [0 as *const BYTE; 4],
        op: [0 as *mut BYTE; 4],
        bits: [0; 4],
        dt: 0 as *const libc::c_void,
        ilimit: 0 as *const BYTE,
        oend: 0 as *mut BYTE,
        iend: [0 as *const BYTE; 4],
    };
    let ret = HUF_DecompressFastArgs_init(
        &mut args,
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
    let err_code = ret;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if ret == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    if args.ip[0 as libc::c_int as usize] >= args.ilimit {} else {
        __assert_fail(
            b"args.ip[0] >= args.ilimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            809 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    loopFn.expect("non-null function pointer")(&mut args);
    if args.ip[0 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[0] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            815 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.ip[1 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[1] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            816 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.ip[2 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[2] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            817 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.ip[3 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[3] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            818 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.op[3 as libc::c_int as usize] <= oend {} else {
        __assert_fail(
            b"args.op[3] <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            819 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    let segmentSize = dstSize
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(4 as libc::c_int as libc::c_ulong);
    let mut segmentEnd = dst as *mut BYTE;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut bit = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: 0 as *const libc::c_char,
            start: 0 as *const libc::c_char,
            limitPtr: 0 as *const libc::c_char,
        };
        if segmentSize <= oend.offset_from(segmentEnd) as libc::c_long as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }
        let err_code_0 = HUF_initRemainingDStream(&mut bit, &mut args, i, segmentEnd);
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        args
            .op[i
            as usize] = (args.op[i as usize])
            .offset(
                HUF_decodeStreamX1(
                    args.op[i as usize],
                    &mut bit,
                    segmentEnd,
                    dt as *const HUF_DEltX1,
                    HUF_DECODER_FAST_TABLELOG as U32,
                ) as isize,
            );
        if args.op[i as usize] != segmentEnd {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        i += 1;
    }
    if dstSize != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"dstSize != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            840 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X1_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: libc::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as libc::c_int != 0 {
        return HUF_decompress1X1_usingDTable_internal_bmi2(
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            DTable,
        );
    }
    return HUF_decompress1X1_usingDTable_internal_default(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_bmi2(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress1X1_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_default(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress1X1_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: libc::c_int,
) -> size_t {
    let mut fallbackFn: HUF_DecompressUsingDTableFn = Some(
        HUF_decompress4X1_usingDTable_internal_default
            as unsafe extern "C" fn(
                *mut libc::c_void,
                size_t,
                *const libc::c_void,
                size_t,
                *const HUF_DTable,
            ) -> size_t,
    );
    let mut loopFn: HUF_DecompressFastLoopFn = Some(
        HUF_decompress4X1_usingDTable_internal_fast_c_loop
            as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
    );
    if flags & HUF_flags_bmi2 as libc::c_int != 0 {
        fallbackFn = Some(
            HUF_decompress4X1_usingDTable_internal_bmi2
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    *const libc::c_void,
                    size_t,
                    *const HUF_DTable,
                ) -> size_t,
        );
        if flags & HUF_flags_disableAsm as libc::c_int == 0 {
            loopFn = Some(
                HUF_decompress4X1_usingDTable_internal_fast_asm_loop
                    as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
            );
        }
    } else {
        return fallbackFn
            .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable)
    }
    if flags & HUF_flags_disableFast as libc::c_int == 0 {
        let ret = HUF_decompress4X1_usingDTable_internal_fast(
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            DTable,
            loopFn,
        );
        if ret != 0 as libc::c_int as libc::c_ulong {
            return ret;
        }
    }
    return fallbackFn
        .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress4X1_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut ip = cSrc as *const BYTE;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as size_t;
    return HUF_decompress4X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dctx,
        flags,
    );
}
unsafe extern "C" fn HUF_buildDEltX2U32(
    mut symbol: U32,
    mut nbBits: U32,
    mut baseSeq: U32,
    mut level: libc::c_int,
) -> U32 {
    let mut seq: U32 = 0;
    if MEM_isLittleEndian() != 0 {
        seq = if level == 1 as libc::c_int {
            symbol
        } else {
            baseSeq.wrapping_add(symbol << 8 as libc::c_int)
        };
        return seq
            .wrapping_add(nbBits << 16 as libc::c_int)
            .wrapping_add((level as U32) << 24 as libc::c_int);
    } else {
        seq = if level == 1 as libc::c_int {
            symbol << 8 as libc::c_int
        } else {
            (baseSeq << 8 as libc::c_int).wrapping_add(symbol)
        };
        return (seq << 16 as libc::c_int)
            .wrapping_add(nbBits << 8 as libc::c_int)
            .wrapping_add(level as U32);
    };
}
unsafe extern "C" fn HUF_buildDEltX2(
    mut symbol: U32,
    mut nbBits: U32,
    mut baseSeq: U32,
    mut level: libc::c_int,
) -> HUF_DEltX2 {
    let mut DElt = HUF_DEltX2 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let val = HUF_buildDEltX2U32(symbol, nbBits, baseSeq, level);
    libc::memcpy(
        &mut DElt as *mut HUF_DEltX2 as *mut libc::c_void,
        &val as *const U32 as *const libc::c_void,
        ::core::mem::size_of::<U32>() as libc::c_ulong as libc::size_t,
    );
    return DElt;
}
unsafe extern "C" fn HUF_buildDEltX2U64(
    mut symbol: U32,
    mut nbBits: U32,
    mut baseSeq: U16,
    mut level: libc::c_int,
) -> U64 {
    let mut DElt = HUF_buildDEltX2U32(symbol, nbBits, baseSeq as U32, level);
    return (DElt as U64).wrapping_add((DElt as U64) << 32 as libc::c_int);
}
unsafe extern "C" fn HUF_fillDTableX2ForWeight(
    mut DTableRank: *mut HUF_DEltX2,
    mut begin: *const sortedSymbol_t,
    mut end: *const sortedSymbol_t,
    mut nbBits: U32,
    mut tableLog: U32,
    mut baseSeq: U16,
    level: libc::c_int,
) {
    let length = (1 as libc::c_uint)
        << (tableLog.wrapping_sub(nbBits) & 0x1f as libc::c_int as libc::c_uint);
    let mut ptr = 0 as *const sortedSymbol_t;
    if level >= 1 as libc::c_int && level <= 2 as libc::c_int {} else {
        __assert_fail(
            b"level >= 1 && level <= 2\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            967 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void HUF_fillDTableX2ForWeight(HUF_DEltX2 *, const sortedSymbol_t *, const sortedSymbol_t *, U32, U32, U16, const int)\0",
            ))
                .as_ptr(),
        );
    }
    match length {
        1 => {
            ptr = begin;
            while ptr != end {
                let DElt = HUF_buildDEltX2(
                    (*ptr).symbol as U32,
                    nbBits,
                    baseSeq as U32,
                    level,
                );
                let fresh25 = DTableRank;
                DTableRank = DTableRank.offset(1);
                *fresh25 = DElt;
                ptr = ptr.offset(1);
            }
        }
        2 => {
            ptr = begin;
            while ptr != end {
                let DElt_0 = HUF_buildDEltX2(
                    (*ptr).symbol as U32,
                    nbBits,
                    baseSeq as U32,
                    level,
                );
                *DTableRank.offset(0 as libc::c_int as isize) = DElt_0;
                *DTableRank.offset(1 as libc::c_int as isize) = DElt_0;
                DTableRank = DTableRank.offset(2 as libc::c_int as isize);
                ptr = ptr.offset(1);
            }
        }
        4 => {
            ptr = begin;
            while ptr != end {
                let DEltX2 = HUF_buildDEltX2U64(
                    (*ptr).symbol as U32,
                    nbBits,
                    baseSeq,
                    level,
                );
                libc::memcpy(
                    DTableRank.offset(0 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                DTableRank = DTableRank.offset(4 as libc::c_int as isize);
                ptr = ptr.offset(1);
            }
        }
        8 => {
            ptr = begin;
            while ptr != end {
                let DEltX2_0 = HUF_buildDEltX2U64(
                    (*ptr).symbol as U32,
                    nbBits,
                    baseSeq,
                    level,
                );
                libc::memcpy(
                    DTableRank.offset(0 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2_0 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2_0 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2_0 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(6 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2_0 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                DTableRank = DTableRank.offset(8 as libc::c_int as isize);
                ptr = ptr.offset(1);
            }
        }
        _ => {
            ptr = begin;
            while ptr != end {
                let DEltX2_1 = HUF_buildDEltX2U64(
                    (*ptr).symbol as U32,
                    nbBits,
                    baseSeq,
                    level,
                );
                let DTableRankEnd = DTableRank.offset(length as isize);
                while DTableRank != DTableRankEnd {
                    libc::memcpy(
                        DTableRank.offset(0 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2_1 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(2 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2_1 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(4 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2_1 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(6 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2_1 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    DTableRank = DTableRank.offset(8 as libc::c_int as isize);
                }
                ptr = ptr.offset(1);
            }
        }
    };
}
unsafe extern "C" fn HUF_fillDTableX2Level2(
    mut DTable: *mut HUF_DEltX2,
    mut targetLog: U32,
    consumedBits: U32,
    mut rankVal: *const U32,
    minWeight: libc::c_int,
    maxWeight1: libc::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    mut rankStart: *const U32,
    mut nbBitsBaseline: U32,
    mut baseSeq: U16,
) {
    if minWeight > 1 as libc::c_int {
        let length = (1 as libc::c_uint)
            << (targetLog.wrapping_sub(consumedBits)
                & 0x1f as libc::c_int as libc::c_uint);
        let DEltX2 = HUF_buildDEltX2U64(
            baseSeq as U32,
            consumedBits,
            0 as libc::c_int as U16,
            1 as libc::c_int,
        );
        let skipSize = *rankVal.offset(minWeight as isize) as libc::c_int;
        if length > 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"length > 1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                    as *const u8 as *const libc::c_char,
                1031 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void HUF_fillDTableX2Level2(HUF_DEltX2 *, U32, const U32, const U32 *, const int, const int, const sortedSymbol_t *, const U32 *, U32, U16)\0",
                ))
                    .as_ptr(),
            );
        }
        if (skipSize as U32) < length {} else {
            __assert_fail(
                b"(U32)skipSize < length\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                    as *const u8 as *const libc::c_char,
                1032 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void HUF_fillDTableX2Level2(HUF_DEltX2 *, U32, const U32, const U32 *, const int, const int, const sortedSymbol_t *, const U32 *, U32, U16)\0",
                ))
                    .as_ptr(),
            );
        }
        match length {
            2 => {
                if skipSize == 1 as libc::c_int {} else {
                    __assert_fail(
                        b"skipSize == 1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                            as *const u8 as *const libc::c_char,
                        1035 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 140],
                            &[libc::c_char; 140],
                        >(
                            b"void HUF_fillDTableX2Level2(HUF_DEltX2 *, U32, const U32, const U32 *, const int, const int, const sortedSymbol_t *, const U32 *, U32, U16)\0",
                        ))
                            .as_ptr(),
                    );
                }
                libc::memcpy(
                    DTable as *mut libc::c_void,
                    &DEltX2 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
            }
            4 => {
                if skipSize <= 4 as libc::c_int {} else {
                    __assert_fail(
                        b"skipSize <= 4\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                            as *const u8 as *const libc::c_char,
                        1039 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 140],
                            &[libc::c_char; 140],
                        >(
                            b"void HUF_fillDTableX2Level2(HUF_DEltX2 *, U32, const U32, const U32 *, const int, const int, const sortedSymbol_t *, const U32 *, U32, U16)\0",
                        ))
                            .as_ptr(),
                    );
                }
                libc::memcpy(
                    DTable.offset(0 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTable.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                    &DEltX2 as *const U64 as *const libc::c_void,
                    ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                );
            }
            _ => {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < skipSize {
                    libc::memcpy(
                        DTable.offset(i as isize).offset(0 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(2 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(4 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(6 as libc::c_int as isize)
                            as *mut libc::c_void,
                        &DEltX2 as *const U64 as *const libc::c_void,
                        ::core::mem::size_of::<U64>() as libc::c_ulong as libc::size_t,
                    );
                    i += 8 as libc::c_int;
                }
            }
        }
    }
    let mut w: libc::c_int = 0;
    w = minWeight;
    while w < maxWeight1 {
        let begin = *rankStart.offset(w as isize) as libc::c_int;
        let end = *rankStart.offset((w + 1 as libc::c_int) as isize) as libc::c_int;
        let nbBits = nbBitsBaseline.wrapping_sub(w as libc::c_uint);
        let totalBits = nbBits.wrapping_add(consumedBits);
        HUF_fillDTableX2ForWeight(
            DTable.offset(*rankVal.offset(w as isize) as isize),
            sortedSymbols.offset(begin as isize),
            sortedSymbols.offset(end as isize),
            totalBits,
            targetLog,
            baseSeq,
            2 as libc::c_int,
        );
        w += 1;
    }
}
unsafe extern "C" fn HUF_fillDTableX2(
    mut DTable: *mut HUF_DEltX2,
    targetLog: U32,
    mut sortedList: *const sortedSymbol_t,
    mut rankStart: *const U32,
    mut rankValOrigin: *mut rankValCol_t,
    maxWeight: U32,
    nbBitsBaseline: U32,
) {
    let rankVal = (*rankValOrigin.offset(0 as libc::c_int as isize)).as_mut_ptr();
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as libc::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut w: libc::c_int = 0;
    let wEnd = maxWeight as libc::c_int + 1 as libc::c_int;
    w = 1 as libc::c_int;
    while w < wEnd {
        let begin = *rankStart.offset(w as isize) as libc::c_int;
        let end = *rankStart.offset((w + 1 as libc::c_int) as isize) as libc::c_int;
        let nbBits = nbBitsBaseline.wrapping_sub(w as libc::c_uint);
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut start = *rankVal.offset(w as isize) as libc::c_int;
            let length = (1 as libc::c_uint)
                << (targetLog.wrapping_sub(nbBits)
                    & 0x1f as libc::c_int as libc::c_uint);
            let mut minWeight = nbBits.wrapping_add(scaleLog as libc::c_uint)
                as libc::c_int;
            let mut s: libc::c_int = 0;
            if minWeight < 1 as libc::c_int {
                minWeight = 1 as libc::c_int;
            }
            s = begin;
            while s != end {
                HUF_fillDTableX2Level2(
                    DTable.offset(start as isize),
                    targetLog,
                    nbBits,
                    (*rankValOrigin.offset(nbBits as isize)).as_mut_ptr(),
                    minWeight,
                    wEnd,
                    sortedList,
                    rankStart,
                    nbBitsBaseline,
                    (*sortedList.offset(s as isize)).symbol as U16,
                );
                start = (start as libc::c_uint).wrapping_add(length) as libc::c_int
                    as libc::c_int;
                s += 1;
            }
        } else {
            HUF_fillDTableX2ForWeight(
                DTable.offset(*rankVal.offset(w as isize) as isize),
                sortedList.offset(begin as isize),
                sortedList.offset(end as isize),
                nbBits,
                targetLog,
                0 as libc::c_int as U16,
                1 as libc::c_int,
            );
        }
        w += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX2_wksp(
    mut DTable: *mut HUF_DTable,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut tableLog: U32 = 0;
    let mut maxW: U32 = 0;
    let mut nbSymbols: U32 = 0;
    let mut dtd = HUF_getDTableDesc(DTable);
    let mut maxTableLog = dtd.maxTableLog as U32;
    let mut iSize: size_t = 0;
    let mut dtPtr = DTable.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dt = dtPtr as *mut HUF_DEltX2;
    let mut rankStart = 0 as *mut U32;
    let wksp = workSpace as *mut HUF_ReadDTableX2_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX2_Workspace>() as libc::c_ulong > wkspSize {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    rankStart = ((*wksp).rankStart0).as_mut_ptr().offset(1 as libc::c_int as isize);
    libc::memset(
        ((*wksp).rankStats).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[U32; 13]>() as libc::c_ulong as libc::size_t,
    );
    libc::memset(
        ((*wksp).rankStart0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[U32; 15]>() as libc::c_ulong as libc::size_t,
    );
    if maxTableLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    iSize = HUF_readStats_wksp(
        ((*wksp).weightList).as_mut_ptr(),
        (HUF_SYMBOLVALUE_MAX + 1 as libc::c_int) as size_t,
        ((*wksp).rankStats).as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
        ((*wksp).calleeWksp).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[U32; 219]>() as libc::c_ulong,
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > maxTableLog {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if tableLog <= HUF_DECODER_FAST_TABLELOG as libc::c_uint
        && maxTableLog > HUF_DECODER_FAST_TABLELOG as libc::c_uint
    {
        maxTableLog = HUF_DECODER_FAST_TABLELOG as U32;
    }
    maxW = tableLog;
    while (*wksp).rankStats[maxW as usize] == 0 as libc::c_int as libc::c_uint {
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: U32 = 0;
    let mut nextRankStart = 0 as libc::c_int as U32;
    w = 1 as libc::c_int as U32;
    while w < maxW.wrapping_add(1 as libc::c_int as libc::c_uint) {
        let mut curr = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add((*wksp).rankStats[w as usize]) as U32 as U32;
        *rankStart.offset(w as isize) = curr;
        w = w.wrapping_add(1);
    }
    *rankStart.offset(0 as libc::c_int as isize) = nextRankStart;
    *rankStart
        .offset(
            maxW.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = nextRankStart;
    let mut s: U32 = 0;
    s = 0 as libc::c_int as U32;
    while s < nbSymbols {
        let w_0 = (*wksp).weightList[s as usize] as U32;
        let ref mut fresh26 = *rankStart.offset(w_0 as isize);
        let fresh27 = *fresh26;
        *fresh26 = (*fresh26).wrapping_add(1);
        let r = fresh27;
        (*wksp).sortedSymbol[r as usize].symbol = s as BYTE;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0 as libc::c_int as isize) = 0 as libc::c_int as U32;
    let rankVal0 = ((*wksp).rankVal[0 as libc::c_int as usize]).as_mut_ptr();
    let rescale = maxTableLog
        .wrapping_sub(tableLog)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut nextRankVal = 0 as libc::c_int as U32;
    let mut w_1: U32 = 0;
    w_1 = 1 as libc::c_int as U32;
    while w_1 < maxW.wrapping_add(1 as libc::c_int as libc::c_uint) {
        let mut curr_0 = nextRankVal;
        nextRankVal = (nextRankVal as libc::c_uint)
            .wrapping_add(
                (*wksp).rankStats[w_1 as usize]
                    << w_1.wrapping_add(rescale as libc::c_uint),
            ) as U32 as U32;
        *rankVal0.offset(w_1 as isize) = curr_0;
        w_1 = w_1.wrapping_add(1);
    }
    let minBits = tableLog
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_sub(maxW);
    let mut consumed: U32 = 0;
    consumed = minBits;
    while consumed
        < maxTableLog
            .wrapping_sub(minBits)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        let rankValPtr = ((*wksp).rankVal[consumed as usize]).as_mut_ptr();
        let mut w_2: U32 = 0;
        w_2 = 1 as libc::c_int as U32;
        while w_2 < maxW.wrapping_add(1 as libc::c_int as libc::c_uint) {
            *rankValPtr
                .offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUF_fillDTableX2(
        dt,
        maxTableLog,
        ((*wksp).sortedSymbol).as_mut_ptr(),
        ((*wksp).rankStart0).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    dtd.tableLog = maxTableLog as BYTE;
    dtd.tableType = 1 as libc::c_int as BYTE;
    libc::memcpy(
        DTable as *mut libc::c_void,
        &mut dtd as *mut DTableDesc as *const libc::c_void,
        ::core::mem::size_of::<DTableDesc>() as libc::c_ulong as libc::size_t,
    );
    return iSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeSymbolX2(
    mut op: *mut libc::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: U32,
) -> U32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    libc::memcpy(
        op,
        &(*dt.offset(val as isize)).sequence as *const U16 as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong as libc::size_t,
    );
    BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    return (*dt.offset(val as isize)).length as U32;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeLastSymbolX2(
    mut op: *mut libc::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: U32,
) -> U32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    libc::memcpy(
        op,
        &(*dt.offset(val as isize)).sequence as *const U16 as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong as libc::size_t,
    );
    if (*dt.offset(val as isize)).length as libc::c_int == 1 as libc::c_int {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    } else if ((*DStream).bitsConsumed as libc::c_ulong)
        < (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
        if (*DStream).bitsConsumed as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            (*DStream)
                .bitsConsumed = (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_uint;
        }
    }
    return 1 as libc::c_int as U32;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeStreamX2(
    mut p: *mut BYTE,
    mut bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut BYTE,
    dt: *const HUF_DEltX2,
    dtLog: U32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as libc::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
    {
        if dtLog <= 11 as libc::c_int as libc::c_uint && MEM_64bits() != 0 {
            while (BIT_reloadDStream(bitDPtr) as libc::c_uint
                == BIT_DStream_unfinished as libc::c_int as libc::c_uint) as libc::c_int
                & (p < pEnd.offset(-(9 as libc::c_int as isize))) as libc::c_int != 0
            {
                p = p
                    .offset(
                        HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                p = p
                    .offset(
                        HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                p = p
                    .offset(
                        HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                p = p
                    .offset(
                        HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                p = p
                    .offset(
                        HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
            }
        } else {
            while (BIT_reloadDStream(bitDPtr) as libc::c_uint
                == BIT_DStream_unfinished as libc::c_int as libc::c_uint) as libc::c_int
                & (p
                    < pEnd
                        .offset(
                            -((::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                        )) as libc::c_int != 0
            {
                if MEM_64bits() != 0 {
                    p = p
                        .offset(
                            HUF_decodeSymbolX2(
                                p as *mut libc::c_void,
                                bitDPtr,
                                dt,
                                dtLog,
                            ) as isize,
                        );
                }
                if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                    p = p
                        .offset(
                            HUF_decodeSymbolX2(
                                p as *mut libc::c_void,
                                bitDPtr,
                                dt,
                                dtLog,
                            ) as isize,
                        );
                }
                if MEM_64bits() != 0 {
                    p = p
                        .offset(
                            HUF_decodeSymbolX2(
                                p as *mut libc::c_void,
                                bitDPtr,
                                dt,
                                dtLog,
                            ) as isize,
                        );
                }
                p = p
                    .offset(
                        HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
            }
        }
    } else {
        BIT_reloadDStream(bitDPtr);
    }
    if pEnd.offset_from(p) as libc::c_long as size_t >= 2 as libc::c_int as libc::c_ulong
    {
        while (BIT_reloadDStream(bitDPtr) as libc::c_uint
            == BIT_DStream_unfinished as libc::c_int as libc::c_uint) as libc::c_int
            & (p <= pEnd.offset(-(2 as libc::c_int as isize))) as libc::c_int != 0
        {
            p = p
                .offset(
                    HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        while p <= pEnd.offset(-(2 as libc::c_int as isize)) {
            p = p
                .offset(
                    HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
    }
    if p < pEnd {
        p = p
            .offset(
                HUF_decodeLastSymbolX2(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    return p.offset_from(pStart) as libc::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_body(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let dtd = HUF_getDTableDesc(DTable);
    HUF_decodeStreamX2(ostart, &mut bitD, oend, dt, dtd.tableLog as U32);
    if BIT_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return dstSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_body(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    if cSrcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend
        .offset(
            -((::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
        );
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let length1 = MEM_readLE16(istart as *const libc::c_void) as size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6 as libc::c_int as libc::c_ulong),
        );
    let istart1 = istart.offset(6 as libc::c_int as isize);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(4 as libc::c_int as libc::c_ulong);
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal = 1 as libc::c_int as U32;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as U32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if dstSize < 6 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let _var_err__ = BIT_initDStream(
        &mut bitD1,
        istart1 as *const libc::c_void,
        length1,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let _var_err___0 = BIT_initDStream(
        &mut bitD2,
        istart2 as *const libc::c_void,
        length2,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let _var_err___1 = BIT_initDStream(
        &mut bitD3,
        istart3 as *const libc::c_void,
        length3,
    );
    if ERR_isError(_var_err___1) != 0 {
        return _var_err___1;
    }
    let _var_err___2 = BIT_initDStream(
        &mut bitD4,
        istart4 as *const libc::c_void,
        length4,
    );
    if ERR_isError(_var_err___2) != 0 {
        return _var_err___2;
    }
    if oend.offset_from(op4) as libc::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
    {
        while endSignal & (op4 < olimit) as libc::c_int as libc::c_uint != 0 {
            if MEM_64bits() != 0 {
                op1 = op1
                    .offset(
                        HUF_decodeSymbolX2(
                            op1 as *mut libc::c_void,
                            &mut bitD1,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                op1 = op1
                    .offset(
                        HUF_decodeSymbolX2(
                            op1 as *mut libc::c_void,
                            &mut bitD1,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 {
                op1 = op1
                    .offset(
                        HUF_decodeSymbolX2(
                            op1 as *mut libc::c_void,
                            &mut bitD1,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            op1 = op1
                .offset(
                    HUF_decodeSymbolX2(op1 as *mut libc::c_void, &mut bitD1, dt, dtLog)
                        as isize,
                );
            if MEM_64bits() != 0 {
                op2 = op2
                    .offset(
                        HUF_decodeSymbolX2(
                            op2 as *mut libc::c_void,
                            &mut bitD2,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                op2 = op2
                    .offset(
                        HUF_decodeSymbolX2(
                            op2 as *mut libc::c_void,
                            &mut bitD2,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 {
                op2 = op2
                    .offset(
                        HUF_decodeSymbolX2(
                            op2 as *mut libc::c_void,
                            &mut bitD2,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            op2 = op2
                .offset(
                    HUF_decodeSymbolX2(op2 as *mut libc::c_void, &mut bitD2, dt, dtLog)
                        as isize,
                );
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD1) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD2) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
            if MEM_64bits() != 0 {
                op3 = op3
                    .offset(
                        HUF_decodeSymbolX2(
                            op3 as *mut libc::c_void,
                            &mut bitD3,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                op3 = op3
                    .offset(
                        HUF_decodeSymbolX2(
                            op3 as *mut libc::c_void,
                            &mut bitD3,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 {
                op3 = op3
                    .offset(
                        HUF_decodeSymbolX2(
                            op3 as *mut libc::c_void,
                            &mut bitD3,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            op3 = op3
                .offset(
                    HUF_decodeSymbolX2(op3 as *mut libc::c_void, &mut bitD3, dt, dtLog)
                        as isize,
                );
            if MEM_64bits() != 0 {
                op4 = op4
                    .offset(
                        HUF_decodeSymbolX2(
                            op4 as *mut libc::c_void,
                            &mut bitD4,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as libc::c_int {
                op4 = op4
                    .offset(
                        HUF_decodeSymbolX2(
                            op4 as *mut libc::c_void,
                            &mut bitD4,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            if MEM_64bits() != 0 {
                op4 = op4
                    .offset(
                        HUF_decodeSymbolX2(
                            op4 as *mut libc::c_void,
                            &mut bitD4,
                            dt,
                            dtLog,
                        ) as isize,
                    );
            }
            op4 = op4
                .offset(
                    HUF_decodeSymbolX2(op4 as *mut libc::c_void, &mut bitD4, dt, dtLog)
                        as isize,
                );
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD3) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD4) as libc::c_uint
                    == BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
        }
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    HUF_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BIT_endOfDStream(&mut bitD1) & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3) & BIT_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_bmi2(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress4X2_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_default(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress4X2_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast_c_loop(
    mut args: *mut HUF_DecompressFastArgs,
) {
    let mut bits: [U64; 4] = [0; 4];
    let mut ip: [*const BYTE; 4] = [0 as *const BYTE; 4];
    let mut op: [*mut BYTE; 4] = [0 as *mut BYTE; 4];
    let mut oend: [*mut BYTE; 4] = [0 as *mut BYTE; 4];
    let dtable = (*args).dt as *const HUF_DEltX2;
    let ilimit = (*args).ilimit;
    libc::memcpy(
        &mut bits as *mut [U64; 4] as *mut libc::c_void,
        &mut (*args).bits as *mut [U64; 4] as *const libc::c_void,
        ::core::mem::size_of::<[U64; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut ip as *mut [*const BYTE; 4] as *mut libc::c_void,
        &mut (*args).ip as *mut [*const BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*const BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut op as *mut [*mut BYTE; 4] as *mut libc::c_void,
        &mut (*args).op as *mut [*mut BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*mut BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
    oend[0 as libc::c_int as usize] = op[1 as libc::c_int as usize];
    oend[1 as libc::c_int as usize] = op[2 as libc::c_int as usize];
    oend[2 as libc::c_int as usize] = op[3 as libc::c_int as usize];
    oend[3 as libc::c_int as usize] = (*args).oend;
    if MEM_isLittleEndian() != 0 {} else {
        __assert_fail(
            b"MEM_isLittleEndian()\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1487 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void HUF_decompress4X2_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
            ))
                .as_ptr(),
        );
    }
    if MEM_32bits() == 0 {} else {
        __assert_fail(
            b"!MEM_32bits()\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1488 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void HUF_decompress4X2_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
            ))
                .as_ptr(),
        );
    }
    's_45: loop {
        let mut olimit = 0 as *mut BYTE;
        let mut stream: libc::c_int = 0;
        let mut symbol: libc::c_int = 0;
        stream = 0 as libc::c_int;
        while stream < 4 as libc::c_int {
            if op[stream as usize] <= oend[stream as usize] {} else {
                __assert_fail(
                    b"op[stream] <= oend[stream]\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                        as *const u8 as *const libc::c_char,
                    1498 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"void HUF_decompress4X2_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ip[stream as usize] >= ilimit {} else {
                __assert_fail(
                    b"ip[stream] >= ilimit\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                        as *const u8 as *const libc::c_char,
                    1499 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"void HUF_decompress4X2_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
                    ))
                        .as_ptr(),
                );
            }
            stream += 1;
        }
        let mut iters = ((ip[0 as libc::c_int as usize]).offset_from(ilimit)
            as libc::c_long as size_t)
            .wrapping_div(7 as libc::c_int as libc::c_ulong);
        stream = 0 as libc::c_int;
        while stream < 4 as libc::c_int {
            let oiters = ((oend[stream as usize]).offset_from(op[stream as usize])
                as libc::c_long as size_t)
                .wrapping_div(10 as libc::c_int as libc::c_ulong);
            iters = if iters < oiters { iters } else { oiters };
            stream += 1;
        }
        olimit = (op[3 as libc::c_int as usize])
            .offset(iters.wrapping_mul(5 as libc::c_int as libc::c_ulong) as isize);
        if (op[3 as libc::c_int as usize]).offset(10 as libc::c_int as isize) > olimit {
            break;
        }
        stream = 1 as libc::c_int;
        while stream < 4 as libc::c_int {
            if ip[stream as usize] < ip[(stream - 1 as libc::c_int) as usize] {
                break 's_45;
            }
            stream += 1;
        }
        stream = 1 as libc::c_int;
        while stream < 4 as libc::c_int {
            if ip[stream as usize] >= ip[(stream - 1 as libc::c_int) as usize] {} else {
                __assert_fail(
                    b"ip[stream] >= ip[stream - 1]\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0"
                        as *const u8 as *const libc::c_char,
                    1546 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"void HUF_decompress4X2_usingDTable_internal_fast_c_loop(HUF_DecompressFastArgs *)\0",
                    ))
                        .as_ptr(),
                );
            }
            stream += 1;
        }
        loop {
            symbol = 0 as libc::c_int;
            while symbol < 5 as libc::c_int {
                stream = 0 as libc::c_int;
                while stream < 3 as libc::c_int {
                    let index = (bits[stream as usize] >> 53 as libc::c_int)
                        as libc::c_int;
                    let entry = *dtable.offset(index as isize);
                    MEM_write16(
                        op[stream as usize] as *mut libc::c_void,
                        entry.sequence,
                    );
                    bits[stream as usize] <<= entry.nbBits as libc::c_int;
                    op[stream
                        as usize] = (op[stream as usize])
                        .offset(entry.length as libc::c_int as isize);
                    stream += 1;
                }
                symbol += 1;
            }
            let index_0 = (bits[3 as libc::c_int as usize] >> 53 as libc::c_int)
                as libc::c_int;
            let entry_0 = *dtable.offset(index_0 as isize);
            MEM_write16(
                op[3 as libc::c_int as usize] as *mut libc::c_void,
                entry_0.sequence,
            );
            bits[3 as libc::c_int as usize] <<= entry_0.nbBits as libc::c_int;
            op[3 as libc::c_int
                as usize] = (op[3 as libc::c_int as usize])
                .offset(entry_0.length as libc::c_int as isize);
            stream = 0 as libc::c_int;
            while stream < 4 as libc::c_int {
                let index_1 = (bits[3 as libc::c_int as usize] >> 53 as libc::c_int)
                    as libc::c_int;
                let entry_1 = *dtable.offset(index_1 as isize);
                MEM_write16(
                    op[3 as libc::c_int as usize] as *mut libc::c_void,
                    entry_1.sequence,
                );
                bits[3 as libc::c_int as usize] <<= entry_1.nbBits as libc::c_int;
                op[3 as libc::c_int
                    as usize] = (op[3 as libc::c_int as usize])
                    .offset(entry_1.length as libc::c_int as isize);
                let ctz = ZSTD_countTrailingZeros64(bits[stream as usize])
                    as libc::c_int;
                let nbBits = ctz & 7 as libc::c_int;
                let nbBytes = ctz >> 3 as libc::c_int;
                ip[stream as usize] = (ip[stream as usize]).offset(-(nbBytes as isize));
                bits[stream
                    as usize] = MEM_read64(ip[stream as usize] as *const libc::c_void)
                    | 1 as libc::c_int as libc::c_ulong;
                bits[stream as usize] <<= nbBits;
                stream += 1;
            }
            if !(op[3 as libc::c_int as usize] < olimit) {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [U64; 4] as *mut libc::c_void,
        &mut bits as *mut [U64; 4] as *const libc::c_void,
        ::core::mem::size_of::<[U64; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const BYTE; 4] as *mut libc::c_void,
        &mut ip as *mut [*const BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*const BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut BYTE; 4] as *mut libc::c_void,
        &mut op as *mut [*mut BYTE; 4] as *const libc::c_void,
        ::core::mem::size_of::<[*mut BYTE; 4]>() as libc::c_ulong as libc::size_t,
    );
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let mut dt = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let iend = (cSrc as *const BYTE).offset(6 as libc::c_int as isize);
    let oend = (dst as *mut BYTE).offset(dstSize as isize);
    let mut args = HUF_DecompressFastArgs {
        ip: [0 as *const BYTE; 4],
        op: [0 as *mut BYTE; 4],
        bits: [0; 4],
        dt: 0 as *const libc::c_void,
        ilimit: 0 as *const BYTE,
        oend: 0 as *mut BYTE,
        iend: [0 as *const BYTE; 4],
    };
    let ret = HUF_DecompressFastArgs_init(
        &mut args,
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
    let err_code = ret;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if ret == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    if args.ip[0 as libc::c_int as usize] >= args.ilimit {} else {
        __assert_fail(
            b"args.ip[0] >= args.ilimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1624 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X2_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    loopFn.expect("non-null function pointer")(&mut args);
    if args.ip[0 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[0] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1628 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X2_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.ip[1 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[1] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1629 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X2_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.ip[2 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[2] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1630 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X2_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.ip[3 as libc::c_int as usize] >= iend {} else {
        __assert_fail(
            b"args.ip[3] >= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1631 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X2_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    if args.op[3 as libc::c_int as usize] <= oend {} else {
        __assert_fail(
            b"args.op[3] <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1632 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"size_t HUF_decompress4X2_usingDTable_internal_fast(void *, size_t, const void *, size_t, const HUF_DTable *, HUF_DecompressFastLoopFn)\0",
            ))
                .as_ptr(),
        );
    }
    let segmentSize = dstSize
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(4 as libc::c_int as libc::c_ulong);
    let mut segmentEnd = dst as *mut BYTE;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut bit = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: 0 as *const libc::c_char,
            start: 0 as *const libc::c_char,
            limitPtr: 0 as *const libc::c_char,
        };
        if segmentSize <= oend.offset_from(segmentEnd) as libc::c_long as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }
        let err_code_0 = HUF_initRemainingDStream(&mut bit, &mut args, i, segmentEnd);
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        args
            .op[i
            as usize] = (args.op[i as usize])
            .offset(
                HUF_decodeStreamX2(
                    args.op[i as usize],
                    &mut bit,
                    segmentEnd,
                    dt as *const HUF_DEltX2,
                    HUF_DECODER_FAST_TABLELOG as U32,
                ) as isize,
            );
        if args.op[i as usize] != segmentEnd {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        i += 1;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: libc::c_int,
) -> size_t {
    let mut fallbackFn: HUF_DecompressUsingDTableFn = Some(
        HUF_decompress4X2_usingDTable_internal_default
            as unsafe extern "C" fn(
                *mut libc::c_void,
                size_t,
                *const libc::c_void,
                size_t,
                *const HUF_DTable,
            ) -> size_t,
    );
    let mut loopFn: HUF_DecompressFastLoopFn = Some(
        HUF_decompress4X2_usingDTable_internal_fast_c_loop
            as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
    );
    if flags & HUF_flags_bmi2 as libc::c_int != 0 {
        fallbackFn = Some(
            HUF_decompress4X2_usingDTable_internal_bmi2
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    *const libc::c_void,
                    size_t,
                    *const HUF_DTable,
                ) -> size_t,
        );
        if flags & HUF_flags_disableAsm as libc::c_int == 0 {
            loopFn = Some(
                HUF_decompress4X2_usingDTable_internal_fast_asm_loop
                    as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
            );
        }
    } else {
        return fallbackFn
            .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable)
    }
    if flags & HUF_flags_disableFast as libc::c_int == 0 {
        let ret = HUF_decompress4X2_usingDTable_internal_fast(
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            DTable,
            loopFn,
        );
        if ret != 0 as libc::c_int as libc::c_ulong {
            return ret;
        }
    }
    return fallbackFn
        .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: libc::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as libc::c_int != 0 {
        return HUF_decompress1X2_usingDTable_internal_bmi2(
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            DTable,
        );
    }
    return HUF_decompress1X2_usingDTable_internal_default(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_default(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress1X2_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_bmi2(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    return HUF_decompress1X2_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X2_DCtx_wksp(
    mut DCtx: *mut HUF_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut ip = cSrc as *const BYTE;
    let hSize = HUF_readDTableX2_wksp(DCtx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as size_t;
    return HUF_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        DCtx,
        flags,
    );
}
unsafe extern "C" fn HUF_decompress4X2_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut ip = cSrc as *const BYTE;
    let mut hSize = HUF_readDTableX2_wksp(
        dctx,
        cSrc,
        cSrcSize,
        workSpace,
        wkspSize,
        flags,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as size_t;
    return HUF_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dctx,
        flags,
    );
}
static mut algoTime: [[algo_time_t; 2]; 16] = [
    [
        {
            let mut init = algo_time_t {
                tableTime: 0 as libc::c_int as U32,
                decode256Time: 0 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1 as libc::c_int as U32,
                decode256Time: 1 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 0 as libc::c_int as U32,
                decode256Time: 0 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1 as libc::c_int as U32,
                decode256Time: 1 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 150 as libc::c_int as U32,
                decode256Time: 216 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 381 as libc::c_int as U32,
                decode256Time: 119 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 170 as libc::c_int as U32,
                decode256Time: 205 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 514 as libc::c_int as U32,
                decode256Time: 112 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 177 as libc::c_int as U32,
                decode256Time: 199 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 539 as libc::c_int as U32,
                decode256Time: 110 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 197 as libc::c_int as U32,
                decode256Time: 194 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 644 as libc::c_int as U32,
                decode256Time: 107 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 221 as libc::c_int as U32,
                decode256Time: 192 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 735 as libc::c_int as U32,
                decode256Time: 107 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 256 as libc::c_int as U32,
                decode256Time: 189 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 881 as libc::c_int as U32,
                decode256Time: 106 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 359 as libc::c_int as U32,
                decode256Time: 188 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1167 as libc::c_int as U32,
                decode256Time: 109 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 582 as libc::c_int as U32,
                decode256Time: 187 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1570 as libc::c_int as U32,
                decode256Time: 114 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 688 as libc::c_int as U32,
                decode256Time: 187 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1712 as libc::c_int as U32,
                decode256Time: 122 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 825 as libc::c_int as U32,
                decode256Time: 186 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1965 as libc::c_int as U32,
                decode256Time: 136 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 976 as libc::c_int as U32,
                decode256Time: 185 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2131 as libc::c_int as U32,
                decode256Time: 150 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1180 as libc::c_int as U32,
                decode256Time: 186 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2070 as libc::c_int as U32,
                decode256Time: 175 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1377 as libc::c_int as U32,
                decode256Time: 185 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1731 as libc::c_int as U32,
                decode256Time: 202 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1412 as libc::c_int as U32,
                decode256Time: 185 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1695 as libc::c_int as U32,
                decode256Time: 202 as libc::c_int as U32,
            };
            init
        },
    ],
];
#[no_mangle]
pub unsafe extern "C" fn HUF_selectDecoder(
    mut dstSize: size_t,
    mut cSrcSize: size_t,
) -> U32 {
    if dstSize > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"dstSize > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1761 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"U32 HUF_selectDecoder(size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if dstSize <= (128 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"dstSize <= 128*1024\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/huf_decompress.c\0" as *const u8
                as *const libc::c_char,
            1762 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"U32 HUF_selectDecoder(size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    let Q = if cSrcSize >= dstSize {
        15 as libc::c_int as libc::c_uint
    } else {
        cSrcSize.wrapping_mul(16 as libc::c_int as libc::c_ulong).wrapping_div(dstSize)
            as U32
    };
    let D256 = (dstSize >> 8 as libc::c_int) as U32;
    let DTime0 = (algoTime[Q as usize][0 as libc::c_int as usize].tableTime)
        .wrapping_add(
            (algoTime[Q as usize][0 as libc::c_int as usize].decode256Time)
                .wrapping_mul(D256),
        );
    let mut DTime1 = (algoTime[Q as usize][1 as libc::c_int as usize].tableTime)
        .wrapping_add(
            (algoTime[Q as usize][1 as libc::c_int as usize].decode256Time)
                .wrapping_mul(D256),
        );
    DTime1 = (DTime1 as libc::c_uint).wrapping_add(DTime1 >> 5 as libc::c_int) as U32
        as U32;
    return (DTime1 < DTime0) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        libc::memcpy(dst, cSrc, dstSize as libc::size_t);
        return dstSize;
    }
    if cSrcSize == 1 as libc::c_int as libc::c_ulong {
        libc::memset(
            dst,
            *(cSrc as *const BYTE) as libc::c_int,
            dstSize as libc::size_t,
        );
        return dstSize;
    }
    let algoNb = HUF_selectDecoder(dstSize, cSrcSize);
    return if algoNb != 0 {
        HUF_decompress1X2_DCtx_wksp(
            dctx,
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            workSpace,
            wkspSize,
            flags,
        )
    } else {
        HUF_decompress1X1_DCtx_wksp(
            dctx,
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            workSpace,
            wkspSize,
            flags,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_usingDTable(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: libc::c_int,
) -> size_t {
    let dtd = HUF_getDTableDesc(DTable);
    return if dtd.tableType as libc::c_int != 0 {
        HUF_decompress1X2_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
            flags,
        )
    } else {
        HUF_decompress1X1_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
            flags,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X1_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut ip = cSrc as *const BYTE;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as size_t;
    return HUF_decompress1X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dctx,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_usingDTable(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: libc::c_int,
) -> size_t {
    let dtd = HUF_getDTableDesc(DTable);
    return if dtd.tableType as libc::c_int != 0 {
        HUF_decompress4X2_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
            flags,
        )
    } else {
        HUF_decompress4X1_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
            flags,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_hufOnly_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if cSrcSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let algoNb = HUF_selectDecoder(dstSize, cSrcSize);
    return if algoNb != 0 {
        HUF_decompress4X2_DCtx_wksp(
            dctx,
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            workSpace,
            wkspSize,
            flags,
        )
    } else {
        HUF_decompress4X1_DCtx_wksp(
            dctx,
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            workSpace,
            wkspSize,
            flags,
        )
    };
}
