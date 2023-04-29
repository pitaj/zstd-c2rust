use ::libc;
extern "C" {
    pub type ZSTDv03_Dctx_s;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTable: [U32; 1025],
    pub OffTable: [U32; 513],
    pub MLTable: [U32; 1025],
    pub previousDstEnd: *mut libc::c_void,
    pub base: *mut libc::c_void,
    pub expected: size_t,
    pub bType: blockType_t,
    pub phase: U32,
    pub litPtr: *const BYTE,
    pub litSize: size_t,
    pub litBuffer: [BYTE; 131080],
}
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type blockType_t = libc::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_t,
    pub origSize: U32,
}
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed_2 = 72;
pub const ZSTD_error_maxCode: C2RustUnnamed_2 = 120;
pub const ZSTD_error_GENERIC: C2RustUnnamed_2 = 1;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed_2 = 70;
pub const ZSTD_error_corruption_detected: C2RustUnnamed_2 = 20;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BIT_DStream_t,
    pub stateLL: FSE_DState_t,
    pub stateOffb: FSE_DState_t,
    pub stateML: FSE_DState_t,
    pub prevOffset: size_t,
    pub dumps: *const BYTE,
    pub dumpsEnd: *const BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub offset: size_t,
    pub matchLength: size_t,
}
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: U32,
    pub c: [BYTE; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type BIT_DStream_status = libc::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type FSE_DTable = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub type S16 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed_2 = 44;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed_2 = 46;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed_2 = 48;
pub type decompressionAlgo = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        size_t,
        *const libc::c_void,
        size_t,
    ) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX4 {
    pub sequence: U16,
    pub nbBits: BYTE,
    pub length: BYTE,
}
pub type rankVal_t = [[U32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: BYTE,
    pub weight: BYTE,
}
pub type DTable_max_t = [U32; 4097];
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HUF_static_assert: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX2 {
    pub byte: BYTE,
    pub nbBits: BYTE,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const HUF_static_assert_0: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: U32,
    pub decode256Time: U32,
}
pub const ZSTD_error_prefix_unknown: C2RustUnnamed_2 = 10;
pub type ZSTDv03_Dctx = ZSTDv03_Dctx_s;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed_2 = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed_2 = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed_2 = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed_2 = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed_2 = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed_2 = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed_2 = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed_2 = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed_2 = 74;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed_2 = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed_2 = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed_2 = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed_2 = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed_2 = 50;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed_2 = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed_2 = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed_2 = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed_2 = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed_2 = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed_2 = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed_2 = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed_2 = 22;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed_2 = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed_2 = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed_2 = 12;
pub const ZSTD_error_no_error: C2RustUnnamed_2 = 0;
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    let one = C2RustUnnamed {
        u: 1 as libc::c_int as U32,
    };
    return one.c[0 as libc::c_int as usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read16(mut memPtr: *const libc::c_void) -> U16 {
    let mut val: U16 = 0;
    memcpy(
        &mut val as *mut U16 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<U16>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn MEM_read32(mut memPtr: *const libc::c_void) -> U32 {
    let mut val: U32 = 0;
    memcpy(
        &mut val as *mut U32 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<U32>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn MEM_read64(mut memPtr: *const libc::c_void) -> U64 {
    let mut val: U64 = 0;
    memcpy(
        &mut val as *mut U64 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<U64>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: U16) {
    memcpy(
        memPtr,
        &mut value as *mut U16 as *const libc::c_void,
        ::core::mem::size_of::<U16>() as libc::c_ulong,
    );
}
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
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: U16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut BYTE;
        *p.offset(0 as libc::c_int as isize) = val as BYTE;
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (val as libc::c_int >> 8 as libc::c_int) as BYTE;
    };
}
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as libc::c_int
        + ((*(memPtr as *const BYTE).offset(2 as libc::c_int as isize) as libc::c_int)
            << 16 as libc::c_int)) as U32;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as U32)
            .wrapping_add(
                (*p.offset(1 as libc::c_int as isize) as U32) << 8 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(2 as libc::c_int as isize) as U32) << 16 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(3 as libc::c_int as isize) as U32) << 24 as libc::c_int,
            );
    };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as U64)
            .wrapping_add(
                (*p.offset(1 as libc::c_int as isize) as U64) << 8 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(2 as libc::c_int as isize) as U64) << 16 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(3 as libc::c_int as isize) as U64) << 24 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(4 as libc::c_int as isize) as U64) << 32 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(5 as libc::c_int as isize) as U64) << 40 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(6 as libc::c_int as isize) as U64) << 48 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(7 as libc::c_int as isize) as U64) << 56 as libc::c_int,
            );
    };
}
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as size_t
    } else {
        return MEM_readLE64(memPtr)
    };
}
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    return (val.leading_zeros() as i32 ^ 31 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 as libc::c_int as libc::c_ulong {
        memset(
            bitD as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<BIT_DStream_t>() as libc::c_ulong,
        );
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if srcSize >= ::core::mem::size_of::<size_t>() as libc::c_ulong {
        let mut contain32: U32 = 0;
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD)
            .ptr = (srcBuffer as *const libc::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<size_t>() as libc::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        contain32 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32;
        if contain32 == 0 as libc::c_int as libc::c_uint {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
        (*bitD)
            .bitsConsumed = (8 as libc::c_int as libc::c_uint)
            .wrapping_sub(BIT_highbit32(contain32));
    } else {
        let mut contain32_0: U32 = 0;
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const BYTE) as size_t;
        let mut current_block_21: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(6 as libc::c_int as isize) as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(16 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_21 = 8720944328620910342;
            }
            6 => {
                current_block_21 = 8720944328620910342;
            }
            5 => {
                current_block_21 = 8800178136727786715;
            }
            4 => {
                current_block_21 = 13300855864153761251;
            }
            3 => {
                current_block_21 = 12880075216549967651;
            }
            2 => {
                current_block_21 = 308338134236164298;
            }
            _ => {
                current_block_21 = 13242334135786603907;
            }
        }
        match current_block_21 {
            8720944328620910342 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(5 as libc::c_int as isize) as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(24 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_21 = 8800178136727786715;
            }
            _ => {}
        }
        match current_block_21 {
            8800178136727786715 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(4 as libc::c_int as isize) as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(32 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_21 = 13300855864153761251;
            }
            _ => {}
        }
        match current_block_21 {
            13300855864153761251 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(3 as libc::c_int as isize) as size_t)
                            << 24 as libc::c_int,
                    ) as size_t as size_t;
                current_block_21 = 12880075216549967651;
            }
            _ => {}
        }
        match current_block_21 {
            12880075216549967651 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(2 as libc::c_int as isize) as size_t)
                            << 16 as libc::c_int,
                    ) as size_t as size_t;
                current_block_21 = 308338134236164298;
            }
            _ => {}
        }
        match current_block_21 {
            308338134236164298 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(1 as libc::c_int as isize) as size_t)
                            << 8 as libc::c_int,
                    ) as size_t as size_t;
            }
            _ => {}
        }
        contain32_0 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32;
        if contain32_0 == 0 as libc::c_int as libc::c_uint {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
        (*bitD)
            .bitsConsumed = (8 as libc::c_int as libc::c_uint)
            .wrapping_sub(BIT_highbit32(contain32_0));
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
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask) >> 1 as libc::c_int
        >> (bitMask.wrapping_sub(nbBits) & bitMask);
}
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1 as libc::c_int as libc::c_uint).wrapping_sub(nbBits)
            & bitMask);
}
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let mut value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let mut value = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong
        > (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr
        >= ((*bitD).start)
            .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize)
    {
        (*bitD)
            .ptr = ((*bitD).ptr)
            .offset(-(((*bitD).bitsConsumed >> 3 as libc::c_int) as isize));
        (*bitD).bitsConsumed &= 7 as libc::c_int as libc::c_uint;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BIT_DStream_unfinished;
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
unsafe extern "C" fn BIT_endOfDStream(
    mut DStream: *const BIT_DStream_t,
) -> libc::c_uint {
    return ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as libc::c_ulong
            == (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    memcpy(
        &mut DTableH as *mut FSE_DTableHeader as *mut libc::c_void,
        dt as *const libc::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong,
    );
    (*DStatePtr).state = BIT_readBits(bitD, DTableH.tableLog as libc::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as libc::c_int as isize) as *const libc::c_void;
}
unsafe extern "C" fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> libc::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let mut symbol = DInfo.symbol;
    let mut lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_decodeSymbolFast(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> libc::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let mut symbol = DInfo.symbol;
    let mut lowBits = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_endOfDState(
    mut DStatePtr: *const FSE_DState_t,
) -> libc::c_uint {
    return ((*DStatePtr).state == 0 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_uint;
}
pub const ZSTD_magicNumber: libc::c_uint = 0xfd2fb523 as libc::c_uint;
pub const FSE_MAX_MEMORY_USAGE: libc::c_int = 14 as libc::c_int;
pub const FSE_MAX_SYMBOL_VALUE: libc::c_int = 255 as libc::c_int;
pub const FSE_MAX_TABLELOG: libc::c_int = FSE_MAX_MEMORY_USAGE - 2 as libc::c_int;
pub const FSE_MIN_TABLELOG: libc::c_int = 5 as libc::c_int;
pub const FSE_TABLELOG_ABSOLUTE_MAX: libc::c_int = 15 as libc::c_int;
unsafe extern "C" fn FSE_tableStep(mut tableSize: U32) -> U32 {
    return (tableSize >> 1 as libc::c_int)
        .wrapping_add(tableSize >> 3 as libc::c_int)
        .wrapping_add(3 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn FSE_buildDTable(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    let mut ptr = dt.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    let tableDecode = ptr as *mut FSE_decode_t;
    let tableSize = ((1 as libc::c_int) << tableLog) as U32;
    let tableMask = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let step = FSE_tableStep(tableSize);
    let mut symbolNext: [U16; 256] = [0; 256];
    let mut position = 0 as libc::c_int as U32;
    let mut highThreshold = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let largeLimit = ((1 as libc::c_int)
        << tableLog.wrapping_sub(1 as libc::c_int as libc::c_uint)) as S16;
    let mut noLarge = 1 as libc::c_int as U32;
    let mut s: U32 = 0;
    if maxSymbolValue > FSE_MAX_SYMBOL_VALUE as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    if tableLog > FSE_MAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    DTableH.tableLog = tableLog as U16;
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -(1 as libc::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as BYTE;
            symbolNext[s as usize] = 1 as libc::c_int as U16;
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int
                >= largeLimit as libc::c_int
            {
                noLarge = 0 as libc::c_int as U32;
            }
            symbolNext[s as usize] = *normalizedCounter.offset(s as isize) as U16;
        }
        s = s.wrapping_add(1);
    }
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < *normalizedCounter.offset(s as isize) as libc::c_int {
            (*tableDecode.offset(position as isize)).symbol = s as BYTE;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    if position != 0 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    let mut i_0: U32 = 0;
    i_0 = 0 as libc::c_int as U32;
    while i_0 < tableSize {
        let mut symbol = (*tableDecode.offset(i_0 as isize)).symbol;
        let fresh1 = symbolNext[symbol as usize];
        symbolNext[symbol as usize] = (symbolNext[symbol as usize]).wrapping_add(1);
        let mut nextState = fresh1;
        (*tableDecode.offset(i_0 as isize))
            .nbBits = tableLog.wrapping_sub(BIT_highbit32(nextState as U32)) as BYTE;
        (*tableDecode.offset(i_0 as isize))
            .newState = (((nextState as libc::c_int)
            << (*tableDecode.offset(i_0 as isize)).nbBits as libc::c_int)
            as libc::c_uint)
            .wrapping_sub(tableSize) as U16;
        i_0 = i_0.wrapping_add(1);
    }
    DTableH.fastMode = noLarge as U16;
    memcpy(
        dt as *mut libc::c_void,
        &mut DTableH as *mut FSE_DTableHeader as *const libc::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong,
    );
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FSE_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
unsafe extern "C" fn FSE_abs(mut a: libc::c_short) -> libc::c_short {
    return (if (a as libc::c_int) < 0 as libc::c_int {
        -(a as libc::c_int)
    } else {
        a as libc::c_int
    }) as libc::c_short;
}
unsafe extern "C" fn FSE_readNCount(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const BYTE;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: libc::c_int = 0;
    let mut charnum = 0 as libc::c_int as libc::c_uint;
    let mut previous0 = 0 as libc::c_int;
    if hbSize < 4 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    bitStream = MEM_readLE32(ip as *const libc::c_void);
    nbBits = (bitStream & 0xf as libc::c_int as libc::c_uint)
        .wrapping_add(FSE_MIN_TABLELOG as libc::c_uint) as libc::c_int;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    bitStream >>= 4 as libc::c_int;
    bitCount = 4 as libc::c_int;
    *tableLogPtr = nbBits as libc::c_uint;
    remaining = ((1 as libc::c_int) << nbBits) + 1 as libc::c_int;
    threshold = (1 as libc::c_int) << nbBits;
    nbBits += 1;
    while remaining > 1 as libc::c_int && charnum <= *maxSVPtr {
        if previous0 != 0 {
            let mut n0 = charnum;
            while bitStream & 0xffff as libc::c_int as libc::c_uint
                == 0xffff as libc::c_int as libc::c_uint
            {
                n0 = n0.wrapping_add(24 as libc::c_int as libc::c_uint);
                if ip < iend.offset(-(5 as libc::c_int as isize)) {
                    ip = ip.offset(2 as libc::c_int as isize);
                    bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
                } else {
                    bitStream >>= 16 as libc::c_int;
                    bitCount += 16 as libc::c_int;
                }
            }
            while bitStream & 3 as libc::c_int as libc::c_uint
                == 3 as libc::c_int as libc::c_uint
            {
                n0 = n0.wrapping_add(3 as libc::c_int as libc::c_uint);
                bitStream >>= 2 as libc::c_int;
                bitCount += 2 as libc::c_int;
            }
            n0 = n0.wrapping_add(bitStream & 3 as libc::c_int as libc::c_uint);
            bitCount += 2 as libc::c_int;
            if n0 > *maxSVPtr {
                return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
            }
            while charnum < n0 {
                let fresh2 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter
                    .offset(fresh2 as isize) = 0 as libc::c_int as libc::c_short;
            }
            if ip <= iend.offset(-(7 as libc::c_int as isize))
                || ip.offset((bitCount >> 3 as libc::c_int) as isize)
                    <= iend.offset(-(4 as libc::c_int as isize))
            {
                ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
                bitCount &= 7 as libc::c_int;
                bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
            } else {
                bitStream >>= 2 as libc::c_int;
            }
        }
        let max = (2 as libc::c_int * threshold - 1 as libc::c_int - remaining)
            as libc::c_short;
        let mut count: libc::c_short = 0;
        if (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint) < max as U32 {
            count = (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_short;
            bitCount += nbBits - 1 as libc::c_int;
        } else {
            count = (bitStream
                & (2 as libc::c_int * threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_short;
            if count as libc::c_int >= threshold {
                count = (count as libc::c_int - max as libc::c_int) as libc::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        remaining -= FSE_abs(count) as libc::c_int;
        let fresh3 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh3 as isize) = count;
        previous0 = (count == 0) as libc::c_int;
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1 as libc::c_int;
        }
        if ip <= iend.offset(-(7 as libc::c_int as isize))
            || ip.offset((bitCount >> 3 as libc::c_int) as isize)
                <= iend.offset(-(4 as libc::c_int as isize))
        {
            ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
            bitCount &= 7 as libc::c_int;
        } else {
            bitCount
                -= (8 as libc::c_int as libc::c_long
                    * iend.offset(-(4 as libc::c_int as isize)).offset_from(ip)
                        as libc::c_long) as libc::c_int;
            ip = iend.offset(-(4 as libc::c_int as isize));
        }
        bitStream = MEM_readLE32(ip as *const libc::c_void)
            >> (bitCount & 31 as libc::c_int);
    }
    if remaining != 1 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1 as libc::c_int as libc::c_uint);
    ip = ip.offset((bitCount + 7 as libc::c_int >> 3 as libc::c_int) as isize);
    if ip.offset_from(istart) as libc::c_long as size_t > hbSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    return ip.offset_from(istart) as libc::c_long as size_t;
}
unsafe extern "C" fn FSE_buildDTable_rle(
    mut dt: *mut FSE_DTable,
    mut symbolValue: BYTE,
) -> size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSE_DTableHeader;
    let cell = (ptr as *mut FSE_decode_t).offset(1 as libc::c_int as isize);
    (*DTableH).tableLog = 0 as libc::c_int as U16;
    (*DTableH).fastMode = 0 as libc::c_int as U16;
    (*cell).newState = 0 as libc::c_int as libc::c_ushort;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0 as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FSE_buildDTable_raw(
    mut dt: *mut FSE_DTable,
    mut nbBits: libc::c_uint,
) -> size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSE_DTableHeader;
    let dinfo = (ptr as *mut FSE_decode_t).offset(1 as libc::c_int as isize);
    let tableSize = ((1 as libc::c_int) << nbBits) as libc::c_uint;
    let tableMask = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let maxSymbolValue = tableMask;
    let mut s: libc::c_uint = 0;
    if nbBits < 1 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    (*DTableH).tableLog = nbBits as U16;
    (*DTableH).fastMode = 1 as libc::c_int as U16;
    s = 0 as libc::c_int as libc::c_uint;
    while s <= maxSymbolValue {
        (*dinfo.offset(s as isize)).newState = 0 as libc::c_int as libc::c_ushort;
        (*dinfo.offset(s as isize)).symbol = s as BYTE;
        (*dinfo.offset(s as isize)).nbBits = nbBits as BYTE;
        s = s.wrapping_add(1);
    }
    return 0 as libc::c_int as size_t;
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
    };
    let mut state1 = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let mut state2 = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let mut errorCode: size_t = 0;
    errorCode = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);
    while BIT_reloadDStream(&mut bitD) as libc::c_uint
        == BIT_DStream_unfinished as libc::c_int as libc::c_uint && op < olimit
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
    while !(BIT_reloadDStream(&mut bitD) as libc::c_uint
        > BIT_DStream_completed as libc::c_int as libc::c_uint || op == omax
        || BIT_endOfDStream(&mut bitD) != 0
            && (fast != 0 || FSE_endOfDState(&mut state1) != 0))
    {
        let fresh4 = op;
        op = op.offset(1);
        *fresh4 = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if BIT_reloadDStream(&mut bitD) as libc::c_uint
            > BIT_DStream_completed as libc::c_int as libc::c_uint || op == omax
            || BIT_endOfDStream(&mut bitD) != 0
                && (fast != 0 || FSE_endOfDState(&mut state2) != 0)
        {
            break;
        }
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as BYTE;
    }
    if BIT_endOfDStream(&mut bitD) != 0 && FSE_endOfDState(&mut state1) != 0
        && FSE_endOfDState(&mut state2) != 0
    {
        return op.offset_from(ostart) as libc::c_long as size_t;
    }
    if op == omax {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
}
unsafe extern "C" fn FSE_decompress_usingDTable(
    mut dst: *mut libc::c_void,
    mut originalSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSE_DTable,
) -> size_t {
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    memcpy(
        &mut DTableH as *mut FSE_DTableHeader as *mut libc::c_void,
        dt as *const libc::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong,
    );
    if DTableH.fastMode != 0 {
        return FSE_decompress_usingDTable_generic(
            dst,
            originalSize,
            cSrc,
            cSrcSize,
            dt,
            1 as libc::c_int as libc::c_uint,
        );
    }
    return FSE_decompress_usingDTable_generic(
        dst,
        originalSize,
        cSrc,
        cSrcSize,
        dt,
        0 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn FSE_decompress(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart = cSrc as *const BYTE;
    let mut ip = istart;
    let mut counting: [libc::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: libc::c_uint = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as libc::c_uint;
    let mut errorCode: size_t = 0;
    if cSrcSize < 2 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    errorCode = FSE_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const libc::c_void,
        cSrcSize,
    );
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(errorCode) as size_t as size_t;
    errorCode = FSE_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    return FSE_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    );
}
unsafe extern "C" fn HUF_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
pub const HUF_ABSOLUTEMAX_TABLELOG: libc::c_int = 16 as libc::c_int;
pub const HUF_MAX_TABLELOG: libc::c_int = 12 as libc::c_int;
pub const HUF_MAX_SYMBOL_VALUE: libc::c_int = 255 as libc::c_int;
unsafe extern "C" fn HUF_readStats(
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightTotal: U32 = 0;
    let mut tableLog: U32 = 0;
    let mut ip = src as *const BYTE;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    let mut n: U32 = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    iSize = *ip.offset(0 as libc::c_int as isize) as size_t;
    if iSize >= 128 as libc::c_int as libc::c_ulong {
        if iSize >= 242 as libc::c_int as libc::c_ulong {
            static mut l: [libc::c_int; 14] = [
                1 as libc::c_int,
                2 as libc::c_int,
                3 as libc::c_int,
                4 as libc::c_int,
                7 as libc::c_int,
                8 as libc::c_int,
                15 as libc::c_int,
                16 as libc::c_int,
                31 as libc::c_int,
                32 as libc::c_int,
                63 as libc::c_int,
                64 as libc::c_int,
                127 as libc::c_int,
                128 as libc::c_int,
            ];
            oSize = l[iSize.wrapping_sub(242 as libc::c_int as libc::c_ulong) as usize]
                as size_t;
            memset(huffWeight as *mut libc::c_void, 1 as libc::c_int, hwSize);
            iSize = 0 as libc::c_int as size_t;
        } else {
            oSize = iSize.wrapping_sub(127 as libc::c_int as libc::c_ulong);
            iSize = oSize
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
            if iSize.wrapping_add(1 as libc::c_int as libc::c_ulong) > srcSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            if oSize >= hwSize {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(1 as libc::c_int as isize);
            n = 0 as libc::c_int as U32;
            while (n as libc::c_ulong) < oSize {
                *huffWeight
                    .offset(
                        n as isize,
                    ) = (*ip
                    .offset(n.wrapping_div(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int >> 4 as libc::c_int) as BYTE;
                *huffWeight
                    .offset(
                        n.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ) = (*ip
                    .offset(n.wrapping_div(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int & 15 as libc::c_int) as BYTE;
                n = (n as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as U32 as U32;
            }
        }
    } else {
        if iSize.wrapping_add(1 as libc::c_int as libc::c_ulong) > srcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
        oSize = FSE_decompress(
            huffWeight as *mut libc::c_void,
            hwSize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
            iSize,
        );
        if FSE_isError(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankStats as *mut libc::c_void,
        0 as libc::c_int,
        ((HUF_ABSOLUTEMAX_TABLELOG + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong),
    );
    weightTotal = 0 as libc::c_int as U32;
    n = 0 as libc::c_int as U32;
    while (n as libc::c_ulong) < oSize {
        if *huffWeight.offset(n as isize) as libc::c_int >= HUF_ABSOLUTEMAX_TABLELOG {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        let ref mut fresh6 = *rankStats.offset(*huffWeight.offset(n as isize) as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        weightTotal = (weightTotal as libc::c_uint)
            .wrapping_add(
                ((1 as libc::c_int) << *huffWeight.offset(n as isize) as libc::c_int
                    >> 1 as libc::c_int) as libc::c_uint,
            ) as U32 as U32;
        n = n.wrapping_add(1);
    }
    if weightTotal == 0 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    tableLog = (BIT_highbit32(weightTotal))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if tableLog > HUF_ABSOLUTEMAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let mut total = ((1 as libc::c_int) << tableLog) as U32;
    let mut rest = total.wrapping_sub(weightTotal);
    let mut verif = ((1 as libc::c_int) << BIT_highbit32(rest)) as U32;
    let mut lastWeight = (BIT_highbit32(rest))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as BYTE;
    let ref mut fresh7 = *rankStats.offset(lastWeight as isize);
    *fresh7 = (*fresh7).wrapping_add(1);
    if *rankStats.offset(1 as libc::c_int as isize) < 2 as libc::c_int as libc::c_uint
        || *rankStats.offset(1 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint != 0
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1 as libc::c_int as libc::c_ulong) as U32;
    *tableLogPtr = tableLog;
    return iSize.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn HUF_readDTableX2(
    mut DTable: *mut U16,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut huffWeight: [BYTE; 256] = [0; 256];
    let mut rankVal: [U32; 17] = [0; 17];
    let mut tableLog = 0 as libc::c_int as U32;
    let mut ip = src as *const BYTE;
    let mut iSize = *ip.offset(0 as libc::c_int as isize) as size_t;
    let mut nbSymbols = 0 as libc::c_int as U32;
    let mut n: U32 = 0;
    let mut nextRankStart: U32 = 0;
    let mut ptr = DTable.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dt = ptr as *mut HUF_DEltX2;
    iSize = HUF_readStats(
        huffWeight.as_mut_ptr(),
        (HUF_MAX_SYMBOL_VALUE + 1 as libc::c_int) as size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUF_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > *DTable.offset(0 as libc::c_int as isize) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    *DTable.offset(0 as libc::c_int as isize) = tableLog as U16;
    nextRankStart = 0 as libc::c_int as U32;
    n = 1 as libc::c_int as U32;
    while n <= tableLog {
        let mut current = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(
                rankVal[n as usize] << n.wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) as U32 as U32;
        rankVal[n as usize] = current;
        n = n.wrapping_add(1);
    }
    n = 0 as libc::c_int as U32;
    while n < nbSymbols {
        let w = huffWeight[n as usize] as U32;
        let length = ((1 as libc::c_int) << w >> 1 as libc::c_int) as U32;
        let mut i: U32 = 0;
        let mut D = HUF_DEltX2 { byte: 0, nbBits: 0 };
        D.byte = n as BYTE;
        D
            .nbBits = tableLog
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(w) as BYTE;
        i = rankVal[w as usize];
        while i < (rankVal[w as usize]).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
        }
        rankVal[w
            as usize] = (rankVal[w as usize] as libc::c_uint).wrapping_add(length) as U32
            as U32;
        n = n.wrapping_add(1);
    }
    return iSize;
}
unsafe extern "C" fn HUF_decodeSymbolX2(
    mut Dstream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: U32,
) -> BYTE {
    let val = BIT_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BIT_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as U32);
    return c;
}
#[inline]
unsafe extern "C" fn HUF_decodeStreamX2(
    mut p: *mut BYTE,
    bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut BYTE,
    dt: *const HUF_DEltX2,
    dtLog: U32,
) -> size_t {
    let pStart = p;
    while BIT_reloadDStream(bitDPtr) as libc::c_uint
        == BIT_DStream_unfinished as libc::c_int as libc::c_uint
        && p <= pEnd.offset(-(4 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = HUF_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = HUF_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = HUF_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = HUF_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while BIT_reloadDStream(bitDPtr) as libc::c_uint
        == BIT_DStream_unfinished as libc::c_int as libc::c_uint && p < pEnd
    {
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = HUF_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = HUF_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    return pEnd.offset_from(pStart) as libc::c_long as size_t;
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U16,
) -> size_t {
    if cSrcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut ptr = DTable as *const libc::c_void;
    let dt = (ptr as *const HUF_DEltX2).offset(1 as libc::c_int as isize);
    let dtLog = *DTable.offset(0 as libc::c_int as isize) as U32;
    let mut errorCode: size_t = 0;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let length1 = MEM_readLE16(istart as *const libc::c_void) as size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let mut length4: size_t = 0;
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
    let mut endSignal: U32 = 0;
    length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6 as libc::c_int as libc::c_ulong),
        );
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    errorCode = BIT_initDStream(&mut bitD1, istart1 as *const libc::c_void, length1);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BIT_initDStream(&mut bitD2, istart2 as *const libc::c_void, length2);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BIT_initDStream(&mut bitD3, istart3 as *const libc::c_void, length3);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BIT_initDStream(&mut bitD4, istart4 as *const libc::c_void, length4);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BIT_reloadDStream(&mut bitD1) as libc::c_uint
        | BIT_reloadDStream(&mut bitD2) as libc::c_uint
        | BIT_reloadDStream(&mut bitD3) as libc::c_uint
        | BIT_reloadDStream(&mut bitD4) as libc::c_uint;
    while endSignal == BIT_DStream_unfinished as libc::c_int as libc::c_uint
        && op4 < oend.offset(-(7 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            let fresh14 = op1;
            op1 = op1.offset(1);
            *fresh14 = HUF_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh15 = op2;
            op2 = op2.offset(1);
            *fresh15 = HUF_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh16 = op3;
            op3 = op3.offset(1);
            *fresh16 = HUF_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh17 = op4;
            op4 = op4.offset(1);
            *fresh17 = HUF_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            let fresh18 = op1;
            op1 = op1.offset(1);
            *fresh18 = HUF_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            let fresh19 = op2;
            op2 = op2.offset(1);
            *fresh19 = HUF_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            let fresh20 = op3;
            op3 = op3.offset(1);
            *fresh20 = HUF_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            let fresh21 = op4;
            op4 = op4.offset(1);
            *fresh21 = HUF_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh22 = op1;
            op1 = op1.offset(1);
            *fresh22 = HUF_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh23 = op2;
            op2 = op2.offset(1);
            *fresh23 = HUF_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh24 = op3;
            op3 = op3.offset(1);
            *fresh24 = HUF_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh25 = op4;
            op4 = op4.offset(1);
            *fresh25 = HUF_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh26 = op1;
        op1 = op1.offset(1);
        *fresh26 = HUF_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh27 = op2;
        op2 = op2.offset(1);
        *fresh27 = HUF_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh28 = op3;
        op3 = op3.offset(1);
        *fresh28 = HUF_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh29 = op4;
        op4 = op4.offset(1);
        *fresh29 = HUF_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = BIT_reloadDStream(&mut bitD1) as libc::c_uint
            | BIT_reloadDStream(&mut bitD2) as libc::c_uint
            | BIT_reloadDStream(&mut bitD3) as libc::c_uint
            | BIT_reloadDStream(&mut bitD4) as libc::c_uint;
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
    endSignal = BIT_endOfDStream(&mut bitD1) & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3) & BIT_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X2(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [libc::c_ushort; 4097] = [
        12 as libc::c_int as libc::c_ushort,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ip = cSrc as *const BYTE;
    let mut errorCode: size_t = 0;
    errorCode = HUF_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(errorCode) as size_t as size_t;
    return HUF_decompress4X2_usingDTable(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    );
}
unsafe extern "C" fn HUF_fillDTableX4Level2(
    mut DTable: *mut HUF_DEltX4,
    mut sizeLog: U32,
    consumed: U32,
    mut rankValOrigin: *const U32,
    minWeight: libc::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    sortedListSize: U32,
    mut nbBitsBaseline: U32,
    mut baseSeq: U16,
) {
    let mut DElt = HUF_DEltX4 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [U32; 17] = [0; 17];
    let mut s: U32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut libc::c_void,
        rankValOrigin as *const libc::c_void,
        ::core::mem::size_of::<[U32; 17]>() as libc::c_ulong,
    );
    if minWeight > 1 as libc::c_int {
        let mut i: U32 = 0;
        let mut skipSize = rankVal[minWeight as usize];
        MEM_writeLE16(&mut DElt.sequence as *mut U16 as *mut libc::c_void, baseSeq);
        DElt.nbBits = consumed as BYTE;
        DElt.length = 1 as libc::c_int as BYTE;
        i = 0 as libc::c_int as U32;
        while i < skipSize {
            *DTable.offset(i as isize) = DElt;
            i = i.wrapping_add(1);
        }
    }
    s = 0 as libc::c_int as U32;
    while s < sortedListSize {
        let symbol = (*sortedSymbols.offset(s as isize)).symbol as U32;
        let weight = (*sortedSymbols.offset(s as isize)).weight as U32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let length = ((1 as libc::c_int) << sizeLog.wrapping_sub(nbBits)) as U32;
        let start = rankVal[weight as usize];
        let mut i_0 = start;
        let end = start.wrapping_add(length);
        MEM_writeLE16(
            &mut DElt.sequence as *mut U16 as *mut libc::c_void,
            (baseSeq as libc::c_uint).wrapping_add(symbol << 8 as libc::c_int) as U16,
        );
        DElt.nbBits = nbBits.wrapping_add(consumed) as BYTE;
        DElt.length = 2 as libc::c_int as BYTE;
        loop {
            let fresh30 = i_0;
            i_0 = i_0.wrapping_add(1);
            *DTable.offset(fresh30 as isize) = DElt;
            if !(i_0 < end) {
                break;
            }
        }
        rankVal[weight
            as usize] = (rankVal[weight as usize] as libc::c_uint).wrapping_add(length)
            as U32 as U32;
        s = s.wrapping_add(1);
    }
}
unsafe extern "C" fn HUF_fillDTableX4(
    mut DTable: *mut HUF_DEltX4,
    targetLog: U32,
    mut sortedList: *const sortedSymbol_t,
    sortedListSize: U32,
    mut rankStart: *const U32,
    mut rankValOrigin: *mut [U32; 17],
    maxWeight: U32,
    nbBitsBaseline: U32,
) {
    let mut rankVal: [U32; 17] = [0; 17];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as libc::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: U32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut libc::c_void,
        rankValOrigin as *const libc::c_void,
        ::core::mem::size_of::<[U32; 17]>() as libc::c_ulong,
    );
    s = 0 as libc::c_int as U32;
    while s < sortedListSize {
        let symbol = (*sortedList.offset(s as isize)).symbol as U16;
        let weight = (*sortedList.offset(s as isize)).weight as U32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let start = rankVal[weight as usize];
        let length = ((1 as libc::c_int) << targetLog.wrapping_sub(nbBits)) as U32;
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut sortedRank: U32 = 0;
            let mut minWeight = nbBits.wrapping_add(scaleLog as libc::c_uint)
                as libc::c_int;
            if minWeight < 1 as libc::c_int {
                minWeight = 1 as libc::c_int;
            }
            sortedRank = *rankStart.offset(minWeight as isize);
            HUF_fillDTableX4Level2(
                DTable.offset(start as isize),
                targetLog.wrapping_sub(nbBits),
                nbBits,
                (*rankValOrigin.offset(nbBits as isize)).as_mut_ptr(),
                minWeight,
                sortedList.offset(sortedRank as isize),
                sortedListSize.wrapping_sub(sortedRank),
                nbBitsBaseline,
                symbol,
            );
        } else {
            let mut i: U32 = 0;
            let end = start.wrapping_add(length);
            let mut DElt = HUF_DEltX4 {
                sequence: 0,
                nbBits: 0,
                length: 0,
            };
            MEM_writeLE16(&mut DElt.sequence as *mut U16 as *mut libc::c_void, symbol);
            DElt.nbBits = nbBits as BYTE;
            DElt.length = 1 as libc::c_int as BYTE;
            i = start;
            while i < end {
                *DTable.offset(i as isize) = DElt;
                i = i.wrapping_add(1);
            }
        }
        rankVal[weight
            as usize] = (rankVal[weight as usize] as libc::c_uint).wrapping_add(length)
            as U32 as U32;
        s = s.wrapping_add(1);
    }
}
unsafe extern "C" fn HUF_readDTableX4(
    mut DTable: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightList: [BYTE; 256] = [0; 256];
    let mut sortedSymbol: [sortedSymbol_t; 256] = [sortedSymbol_t {
        symbol: 0,
        weight: 0,
    }; 256];
    let mut rankStats: [U32; 17] = [
        0 as libc::c_int as U32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut rankStart0: [U32; 18] = [
        0 as libc::c_int as U32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let rankStart = rankStart0.as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut rankVal: rankVal_t = [[0; 17]; 16];
    let mut tableLog: U32 = 0;
    let mut maxW: U32 = 0;
    let mut sizeOfSort: U32 = 0;
    let mut nbSymbols: U32 = 0;
    let memLog = *DTable.offset(0 as libc::c_int as isize);
    let mut ip = src as *const BYTE;
    let mut iSize = *ip.offset(0 as libc::c_int as isize) as size_t;
    let mut ptr = DTable as *mut libc::c_void;
    let dt = (ptr as *mut HUF_DEltX4).offset(1 as libc::c_int as isize);
    if memLog > HUF_ABSOLUTEMAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    iSize = HUF_readStats(
        weightList.as_mut_ptr(),
        (HUF_MAX_SYMBOL_VALUE + 1 as libc::c_int) as size_t,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUF_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > memLog {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    maxW = tableLog;
    while rankStats[maxW as usize] == 0 as libc::c_int as libc::c_uint {
        if maxW == 0 {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: U32 = 0;
    let mut nextRankStart = 0 as libc::c_int as U32;
    w = 1 as libc::c_int as U32;
    while w <= maxW {
        let mut current = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(rankStats[w as usize]) as U32 as U32;
        *rankStart.offset(w as isize) = current;
        w = w.wrapping_add(1);
    }
    *rankStart.offset(0 as libc::c_int as isize) = nextRankStart;
    sizeOfSort = nextRankStart;
    let mut s: U32 = 0;
    s = 0 as libc::c_int as U32;
    while s < nbSymbols {
        let mut w_0 = weightList[s as usize] as U32;
        let ref mut fresh31 = *rankStart.offset(w_0 as isize);
        let fresh32 = *fresh31;
        *fresh31 = (*fresh31).wrapping_add(1);
        let mut r = fresh32;
        sortedSymbol[r as usize].symbol = s as BYTE;
        sortedSymbol[r as usize].weight = w_0 as BYTE;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0 as libc::c_int as isize) = 0 as libc::c_int as U32;
    let minBits = tableLog
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_sub(maxW);
    let mut nextRankVal = 0 as libc::c_int as U32;
    let mut w_1: U32 = 0;
    let mut consumed: U32 = 0;
    let rescale = memLog
        .wrapping_sub(tableLog)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut rankVal0 = (rankVal[0 as libc::c_int as usize]).as_mut_ptr();
    w_1 = 1 as libc::c_int as U32;
    while w_1 <= maxW {
        let mut current_0 = nextRankVal;
        nextRankVal = (nextRankVal as libc::c_uint)
            .wrapping_add(
                rankStats[w_1 as usize] << w_1.wrapping_add(rescale as libc::c_uint),
            ) as U32 as U32;
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
    }
    consumed = minBits;
    while consumed <= memLog.wrapping_sub(minBits) {
        let mut rankValPtr = (rankVal[consumed as usize]).as_mut_ptr();
        w_1 = 1 as libc::c_int as U32;
        while w_1 <= maxW {
            *rankValPtr
                .offset(w_1 as isize) = *rankVal0.offset(w_1 as isize) >> consumed;
            w_1 = w_1.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUF_fillDTableX4(
        dt,
        memLog,
        sortedSymbol.as_mut_ptr(),
        sizeOfSort,
        rankStart0.as_mut_ptr(),
        rankVal.as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    return iSize;
}
unsafe extern "C" fn HUF_decodeSymbolX4(
    mut op: *mut libc::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX4,
    dtLog: U32,
) -> U32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    memcpy(
        op,
        dt.offset(val as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    return (*dt.offset(val as isize)).length as U32;
}
unsafe extern "C" fn HUF_decodeLastSymbolX4(
    mut op: *mut libc::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX4,
    dtLog: U32,
) -> U32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    memcpy(
        op,
        dt.offset(val as isize) as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
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
#[inline]
unsafe extern "C" fn HUF_decodeStreamX4(
    mut p: *mut BYTE,
    mut bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut BYTE,
    dt: *const HUF_DEltX4,
    dtLog: U32,
) -> size_t {
    let pStart = p;
    while BIT_reloadDStream(bitDPtr) as libc::c_uint
        == BIT_DStream_unfinished as libc::c_int as libc::c_uint
        && p < pEnd.offset(-(7 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            p = p
                .offset(
                    HUF_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            p = p
                .offset(
                    HUF_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            p = p
                .offset(
                    HUF_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        p = p
            .offset(
                HUF_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog) as isize,
            );
    }
    while BIT_reloadDStream(bitDPtr) as libc::c_uint
        == BIT_DStream_unfinished as libc::c_int as libc::c_uint
        && p <= pEnd.offset(-(2 as libc::c_int as isize))
    {
        p = p
            .offset(
                HUF_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog) as isize,
            );
    }
    while p <= pEnd.offset(-(2 as libc::c_int as isize)) {
        p = p
            .offset(
                HUF_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog) as isize,
            );
    }
    if p < pEnd {
        p = p
            .offset(
                HUF_decodeLastSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    return p.offset_from(pStart) as libc::c_long as size_t;
}
unsafe extern "C" fn HUF_decompress4X4_usingDTable(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U32,
) -> size_t {
    if cSrcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut ptr = DTable as *const libc::c_void;
    let dt = (ptr as *const HUF_DEltX4).offset(1 as libc::c_int as isize);
    let dtLog = *DTable.offset(0 as libc::c_int as isize);
    let mut errorCode: size_t = 0;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let length1 = MEM_readLE16(istart as *const libc::c_void) as size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let mut length4: size_t = 0;
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
    let mut endSignal: U32 = 0;
    length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6 as libc::c_int as libc::c_ulong),
        );
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    errorCode = BIT_initDStream(&mut bitD1, istart1 as *const libc::c_void, length1);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BIT_initDStream(&mut bitD2, istart2 as *const libc::c_void, length2);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BIT_initDStream(&mut bitD3, istart3 as *const libc::c_void, length3);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BIT_initDStream(&mut bitD4, istart4 as *const libc::c_void, length4);
    if HUF_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BIT_reloadDStream(&mut bitD1) as libc::c_uint
        | BIT_reloadDStream(&mut bitD2) as libc::c_uint
        | BIT_reloadDStream(&mut bitD3) as libc::c_uint
        | BIT_reloadDStream(&mut bitD4) as libc::c_uint;
    while endSignal == BIT_DStream_unfinished as libc::c_int as libc::c_uint
        && op4 < oend.offset(-(7 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            op1 = op1
                .offset(
                    HUF_decodeSymbolX4(op1 as *mut libc::c_void, &mut bitD1, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op2 = op2
                .offset(
                    HUF_decodeSymbolX4(op2 as *mut libc::c_void, &mut bitD2, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op3 = op3
                .offset(
                    HUF_decodeSymbolX4(op3 as *mut libc::c_void, &mut bitD3, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op4 = op4
                .offset(
                    HUF_decodeSymbolX4(op4 as *mut libc::c_void, &mut bitD4, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            op1 = op1
                .offset(
                    HUF_decodeSymbolX4(op1 as *mut libc::c_void, &mut bitD1, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            op2 = op2
                .offset(
                    HUF_decodeSymbolX4(op2 as *mut libc::c_void, &mut bitD2, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            op3 = op3
                .offset(
                    HUF_decodeSymbolX4(op3 as *mut libc::c_void, &mut bitD3, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 || HUF_MAX_TABLELOG <= 12 as libc::c_int {
            op4 = op4
                .offset(
                    HUF_decodeSymbolX4(op4 as *mut libc::c_void, &mut bitD4, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op1 = op1
                .offset(
                    HUF_decodeSymbolX4(op1 as *mut libc::c_void, &mut bitD1, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op2 = op2
                .offset(
                    HUF_decodeSymbolX4(op2 as *mut libc::c_void, &mut bitD2, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op3 = op3
                .offset(
                    HUF_decodeSymbolX4(op3 as *mut libc::c_void, &mut bitD3, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            op4 = op4
                .offset(
                    HUF_decodeSymbolX4(op4 as *mut libc::c_void, &mut bitD4, dt, dtLog)
                        as isize,
                );
        }
        op1 = op1
            .offset(
                HUF_decodeSymbolX4(op1 as *mut libc::c_void, &mut bitD1, dt, dtLog)
                    as isize,
            );
        op2 = op2
            .offset(
                HUF_decodeSymbolX4(op2 as *mut libc::c_void, &mut bitD2, dt, dtLog)
                    as isize,
            );
        op3 = op3
            .offset(
                HUF_decodeSymbolX4(op3 as *mut libc::c_void, &mut bitD3, dt, dtLog)
                    as isize,
            );
        op4 = op4
            .offset(
                HUF_decodeSymbolX4(op4 as *mut libc::c_void, &mut bitD4, dt, dtLog)
                    as isize,
            );
        endSignal = BIT_reloadDStream(&mut bitD1) as libc::c_uint
            | BIT_reloadDStream(&mut bitD2) as libc::c_uint
            | BIT_reloadDStream(&mut bitD3) as libc::c_uint
            | BIT_reloadDStream(&mut bitD4) as libc::c_uint;
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
    HUF_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BIT_endOfDStream(&mut bitD1) & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3) & BIT_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return dstSize;
}
unsafe extern "C" fn HUF_decompress4X4(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [libc::c_uint; 4097] = [
        12 as libc::c_int as libc::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ip = cSrc as *const BYTE;
    let mut hSize = HUF_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUF_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as size_t;
    return HUF_decompress4X4_usingDTable(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    );
}
static mut algoTime: [[algo_time_t; 3]; 16] = [
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
        {
            let mut init = algo_time_t {
                tableTime: 2 as libc::c_int as U32,
                decode256Time: 2 as libc::c_int as U32,
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
        {
            let mut init = algo_time_t {
                tableTime: 2 as libc::c_int as U32,
                decode256Time: 2 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 38 as libc::c_int as U32,
                decode256Time: 130 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1313 as libc::c_int as U32,
                decode256Time: 74 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2151 as libc::c_int as U32,
                decode256Time: 38 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 448 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1353 as libc::c_int as U32,
                decode256Time: 74 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2238 as libc::c_int as U32,
                decode256Time: 41 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 556 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1353 as libc::c_int as U32,
                decode256Time: 74 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2238 as libc::c_int as U32,
                decode256Time: 47 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 714 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1418 as libc::c_int as U32,
                decode256Time: 74 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2436 as libc::c_int as U32,
                decode256Time: 53 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 883 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1437 as libc::c_int as U32,
                decode256Time: 74 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2464 as libc::c_int as U32,
                decode256Time: 61 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 897 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1515 as libc::c_int as U32,
                decode256Time: 75 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2622 as libc::c_int as U32,
                decode256Time: 68 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 926 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1613 as libc::c_int as U32,
                decode256Time: 75 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2730 as libc::c_int as U32,
                decode256Time: 75 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 947 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1729 as libc::c_int as U32,
                decode256Time: 77 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 3359 as libc::c_int as U32,
                decode256Time: 77 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1107 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2083 as libc::c_int as U32,
                decode256Time: 81 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 4006 as libc::c_int as U32,
                decode256Time: 84 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1177 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2379 as libc::c_int as U32,
                decode256Time: 87 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 4785 as libc::c_int as U32,
                decode256Time: 88 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1242 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2415 as libc::c_int as U32,
                decode256Time: 93 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 5155 as libc::c_int as U32,
                decode256Time: 84 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1349 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2644 as libc::c_int as U32,
                decode256Time: 106 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 5260 as libc::c_int as U32,
                decode256Time: 106 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1455 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2422 as libc::c_int as U32,
                decode256Time: 124 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 4174 as libc::c_int as U32,
                decode256Time: 124 as libc::c_int as U32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 722 as libc::c_int as U32,
                decode256Time: 128 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1891 as libc::c_int as U32,
                decode256Time: 145 as libc::c_int as U32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1936 as libc::c_int as U32,
                decode256Time: 146 as libc::c_int as U32,
            };
            init
        },
    ],
];
unsafe extern "C" fn HUF_decompress(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    static mut decompress: [decompressionAlgo; 3] = unsafe {
        [
            Some(
                HUF_decompress4X2
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                HUF_decompress4X4
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            ::core::mem::transmute::<
                libc::intptr_t,
                decompressionAlgo,
            >(NULL as libc::intptr_t),
        ]
    };
    let mut Q: U32 = 0;
    let D256 = (dstSize >> 8 as libc::c_int) as U32;
    let mut Dtime: [U32; 3] = [0; 3];
    let mut algoNb = 0 as libc::c_int as U32;
    let mut n: libc::c_int = 0;
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize;
    }
    if cSrcSize == 1 as libc::c_int as libc::c_ulong {
        memset(dst, *(cSrc as *const BYTE) as libc::c_int, dstSize);
        return dstSize;
    }
    Q = cSrcSize.wrapping_mul(16 as libc::c_int as libc::c_ulong).wrapping_div(dstSize)
        as U32;
    n = 0 as libc::c_int;
    while n < 3 as libc::c_int {
        Dtime[n
            as usize] = (algoTime[Q as usize][n as usize].tableTime)
            .wrapping_add(
                (algoTime[Q as usize][n as usize].decode256Time).wrapping_mul(D256),
            );
        n += 1;
    }
    Dtime[1 as libc::c_int
        as usize] = (Dtime[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(Dtime[1 as libc::c_int as usize] >> 4 as libc::c_int) as U32
        as U32;
    Dtime[2 as libc::c_int
        as usize] = (Dtime[2 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(Dtime[2 as libc::c_int as usize] >> 3 as libc::c_int) as U32
        as U32;
    if Dtime[1 as libc::c_int as usize] < Dtime[0 as libc::c_int as usize] {
        algoNb = 1 as libc::c_int as U32;
    }
    return (decompress[algoNb as usize])
        .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize);
}
pub const BIT1: libc::c_int = 2;
pub const BIT0: libc::c_int = 1;
pub const BLOCKSIZE: libc::c_int = 128 as libc::c_int
    * ((1 as libc::c_int) << 10 as libc::c_int);
pub const MIN_SEQUENCES_SIZE: libc::c_int = 2 as libc::c_int + 2 as libc::c_int
    + 3 as libc::c_int + 1 as libc::c_int;
pub const MIN_CBLOCK_SIZE: libc::c_int = 3 as libc::c_int + MIN_SEQUENCES_SIZE;
pub const IS_RAW: libc::c_int = BIT0;
pub const IS_RLE: libc::c_int = BIT1;
pub const MINMATCH: libc::c_int = 4 as libc::c_int;
pub const MLbits: libc::c_int = 7 as libc::c_int;
pub const LLbits: libc::c_int = 6 as libc::c_int;
pub const Offbits: libc::c_int = 5 as libc::c_int;
pub const MaxML: libc::c_int = ((1 as libc::c_int) << MLbits) - 1 as libc::c_int;
pub const MaxLL: libc::c_int = ((1 as libc::c_int) << LLbits) - 1 as libc::c_int;
pub const MaxOff: libc::c_int = 31 as libc::c_int;
pub const MLFSELog: libc::c_int = 10 as libc::c_int;
pub const LLFSELog: libc::c_int = 10 as libc::c_int;
pub const OffFSELog: libc::c_int = 9 as libc::c_int;
pub const ZSTD_CONTENTSIZE_ERROR: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(2 as libc::c_int as libc::c_ulonglong);
static mut ZSTD_blockHeaderSize: size_t = 3 as libc::c_int as size_t;
static mut ZSTD_frameHeaderSize: size_t = 4 as libc::c_int as size_t;
unsafe extern "C" fn ZSTD_copy4(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    memcpy(dst, src, 4 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_copy8(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    memcpy(dst, src, 8 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut length: ptrdiff_t,
) {
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let oend = op.offset(length as isize);
    loop {
        ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
        op = op.offset(8 as libc::c_int as isize);
        ip = ip.offset(8 as libc::c_int as isize);
        if !(op < oend) {
            break;
        }
    };
}
unsafe extern "C" fn ZSTD_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
unsafe extern "C" fn ZSTD_getcBlockSize(
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const BYTE;
    let mut headerFlags: BYTE = 0;
    let mut cSize: U32 = 0;
    if srcSize < 3 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    headerFlags = *in_0;
    cSize = (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
        + ((*in_0.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        + ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int)
            << 16 as libc::c_int)) as U32;
    (*bpPtr).blockType = (headerFlags as libc::c_int >> 6 as libc::c_int) as blockType_t;
    (*bpPtr)
        .origSize = if (*bpPtr).blockType as libc::c_uint
        == bt_rle as libc::c_int as libc::c_uint
    {
        cSize
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if (*bpPtr).blockType as libc::c_uint == bt_end as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as size_t;
    }
    if (*bpPtr).blockType as libc::c_uint == bt_rle as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as size_t;
    }
    return cSize as size_t;
}
unsafe extern "C" fn ZSTD_copyUncompressedBlock(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize > maxDstSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if srcSize > 0 as libc::c_int as libc::c_ulong {
        memcpy(dst, src, srcSize);
    }
    return srcSize;
}
unsafe extern "C" fn ZSTD_decompressLiterals(
    mut dst: *mut libc::c_void,
    mut maxDstSizePtr: *mut size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    let litSize = ((MEM_readLE32(src) & 0x1fffff as libc::c_int as libc::c_uint)
        >> 2 as libc::c_int) as size_t;
    let litCSize = ((MEM_readLE32(
        ip.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) & 0xffffff as libc::c_int as libc::c_uint) >> 5 as libc::c_int) as size_t;
    if litSize > *maxDstSizePtr {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if litCSize.wrapping_add(5 as libc::c_int as libc::c_ulong) > srcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if HUF_isError(
        HUF_decompress(
            dst,
            litSize,
            ip.offset(5 as libc::c_int as isize) as *const libc::c_void,
            litCSize,
        ),
    ) != 0
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *maxDstSizePtr = litSize;
    return litCSize.wrapping_add(5 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_decodeLiteralsBlock(
    mut ctx: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut dctx = ctx as *mut ZSTD_DCtx;
    let istart = src as *const BYTE;
    if srcSize < MIN_CBLOCK_SIZE as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    match *istart as libc::c_int & 3 as libc::c_int {
        IS_RAW => {
            let litSize_0 = ((MEM_readLE32(istart as *const libc::c_void)
                & 0xffffff as libc::c_int as libc::c_uint) >> 2 as libc::c_int)
                as size_t;
            if litSize_0 > srcSize.wrapping_sub(11 as libc::c_int as libc::c_ulong) {
                if litSize_0 > BLOCKSIZE as libc::c_ulong {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
                }
                if litSize_0 > srcSize.wrapping_sub(3 as libc::c_int as libc::c_ulong) {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
                }
                memcpy(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                    istart as *const libc::c_void,
                    litSize_0,
                );
                (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
                (*dctx).litSize = litSize_0;
                memset(
                    ((*dctx).litBuffer).as_mut_ptr().offset((*dctx).litSize as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    8 as libc::c_int as libc::c_ulong,
                );
                return litSize_0.wrapping_add(3 as libc::c_int as libc::c_ulong);
            }
            (*dctx).litPtr = istart.offset(3 as libc::c_int as isize);
            (*dctx).litSize = litSize_0;
            return litSize_0.wrapping_add(3 as libc::c_int as libc::c_ulong);
        }
        IS_RLE => {
            let litSize_1 = ((MEM_readLE32(istart as *const libc::c_void)
                & 0xffffff as libc::c_int as libc::c_uint) >> 2 as libc::c_int)
                as size_t;
            if litSize_1 > BLOCKSIZE as libc::c_ulong {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            memset(
                ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                *istart.offset(3 as libc::c_int as isize) as libc::c_int,
                litSize_1.wrapping_add(8 as libc::c_int as libc::c_ulong),
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_1;
            return 4 as libc::c_int as size_t;
        }
        0 | _ => {
            let mut litSize = BLOCKSIZE as size_t;
            let readSize = ZSTD_decompressLiterals(
                ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                &mut litSize,
                src,
                srcSize,
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize;
            memset(
                ((*dctx).litBuffer).as_mut_ptr().offset((*dctx).litSize as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                8 as libc::c_int as libc::c_ulong,
            );
            return readSize;
        }
    };
}
unsafe extern "C" fn ZSTD_decodeSeqHeaders(
    mut nbSeq: *mut libc::c_int,
    mut dumpsPtr: *mut *const BYTE,
    mut dumpsLengthPtr: *mut size_t,
    mut DTableLL: *mut FSE_DTable,
    mut DTableML: *mut FSE_DTable,
    mut DTableOffb: *mut FSE_DTable,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const BYTE;
    let mut ip = istart;
    let iend = istart.offset(srcSize as isize);
    let mut LLtype: U32 = 0;
    let mut Offtype: U32 = 0;
    let mut MLtype: U32 = 0;
    let mut LLlog: U32 = 0;
    let mut Offlog: U32 = 0;
    let mut MLlog: U32 = 0;
    let mut dumpsLength: size_t = 0;
    if srcSize < 5 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    *nbSeq = MEM_readLE16(ip as *const libc::c_void) as libc::c_int;
    ip = ip.offset(2 as libc::c_int as isize);
    LLtype = (*ip as libc::c_int >> 6 as libc::c_int) as U32;
    Offtype = (*ip as libc::c_int >> 4 as libc::c_int & 3 as libc::c_int) as U32;
    MLtype = (*ip as libc::c_int >> 2 as libc::c_int & 3 as libc::c_int) as U32;
    if *ip as libc::c_int & 2 as libc::c_int != 0 {
        dumpsLength = *ip.offset(2 as libc::c_int as isize) as size_t;
        dumpsLength = (dumpsLength as libc::c_ulong)
            .wrapping_add(
                ((*ip.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
        ip = ip.offset(3 as libc::c_int as isize);
    } else {
        dumpsLength = *ip.offset(1 as libc::c_int as isize) as size_t;
        dumpsLength = (dumpsLength as libc::c_ulong)
            .wrapping_add(
                ((*ip.offset(0 as libc::c_int as isize) as libc::c_int
                    & 1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
        ip = ip.offset(2 as libc::c_int as isize);
    }
    *dumpsPtr = ip;
    ip = ip.offset(dumpsLength as isize);
    *dumpsLengthPtr = dumpsLength;
    if ip > iend.offset(-(3 as libc::c_int as isize)) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let mut norm: [S16; 128] = [0; 128];
    let mut headerSize: size_t = 0;
    match LLtype {
        2 => {
            LLlog = 0 as libc::c_int as U32;
            let fresh33 = ip;
            ip = ip.offset(1);
            FSE_buildDTable_rle(DTableLL, *fresh33);
        }
        1 => {
            LLlog = LLbits as U32;
            FSE_buildDTable_raw(DTableLL, LLbits as libc::c_uint);
        }
        _ => {
            let mut max = MaxLL as U32;
            headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut LLlog,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            );
            if FSE_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            if LLlog > LLFSELog as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSE_buildDTable(DTableLL, norm.as_mut_ptr(), max, LLlog);
        }
    }
    match Offtype {
        2 => {
            Offlog = 0 as libc::c_int as U32;
            if ip > iend.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            let fresh34 = ip;
            ip = ip.offset(1);
            FSE_buildDTable_rle(
                DTableOffb,
                (*fresh34 as libc::c_int & MaxOff) as libc::c_uchar,
            );
        }
        1 => {
            Offlog = Offbits as U32;
            FSE_buildDTable_raw(DTableOffb, Offbits as libc::c_uint);
        }
        _ => {
            let mut max_0 = MaxOff as U32;
            headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max_0,
                &mut Offlog,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            );
            if FSE_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            if Offlog > OffFSELog as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSE_buildDTable(DTableOffb, norm.as_mut_ptr(), max_0, Offlog);
        }
    }
    match MLtype {
        2 => {
            MLlog = 0 as libc::c_int as U32;
            if ip > iend.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            let fresh35 = ip;
            ip = ip.offset(1);
            FSE_buildDTable_rle(DTableML, *fresh35);
        }
        1 => {
            MLlog = MLbits as U32;
            FSE_buildDTable_raw(DTableML, MLbits as libc::c_uint);
        }
        _ => {
            let mut max_1 = MaxML as U32;
            headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max_1,
                &mut MLlog,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            );
            if FSE_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            if MLlog > MLFSELog as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSE_buildDTable(DTableML, norm.as_mut_ptr(), max_1, MLlog);
        }
    }
    return ip.offset_from(istart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decodeSequence(
    mut seq: *mut seq_t,
    mut seqState: *mut seqState_t,
) {
    let mut litLength: size_t = 0;
    let mut prevOffset: size_t = 0;
    let mut offset: size_t = 0;
    let mut matchLength: size_t = 0;
    let mut dumps = (*seqState).dumps;
    let de = (*seqState).dumpsEnd;
    litLength = FSE_decodeSymbol(&mut (*seqState).stateLL, &mut (*seqState).DStream)
        as size_t;
    prevOffset = if litLength != 0 { (*seq).offset } else { (*seqState).prevOffset };
    (*seqState).prevOffset = (*seq).offset;
    if litLength == MaxLL as libc::c_ulong {
        let add = (if dumps < de {
            let fresh36 = dumps;
            dumps = dumps.offset(1);
            *fresh36 as libc::c_int
        } else {
            0 as libc::c_int
        }) as U32;
        if add < 255 as libc::c_int as libc::c_uint {
            litLength = (litLength as libc::c_ulong).wrapping_add(add as libc::c_ulong)
                as size_t as size_t;
        } else if dumps.offset(3 as libc::c_int as isize) <= de {
            litLength = MEM_readLE24(dumps as *const libc::c_void) as size_t;
            dumps = dumps.offset(3 as libc::c_int as isize);
        }
        if dumps >= de {
            dumps = de.offset(-(1 as libc::c_int as isize));
        }
    }
    static mut offsetPrefix: [size_t; 32] = [
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        8 as libc::c_int as size_t,
        16 as libc::c_int as size_t,
        32 as libc::c_int as size_t,
        64 as libc::c_int as size_t,
        128 as libc::c_int as size_t,
        256 as libc::c_int as size_t,
        512 as libc::c_int as size_t,
        1024 as libc::c_int as size_t,
        2048 as libc::c_int as size_t,
        4096 as libc::c_int as size_t,
        8192 as libc::c_int as size_t,
        16384 as libc::c_int as size_t,
        32768 as libc::c_int as size_t,
        65536 as libc::c_int as size_t,
        131072 as libc::c_int as size_t,
        262144 as libc::c_int as size_t,
        524288 as libc::c_int as size_t,
        1048576 as libc::c_int as size_t,
        2097152 as libc::c_int as size_t,
        4194304 as libc::c_int as size_t,
        8388608 as libc::c_int as size_t,
        16777216 as libc::c_int as size_t,
        33554432 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    ];
    let mut offsetCode: U32 = 0;
    let mut nbBits: U32 = 0;
    offsetCode = FSE_decodeSymbol(&mut (*seqState).stateOffb, &mut (*seqState).DStream)
        as U32;
    if MEM_32bits() != 0 {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    nbBits = offsetCode.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if offsetCode == 0 as libc::c_int as libc::c_uint {
        nbBits = 0 as libc::c_int as U32;
    }
    offset = (offsetPrefix[offsetCode as usize])
        .wrapping_add(BIT_readBits(&mut (*seqState).DStream, nbBits));
    if MEM_32bits() != 0 {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if offsetCode == 0 as libc::c_int as libc::c_uint {
        offset = prevOffset;
    }
    matchLength = FSE_decodeSymbol(&mut (*seqState).stateML, &mut (*seqState).DStream)
        as size_t;
    if matchLength == MaxML as libc::c_ulong {
        let add_0 = (if dumps < de {
            let fresh37 = dumps;
            dumps = dumps.offset(1);
            *fresh37 as libc::c_int
        } else {
            0 as libc::c_int
        }) as U32;
        if add_0 < 255 as libc::c_int as libc::c_uint {
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(add_0 as libc::c_ulong) as size_t as size_t;
        } else if dumps.offset(3 as libc::c_int as isize) <= de {
            matchLength = MEM_readLE24(dumps as *const libc::c_void) as size_t;
            dumps = dumps.offset(3 as libc::c_int as isize);
        }
        if dumps >= de {
            dumps = de.offset(-(1 as libc::c_int as isize));
        }
    }
    matchLength = (matchLength as libc::c_ulong).wrapping_add(MINMATCH as libc::c_ulong)
        as size_t as size_t;
    (*seq).litLength = litLength;
    (*seq).offset = offset;
    (*seq).matchLength = matchLength;
    (*seqState).dumps = dumps;
}
unsafe extern "C" fn ZSTD_execSequence(
    mut op: *mut BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    base: *mut BYTE,
    oend: *mut BYTE,
) -> size_t {
    static mut dec32table: [libc::c_int; 8] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
    ];
    static mut dec64table: [libc::c_int; 8] = [
        8 as libc::c_int,
        8 as libc::c_int,
        8 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    let ostart: *const BYTE = op;
    let oLitEnd = op.offset(sequence.litLength as isize);
    let oMatchEnd = op
        .offset(sequence.litLength as isize)
        .offset(sequence.matchLength as isize);
    let oend_8 = oend.offset(-(8 as libc::c_int as isize));
    let litEnd = (*litPtr).offset(sequence.litLength as isize);
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if oLitEnd > oend_8 {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if sequence.offset
        > oLitEnd.offset_from(base) as libc::c_long as U32 as libc::c_ulong
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if litEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    ZSTD_wildcopy(
        op as *mut libc::c_void,
        *litPtr as *const libc::c_void,
        sequence.litLength as ptrdiff_t,
    );
    op = oLitEnd;
    *litPtr = litEnd;
    let mut match_0: *const BYTE = op.offset(-(sequence.offset as isize));
    if sequence.offset > op as size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if match_0 < base as *const BYTE {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if sequence.offset < 8 as libc::c_int as libc::c_ulong {
        let dec64 = dec64table[sequence.offset as usize];
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = *match_0.offset(0 as libc::c_int as isize);
        *op
            .offset(
                1 as libc::c_int as isize,
            ) = *match_0.offset(1 as libc::c_int as isize);
        *op
            .offset(
                2 as libc::c_int as isize,
            ) = *match_0.offset(2 as libc::c_int as isize);
        *op
            .offset(
                3 as libc::c_int as isize,
            ) = *match_0.offset(3 as libc::c_int as isize);
        match_0 = match_0.offset(dec32table[sequence.offset as usize] as isize);
        ZSTD_copy4(
            op.offset(4 as libc::c_int as isize) as *mut libc::c_void,
            match_0 as *const libc::c_void,
        );
        match_0 = match_0.offset(-(dec64 as isize));
    } else {
        ZSTD_copy8(op as *mut libc::c_void, match_0 as *const libc::c_void);
    }
    op = op.offset(8 as libc::c_int as isize);
    match_0 = match_0.offset(8 as libc::c_int as isize);
    if oMatchEnd > oend.offset(-((16 as libc::c_int - MINMATCH) as isize)) {
        if op < oend_8 {
            ZSTD_wildcopy(
                op as *mut libc::c_void,
                match_0 as *const libc::c_void,
                oend_8.offset_from(op) as libc::c_long,
            );
            match_0 = match_0.offset(oend_8.offset_from(op) as libc::c_long as isize);
            op = oend_8;
        }
        while op < oMatchEnd {
            let fresh38 = match_0;
            match_0 = match_0.offset(1);
            let fresh39 = op;
            op = op.offset(1);
            *fresh39 = *fresh38;
        }
    } else {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t - 8 as libc::c_int as libc::c_long,
        );
    }
    return oMatchEnd.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decompressSequences(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
) -> size_t {
    let mut dctx = ctx as *mut ZSTD_DCtx;
    let mut ip = seqStart as *const BYTE;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(maxDstSize as isize);
    let mut errorCode: size_t = 0;
    let mut dumpsLength: size_t = 0;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.offset((*dctx).litSize as isize);
    let mut nbSeq: libc::c_int = 0;
    let mut dumps = 0 as *const BYTE;
    let mut DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let mut DTableML = ((*dctx).MLTable).as_mut_ptr();
    let mut DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *mut BYTE;
    errorCode = ZSTD_decodeSeqHeaders(
        &mut nbSeq,
        &mut dumps,
        &mut dumpsLength,
        DTableLL,
        DTableML,
        DTableOffb,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
    );
    if ZSTD_isError(errorCode) != 0 {
        return errorCode;
    }
    ip = ip.offset(errorCode as isize);
    let mut sequence = seq_t {
        litLength: 0,
        offset: 0,
        matchLength: 0,
    };
    let mut seqState = seqState_t {
        DStream: BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: 0 as *const libc::c_char,
            start: 0 as *const libc::c_char,
        },
        stateLL: FSE_DState_t {
            state: 0,
            table: 0 as *const libc::c_void,
        },
        stateOffb: FSE_DState_t {
            state: 0,
            table: 0 as *const libc::c_void,
        },
        stateML: FSE_DState_t {
            state: 0,
            table: 0 as *const libc::c_void,
        },
        prevOffset: 0,
        dumps: 0 as *const BYTE,
        dumpsEnd: 0 as *const BYTE,
    };
    memset(
        &mut sequence as *mut seq_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<seq_t>() as libc::c_ulong,
    );
    seqState.dumps = dumps;
    seqState.dumpsEnd = dumps.offset(dumpsLength as isize);
    sequence.offset = 4 as libc::c_int as size_t;
    seqState.prevOffset = sequence.offset;
    errorCode = BIT_initDStream(
        &mut seqState.DStream,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
    );
    if ERR_isError(errorCode) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    FSE_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
    FSE_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
    FSE_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
    while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint
        <= BIT_DStream_completed as libc::c_int as libc::c_uint
        && nbSeq > 0 as libc::c_int
    {
        let mut oneSeqSize: size_t = 0;
        nbSeq -= 1;
        ZSTD_decodeSequence(&mut sequence, &mut seqState);
        oneSeqSize = ZSTD_execSequence(op, sequence, &mut litPtr, litEnd, base, oend);
        if ZSTD_isError(oneSeqSize) != 0 {
            return oneSeqSize;
        }
        op = op.offset(oneSeqSize as isize);
    }
    if BIT_endOfDStream(&mut seqState.DStream) == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if nbSeq < 0 as libc::c_int {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let mut lastLLSize = litEnd.offset_from(litPtr) as libc::c_long as size_t;
    if litPtr > litEnd {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op.offset(lastLLSize as isize) > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if lastLLSize > 0 as libc::c_int as libc::c_ulong {
        if op != litPtr as *mut BYTE {
            memmove(op as *mut libc::c_void, litPtr as *const libc::c_void, lastLLSize);
        }
        op = op.offset(lastLLSize as isize);
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decompressBlock(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    let mut litCSize = ZSTD_decodeLiteralsBlock(ctx, src, srcSize);
    if ZSTD_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(litCSize) as size_t as size_t;
    return ZSTD_decompressSequences(
        ctx,
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        srcSize,
    );
}
unsafe extern "C" fn ZSTD_decompressDCtx(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    let mut iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(maxDstSize as isize);
    let mut remainingSize = srcSize;
    let mut magicNumber: U32 = 0;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if srcSize < ZSTD_frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    magicNumber = MEM_readLE32(src);
    if magicNumber != ZSTD_magicNumber {
        return -(ZSTD_error_prefix_unknown as libc::c_int) as size_t;
    }
    ip = ip.offset(ZSTD_frameHeaderSize as isize);
    remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(ZSTD_frameHeaderSize)
        as size_t as size_t;
    loop {
        let mut decodedSize = 0 as libc::c_int as size_t;
        let mut cBlockSize = ZSTD_getcBlockSize(
            ip as *const libc::c_void,
            iend.offset_from(ip) as libc::c_long as size_t,
            &mut blockProperties,
        );
        if ZSTD_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.offset(ZSTD_blockHeaderSize as isize);
        remainingSize = (remainingSize as libc::c_ulong)
            .wrapping_sub(ZSTD_blockHeaderSize) as size_t as size_t;
        if cBlockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
        match blockProperties.blockType as libc::c_uint {
            0 => {
                decodedSize = ZSTD_decompressBlock(
                    ctx,
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as size_t,
                    ip as *const libc::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTD_copyUncompressedBlock(
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as size_t,
                    ip as *const libc::c_void,
                    cBlockSize,
                );
            }
            2 => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
                }
            }
            _ => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
        }
        if cBlockSize == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if ZSTD_isError(decodedSize) != 0 {
            return decodedSize;
        }
        op = op.offset(decodedSize as isize);
        ip = ip.offset(cBlockSize as isize);
        remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(cBlockSize)
            as size_t as size_t;
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decompress(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ctx = ZSTD_DCtx {
        LLTable: [0; 1025],
        OffTable: [0; 513],
        MLTable: [0; 1025],
        previousDstEnd: 0 as *mut libc::c_void,
        base: 0 as *mut libc::c_void,
        expected: 0,
        bType: bt_compressed,
        phase: 0,
        litPtr: 0 as *const BYTE,
        litSize: 0,
        litBuffer: [0; 131080],
    };
    ctx.base = dst;
    return ZSTD_decompressDCtx(
        &mut ctx as *mut ZSTD_DCtx as *mut libc::c_void,
        dst,
        maxDstSize,
        src,
        srcSize,
    );
}
unsafe extern "C" fn ZSTD_errorFrameSizeInfoLegacy(
    mut cSize: *mut size_t,
    mut dBound: *mut libc::c_ulonglong,
    mut ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_findFrameSizeInfoLegacy(
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut cSize: *mut size_t,
    mut dBound: *mut libc::c_ulonglong,
) {
    let mut ip = src as *const BYTE;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as libc::c_int as size_t;
    let mut magicNumber: U32 = 0;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if srcSize < ZSTD_frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t,
        );
        return;
    }
    magicNumber = MEM_readLE32(src);
    if magicNumber != ZSTD_magicNumber {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as libc::c_int) as size_t,
        );
        return;
    }
    ip = ip.offset(ZSTD_frameHeaderSize as isize);
    remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(ZSTD_frameHeaderSize)
        as size_t as size_t;
    loop {
        let mut cBlockSize = ZSTD_getcBlockSize(
            ip as *const libc::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ZSTD_isError(cBlockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.offset(ZSTD_blockHeaderSize as isize);
        remainingSize = (remainingSize as libc::c_ulong)
            .wrapping_sub(ZSTD_blockHeaderSize) as size_t as size_t;
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t,
            );
            return;
        }
        if cBlockSize == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        ip = ip.offset(cBlockSize as isize);
        remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(cBlockSize)
            as size_t as size_t;
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const BYTE) as libc::c_long as size_t;
    *dBound = nbBlocks.wrapping_mul(BLOCKSIZE as libc::c_ulong) as libc::c_ulonglong;
}
unsafe extern "C" fn ZSTD_resetDCtx(mut dctx: *mut ZSTD_DCtx) -> size_t {
    (*dctx).expected = ZSTD_frameHeaderSize;
    (*dctx).phase = 0 as libc::c_int as U32;
    (*dctx).previousDstEnd = NULL as *mut libc::c_void;
    (*dctx).base = NULL as *mut libc::c_void;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    let mut dctx = malloc(::core::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong)
        as *mut ZSTD_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTD_DCtx;
    }
    ZSTD_resetDCtx(dctx);
    return dctx;
}
unsafe extern "C" fn ZSTD_freeDCtx(mut dctx: *mut ZSTD_DCtx) -> size_t {
    free(dctx as *mut libc::c_void);
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ZSTD_nextSrcSizeToDecompress(mut dctx: *mut ZSTD_DCtx) -> size_t {
    return (*dctx).expected;
}
unsafe extern "C" fn ZSTD_decompressContinue(
    mut ctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize != (*ctx).expected {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if dst != (*ctx).previousDstEnd {
        (*ctx).base = dst;
    }
    if (*ctx).phase == 0 as libc::c_int as libc::c_uint {
        let mut magicNumber = MEM_readLE32(src);
        if magicNumber != ZSTD_magicNumber {
            return -(ZSTD_error_prefix_unknown as libc::c_int) as size_t;
        }
        (*ctx).phase = 1 as libc::c_int as U32;
        (*ctx).expected = ZSTD_blockHeaderSize;
        return 0 as libc::c_int as size_t;
    }
    if (*ctx).phase == 1 as libc::c_int as libc::c_uint {
        let mut bp = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let mut blockSize = ZSTD_getcBlockSize(src, ZSTD_blockHeaderSize, &mut bp);
        if ZSTD_isError(blockSize) != 0 {
            return blockSize;
        }
        if bp.blockType as libc::c_uint == bt_end as libc::c_int as libc::c_uint {
            (*ctx).expected = 0 as libc::c_int as size_t;
            (*ctx).phase = 0 as libc::c_int as U32;
        } else {
            (*ctx).expected = blockSize;
            (*ctx).bType = bp.blockType;
            (*ctx).phase = 2 as libc::c_int as U32;
        }
        return 0 as libc::c_int as size_t;
    }
    let mut rSize: size_t = 0;
    match (*ctx).bType as libc::c_uint {
        0 => {
            rSize = ZSTD_decompressBlock(
                ctx as *mut libc::c_void,
                dst,
                maxDstSize,
                src,
                srcSize,
            );
        }
        1 => {
            rSize = ZSTD_copyUncompressedBlock(dst, maxDstSize, src, srcSize);
        }
        2 => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
        3 => {
            rSize = 0 as libc::c_int as size_t;
        }
        _ => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
    }
    (*ctx).phase = 1 as libc::c_int as U32;
    (*ctx).expected = ZSTD_blockHeaderSize;
    (*ctx)
        .previousDstEnd = (dst as *mut libc::c_char).offset(rSize as isize)
        as *mut libc::c_void;
    return rSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_isError(mut code: size_t) -> libc::c_uint {
    return ZSTD_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_decompress(
    mut dst: *mut libc::c_void,
    mut maxOriginalSize: size_t,
    mut src: *const libc::c_void,
    mut compressedSize: size_t,
) -> size_t {
    return ZSTD_decompress(dst, maxOriginalSize, src, compressedSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_createDCtx() -> *mut ZSTDv03_Dctx {
    return ZSTD_createDCtx() as *mut ZSTDv03_Dctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_freeDCtx(mut dctx: *mut ZSTDv03_Dctx) -> size_t {
    return ZSTD_freeDCtx(dctx as *mut ZSTD_DCtx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_resetDCtx(mut dctx: *mut ZSTDv03_Dctx) -> size_t {
    return ZSTD_resetDCtx(dctx as *mut ZSTD_DCtx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_nextSrcSizeToDecompress(
    mut dctx: *mut ZSTDv03_Dctx,
) -> size_t {
    return ZSTD_nextSrcSizeToDecompress(dctx as *mut ZSTD_DCtx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv03_decompressContinue(
    mut dctx: *mut ZSTDv03_Dctx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_decompressContinue(
        dctx as *mut ZSTD_DCtx,
        dst,
        maxDstSize,
        src,
        srcSize,
    );
}
