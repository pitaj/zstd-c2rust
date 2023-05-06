use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ZSTD_XXH64_reset(
        statePtr: *mut XXH64_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const libc::c_void,
        length: libc::size_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
}
pub type ptrdiff_t = libc::c_long;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type XXH32_hash_t = u32;
pub type XXH64_hash_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_frameParams {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
}
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: u32,
    pub c: [u8; 4],
}
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub type ZSTDv07_DCtx = ZSTDv07_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_DCtx_s {
    pub LLTable: [FSEv07_DTable; 513],
    pub OffTable: [FSEv07_DTable; 257],
    pub MLTable: [FSEv07_DTable; 513],
    pub hufTable: [HUFv07_DTable; 4097],
    pub previousDstEnd: *const libc::c_void,
    pub base: *const libc::c_void,
    pub vBase: *const libc::c_void,
    pub dictEnd: *const libc::c_void,
    pub expected: libc::size_t,
    pub rep: [u32; 3],
    pub fParams: ZSTDv07_frameParams,
    pub bType: blockType_t,
    pub stage: ZSTDv07_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: libc::size_t,
    pub dictID: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTDv07_customMem,
    pub litSize: libc::size_t,
    pub litBuffer: [u8; 131080],
    pub headerBuffer: [u8; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_customMem {
    pub customAlloc: ZSTDv07_allocFunction,
    pub customFree: ZSTDv07_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTDv07_freeFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type ZSTDv07_allocFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::size_t) -> *mut libc::c_void,
>;
pub type ZSTDv07_dStage = libc::c_uint;
pub const ZSTDds_skipFrame: ZSTDv07_dStage = 5;
pub const ZSTDds_decodeSkippableHeader: ZSTDv07_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTDv07_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTDv07_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTDv07_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTDv07_dStage = 0;
pub type blockType_t = libc::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
pub type HUFv07_DTable = u32;
pub type FSEv07_DTable = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_t,
    pub origSize: u32,
}
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BITv07_DStream_t,
    pub stateLL: FSEv07_DState_t,
    pub stateOffb: FSEv07_DState_t,
    pub stateML: FSEv07_DState_t,
    pub prevOffset: [libc::size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv07_DState_t {
    pub state: libc::size_t,
    pub table: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITv07_DStream_t {
    pub bitContainer: libc::size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
}
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: libc::size_t,
    pub matchLength: libc::size_t,
    pub offset: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv07_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type BITv07_DStream_status = libc::c_uint;
pub const BITv07_DStream_overflow: BITv07_DStream_status = 3;
pub const BITv07_DStream_completed: BITv07_DStream_status = 2;
pub const BITv07_DStream_endOfBuffer: BITv07_DStream_status = 1;
pub const BITv07_DStream_unfinished: BITv07_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv07_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
}
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const lbt_rle: litBlockType_t = 3;
pub const lbt_raw: litBlockType_t = 2;
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
pub struct HUFv07_DEltX4 {
    pub sequence: u16,
    pub nbBits: u8,
    pub length: u8,
}
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub const lbt_repeat: litBlockType_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv07_DEltX2 {
    pub byte: u8,
    pub nbBits: u8,
}
pub type DTable_max_t = [u32; 4097];
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HUFv07_static_assert: C2RustUnnamed_0 = 1;
pub type rankVal_t = [[u32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
    pub weight: u8,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const HUFv07_static_assert_0: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: u32,
    pub decode256Time: u32,
}
pub const lbt_huffman: litBlockType_t = 0;
pub type litBlockType_t = libc::c_uint;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub type ZSTD_ErrorCode = ERR_enum;
pub type ERR_enum = libc::c_uint;
pub const ZSTD_error_externalSequences_invalid: ERR_enum = 107;
pub const ZSTD_error_sequenceProducer_failed: ERR_enum = 106;
pub const ZSTD_error_srcBuffer_wrong: ERR_enum = 105;
pub const ZSTD_error_dstBuffer_wrong: ERR_enum = 104;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ERR_enum = 82;
pub const ZSTD_error_noForwardProgress_destFull: ERR_enum = 80;
pub const ZSTD_error_dstBuffer_null: ERR_enum = 74;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ERR_enum = 50;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_parameter_combination_unsupported: ERR_enum = 41;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const ZSTD_error_literals_headerWrong: ERR_enum = 24;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const ZSTD_error_no_error: ERR_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_DDict_s {
    pub dict: *mut libc::c_void,
    pub dictSize: libc::size_t,
    pub refContext: *mut ZSTDv07_DCtx,
}
pub type ZSTDv07_DDict = ZSTDv07_DDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZBUFFv07_DCtx_s {
    pub zd: *mut ZSTDv07_DCtx,
    pub fParams: ZSTDv07_frameParams,
    pub stage: ZBUFFv07_dStage,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: libc::size_t,
    pub inPos: libc::size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: libc::size_t,
    pub outStart: libc::size_t,
    pub outEnd: libc::size_t,
    pub blockSize: libc::size_t,
    pub headerBuffer: [u8; 18],
    pub lhSize: libc::size_t,
    pub customMem: ZSTDv07_customMem,
}
pub type ZBUFFv07_dStage = libc::c_uint;
pub const ZBUFFds_flush: ZBUFFv07_dStage = 4;
pub const ZBUFFds_load: ZBUFFv07_dStage = 3;
pub const ZBUFFds_read: ZBUFFv07_dStage = 2;
pub const ZBUFFds_loadHeader: ZBUFFv07_dStage = 1;
pub const ZBUFFds_init: ZBUFFv07_dStage = 0;
pub type ZBUFFv07_DCtx = ZBUFFv07_DCtx_s;
pub type decompressionAlgo = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::size_t,
        *const libc::c_void,
        libc::size_t,
    ) -> libc::size_t,
>;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZSTDv07_MAGICNUMBER: libc::c_uint = 0xfd2fb527 as libc::c_uint;
pub const __ASSERT_FUNCTION: [libc::c_char; 147] = unsafe {
    *::core::mem::transmute::<
        &[u8; 147],
        &[libc::c_char; 147],
    >(
        b"libc::size_t ZSTDv07_execSequence(u8 *, u8 *const, seq_t, const u8 **, const u8 *const, const u8 *const, const u8 *const, const u8 *const)\0",
    )
};
unsafe extern "C" fn ERR_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
unsafe extern "C" fn ERR_getErrorCode(mut code: libc::size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    return (0 as libc::c_int as libc::c_ulong).wrapping_sub(code) as ERR_enum;
}
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
pub const ZSTDv07_MAGIC_SKIPPABLE_START: libc::c_uint = 0x184d2a50 as libc::c_uint;
pub const ZSTDv07_WINDOWLOG_MAX_32: libc::c_int = 25 as libc::c_int;
pub const ZSTDv07_WINDOWLOG_MAX_64: libc::c_int = 27 as libc::c_int;
pub const ZSTDv07_FRAMEHEADERSIZE_MAX: libc::c_int = 18 as libc::c_int;
static mut ZSTDv07_frameHeaderSize_min: libc::size_t = 5 as libc::c_int as libc::size_t;
static mut ZSTDv07_frameHeaderSize_max: libc::size_t = ZSTDv07_FRAMEHEADERSIZE_MAX as libc::size_t;
static mut ZSTDv07_skippableHeaderSize: libc::size_t = 8 as libc::c_int as libc::size_t;
pub const ZSTDv07_BLOCKSIZE_ABSOLUTEMAX: libc::c_int = 128 as libc::c_int
    * 1024 as libc::c_int;
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    let one = C2RustUnnamed {
        u: 1 as libc::c_int as u32,
    };
    return one.c[0 as libc::c_int as usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read16(mut memPtr: *const libc::c_void) -> u16 {
    let mut val: u16 = 0;
    memcpy(
        &mut val as *mut u16 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<u16>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn MEM_read32(mut memPtr: *const libc::c_void) -> u32 {
    let mut val: u32 = 0;
    memcpy(
        &mut val as *mut u32 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<u32>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn MEM_read64(mut memPtr: *const libc::c_void) -> u64 {
    let mut val: u64 = 0;
    memcpy(
        &mut val as *mut u64 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<u64>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: u16) {
    memcpy(
        memPtr,
        &mut value as *mut u16 as *const libc::c_void,
        ::core::mem::size_of::<u16>() as libc::c_ulong,
    );
}
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0 << 24 as libc::c_int & 0xff000000 as libc::c_uint
        | in_0 << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
        | in_0 >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | in_0 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    return ((in_0 << 56 as libc::c_int) as libc::c_ulonglong
        & 0xff00000000000000 as libc::c_ulonglong
        | (in_0 << 40 as libc::c_int) as libc::c_ulonglong
            & 0xff000000000000 as libc::c_ulonglong
        | (in_0 << 24 as libc::c_int) as libc::c_ulonglong
            & 0xff0000000000 as libc::c_ulonglong
        | (in_0 << 8 as libc::c_int) as libc::c_ulonglong
            & 0xff00000000 as libc::c_ulonglong
        | (in_0 >> 8 as libc::c_int) as libc::c_ulonglong
            & 0xff000000 as libc::c_ulonglong
        | (in_0 >> 24 as libc::c_int) as libc::c_ulonglong
            & 0xff0000 as libc::c_ulonglong
        | (in_0 >> 40 as libc::c_int) as libc::c_ulonglong & 0xff00 as libc::c_ulonglong
        | (in_0 >> 56 as libc::c_int) as libc::c_ulonglong & 0xff as libc::c_ulonglong)
        as u64;
}
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> u16 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const u8;
        return (*p.offset(0 as libc::c_int as isize) as libc::c_int
            + ((*p.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as u16;
    };
}
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut u8;
        *p.offset(0 as libc::c_int as isize) = val as u8;
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (val as libc::c_int >> 8 as libc::c_int) as u8;
    };
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> u32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr)
    } else {
        return MEM_swap32(MEM_read32(memPtr))
    };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> u64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr)
    } else {
        return MEM_swap64(MEM_read64(memPtr))
    };
}
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> libc::size_t {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as libc::size_t
    } else {
        return MEM_readLE64(memPtr)
    };
}
unsafe extern "C" fn BITv07_highbit32(mut val: u32) -> libc::c_uint {
    return (val.leading_zeros() as i32 ^ 31 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn BITv07_initDStream(
    mut bitD: *mut BITv07_DStream_t,
    mut srcBuffer: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    if srcSize < 1 as libc::c_int as libc::c_ulong {
        memset(
            bitD as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<BITv07_DStream_t>() as libc::c_ulong,
        );
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    if srcSize >= ::core::mem::size_of::<libc::size_t>() as libc::c_ulong {
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD)
            .ptr = (srcBuffer as *const libc::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<libc::size_t>() as libc::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        let lastByte = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte as libc::c_int != 0 {
            (8 as libc::c_int as libc::c_uint)
                .wrapping_sub(BITv07_highbit32(lastByte as u32))
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if lastByte as libc::c_int == 0 as libc::c_int {
            return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
        }
    } else {
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as libc::size_t;
        let mut current_block_20: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(6 as libc::c_int as isize)
                            as libc::size_t)
                            << (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(16),
                    ) ;
                current_block_20 = 8561046336891731757;
            }
            6 => {
                current_block_20 = 8561046336891731757;
            }
            5 => {
                current_block_20 = 7783095582258604228;
            }
            4 => {
                current_block_20 = 15690878105716328156;
            }
            3 => {
                current_block_20 = 4388566298172193598;
            }
            2 => {
                current_block_20 = 16942010770580489165;
            }
            _ => {
                current_block_20 = 5689001924483802034;
            }
        }
        match current_block_20 {
            8561046336891731757 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(5 as libc::c_int as isize)
                            as libc::size_t)
                            << (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(24),
                    ) ;
                current_block_20 = 7783095582258604228;
            }
            _ => {}
        }
        match current_block_20 {
            7783095582258604228 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(4 as libc::c_int as isize)
                            as libc::size_t)
                            << (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(32),
                    ) ;
                current_block_20 = 15690878105716328156;
            }
            _ => {}
        }
        match current_block_20 {
            15690878105716328156 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(3 as libc::c_int as isize)
                            as libc::size_t) << 24 as libc::c_int,
                    ) ;
                current_block_20 = 4388566298172193598;
            }
            _ => {}
        }
        match current_block_20 {
            4388566298172193598 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(2 as libc::c_int as isize)
                            as libc::size_t) << 16 as libc::c_int,
                    ) ;
                current_block_20 = 16942010770580489165;
            }
            _ => {}
        }
        match current_block_20 {
            16942010770580489165 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(1 as libc::c_int as isize)
                            as libc::size_t) << 8 as libc::c_int,
                    ) ;
            }
            _ => {}
        }
        let lastByte_0 = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte_0 as libc::c_int != 0 {
            (8 as libc::c_int as libc::c_uint)
                .wrapping_sub(BITv07_highbit32(lastByte_0 as u32))
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if lastByte_0 as libc::c_int == 0 as libc::c_int {
            return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
        }
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                ((::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                    .wrapping_sub(srcSize) as u32)
                    .wrapping_mul(8),
            );
    }
    return srcSize;
}
unsafe extern "C" fn BITv07_lookBits(
    mut bitD: *const BITv07_DStream_t,
    mut nbBits: u32,
) -> libc::size_t {
    let bitMask = (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask) >> 1 as libc::c_int
        >> (bitMask.wrapping_sub(nbBits) & bitMask);
}
unsafe extern "C" fn BITv07_lookBitsFast(
    mut bitD: *const BITv07_DStream_t,
    mut nbBits: u32,
) -> libc::size_t {
    let bitMask = (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1).wrapping_sub(nbBits)
            & bitMask);
}
unsafe extern "C" fn BITv07_skipBits(mut bitD: *mut BITv07_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
unsafe extern "C" fn BITv07_readBits(
    mut bitD: *mut BITv07_DStream_t,
    mut nbBits: u32,
) -> libc::size_t {
    let value = BITv07_lookBits(bitD, nbBits);
    BITv07_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BITv07_readBitsFast(
    mut bitD: *mut BITv07_DStream_t,
    mut nbBits: u32,
) -> libc::size_t {
    let value = BITv07_lookBitsFast(bitD, nbBits);
    BITv07_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BITv07_reloadDStream(
    mut bitD: *mut BITv07_DStream_t,
) -> BITv07_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong
        > (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
            .wrapping_mul(8)
    {
        return BITv07_DStream_overflow;
    }
    if (*bitD).ptr
        >= ((*bitD).start)
            .offset(::core::mem::size_of::<libc::size_t>() as libc::c_ulong as isize)
    {
        (*bitD)
            .ptr = ((*bitD).ptr)
            .offset(-(((*bitD).bitsConsumed >> 3 as libc::c_int) as isize));
        (*bitD).bitsConsumed &= 7 as libc::c_int as libc::c_uint;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BITv07_DStream_unfinished;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong)
            < (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8)
        {
            return BITv07_DStream_endOfBuffer;
        }
        return BITv07_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as libc::c_int;
    let mut result = BITv07_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as libc::c_long as u32;
        result = BITv07_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD)
        .bitsConsumed = ((*bitD).bitsConsumed)
        .wrapping_sub(nbBytes.wrapping_mul(8));
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return result;
}
unsafe extern "C" fn BITv07_endOfDStream(
    mut DStream: *const BITv07_DStream_t,
) -> libc::c_uint {
    return ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as libc::c_ulong
            == (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8)) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn FSEv07_initDState(
    mut DStatePtr: *mut FSEv07_DState_t,
    mut bitD: *mut BITv07_DStream_t,
    mut dt: *const FSEv07_DTable,
) {
    let mut ptr = dt as *const libc::c_void;
    let DTableH = ptr as *const FSEv07_DTableHeader;
    (*DStatePtr).state = BITv07_readBits(bitD, (*DTableH).tableLog as libc::c_uint);
    BITv07_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as libc::c_int as isize) as *const libc::c_void;
}
unsafe extern "C" fn FSEv07_peekSymbol(mut DStatePtr: *const FSEv07_DState_t) -> u8 {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t)
        .offset((*DStatePtr).state as isize);
    return DInfo.symbol;
}
unsafe extern "C" fn FSEv07_updateState(
    mut DStatePtr: *mut FSEv07_DState_t,
    mut bitD: *mut BITv07_DStream_t,
) {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let lowBits = BITv07_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
}
unsafe extern "C" fn FSEv07_decodeSymbol(
    mut DStatePtr: *mut FSEv07_DState_t,
    mut bitD: *mut BITv07_DStream_t,
) -> libc::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv07_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSEv07_decodeSymbolFast(
    mut DStatePtr: *mut FSEv07_DState_t,
    mut bitD: *mut BITv07_DStream_t,
) -> libc::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv07_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
pub const FSEv07_MAX_MEMORY_USAGE: libc::c_int = 14 as libc::c_int;
pub const FSEv07_MAX_SYMBOL_VALUE: libc::c_int = 255 as libc::c_int;
pub const FSEv07_MAX_TABLELOG: libc::c_int = FSEv07_MAX_MEMORY_USAGE - 2 as libc::c_int;
pub const FSEv07_MIN_TABLELOG: libc::c_int = 5 as libc::c_int;
pub const FSEv07_TABLELOG_ABSOLUTE_MAX: libc::c_int = 15 as libc::c_int;
pub const HUFv07_TABLELOG_ABSOLUTEMAX: libc::c_int = 16 as libc::c_int;
pub const HUFv07_TABLELOG_MAX: libc::c_int = 12 as libc::c_int;
pub const HUFv07_SYMBOLVALUE_MAX: libc::c_int = 255 as libc::c_int;
#[export_name = "FSEv07_isError"]
pub unsafe extern "C" fn FSEv07_isError_0(mut code: libc::size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_isError(mut code: libc::size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
unsafe extern "C" fn FSEv07_abs(mut a: libc::c_short) -> libc::c_short {
    return (if (a as libc::c_int) < 0 as libc::c_int {
        -(a as libc::c_int)
    } else {
        a as libc::c_int
    }) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_readNCount(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: libc::size_t,
) -> libc::size_t {
    let istart = headerBuffer as *const u8;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: libc::c_int = 0;
    let mut charnum = 0 as libc::c_int as libc::c_uint;
    let mut previous0 = 0 as libc::c_int;
    if hbSize < 4 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    bitStream = MEM_readLE32(ip as *const libc::c_void);
    nbBits = (bitStream & 0xf as libc::c_int as libc::c_uint)
        .wrapping_add(FSEv07_MIN_TABLELOG as libc::c_uint) as libc::c_int;
    if nbBits > FSEv07_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
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
                n0 = n0.wrapping_add(24);
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
                n0 = n0.wrapping_add(3);
                bitStream >>= 2 as libc::c_int;
                bitCount += 2 as libc::c_int;
            }
            n0 = n0.wrapping_add(bitStream & 3 as libc::c_int as libc::c_uint);
            bitCount += 2 as libc::c_int;
            if n0 > *maxSVPtr {
                return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as libc::size_t;
            }
            while charnum < n0 {
                let fresh0 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter
                    .offset(fresh0 as isize) = 0 as libc::c_int as libc::c_short;
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
        if (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint) < max as u32 {
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
        remaining -= FSEv07_abs(count) as libc::c_int;
        let fresh1 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh1 as isize) = count;
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
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset((bitCount + 7 as libc::c_int >> 3 as libc::c_int) as isize);
    if ip.offset_from(istart) as libc::c_long as libc::size_t > hbSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    return ip.offset_from(istart) as libc::c_long as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_readStats(
    mut huffWeight: *mut u8,
    mut hwSize: libc::size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut weightTotal: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: libc::size_t = 0;
    let mut oSize: libc::size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    iSize = *ip.offset(0 as libc::c_int as isize) as libc::size_t;
    if iSize >= 128 as libc::c_int as libc::c_ulong {
        if iSize >= 242 as libc::c_int as libc::c_ulong {
            static mut l: [u32; 14] = [
                1 as libc::c_int as u32,
                2 as libc::c_int as u32,
                3 as libc::c_int as u32,
                4 as libc::c_int as u32,
                7 as libc::c_int as u32,
                8 as libc::c_int as u32,
                15 as libc::c_int as u32,
                16 as libc::c_int as u32,
                31 as libc::c_int as u32,
                32 as libc::c_int as u32,
                63 as libc::c_int as u32,
                64 as libc::c_int as u32,
                127 as libc::c_int as u32,
                128 as libc::c_int as u32,
            ];
            oSize = l[iSize.wrapping_sub(242) as usize]
                as libc::size_t;
            memset(huffWeight as *mut libc::c_void, 1 as libc::c_int, hwSize);
            iSize = 0 as libc::c_int as libc::size_t;
        } else {
            oSize = iSize.wrapping_sub(127);
            iSize = oSize
                .wrapping_add(1)
                .wrapping_div(2);
            if iSize.wrapping_add(1) > srcSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            if oSize >= hwSize {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            ip = ip.offset(1 as libc::c_int as isize);
            let mut n: u32 = 0;
            n = 0 as libc::c_int as u32;
            while (n as libc::c_ulong) < oSize {
                *huffWeight
                    .offset(
                        n as isize,
                    ) = (*ip
                    .offset(n.wrapping_div(2) as isize)
                    as libc::c_int >> 4 as libc::c_int) as u8;
                *huffWeight
                    .offset(
                        n.wrapping_add(1) as isize,
                    ) = (*ip
                    .offset(n.wrapping_div(2) as isize)
                    as libc::c_int & 15 as libc::c_int) as u8;
                n = (n as libc::c_uint).wrapping_add(2)
                    ;
            }
        }
    } else {
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
        }
        oSize = FSEv07_decompress(
            huffWeight as *mut libc::c_void,
            hwSize.wrapping_sub(1),
            ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
            iSize,
        );
        if FSEv07_isError_0(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankStats as *mut libc::c_void,
        0 as libc::c_int,
        ((HUFv07_TABLELOG_ABSOLUTEMAX + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>() as libc::c_ulong),
    );
    weightTotal = 0 as libc::c_int as u32;
    let mut n_0: u32 = 0;
    n_0 = 0 as libc::c_int as u32;
    while (n_0 as libc::c_ulong) < oSize {
        if *huffWeight.offset(n_0 as isize) as libc::c_int >= HUFv07_TABLELOG_ABSOLUTEMAX
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        let ref mut fresh2 = *rankStats
            .offset(*huffWeight.offset(n_0 as isize) as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        weightTotal = (weightTotal as libc::c_uint)
            .wrapping_add(
                ((1 as libc::c_int) << *huffWeight.offset(n_0 as isize) as libc::c_int
                    >> 1 as libc::c_int) as libc::c_uint,
            ) ;
        n_0 = n_0.wrapping_add(1);
    }
    if weightTotal == 0 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let tableLog = (BITv07_highbit32(weightTotal))
        .wrapping_add(1);
    if tableLog > HUFv07_TABLELOG_ABSOLUTEMAX as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1 as libc::c_int) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1 as libc::c_int) << BITv07_highbit32(rest)) as u32;
    let lastWeight = (BITv07_highbit32(rest))
        .wrapping_add(1);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as u8;
    let ref mut fresh3 = *rankStats.offset(lastWeight as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    if *rankStats.offset(1 as libc::c_int as isize) < 2 as libc::c_int as libc::c_uint
        || *rankStats.offset(1 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint != 0
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    return iSize.wrapping_add(1);
}
pub const FSEv07_isError_1: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
#[no_mangle]
pub unsafe extern "C" fn FSEv07_createDTable(
    mut tableLog: libc::c_uint,
) -> *mut FSEv07_DTable {
    if tableLog > FSEv07_TABLELOG_ABSOLUTE_MAX as libc::c_uint {
        tableLog = FSEv07_TABLELOG_ABSOLUTE_MAX as libc::c_uint;
    }
    return malloc(
        ((1 as libc::c_int + ((1 as libc::c_int) << tableLog)) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>() as libc::c_ulong),
    ) as *mut FSEv07_DTable;
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_freeDTable(mut dt: *mut FSEv07_DTable) {
    free(dt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_buildDTable(
    mut dt: *mut FSEv07_DTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> libc::size_t {
    let tdPtr = dt.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let tableDecode = tdPtr as *mut FSEv07_decode_t;
    let mut symbolNext: [u16; 256] = [0; 256];
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1 as libc::c_int) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    if maxSymbolValue > FSEv07_MAX_SYMBOL_VALUE as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as libc::size_t;
    }
    if tableLog > FSEv07_MAX_TABLELOG as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    let mut DTableH = FSEv07_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as u16;
    DTableH.fastMode = 1 as libc::c_int as u16;
    let largeLimit = ((1 as libc::c_int)
        << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0 as libc::c_int as u32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -(1 as libc::c_int) {
            let fresh4 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh4 as isize)).symbol = s as u8;
            symbolNext[s as usize] = 1 as libc::c_int as u16;
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int
                >= largeLimit as libc::c_int
            {
                DTableH.fastMode = 0 as libc::c_int as u16;
            }
            symbolNext[s as usize] = *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
    }
    memcpy(
        dt as *mut libc::c_void,
        &mut DTableH as *mut FSEv07_DTableHeader as *const libc::c_void,
        ::core::mem::size_of::<FSEv07_DTableHeader>() as libc::c_ulong,
    );
    let tableMask = tableSize.wrapping_sub(1);
    let step = (tableSize >> 1 as libc::c_int)
        .wrapping_add(tableSize >> 3 as libc::c_int)
        .wrapping_add(3);
    let mut s_0: u32 = 0;
    let mut position = 0 as libc::c_int as u32;
    s_0 = 0 as libc::c_int as u32;
    while s_0 < maxSV1 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < *normalizedCounter.offset(s_0 as isize) as libc::c_int {
            (*tableDecode.offset(position as isize)).symbol = s_0 as u8;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
        s_0 = s_0.wrapping_add(1);
    }
    if position != 0 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    let mut u: u32 = 0;
    u = 0 as libc::c_int as u32;
    while u < tableSize {
        let symbol = (*tableDecode.offset(u as isize)).symbol;
        let fresh5 = symbolNext[symbol as usize];
        symbolNext[symbol as usize] = (symbolNext[symbol as usize]).wrapping_add(1);
        let mut nextState = fresh5;
        (*tableDecode.offset(u as isize))
            .nbBits = tableLog.wrapping_sub(BITv07_highbit32(nextState as u32)) as u8;
        (*tableDecode.offset(u as isize))
            .newState = (((nextState as libc::c_int)
            << (*tableDecode.offset(u as isize)).nbBits as libc::c_int) as libc::c_uint)
            .wrapping_sub(tableSize) as u16;
        u = u.wrapping_add(1);
    }
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_buildDTable_rle(
    mut dt: *mut FSEv07_DTable,
    mut symbolValue: u8,
) -> libc::size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSEv07_DTableHeader;
    let mut dPtr = dt.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let cell = dPtr as *mut FSEv07_decode_t;
    (*DTableH).tableLog = 0 as libc::c_int as u16;
    (*DTableH).fastMode = 0 as libc::c_int as u16;
    (*cell).newState = 0 as libc::c_int as libc::c_ushort;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0 as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_buildDTable_raw(
    mut dt: *mut FSEv07_DTable,
    mut nbBits: libc::c_uint,
) -> libc::size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSEv07_DTableHeader;
    let mut dPtr = dt.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dinfo = dPtr as *mut FSEv07_decode_t;
    let tableSize = ((1 as libc::c_int) << nbBits) as libc::c_uint;
    let tableMask = tableSize.wrapping_sub(1);
    let maxSV1 = tableMask.wrapping_add(1);
    let mut s: libc::c_uint = 0;
    if nbBits < 1 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    (*DTableH).tableLog = nbBits as u16;
    (*DTableH).fastMode = 1 as libc::c_int as u16;
    s = 0 as libc::c_int as libc::c_uint;
    while s < maxSV1 {
        (*dinfo.offset(s as isize)).newState = 0 as libc::c_int as libc::c_ushort;
        (*dinfo.offset(s as isize)).symbol = s as u8;
        (*dinfo.offset(s as isize)).nbBits = nbBits as u8;
        s = s.wrapping_add(1);
    }
    return 0 as libc::c_int as libc::size_t;
}
#[inline(always)]
unsafe extern "C" fn FSEv07_decompress_usingDTable_generic(
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut dt: *const FSEv07_DTable,
    fast: libc::c_uint,
) -> libc::size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3 as libc::c_int as isize));
    let mut bitD = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut state1 = FSEv07_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let mut state2 = FSEv07_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let errorCode = BITv07_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv07_initDState(&mut state1, &mut bitD, dt);
    FSEv07_initDState(&mut state2, &mut bitD, dt);
    while BITv07_reloadDStream(&mut bitD) as libc::c_uint
        == BITv07_DStream_unfinished as libc::c_int as libc::c_uint && op < olimit
    {
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8)
        {
            BITv07_reloadDStream(&mut bitD);
        }
        *op
            .offset(
                1 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSEv07_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 4 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8)
        {
            if BITv07_reloadDStream(&mut bitD) as libc::c_uint
                > BITv07_DStream_unfinished as libc::c_int as libc::c_uint
            {
                op = op.offset(2 as libc::c_int as isize);
                break;
            }
        }
        *op
            .offset(
                2 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8)
        {
            BITv07_reloadDStream(&mut bitD);
        }
        *op
            .offset(
                3 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSEv07_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as u8;
        op = op.offset(4 as libc::c_int as isize);
    }
    loop {
        if op > omax.offset(-(2 as libc::c_int as isize)) {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
        }
        let fresh6 = op;
        op = op.offset(1);
        *fresh6 = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as u8;
        if BITv07_reloadDStream(&mut bitD) as libc::c_uint
            == BITv07_DStream_overflow as libc::c_int as libc::c_uint
        {
            let fresh7 = op;
            op = op.offset(1);
            *fresh7 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
            } else {
                FSEv07_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
            }) as u8;
            break;
        } else {
            if op > omax.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
            }
            let fresh8 = op;
            op = op.offset(1);
            *fresh8 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
            } else {
                FSEv07_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
            }) as u8;
            if !(BITv07_reloadDStream(&mut bitD) as libc::c_uint
                == BITv07_DStream_overflow as libc::c_int as libc::c_uint)
            {
                continue;
            }
            let fresh9 = op;
            op = op.offset(1);
            *fresh9 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
            } else {
                FSEv07_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
            }) as u8;
            break;
        }
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_decompress_usingDTable(
    mut dst: *mut libc::c_void,
    mut originalSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut dt: *const FSEv07_DTable,
) -> libc::size_t {
    let mut ptr = dt as *const libc::c_void;
    let mut DTableH = ptr as *const FSEv07_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSEv07_decompress_usingDTable_generic(
            dst,
            originalSize,
            cSrc,
            cSrcSize,
            dt,
            1 as libc::c_int as libc::c_uint,
        );
    }
    return FSEv07_decompress_usingDTable_generic(
        dst,
        originalSize,
        cSrc,
        cSrcSize,
        dt,
        0 as libc::c_int as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FSEv07_decompress(
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let istart = cSrc as *const u8;
    let mut ip = istart;
    let mut counting: [libc::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: libc::c_uint = 0;
    let mut maxSymbolValue = FSEv07_MAX_SYMBOL_VALUE as libc::c_uint;
    if cSrcSize < 2 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let NCountLength = FSEv07_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const libc::c_void,
        cSrcSize,
    );
    if ERR_isError(NCountLength) != 0 {
        return NCountLength;
    }
    if NCountLength >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(NCountLength as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(NCountLength) as libc::size_t
        as libc::size_t;
    let errorCode = FSEv07_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    return FSEv07_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    );
}
unsafe extern "C" fn HUFv07_getDTableDesc(
    mut table: *const HUFv07_DTable,
) -> DTableDesc {
    let mut dtd = DTableDesc {
        maxTableLog: 0,
        tableType: 0,
        tableLog: 0,
        reserved: 0,
    };
    memcpy(
        &mut dtd as *mut DTableDesc as *mut libc::c_void,
        table as *const libc::c_void,
        ::core::mem::size_of::<DTableDesc>() as libc::c_ulong,
    );
    return dtd;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_readDTableX2(
    mut DTable: *mut HUFv07_DTable,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0 as libc::c_int as u32;
    let mut nbSymbols = 0 as libc::c_int as u32;
    let mut iSize: libc::size_t = 0;
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dt = dtPtr as *mut HUFv07_DEltX2;
    iSize = HUFv07_readStats(
        huffWeight.as_mut_ptr(),
        (HUFv07_SYMBOLVALUE_MAX + 1 as libc::c_int) as libc::size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv07_isError(iSize) != 0 {
        return iSize;
    }
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if tableLog > (dtd.maxTableLog as libc::c_int + 1 as libc::c_int) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    dtd.tableType = 0 as libc::c_int as u8;
    dtd.tableLog = tableLog as u8;
    memcpy(
        DTable as *mut libc::c_void,
        &mut dtd as *mut DTableDesc as *const libc::c_void,
        ::core::mem::size_of::<DTableDesc>() as libc::c_ulong,
    );
    let mut n: u32 = 0;
    let mut nextRankStart = 0 as libc::c_int as u32;
    n = 1 as libc::c_int as u32;
    while n < tableLog.wrapping_add(1) {
        let mut current = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(
                rankVal[n as usize] << n.wrapping_sub(1),
            ) ;
        rankVal[n as usize] = current;
        n = n.wrapping_add(1);
    }
    let mut n_0: u32 = 0;
    n_0 = 0 as libc::c_int as u32;
    while n_0 < nbSymbols {
        let w = huffWeight[n_0 as usize] as u32;
        let length = ((1 as libc::c_int) << w >> 1 as libc::c_int) as u32;
        let mut i: u32 = 0;
        let mut D = HUFv07_DEltX2 {
            byte: 0,
            nbBits: 0,
        };
        D.byte = n_0 as u8;
        D
            .nbBits = tableLog
            .wrapping_add(1)
            .wrapping_sub(w) as u8;
        i = rankVal[w as usize];
        while i < (rankVal[w as usize]).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
        }
        rankVal[w
            as usize] = (rankVal[w as usize] as libc::c_uint).wrapping_add(length) as u32
            as u32;
        n_0 = n_0.wrapping_add(1);
    }
    return iSize;
}
unsafe extern "C" fn HUFv07_decodeSymbolX2(
    mut Dstream: *mut BITv07_DStream_t,
    mut dt: *const HUFv07_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = BITv07_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BITv07_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as u32);
    return c;
}
#[inline]
unsafe extern "C" fn HUFv07_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: *mut BITv07_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv07_DEltX2,
    dtLog: u32,
) -> libc::size_t {
    let pStart = p;
    while BITv07_reloadDStream(bitDPtr) as libc::c_uint
        == BITv07_DStream_unfinished as libc::c_int as libc::c_uint
        && p <= pEnd.offset(-(4 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while BITv07_reloadDStream(bitDPtr) as libc::c_uint
        == BITv07_DStream_unfinished as libc::c_int as libc::c_uint && p < pEnd
    {
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    return pEnd.offset_from(pStart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn HUFv07_decompress1X2_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let mut op = dst as *mut u8;
    let oend = op.offset(dstSize as isize);
    let mut dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUFv07_DEltX2;
    let mut bitD = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let errorCode = BITv07_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv07_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if BITv07_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    return dstSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X2_usingDTable(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 0 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    return HUFv07_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X2_DCtx(
    mut DCtx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX2(DCtx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) ;
    return HUFv07_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        DCtx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X2(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut DTable: [HUFv07_DTable; 2049] = [
        ((12 as libc::c_int - 1 as libc::c_int) as u32)
            .wrapping_mul(0x1000001 as libc::c_int as libc::c_uint),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    return HUFv07_decompress1X2_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize);
}
unsafe extern "C" fn HUFv07_decompress4X2_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    if cSrcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUFv07_DEltX2;
    let mut bitD1 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD2 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD3 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD4 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let length1 = MEM_readLE16(istart as *const libc::c_void) as libc::size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as libc::size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as libc::size_t;
    let length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6),
        );
    let istart1 = istart.offset(6 as libc::c_int as isize);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize
        .wrapping_add(3)
        .wrapping_div(4);
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: u32 = 0;
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let errorCode = BITv07_initDStream(
        &mut bitD1,
        istart1 as *const libc::c_void,
        length1,
    );
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    let errorCode_0 = BITv07_initDStream(
        &mut bitD2,
        istart2 as *const libc::c_void,
        length2,
    );
    if HUFv07_isError(errorCode_0) != 0 {
        return errorCode_0;
    }
    let errorCode_1 = BITv07_initDStream(
        &mut bitD3,
        istart3 as *const libc::c_void,
        length3,
    );
    if HUFv07_isError(errorCode_1) != 0 {
        return errorCode_1;
    }
    let errorCode_2 = BITv07_initDStream(
        &mut bitD4,
        istart4 as *const libc::c_void,
        length4,
    );
    if HUFv07_isError(errorCode_2) != 0 {
        return errorCode_2;
    }
    endSignal = BITv07_reloadDStream(&mut bitD1) as libc::c_uint
        | BITv07_reloadDStream(&mut bitD2) as libc::c_uint
        | BITv07_reloadDStream(&mut bitD3) as libc::c_uint
        | BITv07_reloadDStream(&mut bitD4) as libc::c_uint;
    while endSignal == BITv07_DStream_unfinished as libc::c_int as libc::c_uint
        && op4 < oend.offset(-(7 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            let fresh16 = op1;
            op1 = op1.offset(1);
            *fresh16 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh17 = op2;
            op2 = op2.offset(1);
            *fresh17 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh18 = op3;
            op3 = op3.offset(1);
            *fresh18 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh19 = op4;
            op4 = op4.offset(1);
            *fresh19 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            let fresh20 = op1;
            op1 = op1.offset(1);
            *fresh20 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            let fresh21 = op2;
            op2 = op2.offset(1);
            *fresh21 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            let fresh22 = op3;
            op3 = op3.offset(1);
            *fresh22 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            let fresh23 = op4;
            op4 = op4.offset(1);
            *fresh23 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh24 = op1;
            op1 = op1.offset(1);
            *fresh24 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh25 = op2;
            op2 = op2.offset(1);
            *fresh25 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh26 = op3;
            op3 = op3.offset(1);
            *fresh26 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh27 = op4;
            op4 = op4.offset(1);
            *fresh27 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh28 = op1;
        op1 = op1.offset(1);
        *fresh28 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh29 = op2;
        op2 = op2.offset(1);
        *fresh29 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh30 = op3;
        op3 = op3.offset(1);
        *fresh30 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh31 = op4;
        op4 = op4.offset(1);
        *fresh31 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = BITv07_reloadDStream(&mut bitD1) as libc::c_uint
            | BITv07_reloadDStream(&mut bitD2) as libc::c_uint
            | BITv07_reloadDStream(&mut bitD3) as libc::c_uint
            | BITv07_reloadDStream(&mut bitD4) as libc::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    HUFv07_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv07_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv07_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv07_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv07_endOfDStream(&mut bitD1) & BITv07_endOfDStream(&mut bitD2)
        & BITv07_endOfDStream(&mut bitD3) & BITv07_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    return dstSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X2_usingDTable(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 0 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    return HUFv07_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X2_DCtx(
    mut dctx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX2(dctx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) ;
    return HUFv07_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X2(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut DTable: [HUFv07_DTable; 2049] = [
        ((12 as libc::c_int - 1 as libc::c_int) as u32)
            .wrapping_mul(0x1000001 as libc::c_int as libc::c_uint),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    return HUFv07_decompress4X2_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize);
}
unsafe extern "C" fn HUFv07_fillDTableX4Level2(
    mut DTable: *mut HUFv07_DEltX4,
    mut sizeLog: u32,
    consumed: u32,
    mut rankValOrigin: *const u32,
    minWeight: libc::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    sortedListSize: u32,
    mut nbBitsBaseline: u32,
    mut baseSeq: u16,
) {
    let mut DElt = HUFv07_DEltX4 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [u32; 17] = [0; 17];
    memcpy(
        rankVal.as_mut_ptr() as *mut libc::c_void,
        rankValOrigin as *const libc::c_void,
        ::core::mem::size_of::<[u32; 17]>() as libc::c_ulong,
    );
    if minWeight > 1 as libc::c_int {
        let mut i: u32 = 0;
        let mut skipSize = rankVal[minWeight as usize];
        MEM_writeLE16(&mut DElt.sequence as *mut u16 as *mut libc::c_void, baseSeq);
        DElt.nbBits = consumed as u8;
        DElt.length = 1 as libc::c_int as u8;
        i = 0 as libc::c_int as u32;
        while i < skipSize {
            *DTable.offset(i as isize) = DElt;
            i = i.wrapping_add(1);
        }
    }
    let mut s: u32 = 0;
    s = 0 as libc::c_int as u32;
    while s < sortedListSize {
        let symbol = (*sortedSymbols.offset(s as isize)).symbol as u32;
        let weight = (*sortedSymbols.offset(s as isize)).weight as u32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let length = ((1 as libc::c_int) << sizeLog.wrapping_sub(nbBits)) as u32;
        let start = rankVal[weight as usize];
        let mut i_0 = start;
        let end = start.wrapping_add(length);
        MEM_writeLE16(
            &mut DElt.sequence as *mut u16 as *mut libc::c_void,
            (baseSeq as libc::c_uint).wrapping_add(symbol << 8 as libc::c_int) as u16,
        );
        DElt.nbBits = nbBits.wrapping_add(consumed) as u8;
        DElt.length = 2 as libc::c_int as u8;
        loop {
            let fresh32 = i_0;
            i_0 = i_0.wrapping_add(1);
            *DTable.offset(fresh32 as isize) = DElt;
            if !(i_0 < end) {
                break;
            }
        }
        rankVal[weight
            as usize] = (rankVal[weight as usize] as libc::c_uint).wrapping_add(length)
            ;
        s = s.wrapping_add(1);
    }
}
unsafe extern "C" fn HUFv07_fillDTableX4(
    mut DTable: *mut HUFv07_DEltX4,
    targetLog: u32,
    mut sortedList: *const sortedSymbol_t,
    sortedListSize: u32,
    mut rankStart: *const u32,
    mut rankValOrigin: *mut [u32; 17],
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let mut rankVal: [u32; 17] = [0; 17];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as libc::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: u32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut libc::c_void,
        rankValOrigin as *const libc::c_void,
        ::core::mem::size_of::<[u32; 17]>() as libc::c_ulong,
    );
    s = 0 as libc::c_int as u32;
    while s < sortedListSize {
        let symbol = (*sortedList.offset(s as isize)).symbol as u16;
        let weight = (*sortedList.offset(s as isize)).weight as u32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let start = rankVal[weight as usize];
        let length = ((1 as libc::c_int) << targetLog.wrapping_sub(nbBits)) as u32;
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut sortedRank: u32 = 0;
            let mut minWeight = nbBits.wrapping_add(scaleLog as libc::c_uint)
                as libc::c_int;
            if minWeight < 1 as libc::c_int {
                minWeight = 1 as libc::c_int;
            }
            sortedRank = *rankStart.offset(minWeight as isize);
            HUFv07_fillDTableX4Level2(
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
            let mut DElt = HUFv07_DEltX4 {
                sequence: 0,
                nbBits: 0,
                length: 0,
            };
            MEM_writeLE16(&mut DElt.sequence as *mut u16 as *mut libc::c_void, symbol);
            DElt.nbBits = nbBits as u8;
            DElt.length = 1 as libc::c_int as u8;
            let mut u: u32 = 0;
            let end = start.wrapping_add(length);
            u = start;
            while u < end {
                *DTable.offset(u as isize) = DElt;
                u = u.wrapping_add(1);
            }
        }
        rankVal[weight
            as usize] = (rankVal[weight as usize] as libc::c_uint).wrapping_add(length)
            ;
        s = s.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_readDTableX4(
    mut DTable: *mut HUFv07_DTable,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut weightList: [u8; 256] = [0; 256];
    let mut sortedSymbol: [sortedSymbol_t; 256] = [sortedSymbol_t {
        symbol: 0,
        weight: 0,
    }; 256];
    let mut rankStats: [u32; 17] = [
        0 as libc::c_int as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut rankStart0: [u32; 18] = [
        0 as libc::c_int as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut tableLog: u32 = 0;
    let mut maxW: u32 = 0;
    let mut sizeOfSort: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut dtd = HUFv07_getDTableDesc(DTable);
    let maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: libc::size_t = 0;
    let mut dtPtr = DTable.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dt = dtPtr as *mut HUFv07_DEltX4;
    if maxTableLog > HUFv07_TABLELOG_ABSOLUTEMAX as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    iSize = HUFv07_readStats(
        weightList.as_mut_ptr(),
        (HUFv07_SYMBOLVALUE_MAX + 1 as libc::c_int) as libc::size_t,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv07_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > maxTableLog {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    maxW = tableLog;
    while rankStats[maxW as usize] == 0 as libc::c_int as libc::c_uint {
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: u32 = 0;
    let mut nextRankStart = 0 as libc::c_int as u32;
    w = 1 as libc::c_int as u32;
    while w < maxW.wrapping_add(1) {
        let mut current = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(rankStats[w as usize]) ;
        *rankStart.offset(w as isize) = current;
        w = w.wrapping_add(1);
    }
    *rankStart.offset(0 as libc::c_int as isize) = nextRankStart;
    sizeOfSort = nextRankStart;
    let mut s: u32 = 0;
    s = 0 as libc::c_int as u32;
    while s < nbSymbols {
        let w_0 = weightList[s as usize] as u32;
        let ref mut fresh33 = *rankStart.offset(w_0 as isize);
        let fresh34 = *fresh33;
        *fresh33 = (*fresh33).wrapping_add(1);
        let r = fresh34;
        sortedSymbol[r as usize].symbol = s as u8;
        sortedSymbol[r as usize].weight = w_0 as u8;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0 as libc::c_int as isize) = 0 as libc::c_int as u32;
    let rankVal0 = (rankVal[0 as libc::c_int as usize]).as_mut_ptr();
    let rescale = maxTableLog
        .wrapping_sub(tableLog)
        .wrapping_sub(1) as libc::c_int;
    let mut nextRankVal = 0 as libc::c_int as u32;
    let mut w_1: u32 = 0;
    w_1 = 1 as libc::c_int as u32;
    while w_1 < maxW.wrapping_add(1) {
        let mut current_0 = nextRankVal;
        nextRankVal = (nextRankVal as libc::c_uint)
            .wrapping_add(
                rankStats[w_1 as usize] << w_1.wrapping_add(rescale as libc::c_uint),
            ) ;
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
    }
    let minBits = tableLog
        .wrapping_add(1)
        .wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed
        < maxTableLog
            .wrapping_sub(minBits)
            .wrapping_add(1)
    {
        let rankValPtr = (rankVal[consumed as usize]).as_mut_ptr();
        let mut w_2: u32 = 0;
        w_2 = 1 as libc::c_int as u32;
        while w_2 < maxW.wrapping_add(1) {
            *rankValPtr
                .offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUFv07_fillDTableX4(
        dt,
        maxTableLog,
        sortedSymbol.as_mut_ptr(),
        sizeOfSort,
        rankStart0.as_mut_ptr(),
        rankVal.as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1),
    );
    dtd.tableLog = maxTableLog as u8;
    dtd.tableType = 1 as libc::c_int as u8;
    memcpy(
        DTable as *mut libc::c_void,
        &mut dtd as *mut DTableDesc as *const libc::c_void,
        ::core::mem::size_of::<DTableDesc>() as libc::c_ulong,
    );
    return iSize;
}
unsafe extern "C" fn HUFv07_decodeSymbolX4(
    mut op: *mut libc::c_void,
    mut DStream: *mut BITv07_DStream_t,
    mut dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv07_lookBitsFast(DStream, dtLog);
    memcpy(
        op,
        dt.offset(val as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    BITv07_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    return (*dt.offset(val as isize)).length as u32;
}
unsafe extern "C" fn HUFv07_decodeLastSymbolX4(
    mut op: *mut libc::c_void,
    mut DStream: *mut BITv07_DStream_t,
    mut dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv07_lookBitsFast(DStream, dtLog);
    memcpy(
        op,
        dt.offset(val as isize) as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    if (*dt.offset(val as isize)).length as libc::c_int == 1 as libc::c_int {
        BITv07_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as libc::c_ulong)
        < (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
            .wrapping_mul(8)
    {
        BITv07_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
        if (*DStream).bitsConsumed as libc::c_ulong
            > (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8)
        {
            (*DStream)
                .bitsConsumed = (::core::mem::size_of::<libc::size_t>() as libc::c_ulong)
                .wrapping_mul(8) as libc::c_uint;
        }
    }
    return 1 as libc::c_int as u32;
}
#[inline]
unsafe extern "C" fn HUFv07_decodeStreamX4(
    mut p: *mut u8,
    mut bitDPtr: *mut BITv07_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> libc::size_t {
    let pStart = p;
    while BITv07_reloadDStream(bitDPtr) as libc::c_uint
        == BITv07_DStream_unfinished as libc::c_int as libc::c_uint
        && p < pEnd.offset(-(7 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            p = p
                .offset(
                    HUFv07_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            p = p
                .offset(
                    HUFv07_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        if MEM_64bits() != 0 {
            p = p
                .offset(
                    HUFv07_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                        as isize,
                );
        }
        p = p
            .offset(
                HUFv07_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    while BITv07_reloadDStream(bitDPtr) as libc::c_uint
        == BITv07_DStream_unfinished as libc::c_int as libc::c_uint
        && p <= pEnd.offset(-(2 as libc::c_int as isize))
    {
        p = p
            .offset(
                HUFv07_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    while p <= pEnd.offset(-(2 as libc::c_int as isize)) {
        p = p
            .offset(
                HUFv07_decodeSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    if p < pEnd {
        p = p
            .offset(
                HUFv07_decodeLastSymbolX4(p as *mut libc::c_void, bitDPtr, dt, dtLog)
                    as isize,
            );
    }
    return p.offset_from(pStart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn HUFv07_decompress1X4_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let mut bitD = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let errorCode = BITv07_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUFv07_DEltX4;
    let dtd = HUFv07_getDTableDesc(DTable);
    HUFv07_decodeStreamX4(ostart, &mut bitD, oend, dt, dtd.tableLog as u32);
    if BITv07_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    return dstSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X4_usingDTable(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 1 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    return HUFv07_decompress1X4_usingDTable_internal(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X4_DCtx(
    mut DCtx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX4(DCtx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) ;
    return HUFv07_decompress1X4_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        DCtx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X4(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut DTable: [HUFv07_DTable; 4097] = [
        (12 as libc::c_int as u32)
            .wrapping_mul(0x1000001 as libc::c_int as libc::c_uint),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    return HUFv07_decompress1X4_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize);
}
unsafe extern "C" fn HUFv07_decompress4X4_usingDTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    if cSrcSize < 10 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable.offset(1 as libc::c_int as isize) as *const libc::c_void;
    let dt = dtPtr as *const HUFv07_DEltX4;
    let mut bitD1 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD2 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD3 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD4 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let length1 = MEM_readLE16(istart as *const libc::c_void) as libc::size_t;
    let length2 = MEM_readLE16(
        istart.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as libc::size_t;
    let length3 = MEM_readLE16(
        istart.offset(4 as libc::c_int as isize) as *const libc::c_void,
    ) as libc::size_t;
    let length4 = cSrcSize
        .wrapping_sub(
            length1
                .wrapping_add(length2)
                .wrapping_add(length3)
                .wrapping_add(6),
        );
    let istart1 = istart.offset(6 as libc::c_int as isize);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize
        .wrapping_add(3)
        .wrapping_div(4);
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: u32 = 0;
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let errorCode = BITv07_initDStream(
        &mut bitD1,
        istart1 as *const libc::c_void,
        length1,
    );
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    let errorCode_0 = BITv07_initDStream(
        &mut bitD2,
        istart2 as *const libc::c_void,
        length2,
    );
    if HUFv07_isError(errorCode_0) != 0 {
        return errorCode_0;
    }
    let errorCode_1 = BITv07_initDStream(
        &mut bitD3,
        istart3 as *const libc::c_void,
        length3,
    );
    if HUFv07_isError(errorCode_1) != 0 {
        return errorCode_1;
    }
    let errorCode_2 = BITv07_initDStream(
        &mut bitD4,
        istart4 as *const libc::c_void,
        length4,
    );
    if HUFv07_isError(errorCode_2) != 0 {
        return errorCode_2;
    }
    endSignal = BITv07_reloadDStream(&mut bitD1) as libc::c_uint
        | BITv07_reloadDStream(&mut bitD2) as libc::c_uint
        | BITv07_reloadDStream(&mut bitD3) as libc::c_uint
        | BITv07_reloadDStream(&mut bitD4) as libc::c_uint;
    while endSignal == BITv07_DStream_unfinished as libc::c_int as libc::c_uint
        && op4 < oend.offset(-(7 as libc::c_int as isize))
    {
        if MEM_64bits() != 0 {
            op1 = op1
                .offset(
                    HUFv07_decodeSymbolX4(
                        op1 as *mut libc::c_void,
                        &mut bitD1,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op2 = op2
                .offset(
                    HUFv07_decodeSymbolX4(
                        op2 as *mut libc::c_void,
                        &mut bitD2,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op3 = op3
                .offset(
                    HUFv07_decodeSymbolX4(
                        op3 as *mut libc::c_void,
                        &mut bitD3,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op4 = op4
                .offset(
                    HUFv07_decodeSymbolX4(
                        op4 as *mut libc::c_void,
                        &mut bitD4,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            op1 = op1
                .offset(
                    HUFv07_decodeSymbolX4(
                        op1 as *mut libc::c_void,
                        &mut bitD1,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            op2 = op2
                .offset(
                    HUFv07_decodeSymbolX4(
                        op2 as *mut libc::c_void,
                        &mut bitD2,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            op3 = op3
                .offset(
                    HUFv07_decodeSymbolX4(
                        op3 as *mut libc::c_void,
                        &mut bitD3,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 as libc::c_int {
            op4 = op4
                .offset(
                    HUFv07_decodeSymbolX4(
                        op4 as *mut libc::c_void,
                        &mut bitD4,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op1 = op1
                .offset(
                    HUFv07_decodeSymbolX4(
                        op1 as *mut libc::c_void,
                        &mut bitD1,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op2 = op2
                .offset(
                    HUFv07_decodeSymbolX4(
                        op2 as *mut libc::c_void,
                        &mut bitD2,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op3 = op3
                .offset(
                    HUFv07_decodeSymbolX4(
                        op3 as *mut libc::c_void,
                        &mut bitD3,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        if MEM_64bits() != 0 {
            op4 = op4
                .offset(
                    HUFv07_decodeSymbolX4(
                        op4 as *mut libc::c_void,
                        &mut bitD4,
                        dt,
                        dtLog,
                    ) as isize,
                );
        }
        op1 = op1
            .offset(
                HUFv07_decodeSymbolX4(op1 as *mut libc::c_void, &mut bitD1, dt, dtLog)
                    as isize,
            );
        op2 = op2
            .offset(
                HUFv07_decodeSymbolX4(op2 as *mut libc::c_void, &mut bitD2, dt, dtLog)
                    as isize,
            );
        op3 = op3
            .offset(
                HUFv07_decodeSymbolX4(op3 as *mut libc::c_void, &mut bitD3, dt, dtLog)
                    as isize,
            );
        op4 = op4
            .offset(
                HUFv07_decodeSymbolX4(op4 as *mut libc::c_void, &mut bitD4, dt, dtLog)
                    as isize,
            );
        endSignal = BITv07_reloadDStream(&mut bitD1) as libc::c_uint
            | BITv07_reloadDStream(&mut bitD2) as libc::c_uint
            | BITv07_reloadDStream(&mut bitD3) as libc::c_uint
            | BITv07_reloadDStream(&mut bitD4) as libc::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    HUFv07_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv07_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv07_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv07_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BITv07_endOfDStream(&mut bitD1) & BITv07_endOfDStream(&mut bitD2)
        & BITv07_endOfDStream(&mut bitD3) & BITv07_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    return dstSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X4_usingDTable(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 1 as libc::c_int {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    return HUFv07_decompress4X4_usingDTable_internal(
        dst,
        dstSize,
        cSrc,
        cSrcSize,
        DTable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X4_DCtx(
    mut dctx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = cSrc as *const u8;
    let mut hSize = HUFv07_readDTableX4(dctx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(hSize) ;
    return HUFv07_decompress4X4_usingDTable_internal(
        dst,
        dstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X4(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    let mut DTable: [HUFv07_DTable; 4097] = [
        (12 as libc::c_int as u32)
            .wrapping_mul(0x1000001 as libc::c_int as libc::c_uint),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    return HUFv07_decompress4X4_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X_usingDTable(
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    return if dtd.tableType as libc::c_int != 0 {
        HUFv07_decompress1X4_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
        )
    } else {
        HUFv07_decompress1X2_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X_usingDTable(
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
    mut DTable: *const HUFv07_DTable,
) -> libc::size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    return if dtd.tableType as libc::c_int != 0 {
        HUFv07_decompress4X4_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
        )
    } else {
        HUFv07_decompress4X2_usingDTable_internal(
            dst,
            maxDstSize,
            cSrc,
            cSrcSize,
            DTable,
        )
    };
}
static mut algoTime: [[algo_time_t; 3]; 16] = [
    [
        {
            let mut init = algo_time_t {
                tableTime: 0 as libc::c_int as u32,
                decode256Time: 0 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1 as libc::c_int as u32,
                decode256Time: 1 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2 as libc::c_int as u32,
                decode256Time: 2 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 0 as libc::c_int as u32,
                decode256Time: 0 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1 as libc::c_int as u32,
                decode256Time: 1 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2 as libc::c_int as u32,
                decode256Time: 2 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 38 as libc::c_int as u32,
                decode256Time: 130 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1313 as libc::c_int as u32,
                decode256Time: 74 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2151 as libc::c_int as u32,
                decode256Time: 38 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 448 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1353 as libc::c_int as u32,
                decode256Time: 74 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2238 as libc::c_int as u32,
                decode256Time: 41 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 556 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1353 as libc::c_int as u32,
                decode256Time: 74 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2238 as libc::c_int as u32,
                decode256Time: 47 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 714 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1418 as libc::c_int as u32,
                decode256Time: 74 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2436 as libc::c_int as u32,
                decode256Time: 53 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 883 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1437 as libc::c_int as u32,
                decode256Time: 74 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2464 as libc::c_int as u32,
                decode256Time: 61 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 897 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1515 as libc::c_int as u32,
                decode256Time: 75 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2622 as libc::c_int as u32,
                decode256Time: 68 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 926 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1613 as libc::c_int as u32,
                decode256Time: 75 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2730 as libc::c_int as u32,
                decode256Time: 75 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 947 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1729 as libc::c_int as u32,
                decode256Time: 77 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 3359 as libc::c_int as u32,
                decode256Time: 77 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1107 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2083 as libc::c_int as u32,
                decode256Time: 81 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 4006 as libc::c_int as u32,
                decode256Time: 84 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1177 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2379 as libc::c_int as u32,
                decode256Time: 87 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 4785 as libc::c_int as u32,
                decode256Time: 88 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1242 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2415 as libc::c_int as u32,
                decode256Time: 93 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 5155 as libc::c_int as u32,
                decode256Time: 84 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1349 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2644 as libc::c_int as u32,
                decode256Time: 106 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 5260 as libc::c_int as u32,
                decode256Time: 106 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 1455 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 2422 as libc::c_int as u32,
                decode256Time: 124 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 4174 as libc::c_int as u32,
                decode256Time: 124 as libc::c_int as u32,
            };
            init
        },
    ],
    [
        {
            let mut init = algo_time_t {
                tableTime: 722 as libc::c_int as u32,
                decode256Time: 128 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1891 as libc::c_int as u32,
                decode256Time: 145 as libc::c_int as u32,
            };
            init
        },
        {
            let mut init = algo_time_t {
                tableTime: 1936 as libc::c_int as u32,
                decode256Time: 146 as libc::c_int as u32,
            };
            init
        },
    ],
];
#[no_mangle]
pub unsafe extern "C" fn HUFv07_selectDecoder(
    mut dstSize: libc::size_t,
    mut cSrcSize: libc::size_t,
) -> u32 {
    let Q = cSrcSize
        .wrapping_mul(16)
        .wrapping_div(dstSize) as u32;
    let D256 = (dstSize >> 8 as libc::c_int) as u32;
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
    DTime1 = (DTime1 as libc::c_uint).wrapping_add(DTime1 >> 3 as libc::c_int) as u32
        as u32;
    return (DTime1 < DTime0) as libc::c_int as u32;
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    static mut decompress: [decompressionAlgo; 2] = unsafe {
        [
            Some(
                HUFv07_decompress4X2
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::size_t,
                        *const libc::c_void,
                        libc::size_t,
                    ) -> libc::size_t,
            ),
            Some(
                HUFv07_decompress4X4
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::size_t,
                        *const libc::c_void,
                        libc::size_t,
                    ) -> libc::size_t,
            ),
        ]
    };
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize;
    }
    if cSrcSize == 1 as libc::c_int as libc::c_ulong {
        memset(dst, *(cSrc as *const u8) as libc::c_int, dstSize);
        return dstSize;
    }
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    return (decompress[algoNb as usize])
        .expect("non-null function pointer")(dst, dstSize, cSrc, cSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X_DCtx(
    mut dctx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize;
    }
    if cSrcSize == 1 as libc::c_int as libc::c_ulong {
        memset(dst, *(cSrc as *const u8) as libc::c_int, dstSize);
        return dstSize;
    }
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    return if algoNb != 0 {
        HUFv07_decompress4X4_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    } else {
        HUFv07_decompress4X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress4X_hufOnly(
    mut dctx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if cSrcSize >= dstSize || cSrcSize <= 1 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    return if algoNb != 0 {
        HUFv07_decompress4X4_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    } else {
        HUFv07_decompress4X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUFv07_decompress1X_DCtx(
    mut dctx: *mut HUFv07_DTable,
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: libc::size_t,
) -> libc::size_t {
    if dstSize == 0 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize;
    }
    if cSrcSize == 1 as libc::c_int as libc::c_ulong {
        memset(dst, *(cSrc as *const u8) as libc::c_int, dstSize);
        return dstSize;
    }
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    return if algoNb != 0 {
        HUFv07_decompress1X4_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    } else {
        HUFv07_decompress1X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    };
}
#[export_name = "ZSTDv07_isError"]
pub unsafe extern "C" fn ZSTDv07_isError_0(mut code: libc::size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_isError(mut errorCode: libc::size_t) -> libc::c_uint {
    return ERR_isError(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_getErrorName(
    mut errorCode: libc::size_t,
) -> *const libc::c_char {
    return ERR_getErrorName(errorCode);
}
unsafe extern "C" fn ZSTDv07_defaultAllocFunction(
    mut opaque: *mut libc::c_void,
    mut size: libc::size_t,
) -> *mut libc::c_void {
    let mut address = malloc(size);
    return address;
}
unsafe extern "C" fn ZSTDv07_defaultFreeFunction(
    mut opaque: *mut libc::c_void,
    mut address: *mut libc::c_void,
) {
    free(address);
}
pub const ZSTDv07_DICT_MAGIC: libc::c_uint = 0xec30a437 as libc::c_uint;
pub const ZSTDv07_REP_NUM: libc::c_int = 3 as libc::c_int;
pub const ZSTDv07_REP_INIT: libc::c_int = 3 as libc::c_int;
static mut repStartValue: [u32; 3] = [
    1 as libc::c_int as u32,
    4 as libc::c_int as u32,
    8 as libc::c_int as u32,
];
pub const ZSTDv07_WINDOWLOG_ABSOLUTEMIN: libc::c_int = 10 as libc::c_int;
static mut ZSTDv07_fcs_fieldSize: [libc::size_t; 4] = [
    0 as libc::c_int as libc::size_t,
    2 as libc::c_int as libc::size_t,
    4 as libc::c_int as libc::size_t,
    8 as libc::c_int as libc::size_t,
];
static mut ZSTDv07_did_fieldSize: [libc::size_t; 4] = [
    0 as libc::c_int as libc::size_t,
    1 as libc::c_int as libc::size_t,
    2 as libc::c_int as libc::size_t,
    4 as libc::c_int as libc::size_t,
];
pub const ZSTDv07_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
static mut ZSTDv07_blockHeaderSize: libc::size_t = ZSTDv07_BLOCKHEADERSIZE as libc::size_t;
pub const MIN_SEQUENCES_SIZE: libc::c_int = 1 as libc::c_int;
pub const MIN_CBLOCK_SIZE: libc::c_int = 1 as libc::c_int + 1 as libc::c_int
    + MIN_SEQUENCES_SIZE;
pub const LONGNBSEQ: libc::c_int = 0x7f00 as libc::c_int;
pub const MINMATCH: libc::c_int = 3 as libc::c_int;
pub const MaxML: libc::c_int = 52 as libc::c_int;
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const MaxOff: libc::c_int = 28 as libc::c_int;
pub const MLFSELog: libc::c_int = 9 as libc::c_int;
pub const LLFSELog: libc::c_int = 9 as libc::c_int;
pub const OffFSELog: libc::c_int = 8 as libc::c_int;
pub const FSEv07_ENCODING_RAW: libc::c_int = 0;
pub const FSEv07_ENCODING_RLE: libc::c_int = 1;
pub const FSEv07_ENCODING_STATIC: libc::c_int = 2;
pub const FSEv07_ENCODING_DYNAMIC: libc::c_int = 3;
pub const ZSTD_CONTENTSIZE_ERROR: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(2);
static mut LL_bits: [u32; 36] = [
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    1 as libc::c_int as u32,
    1 as libc::c_int as u32,
    1 as libc::c_int as u32,
    1 as libc::c_int as u32,
    2 as libc::c_int as u32,
    2 as libc::c_int as u32,
    3 as libc::c_int as u32,
    3 as libc::c_int as u32,
    4 as libc::c_int as u32,
    6 as libc::c_int as u32,
    7 as libc::c_int as u32,
    8 as libc::c_int as u32,
    9 as libc::c_int as u32,
    10 as libc::c_int as u32,
    11 as libc::c_int as u32,
    12 as libc::c_int as u32,
    13 as libc::c_int as u32,
    14 as libc::c_int as u32,
    15 as libc::c_int as u32,
    16 as libc::c_int as u32,
];
static mut LL_defaultNorm: [i16; 36] = [
    4 as libc::c_int as i16,
    3 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    3 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
];
static mut LL_defaultNormLog: u32 = 6 as libc::c_int as u32;
static mut ML_bits: [u32; 53] = [
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    0 as libc::c_int as u32,
    1 as libc::c_int as u32,
    1 as libc::c_int as u32,
    1 as libc::c_int as u32,
    1 as libc::c_int as u32,
    2 as libc::c_int as u32,
    2 as libc::c_int as u32,
    3 as libc::c_int as u32,
    3 as libc::c_int as u32,
    4 as libc::c_int as u32,
    4 as libc::c_int as u32,
    5 as libc::c_int as u32,
    7 as libc::c_int as u32,
    8 as libc::c_int as u32,
    9 as libc::c_int as u32,
    10 as libc::c_int as u32,
    11 as libc::c_int as u32,
    12 as libc::c_int as u32,
    13 as libc::c_int as u32,
    14 as libc::c_int as u32,
    15 as libc::c_int as u32,
    16 as libc::c_int as u32,
];
static mut ML_defaultNorm: [i16; 53] = [
    1 as libc::c_int as i16,
    4 as libc::c_int as i16,
    3 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
];
static mut ML_defaultNormLog: u32 = 6 as libc::c_int as u32;
static mut OF_defaultNorm: [i16; 29] = [
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    2 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
    -(1 as libc::c_int) as i16,
];
static mut OF_defaultNormLog: u32 = 5 as libc::c_int as u32;
unsafe extern "C" fn ZSTDv07_copy8(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    memcpy(dst, src, 8 as libc::c_int as libc::c_ulong);
}
pub const WILDCOPY_OVERLENGTH: libc::c_int = 8 as libc::c_int;
unsafe extern "C" fn ZSTDv07_wildcopy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut length: ptrdiff_t,
) {
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    loop {
        ZSTDv07_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
        op = op.offset(8 as libc::c_int as isize);
        ip = ip.offset(8 as libc::c_int as isize);
        if !(op < oend) {
            break;
        }
    };
}
static mut defaultCustomMem: ZSTDv07_customMem = unsafe {
    {
        let mut init = ZSTDv07_customMem {
            customAlloc: Some(
                ZSTDv07_defaultAllocFunction
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::size_t,
                    ) -> *mut libc::c_void,
            ),
            customFree: Some(
                ZSTDv07_defaultFreeFunction
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            opaque: NULL as *mut libc::c_void,
        };
        init
    }
};
pub const ZSTDv07_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
pub const FSEv07_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
pub const HUFv07_isError_0: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
unsafe extern "C" fn ZSTDv07_copy4(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    memcpy(dst, src, 4 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_sizeofDCtx(mut dctx: *const ZSTDv07_DCtx) -> libc::size_t {
    return ::core::mem::size_of::<ZSTDv07_DCtx>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_estimateDCtxSize() -> libc::size_t {
    return ::core::mem::size_of::<ZSTDv07_DCtx>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompressBegin(mut dctx: *mut ZSTDv07_DCtx) -> libc::size_t {
    (*dctx).expected = ZSTDv07_frameHeaderSize_min;
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).previousDstEnd = NULL as *const libc::c_void;
    (*dctx).base = NULL as *const libc::c_void;
    (*dctx).vBase = NULL as *const libc::c_void;
    (*dctx).dictEnd = NULL as *const libc::c_void;
    (*dctx)
        .hufTable[0 as libc::c_int
        as usize] = (12 as libc::c_int * 0x1000001 as libc::c_int) as HUFv07_DTable;
    (*dctx).fseEntropy = 0 as libc::c_int as u32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0 as libc::c_int as u32;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ZSTDv07_REP_NUM {
        (*dctx).rep[i as usize] = repStartValue[i as usize];
        i += 1;
    }
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_createDCtx_advanced(
    mut customMem: ZSTDv07_customMem,
) -> *mut ZSTDv07_DCtx {
    let mut dctx = 0 as *mut ZSTDv07_DCtx;
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return NULL as *mut ZSTDv07_DCtx;
    }
    dctx = (customMem.customAlloc)
        .expect(
            "non-null function pointer",
        )(customMem.opaque, ::core::mem::size_of::<ZSTDv07_DCtx>() as libc::c_ulong)
        as *mut ZSTDv07_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTDv07_DCtx;
    }
    memcpy(
        &mut (*dctx).customMem as *mut ZSTDv07_customMem as *mut libc::c_void,
        &mut customMem as *mut ZSTDv07_customMem as *const libc::c_void,
        ::core::mem::size_of::<ZSTDv07_customMem>() as libc::c_ulong,
    );
    ZSTDv07_decompressBegin(dctx);
    return dctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_createDCtx() -> *mut ZSTDv07_DCtx {
    return ZSTDv07_createDCtx_advanced(defaultCustomMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_freeDCtx(mut dctx: *mut ZSTDv07_DCtx) -> libc::size_t {
    if dctx.is_null() {
        return 0 as libc::c_int as libc::size_t;
    }
    ((*dctx).customMem.customFree)
        .expect(
            "non-null function pointer",
        )((*dctx).customMem.opaque, dctx as *mut libc::c_void);
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_copyDCtx(
    mut dstDCtx: *mut ZSTDv07_DCtx,
    mut srcDCtx: *const ZSTDv07_DCtx,
) {
    memcpy(
        dstDCtx as *mut libc::c_void,
        srcDCtx as *const libc::c_void,
        (::core::mem::size_of::<ZSTDv07_DCtx>() as libc::c_ulong)
            .wrapping_sub(
                ((ZSTDv07_BLOCKSIZE_ABSOLUTEMAX + WILDCOPY_OVERLENGTH) as libc::c_ulong)
                    .wrapping_add(ZSTDv07_frameHeaderSize_max),
            ),
    );
}
unsafe extern "C" fn ZSTDv07_frameHeaderSize(
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    if srcSize < ZSTDv07_frameHeaderSize_min {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let fhd = *(src as *const u8).offset(4 as libc::c_int as isize);
    let dictID = (fhd as libc::c_int & 3 as libc::c_int) as u32;
    let directMode = (fhd as libc::c_int >> 5 as libc::c_int & 1 as libc::c_int) as u32;
    let fcsId = (fhd as libc::c_int >> 6 as libc::c_int) as u32;
    return ZSTDv07_frameHeaderSize_min
        .wrapping_add((directMode == 0) as libc::c_int as libc::c_ulong)
        .wrapping_add(ZSTDv07_did_fieldSize[dictID as usize])
        .wrapping_add(ZSTDv07_fcs_fieldSize[fcsId as usize])
        .wrapping_add(
            (directMode != 0 && ZSTDv07_fcs_fieldSize[fcsId as usize] == 0)
                as libc::c_int as libc::c_ulong,
        );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_getFrameParams(
    mut fparamsPtr: *mut ZSTDv07_frameParams,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = src as *const u8;
    if srcSize < ZSTDv07_frameHeaderSize_min {
        return ZSTDv07_frameHeaderSize_min;
    }
    memset(
        fparamsPtr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTDv07_frameParams>() as libc::c_ulong,
    );
    if MEM_readLE32(src) != ZSTDv07_MAGICNUMBER {
        if MEM_readLE32(src) & 0xfffffff0 as libc::c_uint
            == ZSTDv07_MAGIC_SKIPPABLE_START
        {
            if srcSize < ZSTDv07_skippableHeaderSize {
                return ZSTDv07_skippableHeaderSize;
            }
            (*fparamsPtr)
                .frameContentSize = MEM_readLE32(
                (src as *const libc::c_char).offset(4 as libc::c_int as isize)
                    as *const libc::c_void,
            ) as libc::c_ulonglong;
            (*fparamsPtr).windowSize = 0 as libc::c_int as libc::c_uint;
            return 0 as libc::c_int as libc::size_t;
        }
        return -(ZSTD_error_prefix_unknown as libc::c_int) as libc::size_t;
    }
    let fhsize = ZSTDv07_frameHeaderSize(src, srcSize);
    if srcSize < fhsize {
        return fhsize;
    }
    let fhdByte = *ip.offset(4 as libc::c_int as isize);
    let mut pos = 5 as libc::c_int as libc::size_t;
    let dictIDSizeCode = (fhdByte as libc::c_int & 3 as libc::c_int) as u32;
    let checksumFlag = (fhdByte as libc::c_int >> 2 as libc::c_int & 1 as libc::c_int)
        as u32;
    let directMode = (fhdByte as libc::c_int >> 5 as libc::c_int & 1 as libc::c_int)
        as u32;
    let fcsID = (fhdByte as libc::c_int >> 6 as libc::c_int) as u32;
    let windowSizeMax = (1 as libc::c_uint)
        << (if MEM_32bits() != 0 {
            ZSTDv07_WINDOWLOG_MAX_32
        } else {
            ZSTDv07_WINDOWLOG_MAX_64
        }) as u32;
    let mut windowSize = 0 as libc::c_int as u32;
    let mut dictID = 0 as libc::c_int as u32;
    let mut frameContentSize = 0 as libc::c_int as u64;
    if fhdByte as libc::c_int & 0x8 as libc::c_int != 0 as libc::c_int {
        return -(ZSTD_error_frameParameter_unsupported as libc::c_int) as libc::size_t;
    }
    if directMode == 0 {
        let fresh35 = pos;
        pos = pos.wrapping_add(1);
        let wlByte = *ip.offset(fresh35 as isize);
        let windowLog = ((wlByte as libc::c_int >> 3 as libc::c_int)
            + ZSTDv07_WINDOWLOG_ABSOLUTEMIN) as u32;
        if windowLog
            > (if MEM_32bits() != 0 {
                ZSTDv07_WINDOWLOG_MAX_32
            } else {
                ZSTDv07_WINDOWLOG_MAX_64
            }) as u32
        {
            return -(ZSTD_error_frameParameter_unsupported as libc::c_int) as libc::size_t;
        }
        windowSize = (1 as libc::c_uint) << windowLog;
        windowSize = (windowSize as libc::c_uint)
            .wrapping_add(
                (windowSize >> 3 as libc::c_int)
                    .wrapping_mul(
                        (wlByte as libc::c_int & 7 as libc::c_int) as libc::c_uint,
                    ),
            ) ;
    }
    match dictIDSizeCode {
        1 => {
            dictID = *ip.offset(pos as isize) as u32;
            pos = pos.wrapping_add(1);
        }
        2 => {
            dictID = MEM_readLE16(ip.offset(pos as isize) as *const libc::c_void) as u32;
            pos = (pos as libc::c_ulong).wrapping_add(2)
                ;
        }
        3 => {
            dictID = MEM_readLE32(ip.offset(pos as isize) as *const libc::c_void);
            pos = (pos as libc::c_ulong).wrapping_add(4)
                ;
        }
        0 | _ => {}
    }
    match fcsID {
        1 => {
            frameContentSize = (MEM_readLE16(
                ip.offset(pos as isize) as *const libc::c_void,
            ) as libc::c_int + 256 as libc::c_int) as u64;
        }
        2 => {
            frameContentSize = MEM_readLE32(
                ip.offset(pos as isize) as *const libc::c_void,
            ) as u64;
        }
        3 => {
            frameContentSize = MEM_readLE64(
                ip.offset(pos as isize) as *const libc::c_void,
            );
        }
        0 | _ => {
            if directMode != 0 {
                frameContentSize = *ip.offset(pos as isize) as u64;
            }
        }
    }
    if windowSize == 0 {
        windowSize = frameContentSize as u32;
    }
    if windowSize > windowSizeMax {
        return -(ZSTD_error_frameParameter_unsupported as libc::c_int) as libc::size_t;
    }
    (*fparamsPtr).frameContentSize = frameContentSize as libc::c_ulonglong;
    (*fparamsPtr).windowSize = windowSize;
    (*fparamsPtr).dictID = dictID;
    (*fparamsPtr).checksumFlag = checksumFlag;
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_getDecompressedSize(
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::c_ulonglong {
    let mut fparams = ZSTDv07_frameParams {
        frameContentSize: 0,
        windowSize: 0,
        dictID: 0,
        checksumFlag: 0,
    };
    let frResult = ZSTDv07_getFrameParams(&mut fparams, src, srcSize);
    if frResult != 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    return fparams.frameContentSize;
}
unsafe extern "C" fn ZSTDv07_decodeFrameHeader(
    mut dctx: *mut ZSTDv07_DCtx,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let result = ZSTDv07_getFrameParams(&mut (*dctx).fParams, src, srcSize);
    if (*dctx).fParams.dictID != 0 && (*dctx).dictID != (*dctx).fParams.dictID {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as libc::size_t;
    }
    if (*dctx).fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut (*dctx).xxhState, 0 as libc::c_int as XXH64_hash_t);
    }
    return result;
}
unsafe extern "C" fn ZSTDv07_getcBlockSize(
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut bpPtr: *mut blockProperties_t,
) -> libc::size_t {
    let in_0 = src as *const u8;
    let mut cSize: u32 = 0;
    if srcSize < ZSTDv07_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    (*bpPtr).blockType = (*in_0 as libc::c_int >> 6 as libc::c_int) as blockType_t;
    cSize = (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
        + ((*in_0.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        + ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int)
            << 16 as libc::c_int)) as u32;
    (*bpPtr)
        .origSize = if (*bpPtr).blockType as libc::c_uint
        == bt_rle as libc::c_int as libc::c_uint
    {
        cSize
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if (*bpPtr).blockType as libc::c_uint == bt_end as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::size_t;
    }
    if (*bpPtr).blockType as libc::c_uint == bt_rle as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::size_t;
    }
    return cSize as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_copyRawBlock(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if srcSize > 0 as libc::c_int as libc::c_ulong {
        memcpy(dst, src, srcSize);
    }
    return srcSize;
}
unsafe extern "C" fn ZSTDv07_decodeLiteralsBlock(
    mut dctx: *mut ZSTDv07_DCtx,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let istart = src as *const u8;
    if srcSize < MIN_CBLOCK_SIZE as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    match (*istart.offset(0 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int)
        as litBlockType_t as libc::c_uint
    {
        0 => {
            let mut litSize: libc::size_t = 0;
            let mut litCSize: libc::size_t = 0;
            let mut singleStream = 0 as libc::c_int as libc::size_t;
            let mut lhSize = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int & 3 as libc::c_int) as u32;
            if srcSize < 5 as libc::c_int as libc::c_ulong {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            match lhSize {
                2 => {
                    lhSize = 4 as libc::c_int as u32;
                    litSize = (((*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        & 15 as libc::c_int) << 10 as libc::c_int)
                        + ((*istart.offset(1 as libc::c_int as isize) as libc::c_int)
                            << 2 as libc::c_int)
                        + (*istart.offset(2 as libc::c_int as isize) as libc::c_int
                            >> 6 as libc::c_int)) as libc::size_t;
                    litCSize = (((*istart.offset(2 as libc::c_int as isize)
                        as libc::c_int & 63 as libc::c_int) << 8 as libc::c_int)
                        + *istart.offset(3 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                }
                3 => {
                    lhSize = 5 as libc::c_int as u32;
                    litSize = (((*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        & 15 as libc::c_int) << 14 as libc::c_int)
                        + ((*istart.offset(1 as libc::c_int as isize) as libc::c_int)
                            << 6 as libc::c_int)
                        + (*istart.offset(2 as libc::c_int as isize) as libc::c_int
                            >> 2 as libc::c_int)) as libc::size_t;
                    litCSize = (((*istart.offset(2 as libc::c_int as isize)
                        as libc::c_int & 3 as libc::c_int) << 16 as libc::c_int)
                        + ((*istart.offset(3 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)
                        + *istart.offset(4 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                }
                0 | 1 | _ => {
                    lhSize = 3 as libc::c_int as u32;
                    singleStream = (*istart.offset(0 as libc::c_int as isize)
                        as libc::c_int & 16 as libc::c_int) as libc::size_t;
                    litSize = (((*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        & 15 as libc::c_int) << 6 as libc::c_int)
                        + (*istart.offset(1 as libc::c_int as isize) as libc::c_int
                            >> 2 as libc::c_int)) as libc::size_t;
                    litCSize = (((*istart.offset(1 as libc::c_int as isize)
                        as libc::c_int & 3 as libc::c_int) << 8 as libc::c_int)
                        + *istart.offset(2 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                }
            }
            if litSize > ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as libc::c_ulong {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if litCSize.wrapping_add(lhSize as libc::c_ulong) > srcSize {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if ERR_isError(
                if singleStream != 0 {
                    HUFv07_decompress1X2_DCtx(
                        ((*dctx).hufTable).as_mut_ptr(),
                        ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                        litSize,
                        istart.offset(lhSize as isize) as *const libc::c_void,
                        litCSize,
                    )
                } else {
                    HUFv07_decompress4X_hufOnly(
                        ((*dctx).hufTable).as_mut_ptr(),
                        ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                        litSize,
                        istart.offset(lhSize as isize) as *const libc::c_void,
                        litCSize,
                    )
                },
            ) != 0
            {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize;
            (*dctx).litEntropy = 1 as libc::c_int as u32;
            memset(
                ((*dctx).litBuffer).as_mut_ptr().offset((*dctx).litSize as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                WILDCOPY_OVERLENGTH as libc::c_ulong,
            );
            return litCSize.wrapping_add(lhSize as libc::c_ulong);
        }
        1 => {
            let mut litSize_0: libc::size_t = 0;
            let mut litCSize_0: libc::size_t = 0;
            let mut lhSize_0 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int & 3 as libc::c_int) as u32;
            if lhSize_0 != 1 as libc::c_int as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if (*dctx).litEntropy == 0 as libc::c_int as libc::c_uint {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
            }
            lhSize_0 = 3 as libc::c_int as u32;
            litSize_0 = (((*istart.offset(0 as libc::c_int as isize) as libc::c_int
                & 15 as libc::c_int) << 6 as libc::c_int)
                + (*istart.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int)) as libc::size_t;
            litCSize_0 = (((*istart.offset(1 as libc::c_int as isize) as libc::c_int
                & 3 as libc::c_int) << 8 as libc::c_int)
                + *istart.offset(2 as libc::c_int as isize) as libc::c_int) as libc::size_t;
            if litCSize_0.wrapping_add(lhSize_0 as libc::c_ulong) > srcSize {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            let errorCode = HUFv07_decompress1X4_usingDTable(
                ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                litSize_0,
                istart.offset(lhSize_0 as isize) as *const libc::c_void,
                litCSize_0,
                ((*dctx).hufTable).as_mut_ptr(),
            );
            if ERR_isError(errorCode) != 0 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_0;
            memset(
                ((*dctx).litBuffer).as_mut_ptr().offset((*dctx).litSize as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                WILDCOPY_OVERLENGTH as libc::c_ulong,
            );
            return litCSize_0.wrapping_add(lhSize_0 as libc::c_ulong);
        }
        2 => {
            let mut litSize_1: libc::size_t = 0;
            let mut lhSize_1 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int & 3 as libc::c_int) as u32;
            match lhSize_1 {
                2 => {
                    litSize_1 = (((*istart.offset(0 as libc::c_int as isize)
                        as libc::c_int & 15 as libc::c_int) << 8 as libc::c_int)
                        + *istart.offset(1 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                }
                3 => {
                    litSize_1 = (((*istart.offset(0 as libc::c_int as isize)
                        as libc::c_int & 15 as libc::c_int) << 16 as libc::c_int)
                        + ((*istart.offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)
                        + *istart.offset(2 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                }
                0 | 1 | _ => {
                    lhSize_1 = 1 as libc::c_int as u32;
                    litSize_1 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        & 31 as libc::c_int) as libc::size_t;
                }
            }
            if (lhSize_1 as libc::c_ulong)
                .wrapping_add(litSize_1)
                .wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong) > srcSize
            {
                if litSize_1.wrapping_add(lhSize_1 as libc::c_ulong) > srcSize {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
                }
                memcpy(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                    istart.offset(lhSize_1 as isize) as *const libc::c_void,
                    litSize_1,
                );
                (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
                (*dctx).litSize = litSize_1;
                memset(
                    ((*dctx).litBuffer).as_mut_ptr().offset((*dctx).litSize as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    WILDCOPY_OVERLENGTH as libc::c_ulong,
                );
                return (lhSize_1 as libc::c_ulong).wrapping_add(litSize_1);
            }
            (*dctx).litPtr = istart.offset(lhSize_1 as isize);
            (*dctx).litSize = litSize_1;
            return (lhSize_1 as libc::c_ulong).wrapping_add(litSize_1);
        }
        3 => {
            let mut litSize_2: libc::size_t = 0;
            let mut lhSize_2 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int & 3 as libc::c_int) as u32;
            match lhSize_2 {
                2 => {
                    litSize_2 = (((*istart.offset(0 as libc::c_int as isize)
                        as libc::c_int & 15 as libc::c_int) << 8 as libc::c_int)
                        + *istart.offset(1 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                }
                3 => {
                    litSize_2 = (((*istart.offset(0 as libc::c_int as isize)
                        as libc::c_int & 15 as libc::c_int) << 16 as libc::c_int)
                        + ((*istart.offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)
                        + *istart.offset(2 as libc::c_int as isize) as libc::c_int)
                        as libc::size_t;
                    if srcSize < 4 as libc::c_int as libc::c_ulong {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as libc::size_t;
                    }
                }
                0 | 1 | _ => {
                    lhSize_2 = 1 as libc::c_int as u32;
                    litSize_2 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        & 31 as libc::c_int) as libc::size_t;
                }
            }
            if litSize_2 > ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as libc::c_ulong {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            memset(
                ((*dctx).litBuffer).as_mut_ptr() as *mut libc::c_void,
                *istart.offset(lhSize_2 as isize) as libc::c_int,
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong),
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_2;
            return lhSize_2.wrapping_add(1) as libc::size_t;
        }
        _ => return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t,
    };
}
unsafe extern "C" fn ZSTDv07_buildSeqTable(
    mut DTable: *mut FSEv07_DTable,
    mut type_0: u32,
    mut max: u32,
    mut maxLog: u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut defaultNorm: *const i16,
    mut defaultLog: u32,
    mut flagRepeatTable: u32,
) -> libc::size_t {
    match type_0 {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            if *(src as *const u8) as libc::c_uint > max {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            FSEv07_buildDTable_rle(DTable, *(src as *const u8));
            return 1 as libc::c_int as libc::size_t;
        }
        0 => {
            FSEv07_buildDTable(DTable, defaultNorm, max, defaultLog);
            return 0 as libc::c_int as libc::size_t;
        }
        2 => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            return 0 as libc::c_int as libc::size_t;
        }
        3 | _ => {
            let mut tableLog: u32 = 0;
            let mut norm: [i16; 53] = [0; 53];
            let headerSize = FSEv07_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut tableLog,
                src,
                srcSize,
            );
            if ERR_isError(headerSize) != 0 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            FSEv07_buildDTable(DTable, norm.as_mut_ptr(), max, tableLog);
            return headerSize;
        }
    };
}
unsafe extern "C" fn ZSTDv07_decodeSeqHeaders(
    mut nbSeqPtr: *mut libc::c_int,
    mut DTableLL: *mut FSEv07_DTable,
    mut DTableML: *mut FSEv07_DTable,
    mut DTableOffb: *mut FSEv07_DTable,
    mut flagRepeatTable: u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    if srcSize < MIN_SEQUENCES_SIZE as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let fresh36 = ip;
    ip = ip.offset(1);
    let mut nbSeq = *fresh36 as libc::c_int;
    if nbSeq == 0 {
        *nbSeqPtr = 0 as libc::c_int;
        return 1 as libc::c_int as libc::size_t;
    }
    if nbSeq > 0x7f as libc::c_int {
        if nbSeq == 0xff as libc::c_int {
            if ip.offset(2 as libc::c_int as isize) > iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            nbSeq = MEM_readLE16(ip as *const libc::c_void) as libc::c_int + LONGNBSEQ;
            ip = ip.offset(2 as libc::c_int as isize);
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            let fresh37 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as libc::c_int) << 8 as libc::c_int)
                + *fresh37 as libc::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(4 as libc::c_int as isize) > iend {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let LLtype = (*ip as libc::c_int >> 6 as libc::c_int) as u32;
    let OFtype = (*ip as libc::c_int >> 4 as libc::c_int & 3 as libc::c_int) as u32;
    let MLtype = (*ip as libc::c_int >> 2 as libc::c_int & 3 as libc::c_int) as u32;
    ip = ip.offset(1);
    let llhSize = ZSTDv07_buildSeqTable(
        DTableLL,
        LLtype,
        MaxLL as u32,
        LLFSELog as u32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(llhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(llhSize as isize);
    let ofhSize = ZSTDv07_buildSeqTable(
        DTableOffb,
        OFtype,
        MaxOff as u32,
        OffFSELog as u32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(ofhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(ofhSize as isize);
    let mlhSize = ZSTDv07_buildSeqTable(
        DTableML,
        MLtype,
        MaxML as u32,
        MLFSELog as u32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(mlhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(mlhSize as isize);
    return ip.offset_from(istart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_decodeSequence(mut seqState: *mut seqState_t) -> seq_t {
    let mut seq = seq_t {
        litLength: 0,
        matchLength: 0,
        offset: 0,
    };
    let llCode = FSEv07_peekSymbol(&mut (*seqState).stateLL) as u32;
    let mlCode = FSEv07_peekSymbol(&mut (*seqState).stateML) as u32;
    let ofCode = FSEv07_peekSymbol(&mut (*seqState).stateOffb) as u32;
    let llBits = LL_bits[llCode as usize];
    let mlBits = ML_bits[mlCode as usize];
    let ofBits = ofCode;
    let totalBits = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    static mut LL_base: [u32; 36] = [
        0 as libc::c_int as u32,
        1 as libc::c_int as u32,
        2 as libc::c_int as u32,
        3 as libc::c_int as u32,
        4 as libc::c_int as u32,
        5 as libc::c_int as u32,
        6 as libc::c_int as u32,
        7 as libc::c_int as u32,
        8 as libc::c_int as u32,
        9 as libc::c_int as u32,
        10 as libc::c_int as u32,
        11 as libc::c_int as u32,
        12 as libc::c_int as u32,
        13 as libc::c_int as u32,
        14 as libc::c_int as u32,
        15 as libc::c_int as u32,
        16 as libc::c_int as u32,
        18 as libc::c_int as u32,
        20 as libc::c_int as u32,
        22 as libc::c_int as u32,
        24 as libc::c_int as u32,
        28 as libc::c_int as u32,
        32 as libc::c_int as u32,
        40 as libc::c_int as u32,
        48 as libc::c_int as u32,
        64 as libc::c_int as u32,
        0x80 as libc::c_int as u32,
        0x100 as libc::c_int as u32,
        0x200 as libc::c_int as u32,
        0x400 as libc::c_int as u32,
        0x800 as libc::c_int as u32,
        0x1000 as libc::c_int as u32,
        0x2000 as libc::c_int as u32,
        0x4000 as libc::c_int as u32,
        0x8000 as libc::c_int as u32,
        0x10000 as libc::c_int as u32,
    ];
    static mut ML_base: [u32; 53] = [
        3 as libc::c_int as u32,
        4 as libc::c_int as u32,
        5 as libc::c_int as u32,
        6 as libc::c_int as u32,
        7 as libc::c_int as u32,
        8 as libc::c_int as u32,
        9 as libc::c_int as u32,
        10 as libc::c_int as u32,
        11 as libc::c_int as u32,
        12 as libc::c_int as u32,
        13 as libc::c_int as u32,
        14 as libc::c_int as u32,
        15 as libc::c_int as u32,
        16 as libc::c_int as u32,
        17 as libc::c_int as u32,
        18 as libc::c_int as u32,
        19 as libc::c_int as u32,
        20 as libc::c_int as u32,
        21 as libc::c_int as u32,
        22 as libc::c_int as u32,
        23 as libc::c_int as u32,
        24 as libc::c_int as u32,
        25 as libc::c_int as u32,
        26 as libc::c_int as u32,
        27 as libc::c_int as u32,
        28 as libc::c_int as u32,
        29 as libc::c_int as u32,
        30 as libc::c_int as u32,
        31 as libc::c_int as u32,
        32 as libc::c_int as u32,
        33 as libc::c_int as u32,
        34 as libc::c_int as u32,
        35 as libc::c_int as u32,
        37 as libc::c_int as u32,
        39 as libc::c_int as u32,
        41 as libc::c_int as u32,
        43 as libc::c_int as u32,
        47 as libc::c_int as u32,
        51 as libc::c_int as u32,
        59 as libc::c_int as u32,
        67 as libc::c_int as u32,
        83 as libc::c_int as u32,
        99 as libc::c_int as u32,
        0x83 as libc::c_int as u32,
        0x103 as libc::c_int as u32,
        0x203 as libc::c_int as u32,
        0x403 as libc::c_int as u32,
        0x803 as libc::c_int as u32,
        0x1003 as libc::c_int as u32,
        0x2003 as libc::c_int as u32,
        0x4003 as libc::c_int as u32,
        0x8003 as libc::c_int as u32,
        0x10003 as libc::c_int as u32,
    ];
    static mut OF_base: [u32; 29] = [
        0 as libc::c_int as u32,
        1 as libc::c_int as u32,
        1 as libc::c_int as u32,
        5 as libc::c_int as u32,
        0xd as libc::c_int as u32,
        0x1d as libc::c_int as u32,
        0x3d as libc::c_int as u32,
        0x7d as libc::c_int as u32,
        0xfd as libc::c_int as u32,
        0x1fd as libc::c_int as u32,
        0x3fd as libc::c_int as u32,
        0x7fd as libc::c_int as u32,
        0xffd as libc::c_int as u32,
        0x1ffd as libc::c_int as u32,
        0x3ffd as libc::c_int as u32,
        0x7ffd as libc::c_int as u32,
        0xfffd as libc::c_int as u32,
        0x1fffd as libc::c_int as u32,
        0x3fffd as libc::c_int as u32,
        0x7fffd as libc::c_int as u32,
        0xffffd as libc::c_int as u32,
        0x1ffffd as libc::c_int as u32,
        0x3ffffd as libc::c_int as u32,
        0x7ffffd as libc::c_int as u32,
        0xfffffd as libc::c_int as u32,
        0x1fffffd as libc::c_int as u32,
        0x3fffffd as libc::c_int as u32,
        0x7fffffd as libc::c_int as u32,
        0xffffffd as libc::c_int as u32,
    ];
    let mut offset: libc::size_t = 0;
    if ofCode == 0 {
        offset = 0 as libc::c_int as libc::size_t;
    } else {
        offset = (OF_base[ofCode as usize] as libc::c_ulong)
            .wrapping_add(BITv07_readBits(&mut (*seqState).DStream, ofBits));
        if MEM_32bits() != 0 {
            BITv07_reloadDStream(&mut (*seqState).DStream);
        }
    }
    if ofCode <= 1 as libc::c_int as libc::c_uint {
        if (llCode == 0 as libc::c_int as libc::c_uint) as libc::c_int
            & (offset <= 1 as libc::c_int as libc::c_ulong) as libc::c_int != 0
        {
            offset = (1 as libc::c_int as libc::c_ulong).wrapping_sub(offset);
        }
        if offset != 0 {
            let temp = (*seqState).prevOffset[offset as usize];
            if offset != 1 as libc::c_int as libc::c_ulong {
                (*seqState)
                    .prevOffset[2 as libc::c_int
                    as usize] = (*seqState).prevOffset[1 as libc::c_int as usize];
            }
            (*seqState)
                .prevOffset[1 as libc::c_int
                as usize] = (*seqState).prevOffset[0 as libc::c_int as usize];
            offset = temp;
            (*seqState).prevOffset[0 as libc::c_int as usize] = offset;
        } else {
            offset = (*seqState).prevOffset[0 as libc::c_int as usize];
        }
    } else {
        (*seqState)
            .prevOffset[2 as libc::c_int
            as usize] = (*seqState).prevOffset[1 as libc::c_int as usize];
        (*seqState)
            .prevOffset[1 as libc::c_int
            as usize] = (*seqState).prevOffset[0 as libc::c_int as usize];
        (*seqState).prevOffset[0 as libc::c_int as usize] = offset;
    }
    seq.offset = offset;
    seq
        .matchLength = (ML_base[mlCode as usize] as libc::c_ulong)
        .wrapping_add(
            (if mlCode > 31 as libc::c_int as libc::c_uint {
                BITv07_readBits(&mut (*seqState).DStream, mlBits)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        );
    if MEM_32bits() != 0
        && mlBits.wrapping_add(llBits) > 24 as libc::c_int as libc::c_uint
    {
        BITv07_reloadDStream(&mut (*seqState).DStream);
    }
    seq
        .litLength = (LL_base[llCode as usize] as libc::c_ulong)
        .wrapping_add(
            (if llCode > 15 as libc::c_int as libc::c_uint {
                BITv07_readBits(&mut (*seqState).DStream, llBits)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        );
    if MEM_32bits() != 0
        || totalBits
            > (64 as libc::c_int - 7 as libc::c_int - (LLFSELog + MLFSELog + OffFSELog))
                as libc::c_uint
    {
        BITv07_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv07_updateState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    FSEv07_updateState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if MEM_32bits() != 0 {
        BITv07_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv07_updateState(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
    return seq;
}
unsafe extern "C" fn ZSTDv07_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    base: *const u8,
    vBase: *const u8,
    dictEnd: *const u8,
) -> libc::size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    debug_assert!(oend >= op);
    if (sequence.litLength).wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong)
        > oend.offset_from(op) as libc::c_long as libc::size_t
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if sequenceLength > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    debug_assert!(litLimit >= *litPtr);
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as libc::size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ZSTDv07_wildcopy(
        op as *mut libc::c_void,
        *litPtr as *const libc::c_void,
        sequence.litLength as ptrdiff_t,
    );
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(base) as libc::c_long as libc::size_t {
        if sequence.offset > oLitEnd.offset_from(vBase) as libc::c_long as libc::size_t {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0) as libc::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            memmove(
                oLitEnd as *mut libc::c_void,
                match_0 as *const libc::c_void,
                sequence.matchLength,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as libc::size_t;
        memmove(oLitEnd as *mut libc::c_void, match_0 as *const libc::c_void, length1);
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            ;
        match_0 = base;
        if op > oend_w || sequence.matchLength < MINMATCH as libc::c_ulong {
            while op < oMatchEnd {
                let fresh38 = match_0;
                match_0 = match_0.offset(1);
                let fresh39 = op;
                op = op.offset(1);
                *fresh39 = *fresh38;
            }
            return sequenceLength;
        }
    }
    if sequence.offset < 8 as libc::c_int as libc::c_ulong {
        static mut dec32table: [u32; 8] = [
            0 as libc::c_int as u32,
            1 as libc::c_int as u32,
            2 as libc::c_int as u32,
            1 as libc::c_int as u32,
            4 as libc::c_int as u32,
            4 as libc::c_int as u32,
            4 as libc::c_int as u32,
            4 as libc::c_int as u32,
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
        let sub2 = dec64table[sequence.offset as usize];
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
        ZSTDv07_copy4(
            op.offset(4 as libc::c_int as isize) as *mut libc::c_void,
            match_0 as *const libc::c_void,
        );
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ZSTDv07_copy8(op as *mut libc::c_void, match_0 as *const libc::c_void);
    }
    op = op.offset(8 as libc::c_int as isize);
    match_0 = match_0.offset(8 as libc::c_int as isize);
    if oMatchEnd > oend.offset(-((16 as libc::c_int - MINMATCH) as isize)) {
        if op < oend_w {
            ZSTDv07_wildcopy(
                op as *mut libc::c_void,
                match_0 as *const libc::c_void,
                oend_w.offset_from(op) as libc::c_long,
            );
            match_0 = match_0.offset(oend_w.offset_from(op) as libc::c_long as isize);
            op = oend_w;
        }
        while op < oMatchEnd {
            let fresh40 = match_0;
            match_0 = match_0.offset(1);
            let fresh41 = op;
            op = op.offset(1);
            *fresh41 = *fresh40;
        }
    } else {
        ZSTDv07_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t - 8 as libc::c_int as libc::c_long,
        );
    }
    return sequenceLength;
}
unsafe extern "C" fn ZSTDv07_decompressSequences(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
) -> libc::size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(maxDstSize as isize);
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.offset((*dctx).litSize as isize);
    let mut DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let mut DTableML = ((*dctx).MLTable).as_mut_ptr();
    let mut DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *const u8;
    let vBase = (*dctx).vBase as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    let mut nbSeq: libc::c_int = 0;
    let seqHSize = ZSTDv07_decodeSeqHeaders(
        &mut nbSeq,
        DTableLL,
        DTableML,
        DTableOffb,
        (*dctx).fseEntropy,
        ip as *const libc::c_void,
        seqSize,
    );
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = ip.offset(seqHSize as isize);
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BITv07_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: 0 as *const libc::c_char,
                start: 0 as *const libc::c_char,
            },
            stateLL: FSEv07_DState_t {
                state: 0,
                table: 0 as *const libc::c_void,
            },
            stateOffb: FSEv07_DState_t {
                state: 0,
                table: 0 as *const libc::c_void,
            },
            stateML: FSEv07_DState_t {
                state: 0,
                table: 0 as *const libc::c_void,
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1 as libc::c_int as u32;
        let mut i: u32 = 0;
        i = 0 as libc::c_int as u32;
        while i < ZSTDv07_REP_INIT as libc::c_uint {
            seqState.prevOffset[i as usize] = (*dctx).rep[i as usize] as libc::size_t;
            i = i.wrapping_add(1);
        }
        let errorCode = BITv07_initDStream(
            &mut seqState.DStream,
            ip as *const libc::c_void,
            iend.offset_from(ip) as libc::c_long as libc::size_t,
        );
        if ERR_isError(errorCode) != 0 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        FSEv07_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv07_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv07_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while BITv07_reloadDStream(&mut seqState.DStream) as libc::c_uint
            <= BITv07_DStream_completed as libc::c_int as libc::c_uint && nbSeq != 0
        {
            nbSeq -= 1;
            let sequence = ZSTDv07_decodeSequence(&mut seqState);
            let oneSeqSize = ZSTDv07_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litEnd,
                base,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        let mut i_0: u32 = 0;
        i_0 = 0 as libc::c_int as u32;
        while i_0 < ZSTDv07_REP_INIT as libc::c_uint {
            (*dctx).rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as libc::c_long as libc::size_t;
    if lastLLSize > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if lastLLSize > 0 as libc::c_int as libc::c_ulong {
        memcpy(op as *mut libc::c_void, litPtr as *const libc::c_void, lastLLSize);
        op = op.offset(lastLLSize as isize);
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_checkContinuity(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *const libc::c_void,
) {
    if dst != (*dctx).previousDstEnd {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx)
            .vBase = (dst as *const libc::c_char)
            .offset(
                -(((*dctx).previousDstEnd as *const libc::c_char)
                    .offset_from((*dctx).base as *const libc::c_char) as libc::c_long
                    as isize),
            ) as *const libc::c_void;
        (*dctx).base = dst;
        (*dctx).previousDstEnd = dst;
    }
}
unsafe extern "C" fn ZSTDv07_decompressBlock_internal(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = src as *const u8;
    if srcSize >= ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let litCSize = ZSTDv07_decodeLiteralsBlock(dctx, src, srcSize);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(litCSize) ;
    return ZSTDv07_decompressSequences(
        dctx,
        dst,
        dstCapacity,
        ip as *const libc::c_void,
        srcSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompressBlock(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut dSize: libc::size_t = 0;
    ZSTDv07_checkContinuity(dctx, dst);
    dSize = ZSTDv07_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize);
    (*dctx)
        .previousDstEnd = (dst as *mut libc::c_char).offset(dSize as isize)
        as *const libc::c_void;
    return dSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_insertBlock(
    mut dctx: *mut ZSTDv07_DCtx,
    mut blockStart: *const libc::c_void,
    mut blockSize: libc::size_t,
) -> libc::size_t {
    ZSTDv07_checkContinuity(dctx, blockStart);
    (*dctx)
        .previousDstEnd = (blockStart as *const libc::c_char).offset(blockSize as isize)
        as *const libc::c_void;
    return blockSize;
}
unsafe extern "C" fn ZSTDv07_generateNxBytes(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut byte: u8,
    mut length: libc::size_t,
) -> libc::size_t {
    if length > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if length > 0 as libc::c_int as libc::c_ulong {
        memset(dst, byte as libc::c_int, length);
    }
    return length;
}
unsafe extern "C" fn ZSTDv07_decompressFrame(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut remainingSize = srcSize;
    if srcSize < ZSTDv07_frameHeaderSize_min.wrapping_add(ZSTDv07_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let frameHeaderSize = ZSTDv07_frameHeaderSize(src, ZSTDv07_frameHeaderSize_min);
    if ERR_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv07_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    if ZSTDv07_decodeFrameHeader(dctx, src, frameHeaderSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(frameHeaderSize)
        ;
    loop {
        let mut decodedSize: libc::size_t = 0;
        let mut blockProperties = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let cBlockSize = ZSTDv07_getcBlockSize(
            ip as *const libc::c_void,
            iend.offset_from(ip) as libc::c_long as libc::size_t,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.offset(ZSTDv07_blockHeaderSize as isize);
        remainingSize = (remainingSize as libc::c_ulong)
            .wrapping_sub(ZSTDv07_blockHeaderSize) ;
        if cBlockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
        }
        match blockProperties.blockType as libc::c_uint {
            0 => {
                decodedSize = ZSTDv07_decompressBlock_internal(
                    dctx,
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as libc::size_t,
                    ip as *const libc::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTDv07_copyRawBlock(
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as libc::size_t,
                    ip as *const libc::c_void,
                    cBlockSize,
                );
            }
            2 => {
                decodedSize = ZSTDv07_generateNxBytes(
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as libc::size_t,
                    *ip,
                    blockProperties.origSize as libc::size_t,
                );
            }
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
                }
                decodedSize = 0 as libc::c_int as libc::size_t;
            }
            _ => return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t,
        }
        if blockProperties.blockType as libc::c_uint
            == bt_end as libc::c_int as libc::c_uint
        {
            break;
        }
        if ERR_isError(decodedSize) != 0 {
            return decodedSize;
        }
        if (*dctx).fParams.checksumFlag != 0 {
            ZSTD_XXH64_update(
                &mut (*dctx).xxhState,
                op as *const libc::c_void,
                decodedSize,
            );
        }
        op = op.offset(decodedSize as isize);
        ip = ip.offset(cBlockSize as isize);
        remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(cBlockSize)
            ;
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_decompress_usingPreparedDCtx(
    mut dctx: *mut ZSTDv07_DCtx,
    mut refDCtx: *const ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    ZSTDv07_copyDCtx(dctx, refDCtx);
    ZSTDv07_checkContinuity(dctx, dst);
    return ZSTDv07_decompressFrame(dctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompress_usingDict(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::size_t {
    ZSTDv07_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv07_checkContinuity(dctx, dst);
    return ZSTDv07_decompressFrame(dctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompressDCtx(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTDv07_decompress_usingDict(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        NULL as *const libc::c_void,
        0 as libc::c_int as libc::size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompress(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut regenSize: libc::size_t = 0;
    let dctx = ZSTDv07_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as libc::size_t;
    }
    regenSize = ZSTDv07_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTDv07_freeDCtx(dctx);
    return regenSize;
}
unsafe extern "C" fn ZSTD_errorFrameSizeInfoLegacy(
    mut cSize: *mut libc::size_t,
    mut dBound: *mut libc::c_ulonglong,
    mut ret: libc::size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_findFrameSizeInfoLegacy(
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut cSize: *mut libc::size_t,
    mut dBound: *mut libc::c_ulonglong,
) {
    let mut ip = src as *const u8;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as libc::c_int as libc::size_t;
    if srcSize < ZSTDv07_frameHeaderSize_min.wrapping_add(ZSTDv07_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t,
        );
        return;
    }
    let frameHeaderSize = ZSTDv07_frameHeaderSize(src, srcSize);
    if ERR_isError(frameHeaderSize) != 0 {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, frameHeaderSize);
        return;
    }
    if MEM_readLE32(src) != ZSTDv07_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as libc::c_int) as libc::size_t,
        );
        return;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv07_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t,
        );
        return;
    }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(frameHeaderSize)
        ;
    loop {
        let mut blockProperties = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let cBlockSize = ZSTDv07_getcBlockSize(
            ip as *const libc::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.offset(ZSTDv07_blockHeaderSize as isize);
        remainingSize = (remainingSize as libc::c_ulong)
            .wrapping_sub(ZSTDv07_blockHeaderSize) ;
        if blockProperties.blockType as libc::c_uint
            == bt_end as libc::c_int as libc::c_uint
        {
            break;
        }
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t,
            );
            return;
        }
        ip = ip.offset(cBlockSize as isize);
        remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(cBlockSize)
            ;
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const u8) as libc::c_long as libc::size_t;
    *dBound = nbBlocks.wrapping_mul(ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as libc::c_ulong)
        as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_nextSrcSizeToDecompress(
    mut dctx: *mut ZSTDv07_DCtx,
) -> libc::size_t {
    return (*dctx).expected;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_isSkipFrame(
    mut dctx: *mut ZSTDv07_DCtx,
) -> libc::c_int {
    return ((*dctx).stage as libc::c_uint
        == ZSTDds_skipFrame as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompressContinue(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    if dstCapacity != 0 {
        ZSTDv07_checkContinuity(dctx, dst);
    }
    match (*dctx).stage as libc::c_uint {
        0 => {
            if srcSize != ZSTDv07_frameHeaderSize_min {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            if MEM_readLE32(src) & 0xfffffff0 as libc::c_uint
                == ZSTDv07_MAGIC_SKIPPABLE_START
            {
                memcpy(
                    ((*dctx).headerBuffer).as_mut_ptr() as *mut libc::c_void,
                    src,
                    ZSTDv07_frameHeaderSize_min,
                );
                (*dctx)
                    .expected = ZSTDv07_skippableHeaderSize
                    .wrapping_sub(ZSTDv07_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeSkippableHeader;
                return 0 as libc::c_int as libc::size_t;
            }
            (*dctx)
                .headerSize = ZSTDv07_frameHeaderSize(src, ZSTDv07_frameHeaderSize_min);
            if ERR_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut libc::c_void,
                src,
                ZSTDv07_frameHeaderSize_min,
            );
            if (*dctx).headerSize > ZSTDv07_frameHeaderSize_min {
                (*dctx)
                    .expected = ((*dctx).headerSize)
                    .wrapping_sub(ZSTDv07_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeFrameHeader;
                return 0 as libc::c_int as libc::size_t;
            }
            (*dctx).expected = 0 as libc::c_int as libc::size_t;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let cBlockSize = ZSTDv07_getcBlockSize(
                src,
                ZSTDv07_blockHeaderSize,
                &mut bp,
            );
            if ERR_isError(cBlockSize) != 0 {
                return cBlockSize;
            }
            if bp.blockType as libc::c_uint == bt_end as libc::c_int as libc::c_uint {
                if (*dctx).fParams.checksumFlag != 0 {
                    let h64 = ZSTD_XXH64_digest(&mut (*dctx).xxhState);
                    let h32 = (h64 >> 11 as libc::c_int) as u32
                        & (((1 as libc::c_int) << 22 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ip = src as *const u8;
                    let check32 = (*ip.offset(2 as libc::c_int as isize) as libc::c_int
                        + ((*ip.offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)
                        + ((*ip.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) << 16 as libc::c_int)) as u32;
                    if check32 != h32 {
                        return -(ZSTD_error_checksum_wrong as libc::c_int) as libc::size_t;
                    }
                }
                (*dctx).expected = 0 as libc::c_int as libc::size_t;
                (*dctx).stage = ZSTDds_getFrameHeaderSize;
            } else {
                (*dctx).expected = cBlockSize;
                (*dctx).bType = bp.blockType;
                (*dctx).stage = ZSTDds_decompressBlock;
            }
            return 0 as libc::c_int as libc::size_t;
        }
        3 => {
            let mut rSize: libc::size_t = 0;
            match (*dctx).bType as libc::c_uint {
                0 => {
                    rSize = ZSTDv07_decompressBlock_internal(
                        dctx,
                        dst,
                        dstCapacity,
                        src,
                        srcSize,
                    );
                }
                1 => {
                    rSize = ZSTDv07_copyRawBlock(dst, dstCapacity, src, srcSize);
                }
                2 => return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t,
                3 => {
                    rSize = 0 as libc::c_int as libc::size_t;
                }
                _ => return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t,
            }
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            (*dctx).expected = ZSTDv07_blockHeaderSize;
            (*dctx)
                .previousDstEnd = (dst as *mut libc::c_char).offset(rSize as isize)
                as *const libc::c_void;
            if ERR_isError(rSize) != 0 {
                return rSize;
            }
            if (*dctx).fParams.checksumFlag != 0 {
                ZSTD_XXH64_update(&mut (*dctx).xxhState, dst, rSize);
            }
            return rSize;
        }
        4 => {
            memcpy(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .offset(ZSTDv07_frameHeaderSize_min as isize) as *mut libc::c_void,
                src,
                (*dctx).expected,
            );
            (*dctx)
                .expected = MEM_readLE32(
                ((*dctx).headerBuffer).as_mut_ptr().offset(4 as libc::c_int as isize)
                    as *const libc::c_void,
            ) as libc::size_t;
            (*dctx).stage = ZSTDds_skipFrame;
            return 0 as libc::c_int as libc::size_t;
        }
        5 => {
            (*dctx).expected = 0 as libc::c_int as libc::size_t;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            return 0 as libc::c_int as libc::size_t;
        }
        _ => return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t,
    }
    let mut result: libc::size_t = 0;
    memcpy(
        ((*dctx).headerBuffer).as_mut_ptr().offset(ZSTDv07_frameHeaderSize_min as isize)
            as *mut libc::c_void,
        src,
        (*dctx).expected,
    );
    result = ZSTDv07_decodeFrameHeader(
        dctx,
        ((*dctx).headerBuffer).as_mut_ptr() as *const libc::c_void,
        (*dctx).headerSize,
    );
    if ERR_isError(result) != 0 {
        return result;
    }
    (*dctx).expected = ZSTDv07_blockHeaderSize;
    (*dctx).stage = ZSTDds_decodeBlockHeader;
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_refDictContent(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::size_t {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx)
        .vBase = (dict as *const libc::c_char)
        .offset(
            -(((*dctx).previousDstEnd as *const libc::c_char)
                .offset_from((*dctx).base as *const libc::c_char) as libc::c_long
                as isize),
        ) as *const libc::c_void;
    (*dctx).base = dict;
    (*dctx)
        .previousDstEnd = (dict as *const libc::c_char).offset(dictSize as isize)
        as *const libc::c_void;
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_loadEntropy(
    mut dctx: *mut ZSTDv07_DCtx,
    dict: *const libc::c_void,
    dictSize: libc::size_t,
) -> libc::size_t {
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.offset(dictSize as isize);
    let hSize = HUFv07_readDTableX4(((*dctx).hufTable).as_mut_ptr(), dict, dictSize);
    if ERR_isError(hSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    dictPtr = dictPtr.offset(hSize as isize);
    let mut offcodeNCount: [libc::c_short; 29] = [0; 29];
    let mut offcodeMaxValue = MaxOff as u32;
    let mut offcodeLog: u32 = 0;
    let offcodeHeaderSize = FSEv07_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as libc::size_t,
    );
    if ERR_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    if offcodeLog > OffFSELog as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    let errorCode = FSEv07_buildDTable(
        ((*dctx).OffTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    );
    if ERR_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    dictPtr = dictPtr.offset(offcodeHeaderSize as isize);
    let mut matchlengthNCount: [libc::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as libc::c_uint;
    let mut matchlengthLog: libc::c_uint = 0;
    let matchlengthHeaderSize = FSEv07_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as libc::size_t,
    );
    if ERR_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    if matchlengthLog > MLFSELog as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    let errorCode_0 = FSEv07_buildDTable(
        ((*dctx).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    );
    if ERR_isError(errorCode_0) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    dictPtr = dictPtr.offset(matchlengthHeaderSize as isize);
    let mut litlengthNCount: [libc::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as libc::c_uint;
    let mut litlengthLog: libc::c_uint = 0;
    let litlengthHeaderSize = FSEv07_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr as *const libc::c_void,
        dictEnd.offset_from(dictPtr) as libc::c_long as libc::size_t,
    );
    if ERR_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    if litlengthLog > LLFSELog as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    let errorCode_1 = FSEv07_buildDTable(
        ((*dctx).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    );
    if ERR_isError(errorCode_1) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    dictPtr = dictPtr.offset(litlengthHeaderSize as isize);
    if dictPtr.offset(12 as libc::c_int as isize) > dictEnd {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    (*dctx)
        .rep[0 as libc::c_int
        as usize] = MEM_readLE32(
        dictPtr.offset(0 as libc::c_int as isize) as *const libc::c_void,
    );
    if (*dctx).rep[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_uint
        || (*dctx).rep[0 as libc::c_int as usize] as libc::c_ulong >= dictSize
    {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    (*dctx)
        .rep[1 as libc::c_int
        as usize] = MEM_readLE32(
        dictPtr.offset(4 as libc::c_int as isize) as *const libc::c_void,
    );
    if (*dctx).rep[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_uint
        || (*dctx).rep[1 as libc::c_int as usize] as libc::c_ulong >= dictSize
    {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    (*dctx)
        .rep[2 as libc::c_int
        as usize] = MEM_readLE32(
        dictPtr.offset(8 as libc::c_int as isize) as *const libc::c_void,
    );
    if (*dctx).rep[2 as libc::c_int as usize] == 0 as libc::c_int as libc::c_uint
        || (*dctx).rep[2 as libc::c_int as usize] as libc::c_ulong >= dictSize
    {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    dictPtr = dictPtr.offset(12 as libc::c_int as isize);
    (*dctx).fseEntropy = 1 as libc::c_int as u32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    return dictPtr.offset_from(dict as *const u8) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_decompress_insertDictionary(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::size_t {
    if dictSize < 8 as libc::c_int as libc::c_ulong {
        return ZSTDv07_refDictContent(dctx, dict, dictSize);
    }
    let magic = MEM_readLE32(dict);
    if magic != ZSTDv07_DICT_MAGIC {
        return ZSTDv07_refDictContent(dctx, dict, dictSize);
    }
    (*dctx)
        .dictID = MEM_readLE32(
        (dict as *const libc::c_char).offset(4 as libc::c_int as isize)
            as *const libc::c_void,
    );
    dict = (dict as *const libc::c_char).offset(8 as libc::c_int as isize)
        as *const libc::c_void;
    dictSize = (dictSize as libc::c_ulong)
        .wrapping_sub(8) ;
    let eSize = ZSTDv07_loadEntropy(dctx, dict, dictSize);
    if ERR_isError(eSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
    }
    dict = (dict as *const libc::c_char).offset(eSize as isize) as *const libc::c_void;
    dictSize = (dictSize as libc::c_ulong).wrapping_sub(eSize) ;
    return ZSTDv07_refDictContent(dctx, dict, dictSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompressBegin_usingDict(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::size_t {
    let errorCode = ZSTDv07_decompressBegin(dctx);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    if !dict.is_null() && dictSize != 0 {
        let errorCode_0 = ZSTDv07_decompress_insertDictionary(dctx, dict, dictSize);
        if ERR_isError(errorCode_0) != 0 {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
        }
    }
    return 0 as libc::c_int as libc::size_t;
}
unsafe extern "C" fn ZSTDv07_createDDict_advanced(
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
    mut customMem: ZSTDv07_customMem,
) -> *mut ZSTDv07_DDict {
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return NULL as *mut ZSTDv07_DDict;
    }
    let ddict = (customMem.customAlloc)
        .expect(
            "non-null function pointer",
        )(customMem.opaque, ::core::mem::size_of::<ZSTDv07_DDict>() as libc::c_ulong)
        as *mut ZSTDv07_DDict;
    let dictContent = (customMem.customAlloc)
        .expect("non-null function pointer")(customMem.opaque, dictSize);
    let dctx = ZSTDv07_createDCtx_advanced(customMem);
    if dictContent.is_null() || ddict.is_null() || dctx.is_null() {
        (customMem.customFree)
            .expect("non-null function pointer")(customMem.opaque, dictContent);
        (customMem.customFree)
            .expect(
                "non-null function pointer",
            )(customMem.opaque, ddict as *mut libc::c_void);
        (customMem.customFree)
            .expect(
                "non-null function pointer",
            )(customMem.opaque, dctx as *mut libc::c_void);
        return NULL as *mut ZSTDv07_DDict;
    }
    memcpy(dictContent, dict, dictSize);
    let errorCode = ZSTDv07_decompressBegin_usingDict(dctx, dictContent, dictSize);
    if ERR_isError(errorCode) != 0 {
        (customMem.customFree)
            .expect("non-null function pointer")(customMem.opaque, dictContent);
        (customMem.customFree)
            .expect(
                "non-null function pointer",
            )(customMem.opaque, ddict as *mut libc::c_void);
        (customMem.customFree)
            .expect(
                "non-null function pointer",
            )(customMem.opaque, dctx as *mut libc::c_void);
        return NULL as *mut ZSTDv07_DDict;
    }
    (*ddict).dict = dictContent;
    (*ddict).dictSize = dictSize;
    (*ddict).refContext = dctx;
    return ddict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_createDDict(
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> *mut ZSTDv07_DDict {
    let allocator = {
        let mut init = ZSTDv07_customMem {
            customAlloc: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTDv07_allocFunction,
            >(NULL as libc::intptr_t),
            customFree: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTDv07_freeFunction,
            >(NULL as libc::intptr_t),
            opaque: NULL as *mut libc::c_void,
        };
        init
    };
    return ZSTDv07_createDDict_advanced(dict, dictSize, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_freeDDict(mut ddict: *mut ZSTDv07_DDict) -> libc::size_t {
    let cFree: ZSTDv07_freeFunction = (*(*ddict).refContext).customMem.customFree;
    let opaque = (*(*ddict).refContext).customMem.opaque;
    ZSTDv07_freeDCtx((*ddict).refContext);
    cFree.expect("non-null function pointer")(opaque, (*ddict).dict);
    cFree.expect("non-null function pointer")(opaque, ddict as *mut libc::c_void);
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv07_decompress_usingDDict(
    mut dctx: *mut ZSTDv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut ddict: *const ZSTDv07_DDict,
) -> libc::size_t {
    return ZSTDv07_decompress_usingPreparedDCtx(
        dctx,
        (*ddict).refContext,
        dst,
        dstCapacity,
        src,
        srcSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_createDCtx() -> *mut ZBUFFv07_DCtx {
    return ZBUFFv07_createDCtx_advanced(defaultCustomMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_createDCtx_advanced(
    mut customMem: ZSTDv07_customMem,
) -> *mut ZBUFFv07_DCtx {
    let mut zbd = 0 as *mut ZBUFFv07_DCtx;
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return NULL as *mut ZBUFFv07_DCtx;
    }
    zbd = (customMem.customAlloc)
        .expect(
            "non-null function pointer",
        )(customMem.opaque, ::core::mem::size_of::<ZBUFFv07_DCtx>() as libc::c_ulong)
        as *mut ZBUFFv07_DCtx;
    if zbd.is_null() {
        return NULL as *mut ZBUFFv07_DCtx;
    }
    memset(
        zbd as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZBUFFv07_DCtx>() as libc::c_ulong,
    );
    memcpy(
        &mut (*zbd).customMem as *mut ZSTDv07_customMem as *mut libc::c_void,
        &mut customMem as *mut ZSTDv07_customMem as *const libc::c_void,
        ::core::mem::size_of::<ZSTDv07_customMem>() as libc::c_ulong,
    );
    (*zbd).zd = ZSTDv07_createDCtx_advanced(customMem);
    if ((*zbd).zd).is_null() {
        ZBUFFv07_freeDCtx(zbd);
        return NULL as *mut ZBUFFv07_DCtx;
    }
    (*zbd).stage = ZBUFFds_init;
    return zbd;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_freeDCtx(mut zbd: *mut ZBUFFv07_DCtx) -> libc::size_t {
    if zbd.is_null() {
        return 0 as libc::c_int as libc::size_t;
    }
    ZSTDv07_freeDCtx((*zbd).zd);
    if !((*zbd).inBuff).is_null() {
        ((*zbd).customMem.customFree)
            .expect(
                "non-null function pointer",
            )((*zbd).customMem.opaque, (*zbd).inBuff as *mut libc::c_void);
    }
    if !((*zbd).outBuff).is_null() {
        ((*zbd).customMem.customFree)
            .expect(
                "non-null function pointer",
            )((*zbd).customMem.opaque, (*zbd).outBuff as *mut libc::c_void);
    }
    ((*zbd).customMem.customFree)
        .expect(
            "non-null function pointer",
        )((*zbd).customMem.opaque, zbd as *mut libc::c_void);
    return 0 as libc::c_int as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_decompressInitDictionary(
    mut zbd: *mut ZBUFFv07_DCtx,
    mut dict: *const libc::c_void,
    mut dictSize: libc::size_t,
) -> libc::size_t {
    (*zbd).stage = ZBUFFds_loadHeader;
    (*zbd).outEnd = 0 as libc::c_int as libc::size_t;
    (*zbd).outStart = (*zbd).outEnd;
    (*zbd).inPos = (*zbd).outStart;
    (*zbd).lhSize = (*zbd).inPos;
    return ZSTDv07_decompressBegin_usingDict((*zbd).zd, dict, dictSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_decompressInit(mut zbd: *mut ZBUFFv07_DCtx) -> libc::size_t {
    return ZBUFFv07_decompressInitDictionary(
        zbd,
        NULL as *const libc::c_void,
        0 as libc::c_int as libc::size_t,
    );
}
unsafe extern "C" fn ZBUFFv07_limitCopy(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let length = if dstCapacity < srcSize { dstCapacity } else { srcSize };
    if length > 0 as libc::c_int as libc::c_ulong {
        memcpy(dst, src, length);
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_decompressContinue(
    mut zbd: *mut ZBUFFv07_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacityPtr: *mut libc::size_t,
    mut src: *const libc::c_void,
    mut srcSizePtr: *mut libc::size_t,
) -> libc::size_t {
    let istart = src as *const libc::c_char;
    let iend = istart.offset(*srcSizePtr as isize);
    let mut ip = istart;
    let ostart = dst as *mut libc::c_char;
    let oend = ostart.offset(*dstCapacityPtr as isize);
    let mut op = ostart;
    let mut notDone = 1 as libc::c_int as u32;
    while notDone != 0 {
        let mut current_block_66: u64;
        match (*zbd).stage as libc::c_uint {
            0 => return -(ZSTD_error_init_missing as libc::c_int) as libc::size_t,
            1 => {
                let hSize = ZSTDv07_getFrameParams(
                    &mut (*zbd).fParams,
                    ((*zbd).headerBuffer).as_mut_ptr() as *const libc::c_void,
                    (*zbd).lhSize,
                );
                if ERR_isError(hSize) != 0 {
                    return hSize;
                }
                if hSize != 0 as libc::c_int as libc::c_ulong {
                    let toLoad = hSize.wrapping_sub((*zbd).lhSize);
                    if toLoad > iend.offset_from(ip) as libc::c_long as libc::size_t {
                        if !ip.is_null() {
                            memcpy(
                                ((*zbd).headerBuffer)
                                    .as_mut_ptr()
                                    .offset((*zbd).lhSize as isize) as *mut libc::c_void,
                                ip as *const libc::c_void,
                                iend.offset_from(ip) as libc::c_long as libc::c_ulong,
                            );
                        }
                        (*zbd)
                            .lhSize = ((*zbd).lhSize as libc::c_ulong)
                            .wrapping_add(
                                iend.offset_from(ip) as libc::c_long as libc::c_ulong,
                            ) ;
                        *dstCapacityPtr = 0 as libc::c_int as libc::size_t;
                        return hSize
                            .wrapping_sub((*zbd).lhSize)
                            .wrapping_add(ZSTDv07_blockHeaderSize);
                    }
                    memcpy(
                        ((*zbd).headerBuffer).as_mut_ptr().offset((*zbd).lhSize as isize)
                            as *mut libc::c_void,
                        ip as *const libc::c_void,
                        toLoad,
                    );
                    (*zbd).lhSize = hSize;
                    ip = ip.offset(toLoad as isize);
                    current_block_66 = 12961834331865314435;
                } else {
                    let h1Size = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                    let h1Result = ZSTDv07_decompressContinue(
                        (*zbd).zd,
                        NULL as *mut libc::c_void,
                        0 as libc::c_int as libc::size_t,
                        ((*zbd).headerBuffer).as_mut_ptr() as *const libc::c_void,
                        h1Size,
                    );
                    if ERR_isError(h1Result) != 0 {
                        return h1Result;
                    }
                    if h1Size < (*zbd).lhSize {
                        let h2Size = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                        let h2Result = ZSTDv07_decompressContinue(
                            (*zbd).zd,
                            NULL as *mut libc::c_void,
                            0 as libc::c_int as libc::size_t,
                            ((*zbd).headerBuffer).as_mut_ptr().offset(h1Size as isize)
                                as *const libc::c_void,
                            h2Size,
                        );
                        if ERR_isError(h2Result) != 0 {
                            return h2Result;
                        }
                    }
                    (*zbd)
                        .fParams
                        .windowSize = if (*zbd).fParams.windowSize
                        > (1 as libc::c_uint) << 10 as libc::c_int
                    {
                        (*zbd).fParams.windowSize
                    } else {
                        (1 as libc::c_uint) << 10 as libc::c_int
                    };
                    let blockSize = (if (*zbd).fParams.windowSize
                        < (128 as libc::c_int * 1024 as libc::c_int) as libc::c_uint
                    {
                        (*zbd).fParams.windowSize
                    } else {
                        (128 as libc::c_int * 1024 as libc::c_int) as libc::c_uint
                    }) as libc::size_t;
                    (*zbd).blockSize = blockSize;
                    if (*zbd).inBuffSize < blockSize {
                        ((*zbd).customMem.customFree)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*zbd).customMem.opaque,
                            (*zbd).inBuff as *mut libc::c_void,
                        );
                        (*zbd).inBuffSize = blockSize;
                        (*zbd)
                            .inBuff = ((*zbd).customMem.customAlloc)
                            .expect(
                                "non-null function pointer",
                            )((*zbd).customMem.opaque, blockSize) as *mut libc::c_char;
                        if ((*zbd).inBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as libc::c_int)
                                as libc::size_t;
                        }
                    }
                    let neededOutSize = ((*zbd).fParams.windowSize as libc::c_ulong)
                        .wrapping_add(blockSize)
                        .wrapping_add(
                            (WILDCOPY_OVERLENGTH * 2 as libc::c_int) as libc::c_ulong,
                        );
                    if (*zbd).outBuffSize < neededOutSize {
                        ((*zbd).customMem.customFree)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*zbd).customMem.opaque,
                            (*zbd).outBuff as *mut libc::c_void,
                        );
                        (*zbd).outBuffSize = neededOutSize;
                        (*zbd)
                            .outBuff = ((*zbd).customMem.customAlloc)
                            .expect(
                                "non-null function pointer",
                            )((*zbd).customMem.opaque, neededOutSize)
                            as *mut libc::c_char;
                        if ((*zbd).outBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as libc::c_int)
                                as libc::size_t;
                        }
                    }
                    (*zbd).stage = ZBUFFds_read;
                    current_block_66 = 8845338526596852646;
                }
            }
            2 => {
                current_block_66 = 8845338526596852646;
            }
            3 => {
                current_block_66 = 14945149239039849694;
            }
            4 => {
                current_block_66 = 5181772461570869434;
            }
            _ => return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t,
        }
        match current_block_66 {
            8845338526596852646 => {
                let neededInSize = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                if neededInSize == 0 as libc::c_int as libc::c_ulong {
                    (*zbd).stage = ZBUFFds_init;
                    notDone = 0 as libc::c_int as u32;
                    current_block_66 = 12961834331865314435;
                } else if iend.offset_from(ip) as libc::c_long as libc::size_t >= neededInSize
                {
                    let isSkipFrame = ZSTDv07_isSkipFrame((*zbd).zd);
                    let decodedSize = ZSTDv07_decompressContinue(
                        (*zbd).zd,
                        ((*zbd).outBuff).offset((*zbd).outStart as isize)
                            as *mut libc::c_void,
                        if isSkipFrame != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart)
                        },
                        ip as *const libc::c_void,
                        neededInSize,
                    );
                    if ERR_isError(decodedSize) != 0 {
                        return decodedSize;
                    }
                    ip = ip.offset(neededInSize as isize);
                    if decodedSize == 0 && isSkipFrame == 0 {
                        current_block_66 = 12961834331865314435;
                    } else {
                        (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize);
                        (*zbd).stage = ZBUFFds_flush;
                        current_block_66 = 12961834331865314435;
                    }
                } else if ip == iend {
                    notDone = 0 as libc::c_int as u32;
                    current_block_66 = 12961834331865314435;
                } else {
                    (*zbd).stage = ZBUFFds_load;
                    current_block_66 = 14945149239039849694;
                }
            }
            _ => {}
        }
        match current_block_66 {
            14945149239039849694 => {
                let neededInSize_0 = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                let toLoad_0 = neededInSize_0.wrapping_sub((*zbd).inPos);
                let mut loadedSize: libc::size_t = 0;
                if toLoad_0 > ((*zbd).inBuffSize).wrapping_sub((*zbd).inPos) {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
                }
                loadedSize = ZBUFFv07_limitCopy(
                    ((*zbd).inBuff).offset((*zbd).inPos as isize) as *mut libc::c_void,
                    toLoad_0,
                    ip as *const libc::c_void,
                    iend.offset_from(ip) as libc::c_long as libc::size_t,
                );
                ip = ip.offset(loadedSize as isize);
                (*zbd)
                    .inPos = ((*zbd).inPos as libc::c_ulong).wrapping_add(loadedSize)
                    ;
                if loadedSize < toLoad_0 {
                    notDone = 0 as libc::c_int as u32;
                    current_block_66 = 12961834331865314435;
                } else {
                    let isSkipFrame_0 = ZSTDv07_isSkipFrame((*zbd).zd);
                    let decodedSize_0 = ZSTDv07_decompressContinue(
                        (*zbd).zd,
                        ((*zbd).outBuff).offset((*zbd).outStart as isize)
                            as *mut libc::c_void,
                        ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart),
                        (*zbd).inBuff as *const libc::c_void,
                        neededInSize_0,
                    );
                    if ERR_isError(decodedSize_0) != 0 {
                        return decodedSize_0;
                    }
                    (*zbd).inPos = 0 as libc::c_int as libc::size_t;
                    if decodedSize_0 == 0 && isSkipFrame_0 == 0 {
                        (*zbd).stage = ZBUFFds_read;
                        current_block_66 = 12961834331865314435;
                    } else {
                        (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize_0);
                        (*zbd).stage = ZBUFFds_flush;
                        current_block_66 = 5181772461570869434;
                    }
                }
            }
            _ => {}
        }
        match current_block_66 {
            5181772461570869434 => {
                let toFlushSize = ((*zbd).outEnd).wrapping_sub((*zbd).outStart);
                let flushedSize = ZBUFFv07_limitCopy(
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as libc::size_t,
                    ((*zbd).outBuff).offset((*zbd).outStart as isize)
                        as *const libc::c_void,
                    toFlushSize,
                );
                op = op.offset(flushedSize as isize);
                (*zbd)
                    .outStart = ((*zbd).outStart as libc::c_ulong)
                    .wrapping_add(flushedSize) ;
                if flushedSize == toFlushSize {
                    (*zbd).stage = ZBUFFds_read;
                    if ((*zbd).outStart).wrapping_add((*zbd).blockSize)
                        > (*zbd).outBuffSize
                    {
                        (*zbd).outEnd = 0 as libc::c_int as libc::size_t;
                        (*zbd).outStart = (*zbd).outEnd;
                    }
                } else {
                    notDone = 0 as libc::c_int as u32;
                }
            }
            _ => {}
        }
    }
    *srcSizePtr = ip.offset_from(istart) as libc::c_long as libc::size_t;
    *dstCapacityPtr = op.offset_from(ostart) as libc::c_long as libc::size_t;
    let mut nextSrcSizeHint = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
    nextSrcSizeHint = (nextSrcSizeHint as libc::c_ulong).wrapping_sub((*zbd).inPos)
        ;
    return nextSrcSizeHint;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_recommendedDInSize() -> libc::size_t {
    return (ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as libc::c_ulong)
        .wrapping_add(ZSTDv07_blockHeaderSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv07_recommendedDOutSize() -> libc::size_t {
    return ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as libc::size_t;
}
