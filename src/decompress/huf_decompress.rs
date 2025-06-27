use ::libc;
extern "C" {
    fn HUF_readStats_wksp(
        huffWeight: *mut u8,
        hwSize: usize,
        rankStats: *mut u32,
        nbSymbolsPtr: *mut u32,
        tableLogPtr: *mut u32,
        src: *const std::ffi::c_void,
        srcSize: usize,
        workspace: *mut std::ffi::c_void,
        wkspSize: usize,
        flags: std::ffi::c_int,
    ) -> usize;
}
pub type ptrdiff_t = std::ffi::c_long;
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
pub type C2RustUnnamed = std::ffi::c_uint;
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
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed = 49;
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
pub type BitContainerType = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: BitContainerType,
    pub bitsConsumed: std::ffi::c_uint,
    pub ptr: *const std::ffi::c_char,
    pub start: *const std::ffi::c_char,
    pub limitPtr: *const std::ffi::c_char,
}
pub type BIT_DStream_status = std::ffi::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type HUF_DTable = u32;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: u32,
    pub decode256Time: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DTableDesc {
    pub maxTableLog: u8,
    pub tableType: u8,
    pub tableLog: u8,
    pub reserved: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX1 {
    pub nbBits: u8,
    pub byte: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_ReadDTableX1_Workspace {
    pub rankVal: [u32; 13],
    pub rankStart: [u32; 13],
    pub statsWksp: [u32; 219],
    pub symbols: [u8; 256],
    pub huffWeight: [u8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX2 {
    pub sequence: u16,
    pub nbBits: u8,
    pub length: u8,
}
pub type rankValCol_t = [u32; 13];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_ReadDTableX2_Workspace {
    pub rankVal: [rankValCol_t; 12],
    pub rankStats: [u32; 13],
    pub rankStart0: [u32; 15],
    pub sortedSymbol: [sortedSymbol_t; 256],
    pub weightList: [u8; 256],
    pub calleeWksp: [u32; 219],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
}
pub type HUF_DecompressUsingDTableFn = Option::<
    unsafe extern "C" fn(
        *mut std::ffi::c_void,
        usize,
        *const std::ffi::c_void,
        usize,
        *const HUF_DTable,
    ) -> usize,
>;
pub type HUF_DecompressFastLoopFn = Option::<
    unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DecompressFastArgs {
    pub ip: [*const u8; 4],
    pub op: [*mut u8; 4],
    pub bits: [u64; 4],
    pub dt: *const std::ffi::c_void,
    pub ilowest: *const u8,
    pub oend: *mut u8,
    pub iend: [*const u8; 4],
}
#[inline]
unsafe extern "C" fn ZSTD_maybeNullPtrAdd(
    mut ptr: *mut std::ffi::c_void,
    mut add: ptrdiff_t,
) -> *mut std::ffi::c_void {
    return if add > 0 {
        (ptr as *mut std::ffi::c_char).offset(add as isize) as *mut std::ffi::c_void
    } else {
        ptr
    };
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    return (::core::mem::size_of::<usize>()
        == 4) as std::ffi::c_int
        as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    return (::core::mem::size_of::<usize>()
        == 8) as std::ffi::c_int
        as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    return 1;
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const std::ffi::c_void) -> u16 {
    return *(ptr as *const unalign16);
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const std::ffi::c_void) -> u32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const std::ffi::c_void) -> u64 {
    return *(ptr as *const unalign64);
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut std::ffi::c_void, mut value: u16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut std::ffi::c_void, mut value: u64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const std::ffi::c_void) -> u16 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const u8;
        return (*p.offset(0) as std::ffi::c_int
            + ((*p.offset(1) as std::ffi::c_int)
                << 8)) as u16;
    };
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const std::ffi::c_void) -> u32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr)
    } else {
        return MEM_swap32(MEM_read32(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const std::ffi::c_void) -> u64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        return MEM_swap64(MEM_read64(memPtr))
    };
}
#[inline]
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const std::ffi::c_void) -> usize {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as usize
    } else {
        return MEM_readLE64(memPtr)
    };
}
unsafe extern "C" fn ERR_isError(mut code: usize) -> std::ffi::c_uint {
    return (code > ERROR!(maxCode)) as std::ffi::c_int as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const std::ffi::c_char,
    mut args: ...
) {}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    return val.leading_zeros() as i32 as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: u64) -> std::ffi::c_uint {
    return (val as std::ffi::c_ulonglong).trailing_zeros() as i32 as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    return (31 as std::ffi::c_uint)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
#[inline]
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    if srcSize < 1 {
        libc::memset(
            bitD as *mut std::ffi::c_void,
            0,
            ::core::mem::size_of::<BIT_DStream_t>() as usize,
        );
        return ERROR!(srcSize_wrong);
    }
    (*bitD).start = srcBuffer as *const std::ffi::c_char;
    (*bitD)
        .limitPtr = ((*bitD).start)
        .offset(
            ::core::mem::size_of::<BitContainerType>() as isize,
        );
    if srcSize >= ::core::mem::size_of::<BitContainerType>() {
        (*bitD)
            .ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(
                -(::core::mem::size_of::<BitContainerType>()
                    as isize),
            );
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        let lastByte = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte as std::ffi::c_int != 0 {
            (8 as std::ffi::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte as u32))
        } else {
            0 as std::ffi::c_uint
        };
        if lastByte as std::ffi::c_int == 0 {
            return ERROR!(GENERIC);
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as BitContainerType;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(6) as BitContainerType)
                            << (::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(16),
                    );
                current_block_32 = 1424294967110715298;
            }
            6 => {
                current_block_32 = 1424294967110715298;
            }
            5 => {
                current_block_32 = 710969857927062209;
            }
            4 => {
                current_block_32 = 3270920899165849433;
            }
            3 => {
                current_block_32 = 14109400006505043039;
            }
            2 => {
                current_block_32 = 740903896902006284;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        match current_block_32 {
            1424294967110715298 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(5) as BitContainerType)
                            << (::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(24),
                    );
                current_block_32 = 710969857927062209;
            }
            _ => {}
        }
        match current_block_32 {
            710969857927062209 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(4) as BitContainerType)
                            << (::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(32),
                    );
                current_block_32 = 3270920899165849433;
            }
            _ => {}
        }
        match current_block_32 {
            3270920899165849433 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(3) as BitContainerType)
                            << 24,
                    );
                current_block_32 = 14109400006505043039;
            }
            _ => {}
        }
        match current_block_32 {
            14109400006505043039 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(2) as BitContainerType)
                            << 16,
                    );
                current_block_32 = 740903896902006284;
            }
            _ => {}
        }
        match current_block_32 {
            740903896902006284 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(1) as BitContainerType)
                            << 8,
                    );
            }
            _ => {}
        }
        let lastByte_0 = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte_0 as std::ffi::c_int != 0 {
            (8 as std::ffi::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as u32))
        } else {
            0 as std::ffi::c_uint
        };
        if lastByte_0 as std::ffi::c_int == 0 {
            return ERROR!(corruption_detected);
        }
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                (::core::mem::size_of::<BitContainerType>())
                    .wrapping_sub(srcSize) as u32 * 8,
            );
    }
    return srcSize;
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask.wrapping_add(1).wrapping_sub(nbBits)
            & regMask);
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn BIT_reloadDStream_internal(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    (*bitD)
        .ptr = ((*bitD).ptr)
        .offset(-(((*bitD).bitsConsumed >> 3) as isize));
    (*bitD).bitsConsumed &= 7;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    return BIT_DStream_unfinished;
}
#[inline]
unsafe extern "C" fn BIT_reloadDStreamFast(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if UNLIKELY!(bitD -> ptr < bitD -> limitPtr) != 0 {
        return BIT_DStream_overflow;
    }
    return BIT_reloadDStream_internal(bitD);
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if ((*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<BitContainerType>())
            .wrapping_mul(8)) as std::ffi::c_int
        as std::ffi::c_long != 0
    {
        static mut zeroFilled: BitContainerType = 0 as std::ffi::c_int
            as BitContainerType;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as u32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD)
        .bitsConsumed = ((*bitD).bitsConsumed)
        .wrapping_sub(nbBytes * 8 as u32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    return result;
}
#[inline]
unsafe extern "C" fn BIT_endOfDStream(
    mut DStream: *const BIT_DStream_t,
) -> std::ffi::c_uint {
    return ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as std::ffi::c_ulong
            == (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8))
        as std::ffi::c_int as std::ffi::c_uint;
}
pub const HUF_TABLELOG_MAX: std::ffi::c_int = 12;
pub const HUF_SYMBOLVALUE_MAX: std::ffi::c_int = 255;
pub const HUF_DECODER_FAST_TABLELOG: std::ffi::c_int = 11;
pub const HUF_ENABLE_FAST_DECODE: std::ffi::c_int = 1;
pub const HUF_isError: unsafe extern "C" fn(usize) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn HUF_getDTableDesc(mut table: *const HUF_DTable) -> DTableDesc {
    let mut dtd = DTableDesc {
        maxTableLog: 0,
        tableType: 0,
        tableLog: 0,
        reserved: 0,
    };
    libc::memcpy(
        &mut dtd as *mut DTableDesc as *mut std::ffi::c_void,
        table as *const std::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as usize,
    );
    return dtd;
}
unsafe extern "C" fn HUF_initFastDStream(mut ip: *const u8) -> usize {
    let lastByte = *ip.offset(7);
    let bitsConsumed = (if lastByte as std::ffi::c_int != 0 {
        (8 as std::ffi::c_uint)
            .wrapping_sub(ZSTD_highbit32(lastByte as u32))
    } else {
        0 as std::ffi::c_uint
    }) as usize;
    let value = MEM_readLEST(ip as *const std::ffi::c_void)
        | 1;
    return value << bitsConsumed;
}
unsafe extern "C" fn HUF_DecompressFastArgs_init(
    mut args: *mut HUF_DecompressFastArgs,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    let mut dt = DTable.offset(1) as *const std::ffi::c_void;
    let dtLog = (HUF_getDTableDesc(DTable)).tableLog as u32;
    let istart = src as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    if MEM_isLittleEndian() == 0 || MEM_32bits() != 0 {
        return 0;
    }
    if dstSize == 0 {
        return 0;
    }
    if srcSize < 10 {
        return ERROR!(corruption_detected);
    }
    if dtLog != HUF_DECODER_FAST_TABLELOG as u32 {
        return 0;
    }
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as usize;
    let length2 = MEM_readLE16(
        istart.offset(2) as *const std::ffi::c_void,
    ) as usize;
    let length3 = MEM_readLE16(
        istart.offset(4) as *const std::ffi::c_void,
    ) as usize;
    let length4 = srcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6),
        );
    (*args)
        .iend[0] = istart.offset(6);
    (*args)
        .iend[1] = ((*args).iend[0])
        .offset(length1 as isize);
    (*args)
        .iend[2] = ((*args).iend[1])
        .offset(length2 as isize);
    (*args)
        .iend[3] = ((*args).iend[2])
        .offset(length3 as isize);
    if length1 < 8
        || length2 < 8
        || length3 < 8
        || length4 < 8
    {
        return 0;
    }
    if length4 > srcSize {
        return ERROR!(corruption_detected);
    }
    (*args)
        .ip[0] = ((*args).iend[1])
        .offset(-(::core::mem::size_of::<u64>() as isize));
    (*args)
        .ip[1] = ((*args).iend[2])
        .offset(-(::core::mem::size_of::<u64>() as isize));
    (*args)
        .ip[2] = ((*args).iend[3])
        .offset(-(::core::mem::size_of::<u64>() as isize));
    (*args)
        .ip[3] = (src as *const u8)
        .offset(srcSize as isize)
        .offset(-(::core::mem::size_of::<u64>() as isize));
    (*args).op[0] = dst as *mut u8;
    (*args)
        .op[1] = ((*args).op[0])
        .offset(
            (dstSize.wrapping_add(3)
                / 4 as usize) as isize,
        );
    (*args)
        .op[2] = ((*args).op[1])
        .offset(
            (dstSize.wrapping_add(3)
                / 4 as usize) as isize,
        );
    (*args)
        .op[3] = ((*args).op[2])
        .offset(
            (dstSize.wrapping_add(3)
                / 4 as usize) as isize,
        );
    if (*args).op[3] >= oend {
        return 0;
    }
    (*args)
        .bits[0] = HUF_initFastDStream((*args).ip[0]);
    (*args)
        .bits[1] = HUF_initFastDStream((*args).ip[1]);
    (*args)
        .bits[2] = HUF_initFastDStream((*args).ip[2]);
    (*args)
        .bits[3] = HUF_initFastDStream((*args).ip[3]);
    (*args).ilowest = istart;
    (*args).oend = oend;
    (*args).dt = dt;
    return 1;
}
unsafe extern "C" fn HUF_initRemainingDStream(
    mut bit: *mut BIT_DStream_t,
    mut args: *const HUF_DecompressFastArgs,
    mut stream: std::ffi::c_int,
    mut segmentEnd: *mut u8,
) -> usize {
    if (*args).op[stream as usize] > segmentEnd {
        return ERROR!(corruption_detected);
    }
    if (*args).ip[stream as usize]
        < ((*args).iend[stream as usize]).offset(-8_isize)
    {
        return ERROR!(corruption_detected);
    }
    (*bit)
        .bitContainer = MEM_readLEST(
        (*args).ip[stream as usize] as *const std::ffi::c_void,
    );
    (*bit).bitsConsumed = ZSTD_countTrailingZeros64((*args).bits[stream as usize]);
    (*bit).start = (*args).ilowest as *const std::ffi::c_char;
    (*bit)
        .limitPtr = ((*bit).start)
        .offset(::core::mem::size_of::<usize>() as isize);
    (*bit).ptr = (*args).ip[stream as usize] as *const std::ffi::c_char;
    return 0;
}
unsafe extern "C" fn HUF_DEltX1_set4(mut symbol: u8, mut nbBits: u8) -> u64 {
    let mut D4: u64 = 0;
    if MEM_isLittleEndian() != 0 {
        D4 = (((symbol as std::ffi::c_int) << 8)
            + nbBits as std::ffi::c_int) as u64;
    } else {
        D4 = (symbol as std::ffi::c_int
            + ((nbBits as std::ffi::c_int) << 8)) as u64;
    }
    D4 = (D4 as std::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as std::ffi::c_ulonglong) as u64 as u64;
    return D4;
}
unsafe extern "C" fn HUF_rescaleStats(
    mut huffWeight: *mut u8,
    mut rankVal: *mut u32,
    mut nbSymbols: u32,
    mut tableLog: u32,
    mut targetTableLog: u32,
) -> u32 {
    if tableLog > targetTableLog {
        return tableLog;
    }
    if tableLog < targetTableLog {
        let scale = targetTableLog.wrapping_sub(tableLog);
        let mut s: u32 = 0;
        s = 0;
        while s < nbSymbols {
            let ref mut fresh0 = *huffWeight.offset(s as isize);
            *fresh0 = (*fresh0 as std::ffi::c_int
                + (if *huffWeight.offset(s as isize) as std::ffi::c_int
                    == 0
                {
                    0 as u32
                } else {
                    scale
                }) as u8 as std::ffi::c_int) as u8;
            s = s.wrapping_add(1);
            s;
        }
        s = targetTableLog;
        while s > scale {
            *rankVal
                .offset(s as isize) = *rankVal.offset(s.wrapping_sub(scale) as isize);
            s = s.wrapping_sub(1);
            s;
        }
        s = scale;
        while s > 0 {
            *rankVal.offset(s as isize) = 0;
            s = s.wrapping_sub(1);
            s;
        }
    }
    return targetTableLog;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX1_wksp(
    mut DTable: *mut HUF_DTable,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut tableLog: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut iSize: usize = 0;
    let dtPtr = DTable.offset(1) as *mut std::ffi::c_void;
    let dt = dtPtr as *mut HUF_DEltX1;
    let mut wksp = workSpace as *mut HUF_ReadDTableX1_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX1_Workspace>()
        > wkspSize
    {
        return ERROR!(tableLog_tooLarge);
    }
    iSize = HUF_readStats_wksp(
        ((*wksp).huffWeight).as_mut_ptr(),
        (HUF_SYMBOLVALUE_MAX + 1 as std::ffi::c_int) as usize,
        ((*wksp).rankVal).as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
        ((*wksp).statsWksp).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 219]>(),
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }
    let mut dtd = HUF_getDTableDesc(DTable);
    let maxTableLog = (dtd.maxTableLog as std::ffi::c_int + 1 as std::ffi::c_int) as u32;
    let targetTableLog = MIN!(maxTableLog, HUF_DECODER_FAST_TABLELOG);
    tableLog = HUF_rescaleStats(
        ((*wksp).huffWeight).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        nbSymbols,
        tableLog,
        targetTableLog,
    );
    if tableLog > (dtd.maxTableLog as std::ffi::c_int + 1 as std::ffi::c_int) as u32 {
        return ERROR!(tableLog_tooLarge);
    }
    dtd.tableType = 0;
    dtd.tableLog = tableLog as u8;
    libc::memcpy(
        DTable as *mut std::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const std::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as usize,
    );
    let mut n: std::ffi::c_int = 0;
    let mut nextRankStart: u32 = 0;
    let unroll = 4;
    let nLimit = nbSymbols as std::ffi::c_int - unroll + 1;
    n = 0;
    while n < tableLog as std::ffi::c_int + 1 as std::ffi::c_int {
        let curr = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add((*wksp).rankVal[n as usize]);
        (*wksp).rankStart[n as usize] = curr;
        n += 1;
        n;
    }
    n = 0;
    while n < nLimit {
        let mut u: std::ffi::c_int = 0;
        u = 0;
        while u < unroll {
            let w = (*wksp).huffWeight[(n + u) as usize] as usize;
            let fresh1 = (*wksp).rankStart[w as usize];
            (*wksp)
                .rankStart[w as usize] = ((*wksp).rankStart[w as usize]).wrapping_add(1);
            (*wksp).symbols[fresh1 as usize] = (n + u) as u8;
            u += 1;
            u;
        }
        n += unroll;
    }
    while n < nbSymbols as std::ffi::c_int {
        let w_0 = (*wksp).huffWeight[n as usize] as usize;
        let fresh2 = (*wksp).rankStart[w_0 as usize];
        (*wksp)
            .rankStart[w_0 as usize] = ((*wksp).rankStart[w_0 as usize]).wrapping_add(1);
        (*wksp).symbols[fresh2 as usize] = n as u8;
        n += 1;
        n;
    }
    let mut w_1: u32 = 0;
    let mut symbol = (*wksp).rankVal[0] as std::ffi::c_int;
    let mut rankStart: std::ffi::c_int = 0;
    w_1 = 1;
    while w_1 < tableLog.wrapping_add(1) {
        let symbolCount = (*wksp).rankVal[w_1 as usize] as std::ffi::c_int;
        let length = (1 as std::ffi::c_int) << w_1 >> 1;
        let mut uStart = rankStart;
        let nbBits = tableLog.wrapping_add(1).wrapping_sub(w_1)
            as u8;
        let mut s: std::ffi::c_int = 0;
        let mut u_0: std::ffi::c_int = 0;
        match length {
            1 => {
                s = 0;
                while s < symbolCount {
                    let mut D = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D.byte = (*wksp).symbols[(symbol + s) as usize];
                    D.nbBits = nbBits;
                    *dt.offset(uStart as isize) = D;
                    uStart += 1;
                    s += 1;
                    s;
                }
            }
            2 => {
                s = 0;
                while s < symbolCount {
                    let mut D_0 = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D_0.byte = (*wksp).symbols[(symbol + s) as usize];
                    D_0.nbBits = nbBits;
                    *dt.offset((uStart + 0 as std::ffi::c_int) as isize) = D_0;
                    *dt.offset((uStart + 1 as std::ffi::c_int) as isize) = D_0;
                    uStart += 2;
                    s += 1;
                    s;
                }
            }
            4 => {
                s = 0;
                while s < symbolCount {
                    let D4 = HUF_DEltX1_set4(
                        (*wksp).symbols[(symbol + s) as usize],
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut std::ffi::c_void, D4);
                    uStart += 4;
                    s += 1;
                    s;
                }
            }
            8 => {
                s = 0;
                while s < symbolCount {
                    let D4_0 = HUF_DEltX1_set4(
                        (*wksp).symbols[(symbol + s) as usize],
                        nbBits,
                    );
                    MEM_write64(
                        dt.offset(uStart as isize) as *mut std::ffi::c_void,
                        D4_0,
                    );
                    MEM_write64(
                        dt.offset(uStart as isize).offset(4)
                            as *mut std::ffi::c_void,
                        D4_0,
                    );
                    uStart += 8;
                    s += 1;
                    s;
                }
            }
            _ => {
                s = 0;
                while s < symbolCount {
                    let D4_1 = HUF_DEltX1_set4(
                        (*wksp).symbols[(symbol + s) as usize],
                        nbBits,
                    );
                    u_0 = 0;
                    while u_0 < length {
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(0)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(4)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(8)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt
                                .offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(12)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        u_0 += 16;
                    }
                    uStart += length;
                    s += 1;
                    s;
                }
            }
        }
        symbol += symbolCount;
        rankStart += symbolCount * length;
        w_1 = w_1.wrapping_add(1);
        w_1;
    }
    return iSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeSymbolX1(
    mut Dstream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX1,
    dtLog: u32,
) -> u8 {
    let val = BIT_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BIT_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as u32);
    return c;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeStreamX1(
    mut p: *mut u8,
    bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: *const HUF_DEltX1,
    dtLog: u32,
) -> usize {
    let pStart = p;
    if pEnd.offset_from(p) as std::ffi::c_long > 3
    {
        while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
            == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int
            & (p < pEnd.offset(-3_isize)) as std::ffi::c_int != 0
        {
            if HUF_DECODE_SYMBOLX1_2!(p, bitDPtr) != 0 {
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_1!(p, bitDPtr) != 0 {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(p, bitDPtr) != 0 {
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            let ref mut fresh6 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
            *fresh6 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
        }
    } else {
        BIT_reloadDStream(bitDPtr);
    }
    if MEM_32bits() != 0 {
        while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
            == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int & (p < pEnd) as std::ffi::c_int != 0
        {
            let ref mut fresh7 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
            *fresh7 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
        }
    }
    while p < pEnd {
        let ref mut fresh8 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
        *fresh8 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
    }
    return pEnd.offset_from(pStart) as std::ffi::c_long as usize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    let mut op = dst as *mut u8;
    let oend = ZSTD_maybeNullPtrAdd(op as *mut std::ffi::c_void, dstSize as ptrdiff_t)
        as *mut u8;
    let mut dtPtr = DTable.offset(1)
        as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    HUF_decodeStreamX1(op, &mut bitD, oend, dt, dtLog);
    if BIT_endOfDStream(&mut bitD) == 0 {
        return ERROR!(corruption_detected);
    }
    return dstSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    if cSrcSize < 10 {
        return ERROR!(corruption_detected);
    }
    if dstSize < 6 {
        return ERROR!(corruption_detected);
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(-3_isize);
    let dtPtr = DTable.offset(1) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as usize;
    let length2 = MEM_readLE16(
        istart.offset(2) as *const std::ffi::c_void,
    ) as usize;
    let length3 = MEM_readLE16(
        istart.offset(4) as *const std::ffi::c_void,
    ) as usize;
    let length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6),
        );
    let istart1 = istart.offset(6);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize.wrapping_add(3)
        / 4;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let mut endSignal: u32 = 1;
    if length4 > cSrcSize {
        return ERROR!(corruption_detected);
    }
    if opStart4 > oend {
        return ERROR!(corruption_detected);
    }
    let _var_err__ = BIT_initDStream(
        &mut bitD1,
        istart1 as *const std::ffi::c_void,
        length1,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let _var_err___0 = BIT_initDStream(
        &mut bitD2,
        istart2 as *const std::ffi::c_void,
        length2,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let _var_err___1 = BIT_initDStream(
        &mut bitD3,
        istart3 as *const std::ffi::c_void,
        length3,
    );
    if ERR_isError(_var_err___1) != 0 {
        return _var_err___1;
    }
    let _var_err___2 = BIT_initDStream(
        &mut bitD4,
        istart4 as *const std::ffi::c_void,
        length4,
    );
    if ERR_isError(_var_err___2) != 0 {
        return _var_err___2;
    }
    if oend.offset_from(op4) as std::ffi::c_long as usize
        >= ::core::mem::size_of::<usize>()
    {
        while endSignal & (op4 < olimit) as std::ffi::c_int as u32 != 0 {
            if HUF_DECODE_SYMBOLX1_2!(op1, & bitD1) != 0 {
                let fresh9 = op1;
                op1 = op1.offset(1);
                *fresh9 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op2, & bitD2) != 0 {
                let fresh10 = op2;
                op2 = op2.offset(1);
                *fresh10 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op3, & bitD3) != 0 {
                let fresh11 = op3;
                op3 = op3.offset(1);
                *fresh11 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op4, & bitD4) != 0 {
                let fresh12 = op4;
                op4 = op4.offset(1);
                *fresh12 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_1!(op1, & bitD1) != 0 {
                let fresh13 = op1;
                op1 = op1.offset(1);
                *fresh13 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_1!(op2, & bitD2) != 0 {
                let fresh14 = op2;
                op2 = op2.offset(1);
                *fresh14 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_1!(op3, & bitD3) != 0 {
                let fresh15 = op3;
                op3 = op3.offset(1);
                *fresh15 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_1!(op4, & bitD4) != 0 {
                let fresh16 = op4;
                op4 = op4.offset(1);
                *fresh16 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op1, & bitD1) != 0 {
                let fresh17 = op1;
                op1 = op1.offset(1);
                *fresh17 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op2, & bitD2) != 0 {
                let fresh18 = op2;
                op2 = op2.offset(1);
                *fresh18 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op3, & bitD3) != 0 {
                let fresh19 = op3;
                op3 = op3.offset(1);
                *fresh19 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if HUF_DECODE_SYMBOLX1_2!(op4, & bitD4) != 0 {
                let fresh20 = op4;
                op4 = op4.offset(1);
                *fresh20 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            let ref mut fresh21 = HUF_DECODE_SYMBOLX1_0!(op1, & bitD1);
            *fresh21 = HUF_DECODE_SYMBOLX1_0!(op1, & bitD1);
            let ref mut fresh22 = HUF_DECODE_SYMBOLX1_0!(op2, & bitD2);
            *fresh22 = HUF_DECODE_SYMBOLX1_0!(op2, & bitD2);
            let ref mut fresh23 = HUF_DECODE_SYMBOLX1_0!(op3, & bitD3);
            *fresh23 = HUF_DECODE_SYMBOLX1_0!(op3, & bitD3);
            let ref mut fresh24 = HUF_DECODE_SYMBOLX1_0!(op4, & bitD4);
            *fresh24 = HUF_DECODE_SYMBOLX1_0!(op4, & bitD4);
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD1) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD2) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD3) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD4) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
        }
    }
    if op1 > opStart2 {
        return ERROR!(corruption_detected);
    }
    if op2 > opStart3 {
        return ERROR!(corruption_detected);
    }
    if op3 > opStart4 {
        return ERROR!(corruption_detected);
    }
    HUF_decodeStreamX1(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX1(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX1(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX1(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BIT_endOfDStream(&mut bitD1) & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3) & BIT_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return ERROR!(corruption_detected);
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    return HUF_decompress4X1_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
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
    let mut bits: [u64; 4] = [0; 4];
    let mut ip: [*const u8; 4] = [0 as *const u8; 4];
    let mut op: [*mut u8; 4] = [0 as *mut u8; 4];
    let dtable = (*args).dt as *const u16;
    let oend = (*args).oend;
    let ilowest = (*args).ilowest;
    libc::memcpy(
        &mut bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut (*args).bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as usize,
    );
    libc::memcpy(
        &mut ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut (*args).ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as usize,
    );
    libc::memcpy(
        &mut op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut (*args).op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as usize,
    );
    's_33: loop {
        let mut olimit = 0 as *mut u8;
        let mut stream: std::ffi::c_int = 0;
        stream = 0;
        while stream < 4 {
            stream += 1;
            stream;
        }
        let oiters = oend.offset_from(op[3])
            as std::ffi::c_long as usize / 5;
        let iiters = (ip[0]).offset_from(ilowest)
            as std::ffi::c_long as usize / 7;
        let iters = MIN!(oiters, iiters);
        let symbols = iters * 5;
        olimit = (op[3]).offset(symbols as isize);
        if op[3] == olimit {
            break;
        }
        stream = 1;
        while stream < 4 {
            if ip[stream as usize] < ip[(stream - 1 as std::ffi::c_int) as usize] {
                break 's_33;
            }
            stream += 1;
            stream;
        }
        stream = 1;
        while stream < 4 {
            stream += 1;
            stream;
        }
        loop {
            let index = (bits[0] >> 53)
                as std::ffi::c_int;
            let entry = *dtable.offset(index as isize) as std::ffi::c_int;
            bits[0] <<= entry & 0x3f as std::ffi::c_int;
            *(op[0])
                .offset(
                    0,
                ) = (entry >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_0 = (bits[1] >> 53)
                as std::ffi::c_int;
            let entry_0 = *dtable.offset(index_0 as isize) as std::ffi::c_int;
            bits[1] <<= entry_0 & 0x3f as std::ffi::c_int;
            *(op[1])
                .offset(
                    0,
                ) = (entry_0 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_1 = (bits[2] >> 53)
                as std::ffi::c_int;
            let entry_1 = *dtable.offset(index_1 as isize) as std::ffi::c_int;
            bits[2] <<= entry_1 & 0x3f as std::ffi::c_int;
            *(op[2])
                .offset(
                    0,
                ) = (entry_1 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_2 = (bits[3] >> 53)
                as std::ffi::c_int;
            let entry_2 = *dtable.offset(index_2 as isize) as std::ffi::c_int;
            bits[3] <<= entry_2 & 0x3f as std::ffi::c_int;
            *(op[3])
                .offset(
                    0,
                ) = (entry_2 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_3 = (bits[0] >> 53)
                as std::ffi::c_int;
            let entry_3 = *dtable.offset(index_3 as isize) as std::ffi::c_int;
            bits[0] <<= entry_3 & 0x3f as std::ffi::c_int;
            *(op[0])
                .offset(
                    1,
                ) = (entry_3 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_4 = (bits[1] >> 53)
                as std::ffi::c_int;
            let entry_4 = *dtable.offset(index_4 as isize) as std::ffi::c_int;
            bits[1] <<= entry_4 & 0x3f as std::ffi::c_int;
            *(op[1])
                .offset(
                    1,
                ) = (entry_4 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_5 = (bits[2] >> 53)
                as std::ffi::c_int;
            let entry_5 = *dtable.offset(index_5 as isize) as std::ffi::c_int;
            bits[2] <<= entry_5 & 0x3f as std::ffi::c_int;
            *(op[2])
                .offset(
                    1,
                ) = (entry_5 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_6 = (bits[3] >> 53)
                as std::ffi::c_int;
            let entry_6 = *dtable.offset(index_6 as isize) as std::ffi::c_int;
            bits[3] <<= entry_6 & 0x3f as std::ffi::c_int;
            *(op[3])
                .offset(
                    1,
                ) = (entry_6 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_7 = (bits[0] >> 53)
                as std::ffi::c_int;
            let entry_7 = *dtable.offset(index_7 as isize) as std::ffi::c_int;
            bits[0] <<= entry_7 & 0x3f as std::ffi::c_int;
            *(op[0])
                .offset(
                    2,
                ) = (entry_7 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_8 = (bits[1] >> 53)
                as std::ffi::c_int;
            let entry_8 = *dtable.offset(index_8 as isize) as std::ffi::c_int;
            bits[1] <<= entry_8 & 0x3f as std::ffi::c_int;
            *(op[1])
                .offset(
                    2,
                ) = (entry_8 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_9 = (bits[2] >> 53)
                as std::ffi::c_int;
            let entry_9 = *dtable.offset(index_9 as isize) as std::ffi::c_int;
            bits[2] <<= entry_9 & 0x3f as std::ffi::c_int;
            *(op[2])
                .offset(
                    2,
                ) = (entry_9 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_10 = (bits[3] >> 53)
                as std::ffi::c_int;
            let entry_10 = *dtable.offset(index_10 as isize) as std::ffi::c_int;
            bits[3] <<= entry_10 & 0x3f as std::ffi::c_int;
            *(op[3])
                .offset(
                    2,
                ) = (entry_10 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_11 = (bits[0] >> 53)
                as std::ffi::c_int;
            let entry_11 = *dtable.offset(index_11 as isize) as std::ffi::c_int;
            bits[0] <<= entry_11 & 0x3f as std::ffi::c_int;
            *(op[0])
                .offset(
                    3,
                ) = (entry_11 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_12 = (bits[1] >> 53)
                as std::ffi::c_int;
            let entry_12 = *dtable.offset(index_12 as isize) as std::ffi::c_int;
            bits[1] <<= entry_12 & 0x3f as std::ffi::c_int;
            *(op[1])
                .offset(
                    3,
                ) = (entry_12 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_13 = (bits[2] >> 53)
                as std::ffi::c_int;
            let entry_13 = *dtable.offset(index_13 as isize) as std::ffi::c_int;
            bits[2] <<= entry_13 & 0x3f as std::ffi::c_int;
            *(op[2])
                .offset(
                    3,
                ) = (entry_13 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_14 = (bits[3] >> 53)
                as std::ffi::c_int;
            let entry_14 = *dtable.offset(index_14 as isize) as std::ffi::c_int;
            bits[3] <<= entry_14 & 0x3f as std::ffi::c_int;
            *(op[3])
                .offset(
                    3,
                ) = (entry_14 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_15 = (bits[0] >> 53)
                as std::ffi::c_int;
            let entry_15 = *dtable.offset(index_15 as isize) as std::ffi::c_int;
            bits[0] <<= entry_15 & 0x3f as std::ffi::c_int;
            *(op[0])
                .offset(
                    4,
                ) = (entry_15 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_16 = (bits[1] >> 53)
                as std::ffi::c_int;
            let entry_16 = *dtable.offset(index_16 as isize) as std::ffi::c_int;
            bits[1] <<= entry_16 & 0x3f as std::ffi::c_int;
            *(op[1])
                .offset(
                    4,
                ) = (entry_16 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_17 = (bits[2] >> 53)
                as std::ffi::c_int;
            let entry_17 = *dtable.offset(index_17 as isize) as std::ffi::c_int;
            bits[2] <<= entry_17 & 0x3f as std::ffi::c_int;
            *(op[2])
                .offset(
                    4,
                ) = (entry_17 >> 8 & 0xff as std::ffi::c_int) as u8;
            let index_18 = (bits[3] >> 53)
                as std::ffi::c_int;
            let entry_18 = *dtable.offset(index_18 as isize) as std::ffi::c_int;
            bits[3] <<= entry_18 & 0x3f as std::ffi::c_int;
            *(op[3])
                .offset(
                    4,
                ) = (entry_18 >> 8 & 0xff as std::ffi::c_int) as u8;
            let ctz = ZSTD_countTrailingZeros64(bits[0])
                as std::ffi::c_int;
            let nbBits = ctz & 7;
            let nbBytes = ctz >> 3;
            op[0] = (op[0])
                .offset(5);
            ip[0] = (ip[0])
                .offset(-(nbBytes as isize));
            bits[0] = MEM_read64(
                ip[0] as *const std::ffi::c_void,
            ) | 1;
            bits[0] <<= nbBits;
            let ctz_0 = ZSTD_countTrailingZeros64(bits[1])
                as std::ffi::c_int;
            let nbBits_0 = ctz_0 & 7;
            let nbBytes_0 = ctz_0 >> 3;
            op[1] = (op[1])
                .offset(5);
            ip[1] = (ip[1])
                .offset(-(nbBytes_0 as isize));
            bits[1] = MEM_read64(
                ip[1] as *const std::ffi::c_void,
            ) | 1;
            bits[1] <<= nbBits_0;
            let ctz_1 = ZSTD_countTrailingZeros64(bits[2])
                as std::ffi::c_int;
            let nbBits_1 = ctz_1 & 7;
            let nbBytes_1 = ctz_1 >> 3;
            op[2] = (op[2])
                .offset(5);
            ip[2] = (ip[2])
                .offset(-(nbBytes_1 as isize));
            bits[2] = MEM_read64(
                ip[2] as *const std::ffi::c_void,
            ) | 1;
            bits[2] <<= nbBits_1;
            let ctz_2 = ZSTD_countTrailingZeros64(bits[3])
                as std::ffi::c_int;
            let nbBits_2 = ctz_2 & 7;
            let nbBytes_2 = ctz_2 >> 3;
            op[3] = (op[3])
                .offset(5);
            ip[3] = (ip[3])
                .offset(-(nbBytes_2 as isize));
            bits[3] = MEM_read64(
                ip[3] as *const std::ffi::c_void,
            ) | 1;
            bits[3] <<= nbBits_2;
            if !(op[3] < olimit) {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as usize,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as usize,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as usize,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut loopFn: HUF_DecompressFastLoopFn,
) -> usize {
    let mut dt = DTable.offset(1) as *const std::ffi::c_void;
    let ilowest = cSrc as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    let mut args = HUF_DecompressFastArgs {
        ip: [0 as *const u8; 4],
        op: [0 as *mut u8; 4],
        bits: [0; 4],
        dt: 0 as *const std::ffi::c_void,
        ilowest: 0 as *const u8,
        oend: 0 as *mut u8,
        iend: [0 as *const u8; 4],
    };
    let ret = HUF_DecompressFastArgs_init(
        &mut args,
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
    let err_code = FORWARD_IF_ERROR!(ret, "Failed to init fast loop args");
    if FORWARD_IF_ERROR!(ret, "Failed to init fast loop args") != 0 {
        return FORWARD_IF_ERROR!(ret, "Failed to init fast loop args");
    }
    if ret == 0 {
        return 0;
    }
    loopFn.expect("non-null function pointer")(&mut args);
    let segmentSize = dstSize.wrapping_add(3)
        / 4;
    let mut segmentEnd = dst as *mut u8;
    let mut i: std::ffi::c_int = 0;
    i = 0;
    while i < 4 {
        let mut bit = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: 0 as *const std::ffi::c_char,
            start: 0 as *const std::ffi::c_char,
            limitPtr: 0 as *const std::ffi::c_char,
        };
        if segmentSize <= oend.offset_from(segmentEnd) as std::ffi::c_long as usize {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }
        let err_code_0 = FORWARD_IF_ERROR!(
            HUF_initRemainingDStream(& bit, & args, i, segmentEnd), "corruption"
        );
        if FORWARD_IF_ERROR!(
            HUF_initRemainingDStream(& bit, & args, i, segmentEnd), "corruption"
        ) != 0
        {
            return FORWARD_IF_ERROR!(
                HUF_initRemainingDStream(& bit, & args, i, segmentEnd), "corruption"
            );
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
                    HUF_DECODER_FAST_TABLELOG as u32,
                ) as isize,
            );
        if args.op[i as usize] != segmentEnd {
            return ERROR!(corruption_detected);
        }
        i += 1;
        i;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    return HUF_decompress1X1_usingDTable_internal_body(
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
    );
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> usize {
    if HUF_DGEN!(HUF_decompress1X1_usingDTable_internal) != 0 {
        return HUF_decompress1X1_usingDTable_internal_bmi2(
            HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        );
    }
    return HUF_decompress1X1_usingDTable_internal_default(
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
    );
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    return HUF_decompress1X1_usingDTable_internal_body(
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X1_usingDTable_internal),
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut fallbackFn: HUF_DecompressUsingDTableFn = Some(
        HUF_decompress4X1_usingDTable_internal_default
            as unsafe extern "C" fn(
                *mut std::ffi::c_void,
                usize,
                *const std::ffi::c_void,
                usize,
                *const HUF_DTable,
            ) -> usize,
    );
    let mut loopFn: HUF_DecompressFastLoopFn = Some(
        HUF_decompress4X1_usingDTable_internal_fast_c_loop
            as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
    );
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        fallbackFn = Some(
            HUF_decompress4X1_usingDTable_internal_bmi2
                as unsafe extern "C" fn(
                    *mut std::ffi::c_void,
                    usize,
                    *const std::ffi::c_void,
                    usize,
                    *const HUF_DTable,
                ) -> usize,
        );
    } else {
        return fallbackFn
            .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable)
    }
    if HUF_ENABLE_FAST_DECODE != 0
        && flags & HUF_flags_disableFast as std::ffi::c_int == 0
    {
        let ret = HUF_decompress4X1_usingDTable_internal_fast(
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            DTable,
            loopFn,
        );
        if ret != 0 {
            return ret;
        }
    }
    return fallbackFn
        .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress4X1_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return ERROR!(srcSize_wrong);
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    return HUF_decompress4X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    );
}
unsafe extern "C" fn HUF_buildDEltX2U32(
    mut symbol: u32,
    mut nbBits: u32,
    mut baseSeq: u32,
    mut level: std::ffi::c_int,
) -> u32 {
    let mut seq: u32 = 0;
    if MEM_isLittleEndian() != 0 {
        seq = if level == 1 {
            symbol
        } else {
            baseSeq.wrapping_add(symbol << 8)
        };
        return seq
            .wrapping_add(nbBits << 16)
            .wrapping_add((level as u32) << 24);
    } else {
        seq = if level == 1 {
            symbol << 8
        } else {
            (baseSeq << 8).wrapping_add(symbol)
        };
        return (seq << 16)
            .wrapping_add(nbBits << 8)
            .wrapping_add(level as u32);
    };
}
unsafe extern "C" fn HUF_buildDEltX2(
    mut symbol: u32,
    mut nbBits: u32,
    mut baseSeq: u32,
    mut level: std::ffi::c_int,
) -> HUF_DEltX2 {
    let mut DElt = HUF_DEltX2 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let val = HUF_buildDEltX2U32(symbol, nbBits, baseSeq, level);
    libc::memcpy(
        &mut DElt as *mut HUF_DEltX2 as *mut std::ffi::c_void,
        &val as *const u32 as *const std::ffi::c_void,
        ::core::mem::size_of::<u32>() as usize,
    );
    return DElt;
}
unsafe extern "C" fn HUF_buildDEltX2U64(
    mut symbol: u32,
    mut nbBits: u32,
    mut baseSeq: u16,
    mut level: std::ffi::c_int,
) -> u64 {
    let mut DElt = HUF_buildDEltX2U32(symbol, nbBits, baseSeq as u32, level);
    return (DElt as u64).wrapping_add((DElt as u64) << 32);
}
unsafe extern "C" fn HUF_fillDTableX2ForWeight(
    mut DTableRank: *mut HUF_DEltX2,
    mut begin: *const sortedSymbol_t,
    mut end: *const sortedSymbol_t,
    mut nbBits: u32,
    mut tableLog: u32,
    mut baseSeq: u16,
    level: std::ffi::c_int,
) {
    let length = (1 as std::ffi::c_uint)
        << (tableLog.wrapping_sub(nbBits) & 0x1f as std::ffi::c_int as u32);
    let mut ptr = 0 as *const sortedSymbol_t;
    match length {
        1 => {
            ptr = begin;
            while ptr != end {
                let DElt = HUF_buildDEltX2(
                    (*ptr).symbol as u32,
                    nbBits,
                    baseSeq as u32,
                    level,
                );
                let fresh25 = DTableRank;
                DTableRank = DTableRank.offset(1);
                *fresh25 = DElt;
                ptr = ptr.offset(1);
                ptr;
            }
        }
        2 => {
            ptr = begin;
            while ptr != end {
                let DElt_0 = HUF_buildDEltX2(
                    (*ptr).symbol as u32,
                    nbBits,
                    baseSeq as u32,
                    level,
                );
                *DTableRank.offset(0) = DElt_0;
                *DTableRank.offset(1) = DElt_0;
                DTableRank = DTableRank.offset(2);
                ptr = ptr.offset(1);
                ptr;
            }
        }
        4 => {
            ptr = begin;
            while ptr != end {
                let DEltX2 = HUF_buildDEltX2U64(
                    (*ptr).symbol as u32,
                    nbBits,
                    baseSeq,
                    level,
                );
                libc::memcpy(
                    DTableRank.offset(0)
                        as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                libc::memcpy(
                    DTableRank.offset(2)
                        as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                DTableRank = DTableRank.offset(4);
                ptr = ptr.offset(1);
                ptr;
            }
        }
        8 => {
            ptr = begin;
            while ptr != end {
                let DEltX2_0 = HUF_buildDEltX2U64(
                    (*ptr).symbol as u32,
                    nbBits,
                    baseSeq,
                    level,
                );
                libc::memcpy(
                    DTableRank.offset(0)
                        as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                libc::memcpy(
                    DTableRank.offset(2)
                        as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                libc::memcpy(
                    DTableRank.offset(4)
                        as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                libc::memcpy(
                    DTableRank.offset(6)
                        as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                DTableRank = DTableRank.offset(8);
                ptr = ptr.offset(1);
                ptr;
            }
        }
        _ => {
            ptr = begin;
            while ptr != end {
                let DEltX2_1 = HUF_buildDEltX2U64(
                    (*ptr).symbol as u32,
                    nbBits,
                    baseSeq,
                    level,
                );
                let DTableRankEnd = DTableRank.offset(length as isize);
                while DTableRank != DTableRankEnd {
                    libc::memcpy(
                        DTableRank.offset(0)
                            as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    libc::memcpy(
                        DTableRank.offset(2)
                            as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    libc::memcpy(
                        DTableRank.offset(4)
                            as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    libc::memcpy(
                        DTableRank.offset(6)
                            as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    DTableRank = DTableRank.offset(8);
                }
                ptr = ptr.offset(1);
                ptr;
            }
        }
    };
}
unsafe extern "C" fn HUF_fillDTableX2Level2(
    mut DTable: *mut HUF_DEltX2,
    mut targetLog: u32,
    consumedBits: u32,
    mut rankVal: *const u32,
    minWeight: std::ffi::c_int,
    maxWeight1: std::ffi::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    mut rankStart: *const u32,
    mut nbBitsBaseline: u32,
    mut baseSeq: u16,
) {
    if minWeight > 1 {
        let length = (1 as std::ffi::c_uint)
            << (targetLog.wrapping_sub(consumedBits) & 0x1f as std::ffi::c_int as u32);
        let DEltX2 = HUF_buildDEltX2U64(
            baseSeq as u32,
            consumedBits,
            0,
            1,
        );
        let skipSize = *rankVal.offset(minWeight as isize) as std::ffi::c_int;
        match length {
            2 => {
                libc::memcpy(
                    DTable as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
            }
            4 => {
                libc::memcpy(
                    DTable.offset(0)
                        as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
                libc::memcpy(
                    DTable.offset(2)
                        as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as usize,
                );
            }
            _ => {
                let mut i: std::ffi::c_int = 0;
                i = 0;
                while i < skipSize {
                    libc::memcpy(
                        DTable.offset(i as isize).offset(0)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(2)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(4)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(6)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>()
                            as usize,
                    );
                    i += 8;
                }
            }
        }
    }
    let mut w: std::ffi::c_int = 0;
    w = minWeight;
    while w < maxWeight1 {
        let begin = *rankStart.offset(w as isize) as std::ffi::c_int;
        let end = *rankStart.offset((w + 1 as std::ffi::c_int) as isize)
            as std::ffi::c_int;
        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        let totalBits = nbBits.wrapping_add(consumedBits);
        HUF_fillDTableX2ForWeight(
            DTable.offset(*rankVal.offset(w as isize) as isize),
            sortedSymbols.offset(begin as isize),
            sortedSymbols.offset(end as isize),
            totalBits,
            targetLog,
            baseSeq,
            2,
        );
        w += 1;
        w;
    }
}
unsafe extern "C" fn HUF_fillDTableX2(
    mut DTable: *mut HUF_DEltX2,
    targetLog: u32,
    mut sortedList: *const sortedSymbol_t,
    mut rankStart: *const u32,
    mut rankValOrigin: *mut rankValCol_t,
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let rankVal = (*rankValOrigin.offset(0)).as_mut_ptr();
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as std::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut w: std::ffi::c_int = 0;
    let wEnd = maxWeight as std::ffi::c_int + 1;
    w = 1;
    while w < wEnd {
        let begin = *rankStart.offset(w as isize) as std::ffi::c_int;
        let end = *rankStart.offset((w + 1 as std::ffi::c_int) as isize)
            as std::ffi::c_int;
        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut start = *rankVal.offset(w as isize) as std::ffi::c_int;
            let length = (1 as std::ffi::c_uint)
                << (targetLog.wrapping_sub(nbBits) & 0x1f as std::ffi::c_int as u32);
            let mut minWeight = nbBits.wrapping_add(scaleLog as u32) as std::ffi::c_int;
            let mut s: std::ffi::c_int = 0;
            if minWeight < 1 {
                minWeight = 1;
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
                    (*sortedList.offset(s as isize)).symbol as u16,
                );
                start = (start as u32).wrapping_add(length) as std::ffi::c_int
                    as std::ffi::c_int;
                s += 1;
                s;
            }
        } else {
            HUF_fillDTableX2ForWeight(
                DTable.offset(*rankVal.offset(w as isize) as isize),
                sortedList.offset(begin as isize),
                sortedList.offset(end as isize),
                nbBits,
                targetLog,
                0,
                1,
            );
        }
        w += 1;
        w;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX2_wksp(
    mut DTable: *mut HUF_DTable,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut tableLog: u32 = 0;
    let mut maxW: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut dtd = HUF_getDTableDesc(DTable);
    let mut maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: usize = 0;
    let mut dtPtr = DTable.offset(1)
        as *mut std::ffi::c_void;
    let dt = dtPtr as *mut HUF_DEltX2;
    let mut rankStart = 0 as *mut u32;
    let wksp = workSpace as *mut HUF_ReadDTableX2_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX2_Workspace>()
        > wkspSize
    {
        return ERROR!(GENERIC);
    }
    rankStart = ((*wksp).rankStart0).as_mut_ptr().offset(1);
    libc::memset(
        ((*wksp).rankStats).as_mut_ptr() as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<[u32; 13]>() as usize,
    );
    libc::memset(
        ((*wksp).rankStart0).as_mut_ptr() as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<[u32; 15]>() as usize,
    );
    if maxTableLog > HUF_TABLELOG_MAX as u32 {
        return ERROR!(tableLog_tooLarge);
    }
    iSize = HUF_readStats_wksp(
        ((*wksp).weightList).as_mut_ptr(),
        (HUF_SYMBOLVALUE_MAX + 1 as std::ffi::c_int) as usize,
        ((*wksp).rankStats).as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
        ((*wksp).calleeWksp).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 219]>(),
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > maxTableLog {
        return ERROR!(tableLog_tooLarge);
    }
    if tableLog <= HUF_DECODER_FAST_TABLELOG as u32
        && maxTableLog > HUF_DECODER_FAST_TABLELOG as u32
    {
        maxTableLog = HUF_DECODER_FAST_TABLELOG as u32;
    }
    maxW = tableLog;
    while (*wksp).rankStats[maxW as usize] == 0 {
        maxW = maxW.wrapping_sub(1);
        maxW;
    }
    let mut w: u32 = 0;
    let mut nextRankStart: u32 = 0;
    w = 1;
    while w < maxW.wrapping_add(1) {
        let mut curr = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add((*wksp).rankStats[w as usize]);
        *rankStart.offset(w as isize) = curr;
        w = w.wrapping_add(1);
        w;
    }
    *rankStart.offset(0) = nextRankStart;
    *rankStart
        .offset(maxW.wrapping_add(1) as isize) = nextRankStart;
    let mut s: u32 = 0;
    s = 0;
    while s < nbSymbols {
        let w_0 = (*wksp).weightList[s as usize] as u32;
        let ref mut fresh26 = *rankStart.offset(w_0 as isize);
        let fresh27 = *fresh26;
        *fresh26 = (*fresh26).wrapping_add(1);
        let r = fresh27;
        (*wksp).sortedSymbol[r as usize].symbol = s as u8;
        s = s.wrapping_add(1);
        s;
    }
    *rankStart.offset(0) = 0;
    let rankVal0 = ((*wksp).rankVal[0]).as_mut_ptr();
    let rescale = maxTableLog
        .wrapping_sub(tableLog)
        .wrapping_sub(1) as std::ffi::c_int;
    let mut nextRankVal: u32 = 0;
    let mut w_1: u32 = 0;
    w_1 = 1;
    while w_1 < maxW.wrapping_add(1) {
        let mut curr_0 = nextRankVal;
        nextRankVal = nextRankVal
            .wrapping_add(
                (*wksp).rankStats[w_1 as usize] << w_1.wrapping_add(rescale as u32),
            );
        *rankVal0.offset(w_1 as isize) = curr_0;
        w_1 = w_1.wrapping_add(1);
        w_1;
    }
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed
        < maxTableLog.wrapping_sub(minBits).wrapping_add(1)
    {
        let rankValPtr = ((*wksp).rankVal[consumed as usize]).as_mut_ptr();
        let mut w_2: u32 = 0;
        w_2 = 1;
        while w_2 < maxW.wrapping_add(1) {
            *rankValPtr
                .offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
            w_2;
        }
        consumed = consumed.wrapping_add(1);
        consumed;
    }
    HUF_fillDTableX2(
        dt,
        maxTableLog,
        ((*wksp).sortedSymbol).as_mut_ptr(),
        ((*wksp).rankStart0).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1),
    );
    dtd.tableLog = maxTableLog as u8;
    dtd.tableType = 1;
    libc::memcpy(
        DTable as *mut std::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const std::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as usize,
    );
    return iSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeSymbolX2(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: u32,
) -> u32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    libc::memcpy(
        ZSTD_memcpy!(op, & dt[val].sequence, 2),
        ZSTD_memcpy!(op, & dt[val].sequence, 2),
        ZSTD_memcpy!(op, & dt[val].sequence, 2) as usize,
    );
    BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    return (*dt.offset(val as isize)).length as u32;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeLastSymbolX2(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: u32,
) -> u32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    libc::memcpy(
        ZSTD_memcpy!(op, & dt[val].sequence, 1),
        ZSTD_memcpy!(op, & dt[val].sequence, 1),
        ZSTD_memcpy!(op, & dt[val].sequence, 1) as usize,
    );
    if (*dt.offset(val as isize)).length as std::ffi::c_int == 1 {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as std::ffi::c_ulong)
        < (::core::mem::size_of::<BitContainerType>())
            .wrapping_mul(8)
    {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
        if (*DStream).bitsConsumed as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8)
        {
            (*DStream)
                .bitsConsumed = (::core::mem::size_of::<BitContainerType>()
                as std::ffi::c_ulong)
                .wrapping_mul(8)
                as std::ffi::c_uint;
        }
    }
    return 1;
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeStreamX2(
    mut p: *mut u8,
    mut bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: *const HUF_DEltX2,
    dtLog: u32,
) -> usize {
    let pStart = p;
    if pEnd.offset_from(p) as std::ffi::c_long as usize
        >= ::core::mem::size_of::<BitContainerType>()
    {
        if dtLog <= 11 && MEM_64bits() != 0 {
            while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int
                & (p < pEnd.offset(-9_isize)) as std::ffi::c_int
                != 0
            {
                p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
                p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
                p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
                p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
                p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
            }
        } else {
            while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int
                & (p
                    < pEnd
                        .offset(
                            -((::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_sub(1)
                                as isize),
                        )) as std::ffi::c_int != 0
            {
                if HUF_DECODE_SYMBOLX2_2!(p, bitDPtr) != 0 {
                    p = p.offset(HUF_DECODE_SYMBOLX2_2!(p, bitDPtr) as isize);
                }
                if HUF_DECODE_SYMBOLX2_1!(p, bitDPtr) != 0 {
                    p = p.offset(HUF_DECODE_SYMBOLX2_1!(p, bitDPtr) as isize);
                }
                if HUF_DECODE_SYMBOLX2_2!(p, bitDPtr) != 0 {
                    p = p.offset(HUF_DECODE_SYMBOLX2_2!(p, bitDPtr) as isize);
                }
                p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
            }
        }
    } else {
        BIT_reloadDStream(bitDPtr);
    }
    if pEnd.offset_from(p) as std::ffi::c_long as usize
        >= 2
    {
        while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
            == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int
            & (p <= pEnd.offset(-2_isize)) as std::ffi::c_int
            != 0
        {
            p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
        }
        while p <= pEnd.offset(-2_isize) {
            p = p.offset(HUF_DECODE_SYMBOLX2_0!(p, bitDPtr) as isize);
        }
    }
    if p < pEnd {
        p = p
            .offset(
                HUF_decodeLastSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    return p.offset_from(pStart) as std::ffi::c_long as usize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let ostart = dst as *mut u8;
    let oend = ZSTD_maybeNullPtrAdd(
        ostart as *mut std::ffi::c_void,
        dstSize as ptrdiff_t,
    ) as *mut u8;
    let dtPtr = DTable.offset(1) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let dtd = HUF_getDTableDesc(DTable);
    HUF_decodeStreamX2(ostart, &mut bitD, oend, dt, dtd.tableLog as u32);
    if BIT_endOfDStream(&mut bitD) == 0 {
        return ERROR!(corruption_detected);
    }
    return dstSize;
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    if cSrcSize < 10 {
        return ERROR!(corruption_detected);
    }
    if dstSize < 6 {
        return ERROR!(corruption_detected);
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend
        .offset(
            -((::core::mem::size_of::<usize>())
                .wrapping_sub(1) as isize),
        );
    let dtPtr = DTable.offset(1) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const std::ffi::c_char,
        start: 0 as *const std::ffi::c_char,
        limitPtr: 0 as *const std::ffi::c_char,
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as usize;
    let length2 = MEM_readLE16(
        istart.offset(2) as *const std::ffi::c_void,
    ) as usize;
    let length3 = MEM_readLE16(
        istart.offset(4) as *const std::ffi::c_void,
    ) as usize;
    let length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6),
        );
    let istart1 = istart.offset(6);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize.wrapping_add(3)
        / 4;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: u32 = 1;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return ERROR!(corruption_detected);
    }
    if opStart4 > oend {
        return ERROR!(corruption_detected);
    }
    let _var_err__ = BIT_initDStream(
        &mut bitD1,
        istart1 as *const std::ffi::c_void,
        length1,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let _var_err___0 = BIT_initDStream(
        &mut bitD2,
        istart2 as *const std::ffi::c_void,
        length2,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let _var_err___1 = BIT_initDStream(
        &mut bitD3,
        istart3 as *const std::ffi::c_void,
        length3,
    );
    if ERR_isError(_var_err___1) != 0 {
        return _var_err___1;
    }
    let _var_err___2 = BIT_initDStream(
        &mut bitD4,
        istart4 as *const std::ffi::c_void,
        length4,
    );
    if ERR_isError(_var_err___2) != 0 {
        return _var_err___2;
    }
    if oend.offset_from(op4) as std::ffi::c_long as usize
        >= ::core::mem::size_of::<usize>()
    {
        while endSignal & (op4 < olimit) as std::ffi::c_int as u32 != 0 {
            if HUF_DECODE_SYMBOLX2_2!(op1, & bitD1) != 0 {
                op1 = op1.offset(HUF_DECODE_SYMBOLX2_2!(op1, & bitD1) as isize);
            }
            if HUF_DECODE_SYMBOLX2_1!(op1, & bitD1) != 0 {
                op1 = op1.offset(HUF_DECODE_SYMBOLX2_1!(op1, & bitD1) as isize);
            }
            if HUF_DECODE_SYMBOLX2_2!(op1, & bitD1) != 0 {
                op1 = op1.offset(HUF_DECODE_SYMBOLX2_2!(op1, & bitD1) as isize);
            }
            op1 = op1.offset(HUF_DECODE_SYMBOLX2_0!(op1, & bitD1) as isize);
            if HUF_DECODE_SYMBOLX2_2!(op2, & bitD2) != 0 {
                op2 = op2.offset(HUF_DECODE_SYMBOLX2_2!(op2, & bitD2) as isize);
            }
            if HUF_DECODE_SYMBOLX2_1!(op2, & bitD2) != 0 {
                op2 = op2.offset(HUF_DECODE_SYMBOLX2_1!(op2, & bitD2) as isize);
            }
            if HUF_DECODE_SYMBOLX2_2!(op2, & bitD2) != 0 {
                op2 = op2.offset(HUF_DECODE_SYMBOLX2_2!(op2, & bitD2) as isize);
            }
            op2 = op2.offset(HUF_DECODE_SYMBOLX2_0!(op2, & bitD2) as isize);
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD1) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD2) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
            if HUF_DECODE_SYMBOLX2_2!(op3, & bitD3) != 0 {
                op3 = op3.offset(HUF_DECODE_SYMBOLX2_2!(op3, & bitD3) as isize);
            }
            if HUF_DECODE_SYMBOLX2_1!(op3, & bitD3) != 0 {
                op3 = op3.offset(HUF_DECODE_SYMBOLX2_1!(op3, & bitD3) as isize);
            }
            if HUF_DECODE_SYMBOLX2_2!(op3, & bitD3) != 0 {
                op3 = op3.offset(HUF_DECODE_SYMBOLX2_2!(op3, & bitD3) as isize);
            }
            op3 = op3.offset(HUF_DECODE_SYMBOLX2_0!(op3, & bitD3) as isize);
            if HUF_DECODE_SYMBOLX2_2!(op4, & bitD4) != 0 {
                op4 = op4.offset(HUF_DECODE_SYMBOLX2_2!(op4, & bitD4) as isize);
            }
            if HUF_DECODE_SYMBOLX2_1!(op4, & bitD4) != 0 {
                op4 = op4.offset(HUF_DECODE_SYMBOLX2_1!(op4, & bitD4) as isize);
            }
            if HUF_DECODE_SYMBOLX2_2!(op4, & bitD4) != 0 {
                op4 = op4.offset(HUF_DECODE_SYMBOLX2_2!(op4, & bitD4) as isize);
            }
            op4 = op4.offset(HUF_DECODE_SYMBOLX2_0!(op4, & bitD4) as isize);
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD3) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
            endSignal
                &= (BIT_reloadDStreamFast(&mut bitD4) as std::ffi::c_uint
                    == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as u32;
        }
    }
    if op1 > opStart2 {
        return ERROR!(corruption_detected);
    }
    if op2 > opStart3 {
        return ERROR!(corruption_detected);
    }
    if op3 > opStart4 {
        return ERROR!(corruption_detected);
    }
    HUF_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BIT_endOfDStream(&mut bitD1) & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3) & BIT_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return ERROR!(corruption_detected);
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    return HUF_decompress4X2_usingDTable_internal_body(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
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
    let mut bits: [u64; 4] = [0; 4];
    let mut ip: [*const u8; 4] = [0 as *const u8; 4];
    let mut op: [*mut u8; 4] = [0 as *mut u8; 4];
    let mut oend: [*mut u8; 4] = [0 as *mut u8; 4];
    let dtable = (*args).dt as *const HUF_DEltX2;
    let ilowest = (*args).ilowest;
    libc::memcpy(
        &mut bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut (*args).bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as usize,
    );
    libc::memcpy(
        &mut ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut (*args).ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as usize,
    );
    libc::memcpy(
        &mut op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut (*args).op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as usize,
    );
    oend[0] = op[1];
    oend[1] = op[2];
    oend[2] = op[3];
    oend[3] = (*args).oend;
    's_45: loop {
        let mut olimit = 0 as *mut u8;
        let mut stream: std::ffi::c_int = 0;
        stream = 0;
        while stream < 4 {
            stream += 1;
            stream;
        }
        let mut iters = (ip[0]).offset_from(ilowest)
            as std::ffi::c_long as usize / 7;
        stream = 0;
        while stream < 4 {
            let oiters = (oend[stream as usize]).offset_from(op[stream as usize])
                as std::ffi::c_long as usize / 10;
            iters = MIN!(iters, oiters);
            stream += 1;
            stream;
        }
        olimit = (op[3])
            .offset((iters * 5 as usize) as isize);
        if op[3] == olimit {
            break;
        }
        stream = 1;
        while stream < 4 {
            if ip[stream as usize] < ip[(stream - 1 as std::ffi::c_int) as usize] {
                break 's_45;
            }
            stream += 1;
            stream;
        }
        stream = 1;
        while stream < 4 {
            stream += 1;
            stream;
        }
        loop {
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3
            {
                let index = (bits[0]
                    >> 53) as std::ffi::c_int;
                let entry = *dtable.offset(index as isize);
                MEM_write16(
                    op[0] as *mut std::ffi::c_void,
                    entry.sequence,
                );
                bits[0]
                    <<= entry.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[0] = (op[0])
                    .offset(entry.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3
            {
                let index_0 = (bits[1]
                    >> 53) as std::ffi::c_int;
                let entry_0 = *dtable.offset(index_0 as isize);
                MEM_write16(
                    op[1] as *mut std::ffi::c_void,
                    entry_0.sequence,
                );
                bits[1]
                    <<= entry_0.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[1] = (op[1])
                    .offset(entry_0.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3
            {
                let index_1 = (bits[2]
                    >> 53) as std::ffi::c_int;
                let entry_1 = *dtable.offset(index_1 as isize);
                MEM_write16(
                    op[2] as *mut std::ffi::c_void,
                    entry_1.sequence,
                );
                bits[2]
                    <<= entry_1.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[2] = (op[2])
                    .offset(entry_1.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_2 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_2 = *dtable.offset(index_2 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_2.sequence,
                );
                bits[3]
                    <<= entry_2.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_2.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3
            {
                let index_3 = (bits[0]
                    >> 53) as std::ffi::c_int;
                let entry_3 = *dtable.offset(index_3 as isize);
                MEM_write16(
                    op[0] as *mut std::ffi::c_void,
                    entry_3.sequence,
                );
                bits[0]
                    <<= entry_3.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[0] = (op[0])
                    .offset(entry_3.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3
            {
                let index_4 = (bits[1]
                    >> 53) as std::ffi::c_int;
                let entry_4 = *dtable.offset(index_4 as isize);
                MEM_write16(
                    op[1] as *mut std::ffi::c_void,
                    entry_4.sequence,
                );
                bits[1]
                    <<= entry_4.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[1] = (op[1])
                    .offset(entry_4.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3
            {
                let index_5 = (bits[2]
                    >> 53) as std::ffi::c_int;
                let entry_5 = *dtable.offset(index_5 as isize);
                MEM_write16(
                    op[2] as *mut std::ffi::c_void,
                    entry_5.sequence,
                );
                bits[2]
                    <<= entry_5.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[2] = (op[2])
                    .offset(entry_5.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_6 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_6 = *dtable.offset(index_6 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_6.sequence,
                );
                bits[3]
                    <<= entry_6.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_6.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3
            {
                let index_7 = (bits[0]
                    >> 53) as std::ffi::c_int;
                let entry_7 = *dtable.offset(index_7 as isize);
                MEM_write16(
                    op[0] as *mut std::ffi::c_void,
                    entry_7.sequence,
                );
                bits[0]
                    <<= entry_7.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[0] = (op[0])
                    .offset(entry_7.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3
            {
                let index_8 = (bits[1]
                    >> 53) as std::ffi::c_int;
                let entry_8 = *dtable.offset(index_8 as isize);
                MEM_write16(
                    op[1] as *mut std::ffi::c_void,
                    entry_8.sequence,
                );
                bits[1]
                    <<= entry_8.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[1] = (op[1])
                    .offset(entry_8.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3
            {
                let index_9 = (bits[2]
                    >> 53) as std::ffi::c_int;
                let entry_9 = *dtable.offset(index_9 as isize);
                MEM_write16(
                    op[2] as *mut std::ffi::c_void,
                    entry_9.sequence,
                );
                bits[2]
                    <<= entry_9.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[2] = (op[2])
                    .offset(entry_9.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_10 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_10 = *dtable.offset(index_10 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_10.sequence,
                );
                bits[3]
                    <<= entry_10.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_10.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3
            {
                let index_11 = (bits[0]
                    >> 53) as std::ffi::c_int;
                let entry_11 = *dtable.offset(index_11 as isize);
                MEM_write16(
                    op[0] as *mut std::ffi::c_void,
                    entry_11.sequence,
                );
                bits[0]
                    <<= entry_11.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[0] = (op[0])
                    .offset(entry_11.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3
            {
                let index_12 = (bits[1]
                    >> 53) as std::ffi::c_int;
                let entry_12 = *dtable.offset(index_12 as isize);
                MEM_write16(
                    op[1] as *mut std::ffi::c_void,
                    entry_12.sequence,
                );
                bits[1]
                    <<= entry_12.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[1] = (op[1])
                    .offset(entry_12.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3
            {
                let index_13 = (bits[2]
                    >> 53) as std::ffi::c_int;
                let entry_13 = *dtable.offset(index_13 as isize);
                MEM_write16(
                    op[2] as *mut std::ffi::c_void,
                    entry_13.sequence,
                );
                bits[2]
                    <<= entry_13.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[2] = (op[2])
                    .offset(entry_13.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_14 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_14 = *dtable.offset(index_14 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_14.sequence,
                );
                bits[3]
                    <<= entry_14.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_14.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3
            {
                let index_15 = (bits[0]
                    >> 53) as std::ffi::c_int;
                let entry_15 = *dtable.offset(index_15 as isize);
                MEM_write16(
                    op[0] as *mut std::ffi::c_void,
                    entry_15.sequence,
                );
                bits[0]
                    <<= entry_15.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[0] = (op[0])
                    .offset(entry_15.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3
            {
                let index_16 = (bits[1]
                    >> 53) as std::ffi::c_int;
                let entry_16 = *dtable.offset(index_16 as isize);
                MEM_write16(
                    op[1] as *mut std::ffi::c_void,
                    entry_16.sequence,
                );
                bits[1]
                    <<= entry_16.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[1] = (op[1])
                    .offset(entry_16.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3
            {
                let index_17 = (bits[2]
                    >> 53) as std::ffi::c_int;
                let entry_17 = *dtable.offset(index_17 as isize);
                MEM_write16(
                    op[2] as *mut std::ffi::c_void,
                    entry_17.sequence,
                );
                bits[2]
                    <<= entry_17.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[2] = (op[2])
                    .offset(entry_17.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_18 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_18 = *dtable.offset(index_18 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_18.sequence,
                );
                bits[3]
                    <<= entry_18.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_18.length as std::ffi::c_int as isize);
            }
            if HUF_4X2_DECODE_SYMBOL!(3, 1) != 0 {
                let index_19 = HUF_4X2_DECODE_SYMBOL!(3, 1);
                let entry_19 = HUF_4X2_DECODE_SYMBOL!(3, 1);
                HUF_4X2_DECODE_SYMBOL!(
                    3, 1
                )(HUF_4X2_DECODE_SYMBOL!(3, 1), HUF_4X2_DECODE_SYMBOL!(3, 1));
                let ref mut fresh28 = HUF_4X2_DECODE_SYMBOL!(3, 1);
                *fresh28 <<= HUF_4X2_DECODE_SYMBOL!(3, 1);
                let ref mut fresh29 = HUF_4X2_DECODE_SYMBOL!(3, 1);
                *fresh29 = (*fresh29).offset(HUF_4X2_DECODE_SYMBOL!(3, 1) as isize);
            }
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_20 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_20 = *dtable.offset(index_20 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_20.sequence,
                );
                bits[3]
                    <<= entry_20.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_20.length as std::ffi::c_int as isize);
            }
            let ctz = ZSTD_countTrailingZeros64(bits[0])
                as std::ffi::c_int;
            let nbBits = ctz & 7;
            let nbBytes = ctz >> 3;
            ip[0] = (ip[0])
                .offset(-(nbBytes as isize));
            bits[0] = MEM_read64(
                ip[0] as *const std::ffi::c_void,
            ) | 1;
            bits[0] <<= nbBits;
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_21 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_21 = *dtable.offset(index_21 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_21.sequence,
                );
                bits[3]
                    <<= entry_21.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_21.length as std::ffi::c_int as isize);
            }
            let ctz_0 = ZSTD_countTrailingZeros64(bits[1])
                as std::ffi::c_int;
            let nbBits_0 = ctz_0 & 7;
            let nbBytes_0 = ctz_0 >> 3;
            ip[1] = (ip[1])
                .offset(-(nbBytes_0 as isize));
            bits[1] = MEM_read64(
                ip[1] as *const std::ffi::c_void,
            ) | 1;
            bits[1] <<= nbBits_0;
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_22 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_22 = *dtable.offset(index_22 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_22.sequence,
                );
                bits[3]
                    <<= entry_22.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_22.length as std::ffi::c_int as isize);
            }
            let ctz_1 = ZSTD_countTrailingZeros64(bits[2])
                as std::ffi::c_int;
            let nbBits_1 = ctz_1 & 7;
            let nbBytes_1 = ctz_1 >> 3;
            ip[2] = (ip[2])
                .offset(-(nbBytes_1 as isize));
            bits[2] = MEM_read64(
                ip[2] as *const std::ffi::c_void,
            ) | 1;
            bits[2] <<= nbBits_1;
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3
            {
                let index_23 = (bits[3]
                    >> 53) as std::ffi::c_int;
                let entry_23 = *dtable.offset(index_23 as isize);
                MEM_write16(
                    op[3] as *mut std::ffi::c_void,
                    entry_23.sequence,
                );
                bits[3]
                    <<= entry_23.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                op[3] = (op[3])
                    .offset(entry_23.length as std::ffi::c_int as isize);
            }
            let ctz_2 = ZSTD_countTrailingZeros64(bits[3])
                as std::ffi::c_int;
            let nbBits_2 = ctz_2 & 7;
            let nbBytes_2 = ctz_2 >> 3;
            ip[3] = (ip[3])
                .offset(-(nbBytes_2 as isize));
            bits[3] = MEM_read64(
                ip[3] as *const std::ffi::c_void,
            ) | 1;
            bits[3] <<= nbBits_2;
            if !(op[3] < olimit) {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as usize,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as usize,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as usize,
    );
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut loopFn: HUF_DecompressFastLoopFn,
) -> usize {
    let mut dt = DTable.offset(1) as *const std::ffi::c_void;
    let ilowest = cSrc as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    let mut args = HUF_DecompressFastArgs {
        ip: [0 as *const u8; 4],
        op: [0 as *mut u8; 4],
        bits: [0; 4],
        dt: 0 as *const std::ffi::c_void,
        ilowest: 0 as *const u8,
        oend: 0 as *mut u8,
        iend: [0 as *const u8; 4],
    };
    let ret = HUF_DecompressFastArgs_init(
        &mut args,
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
    let err_code = FORWARD_IF_ERROR!(ret, "Failed to init asm args");
    if FORWARD_IF_ERROR!(ret, "Failed to init asm args") != 0 {
        return FORWARD_IF_ERROR!(ret, "Failed to init asm args");
    }
    if ret == 0 {
        return 0;
    }
    loopFn.expect("non-null function pointer")(&mut args);
    let segmentSize = dstSize.wrapping_add(3)
        / 4;
    let mut segmentEnd = dst as *mut u8;
    let mut i: std::ffi::c_int = 0;
    i = 0;
    while i < 4 {
        let mut bit = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: 0 as *const std::ffi::c_char,
            start: 0 as *const std::ffi::c_char,
            limitPtr: 0 as *const std::ffi::c_char,
        };
        if segmentSize <= oend.offset_from(segmentEnd) as std::ffi::c_long as usize {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }
        let err_code_0 = FORWARD_IF_ERROR!(
            HUF_initRemainingDStream(& bit, & args, i, segmentEnd), "corruption"
        );
        if FORWARD_IF_ERROR!(
            HUF_initRemainingDStream(& bit, & args, i, segmentEnd), "corruption"
        ) != 0
        {
            return FORWARD_IF_ERROR!(
                HUF_initRemainingDStream(& bit, & args, i, segmentEnd), "corruption"
            );
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
                    HUF_DECODER_FAST_TABLELOG as u32,
                ) as isize,
            );
        if args.op[i as usize] != segmentEnd {
            return ERROR!(corruption_detected);
        }
        i += 1;
        i;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut fallbackFn: HUF_DecompressUsingDTableFn = Some(
        HUF_decompress4X2_usingDTable_internal_default
            as unsafe extern "C" fn(
                *mut std::ffi::c_void,
                usize,
                *const std::ffi::c_void,
                usize,
                *const HUF_DTable,
            ) -> usize,
    );
    let mut loopFn: HUF_DecompressFastLoopFn = Some(
        HUF_decompress4X2_usingDTable_internal_fast_c_loop
            as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
    );
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        fallbackFn = Some(
            HUF_decompress4X2_usingDTable_internal_bmi2
                as unsafe extern "C" fn(
                    *mut std::ffi::c_void,
                    usize,
                    *const std::ffi::c_void,
                    usize,
                    *const HUF_DTable,
                ) -> usize,
        );
    } else {
        return fallbackFn
            .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable)
    }
    if HUF_ENABLE_FAST_DECODE != 0
        && flags & HUF_flags_disableFast as std::ffi::c_int == 0
    {
        let ret = HUF_decompress4X2_usingDTable_internal_fast(
            dst,
            dstSize,
            cSrc,
            cSrcSize,
            DTable,
            loopFn,
        );
        if ret != 0 {
            return ret;
        }
    }
    return fallbackFn
        .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    return HUF_decompress1X2_usingDTable_internal_body(
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
    );
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
) -> usize {
    return HUF_decompress1X2_usingDTable_internal_body(
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
    );
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> usize {
    if HUF_DGEN!(HUF_decompress1X2_usingDTable_internal) != 0 {
        return HUF_decompress1X2_usingDTable_internal_bmi2(
            HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
            HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        );
    }
    return HUF_decompress1X2_usingDTable_internal_default(
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
        HUF_DGEN!(HUF_decompress1X2_usingDTable_internal),
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X2_DCtx_wksp(
    mut DCtx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX2_wksp(DCtx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return ERROR!(srcSize_wrong);
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    return HUF_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DCtx,
        flags,
    );
}
unsafe extern "C" fn HUF_decompress4X2_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut ip = cSrc as *const u8;
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
        return ERROR!(srcSize_wrong);
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    return HUF_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    );
}
static mut algoTime: [[algo_time_t; 2]; 16] = [
    [
        {
            let mut init = algo_time_t {
                tableTime: 0,
                decode256Time: 0,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1,
                decode256Time: 1,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 0,
                decode256Time: 0,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1,
                decode256Time: 1,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 150,
                decode256Time: 216,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 381,
                decode256Time: 119,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 170,
                decode256Time: 205,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 514,
                decode256Time: 112,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 177,
                decode256Time: 199,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 539,
                decode256Time: 110,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 197,
                decode256Time: 194,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 644,
                decode256Time: 107,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 221,
                decode256Time: 192,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 735,
                decode256Time: 107,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 256,
                decode256Time: 189,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 881,
                decode256Time: 106,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 359,
                decode256Time: 188,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1167,
                decode256Time: 109,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 582,
                decode256Time: 187,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1570,
                decode256Time: 114,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 688,
                decode256Time: 187,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1712,
                decode256Time: 122,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 825,
                decode256Time: 186,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1965,
                decode256Time: 136,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 976,
                decode256Time: 185,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2131,
                decode256Time: 150,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1180,
                decode256Time: 186,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2070,
                decode256Time: 175,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1377,
                decode256Time: 185,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1731,
                decode256Time: 202,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1412,
                decode256Time: 185,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1695,
                decode256Time: 202,
            };
            init
        },
    ],
];
#[no_mangle]
pub unsafe extern "C" fn HUF_selectDecoder(
    mut dstSize: usize,
    mut cSrcSize: usize,
) -> u32 {
    let Q = if cSrcSize >= dstSize {
        15 as u32
    } else {
        (cSrcSize * 16 as usize / dstSize) as u32
    };
    let D256 = (dstSize >> 8) as u32;
    let DTime0 = (algoTime[Q as usize][0].tableTime)
        .wrapping_add(
            algoTime[Q as usize][0].decode256Time * D256,
        );
    let mut DTime1 = (algoTime[Q as usize][1].tableTime)
        .wrapping_add(
            algoTime[Q as usize][1].decode256Time * D256,
        );
    DTime1 = DTime1.wrapping_add(DTime1 >> 5);
    return (DTime1 < DTime0) as std::ffi::c_int as u32;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    if dstSize == 0 {
        return ERROR!(dstSize_tooSmall);
    }
    if cSrcSize > dstSize {
        return ERROR!(corruption_detected);
    }
    if cSrcSize == dstSize {
        libc::memcpy(
            ZSTD_memcpy!(dst, cSrc, dstSize),
            ZSTD_memcpy!(dst, cSrc, dstSize),
            ZSTD_memcpy!(dst, cSrc, dstSize) as usize,
        );
        return dstSize;
    }
    if cSrcSize == 1 {
        libc::memset(
            ZSTD_memset!(dst, * (const u8 *) cSrc, dstSize),
            ZSTD_memset!(dst, * (const u8 *) cSrc, dstSize),
            ZSTD_memset!(dst, * (const u8 *) cSrc, dstSize) as usize,
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> usize {
    let dtd = HUF_getDTableDesc(DTable);
    return if dtd.tableType as std::ffi::c_int != 0 {
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
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return ERROR!(srcSize_wrong);
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    return HUF_decompress1X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> usize {
    let dtd = HUF_getDTableDesc(DTable);
    return if dtd.tableType as std::ffi::c_int != 0 {
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
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: std::ffi::c_int,
) -> usize {
    if dstSize == 0 {
        return ERROR!(dstSize_tooSmall);
    }
    if cSrcSize == 0 {
        return ERROR!(corruption_detected);
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
