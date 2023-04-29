use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use core::arch::asm;
extern "C" {
    pub type ZSTD_DDict_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn FSE_readNCount(
        normalizedCounter: *mut libc::c_short,
        maxSymbolValuePtr: *mut libc::c_uint,
        tableLogPtr: *mut libc::c_uint,
        rBuffer: *const libc::c_void,
        rBuffSize: size_t,
    ) -> size_t;
    fn HUF_decompress1X_usingDTable(
        dst: *mut libc::c_void,
        maxDstSize: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
        DTable: *const HUF_DTable,
        flags: libc::c_int,
    ) -> size_t;
    fn HUF_decompress1X1_DCtx_wksp(
        dctx: *mut HUF_DTable,
        dst: *mut libc::c_void,
        dstSize: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
        flags: libc::c_int,
    ) -> size_t;
    fn HUF_decompress4X_usingDTable(
        dst: *mut libc::c_void,
        maxDstSize: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
        DTable: *const HUF_DTable,
        flags: libc::c_int,
    ) -> size_t;
    fn HUF_decompress4X_hufOnly_wksp(
        dctx: *mut HUF_DTable,
        dst: *mut libc::c_void,
        dstSize: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
        flags: libc::c_int,
    ) -> size_t;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
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
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [U32; 640],
    pub previousDstEnd: *const libc::c_void,
    pub prefixStart: *const libc::c_void,
    pub virtualStart: *const libc::c_void,
    pub dictEnd: *const libc::c_void,
    pub expected: size_t,
    pub fParams: ZSTD_frameHeader,
    pub processedCSize: U64,
    pub decodedSize: U64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: U32,
    pub fseEntropy: U32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: U32,
    pub litPtr: *const BYTE,
    pub customMem: ZSTD_customMem,
    pub litSize: size_t,
    pub rleSize: size_t,
    pub staticSize: size_t,
    pub isFrameDecompression: libc::c_int,
    pub bmi2: libc::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: U32,
    pub ddictIsCold: libc::c_int,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: libc::c_int,
    pub maxBlockSizeParam: libc::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub maxWindowSize: size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub lhSize: size_t,
    pub legacyContext: *mut libc::c_void,
    pub previousLegacyVersion: U32,
    pub legacyVersion: U32,
    pub hostageByte: U32,
    pub noForwardProgress: libc::c_int,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut BYTE,
    pub litBufferEnd: *const BYTE,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [BYTE; 65568],
    pub headerBuffer: [BYTE; 18],
    pub oversizedDuration: size_t,
    pub traceCtx: ZSTD_TraceCtx,
}
pub type ZSTD_TraceCtx = libc::c_ulonglong;
pub type ZSTD_litLocation_e = libc::c_uint;
pub const ZSTD_split: ZSTD_litLocation_e = 2;
pub const ZSTD_in_dst: ZSTD_litLocation_e = 1;
pub const ZSTD_not_in_dst: ZSTD_litLocation_e = 0;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_bufferMode_e = libc::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
pub type ZSTD_dStreamStage = libc::c_uint;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const zdss_read: ZSTD_dStreamStage = 2;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const zdss_init: ZSTD_dStreamStage = 0;
pub type ZSTD_refMultipleDDicts_e = libc::c_uint;
pub const ZSTD_rmd_refMultipleDDicts: ZSTD_refMultipleDDicts_e = 1;
pub const ZSTD_rmd_refSingleDDict: ZSTD_refMultipleDDicts_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDictHashSet {
    pub ddictPtrTable: *mut *const ZSTD_DDict,
    pub ddictPtrTableSize: size_t,
    pub ddictPtrCount: size_t,
}
pub type ZSTD_DDict = ZSTD_DDict_s;
pub type ZSTD_dictUses_e = libc::c_int;
pub const ZSTD_use_once: ZSTD_dictUses_e = 1;
pub const ZSTD_dont_use: ZSTD_dictUses_e = 0;
pub const ZSTD_use_indefinitely: ZSTD_dictUses_e = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type ZSTD_forceIgnoreChecksum_e = libc::c_uint;
pub const ZSTD_d_ignoreChecksum: ZSTD_forceIgnoreChecksum_e = 1;
pub const ZSTD_d_validateChecksum: ZSTD_forceIgnoreChecksum_e = 0;
pub type ZSTD_format_e = libc::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub type XXH64_state_t = XXH64_state_s;
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
pub type XXH64_hash_t = uint64_t;
pub type XXH32_hash_t = uint32_t;
pub type ZSTD_dStage = libc::c_uint;
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;
pub type blockType_e = libc::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameHeader {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_ulonglong,
    pub blockSizeMax: libc::c_uint,
    pub frameType: ZSTD_frameType_e,
    pub headerSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
    pub _reserved1: libc::c_uint,
    pub _reserved2: libc::c_uint,
}
pub type ZSTD_frameType_e = libc::c_uint;
pub const ZSTD_skippableFrame: ZSTD_frameType_e = 1;
pub const ZSTD_frame: ZSTD_frameType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; 513],
    pub OFTable: [ZSTD_seqSymbol; 257],
    pub MLTable: [ZSTD_seqSymbol; 513],
    pub hufTable: [HUF_DTable; 4097],
    pub rep: [U32; 3],
    pub workspace: [U32; 157],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol {
    pub nextState: U16,
    pub nbAdditionalBits: BYTE,
    pub nbBits: BYTE,
    pub baseValue: U32,
}
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type streaming_operation = libc::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;
pub type ZSTD_longOffset_e = libc::c_uint;
pub const ZSTD_lo_isLongOffset: ZSTD_longOffset_e = 1;
pub const ZSTD_lo_isRegularOffset: ZSTD_longOffset_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BIT_DStream_t,
    pub stateLL: ZSTD_fseState,
    pub stateOffb: ZSTD_fseState,
    pub stateML: ZSTD_fseState,
    pub prevOffset: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseState {
    pub state: size_t,
    pub table: *const ZSTD_seqSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub matchLength: size_t,
    pub offset: size_t,
}
pub type ZSTD_overlap_e = libc::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol_header {
    pub fastMode: U32,
    pub tableLog: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_OffsetInfo {
    pub longOffsetShare: libc::c_uint,
    pub maxNbAdditionalBits: libc::c_uint,
}
pub type symbolEncodingType_e = libc::c_uint;
pub const set_repeat: symbolEncodingType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub const set_rle: symbolEncodingType_e = 1;
pub const set_basic: symbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: U32,
    pub origSize: U32,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const CACHELINE_SIZE: libc::c_int = 64 as libc::c_int;
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
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as U32)
        .wrapping_add(
            (*(memPtr as *const BYTE).offset(2 as libc::c_int as isize) as U32)
                << 16 as libc::c_int,
        );
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
                current_block_32 = 16663583946819950022;
            }
            6 => {
                current_block_32 = 16663583946819950022;
            }
            5 => {
                current_block_32 = 5467347503347430154;
            }
            4 => {
                current_block_32 = 15935546777885233963;
            }
            3 => {
                current_block_32 = 15098265657425327076;
            }
            2 => {
                current_block_32 = 18096294377129956667;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        match current_block_32 {
            16663583946819950022 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(5 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(24 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 5467347503347430154;
            }
            _ => {}
        }
        match current_block_32 {
            5467347503347430154 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(4 as libc::c_int as isize)
                            as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(32 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_32 = 15935546777885233963;
            }
            _ => {}
        }
        match current_block_32 {
            15935546777885233963 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(3 as libc::c_int as isize)
                            as size_t) << 24 as libc::c_int,
                    ) as size_t as size_t;
                current_block_32 = 15098265657425327076;
            }
            _ => {}
        }
        match current_block_32 {
            15098265657425327076 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const BYTE).offset(2 as libc::c_int as isize)
                            as size_t) << 16 as libc::c_int,
                    ) as size_t as size_t;
                current_block_32 = 18096294377129956667;
            }
            _ => {}
        }
        match current_block_32 {
            18096294377129956667 => {
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
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: libc::c_uint,
) -> size_t {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
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
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
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
pub const STREAM_ACCUMULATOR_MIN_64: libc::c_int = 57 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
pub const STREAM_ACCUMULATOR_MIN_32: libc::c_int = 25 as libc::c_int;
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1 as libc::c_int) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_WINDOWLOG_MAX_32: libc::c_int = 30 as libc::c_int;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
pub const ZSTD_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
static mut ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
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
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/../common/zstd_internal.h\0"
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
unsafe extern "C" fn ZSTD_copy16(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
unsafe extern "C" fn ZSTD_copy8(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    libc::memcpy(dst, src, 8 as libc::c_int as libc::c_ulong as libc::size_t);
}
pub const WILDCOPY_VECLEN: libc::c_int = 16 as libc::c_int;
pub const WILDCOPY_OVERLENGTH: libc::c_int = 32 as libc::c_int;
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
pub const OffFSELog: libc::c_int = 8 as libc::c_int;
pub const LONGNBSEQ: libc::c_int = 0x7f00 as libc::c_int;
pub const ML_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
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
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const MLFSELog: libc::c_int = 9 as libc::c_int;
pub const MaxML: libc::c_int = 52 as libc::c_int;
pub const LLFSELog: libc::c_int = 9 as libc::c_int;
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
pub const LL_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
pub const MaxOff: libc::c_int = 31 as libc::c_int;
pub const OF_DEFAULTNORMLOG: libc::c_int = 5 as libc::c_int;
static mut LL_base: [U32; 36] = [
    0 as libc::c_int as U32,
    1 as libc::c_int as U32,
    2 as libc::c_int as U32,
    3 as libc::c_int as U32,
    4 as libc::c_int as U32,
    5 as libc::c_int as U32,
    6 as libc::c_int as U32,
    7 as libc::c_int as U32,
    8 as libc::c_int as U32,
    9 as libc::c_int as U32,
    10 as libc::c_int as U32,
    11 as libc::c_int as U32,
    12 as libc::c_int as U32,
    13 as libc::c_int as U32,
    14 as libc::c_int as U32,
    15 as libc::c_int as U32,
    16 as libc::c_int as U32,
    18 as libc::c_int as U32,
    20 as libc::c_int as U32,
    22 as libc::c_int as U32,
    24 as libc::c_int as U32,
    28 as libc::c_int as U32,
    32 as libc::c_int as U32,
    40 as libc::c_int as U32,
    48 as libc::c_int as U32,
    64 as libc::c_int as U32,
    0x80 as libc::c_int as U32,
    0x100 as libc::c_int as U32,
    0x200 as libc::c_int as U32,
    0x400 as libc::c_int as U32,
    0x800 as libc::c_int as U32,
    0x1000 as libc::c_int as U32,
    0x2000 as libc::c_int as U32,
    0x4000 as libc::c_int as U32,
    0x8000 as libc::c_int as U32,
    0x10000 as libc::c_int as U32,
];
static mut OF_base: [U32; 32] = [
    0 as libc::c_int as U32,
    1 as libc::c_int as U32,
    1 as libc::c_int as U32,
    5 as libc::c_int as U32,
    0xd as libc::c_int as U32,
    0x1d as libc::c_int as U32,
    0x3d as libc::c_int as U32,
    0x7d as libc::c_int as U32,
    0xfd as libc::c_int as U32,
    0x1fd as libc::c_int as U32,
    0x3fd as libc::c_int as U32,
    0x7fd as libc::c_int as U32,
    0xffd as libc::c_int as U32,
    0x1ffd as libc::c_int as U32,
    0x3ffd as libc::c_int as U32,
    0x7ffd as libc::c_int as U32,
    0xfffd as libc::c_int as U32,
    0x1fffd as libc::c_int as U32,
    0x3fffd as libc::c_int as U32,
    0x7fffd as libc::c_int as U32,
    0xffffd as libc::c_int as U32,
    0x1ffffd as libc::c_int as U32,
    0x3ffffd as libc::c_int as U32,
    0x7ffffd as libc::c_int as U32,
    0xfffffd as libc::c_int as U32,
    0x1fffffd as libc::c_int as U32,
    0x3fffffd as libc::c_int as U32,
    0x7fffffd as libc::c_int as U32,
    0xffffffd as libc::c_int as U32,
    0x1ffffffd as libc::c_int as U32,
    0x3ffffffd as libc::c_int as U32,
    0x7ffffffd as libc::c_int as U32,
];
static mut OF_bits: [U8; 32] = [
    0 as libc::c_int as U8,
    1 as libc::c_int as U8,
    2 as libc::c_int as U8,
    3 as libc::c_int as U8,
    4 as libc::c_int as U8,
    5 as libc::c_int as U8,
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
    17 as libc::c_int as U8,
    18 as libc::c_int as U8,
    19 as libc::c_int as U8,
    20 as libc::c_int as U8,
    21 as libc::c_int as U8,
    22 as libc::c_int as U8,
    23 as libc::c_int as U8,
    24 as libc::c_int as U8,
    25 as libc::c_int as U8,
    26 as libc::c_int as U8,
    27 as libc::c_int as U8,
    28 as libc::c_int as U8,
    29 as libc::c_int as U8,
    30 as libc::c_int as U8,
    31 as libc::c_int as U8,
];
static mut ML_base: [U32; 53] = [
    3 as libc::c_int as U32,
    4 as libc::c_int as U32,
    5 as libc::c_int as U32,
    6 as libc::c_int as U32,
    7 as libc::c_int as U32,
    8 as libc::c_int as U32,
    9 as libc::c_int as U32,
    10 as libc::c_int as U32,
    11 as libc::c_int as U32,
    12 as libc::c_int as U32,
    13 as libc::c_int as U32,
    14 as libc::c_int as U32,
    15 as libc::c_int as U32,
    16 as libc::c_int as U32,
    17 as libc::c_int as U32,
    18 as libc::c_int as U32,
    19 as libc::c_int as U32,
    20 as libc::c_int as U32,
    21 as libc::c_int as U32,
    22 as libc::c_int as U32,
    23 as libc::c_int as U32,
    24 as libc::c_int as U32,
    25 as libc::c_int as U32,
    26 as libc::c_int as U32,
    27 as libc::c_int as U32,
    28 as libc::c_int as U32,
    29 as libc::c_int as U32,
    30 as libc::c_int as U32,
    31 as libc::c_int as U32,
    32 as libc::c_int as U32,
    33 as libc::c_int as U32,
    34 as libc::c_int as U32,
    35 as libc::c_int as U32,
    37 as libc::c_int as U32,
    39 as libc::c_int as U32,
    41 as libc::c_int as U32,
    43 as libc::c_int as U32,
    47 as libc::c_int as U32,
    51 as libc::c_int as U32,
    59 as libc::c_int as U32,
    67 as libc::c_int as U32,
    83 as libc::c_int as U32,
    99 as libc::c_int as U32,
    0x83 as libc::c_int as U32,
    0x103 as libc::c_int as U32,
    0x203 as libc::c_int as U32,
    0x403 as libc::c_int as U32,
    0x803 as libc::c_int as U32,
    0x1003 as libc::c_int as U32,
    0x2003 as libc::c_int as U32,
    0x4003 as libc::c_int as U32,
    0x8003 as libc::c_int as U32,
    0x10003 as libc::c_int as U32,
];
#[inline]
unsafe extern "C" fn ZSTD_DCtx_get_bmi2(mut dctx: *const ZSTD_DCtx_s) -> libc::c_int {
    return (*dctx).bmi2;
}
unsafe extern "C" fn ZSTD_copy4(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    libc::memcpy(dst, src, 4 as libc::c_int as libc::c_ulong as libc::size_t);
}
unsafe extern "C" fn ZSTD_blockSizeMax(mut dctx: *const ZSTD_DCtx) -> size_t {
    let blockSizeMax = (if (*dctx).isFrameDecompression != 0 {
        (*dctx).fParams.blockSizeMax
    } else {
        ZSTD_BLOCKSIZE_MAX as libc::c_uint
    }) as size_t;
    if blockSizeMax <= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong
    {} else {
        __assert_fail(
            b"blockSizeMax <= ZSTD_BLOCKSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"size_t ZSTD_blockSizeMax(const ZSTD_DCtx *)\0"))
                .as_ptr(),
        );
    }
    return blockSizeMax;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getcBlockSize(
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let cBlockHeader = MEM_readLE24(src);
    let cSize = cBlockHeader >> 3 as libc::c_int;
    (*bpPtr).lastBlock = cBlockHeader & 1 as libc::c_int as libc::c_uint;
    (*bpPtr)
        .blockType = (cBlockHeader >> 1 as libc::c_int
        & 3 as libc::c_int as libc::c_uint) as blockType_e;
    (*bpPtr).origSize = cSize;
    if (*bpPtr).blockType as libc::c_uint == bt_rle as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as size_t;
    }
    if (*bpPtr).blockType as libc::c_uint == bt_reserved as libc::c_int as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    return cSize as size_t;
}
unsafe extern "C" fn ZSTD_allocateLiteralsBuffer(
    mut dctx: *mut ZSTD_DCtx,
    dst: *mut libc::c_void,
    dstCapacity: size_t,
    litSize: size_t,
    streaming: streaming_operation,
    expectedWriteSize: size_t,
    splitImmediately: libc::c_uint,
) {
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    if litSize <= blockSizeMax {} else {
        __assert_fail(
            b"litSize <= blockSizeMax\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"void ZSTD_allocateLiteralsBuffer(ZSTD_DCtx *, void *const, const size_t, const size_t, const streaming_operation, const size_t, const unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*dctx).isFrameDecompression != 0
        || streaming as libc::c_uint == not_streaming as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"dctx->isFrameDecompression || streaming == not_streaming\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"void ZSTD_allocateLiteralsBuffer(ZSTD_DCtx *, void *const, const size_t, const size_t, const streaming_operation, const size_t, const unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    if expectedWriteSize <= blockSizeMax {} else {
        __assert_fail(
            b"expectedWriteSize <= blockSizeMax\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"void ZSTD_allocateLiteralsBuffer(ZSTD_DCtx *, void *const, const size_t, const size_t, const streaming_operation, const size_t, const unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    if streaming as libc::c_uint == not_streaming as libc::c_int as libc::c_uint
        && dstCapacity
            > blockSizeMax
                .wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong)
                .wrapping_add(litSize)
                .wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong)
    {
        (*dctx)
            .litBuffer = (dst as *mut BYTE)
            .offset(blockSizeMax as isize)
            .offset(WILDCOPY_OVERLENGTH as isize);
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_in_dst;
    } else if litSize
        <= (if 64 as libc::c_int
            > (if ((1 as libc::c_int) << 16 as libc::c_int)
                < (128 as libc::c_int) << 10 as libc::c_int
            {
                (1 as libc::c_int) << 16 as libc::c_int
            } else {
                (128 as libc::c_int) << 10 as libc::c_int
            })
        {
            64 as libc::c_int
        } else {
            (if ((1 as libc::c_int) << 16 as libc::c_int)
                < (128 as libc::c_int) << 10 as libc::c_int
            {
                (1 as libc::c_int) << 16 as libc::c_int
            } else {
                (128 as libc::c_int) << 10 as libc::c_int
            })
        }) as libc::c_ulong
    {
        (*dctx).litBuffer = ((*dctx).litExtraBuffer).as_mut_ptr();
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    } else {
        if blockSizeMax
            > (if 64 as libc::c_int
                > (if ((1 as libc::c_int) << 16 as libc::c_int)
                    < (128 as libc::c_int) << 10 as libc::c_int
                {
                    (1 as libc::c_int) << 16 as libc::c_int
                } else {
                    (128 as libc::c_int) << 10 as libc::c_int
                })
            {
                64 as libc::c_int
            } else {
                (if ((1 as libc::c_int) << 16 as libc::c_int)
                    < (128 as libc::c_int) << 10 as libc::c_int
                {
                    (1 as libc::c_int) << 16 as libc::c_int
                } else {
                    (128 as libc::c_int) << 10 as libc::c_int
                })
            }) as libc::c_ulong
        {} else {
            __assert_fail(
                b"blockSizeMax > ZSTD_LITBUFFEREXTRASIZE\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 148],
                    &[libc::c_char; 148],
                >(
                    b"void ZSTD_allocateLiteralsBuffer(ZSTD_DCtx *, void *const, const size_t, const size_t, const streaming_operation, const size_t, const unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        if splitImmediately != 0 {
            (*dctx)
                .litBuffer = (dst as *mut BYTE)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize))
                .offset(
                    (if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    }) as isize,
                )
                .offset(-(WILDCOPY_OVERLENGTH as isize));
            (*dctx)
                .litBufferEnd = ((*dctx).litBuffer)
                .offset(litSize as isize)
                .offset(
                    -((if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    }) as isize),
                );
        } else {
            (*dctx)
                .litBuffer = (dst as *mut BYTE)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize));
            (*dctx).litBufferEnd = (dst as *mut BYTE).offset(expectedWriteSize as isize);
        }
        (*dctx).litBufferLocation = ZSTD_split;
        if (*dctx).litBufferEnd
            <= (dst as *mut BYTE).offset(expectedWriteSize as isize) as *const BYTE
        {} else {
            __assert_fail(
                b"dctx->litBufferEnd <= (BYTE*)dst + expectedWriteSize\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 148],
                    &[libc::c_char; 148],
                >(
                    b"void ZSTD_allocateLiteralsBuffer(ZSTD_DCtx *, void *const, const size_t, const size_t, const streaming_operation, const size_t, const unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn ZSTD_decodeLiteralsBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    streaming: streaming_operation,
) -> size_t {
    if srcSize < (1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let istart = src as *const BYTE;
    let litEncType = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
        & 3 as libc::c_int) as symbolEncodingType_e;
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    match litEncType as libc::c_uint {
        3 => {
            if (*dctx).litEntropy == 0 as libc::c_int as libc::c_uint {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t;
            }
        }
        2 => {}
        0 => {
            let mut litSize_0: size_t = 0;
            let mut lhSize_0: size_t = 0;
            let lhlCode_0 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int & 3 as libc::c_int) as U32;
            let mut expectedWriteSize_0 = if blockSizeMax < dstCapacity {
                blockSizeMax
            } else {
                dstCapacity
            };
            match lhlCode_0 {
                1 => {
                    lhSize_0 = 2 as libc::c_int as size_t;
                    litSize_0 = (MEM_readLE16(istart as *const libc::c_void)
                        as libc::c_int >> 4 as libc::c_int) as size_t;
                }
                3 => {
                    lhSize_0 = 3 as libc::c_int as size_t;
                    if srcSize < 3 as libc::c_int as libc::c_ulong {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as size_t;
                    }
                    litSize_0 = (MEM_readLE24(istart as *const libc::c_void)
                        >> 4 as libc::c_int) as size_t;
                }
                0 | 2 | _ => {
                    lhSize_0 = 1 as libc::c_int as size_t;
                    litSize_0 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        >> 3 as libc::c_int) as size_t;
                }
            }
            if litSize_0 > 0 as libc::c_int as libc::c_ulong && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            if litSize_0 > blockSizeMax {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            if expectedWriteSize_0 < litSize_0 {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst,
                dstCapacity,
                litSize_0,
                streaming,
                expectedWriteSize_0,
                1 as libc::c_int as libc::c_uint,
            );
            if lhSize_0
                .wrapping_add(litSize_0)
                .wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong) > srcSize
            {
                if litSize_0.wrapping_add(lhSize_0) > srcSize {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
                }
                if (*dctx).litBufferLocation as libc::c_uint
                    == ZSTD_split as libc::c_int as libc::c_uint
                {
                    libc::memcpy(
                        (*dctx).litBuffer as *mut libc::c_void,
                        istart.offset(lhSize_0 as isize) as *const libc::c_void,
                        litSize_0
                            .wrapping_sub(
                                (if 64 as libc::c_int
                                    > (if ((1 as libc::c_int) << 16 as libc::c_int)
                                        < (128 as libc::c_int) << 10 as libc::c_int
                                    {
                                        (1 as libc::c_int) << 16 as libc::c_int
                                    } else {
                                        (128 as libc::c_int) << 10 as libc::c_int
                                    })
                                {
                                    64 as libc::c_int
                                } else {
                                    (if ((1 as libc::c_int) << 16 as libc::c_int)
                                        < (128 as libc::c_int) << 10 as libc::c_int
                                    {
                                        (1 as libc::c_int) << 16 as libc::c_int
                                    } else {
                                        (128 as libc::c_int) << 10 as libc::c_int
                                    })
                                }) as libc::c_ulong,
                            ) as libc::size_t,
                    );
                    libc::memcpy(
                        ((*dctx).litExtraBuffer).as_mut_ptr() as *mut libc::c_void,
                        istart
                            .offset(lhSize_0 as isize)
                            .offset(litSize_0 as isize)
                            .offset(
                                -((if 64 as libc::c_int
                                    > (if ((1 as libc::c_int) << 16 as libc::c_int)
                                        < (128 as libc::c_int) << 10 as libc::c_int
                                    {
                                        (1 as libc::c_int) << 16 as libc::c_int
                                    } else {
                                        (128 as libc::c_int) << 10 as libc::c_int
                                    })
                                {
                                    64 as libc::c_int
                                } else {
                                    (if ((1 as libc::c_int) << 16 as libc::c_int)
                                        < (128 as libc::c_int) << 10 as libc::c_int
                                    {
                                        (1 as libc::c_int) << 16 as libc::c_int
                                    } else {
                                        (128 as libc::c_int) << 10 as libc::c_int
                                    })
                                }) as isize),
                            ) as *const libc::c_void,
                        (if 64 as libc::c_int
                            > (if ((1 as libc::c_int) << 16 as libc::c_int)
                                < (128 as libc::c_int) << 10 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                (128 as libc::c_int) << 10 as libc::c_int
                            })
                        {
                            64 as libc::c_int
                        } else if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        }) as libc::c_ulong as libc::size_t,
                    );
                } else {
                    libc::memcpy(
                        (*dctx).litBuffer as *mut libc::c_void,
                        istart.offset(lhSize_0 as isize) as *const libc::c_void,
                        litSize_0 as libc::size_t,
                    );
                }
                (*dctx).litPtr = (*dctx).litBuffer;
                (*dctx).litSize = litSize_0;
                return lhSize_0.wrapping_add(litSize_0);
            }
            (*dctx).litPtr = istart.offset(lhSize_0 as isize);
            (*dctx).litSize = litSize_0;
            (*dctx).litBufferEnd = ((*dctx).litPtr).offset(litSize_0 as isize);
            (*dctx).litBufferLocation = ZSTD_not_in_dst;
            return lhSize_0.wrapping_add(litSize_0);
        }
        1 => {
            let lhlCode_1 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int & 3 as libc::c_int) as U32;
            let mut litSize_1: size_t = 0;
            let mut lhSize_1: size_t = 0;
            let mut expectedWriteSize_1 = if blockSizeMax < dstCapacity {
                blockSizeMax
            } else {
                dstCapacity
            };
            match lhlCode_1 {
                1 => {
                    lhSize_1 = 2 as libc::c_int as size_t;
                    if srcSize < 3 as libc::c_int as libc::c_ulong {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as size_t;
                    }
                    litSize_1 = (MEM_readLE16(istart as *const libc::c_void)
                        as libc::c_int >> 4 as libc::c_int) as size_t;
                }
                3 => {
                    lhSize_1 = 3 as libc::c_int as size_t;
                    if srcSize < 4 as libc::c_int as libc::c_ulong {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as size_t;
                    }
                    litSize_1 = (MEM_readLE24(istart as *const libc::c_void)
                        >> 4 as libc::c_int) as size_t;
                }
                0 | 2 | _ => {
                    lhSize_1 = 1 as libc::c_int as size_t;
                    litSize_1 = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
                        >> 3 as libc::c_int) as size_t;
                }
            }
            if litSize_1 > 0 as libc::c_int as libc::c_ulong && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            if litSize_1 > blockSizeMax {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            if expectedWriteSize_1 < litSize_1 {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst,
                dstCapacity,
                litSize_1,
                streaming,
                expectedWriteSize_1,
                1 as libc::c_int as libc::c_uint,
            );
            if (*dctx).litBufferLocation as libc::c_uint
                == ZSTD_split as libc::c_int as libc::c_uint
            {
                libc::memset(
                    (*dctx).litBuffer as *mut libc::c_void,
                    *istart.offset(lhSize_1 as isize) as libc::c_int,
                    litSize_1
                        .wrapping_sub(
                            (if 64 as libc::c_int
                                > (if ((1 as libc::c_int) << 16 as libc::c_int)
                                    < (128 as libc::c_int) << 10 as libc::c_int
                                {
                                    (1 as libc::c_int) << 16 as libc::c_int
                                } else {
                                    (128 as libc::c_int) << 10 as libc::c_int
                                })
                            {
                                64 as libc::c_int
                            } else {
                                (if ((1 as libc::c_int) << 16 as libc::c_int)
                                    < (128 as libc::c_int) << 10 as libc::c_int
                                {
                                    (1 as libc::c_int) << 16 as libc::c_int
                                } else {
                                    (128 as libc::c_int) << 10 as libc::c_int
                                })
                            }) as libc::c_ulong,
                        ) as libc::size_t,
                );
                libc::memset(
                    ((*dctx).litExtraBuffer).as_mut_ptr() as *mut libc::c_void,
                    *istart.offset(lhSize_1 as isize) as libc::c_int,
                    (if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    }) as libc::c_ulong as libc::size_t,
                );
            } else {
                libc::memset(
                    (*dctx).litBuffer as *mut libc::c_void,
                    *istart.offset(lhSize_1 as isize) as libc::c_int,
                    litSize_1 as libc::size_t,
                );
            }
            (*dctx).litPtr = (*dctx).litBuffer;
            (*dctx).litSize = litSize_1;
            return lhSize_1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        _ => return -(ZSTD_error_corruption_detected as libc::c_int) as size_t,
    }
    if srcSize < 5 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let mut lhSize: size_t = 0;
    let mut litSize: size_t = 0;
    let mut litCSize: size_t = 0;
    let mut singleStream = 0 as libc::c_int as U32;
    let lhlCode = (*istart.offset(0 as libc::c_int as isize) as libc::c_int
        >> 2 as libc::c_int & 3 as libc::c_int) as U32;
    let lhc = MEM_readLE32(istart as *const libc::c_void);
    let mut hufSuccess: size_t = 0;
    let mut expectedWriteSize = if blockSizeMax < dstCapacity {
        blockSizeMax
    } else {
        dstCapacity
    };
    let flags = 0 as libc::c_int
        | (if ZSTD_DCtx_get_bmi2(dctx) != 0 {
            HUF_flags_bmi2 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*dctx).disableHufAsm != 0 {
            HUF_flags_disableAsm as libc::c_int
        } else {
            0 as libc::c_int
        });
    match lhlCode {
        2 => {
            lhSize = 4 as libc::c_int as size_t;
            litSize = (lhc >> 4 as libc::c_int & 0x3fff as libc::c_int as libc::c_uint)
                as size_t;
            litCSize = (lhc >> 18 as libc::c_int) as size_t;
        }
        3 => {
            lhSize = 5 as libc::c_int as size_t;
            litSize = (lhc >> 4 as libc::c_int & 0x3ffff as libc::c_int as libc::c_uint)
                as size_t;
            litCSize = ((lhc >> 22 as libc::c_int) as libc::c_ulong)
                .wrapping_add(
                    (*istart.offset(4 as libc::c_int as isize) as size_t)
                        << 10 as libc::c_int,
                );
        }
        0 | 1 | _ => {
            singleStream = (lhlCode == 0) as libc::c_int as U32;
            lhSize = 3 as libc::c_int as size_t;
            litSize = (lhc >> 4 as libc::c_int & 0x3ff as libc::c_int as libc::c_uint)
                as size_t;
            litCSize = (lhc >> 14 as libc::c_int & 0x3ff as libc::c_int as libc::c_uint)
                as size_t;
        }
    }
    if litSize > 0 as libc::c_int as libc::c_ulong && dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if litSize > blockSizeMax {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if singleStream == 0 {
        if litSize < 6 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_literals_headerWrong as libc::c_int) as size_t;
        }
    }
    if litCSize.wrapping_add(lhSize) > srcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if expectedWriteSize < litSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    ZSTD_allocateLiteralsBuffer(
        dctx,
        dst,
        dstCapacity,
        litSize,
        streaming,
        expectedWriteSize,
        0 as libc::c_int as libc::c_uint,
    );
    if (*dctx).ddictIsCold != 0 && litSize > 768 as libc::c_int as libc::c_ulong {
        let _ptr = (*dctx).HUFptr as *const libc::c_char;
        let _size = ::core::mem::size_of::<[HUF_DTable; 4097]>() as libc::c_ulong;
        let mut _pos: size_t = 0;
        _pos = 0 as libc::c_int as size_t;
        while _pos < _size {
            _pos = (_pos as libc::c_ulong).wrapping_add(CACHELINE_SIZE as libc::c_ulong)
                as size_t as size_t;
        }
    }
    if litEncType as libc::c_uint == set_repeat as libc::c_int as libc::c_uint {
        if singleStream != 0 {
            hufSuccess = HUF_decompress1X_usingDTable(
                (*dctx).litBuffer as *mut libc::c_void,
                litSize,
                istart.offset(lhSize as isize) as *const libc::c_void,
                litCSize,
                (*dctx).HUFptr,
                flags,
            );
        } else {
            if litSize >= 6 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"litSize >= MIN_LITERALS_FOR_4_STREAMS\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                        as *const u8 as *const libc::c_char,
                    206 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 110],
                        &[libc::c_char; 110],
                    >(
                        b"size_t ZSTD_decodeLiteralsBlock(ZSTD_DCtx *, const void *, size_t, void *, size_t, const streaming_operation)\0",
                    ))
                        .as_ptr(),
                );
            }
            hufSuccess = HUF_decompress4X_usingDTable(
                (*dctx).litBuffer as *mut libc::c_void,
                litSize,
                istart.offset(lhSize as isize) as *const libc::c_void,
                litCSize,
                (*dctx).HUFptr,
                flags,
            );
        }
    } else if singleStream != 0 {
        hufSuccess = HUF_decompress1X1_DCtx_wksp(
            ((*dctx).entropy.hufTable).as_mut_ptr(),
            (*dctx).litBuffer as *mut libc::c_void,
            litSize,
            istart.offset(lhSize as isize) as *const libc::c_void,
            litCSize,
            ((*dctx).workspace).as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[U32; 640]>() as libc::c_ulong,
            flags,
        );
    } else {
        hufSuccess = HUF_decompress4X_hufOnly_wksp(
            ((*dctx).entropy.hufTable).as_mut_ptr(),
            (*dctx).litBuffer as *mut libc::c_void,
            litSize,
            istart.offset(lhSize as isize) as *const libc::c_void,
            litCSize,
            ((*dctx).workspace).as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[U32; 640]>() as libc::c_ulong,
            flags,
        );
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        if litSize
            > (if 64 as libc::c_int
                > (if ((1 as libc::c_int) << 16 as libc::c_int)
                    < (128 as libc::c_int) << 10 as libc::c_int
                {
                    (1 as libc::c_int) << 16 as libc::c_int
                } else {
                    (128 as libc::c_int) << 10 as libc::c_int
                })
            {
                64 as libc::c_int
            } else {
                (if ((1 as libc::c_int) << 16 as libc::c_int)
                    < (128 as libc::c_int) << 10 as libc::c_int
                {
                    (1 as libc::c_int) << 16 as libc::c_int
                } else {
                    (128 as libc::c_int) << 10 as libc::c_int
                })
            }) as libc::c_ulong
        {} else {
            __assert_fail(
                b"litSize > ZSTD_LITBUFFEREXTRASIZE\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"size_t ZSTD_decodeLiteralsBlock(ZSTD_DCtx *, const void *, size_t, void *, size_t, const streaming_operation)\0",
                ))
                    .as_ptr(),
            );
        }
        libc::memcpy(
            ((*dctx).litExtraBuffer).as_mut_ptr() as *mut libc::c_void,
            ((*dctx).litBufferEnd)
                .offset(
                    -((if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    }) as isize),
                ) as *const libc::c_void,
            (if 64 as libc::c_int
                > (if ((1 as libc::c_int) << 16 as libc::c_int)
                    < (128 as libc::c_int) << 10 as libc::c_int
                {
                    (1 as libc::c_int) << 16 as libc::c_int
                } else {
                    (128 as libc::c_int) << 10 as libc::c_int
                })
            {
                64 as libc::c_int
            } else if ((1 as libc::c_int) << 16 as libc::c_int)
                < (128 as libc::c_int) << 10 as libc::c_int
            {
                (1 as libc::c_int) << 16 as libc::c_int
            } else {
                (128 as libc::c_int) << 10 as libc::c_int
            }) as libc::c_ulong as libc::size_t,
        );
        libc::memmove(
            ((*dctx).litBuffer)
                .offset(
                    (if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    }) as isize,
                )
                .offset(-(32 as libc::c_int as isize)) as *mut libc::c_void,
            (*dctx).litBuffer as *const libc::c_void,
            litSize
                .wrapping_sub(
                    (if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    }) as libc::c_ulong,
                ) as libc::size_t,
        );
        (*dctx)
            .litBuffer = ((*dctx).litBuffer)
            .offset(
                ((if 64 as libc::c_int
                    > (if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    })
                {
                    64 as libc::c_int
                } else {
                    (if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    })
                }) - WILDCOPY_OVERLENGTH) as isize,
            );
        (*dctx)
            .litBufferEnd = ((*dctx).litBufferEnd)
            .offset(-(WILDCOPY_OVERLENGTH as isize));
        if (*dctx).litBufferEnd
            <= (dst as *mut BYTE).offset(blockSizeMax as isize) as *const BYTE
        {} else {
            __assert_fail(
                b"dctx->litBufferEnd <= (BYTE*)dst + blockSizeMax\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                238 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"size_t ZSTD_decodeLiteralsBlock(ZSTD_DCtx *, const void *, size_t, void *, size_t, const streaming_operation)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if ERR_isError(hufSuccess) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    (*dctx).litPtr = (*dctx).litBuffer;
    (*dctx).litSize = litSize;
    (*dctx).litEntropy = 1 as libc::c_int as U32;
    if litEncType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint {
        (*dctx).HUFptr = ((*dctx).entropy.hufTable).as_mut_ptr();
    }
    return litCSize.wrapping_add(lhSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeLiteralsBlock_wrapper(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    (*dctx).isFrameDecompression = 0 as libc::c_int;
    return ZSTD_decodeLiteralsBlock(dctx, src, srcSize, dst, dstCapacity, not_streaming);
}
static mut LL_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 1 as libc::c_int as BYTE,
            baseValue: LL_DEFAULTNORMLOG as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 0 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 0 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 1 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 3 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 4 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 6 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 7 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 9 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 10 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 12 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 14 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 16 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 20 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 22 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 28 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 32 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 4 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 48 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 6 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 64 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 7 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 128 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 8 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 256 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 10 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 1024 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 12 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 4096 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 0 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 1 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 2 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 4 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 5 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 7 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 8 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 10 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 11 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 13 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 16 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 18 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 22 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 24 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 32 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 40 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 6 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 64 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 6 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 64 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 7 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 128 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 9 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 512 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 11 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 2048 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 48 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 0 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 1 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 2 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 3 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 5 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 6 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 8 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 9 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 11 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 12 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 15 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 18 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 20 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 24 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 28 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 40 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 4 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 48 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 16 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 65536 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 15 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 32768 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 14 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 16384 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 13 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 8192 as libc::c_int as U32,
        };
        init
    },
];
static mut OF_defaultDTable: [ZSTD_seqSymbol; 33] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 1 as libc::c_int as BYTE,
            baseValue: OF_DEFAULTNORMLOG as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 0 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 6 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 61 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 9 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 509 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 15 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 32765 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 21 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 2097149 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 5 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 7 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 125 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 12 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 4093 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 18 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 262141 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 23 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 8388605 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 5 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 29 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 8 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 253 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 14 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 16381 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 20 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 1048573 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 1 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 7 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 125 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 11 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 2045 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 17 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 131069 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 22 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 4194301 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 4 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 13 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 8 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 253 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 13 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 8189 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 19 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 524285 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 1 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 6 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 61 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 10 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 1021 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 16 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 65533 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 28 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 268435453 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 27 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 134217725 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 26 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 67108861 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 25 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 33554429 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 24 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 16777213 as libc::c_int as U32,
        };
        init
    },
];
static mut ML_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 1 as libc::c_int as BYTE,
            baseValue: ML_DEFAULTNORMLOG as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 3 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 4 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 5 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 6 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 8 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 9 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 11 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 13 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 16 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 19 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 22 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 25 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 28 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 31 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 34 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 37 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 41 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 47 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 59 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 4 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 83 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 7 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 131 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 9 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 515 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 4 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 5 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 6 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 7 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 9 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 10 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 12 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 15 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 18 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 21 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 24 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 27 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 30 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 33 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 35 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 1 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 39 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 2 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 43 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 3 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 51 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 4 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 67 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 5 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 99 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 8 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 259 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 4 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 48 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 4 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 4 as libc::c_int as BYTE,
            baseValue: 5 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 7 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 8 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 10 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 5 as libc::c_int as BYTE,
            baseValue: 11 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 14 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 17 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 20 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 23 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 26 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 29 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 0 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 32 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 16 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 65539 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 15 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 32771 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 14 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 16387 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 13 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 8195 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 12 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 4099 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 11 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 2051 as libc::c_int as U32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as U16,
            nbAdditionalBits: 10 as libc::c_int as BYTE,
            nbBits: 6 as libc::c_int as BYTE,
            baseValue: 1027 as libc::c_int as U32,
        };
        init
    },
];
unsafe extern "C" fn ZSTD_buildSeqTable_rle(
    mut dt: *mut ZSTD_seqSymbol,
    mut baseValue: U32,
    mut nbAddBits: U8,
) {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut ZSTD_seqSymbol_header;
    let cell = dt.offset(1 as libc::c_int as isize);
    (*DTableH).tableLog = 0 as libc::c_int as U32;
    (*DTableH).fastMode = 0 as libc::c_int as U32;
    (*cell).nbBits = 0 as libc::c_int as BYTE;
    (*cell).nextState = 0 as libc::c_int as U16;
    if (nbAddBits as libc::c_int) < 255 as libc::c_int {} else {
        __assert_fail(
            b"nbAddBits < 255\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            474 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void ZSTD_buildSeqTable_rle(ZSTD_seqSymbol *, U32, U8)\0"))
                .as_ptr(),
        );
    }
    (*cell).nbAdditionalBits = nbAddBits;
    (*cell).baseValue = baseValue;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_buildFSETable_body(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut baseValue: *const U32,
    mut nbAdditionalBits: *const U8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: size_t,
) {
    let tableDecode = dt.offset(1 as libc::c_int as isize);
    let maxSV1 = maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint);
    let tableSize = ((1 as libc::c_int) << tableLog) as U32;
    let mut symbolNext = wksp as *mut U16;
    let mut spread = symbolNext
        .offset(
            (if 35 as libc::c_int > 52 as libc::c_int {
                35 as libc::c_int
            } else {
                52 as libc::c_int
            }) as isize,
        )
        .offset(1 as libc::c_int as isize) as *mut BYTE;
    let mut highThreshold = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if maxSymbolValue
        <= (if 35 as libc::c_int > 52 as libc::c_int {
            35 as libc::c_int
        } else {
            52 as libc::c_int
        }) as libc::c_uint
    {} else {
        __assert_fail(
            b"maxSymbolValue <= MaxSeq\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            500 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 131],
                &[libc::c_char; 131],
            >(
                b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if tableLog
        <= (if (if 9 as libc::c_int > 9 as libc::c_int {
            9 as libc::c_int
        } else {
            9 as libc::c_int
        }) > 8 as libc::c_int
        {
            (if 9 as libc::c_int > 9 as libc::c_int {
                9 as libc::c_int
            } else {
                9 as libc::c_int
            })
        } else {
            8 as libc::c_int
        }) as libc::c_uint
    {} else {
        __assert_fail(
            b"tableLog <= MaxFSELog\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            501 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 131],
                &[libc::c_char; 131],
            >(
                b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if wkspSize
        >= (::core::mem::size_of::<S16>() as libc::c_ulong)
            .wrapping_mul(
                ((if 35 as libc::c_int > 52 as libc::c_int {
                    35 as libc::c_int
                } else {
                    52 as libc::c_int
                }) + 1 as libc::c_int) as libc::c_ulong,
            )
            .wrapping_add(
                ((1 as libc::c_uint)
                    << (if (if 9 as libc::c_int > 9 as libc::c_int {
                        9 as libc::c_int
                    } else {
                        9 as libc::c_int
                    }) > 8 as libc::c_int
                    {
                        (if 9 as libc::c_int > 9 as libc::c_int {
                            9 as libc::c_int
                        } else {
                            9 as libc::c_int
                        })
                    } else {
                        8 as libc::c_int
                    })) as libc::c_ulong,
            )
            .wrapping_add(::core::mem::size_of::<U64>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"wkspSize >= ZSTD_BUILD_FSE_TABLE_WKSP_SIZE\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            502 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 131],
                &[libc::c_char; 131],
            >(
                b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let mut DTableH = ZSTD_seqSymbol_header {
        fastMode: 0,
        tableLog: 0,
    };
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1 as libc::c_int as U32;
    let largeLimit = ((1 as libc::c_int)
        << tableLog.wrapping_sub(1 as libc::c_int as libc::c_uint)) as S16;
    let mut s: U32 = 0;
    s = 0 as libc::c_int as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -(1 as libc::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).baseValue = s;
            *symbolNext.offset(s as isize) = 1 as libc::c_int as U16;
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int
                >= largeLimit as libc::c_int
            {
                DTableH.fastMode = 0 as libc::c_int as U32;
            }
            if *normalizedCounter.offset(s as isize) as libc::c_int >= 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"normalizedCounter[s]>=0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                        as *const u8 as *const libc::c_char,
                    516 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            *symbolNext
                .offset(s as isize) = *normalizedCounter.offset(s as isize) as U16;
        }
        s = s.wrapping_add(1);
    }
    libc::memcpy(
        dt as *mut libc::c_void,
        &mut DTableH as *mut ZSTD_seqSymbol_header as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_seqSymbol_header>() as libc::c_ulong as libc::size_t,
    );
    if tableSize <= 512 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tableSize <= 512\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            523 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 131],
                &[libc::c_char; 131],
            >(
                b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
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
            if n >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"n>=0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                        as *const u8 as *const libc::c_char,
                    550 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            pos = (pos as libc::c_ulong).wrapping_add(n as size_t) as size_t as size_t;
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
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                564 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 131],
                    &[libc::c_char; 131],
                >(
                    b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
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
                    .baseValue = *spread.offset(s_1.wrapping_add(u) as isize) as U32;
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll.wrapping_mul(step)) & tableMask;
            s_1 = (s_1 as libc::c_ulong).wrapping_add(unroll) as size_t as size_t;
        }
        if position == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"position == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                573 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 131],
                    &[libc::c_char; 131],
                >(
                    b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
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
            let n_0 = *normalizedCounter.offset(s_2 as isize) as libc::c_int;
            i_0 = 0 as libc::c_int;
            while i_0 < n_0 {
                (*tableDecode.offset(position_0 as isize)).baseValue = s_2;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while (position_0 > highThreshold) as libc::c_int as libc::c_long != 0 {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
            }
            s_2 = s_2.wrapping_add(1);
        }
        if position_0 == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"position == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                587 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 131],
                    &[libc::c_char; 131],
                >(
                    b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    let mut u_0: U32 = 0;
    u_0 = 0 as libc::c_int as U32;
    while u_0 < tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).baseValue;
        let ref mut fresh1 = *symbolNext.offset(symbol as isize);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as U32;
        (*tableDecode.offset(u_0 as isize))
            .nbBits = tableLog.wrapping_sub(ZSTD_highbit32(nextState)) as BYTE;
        (*tableDecode.offset(u_0 as isize))
            .nextState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as libc::c_int)
            .wrapping_sub(tableSize) as U16;
        if (*nbAdditionalBits.offset(symbol as isize) as libc::c_int)
            < 255 as libc::c_int
        {} else {
            __assert_fail(
                b"nbAdditionalBits[symbol] < 255\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                598 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 131],
                    &[libc::c_char; 131],
                >(
                    b"void ZSTD_buildFSETable_body(ZSTD_seqSymbol *, const short *, unsigned int, const U32 *, const U8 *, unsigned int, void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        (*tableDecode.offset(u_0 as isize))
            .nbAdditionalBits = *nbAdditionalBits.offset(symbol as isize);
        (*tableDecode.offset(u_0 as isize))
            .baseValue = *baseValue.offset(symbol as isize);
        u_0 = u_0.wrapping_add(1);
    }
}
unsafe extern "C" fn ZSTD_buildFSETable_body_default(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut baseValue: *const U32,
    mut nbAdditionalBits: *const U8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: size_t,
) {
    ZSTD_buildFSETable_body(
        dt,
        normalizedCounter,
        maxSymbolValue,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
        wkspSize,
    );
}
unsafe extern "C" fn ZSTD_buildFSETable_body_bmi2(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut baseValue: *const U32,
    mut nbAdditionalBits: *const U8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: size_t,
) {
    ZSTD_buildFSETable_body(
        dt,
        normalizedCounter,
        maxSymbolValue,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
        wkspSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_buildFSETable(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut baseValue: *const U32,
    mut nbAdditionalBits: *const U8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: size_t,
    mut bmi2: libc::c_int,
) {
    if bmi2 != 0 {
        ZSTD_buildFSETable_body_bmi2(
            dt,
            normalizedCounter,
            maxSymbolValue,
            baseValue,
            nbAdditionalBits,
            tableLog,
            wksp,
            wkspSize,
        );
        return;
    }
    ZSTD_buildFSETable_body_default(
        dt,
        normalizedCounter,
        maxSymbolValue,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
        wkspSize,
    );
}
unsafe extern "C" fn ZSTD_buildSeqTable(
    mut DTableSpace: *mut ZSTD_seqSymbol,
    mut DTablePtr: *mut *const ZSTD_seqSymbol,
    mut type_0: symbolEncodingType_e,
    mut max: libc::c_uint,
    mut maxLog: U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut baseValue: *const U32,
    mut nbAdditionalBits: *const U8,
    mut defaultTable: *const ZSTD_seqSymbol,
    mut flagRepeatTable: U32,
    mut ddictIsCold: libc::c_int,
    mut nbSeq: libc::c_int,
    mut wksp: *mut U32,
    mut wkspSize: size_t,
    mut bmi2: libc::c_int,
) -> size_t {
    match type_0 as libc::c_uint {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            if *(src as *const BYTE) as libc::c_uint > max {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            let symbol = *(src as *const BYTE) as U32;
            let baseline = *baseValue.offset(symbol as isize);
            let nbBits = *nbAdditionalBits.offset(symbol as isize);
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);
            *DTablePtr = DTableSpace;
            return 1 as libc::c_int as size_t;
        }
        0 => {
            *DTablePtr = defaultTable;
            return 0 as libc::c_int as size_t;
        }
        3 => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            if ddictIsCold != 0 && nbSeq > 24 as libc::c_int {
                let pStart = *DTablePtr as *const libc::c_void;
                let pSize = (::core::mem::size_of::<ZSTD_seqSymbol>() as libc::c_ulong)
                    .wrapping_mul(
                        (1 as libc::c_int + ((1 as libc::c_int) << maxLog))
                            as libc::c_ulong,
                    );
                let _ptr = pStart as *const libc::c_char;
                let _size = pSize;
                let mut _pos: size_t = 0;
                _pos = 0 as libc::c_int as size_t;
                while _pos < _size {
                    _pos = (_pos as libc::c_ulong)
                        .wrapping_add(CACHELINE_SIZE as libc::c_ulong) as size_t
                        as size_t;
                }
            }
            return 0 as libc::c_int as size_t;
        }
        2 => {
            let mut tableLog: libc::c_uint = 0;
            let mut norm: [S16; 53] = [0; 53];
            let headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut tableLog,
                src,
                srcSize,
            );
            if ERR_isError(headerSize) != 0 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ZSTD_buildFSETable(
                DTableSpace,
                norm.as_mut_ptr(),
                max,
                baseValue,
                nbAdditionalBits,
                tableLog,
                wksp as *mut libc::c_void,
                wkspSize,
                bmi2,
            );
            *DTablePtr = DTableSpace;
            return headerSize;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                690 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 216],
                    &[libc::c_char; 216],
                >(
                    b"size_t ZSTD_buildSeqTable(ZSTD_seqSymbol *, const ZSTD_seqSymbol **, symbolEncodingType_e, unsigned int, U32, const void *, size_t, const U32 *, const U8 *, const ZSTD_seqSymbol *, U32, int, int, U32 *, size_t, int)\0",
                ))
                    .as_ptr(),
            );
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeSeqHeaders(
    mut dctx: *mut ZSTD_DCtx,
    mut nbSeqPtr: *mut libc::c_int,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const BYTE;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    let mut nbSeq: libc::c_int = 0;
    if srcSize < 1 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let fresh3 = ip;
    ip = ip.offset(1);
    nbSeq = *fresh3 as libc::c_int;
    if nbSeq == 0 {
        *nbSeqPtr = 0 as libc::c_int;
        if srcSize != 1 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
        return 1 as libc::c_int as size_t;
    }
    if nbSeq > 0x7f as libc::c_int {
        if nbSeq == 0xff as libc::c_int {
            if ip.offset(2 as libc::c_int as isize) > iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            nbSeq = MEM_readLE16(ip as *const libc::c_void) as libc::c_int + LONGNBSEQ;
            ip = ip.offset(2 as libc::c_int as isize);
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            let fresh4 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as libc::c_int) << 8 as libc::c_int)
                + *fresh4 as libc::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(1 as libc::c_int as isize) > iend {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let LLtype = (*ip as libc::c_int >> 6 as libc::c_int) as symbolEncodingType_e;
    let OFtype = (*ip as libc::c_int >> 4 as libc::c_int & 3 as libc::c_int)
        as symbolEncodingType_e;
    let MLtype = (*ip as libc::c_int >> 2 as libc::c_int & 3 as libc::c_int)
        as symbolEncodingType_e;
    ip = ip.offset(1);
    let llhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.LLTable).as_mut_ptr(),
        &mut (*dctx).LLTptr,
        LLtype,
        MaxLL as libc::c_uint,
        LLFSELog as U32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
        LL_base.as_ptr(),
        LL_bits.as_ptr(),
        LL_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[U32; 640]>() as libc::c_ulong,
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(llhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    ip = ip.offset(llhSize as isize);
    let ofhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.OFTable).as_mut_ptr(),
        &mut (*dctx).OFTptr,
        OFtype,
        MaxOff as libc::c_uint,
        OffFSELog as U32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
        OF_base.as_ptr(),
        OF_bits.as_ptr(),
        OF_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[U32; 640]>() as libc::c_ulong,
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(ofhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    ip = ip.offset(ofhSize as isize);
    let mlhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.MLTable).as_mut_ptr(),
        &mut (*dctx).MLTptr,
        MLtype,
        MaxML as libc::c_uint,
        MLFSELog as U32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
        ML_base.as_ptr(),
        ML_bits.as_ptr(),
        ML_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[U32; 640]>() as libc::c_ulong,
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(mlhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    ip = ip.offset(mlhSize as isize);
    return ip.offset_from(istart) as libc::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_overlapCopy8(
    mut op: *mut *mut BYTE,
    mut ip: *mut *const BYTE,
    mut offset: size_t,
) {
    if *ip <= *op as *const BYTE {} else {
        __assert_fail(
            b"*ip <= *op\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            802 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void ZSTD_overlapCopy8(BYTE **, const BYTE **, size_t)\0"))
                .as_ptr(),
        );
    }
    if offset < 8 as libc::c_int as libc::c_ulong {
        static mut dec32table: [U32; 8] = [
            0 as libc::c_int as U32,
            1 as libc::c_int as U32,
            2 as libc::c_int as U32,
            1 as libc::c_int as U32,
            4 as libc::c_int as U32,
            4 as libc::c_int as U32,
            4 as libc::c_int as U32,
            4 as libc::c_int as U32,
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
        let sub2 = dec64table[offset as usize];
        *(*op)
            .offset(
                0 as libc::c_int as isize,
            ) = *(*ip).offset(0 as libc::c_int as isize);
        *(*op)
            .offset(
                1 as libc::c_int as isize,
            ) = *(*ip).offset(1 as libc::c_int as isize);
        *(*op)
            .offset(
                2 as libc::c_int as isize,
            ) = *(*ip).offset(2 as libc::c_int as isize);
        *(*op)
            .offset(
                3 as libc::c_int as isize,
            ) = *(*ip).offset(3 as libc::c_int as isize);
        *ip = (*ip).offset(dec32table[offset as usize] as isize);
        ZSTD_copy4(
            (*op).offset(4 as libc::c_int as isize) as *mut libc::c_void,
            *ip as *const libc::c_void,
        );
        *ip = (*ip).offset(-(sub2 as isize));
    } else {
        ZSTD_copy8(*op as *mut libc::c_void, *ip as *const libc::c_void);
    }
    *ip = (*ip).offset(8 as libc::c_int as isize);
    *op = (*op).offset(8 as libc::c_int as isize);
    if (*op).offset_from(*ip) as libc::c_long >= 8 as libc::c_int as libc::c_long
    {} else {
        __assert_fail(
            b"*op - *ip >= 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            820 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void ZSTD_overlapCopy8(BYTE **, const BYTE **, size_t)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn ZSTD_safecopy(
    mut op: *mut BYTE,
    oend_w: *const BYTE,
    mut ip: *const BYTE,
    mut length: ptrdiff_t,
    mut ovtype: ZSTD_overlap_e,
) {
    let diff = op.offset_from(ip) as libc::c_long;
    let oend = op.offset(length as isize);
    if ovtype as libc::c_uint == ZSTD_no_overlap as libc::c_int as libc::c_uint
        && (diff <= -(8 as libc::c_int) as libc::c_long
            || diff >= 8 as libc::c_int as libc::c_long || op >= oend_w as *mut BYTE)
        || ovtype as libc::c_uint
            == ZSTD_overlap_src_before_dst as libc::c_int as libc::c_uint
            && diff >= 0 as libc::c_int as libc::c_long
    {} else {
        __assert_fail(
            b"(ovtype == ZSTD_no_overlap && (diff <= -8 || diff >= 8 || op >= oend_w)) || (ovtype == ZSTD_overlap_src_before_dst && diff >= 0)\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            839 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"void ZSTD_safecopy(BYTE *, const BYTE *const, const BYTE *, ptrdiff_t, ZSTD_overlap_e)\0",
            ))
                .as_ptr(),
        );
    }
    if length < 8 as libc::c_int as libc::c_long {
        while op < oend {
            let fresh5 = ip;
            ip = ip.offset(1);
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = *fresh5;
        }
        return;
    }
    if ovtype as libc::c_uint
        == ZSTD_overlap_src_before_dst as libc::c_int as libc::c_uint
    {
        if length >= 8 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"length >= 8\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                848 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[libc::c_char; 87],
                >(
                    b"void ZSTD_safecopy(BYTE *, const BYTE *const, const BYTE *, ptrdiff_t, ZSTD_overlap_e)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_overlapCopy8(&mut op, &mut ip, diff as size_t);
        length -= 8 as libc::c_int as libc::c_long;
        if op.offset_from(ip) as libc::c_long >= 8 as libc::c_int as libc::c_long
        {} else {
            __assert_fail(
                b"op - ip >= 8\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                851 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[libc::c_char; 87],
                >(
                    b"void ZSTD_safecopy(BYTE *, const BYTE *const, const BYTE *, ptrdiff_t, ZSTD_overlap_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if op <= oend {} else {
            __assert_fail(
                b"op <= oend\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                852 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[libc::c_char; 87],
                >(
                    b"void ZSTD_safecopy(BYTE *, const BYTE *const, const BYTE *, ptrdiff_t, ZSTD_overlap_e)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if oend <= oend_w as *mut BYTE {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            ip as *const libc::c_void,
            length,
            ovtype,
        );
        return;
    }
    if op <= oend_w as *mut BYTE {
        if oend > oend_w as *mut BYTE {} else {
            __assert_fail(
                b"oend > oend_w\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                862 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[libc::c_char; 87],
                >(
                    b"void ZSTD_safecopy(BYTE *, const BYTE *const, const BYTE *, ptrdiff_t, ZSTD_overlap_e)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            ip as *const libc::c_void,
            oend_w.offset_from(op) as libc::c_long,
            ovtype,
        );
        ip = ip.offset(oend_w.offset_from(op) as libc::c_long as isize);
        op = op.offset(oend_w.offset_from(op) as libc::c_long as isize);
    }
    while op < oend {
        let fresh7 = ip;
        ip = ip.offset(1);
        let fresh8 = op;
        op = op.offset(1);
        *fresh8 = *fresh7;
    }
}
unsafe extern "C" fn ZSTD_safecopyDstBeforeSrc(
    mut op: *mut BYTE,
    mut ip: *const BYTE,
    mut length: ptrdiff_t,
) {
    let diff = op.offset_from(ip) as libc::c_long;
    let oend = op.offset(length as isize);
    if length < 8 as libc::c_int as libc::c_long
        || diff > -(8 as libc::c_int) as libc::c_long
    {
        while op < oend {
            let fresh9 = ip;
            ip = ip.offset(1);
            let fresh10 = op;
            op = op.offset(1);
            *fresh10 = *fresh9;
        }
        return;
    }
    if op <= oend.offset(-(WILDCOPY_OVERLENGTH as isize))
        && diff < -WILDCOPY_VECLEN as libc::c_long
    {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            ip as *const libc::c_void,
            oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op) as libc::c_long,
            ZSTD_no_overlap,
        );
        ip = ip
            .offset(
                oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op)
                    as libc::c_long as isize,
            );
        op = op
            .offset(
                oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op)
                    as libc::c_long as isize,
            );
    }
    while op < oend {
        let fresh11 = ip;
        ip = ip.offset(1);
        let fresh12 = op;
        op = op.offset(1);
        *fresh12 = *fresh11;
    }
}
#[inline(never)]
unsafe extern "C" fn ZSTD_execSequenceEnd(
    mut op: *mut BYTE,
    oend: *mut BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    prefixStart: *const BYTE,
    virtualStart: *const BYTE,
    dictEnd: *const BYTE,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = oLitEnd.offset(-(sequence.offset as isize));
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    if sequenceLength > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op < op.offset(sequenceLength as isize) {} else {
        __assert_fail(
            b"op < op + sequenceLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            917 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 147],
                &[libc::c_char; 147],
            >(
                b"size_t ZSTD_execSequenceEnd(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oLitEnd < op.offset(sequenceLength as isize) {} else {
        __assert_fail(
            b"oLitEnd < op + sequenceLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            918 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 147],
                &[libc::c_char; 147],
            >(
                b"size_t ZSTD_execSequenceEnd(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_safecopy(op, oend_w, *litPtr, sequence.litLength as ptrdiff_t, ZSTD_no_overlap);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as libc::c_long as size_t
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        match_0 = dictEnd
            .offset(-(prefixStart.offset_from(match_0) as libc::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut libc::c_void,
                match_0 as *const libc::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            as size_t as size_t;
        match_0 = prefixStart;
    }
    ZSTD_safecopy(
        op,
        oend_w,
        match_0,
        sequence.matchLength as ptrdiff_t,
        ZSTD_overlap_src_before_dst,
    );
    return sequenceLength;
}
#[inline(never)]
unsafe extern "C" fn ZSTD_execSequenceEndSplitLitBuffer(
    mut op: *mut BYTE,
    oend: *mut BYTE,
    oend_w: *const BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    prefixStart: *const BYTE,
    virtualStart: *const BYTE,
    dictEnd: *const BYTE,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = oLitEnd.offset(-(sequence.offset as isize));
    if sequenceLength > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if op < op.offset(sequenceLength as isize) {} else {
        __assert_fail(
            b"op < op + sequenceLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            964 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 180],
                &[libc::c_char; 180],
            >(
                b"size_t ZSTD_execSequenceEndSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oLitEnd < op.offset(sequenceLength as isize) {} else {
        __assert_fail(
            b"oLitEnd < op + sequenceLength\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            965 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 180],
                &[libc::c_char; 180],
            >(
                b"size_t ZSTD_execSequenceEndSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if op > *litPtr as *mut BYTE
        && op < (*litPtr).offset(sequence.litLength as isize) as *mut BYTE
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    ZSTD_safecopyDstBeforeSrc(op, *litPtr, sequence.litLength as ptrdiff_t);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as libc::c_long as size_t
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        match_0 = dictEnd
            .offset(-(prefixStart.offset_from(match_0) as libc::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut libc::c_void,
                match_0 as *const libc::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            as size_t as size_t;
        match_0 = prefixStart;
    }
    ZSTD_safecopy(
        op,
        oend_w,
        match_0,
        sequence.matchLength as ptrdiff_t,
        ZSTD_overlap_src_before_dst,
    );
    return sequenceLength;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_execSequence(
    mut op: *mut BYTE,
    oend: *mut BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    prefixStart: *const BYTE,
    virtualStart: *const BYTE,
    dictEnd: *const BYTE,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = oLitEnd.offset(-(sequence.offset as isize));
    if !op.is_null() {} else {
        __assert_fail(
            b"op != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1007 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oend_w < oend {} else {
        __assert_fail(
            b"oend_w < oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1008 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if (iLitEnd > litLimit || oMatchEnd > oend_w
        || MEM_32bits() != 0
            && (oend.offset_from(op) as libc::c_long as size_t)
                < sequenceLength.wrapping_add(32 as libc::c_int as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {
        return ZSTD_execSequenceEnd(
            op,
            oend,
            sequence,
            litPtr,
            litLimit,
            prefixStart,
            virtualStart,
            dictEnd,
        );
    }
    if op <= oLitEnd {} else {
        __assert_fail(
            b"op <= oLitEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1026 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oLitEnd < oMatchEnd {} else {
        __assert_fail(
            b"oLitEnd < oMatchEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1027 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oMatchEnd <= oend {} else {
        __assert_fail(
            b"oMatchEnd <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1028 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if iLitEnd <= litLimit {} else {
        __assert_fail(
            b"iLitEnd <= litLimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1029 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oLitEnd <= oend_w {} else {
        __assert_fail(
            b"oLitEnd <= oend_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1030 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oMatchEnd <= oend_w {} else {
        __assert_fail(
            b"oMatchEnd <= oend_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1031 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if 32 as libc::c_int >= 16 as libc::c_int {} else {
        __assert_fail(
            b"WILDCOPY_OVERLENGTH >= 16\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1037 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_copy16(op as *mut libc::c_void, *litPtr as *const libc::c_void);
    if (sequence.litLength > 16 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        ZSTD_wildcopy(
            op.offset(16 as libc::c_int as isize) as *mut libc::c_void,
            (*litPtr).offset(16 as libc::c_int as isize) as *const libc::c_void,
            (sequence.litLength).wrapping_sub(16 as libc::c_int as libc::c_ulong)
                as ptrdiff_t,
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as size_t {
        if (sequence.offset
            > oLitEnd.offset_from(virtualStart) as libc::c_long as size_t) as libc::c_int
            as libc::c_long != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        match_0 = dictEnd
            .offset(match_0.offset_from(prefixStart) as libc::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut libc::c_void,
                match_0 as *const libc::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            as size_t as size_t;
        match_0 = prefixStart;
    }
    if op <= oMatchEnd {} else {
        __assert_fail(
            b"op <= oMatchEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1063 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oMatchEnd <= oend_w {} else {
        __assert_fail(
            b"oMatchEnd <= oend_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1064 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if match_0 >= prefixStart {} else {
        __assert_fail(
            b"match >= prefixStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1065 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if sequence.matchLength >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sequence.matchLength >= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1066 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if (sequence.offset >= 16 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    if sequence.offset < 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sequence.offset < WILDCOPY_VECLEN\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1079 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 144],
                &[libc::c_char; 144],
            >(
                b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 as libc::c_int as libc::c_ulong {
        if op < oMatchEnd {} else {
            __assert_fail(
                b"op < oMatchEnd\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                1086 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 144],
                    &[libc::c_char; 144],
                >(
                    b"size_t ZSTD_execSequence(BYTE *, BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t - 8 as libc::c_int as libc::c_long,
            ZSTD_overlap_src_before_dst,
        );
    }
    return sequenceLength;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_execSequenceSplitLitBuffer(
    mut op: *mut BYTE,
    oend: *mut BYTE,
    oend_w: *const BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    prefixStart: *const BYTE,
    virtualStart: *const BYTE,
    dictEnd: *const BYTE,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = oLitEnd.offset(-(sequence.offset as isize));
    if !op.is_null() {} else {
        __assert_fail(
            b"op != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1104 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oend_w < oend as *const BYTE {} else {
        __assert_fail(
            b"oend_w < oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1105 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if (iLitEnd > litLimit || oMatchEnd > oend_w as *mut BYTE
        || MEM_32bits() != 0
            && (oend.offset_from(op) as libc::c_long as size_t)
                < sequenceLength.wrapping_add(32 as libc::c_int as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {
        return ZSTD_execSequenceEndSplitLitBuffer(
            op,
            oend,
            oend_w,
            sequence,
            litPtr,
            litLimit,
            prefixStart,
            virtualStart,
            dictEnd,
        );
    }
    if op <= oLitEnd {} else {
        __assert_fail(
            b"op <= oLitEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1118 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oLitEnd < oMatchEnd {} else {
        __assert_fail(
            b"oLitEnd < oMatchEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1119 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oMatchEnd <= oend {} else {
        __assert_fail(
            b"oMatchEnd <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1120 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if iLitEnd <= litLimit {} else {
        __assert_fail(
            b"iLitEnd <= litLimit\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1121 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oLitEnd <= oend_w as *mut BYTE {} else {
        __assert_fail(
            b"oLitEnd <= oend_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1122 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oMatchEnd <= oend_w as *mut BYTE {} else {
        __assert_fail(
            b"oMatchEnd <= oend_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1123 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if 32 as libc::c_int >= 16 as libc::c_int {} else {
        __assert_fail(
            b"WILDCOPY_OVERLENGTH >= 16\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1129 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_copy16(op as *mut libc::c_void, *litPtr as *const libc::c_void);
    if (sequence.litLength > 16 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        ZSTD_wildcopy(
            op.offset(16 as libc::c_int as isize) as *mut libc::c_void,
            (*litPtr).offset(16 as libc::c_int as isize) as *const libc::c_void,
            (sequence.litLength).wrapping_sub(16 as libc::c_int as libc::c_ulong)
                as ptrdiff_t,
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as size_t {
        if (sequence.offset
            > oLitEnd.offset_from(virtualStart) as libc::c_long as size_t) as libc::c_int
            as libc::c_long != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        match_0 = dictEnd
            .offset(match_0.offset_from(prefixStart) as libc::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut libc::c_void,
                match_0 as *const libc::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            as size_t as size_t;
        match_0 = prefixStart;
    }
    if op <= oMatchEnd {} else {
        __assert_fail(
            b"op <= oMatchEnd\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if oMatchEnd <= oend_w as *mut BYTE {} else {
        __assert_fail(
            b"oMatchEnd <= oend_w\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1155 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if match_0 >= prefixStart {} else {
        __assert_fail(
            b"match >= prefixStart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1156 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if sequence.matchLength >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sequence.matchLength >= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1157 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    if (sequence.offset >= 16 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    if sequence.offset < 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sequence.offset < WILDCOPY_VECLEN\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1170 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
            ))
                .as_ptr(),
        );
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 as libc::c_int as libc::c_ulong {
        if op < oMatchEnd {} else {
            __assert_fail(
                b"op < oMatchEnd\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                1177 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 177],
                    &[libc::c_char; 177],
                >(
                    b"size_t ZSTD_execSequenceSplitLitBuffer(BYTE *, BYTE *const, const BYTE *const, seq_t, const BYTE **, const BYTE *const, const BYTE *const, const BYTE *const, const BYTE *const)\0",
                ))
                    .as_ptr(),
            );
        }
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t - 8 as libc::c_int as libc::c_long,
            ZSTD_overlap_src_before_dst,
        );
    }
    return sequenceLength;
}
unsafe extern "C" fn ZSTD_initFseState(
    mut DStatePtr: *mut ZSTD_fseState,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const ZSTD_seqSymbol,
) {
    let mut ptr = dt as *const libc::c_void;
    let DTableH = ptr as *const ZSTD_seqSymbol_header;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as libc::c_int as isize);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_updateFseStateWithDInfo(
    mut DStatePtr: *mut ZSTD_fseState,
    mut bitD: *mut BIT_DStream_t,
    mut nextState: U16,
    mut nbBits: U32,
) {
    let lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (nextState as libc::c_ulong).wrapping_add(lowBits);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decodeSequence(
    mut seqState: *mut seqState_t,
    longOffsets: ZSTD_longOffset_e,
) -> seq_t {
    let mut seq = seq_t {
        litLength: 0,
        matchLength: 0,
        offset: 0,
    };
    let llDInfo = ((*seqState).stateLL.table).offset((*seqState).stateLL.state as isize);
    let mlDInfo = ((*seqState).stateML.table).offset((*seqState).stateML.state as isize);
    let ofDInfo = ((*seqState).stateOffb.table)
        .offset((*seqState).stateOffb.state as isize);
    seq.matchLength = (*mlDInfo).baseValue as size_t;
    seq.litLength = (*llDInfo).baseValue as size_t;
    let ofBase = (*ofDInfo).baseValue;
    let llBits = (*llDInfo).nbAdditionalBits;
    let mlBits = (*mlDInfo).nbAdditionalBits;
    let ofBits = (*ofDInfo).nbAdditionalBits;
    let totalBits = (llBits as libc::c_int + mlBits as libc::c_int
        + ofBits as libc::c_int) as BYTE;
    let llNext = (*llDInfo).nextState;
    let mlNext = (*mlDInfo).nextState;
    let ofNext = (*ofDInfo).nextState;
    let llnbBits = (*llDInfo).nbBits as U32;
    let mlnbBits = (*mlDInfo).nbBits as U32;
    let ofnbBits = (*ofDInfo).nbBits as U32;
    if llBits as libc::c_int <= 16 as libc::c_int {} else {
        __assert_fail(
            b"llBits <= MaxLLBits\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1255 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"seq_t ZSTD_decodeSequence(seqState_t *, const ZSTD_longOffset_e)\0"))
                .as_ptr(),
        );
    }
    if mlBits as libc::c_int <= 16 as libc::c_int {} else {
        __assert_fail(
            b"mlBits <= MaxMLBits\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1256 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"seq_t ZSTD_decodeSequence(seqState_t *, const ZSTD_longOffset_e)\0"))
                .as_ptr(),
        );
    }
    if ofBits as libc::c_int <= 31 as libc::c_int {} else {
        __assert_fail(
            b"ofBits <= MaxOff\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                as *const u8 as *const libc::c_char,
            1257 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"seq_t ZSTD_decodeSequence(seqState_t *, const ZSTD_longOffset_e)\0"))
                .as_ptr(),
        );
    }
    let mut offset: size_t = 0;
    if ofBits as libc::c_int > 1 as libc::c_int {
        if MEM_32bits() != 0 && longOffsets as libc::c_uint != 0
            && ofBits as libc::c_int >= STREAM_ACCUMULATOR_MIN_32
        {
            let extraBits = (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
            } else {
                0 as libc::c_int
            }) as U32;
            offset = (ofBase as libc::c_ulong)
                .wrapping_add(
                    BIT_readBitsFast(
                        &mut (*seqState).DStream,
                        (ofBits as libc::c_uint).wrapping_sub(extraBits),
                    ) << extraBits,
                );
            BIT_reloadDStream(&mut (*seqState).DStream);
            offset = (offset as libc::c_ulong)
                .wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream, extraBits))
                as size_t as size_t;
        } else {
            offset = (ofBase as libc::c_ulong)
                .wrapping_add(
                    BIT_readBitsFast(&mut (*seqState).DStream, ofBits as libc::c_uint),
                );
            if MEM_32bits() != 0 {
                BIT_reloadDStream(&mut (*seqState).DStream);
            }
        }
        (*seqState)
            .prevOffset[2 as libc::c_int
            as usize] = (*seqState).prevOffset[1 as libc::c_int as usize];
        (*seqState)
            .prevOffset[1 as libc::c_int
            as usize] = (*seqState).prevOffset[0 as libc::c_int as usize];
        (*seqState).prevOffset[0 as libc::c_int as usize] = offset;
    } else {
        let ll0 = ((*llDInfo).baseValue == 0 as libc::c_int as libc::c_uint)
            as libc::c_int as U32;
        if (ofBits as libc::c_int == 0 as libc::c_int) as libc::c_int as libc::c_long
            != 0
        {
            offset = (*seqState).prevOffset[ll0 as usize];
            (*seqState)
                .prevOffset[1 as libc::c_int
                as usize] = (*seqState).prevOffset[(ll0 == 0) as libc::c_int as usize];
            (*seqState).prevOffset[0 as libc::c_int as usize] = offset;
        } else {
            offset = (ofBase.wrapping_add(ll0) as libc::c_ulong)
                .wrapping_add(
                    BIT_readBitsFast(
                        &mut (*seqState).DStream,
                        1 as libc::c_int as libc::c_uint,
                    ),
                );
            let mut temp = if offset == 3 as libc::c_int as libc::c_ulong {
                ((*seqState).prevOffset[0 as libc::c_int as usize])
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                (*seqState).prevOffset[offset as usize]
            };
            temp = (temp as libc::c_ulong)
                .wrapping_add((temp == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
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
        }
    }
    seq.offset = offset;
    if mlBits as libc::c_int > 0 as libc::c_int {
        seq
            .matchLength = (seq.matchLength as libc::c_ulong)
            .wrapping_add(
                BIT_readBitsFast(&mut (*seqState).DStream, mlBits as libc::c_uint),
            ) as size_t as size_t;
    }
    if MEM_32bits() != 0
        && mlBits as libc::c_int + llBits as libc::c_int
            >= STREAM_ACCUMULATOR_MIN_32
                - (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                    ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
                } else {
                    0 as libc::c_int
                })
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if MEM_64bits() != 0
        && (totalBits as libc::c_int
            >= 57 as libc::c_int
                - (9 as libc::c_int + 9 as libc::c_int + 8 as libc::c_int))
            as libc::c_int as libc::c_long != 0
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if llBits as libc::c_int > 0 as libc::c_int {
        seq
            .litLength = (seq.litLength as libc::c_ulong)
            .wrapping_add(
                BIT_readBitsFast(&mut (*seqState).DStream, llBits as libc::c_uint),
            ) as size_t as size_t;
    }
    if MEM_32bits() != 0 {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    ZSTD_updateFseStateWithDInfo(
        &mut (*seqState).stateLL,
        &mut (*seqState).DStream,
        llNext,
        llnbBits,
    );
    ZSTD_updateFseStateWithDInfo(
        &mut (*seqState).stateML,
        &mut (*seqState).DStream,
        mlNext,
        mlnbBits,
    );
    if MEM_32bits() != 0 {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    ZSTD_updateFseStateWithDInfo(
        &mut (*seqState).stateOffb,
        &mut (*seqState).DStream,
        ofNext,
        ofnbBits,
    );
    return seq;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_bodySplitLitBuffer(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    let mut ip = seqStart as *const BYTE;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(maxDstSize as isize);
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let mut litBufferEnd = (*dctx).litBufferEnd;
    let prefixStart = (*dctx).prefixStart as *const BYTE;
    let vBase = (*dctx).virtualStart as *const BYTE;
    let dictEnd = (*dctx).dictEnd as *const BYTE;
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: 0 as *const libc::c_char,
                start: 0 as *const libc::c_char,
                limitPtr: 0 as *const libc::c_char,
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1 as libc::c_int as U32;
        let mut i: U32 = 0;
        i = 0 as libc::c_int as U32;
        while i < ZSTD_REP_NUM as libc::c_uint {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as size_t;
            i = i.wrapping_add(1);
        }
        if ERR_isError(
            BIT_initDStream(
                &mut seqState.DStream,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            ),
        ) != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        if !dst.is_null() {} else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                1412 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 132],
                    &[libc::c_char; 132],
                >(
                    b"size_t ZSTD_decompressSequences_bodySplitLitBuffer(ZSTD_DCtx *, void *, size_t, const void *, size_t, int, const ZSTD_longOffset_e)\0",
                ))
                    .as_ptr(),
            );
        }
        let mut sequence = ZSTD_decodeSequence(&mut seqState, isLongOffset);
        asm!(".p2align 6", options(preserves_flags, att_syntax));
        while litPtr.offset(sequence.litLength as isize) <= (*dctx).litBufferEnd {
            let oneSeqSize = ZSTD_execSequenceSplitLitBuffer(
                op,
                oend,
                litPtr
                    .offset(sequence.litLength as isize)
                    .offset(-(WILDCOPY_OVERLENGTH as isize)),
                sequence,
                &mut litPtr,
                litBufferEnd,
                prefixStart,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) as libc::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
            if (nbSeq == 0) as libc::c_int as libc::c_long != 0 {
                break;
            }
            BIT_reloadDStream(&mut seqState.DStream);
            sequence = ZSTD_decodeSequence(&mut seqState, isLongOffset);
        }
        if nbSeq > 0 as libc::c_int {
            let leftoverLit = ((*dctx).litBufferEnd).offset_from(litPtr) as libc::c_long
                as size_t;
            if leftoverLit != 0 {
                if leftoverLit > oend.offset_from(op) as libc::c_long as size_t {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
                }
                ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit as ptrdiff_t);
                sequence
                    .litLength = (sequence.litLength as libc::c_ulong)
                    .wrapping_sub(leftoverLit) as size_t as size_t;
                op = op.offset(leftoverLit as isize);
            }
            litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
            litBufferEnd = ((*dctx).litExtraBuffer)
                .as_mut_ptr()
                .offset(
                    (if 64 as libc::c_int
                        > (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1 as libc::c_int) << 16 as libc::c_int)
                            < (128 as libc::c_int) << 10 as libc::c_int
                        {
                            (1 as libc::c_int) << 16 as libc::c_int
                        } else {
                            (128 as libc::c_int) << 10 as libc::c_int
                        })
                    }) as isize,
                );
            (*dctx).litBufferLocation = ZSTD_not_in_dst;
            let oneSeqSize_0 = ZSTD_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litBufferEnd,
                prefixStart,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize_0) as libc::c_long != 0 {
                return oneSeqSize_0;
            }
            op = op.offset(oneSeqSize_0 as isize);
            nbSeq -= 1;
            if nbSeq != 0 {
                BIT_reloadDStream(&mut seqState.DStream);
            }
        }
        if nbSeq > 0 as libc::c_int {
            asm!(".p2align 6", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 4", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 3", options(preserves_flags, att_syntax));
            loop {
                let sequence_0 = ZSTD_decodeSequence(&mut seqState, isLongOffset);
                let oneSeqSize_1 = ZSTD_execSequence(
                    op,
                    oend,
                    sequence_0,
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    vBase,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize_1) as libc::c_long != 0 {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
                nbSeq -= 1;
                if (nbSeq == 0) as libc::c_int as libc::c_long != 0 {
                    break;
                }
                BIT_reloadDStream(&mut seqState.DStream);
            }
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        if (BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint)
            < BIT_DStream_completed as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        let mut i_0: U32 = 0;
        i_0 = 0 as libc::c_int as U32;
        while i_0 < ZSTD_REP_NUM as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as U32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as libc::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as libc::c_long as size_t {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut libc::c_void,
                litPtr as *const libc::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.offset(lastLLSize as isize);
        }
        litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
        litBufferEnd = ((*dctx).litExtraBuffer)
            .as_mut_ptr()
            .offset(
                (if 64 as libc::c_int
                    > (if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    })
                {
                    64 as libc::c_int
                } else {
                    (if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    })
                }) as isize,
            );
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as libc::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut libc::c_void,
            litPtr as *const libc::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    let mut ip = seqStart as *const BYTE;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_not_in_dst as libc::c_int as libc::c_uint
    {
        ostart.offset(maxDstSize as isize)
    } else {
        (*dctx).litBuffer
    };
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.offset((*dctx).litSize as isize);
    let prefixStart = (*dctx).prefixStart as *const BYTE;
    let vBase = (*dctx).virtualStart as *const BYTE;
    let dictEnd = (*dctx).dictEnd as *const BYTE;
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: 0 as *const libc::c_char,
                start: 0 as *const libc::c_char,
                limitPtr: 0 as *const libc::c_char,
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1 as libc::c_int as U32;
        let mut i: U32 = 0;
        i = 0 as libc::c_int as U32;
        while i < ZSTD_REP_NUM as libc::c_uint {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as size_t;
            i = i.wrapping_add(1);
        }
        if ERR_isError(
            BIT_initDStream(
                &mut seqState.DStream,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            ),
        ) != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        if !dst.is_null() {} else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                1627 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 118],
                    &[libc::c_char; 118],
                >(
                    b"size_t ZSTD_decompressSequences_body(ZSTD_DCtx *, void *, size_t, const void *, size_t, int, const ZSTD_longOffset_e)\0",
                ))
                    .as_ptr(),
            );
        }
        asm!(".p2align 6", options(preserves_flags, att_syntax));
        asm!("nop", options(preserves_flags, att_syntax));
        asm!(".p2align 4", options(preserves_flags, att_syntax));
        asm!("nop", options(preserves_flags, att_syntax));
        asm!(".p2align 3", options(preserves_flags, att_syntax));
        loop {
            let sequence = ZSTD_decodeSequence(&mut seqState, isLongOffset);
            let oneSeqSize = ZSTD_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litEnd,
                prefixStart,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) as libc::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
            if (nbSeq == 0) as libc::c_int as libc::c_long != 0 {
                break;
            }
            BIT_reloadDStream(&mut seqState.DStream);
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        if (BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint)
            < BIT_DStream_completed as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        let mut i_0: U32 = 0;
        i_0 = 0 as libc::c_int as U32;
        while i_0 < ZSTD_REP_NUM as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as U32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as libc::c_long as size_t;
    if lastLLSize > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut libc::c_void,
            litPtr as *const libc::c_void,
            lastLLSize as libc::size_t,
        );
        op = op.offset(lastLLSize as isize);
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decompressSequences_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    return ZSTD_decompressSequences_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequencesSplitLitBuffer_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    return ZSTD_decompressSequences_bodySplitLitBuffer(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
#[inline(always)]
unsafe extern "C" fn ZSTD_prefetchMatch(
    mut prefetchPos: size_t,
    sequence: seq_t,
    prefixStart: *const BYTE,
    dictEnd: *const BYTE,
) -> size_t {
    prefetchPos = (prefetchPos as libc::c_ulong).wrapping_add(sequence.litLength)
        as size_t as size_t;
    let matchBase = if sequence.offset > prefetchPos { dictEnd } else { prefixStart };
    let match_0 = matchBase
        .offset(prefetchPos as isize)
        .offset(-(sequence.offset as isize));
    return prefetchPos.wrapping_add(sequence.matchLength);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequencesLong_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    let mut ip = seqStart as *const BYTE;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_in_dst as libc::c_int as libc::c_uint
    {
        (*dctx).litBuffer
    } else {
        ostart.offset(maxDstSize as isize)
    };
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let mut litBufferEnd = (*dctx).litBufferEnd;
    let prefixStart = (*dctx).prefixStart as *const BYTE;
    let dictStart = (*dctx).virtualStart as *const BYTE;
    let dictEnd = (*dctx).dictEnd as *const BYTE;
    if nbSeq != 0 {
        let mut sequences: [seq_t; 8] = [seq_t {
            litLength: 0,
            matchLength: 0,
            offset: 0,
        }; 8];
        let seqAdvance = if nbSeq < 8 as libc::c_int { nbSeq } else { 8 as libc::c_int };
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: 0 as *const libc::c_char,
                start: 0 as *const libc::c_char,
                limitPtr: 0 as *const libc::c_char,
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: 0 as *const ZSTD_seqSymbol,
            },
            prevOffset: [0; 3],
        };
        let mut seqNb: libc::c_int = 0;
        let mut prefetchPos = op.offset_from(prefixStart) as libc::c_long as size_t;
        (*dctx).fseEntropy = 1 as libc::c_int as U32;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < ZSTD_REP_NUM {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as size_t;
            i += 1;
        }
        if !dst.is_null() {} else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                1753 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"size_t ZSTD_decompressSequencesLong_body(ZSTD_DCtx *, void *, size_t, const void *, size_t, int, const ZSTD_longOffset_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if iend >= ip {} else {
            __assert_fail(
                b"iend >= ip\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                1754 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"size_t ZSTD_decompressSequencesLong_body(ZSTD_DCtx *, void *, size_t, const void *, size_t, int, const ZSTD_longOffset_e)\0",
                ))
                    .as_ptr(),
            );
        }
        if ERR_isError(
            BIT_initDStream(
                &mut seqState.DStream,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            ),
        ) != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        seqNb = 0 as libc::c_int;
        while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint
            <= BIT_DStream_completed as libc::c_int as libc::c_uint && seqNb < seqAdvance
        {
            let sequence = ZSTD_decodeSequence(&mut seqState, isLongOffset);
            prefetchPos = ZSTD_prefetchMatch(
                prefetchPos,
                sequence,
                prefixStart,
                dictEnd,
            );
            sequences[seqNb as usize] = sequence;
            seqNb += 1;
        }
        if seqNb < seqAdvance {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint
            <= BIT_DStream_completed as libc::c_int as libc::c_uint && seqNb < nbSeq
        {
            let mut sequence_0 = ZSTD_decodeSequence(&mut seqState, isLongOffset);
            let mut oneSeqSize: size_t = 0;
            if (*dctx).litBufferLocation as libc::c_uint
                == ZSTD_split as libc::c_int as libc::c_uint
                && litPtr
                    .offset(
                        sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize]
                            .litLength as isize,
                    ) > (*dctx).litBufferEnd
            {
                let leftoverLit = ((*dctx).litBufferEnd).offset_from(litPtr)
                    as libc::c_long as size_t;
                if leftoverLit != 0 {
                    if leftoverLit > oend.offset_from(op) as libc::c_long as size_t {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit as ptrdiff_t);
                    sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize]
                        .litLength = (sequences[(seqNb - ADVANCED_SEQS
                            & STORED_SEQS_MASK) as usize]
                        .litLength as libc::c_ulong)
                        .wrapping_sub(leftoverLit) as size_t as size_t;
                    op = op.offset(leftoverLit as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer)
                    .as_mut_ptr()
                    .offset(
                        (if 64 as libc::c_int
                            > (if ((1 as libc::c_int) << 16 as libc::c_int)
                                < (128 as libc::c_int) << 10 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                (128 as libc::c_int) << 10 as libc::c_int
                            })
                        {
                            64 as libc::c_int
                        } else {
                            (if ((1 as libc::c_int) << 16 as libc::c_int)
                                < (128 as libc::c_int) << 10 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                (128 as libc::c_int) << 10 as libc::c_int
                            })
                        }) as isize,
                    );
                (*dctx).litBufferLocation = ZSTD_not_in_dst;
                oneSeqSize = ZSTD_execSequence(
                    op,
                    oend,
                    sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize],
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize) != 0 {
                    return oneSeqSize;
                }
                prefetchPos = ZSTD_prefetchMatch(
                    prefetchPos,
                    sequence_0,
                    prefixStart,
                    dictEnd,
                );
                sequences[(seqNb & STORED_SEQS_MASK) as usize] = sequence_0;
                op = op.offset(oneSeqSize as isize);
            } else {
                oneSeqSize = if (*dctx).litBufferLocation as libc::c_uint
                    == ZSTD_split as libc::c_int as libc::c_uint
                {
                    ZSTD_execSequenceSplitLitBuffer(
                        op,
                        oend,
                        litPtr
                            .offset(
                                sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK)
                                        as usize]
                                    .litLength as isize,
                            )
                            .offset(-(WILDCOPY_OVERLENGTH as isize)),
                        sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize],
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                } else {
                    ZSTD_execSequence(
                        op,
                        oend,
                        sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize],
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                };
                if ERR_isError(oneSeqSize) != 0 {
                    return oneSeqSize;
                }
                prefetchPos = ZSTD_prefetchMatch(
                    prefetchPos,
                    sequence_0,
                    prefixStart,
                    dictEnd,
                );
                sequences[(seqNb & STORED_SEQS_MASK) as usize] = sequence_0;
                op = op.offset(oneSeqSize as isize);
            }
            seqNb += 1;
        }
        if seqNb < nbSeq {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        seqNb -= seqAdvance;
        while seqNb < nbSeq {
            let mut sequence_1: *mut seq_t = &mut *sequences
                .as_mut_ptr()
                .offset((seqNb & STORED_SEQS_MASK) as isize) as *mut seq_t;
            if (*dctx).litBufferLocation as libc::c_uint
                == ZSTD_split as libc::c_int as libc::c_uint
                && litPtr.offset((*sequence_1).litLength as isize) > (*dctx).litBufferEnd
            {
                let leftoverLit_0 = ((*dctx).litBufferEnd).offset_from(litPtr)
                    as libc::c_long as size_t;
                if leftoverLit_0 != 0 {
                    if leftoverLit_0 > oend.offset_from(op) as libc::c_long as size_t {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit_0 as ptrdiff_t);
                    (*sequence_1)
                        .litLength = ((*sequence_1).litLength as libc::c_ulong)
                        .wrapping_sub(leftoverLit_0) as size_t as size_t;
                    op = op.offset(leftoverLit_0 as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer)
                    .as_mut_ptr()
                    .offset(
                        (if 64 as libc::c_int
                            > (if ((1 as libc::c_int) << 16 as libc::c_int)
                                < (128 as libc::c_int) << 10 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                (128 as libc::c_int) << 10 as libc::c_int
                            })
                        {
                            64 as libc::c_int
                        } else {
                            (if ((1 as libc::c_int) << 16 as libc::c_int)
                                < (128 as libc::c_int) << 10 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                (128 as libc::c_int) << 10 as libc::c_int
                            })
                        }) as isize,
                    );
                (*dctx).litBufferLocation = ZSTD_not_in_dst;
                let oneSeqSize_0 = ZSTD_execSequence(
                    op,
                    oend,
                    *sequence_1,
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize_0) != 0 {
                    return oneSeqSize_0;
                }
                op = op.offset(oneSeqSize_0 as isize);
            } else {
                let oneSeqSize_1 = if (*dctx).litBufferLocation as libc::c_uint
                    == ZSTD_split as libc::c_int as libc::c_uint
                {
                    ZSTD_execSequenceSplitLitBuffer(
                        op,
                        oend,
                        litPtr
                            .offset((*sequence_1).litLength as isize)
                            .offset(-(WILDCOPY_OVERLENGTH as isize)),
                        *sequence_1,
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                } else {
                    ZSTD_execSequence(
                        op,
                        oend,
                        *sequence_1,
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                };
                if ERR_isError(oneSeqSize_1) != 0 {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
            }
            seqNb += 1;
        }
        let mut i_0: U32 = 0;
        i_0 = 0 as libc::c_int as U32;
        while i_0 < ZSTD_REP_NUM as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as U32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as libc::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as libc::c_long as size_t {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut libc::c_void,
                litPtr as *const libc::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.offset(lastLLSize as isize);
        }
        litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
        litBufferEnd = ((*dctx).litExtraBuffer)
            .as_mut_ptr()
            .offset(
                (if 64 as libc::c_int
                    > (if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    })
                {
                    64 as libc::c_int
                } else {
                    (if ((1 as libc::c_int) << 16 as libc::c_int)
                        < (128 as libc::c_int) << 10 as libc::c_int
                    {
                        (1 as libc::c_int) << 16 as libc::c_int
                    } else {
                        (128 as libc::c_int) << 10 as libc::c_int
                    })
                }) as isize,
            );
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as libc::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memmove(
            op as *mut libc::c_void,
            litPtr as *const libc::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
pub const STORED_SEQS: libc::c_int = 8 as libc::c_int;
pub const STORED_SEQS_MASK: libc::c_int = STORED_SEQS - 1 as libc::c_int;
pub const ADVANCED_SEQS: libc::c_int = STORED_SEQS;
unsafe extern "C" fn ZSTD_decompressSequencesLong_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    return ZSTD_decompressSequencesLong_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequences_bmi2(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    return ZSTD_decompressSequences_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequencesSplitLitBuffer_bmi2(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    return ZSTD_decompressSequences_bodySplitLitBuffer(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_bmi2(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    return ZSTD_decompressSequencesLong_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequences(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        return ZSTD_decompressSequences_bmi2(
            dctx,
            dst,
            maxDstSize,
            seqStart,
            seqSize,
            nbSeq,
            isLongOffset,
        );
    }
    return ZSTD_decompressSequences_default(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequencesSplitLitBuffer(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        return ZSTD_decompressSequencesSplitLitBuffer_bmi2(
            dctx,
            dst,
            maxDstSize,
            seqStart,
            seqSize,
            nbSeq,
            isLongOffset,
        );
    }
    return ZSTD_decompressSequencesSplitLitBuffer_default(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_decompressSequencesLong(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        return ZSTD_decompressSequencesLong_bmi2(
            dctx,
            dst,
            maxDstSize,
            seqStart,
            seqSize,
            nbSeq,
            isLongOffset,
        );
    }
    return ZSTD_decompressSequencesLong_default(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    );
}
unsafe extern "C" fn ZSTD_totalHistorySize(
    mut op: *mut BYTE,
    mut virtualStart: *const BYTE,
) -> size_t {
    return op.offset_from(virtualStart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_getOffsetInfo(
    mut offTable: *const ZSTD_seqSymbol,
    mut nbSeq: libc::c_int,
) -> ZSTD_OffsetInfo {
    let mut info = {
        let mut init = ZSTD_OffsetInfo {
            longOffsetShare: 0 as libc::c_int as libc::c_uint,
            maxNbAdditionalBits: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    if nbSeq != 0 as libc::c_int {
        let mut ptr = offTable as *const libc::c_void;
        let tableLog = (*(ptr as *const ZSTD_seqSymbol_header)
            .offset(0 as libc::c_int as isize))
            .tableLog;
        let mut table = offTable.offset(1 as libc::c_int as isize);
        let max = ((1 as libc::c_int) << tableLog) as U32;
        let mut u: U32 = 0;
        if max <= ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint {} else {
            __assert_fail(
                b"max <= (1 << OffFSELog)\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                2030 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"ZSTD_OffsetInfo ZSTD_getOffsetInfo(const ZSTD_seqSymbol *, int)\0"))
                    .as_ptr(),
            );
        }
        u = 0 as libc::c_int as U32;
        while u < max {
            info
                .maxNbAdditionalBits = if info.maxNbAdditionalBits
                > (*table.offset(u as isize)).nbAdditionalBits as libc::c_uint
            {
                info.maxNbAdditionalBits
            } else {
                (*table.offset(u as isize)).nbAdditionalBits as libc::c_uint
            };
            if (*table.offset(u as isize)).nbAdditionalBits as libc::c_int
                > 22 as libc::c_int
            {
                info
                    .longOffsetShare = (info.longOffsetShare)
                    .wrapping_add(1 as libc::c_int as libc::c_uint);
            }
            u = u.wrapping_add(1);
        }
        if tableLog <= 8 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"tableLog <= OffFSELog\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                2036 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"ZSTD_OffsetInfo ZSTD_getOffsetInfo(const ZSTD_seqSymbol *, int)\0"))
                    .as_ptr(),
            );
        }
        info.longOffsetShare <<= (OffFSELog as libc::c_uint).wrapping_sub(tableLog);
    }
    return info;
}
unsafe extern "C" fn ZSTD_maxShortOffset() -> size_t {
    if MEM_64bits() != 0 {
        return -(1 as libc::c_int) as size_t
    } else {
        let maxOffbase = ((1 as libc::c_int as size_t)
            << ((if MEM_32bits() != 0 {
                STREAM_ACCUMULATOR_MIN_32
            } else {
                STREAM_ACCUMULATOR_MIN_64
            }) as U32)
                .wrapping_add(1 as libc::c_int as libc::c_uint))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let maxOffset = maxOffbase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong);
        if ZSTD_highbit32(maxOffbase as U32)
            == (if MEM_32bits() != 0 { 25 as libc::c_int } else { 57 as libc::c_int })
                as U32
        {} else {
            __assert_fail(
                b"ZSTD_highbit32((U32)maxOffbase) == STREAM_ACCUMULATOR_MIN\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/decompress/zstd_decompress_block.c\0"
                    as *const u8 as *const libc::c_char,
                2063 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"size_t ZSTD_maxShortOffset(void)\0"))
                    .as_ptr(),
            );
        }
        return maxOffset;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock_internal(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    streaming: streaming_operation,
) -> size_t {
    let mut ip = src as *const BYTE;
    if srcSize > ZSTD_blockSizeMax(dctx) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let litCSize = ZSTD_decodeLiteralsBlock(
        dctx,
        src,
        srcSize,
        dst,
        dstCapacity,
        streaming,
    );
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(litCSize) as size_t as size_t;
    let blockSizeMax = if dstCapacity < ZSTD_blockSizeMax(dctx) {
        dstCapacity
    } else {
        ZSTD_blockSizeMax(dctx)
    };
    let totalHistorySize = ZSTD_totalHistorySize(
        (dst as *mut BYTE).offset(blockSizeMax as isize),
        (*dctx).virtualStart as *const BYTE,
    );
    let mut isLongOffset = (MEM_32bits() != 0
        && totalHistorySize > ZSTD_maxShortOffset()) as libc::c_int as ZSTD_longOffset_e;
    let mut usePrefetchDecoder = (*dctx).ddictIsCold;
    let mut nbSeq: libc::c_int = 0;
    let seqHSize = ZSTD_decodeSeqHeaders(
        dctx,
        &mut nbSeq,
        ip as *const libc::c_void,
        srcSize,
    );
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = ip.offset(seqHSize as isize);
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(seqHSize) as size_t as size_t;
    if (dst.is_null() || dstCapacity == 0 as libc::c_int as libc::c_ulong)
        && nbSeq > 0 as libc::c_int
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if MEM_64bits() != 0
        && ::core::mem::size_of::<size_t>() as libc::c_ulong
            == ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        && (-(1 as libc::c_int) as size_t).wrapping_sub(dst as size_t)
            < ((1 as libc::c_int) << 20 as libc::c_int) as size_t
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if isLongOffset as libc::c_uint != 0
        || usePrefetchDecoder == 0
            && totalHistorySize
                > ((1 as libc::c_uint) << 24 as libc::c_int) as libc::c_ulong
            && nbSeq > 8 as libc::c_int
    {
        let info = ZSTD_getOffsetInfo((*dctx).OFTptr, nbSeq);
        if isLongOffset as libc::c_uint != 0
            && info.maxNbAdditionalBits
                <= (if MEM_32bits() != 0 {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as U32
        {
            isLongOffset = ZSTD_lo_isRegularOffset;
        }
        if usePrefetchDecoder == 0 {
            let minShare = (if MEM_64bits() != 0 {
                7 as libc::c_int
            } else {
                20 as libc::c_int
            }) as U32;
            usePrefetchDecoder = (info.longOffsetShare >= minShare) as libc::c_int;
        }
    }
    (*dctx).ddictIsCold = 0 as libc::c_int;
    if usePrefetchDecoder != 0 {
        return ZSTD_decompressSequencesLong(
            dctx,
            dst,
            dstCapacity,
            ip as *const libc::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        );
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        return ZSTD_decompressSequencesSplitLitBuffer(
            dctx,
            dst,
            dstCapacity,
            ip as *const libc::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        )
    } else {
        return ZSTD_decompressSequences(
            dctx,
            dst,
            dstCapacity,
            ip as *const libc::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_checkContinuity(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *const libc::c_void,
    mut dstSize: size_t,
) {
    if dst != (*dctx).previousDstEnd && dstSize > 0 as libc::c_int as libc::c_ulong {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx)
            .virtualStart = (dst as *const libc::c_char)
            .offset(
                -(((*dctx).previousDstEnd as *const libc::c_char)
                    .offset_from((*dctx).prefixStart as *const libc::c_char)
                    as libc::c_long as isize),
            ) as *const libc::c_void;
        (*dctx).prefixStart = dst;
        (*dctx).previousDstEnd = dst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock_deprecated(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut dSize: size_t = 0;
    (*dctx).isFrameDecompression = 0 as libc::c_int;
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    dSize = ZSTD_decompressBlock_internal(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        not_streaming,
    );
    (*dctx)
        .previousDstEnd = (dst as *mut libc::c_char).offset(dSize as isize)
        as *const libc::c_void;
    return dSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZSTD_decompressBlock_deprecated(dctx, dst, dstCapacity, src, srcSize);
}
