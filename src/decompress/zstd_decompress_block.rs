use crate::__m128i_u;
use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use core::arch::asm;
extern "C" {
    pub type ZSTD_DDict_s;
    fn FSE_readNCount(
        normalizedCounter: *mut libc::c_short,
        maxSymbolValuePtr: *mut libc::c_uint,
        tableLogPtr: *mut libc::c_uint,
        rBuffer: *const libc::c_void,
        rBuffSize: libc::size_t,
    ) -> libc::size_t;
    fn HUF_decompress1X_usingDTable(
        dst: *mut libc::c_void,
        maxDstSize: libc::size_t,
        cSrc: *const libc::c_void,
        cSrcSize: libc::size_t,
        DTable: *const HUF_DTable,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn HUF_decompress1X1_DCtx_wksp(
        dctx: *mut HUF_DTable,
        dst: *mut libc::c_void,
        dstSize: libc::size_t,
        cSrc: *const libc::c_void,
        cSrcSize: libc::size_t,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn HUF_decompress4X_usingDTable(
        dst: *mut libc::c_void,
        maxDstSize: libc::size_t,
        cSrc: *const libc::c_void,
        cSrcSize: libc::size_t,
        DTable: *const HUF_DTable,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn HUF_decompress4X_hufOnly_wksp(
        dctx: *mut HUF_DTable,
        dst: *mut libc::c_void,
        dstSize: libc::size_t,
        cSrc: *const libc::c_void,
        cSrcSize: libc::size_t,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
        flags: libc::c_int,
    ) -> libc::size_t;
}
pub type ptrdiff_t = libc::c_long;
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
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
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
    pub bitContainer: libc::size_t,
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
pub type HUF_DTable = u32;
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
    pub workspace: [u32; 640],
    pub previousDstEnd: *const libc::c_void,
    pub prefixStart: *const libc::c_void,
    pub virtualStart: *const libc::c_void,
    pub dictEnd: *const libc::c_void,
    pub expected: libc::size_t,
    pub fParams: ZSTD_frameHeader,
    pub processedCSize: u64,
    pub decodedSize: u64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: libc::size_t,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTD_customMem,
    pub litSize: libc::size_t,
    pub rleSize: libc::size_t,
    pub staticSize: libc::size_t,
    pub isFrameDecompression: libc::c_int,
    pub bmi2: libc::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: u32,
    pub ddictIsCold: libc::c_int,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: libc::c_int,
    pub maxBlockSizeParam: libc::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: libc::size_t,
    pub inPos: libc::size_t,
    pub maxWindowSize: libc::size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: libc::size_t,
    pub outStart: libc::size_t,
    pub outEnd: libc::size_t,
    pub lhSize: libc::size_t,
    pub legacyContext: *mut libc::c_void,
    pub previousLegacyVersion: u32,
    pub legacyVersion: u32,
    pub hostageByte: u32,
    pub noForwardProgress: libc::c_int,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut u8,
    pub litBufferEnd: *const u8,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [u8; 65568],
    pub headerBuffer: [u8; 18],
    pub oversizedDuration: libc::size_t,
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
    pub size: libc::size_t,
    pub pos: libc::size_t,
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
    pub ddictPtrTableSize: libc::size_t,
    pub ddictPtrCount: libc::size_t,
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
    unsafe extern "C" fn(*mut libc::c_void, libc::size_t) -> *mut libc::c_void,
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
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
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
    pub rep: [u32; 3],
    pub workspace: [u32; 157],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol {
    pub nextState: u16,
    pub nbAdditionalBits: u8,
    pub nbBits: u8,
    pub baseValue: u32,
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
    pub prevOffset: [libc::size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseState {
    pub state: libc::size_t,
    pub table: *const ZSTD_seqSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: libc::size_t,
    pub matchLength: libc::size_t,
    pub offset: libc::size_t,
}
pub type ZSTD_overlap_e = libc::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol_header {
    pub fastMode: u32,
    pub tableLog: u32,
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
    pub lastBlock: u32,
    pub origSize: u32,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const CACHELINE_SIZE: libc::c_int = 64 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>()
        == 4) as libc::c_int as libc::c_uint;
}
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
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: u64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> u16 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const u8;
        return (*p.offset(0) as libc::c_int
            + ((*p.offset(1) as libc::c_int)
                << 8 as libc::c_int)) as u16;
    };
}
#[inline]
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> u32 {
    return (MEM_readLE16(memPtr) as u32)
        .wrapping_add(
            (*(memPtr as *const u8).offset(2) as u32)
                << 16 as libc::c_int,
        );
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
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> libc::size_t {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as libc::size_t
    } else {
        return MEM_readLE64(memPtr)
    };
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return (31)
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
    mut srcSize: libc::size_t,
) -> libc::size_t {
    if srcSize < 1 {
        libc::memset(
            bitD as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<BIT_DStream_t>() as libc::size_t,
        );
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    (*bitD).start = srcBuffer as *const libc::c_char;
    (*bitD)
        .limitPtr = ((*bitD).start)
        .offset(::core::mem::size_of::<libc::size_t>() as isize);
    if srcSize >= ::core::mem::size_of::<libc::size_t>() {
        (*bitD)
            .ptr = (srcBuffer as *const libc::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<libc::size_t>() as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        let lastByte = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte as libc::c_int != 0 {
            (8)
                .wrapping_sub(ZSTD_highbit32(lastByte as u32))
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if lastByte as libc::c_int == 0 {
            return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as libc::size_t;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(6)
                            as libc::size_t)
                            << (::core::mem::size_of::<libc::size_t>())
                                .wrapping_mul(8)
                                .wrapping_sub(16),
                    ) ;
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
                        (*(srcBuffer as *const u8).offset(5)
                            as libc::size_t)
                            << (::core::mem::size_of::<libc::size_t>())
                                .wrapping_mul(8)
                                .wrapping_sub(24),
                    ) ;
                current_block_32 = 5467347503347430154;
            }
            _ => {}
        }
        match current_block_32 {
            5467347503347430154 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(4)
                            as libc::size_t)
                            << (::core::mem::size_of::<libc::size_t>())
                                .wrapping_mul(8)
                                .wrapping_sub(32),
                    ) ;
                current_block_32 = 15935546777885233963;
            }
            _ => {}
        }
        match current_block_32 {
            15935546777885233963 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(3)
                            as libc::size_t) << 24 as libc::c_int,
                    ) ;
                current_block_32 = 15098265657425327076;
            }
            _ => {}
        }
        match current_block_32 {
            15098265657425327076 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(2)
                            as libc::size_t) << 16 as libc::c_int,
                    ) ;
                current_block_32 = 18096294377129956667;
            }
            _ => {}
        }
        match current_block_32 {
            18096294377129956667 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*(srcBuffer as *const u8).offset(1)
                            as libc::size_t) << 8 as libc::c_int,
                    ) ;
            }
            _ => {}
        }
        let lastByte_0 = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte_0 as libc::c_int != 0 {
            (8)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as u32))
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if lastByte_0 as libc::c_int == 0 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                ((::core::mem::size_of::<libc::size_t>())
                    .wrapping_sub(srcSize) as u32)
                    .wrapping_mul(8),
            );
    }
    return srcSize;
}
#[inline(always)]
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: libc::c_uint,
) -> libc::size_t {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> libc::size_t {
    return BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8)
            .wrapping_sub((*bitD).bitsConsumed as libc::c_ulong)
            .wrapping_sub(nbBits as libc::c_ulong) as u32,
        nbBits,
    );
}
#[inline(always)]
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: libc::size_t,
    start: u32,
    nbBits: u32,
) -> libc::size_t {
    let regMask = (::core::mem::size_of::<libc::size_t>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    debug_assert!((nbBits as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uint; 32]>())
            .wrapping_div(::core::mem::size_of::<libc::c_uint>()));
    return bitContainer >> (start & regMask)
        & ((1) << nbBits)
            .wrapping_sub(1);
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong
        > (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8)
    {
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStreamFast(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong)
            < (::core::mem::size_of::<libc::size_t>())
                .wrapping_mul(8)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as libc::c_int;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as libc::c_long as u32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD)
        .bitsConsumed = ((*bitD).bitsConsumed)
        .wrapping_sub(nbBytes.wrapping_mul(8));
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
    debug_assert!((*bitD).bitsConsumed as libc::c_ulong
        <= (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8));
    (*bitD)
        .ptr = ((*bitD).ptr)
        .offset(-(((*bitD).bitsConsumed >> 3 as libc::c_int) as isize));
    (*bitD).bitsConsumed &= 7;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return BIT_DStream_unfinished;
}
#[inline]
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: libc::c_uint,
) -> libc::size_t {
    let value = BIT_lookBitsFast(bitD, nbBits);
    debug_assert!(nbBits >= 1);
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> libc::size_t {
    let regMask = (::core::mem::size_of::<libc::size_t>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    debug_assert!(nbBits >= 1);
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask.wrapping_add(1).wrapping_sub(nbBits)
            & regMask);
}
pub const STREAM_ACCUMULATOR_MIN_64: libc::c_int = 57 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
pub const STREAM_ACCUMULATOR_MIN_32: libc::c_int = 25 as libc::c_int;
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
pub const ZSTD_BLOCKSIZELOG_MAX: libc::c_int = 17 as libc::c_int;
pub const ZSTD_BLOCKSIZE_MAX: libc::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_WINDOWLOG_MAX_32: libc::c_int = 30 as libc::c_int;
pub const ZSTD_REP_NUM: libc::c_int = 3 as libc::c_int;
pub const ZSTD_BLOCKHEADERSIZE: libc::c_int = 3 as libc::c_int;
static mut ZSTD_blockHeaderSize: libc::size_t = ZSTD_BLOCKHEADERSIZE as libc::size_t;
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
pub const ZSTD_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
pub const OffFSELog: libc::c_int = 8 as libc::c_int;
pub const LONGNBSEQ: libc::c_int = 0x7f00 as libc::c_int;
pub const ML_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
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
pub const MaxLL: libc::c_int = 35 as libc::c_int;
pub const MLFSELog: libc::c_int = 9 as libc::c_int;
pub const MaxML: libc::c_int = 52 as libc::c_int;
pub const LLFSELog: libc::c_int = 9 as libc::c_int;
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
pub const LL_DEFAULTNORMLOG: libc::c_int = 6 as libc::c_int;
pub const MaxOff: libc::c_int = 31 as libc::c_int;
pub const OF_DEFAULTNORMLOG: libc::c_int = 5 as libc::c_int;
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
static mut OF_base: [u32; 32] = [
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
    0x1ffffffd as libc::c_int as u32,
    0x3ffffffd as libc::c_int as u32,
    0x7ffffffd as libc::c_int as u32,
];
static mut OF_bits: [u8; 32] = [
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
unsafe extern "C" fn ZSTD_blockSizeMax(mut dctx: *const ZSTD_DCtx) -> libc::size_t {
    let blockSizeMax = (if (*dctx).isFrameDecompression != 0 {
        (*dctx).fParams.blockSizeMax
    } else {
        ZSTD_BLOCKSIZE_MAX as libc::c_uint
    }) as libc::size_t;
    debug_assert!(blockSizeMax <= ((1) << 17 as libc::c_int) as libc::c_ulong);
    return blockSizeMax;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getcBlockSize(
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut bpPtr: *mut blockProperties_t,
) -> libc::size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let cBlockHeader = MEM_readLE24(src);
    let cSize = cBlockHeader >> 3 as libc::c_int;
    (*bpPtr).lastBlock = cBlockHeader & 1;
    (*bpPtr)
        .blockType = (cBlockHeader >> 1 as libc::c_int
        & 3) as blockType_e;
    (*bpPtr).origSize = cSize;
    if (*bpPtr).blockType as libc::c_uint == bt_rle as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::size_t;
    }
    if (*bpPtr).blockType as libc::c_uint == bt_reserved as libc::c_int as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    return cSize as libc::size_t;
}
unsafe extern "C" fn ZSTD_allocateLiteralsBuffer(
    mut dctx: *mut ZSTD_DCtx,
    dst: *mut libc::c_void,
    dstCapacity: libc::size_t,
    litSize: libc::size_t,
    streaming: streaming_operation,
    expectedWriteSize: libc::size_t,
    splitImmediately: libc::c_uint,
) {
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    debug_assert!(litSize <= blockSizeMax);
    debug_assert!((*dctx).isFrameDecompression != 0
        || streaming as libc::c_uint == not_streaming as libc::c_int as libc::c_uint);
    debug_assert!(expectedWriteSize <= blockSizeMax);
    if streaming as libc::c_uint == not_streaming as libc::c_int as libc::c_uint
        && dstCapacity
            > blockSizeMax
                .wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong)
                .wrapping_add(litSize)
                .wrapping_add(WILDCOPY_OVERLENGTH as libc::c_ulong)
    {
        (*dctx)
            .litBuffer = (dst as *mut u8)
            .offset(blockSizeMax as isize)
            .offset(WILDCOPY_OVERLENGTH as isize);
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_in_dst;
    } else if litSize
        <= (if 64 as libc::c_int
            > (if ((1) << 16 as libc::c_int)
                < (128) << 10 as libc::c_int
            {
                (1) << 16 as libc::c_int
            } else {
                (128) << 10 as libc::c_int
            })
        {
            64 as libc::c_int
        } else {
            (if ((1) << 16 as libc::c_int)
                < (128) << 10 as libc::c_int
            {
                (1) << 16 as libc::c_int
            } else {
                (128) << 10 as libc::c_int
            })
        }) as libc::c_ulong
    {
        (*dctx).litBuffer = ((*dctx).litExtraBuffer).as_mut_ptr();
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    } else {
        debug_assert!(blockSizeMax
            > (if 64 as libc::c_int
                > (if ((1) << 16 as libc::c_int)
                    < (128) << 10 as libc::c_int
                {
                    (1) << 16 as libc::c_int
                } else {
                    (128) << 10 as libc::c_int
                })
            {
                64 as libc::c_int
            } else {
                (if ((1) << 16 as libc::c_int)
                    < (128) << 10 as libc::c_int
                {
                    (1) << 16 as libc::c_int
                } else {
                    (128) << 10 as libc::c_int
                })
            }) as libc::c_ulong);
        if splitImmediately != 0 {
            (*dctx)
                .litBuffer = (dst as *mut u8)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize))
                .offset(
                    (if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    }) as isize,
                )
                .offset(-(WILDCOPY_OVERLENGTH as isize));
            (*dctx)
                .litBufferEnd = ((*dctx).litBuffer)
                .offset(litSize as isize)
                .offset(
                    -((if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    }) as isize),
                );
        } else {
            (*dctx)
                .litBuffer = (dst as *mut u8)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize));
            (*dctx).litBufferEnd = (dst as *mut u8).offset(expectedWriteSize as isize);
        }
        (*dctx).litBufferLocation = ZSTD_split;
        debug_assert!((*dctx).litBufferEnd
            <= (dst as *mut u8).offset(expectedWriteSize as isize) as *const u8);
    };
}
unsafe extern "C" fn ZSTD_decodeLiteralsBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    streaming: streaming_operation,
) -> libc::size_t {
    if srcSize < (1 as libc::c_int + 1) as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let istart = src as *const u8;
    let litEncType = (*istart.offset(0) as libc::c_int
        & 3) as symbolEncodingType_e;
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    match litEncType as libc::c_uint {
        3 => {
            if (*dctx).litEntropy == 0 {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as libc::size_t;
            }
        }
        2 => {}
        0 => {
            let mut litSize_0: libc::size_t = 0;
            let mut lhSize_0: libc::size_t = 0;
            let lhlCode_0 = (*istart.offset(0) as libc::c_int
                >> 2 as libc::c_int & 3) as u32;
            let mut expectedWriteSize_0 = if blockSizeMax < dstCapacity {
                blockSizeMax
            } else {
                dstCapacity
            };
            match lhlCode_0 {
                1 => {
                    lhSize_0 = 2 as libc::c_int as libc::size_t;
                    litSize_0 = (MEM_readLE16(istart as *const libc::c_void)
                        as libc::c_int >> 4 as libc::c_int) as libc::size_t;
                }
                3 => {
                    lhSize_0 = 3 as libc::c_int as libc::size_t;
                    if srcSize < 3 {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as libc::size_t;
                    }
                    litSize_0 = (MEM_readLE24(istart as *const libc::c_void)
                        >> 4 as libc::c_int) as libc::size_t;
                }
                0 | 2 | _ => {
                    lhSize_0 = 1 as libc::c_int as libc::size_t;
                    litSize_0 = (*istart.offset(0) as libc::c_int
                        >> 3 as libc::c_int) as libc::size_t;
                }
            }
            if litSize_0 > 0 && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
            }
            if litSize_0 > blockSizeMax {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if expectedWriteSize_0 < litSize_0 {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
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
                    return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
                                    > (if ((1) << 16 as libc::c_int)
                                        < (128) << 10 as libc::c_int
                                    {
                                        (1) << 16 as libc::c_int
                                    } else {
                                        (128) << 10 as libc::c_int
                                    })
                                {
                                    64 as libc::c_int
                                } else {
                                    (if ((1) << 16 as libc::c_int)
                                        < (128) << 10 as libc::c_int
                                    {
                                        (1) << 16 as libc::c_int
                                    } else {
                                        (128) << 10 as libc::c_int
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
                                    > (if ((1) << 16 as libc::c_int)
                                        < (128) << 10 as libc::c_int
                                    {
                                        (1) << 16 as libc::c_int
                                    } else {
                                        (128) << 10 as libc::c_int
                                    })
                                {
                                    64 as libc::c_int
                                } else {
                                    (if ((1) << 16 as libc::c_int)
                                        < (128) << 10 as libc::c_int
                                    {
                                        (1) << 16 as libc::c_int
                                    } else {
                                        (128) << 10 as libc::c_int
                                    })
                                }) as isize),
                            ) as *const libc::c_void,
                        (if 64 as libc::c_int
                            > (if ((1) << 16 as libc::c_int)
                                < (128) << 10 as libc::c_int
                            {
                                (1) << 16 as libc::c_int
                            } else {
                                (128) << 10 as libc::c_int
                            })
                        {
                            64 as libc::c_int
                        } else if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
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
            let lhlCode_1 = (*istart.offset(0) as libc::c_int
                >> 2 as libc::c_int & 3) as u32;
            let mut litSize_1: libc::size_t = 0;
            let mut lhSize_1: libc::size_t = 0;
            let mut expectedWriteSize_1 = if blockSizeMax < dstCapacity {
                blockSizeMax
            } else {
                dstCapacity
            };
            match lhlCode_1 {
                1 => {
                    lhSize_1 = 2 as libc::c_int as libc::size_t;
                    if srcSize < 3 {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as libc::size_t;
                    }
                    litSize_1 = (MEM_readLE16(istart as *const libc::c_void)
                        as libc::c_int >> 4 as libc::c_int) as libc::size_t;
                }
                3 => {
                    lhSize_1 = 3 as libc::c_int as libc::size_t;
                    if srcSize < 4 {
                        return -(ZSTD_error_corruption_detected as libc::c_int)
                            as libc::size_t;
                    }
                    litSize_1 = (MEM_readLE24(istart as *const libc::c_void)
                        >> 4 as libc::c_int) as libc::size_t;
                }
                0 | 2 | _ => {
                    lhSize_1 = 1 as libc::c_int as libc::size_t;
                    litSize_1 = (*istart.offset(0) as libc::c_int
                        >> 3 as libc::c_int) as libc::size_t;
                }
            }
            if litSize_1 > 0 && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
            }
            if litSize_1 > blockSizeMax {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if expectedWriteSize_1 < litSize_1 {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
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
                                > (if ((1) << 16 as libc::c_int)
                                    < (128) << 10 as libc::c_int
                                {
                                    (1) << 16 as libc::c_int
                                } else {
                                    (128) << 10 as libc::c_int
                                })
                            {
                                64 as libc::c_int
                            } else {
                                (if ((1) << 16 as libc::c_int)
                                    < (128) << 10 as libc::c_int
                                {
                                    (1) << 16 as libc::c_int
                                } else {
                                    (128) << 10 as libc::c_int
                                })
                            }) as libc::c_ulong,
                        ) as libc::size_t,
                );
                libc::memset(
                    ((*dctx).litExtraBuffer).as_mut_ptr() as *mut libc::c_void,
                    *istart.offset(lhSize_1 as isize) as libc::c_int,
                    (if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
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
            return lhSize_1.wrapping_add(1);
        }
        _ => return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t,
    }
    if srcSize < 5 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let mut lhSize: libc::size_t = 0;
    let mut litSize: libc::size_t = 0;
    let mut litCSize: libc::size_t = 0;
    let mut singleStream = 0 as libc::c_int as u32;
    let lhlCode = (*istart.offset(0) as libc::c_int
        >> 2 as libc::c_int & 3) as u32;
    let lhc = MEM_readLE32(istart as *const libc::c_void);
    let mut hufSuccess: libc::size_t = 0;
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
            lhSize = 4 as libc::c_int as libc::size_t;
            litSize = (lhc >> 4 as libc::c_int & 0x3fff as libc::c_int as libc::c_uint)
                as libc::size_t;
            litCSize = (lhc >> 18 as libc::c_int) as libc::size_t;
        }
        3 => {
            lhSize = 5 as libc::c_int as libc::size_t;
            litSize = (lhc >> 4 as libc::c_int & 0x3ffff as libc::c_int as libc::c_uint)
                as libc::size_t;
            litCSize = ((lhc >> 22 as libc::c_int) as libc::c_ulong)
                .wrapping_add(
                    (*istart.offset(4) as libc::size_t)
                        << 10 as libc::c_int,
                );
        }
        0 | 1 | _ => {
            singleStream = (lhlCode == 0) as libc::c_int as u32;
            lhSize = 3 as libc::c_int as libc::size_t;
            litSize = (lhc >> 4 as libc::c_int & 0x3ff as libc::c_int as libc::c_uint)
                as libc::size_t;
            litCSize = (lhc >> 14 as libc::c_int & 0x3ff as libc::c_int as libc::c_uint)
                as libc::size_t;
        }
    }
    if litSize > 0 && dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if litSize > blockSizeMax {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if singleStream == 0 {
        if litSize < 6 {
            return -(ZSTD_error_literals_headerWrong as libc::c_int) as libc::size_t;
        }
    }
    if litCSize.wrapping_add(lhSize) > srcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if expectedWriteSize < litSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
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
    if (*dctx).ddictIsCold != 0 && litSize > 768 {
        let _ptr = (*dctx).HUFptr as *const libc::c_char;
        let _size = ::core::mem::size_of::<[HUF_DTable; 4097]>();
        let mut _pos: libc::size_t = 0;
        _pos = 0 as libc::c_int as libc::size_t;
        while _pos < _size {
            _pos = (_pos as libc::c_ulong).wrapping_add(CACHELINE_SIZE as libc::c_ulong)
                ;
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
            debug_assert!(litSize >= 6);
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
            ::core::mem::size_of::<[u32; 640]>(),
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
            ::core::mem::size_of::<[u32; 640]>(),
            flags,
        );
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        debug_assert!(litSize
            > (if 64 as libc::c_int
                > (if ((1) << 16 as libc::c_int)
                    < (128) << 10 as libc::c_int
                {
                    (1) << 16 as libc::c_int
                } else {
                    (128) << 10 as libc::c_int
                })
            {
                64 as libc::c_int
            } else {
                (if ((1) << 16 as libc::c_int)
                    < (128) << 10 as libc::c_int
                {
                    (1) << 16 as libc::c_int
                } else {
                    (128) << 10 as libc::c_int
                })
            }) as libc::c_ulong);
        libc::memcpy(
            ((*dctx).litExtraBuffer).as_mut_ptr() as *mut libc::c_void,
            ((*dctx).litBufferEnd)
                .offset(
                    -((if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    }) as isize),
                ) as *const libc::c_void,
            (if 64 as libc::c_int
                > (if ((1) << 16 as libc::c_int)
                    < (128) << 10 as libc::c_int
                {
                    (1) << 16 as libc::c_int
                } else {
                    (128) << 10 as libc::c_int
                })
            {
                64 as libc::c_int
            } else if ((1) << 16 as libc::c_int)
                < (128) << 10 as libc::c_int
            {
                (1) << 16 as libc::c_int
            } else {
                (128) << 10 as libc::c_int
            }) as libc::c_ulong as libc::size_t,
        );
        libc::memmove(
            ((*dctx).litBuffer)
                .offset(
                    (if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    }) as isize,
                )
                .offset(-(32)) as *mut libc::c_void,
            (*dctx).litBuffer as *const libc::c_void,
            litSize
                .wrapping_sub(
                    (if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    }) as libc::c_ulong,
                ) as libc::size_t,
        );
        (*dctx)
            .litBuffer = ((*dctx).litBuffer)
            .offset(
                ((if 64 as libc::c_int
                    > (if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
                    })
                {
                    64 as libc::c_int
                } else {
                    (if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
                    })
                }) - WILDCOPY_OVERLENGTH) as isize,
            );
        (*dctx)
            .litBufferEnd = ((*dctx).litBufferEnd)
            .offset(-(WILDCOPY_OVERLENGTH as isize));
        debug_assert!((*dctx).litBufferEnd
            <= (dst as *mut u8).offset(blockSizeMax as isize) as *const u8);
    }
    if ERR_isError(hufSuccess) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    (*dctx).litPtr = (*dctx).litBuffer;
    (*dctx).litSize = litSize;
    (*dctx).litEntropy = 1 as libc::c_int as u32;
    if litEncType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint {
        (*dctx).HUFptr = ((*dctx).entropy.hufTable).as_mut_ptr();
    }
    return litCSize.wrapping_add(lhSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeLiteralsBlock_wrapper(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
) -> libc::size_t {
    (*dctx).isFrameDecompression = 0 as libc::c_int;
    return ZSTD_decodeLiteralsBlock(dctx, src, srcSize, dst, dstCapacity, not_streaming);
}
static mut LL_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 1 as libc::c_int as u8,
            baseValue: LL_DEFAULTNORMLOG as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 0 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 0 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 1 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 3 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 4 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 6 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 7 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 9 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 10 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 12 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 14 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 16 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 20 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 22 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 28 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 32 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 4 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 48 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 6 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 64 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 7 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 128 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 8 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 256 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 10 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 1024 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 12 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 4096 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 0 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 1 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 2 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 4 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 5 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 7 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 8 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 10 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 11 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 13 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 16 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 18 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 22 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 24 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 32 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 40 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 6 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 64 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 6 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 64 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 7 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 128 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 9 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 512 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 11 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 2048 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 48 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 0 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 1 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 2 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 3 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 5 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 6 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 8 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 9 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 11 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 12 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 15 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 18 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 20 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 24 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 28 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 40 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 4 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 48 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 16 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 65536 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 15 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 32768 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 14 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 16384 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 13 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 8192 as libc::c_int as u32,
        };
        init
    },
];
static mut OF_defaultDTable: [ZSTD_seqSymbol; 33] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 1 as libc::c_int as u8,
            baseValue: OF_DEFAULTNORMLOG as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 0 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 6 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 61 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 9 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 509 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 15 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 32765 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 21 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 2097149 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 5 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 7 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 125 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 12 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 4093 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 18 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 262141 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 23 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 8388605 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 5 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 29 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 8 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 253 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 14 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 16381 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 20 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 1048573 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 1 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 7 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 125 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 11 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 2045 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 17 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 131069 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 22 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 4194301 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 4 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 13 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 8 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 253 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 13 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 8189 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 19 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 524285 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 1 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 6 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 61 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 10 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 1021 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 16 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 65533 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 28 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 268435453 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 27 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 134217725 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 26 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 67108861 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 25 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 33554429 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 24 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 16777213 as libc::c_int as u32,
        };
        init
    },
];
static mut ML_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 1 as libc::c_int as u8,
            baseValue: ML_DEFAULTNORMLOG as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 3 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 4 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 5 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 6 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 8 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 9 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 11 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 13 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 16 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 19 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 22 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 25 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 28 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 31 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 34 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 37 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 41 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 47 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 59 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 4 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 83 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 7 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 131 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 9 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 515 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 4 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 5 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 6 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 7 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 9 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 10 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 12 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 15 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 18 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 21 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 24 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 27 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 30 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 33 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 35 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 1 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 39 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 2 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 43 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 3 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 51 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 4 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 67 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 5 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 99 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 8 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 259 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 4 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 48 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 4 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 4 as libc::c_int as u8,
            baseValue: 5 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 7 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 8 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 10 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 5 as libc::c_int as u8,
            baseValue: 11 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 14 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 17 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 20 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 23 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 26 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 29 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 0 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 32 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 16 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 65539 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 15 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 32771 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 14 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 16387 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 13 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 8195 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 12 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 4099 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 11 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 2051 as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0 as libc::c_int as u16,
            nbAdditionalBits: 10 as libc::c_int as u8,
            nbBits: 6 as libc::c_int as u8,
            baseValue: 1027 as libc::c_int as u32,
        };
        init
    },
];
unsafe extern "C" fn ZSTD_buildSeqTable_rle(
    mut dt: *mut ZSTD_seqSymbol,
    mut baseValue: u32,
    mut nbAddBits: u8,
) {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut ZSTD_seqSymbol_header;
    let cell = dt.offset(1);
    (*DTableH).tableLog = 0 as libc::c_int as u32;
    (*DTableH).fastMode = 0 as libc::c_int as u32;
    (*cell).nbBits = 0 as libc::c_int as u8;
    (*cell).nextState = 0 as libc::c_int as u16;
    debug_assert!((nbAddBits as libc::c_int) < 255);
    (*cell).nbAdditionalBits = nbAddBits;
    (*cell).baseValue = baseValue;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_buildFSETable_body(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: libc::size_t,
) {
    let tableDecode = dt.offset(1);
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut symbolNext = wksp as *mut u16;
    let mut spread = symbolNext
        .offset(
            (if 35 as libc::c_int > 52 {
                35 as libc::c_int
            } else {
                52 as libc::c_int
            }) as isize,
        )
        .offset(1) as *mut u8;
    let mut highThreshold = tableSize.wrapping_sub(1);
    debug_assert!(maxSymbolValue
        <= (if 35 as libc::c_int > 52 {
            35 as libc::c_int
        } else {
            52 as libc::c_int
        }) as libc::c_uint);
    debug_assert!(tableLog
        <= (if (if 9 as libc::c_int > 9 {
            9 as libc::c_int
        } else {
            9 as libc::c_int
        }) > 8
        {
            (if 9 as libc::c_int > 9 {
                9 as libc::c_int
            } else {
                9 as libc::c_int
            })
        } else {
            8 as libc::c_int
        }) as libc::c_uint);
    debug_assert!(wkspSize
        >= (::core::mem::size_of::<i16>())
            .wrapping_mul(
                ((if 35 as libc::c_int > 52 {
                    35 as libc::c_int
                } else {
                    52 as libc::c_int
                }) + 1) as libc::c_ulong,
            )
            .wrapping_add(
                ((1)
                    << (if (if 9 as libc::c_int > 9 {
                        9 as libc::c_int
                    } else {
                        9 as libc::c_int
                    }) > 8
                    {
                        (if 9 as libc::c_int > 9 {
                            9 as libc::c_int
                        } else {
                            9 as libc::c_int
                        })
                    } else {
                        8 as libc::c_int
                    })) as libc::c_ulong,
            )
            .wrapping_add(::core::mem::size_of::<u64>()));
    let mut DTableH = ZSTD_seqSymbol_header {
        fastMode: 0,
        tableLog: 0,
    };
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1 as libc::c_int as u32;
    let largeLimit = ((1)
        << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0 as libc::c_int as u32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -(1) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).baseValue = s;
            *symbolNext.offset(s as isize) = 1 as libc::c_int as u16;
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int
                >= largeLimit as libc::c_int
            {
                DTableH.fastMode = 0 as libc::c_int as u32;
            }
            debug_assert!(*normalizedCounter.offset(s as isize) as libc::c_int >= 0);
            *symbolNext
                .offset(s as isize) = *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
    }
    libc::memcpy(
        dt as *mut libc::c_void,
        &mut DTableH as *mut ZSTD_seqSymbol_header as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_seqSymbol_header>() as libc::size_t,
    );
    debug_assert!(tableSize <= 512);
    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1)
            as libc::size_t;
        let step = (tableSize >> 1 as libc::c_int)
            .wrapping_add(tableSize >> 3 as libc::c_int)
            .wrapping_add(3) as libc::size_t;
        let add = 0x101010101010101 as libc::c_ulonglong as u64;
        let mut pos = 0 as libc::c_int as libc::size_t;
        let mut sv = 0 as libc::c_int as u64;
        let mut s_0: u32 = 0;
        s_0 = 0 as libc::c_int as u32;
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
            debug_assert!(n >= 0);
            pos = (pos as libc::c_ulong).wrapping_add(n as libc::size_t) ;
            s_0 = s_0.wrapping_add(1);
            sv = (sv as libc::c_ulong).wrapping_add(add) ;
        }
        let mut position = 0 as libc::c_int as libc::size_t;
        let mut s_1: libc::size_t = 0;
        let unroll = 2 as libc::c_int as libc::size_t;
        debug_assert!((tableSize as libc::c_ulong).wrapping_rem(unroll)
            == 0);
        s_1 = 0 as libc::c_int as libc::size_t;
        while s_1 < tableSize as libc::size_t {
            let mut u: libc::size_t = 0;
            u = 0 as libc::c_int as libc::size_t;
            while u < unroll {
                let uPosition = position.wrapping_add(u.wrapping_mul(step)) & tableMask;
                (*tableDecode.offset(uPosition as isize))
                    .baseValue = *spread.offset(s_1.wrapping_add(u) as isize) as u32;
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll.wrapping_mul(step)) & tableMask;
            s_1 = (s_1 as libc::c_ulong).wrapping_add(unroll) ;
        }
        debug_assert!(position == 0);
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = (tableSize >> 1 as libc::c_int)
            .wrapping_add(tableSize >> 3 as libc::c_int)
            .wrapping_add(3);
        let mut s_2: u32 = 0;
        let mut position_0 = 0 as libc::c_int as u32;
        s_2 = 0 as libc::c_int as u32;
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
        debug_assert!(position_0 == 0);
    }
    let mut u_0: u32 = 0;
    u_0 = 0 as libc::c_int as u32;
    while u_0 < tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).baseValue;
        let ref mut fresh1 = *symbolNext.offset(symbol as isize);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as u32;
        (*tableDecode.offset(u_0 as isize))
            .nbBits = tableLog.wrapping_sub(ZSTD_highbit32(nextState)) as u8;
        (*tableDecode.offset(u_0 as isize))
            .nextState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as libc::c_int)
            .wrapping_sub(tableSize) as u16;
        debug_assert!((*nbAdditionalBits.offset(symbol as isize) as libc::c_int)
            < 255);
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
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: libc::size_t,
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
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: libc::size_t,
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
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: libc::c_uint,
    mut wksp: *mut libc::c_void,
    mut wkspSize: libc::size_t,
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
    mut maxLog: u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut defaultTable: *const ZSTD_seqSymbol,
    mut flagRepeatTable: u32,
    mut ddictIsCold: libc::c_int,
    mut nbSeq: libc::c_int,
    mut wksp: *mut u32,
    mut wkspSize: libc::size_t,
    mut bmi2: libc::c_int,
) -> libc::size_t {
    match type_0 as libc::c_uint {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            if *(src as *const u8) as libc::c_uint > max {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            let symbol = *(src as *const u8) as u32;
            let baseline = *baseValue.offset(symbol as isize);
            let nbBits = *nbAdditionalBits.offset(symbol as isize);
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);
            *DTablePtr = DTableSpace;
            return 1 as libc::c_int as libc::size_t;
        }
        0 => {
            *DTablePtr = defaultTable;
            return 0 as libc::c_int as libc::size_t;
        }
        3 => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
            }
            if ddictIsCold != 0 && nbSeq > 24 {
                let pStart = *DTablePtr as *const libc::c_void;
                let pSize = (::core::mem::size_of::<ZSTD_seqSymbol>())
                    .wrapping_mul(
                        (1 as libc::c_int + ((1) << maxLog))
                            as libc::c_ulong,
                    );
                let _ptr = pStart as *const libc::c_char;
                let _size = pSize;
                let mut _pos: libc::size_t = 0;
                _pos = 0 as libc::c_int as libc::size_t;
                while _pos < _size {
                    _pos = (_pos as libc::c_ulong)
                        .wrapping_add(CACHELINE_SIZE as libc::c_ulong) as libc::size_t
                        as libc::size_t;
                }
            }
            return 0 as libc::c_int as libc::size_t;
        }
        2 => {
            let mut tableLog: libc::c_uint = 0;
            let mut norm: [i16; 53] = [0; 53];
            let headerSize = FSE_readNCount(
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
            debug_assert!(false);
            return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeSeqHeaders(
    mut dctx: *mut ZSTD_DCtx,
    mut nbSeqPtr: *mut libc::c_int,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    let mut nbSeq: libc::c_int = 0;
    if srcSize < 1 {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let fresh3 = ip;
    ip = ip.offset(1);
    nbSeq = *fresh3 as libc::c_int;
    if nbSeq == 0 {
        *nbSeqPtr = 0 as libc::c_int;
        if srcSize != 1 as libc::c_int as libc::c_ulong {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
        }
        return 1 as libc::c_int as libc::size_t;
    }
    if nbSeq > 0x7f as libc::c_int {
        if nbSeq == 0xff as libc::c_int {
            if ip.offset(2) > iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            nbSeq = MEM_readLE16(ip as *const libc::c_void) as libc::c_int + LONGNBSEQ;
            ip = ip.offset(2);
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
            }
            let fresh4 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as libc::c_int) << 8 as libc::c_int)
                + *fresh4 as libc::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(1) > iend {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    let LLtype = (*ip as libc::c_int >> 6 as libc::c_int) as symbolEncodingType_e;
    let OFtype = (*ip as libc::c_int >> 4 as libc::c_int & 3)
        as symbolEncodingType_e;
    let MLtype = (*ip as libc::c_int >> 2 as libc::c_int & 3)
        as symbolEncodingType_e;
    ip = ip.offset(1);
    let llhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.LLTable).as_mut_ptr(),
        &mut (*dctx).LLTptr,
        LLtype,
        MaxLL as libc::c_uint,
        LLFSELog as u32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        LL_base.as_ptr(),
        LL_bits.as_ptr(),
        LL_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; 640]>(),
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(llhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(llhSize as isize);
    let ofhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.OFTable).as_mut_ptr(),
        &mut (*dctx).OFTptr,
        OFtype,
        MaxOff as libc::c_uint,
        OffFSELog as u32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        OF_base.as_ptr(),
        OF_bits.as_ptr(),
        OF_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; 640]>(),
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(ofhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(ofhSize as isize);
    let mlhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.MLTable).as_mut_ptr(),
        &mut (*dctx).MLTptr,
        MLtype,
        MaxML as libc::c_uint,
        MLFSELog as u32,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        ML_base.as_ptr(),
        ML_bits.as_ptr(),
        ML_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; 640]>(),
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(mlhSize) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    ip = ip.offset(mlhSize as isize);
    return ip.offset_from(istart) as libc::c_long as libc::size_t;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_overlapCopy8(
    mut op: *mut *mut u8,
    mut ip: *mut *const u8,
    mut offset: libc::size_t,
) {
    debug_assert!(*ip <= *op as *const u8);
    if offset < 8 {
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
        let sub2 = dec64table[offset as usize];
        *(*op)
            .offset(
                0 as libc::c_int as isize,
            ) = *(*ip).offset(0);
        *(*op)
            .offset(
                1 as libc::c_int as isize,
            ) = *(*ip).offset(1);
        *(*op)
            .offset(
                2 as libc::c_int as isize,
            ) = *(*ip).offset(2);
        *(*op)
            .offset(
                3 as libc::c_int as isize,
            ) = *(*ip).offset(3);
        *ip = (*ip).offset(dec32table[offset as usize] as isize);
        ZSTD_copy4(
            (*op).offset(4) as *mut libc::c_void,
            *ip as *const libc::c_void,
        );
        *ip = (*ip).offset(-(sub2 as isize));
    } else {
        ZSTD_copy8(*op as *mut libc::c_void, *ip as *const libc::c_void);
    }
    *ip = (*ip).offset(8);
    *op = (*op).offset(8);
    debug_assert!((*op).offset_from(*ip) as libc::c_long >= 8);;
}
unsafe extern "C" fn ZSTD_safecopy(
    mut op: *mut u8,
    oend_w: *const u8,
    mut ip: *const u8,
    mut length: ptrdiff_t,
    mut ovtype: ZSTD_overlap_e,
) {
    let diff = op.offset_from(ip) as libc::c_long;
    let oend = op.offset(length as isize);
    debug_assert!(ovtype as libc::c_uint == ZSTD_no_overlap as libc::c_int as libc::c_uint
        && (diff <= -(8) as libc::c_long
            || diff >= 8 || op >= oend_w as *mut u8)
        || ovtype as libc::c_uint
            == ZSTD_overlap_src_before_dst as libc::c_int as libc::c_uint
            && diff >= 0);
    if length < 8 {
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
        debug_assert!(length >= 8);
        ZSTD_overlapCopy8(&mut op, &mut ip, diff as libc::size_t);
        length -= 8 as libc::c_int as libc::c_long;
        debug_assert!(op.offset_from(ip) as libc::c_long >= 8);
        debug_assert!(op <= oend);
    }
    if oend <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            ip as *const libc::c_void,
            length,
            ovtype,
        );
        return;
    }
    if op <= oend_w as *mut u8 {
        debug_assert!(oend > oend_w as *mut u8);
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
    mut op: *mut u8,
    mut ip: *const u8,
    mut length: ptrdiff_t,
) {
    let diff = op.offset_from(ip) as libc::c_long;
    let oend = op.offset(length as isize);
    if length < 8
        || diff > -(8) as libc::c_long
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
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> libc::size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    if sequenceLength > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as libc::size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    debug_assert!(op < op.offset(sequenceLength as isize));
    debug_assert!(oLitEnd < op.offset(sequenceLength as isize));
    ZSTD_safecopy(op, oend_w, *litPtr, sequence.litLength as ptrdiff_t, ZSTD_no_overlap);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as libc::size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as libc::c_long as libc::size_t
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as libc::size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            ;
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
    mut op: *mut u8,
    oend: *mut u8,
    oend_w: *const u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> libc::size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    if sequenceLength > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as libc::size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    debug_assert!(op < op.offset(sequenceLength as isize));
    debug_assert!(oLitEnd < op.offset(sequenceLength as isize));
    if op > *litPtr as *mut u8
        && op < (*litPtr).offset(sequence.litLength as isize) as *mut u8
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    ZSTD_safecopyDstBeforeSrc(op, *litPtr, sequence.litLength as ptrdiff_t);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as libc::size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as libc::c_long as libc::size_t
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as libc::size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            ;
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
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> libc::size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    debug_assert!(!op.is_null());
    debug_assert!(oend_w < oend);
    if (iLitEnd > litLimit || oMatchEnd > oend_w
        || MEM_32bits() != 0
            && (oend.offset_from(op) as libc::c_long as libc::size_t)
                < sequenceLength.wrapping_add(32))
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
    debug_assert!(op <= oLitEnd);
    debug_assert!(oLitEnd < oMatchEnd);
    debug_assert!(oMatchEnd <= oend);
    debug_assert!(iLitEnd <= litLimit);
    debug_assert!(oLitEnd <= oend_w);
    debug_assert!(oMatchEnd <= oend_w);
    debug_assert!(32 as libc::c_int >= 16);
    ZSTD_copy16(op as *mut libc::c_void, *litPtr as *const libc::c_void);
    if (sequence.litLength > 16) as libc::c_int
        as libc::c_long != 0
    {
        ZSTD_wildcopy(
            op.offset(16) as *mut libc::c_void,
            (*litPtr).offset(16) as *const libc::c_void,
            (sequence.litLength).wrapping_sub(16)
                as ptrdiff_t,
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as libc::size_t {
        if (sequence.offset
            > oLitEnd.offset_from(virtualStart) as libc::c_long as libc::size_t) as libc::c_int
            as libc::c_long != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as libc::size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            ;
        match_0 = prefixStart;
    }
    debug_assert!(op <= oMatchEnd);
    debug_assert!(oMatchEnd <= oend_w);
    debug_assert!(match_0 >= prefixStart);
    debug_assert!(sequence.matchLength >= 1);
    if (sequence.offset >= 16) as libc::c_int
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
    debug_assert!(sequence.offset < 16);
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        debug_assert!(op < oMatchEnd);
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
    mut op: *mut u8,
    oend: *mut u8,
    oend_w: *const u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> libc::size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    debug_assert!(!op.is_null());
    debug_assert!(oend_w < oend as *const u8);
    if (iLitEnd > litLimit || oMatchEnd > oend_w as *mut u8
        || MEM_32bits() != 0
            && (oend.offset_from(op) as libc::c_long as libc::size_t)
                < sequenceLength.wrapping_add(32))
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
    debug_assert!(op <= oLitEnd);
    debug_assert!(oLitEnd < oMatchEnd);
    debug_assert!(oMatchEnd <= oend);
    debug_assert!(iLitEnd <= litLimit);
    debug_assert!(oLitEnd <= oend_w as *mut u8);
    debug_assert!(oMatchEnd <= oend_w as *mut u8);
    debug_assert!(32 as libc::c_int >= 16);
    ZSTD_copy16(op as *mut libc::c_void, *litPtr as *const libc::c_void);
    if (sequence.litLength > 16) as libc::c_int
        as libc::c_long != 0
    {
        ZSTD_wildcopy(
            op.offset(16) as *mut libc::c_void,
            (*litPtr).offset(16) as *const libc::c_void,
            (sequence.litLength).wrapping_sub(16)
                as ptrdiff_t,
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as libc::c_long as libc::size_t {
        if (sequence.offset
            > oLitEnd.offset_from(virtualStart) as libc::c_long as libc::size_t) as libc::c_int
            as libc::c_long != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
        let length1 = dictEnd.offset_from(match_0) as libc::c_long as libc::size_t;
        libc::memmove(
            oLitEnd as *mut libc::c_void,
            match_0 as *const libc::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence
            .matchLength = (sequence.matchLength as libc::c_ulong).wrapping_sub(length1)
            ;
        match_0 = prefixStart;
    }
    debug_assert!(op <= oMatchEnd);
    debug_assert!(oMatchEnd <= oend_w as *mut u8);
    debug_assert!(match_0 >= prefixStart);
    debug_assert!(sequence.matchLength >= 1);
    if (sequence.offset >= 16) as libc::c_int
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
    debug_assert!(sequence.offset < 16);
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        debug_assert!(op < oMatchEnd);
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
    (*DStatePtr).table = dt.offset(1);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_updateFseStateWithDInfo(
    mut DStatePtr: *mut ZSTD_fseState,
    mut bitD: *mut BIT_DStream_t,
    mut nextState: u16,
    mut nbBits: u32,
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
    seq.matchLength = (*mlDInfo).baseValue as libc::size_t;
    seq.litLength = (*llDInfo).baseValue as libc::size_t;
    let ofBase = (*ofDInfo).baseValue;
    let llBits = (*llDInfo).nbAdditionalBits;
    let mlBits = (*mlDInfo).nbAdditionalBits;
    let ofBits = (*ofDInfo).nbAdditionalBits;
    let totalBits = (llBits as libc::c_int + mlBits as libc::c_int
        + ofBits as libc::c_int) as u8;
    let llNext = (*llDInfo).nextState;
    let mlNext = (*mlDInfo).nextState;
    let ofNext = (*ofDInfo).nextState;
    let llnbBits = (*llDInfo).nbBits as u32;
    let mlnbBits = (*mlDInfo).nbBits as u32;
    let ofnbBits = (*ofDInfo).nbBits as u32;
    debug_assert!(llBits as libc::c_int <= 16);
    debug_assert!(mlBits as libc::c_int <= 16);
    debug_assert!(ofBits as libc::c_int <= 31);
    let mut offset: libc::size_t = 0;
    if ofBits as libc::c_int > 1 {
        if MEM_32bits() != 0 && longOffsets as libc::c_uint != 0
            && ofBits as libc::c_int >= STREAM_ACCUMULATOR_MIN_32
        {
            let extraBits = (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
            } else {
                0 as libc::c_int
            }) as u32;
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
                ;
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
        let ll0 = ((*llDInfo).baseValue == 0)
            as libc::c_int as u32;
        if (ofBits as libc::c_int == 0) as libc::c_int as libc::c_long
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
            let mut temp = if offset == 3 {
                ((*seqState).prevOffset[0 as libc::c_int as usize])
                    .wrapping_sub(1)
            } else {
                (*seqState).prevOffset[offset as usize]
            };
            temp = (temp as libc::c_ulong)
                .wrapping_add((temp == 0) as libc::c_int as libc::c_ulong) as libc::size_t
                as libc::size_t;
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
    if mlBits as libc::c_int > 0 {
        seq
            .matchLength = (seq.matchLength as libc::c_ulong)
            .wrapping_add(
                BIT_readBitsFast(&mut (*seqState).DStream, mlBits as libc::c_uint),
            ) ;
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
            >= 57
                - (9 as libc::c_int + 9 + 8))
            as libc::c_int as libc::c_long != 0
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if llBits as libc::c_int > 0 {
        seq
            .litLength = (seq.litLength as libc::c_ulong)
            .wrapping_add(
                BIT_readBitsFast(&mut (*seqState).DStream, llBits as libc::c_uint),
            ) ;
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(maxDstSize as isize);
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let mut litBufferEnd = (*dctx).litBufferEnd;
    let prefixStart = (*dctx).prefixStart as *const u8;
    let vBase = (*dctx).virtualStart as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
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
        (*dctx).fseEntropy = 1 as libc::c_int as u32;
        let mut i: u32 = 0;
        i = 0 as libc::c_int as u32;
        while i < ZSTD_REP_NUM as libc::c_uint {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as libc::size_t;
            i = i.wrapping_add(1);
        }
        if ERR_isError(
            BIT_initDStream(
                &mut seqState.DStream,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as libc::size_t,
            ),
        ) != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        debug_assert!(!dst.is_null());
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
        if nbSeq > 0 {
            let leftoverLit = ((*dctx).litBufferEnd).offset_from(litPtr) as libc::c_long
                as libc::size_t;
            if leftoverLit != 0 {
                if leftoverLit > oend.offset_from(op) as libc::c_long as libc::size_t {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
                }
                ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit as ptrdiff_t);
                sequence
                    .litLength = (sequence.litLength as libc::c_ulong)
                    .wrapping_sub(leftoverLit) ;
                op = op.offset(leftoverLit as isize);
            }
            litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
            litBufferEnd = ((*dctx).litExtraBuffer)
                .as_mut_ptr()
                .offset(
                    (if 64 as libc::c_int
                        > (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
                        })
                    {
                        64 as libc::c_int
                    } else {
                        (if ((1) << 16 as libc::c_int)
                            < (128) << 10 as libc::c_int
                        {
                            (1) << 16 as libc::c_int
                        } else {
                            (128) << 10 as libc::c_int
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
        if nbSeq > 0 {
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
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        if (BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint)
            < BIT_DStream_completed as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        let mut i_0: u32 = 0;
        i_0 = 0 as libc::c_int as u32;
        while i_0 < ZSTD_REP_NUM as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as libc::c_long as libc::size_t;
        if lastLLSize > oend.offset_from(op) as libc::c_long as libc::size_t {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
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
                    > (if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
                    })
                {
                    64 as libc::c_int
                } else {
                    (if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
                    })
                }) as isize,
            );
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as libc::c_long as libc::size_t;
    if lastLLSize_0 > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut libc::c_void,
            litPtr as *const libc::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut u8;
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
    let prefixStart = (*dctx).prefixStart as *const u8;
    let vBase = (*dctx).virtualStart as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
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
        (*dctx).fseEntropy = 1 as libc::c_int as u32;
        let mut i: u32 = 0;
        i = 0 as libc::c_int as u32;
        while i < ZSTD_REP_NUM as libc::c_uint {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as libc::size_t;
            i = i.wrapping_add(1);
        }
        if ERR_isError(
            BIT_initDStream(
                &mut seqState.DStream,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as libc::size_t,
            ),
        ) != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        debug_assert!(!dst.is_null());
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
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        if (BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint)
            < BIT_DStream_completed as libc::c_int as libc::c_uint
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        let mut i_0: u32 = 0;
        i_0 = 0 as libc::c_int as u32;
        while i_0 < ZSTD_REP_NUM as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as libc::c_long as libc::size_t;
    if lastLLSize > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut libc::c_void,
            litPtr as *const libc::c_void,
            lastLLSize as libc::size_t,
        );
        op = op.offset(lastLLSize as isize);
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn ZSTD_decompressSequences_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut prefetchPos: libc::size_t,
    sequence: seq_t,
    prefixStart: *const u8,
    dictEnd: *const u8,
) -> libc::size_t {
    prefetchPos = (prefetchPos as libc::c_ulong).wrapping_add(sequence.litLength)
        ;
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut u8;
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
    let prefixStart = (*dctx).prefixStart as *const u8;
    let dictStart = (*dctx).virtualStart as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    if nbSeq != 0 {
        let mut sequences: [seq_t; 8] = [seq_t {
            litLength: 0,
            matchLength: 0,
            offset: 0,
        }; 8];
        let seqAdvance = if nbSeq < 8 { nbSeq } else { 8 as libc::c_int };
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
        let mut prefetchPos = op.offset_from(prefixStart) as libc::c_long as libc::size_t;
        (*dctx).fseEntropy = 1 as libc::c_int as u32;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < ZSTD_REP_NUM {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as libc::size_t;
            i += 1;
        }
        debug_assert!(!dst.is_null());
        debug_assert!(iend >= ip);
        if ERR_isError(
            BIT_initDStream(
                &mut seqState.DStream,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as libc::size_t,
            ),
        ) != 0
        {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint
            <= BIT_DStream_completed as libc::c_int as libc::c_uint && seqNb < nbSeq
        {
            let mut sequence_0 = ZSTD_decodeSequence(&mut seqState, isLongOffset);
            let mut oneSeqSize: libc::size_t = 0;
            if (*dctx).litBufferLocation as libc::c_uint
                == ZSTD_split as libc::c_int as libc::c_uint
                && litPtr
                    .offset(
                        sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize]
                            .litLength as isize,
                    ) > (*dctx).litBufferEnd
            {
                let leftoverLit = ((*dctx).litBufferEnd).offset_from(litPtr)
                    as libc::c_long as libc::size_t;
                if leftoverLit != 0 {
                    if leftoverLit > oend.offset_from(op) as libc::c_long as libc::size_t {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit as ptrdiff_t);
                    sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize]
                        .litLength = (sequences[(seqNb - ADVANCED_SEQS
                            & STORED_SEQS_MASK) as usize]
                        .litLength as libc::c_ulong)
                        .wrapping_sub(leftoverLit) ;
                    op = op.offset(leftoverLit as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer)
                    .as_mut_ptr()
                    .offset(
                        (if 64 as libc::c_int
                            > (if ((1) << 16 as libc::c_int)
                                < (128) << 10 as libc::c_int
                            {
                                (1) << 16 as libc::c_int
                            } else {
                                (128) << 10 as libc::c_int
                            })
                        {
                            64 as libc::c_int
                        } else {
                            (if ((1) << 16 as libc::c_int)
                                < (128) << 10 as libc::c_int
                            {
                                (1) << 16 as libc::c_int
                            } else {
                                (128) << 10 as libc::c_int
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
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
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
                    as libc::c_long as libc::size_t;
                if leftoverLit_0 != 0 {
                    if leftoverLit_0 > oend.offset_from(op) as libc::c_long as libc::size_t {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit_0 as ptrdiff_t);
                    (*sequence_1)
                        .litLength = ((*sequence_1).litLength as libc::c_ulong)
                        .wrapping_sub(leftoverLit_0) ;
                    op = op.offset(leftoverLit_0 as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer)
                    .as_mut_ptr()
                    .offset(
                        (if 64 as libc::c_int
                            > (if ((1) << 16 as libc::c_int)
                                < (128) << 10 as libc::c_int
                            {
                                (1) << 16 as libc::c_int
                            } else {
                                (128) << 10 as libc::c_int
                            })
                        {
                            64 as libc::c_int
                        } else {
                            (if ((1) << 16 as libc::c_int)
                                < (128) << 10 as libc::c_int
                            {
                                (1) << 16 as libc::c_int
                            } else {
                                (128) << 10 as libc::c_int
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
        let mut i_0: u32 = 0;
        i_0 = 0 as libc::c_int as u32;
        while i_0 < ZSTD_REP_NUM as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*dctx).litBufferLocation as libc::c_uint
        == ZSTD_split as libc::c_int as libc::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as libc::c_long as libc::size_t;
        if lastLLSize > oend.offset_from(op) as libc::c_long as libc::size_t {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
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
                    > (if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
                    })
                {
                    64 as libc::c_int
                } else {
                    (if ((1) << 16 as libc::c_int)
                        < (128) << 10 as libc::c_int
                    {
                        (1) << 16 as libc::c_int
                    } else {
                        (128) << 10 as libc::c_int
                    })
                }) as isize,
            );
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as libc::c_long as libc::size_t;
    if lastLLSize_0 > oend.offset_from(op) as libc::c_long as libc::size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if !op.is_null() {
        libc::memmove(
            op as *mut libc::c_void,
            litPtr as *const libc::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
pub const STORED_SEQS: libc::c_int = 8 as libc::c_int;
pub const STORED_SEQS_MASK: libc::c_int = STORED_SEQS - 1 as libc::c_int;
pub const ADVANCED_SEQS: libc::c_int = STORED_SEQS;
unsafe extern "C" fn ZSTD_decompressSequencesLong_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut maxDstSize: libc::size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: libc::size_t,
    mut nbSeq: libc::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> libc::size_t {
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
    mut op: *mut u8,
    mut virtualStart: *const u8,
) -> libc::size_t {
    return op.offset_from(virtualStart) as libc::c_long as libc::size_t;
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
            .offset(0))
            .tableLog;
        let mut table = offTable.offset(1);
        let max = ((1) << tableLog) as u32;
        let mut u: u32 = 0;
        debug_assert!(max <= ((1) << 8 as libc::c_int) as libc::c_uint);
        u = 0 as libc::c_int as u32;
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
                > 22
            {
                info
                    .longOffsetShare = (info.longOffsetShare)
                    .wrapping_add(1);
            }
            u = u.wrapping_add(1);
        }
        debug_assert!(tableLog <= 8);
        info.longOffsetShare <<= (OffFSELog as libc::c_uint).wrapping_sub(tableLog);
    }
    return info;
}
unsafe extern "C" fn ZSTD_maxShortOffset() -> libc::size_t {
    if MEM_64bits() != 0 {
        return -(1) as libc::size_t
    } else {
        let maxOffbase = ((1)
            << ((if MEM_32bits() != 0 {
                STREAM_ACCUMULATOR_MIN_32
            } else {
                STREAM_ACCUMULATOR_MIN_64
            }) as u32)
                .wrapping_add(1))
            .wrapping_sub(1);
        let maxOffset = maxOffbase.wrapping_sub(ZSTD_REP_NUM as libc::c_ulong);
        debug_assert!(ZSTD_highbit32(maxOffbase as u32)
            == (if MEM_32bits() != 0 { 25 as libc::c_int } else { 57 as libc::c_int })
                as u32);
        return maxOffset;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock_internal(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    streaming: streaming_operation,
) -> libc::size_t {
    let mut ip = src as *const u8;
    if srcSize > ZSTD_blockSizeMax(dctx) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
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
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(litCSize) ;
    let blockSizeMax = if dstCapacity < ZSTD_blockSizeMax(dctx) {
        dstCapacity
    } else {
        ZSTD_blockSizeMax(dctx)
    };
    let totalHistorySize = ZSTD_totalHistorySize(
        (dst as *mut u8).offset(blockSizeMax as isize),
        (*dctx).virtualStart as *const u8,
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
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(seqHSize) ;
    if (dst.is_null() || dstCapacity == 0)
        && nbSeq > 0
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if MEM_64bits() != 0
        && ::core::mem::size_of::<libc::size_t>()
            == ::core::mem::size_of::<*mut libc::c_void>()
        && (-(1) as libc::size_t).wrapping_sub(dst as libc::size_t)
            < ((1) << 20 as libc::c_int) as libc::size_t
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    if isLongOffset as libc::c_uint != 0
        || usePrefetchDecoder == 0
            && totalHistorySize
                > ((1) << 24 as libc::c_int) as libc::c_ulong
            && nbSeq > 8
    {
        let info = ZSTD_getOffsetInfo((*dctx).OFTptr, nbSeq);
        if isLongOffset as libc::c_uint != 0
            && info.maxNbAdditionalBits
                <= (if MEM_32bits() != 0 {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as u32
        {
            isLongOffset = ZSTD_lo_isRegularOffset;
        }
        if usePrefetchDecoder == 0 {
            let minShare = (if MEM_64bits() != 0 {
                7 as libc::c_int
            } else {
                20 as libc::c_int
            }) as u32;
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
    mut dstSize: libc::size_t,
) {
    if dst != (*dctx).previousDstEnd && dstSize > 0 {
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
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut dSize: libc::size_t = 0;
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
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    return ZSTD_decompressBlock_deprecated(dctx, dst, dstCapacity, src, srcSize);
}
