use crate::__m128i_u;
use crate::__m128i_u;
use ::libc;
use crate::common::fse_h::*;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use core::arch::asm;
use std::ptr::addr_of_mut;
extern "C" {
    pub type ZSTD_DDict_s;
    fn FSE_readNCount(
        normalizedCounter: *mut std::ffi::c_short,
        maxSymbolValuePtr: *mut std::ffi::c_uint,
        tableLogPtr: *mut std::ffi::c_uint,
        rBuffer: *const std::ffi::c_void,
        rBuffSize: usize,
    ) -> usize;
    fn HUF_decompress1X_usingDTable(
        dst: *mut std::ffi::c_void,
        maxDstSize: usize,
        cSrc: *const std::ffi::c_void,
        cSrcSize: usize,
        DTable: *const HUF_DTable,
        flags: std::ffi::c_int,
    ) -> usize;
    fn HUF_decompress1X1_DCtx_wksp(
        dctx: *mut HUF_DTable,
        dst: *mut std::ffi::c_void,
        dstSize: usize,
        cSrc: *const std::ffi::c_void,
        cSrcSize: usize,
        workSpace: *mut std::ffi::c_void,
        wkspSize: usize,
        flags: std::ffi::c_int,
    ) -> usize;
    fn HUF_decompress4X_usingDTable(
        dst: *mut std::ffi::c_void,
        maxDstSize: usize,
        cSrc: *const std::ffi::c_void,
        cSrcSize: usize,
        DTable: *const HUF_DTable,
        flags: std::ffi::c_int,
    ) -> usize;
    fn HUF_decompress4X_hufOnly_wksp(
        dctx: *mut HUF_DTable,
        dst: *mut std::ffi::c_void,
        dstSize: usize,
        cSrc: *const std::ffi::c_void,
        cSrcSize: usize,
        workSpace: *mut std::ffi::c_void,
        wkspSize: usize,
        flags: std::ffi::c_int,
    ) -> usize;
}
pub type ptrdiff_t = std::ffi::c_long;
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
use crate::common::error::*;
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

const ZSTD_DECODER_INTERNAL_BUFFER: usize = 1 << 16; // TODO: configurable?

const ZSTD_LBMIN: usize = 64;
const ZSTD_LBMAX: usize = 128_usize << 10;

/* extra buffer, compensates when dst is not large enough to store litBuffer */
const ZSTD_LITBUFFEREXTRASIZE: usize = ZSTD_DECODER_INTERNAL_BUFFER.clamp(ZSTD_LBMIN, ZSTD_LBMAX);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [u32; 640],
    pub previousDstEnd: *const std::ffi::c_void,
    pub prefixStart: *const std::ffi::c_void,
    pub virtualStart: *const std::ffi::c_void,
    pub dictEnd: *const std::ffi::c_void,
    pub expected: usize,
    pub fParams: ZSTD_FrameHeader,
    pub processedCSize: u64,
    pub decodedSize: u64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: usize,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTD_customMem,
    pub litSize: usize,
    pub rleSize: usize,
    pub staticSize: usize,
    pub isFrameDecompression: std::ffi::c_int,
    pub bmi2: std::ffi::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: u32,
    pub ddictIsCold: std::ffi::c_int,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: std::ffi::c_int,
    pub maxBlockSizeParam: std::ffi::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: usize,
    pub inPos: usize,
    pub maxWindowSize: usize,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: usize,
    pub outStart: usize,
    pub outEnd: usize,
    pub lhSize: usize,
    pub legacyContext: *mut std::ffi::c_void,
    pub previousLegacyVersion: u32,
    pub legacyVersion: u32,
    pub hostageByte: u32,
    pub noForwardProgress: std::ffi::c_int,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut u8,
    pub litBufferEnd: *const u8,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [u8; 65568],
    pub headerBuffer: [u8; 18],
    pub oversizedDuration: usize,
    pub traceCtx: ZSTD_TraceCtx,
}
pub type ZSTD_TraceCtx = std::ffi::c_ulonglong;
pub type ZSTD_litLocation_e = std::ffi::c_uint;
pub const ZSTD_split: ZSTD_litLocation_e = 2;
pub const ZSTD_in_dst: ZSTD_litLocation_e = 1;
pub const ZSTD_not_in_dst: ZSTD_litLocation_e = 0;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut std::ffi::c_void,
    pub size: usize,
    pub pos: usize,
}
pub type ZSTD_bufferMode_e = std::ffi::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
pub type ZSTD_dStreamStage = std::ffi::c_uint;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const zdss_read: ZSTD_dStreamStage = 2;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const zdss_init: ZSTD_dStreamStage = 0;
pub type ZSTD_refMultipleDDicts_e = std::ffi::c_uint;
pub const ZSTD_rmd_refMultipleDDicts: ZSTD_refMultipleDDicts_e = 1;
pub const ZSTD_rmd_refSingleDDict: ZSTD_refMultipleDDicts_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDictHashSet {
    pub ddictPtrTable: *mut *const ZSTD_DDict,
    pub ddictPtrTableSize: usize,
    pub ddictPtrCount: usize,
}
pub type ZSTD_DDict = ZSTD_DDict_s;
pub type ZSTD_dictUses_e = std::ffi::c_int;
pub const ZSTD_use_once: ZSTD_dictUses_e = 1;
pub const ZSTD_dont_use: ZSTD_dictUses_e = 0;
pub const ZSTD_use_indefinitely: ZSTD_dictUses_e = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut std::ffi::c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void,
>;
pub type ZSTD_forceIgnoreChecksum_e = std::ffi::c_uint;
pub const ZSTD_d_ignoreChecksum: ZSTD_forceIgnoreChecksum_e = 1;
pub const ZSTD_d_validateChecksum: ZSTD_forceIgnoreChecksum_e = 0;
pub type ZSTD_format_e = std::ffi::c_uint;
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
pub type ZSTD_dStage = std::ffi::c_uint;
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;
pub type blockType_e = std::ffi::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_FrameHeader {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowSize: std::ffi::c_ulonglong,
    pub blockSizeMax: std::ffi::c_uint,
    pub frameType: ZSTD_FrameType_e,
    pub headerSize: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
    pub checksumFlag: std::ffi::c_uint,
    pub _reserved1: std::ffi::c_uint,
    pub _reserved2: std::ffi::c_uint,
}
pub type ZSTD_FrameType_e = std::ffi::c_uint;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;
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
pub type streaming_operation = std::ffi::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;
pub type ZSTD_longOffset_e = std::ffi::c_uint;
pub const ZSTD_lo_isLongOffset: ZSTD_longOffset_e = 1;
pub const ZSTD_lo_isRegularOffset: ZSTD_longOffset_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BIT_DStream_t,
    pub stateLL: ZSTD_fseState,
    pub stateOffb: ZSTD_fseState,
    pub stateML: ZSTD_fseState,
    pub prevOffset: [usize; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseState {
    pub state: usize,
    pub table: *const ZSTD_seqSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: usize,
    pub matchLength: usize,
    pub offset: usize,
}
pub type ZSTD_overlap_e = std::ffi::c_uint;
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
    pub longOffsetShare: std::ffi::c_uint,
    pub maxNbAdditionalBits: std::ffi::c_uint,
}
pub type SymbolEncodingType_e = std::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: u32,
    pub origSize: u32,
}
pub const CACHELINE_SIZE: std::ffi::c_int = 64;
#[inline]
unsafe extern "C" fn ZSTD_wrappedPtrAdd(
    mut ptr: *const std::ffi::c_void,
    mut add: ptrdiff_t,
) -> *const std::ffi::c_void {
    return (ptr as *const std::ffi::c_char).offset(add as isize)
        as *const std::ffi::c_void;
}
#[inline]
unsafe extern "C" fn ZSTD_wrappedPtrSub(
    mut ptr: *const std::ffi::c_void,
    mut sub: ptrdiff_t,
) -> *const std::ffi::c_void {
    return (ptr as *const std::ffi::c_char).offset(-(sub as isize))
        as *const std::ffi::c_void;
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
use crate::common::mem::*;
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const std::ffi::c_char,
    mut args: ...
) {}
use crate::common::bits::*;
pub const STREAM_ACCUMULATOR_MIN_32: std::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: std::ffi::c_int = 57;
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
        return ERROR(ZSTD_error_srcSize_wrong);
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
        RETURN_ERROR_IF!(lastByte as std::ffi::c_int == 0, ZSTD_error_GENERIC);
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
                current_block_32 = 3088805359668119436;
            }
            6 => {
                current_block_32 = 3088805359668119436;
            }
            5 => {
                current_block_32 = 470480923044399487;
            }
            4 => {
                current_block_32 = 9108891262973622346;
            }
            3 => {
                current_block_32 = 3706235558033962171;
            }
            2 => {
                current_block_32 = 3205284525055210370;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        match current_block_32 {
            3088805359668119436 => {
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
                current_block_32 = 470480923044399487;
            }
            _ => {}
        }
        match current_block_32 {
            470480923044399487 => {
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
                current_block_32 = 9108891262973622346;
            }
            _ => {}
        }
        match current_block_32 {
            9108891262973622346 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(3) as BitContainerType)
                            << 24,
                    );
                current_block_32 = 3706235558033962171;
            }
            _ => {}
        }
        match current_block_32 {
            3706235558033962171 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(2) as BitContainerType)
                            << 16,
                    );
                current_block_32 = 3205284525055210370;
            }
            _ => {}
        }
        match current_block_32 {
            3205284525055210370 => {
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
        RETURN_ERROR_IF!(lastByte_0 as std::ffi::c_int == 0, ZSTD_error_corruption_detected);
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                (::core::mem::size_of::<BitContainerType>())
                    .wrapping_sub(srcSize) as u32 * 8,
            );
    }
    return srcSize;
}
#[inline(always)]
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    return bitContainer >> (start & regMask)
        & (1_usize << nbBits)
            .wrapping_sub(1);
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    return BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<BitContainerType>() as u32)
            .wrapping_mul(8)
            .wrapping_sub((*bitD).bitsConsumed)
            .wrapping_sub(nbBits),
        nbBits,
    );
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
#[inline(always)]
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline]
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> usize {
    let value = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
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
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if (*bitD).bitsConsumed as usize
        > (::core::mem::size_of::<BitContainerType>())
            .wrapping_mul(8)
    {
        static mut zeroFilled: BitContainerType = 0;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as usize)
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
        .wrapping_sub(nbBytes * 8_u32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    return result;
}
#[inline]
unsafe extern "C" fn BIT_endOfDStream(
    mut DStream: *const BIT_DStream_t,
) -> bool {
    (*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as usize
            == ::core::mem::size_of::<BitContainerType>().wrapping_mul(8)
}
pub const ZSTD_BLOCKSIZELOG_MAX: std::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: std::ffi::c_int = (1 as std::ffi::c_int)
    << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_WINDOWLOG_MAX_32: std::ffi::c_int = 30;
static mut LL_base: [u32; 36] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    18,
    20,
    22,
    24,
    28,
    32,
    40,
    48,
    64,
    0x80 as std::ffi::c_int as u32,
    0x100 as std::ffi::c_int as u32,
    0x200 as std::ffi::c_int as u32,
    0x400 as std::ffi::c_int as u32,
    0x800 as std::ffi::c_int as u32,
    0x1000 as std::ffi::c_int as u32,
    0x2000 as std::ffi::c_int as u32,
    0x4000 as std::ffi::c_int as u32,
    0x8000 as std::ffi::c_int as u32,
    0x10000 as std::ffi::c_int as u32,
];
static mut OF_base: [u32; 32] = [
    0,
    1,
    1,
    5,
    0xd as std::ffi::c_int as u32,
    0x1d as std::ffi::c_int as u32,
    0x3d as std::ffi::c_int as u32,
    0x7d as std::ffi::c_int as u32,
    0xfd as std::ffi::c_int as u32,
    0x1fd as std::ffi::c_int as u32,
    0x3fd as std::ffi::c_int as u32,
    0x7fd as std::ffi::c_int as u32,
    0xffd as std::ffi::c_int as u32,
    0x1ffd as std::ffi::c_int as u32,
    0x3ffd as std::ffi::c_int as u32,
    0x7ffd as std::ffi::c_int as u32,
    0xfffd as std::ffi::c_int as u32,
    0x1fffd as std::ffi::c_int as u32,
    0x3fffd as std::ffi::c_int as u32,
    0x7fffd as std::ffi::c_int as u32,
    0xffffd as std::ffi::c_int as u32,
    0x1ffffd as std::ffi::c_int as u32,
    0x3ffffd as std::ffi::c_int as u32,
    0x7ffffd as std::ffi::c_int as u32,
    0xfffffd as std::ffi::c_int as u32,
    0x1fffffd as std::ffi::c_int as u32,
    0x3fffffd as std::ffi::c_int as u32,
    0x7fffffd as std::ffi::c_int as u32,
    0xffffffd as std::ffi::c_int as u32,
    0x1ffffffd as std::ffi::c_int as u32,
    0x3ffffffd as std::ffi::c_int as u32,
    0x7ffffffd as std::ffi::c_int as u32,
];
static mut OF_bits: [u8; 32] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
];
static mut ML_base: [u32; 53] = [
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    37,
    39,
    41,
    43,
    47,
    51,
    59,
    67,
    83,
    99,
    0x83 as std::ffi::c_int as u32,
    0x103 as std::ffi::c_int as u32,
    0x203 as std::ffi::c_int as u32,
    0x403 as std::ffi::c_int as u32,
    0x803 as std::ffi::c_int as u32,
    0x1003 as std::ffi::c_int as u32,
    0x2003 as std::ffi::c_int as u32,
    0x4003 as std::ffi::c_int as u32,
    0x8003 as std::ffi::c_int as u32,
    0x10003 as std::ffi::c_int as u32,
];
#[inline]
unsafe extern "C" fn ZSTD_DCtx_get_bmi2(
    mut dctx: *const ZSTD_DCtx_s,
) -> std::ffi::c_int {
    return (*dctx).bmi2;
}
pub const ZSTD_REP_NUM: std::ffi::c_int = 3;
pub const ZSTD_BLOCKHEADERSIZE: std::ffi::c_int = 3;
static mut ZSTD_blockHeaderSize: usize = ZSTD_BLOCKHEADERSIZE as usize;
pub const LONGNBSEQ: std::ffi::c_int = 0x7f00 as std::ffi::c_int;
pub const MaxML: std::ffi::c_int = 52;
pub const MaxLL: std::ffi::c_int = 35;
pub const MaxOff: std::ffi::c_int = 31;
pub const MLFSELog: std::ffi::c_int = 9;
pub const LLFSELog: std::ffi::c_int = 9;
pub const OffFSELog: std::ffi::c_int = 8;
static mut LL_bits: [u8; 36] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    2,
    2,
    3,
    3,
    4,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
];
pub const LL_DEFAULTNORMLOG: std::ffi::c_int = 6;
static mut ML_bits: [u8; 53] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    2,
    2,
    3,
    3,
    4,
    4,
    5,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
];
pub const ML_DEFAULTNORMLOG: std::ffi::c_int = 6;
pub const OF_DEFAULTNORMLOG: std::ffi::c_int = 5;
unsafe extern "C" fn ZSTD_copy8(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    libc::memcpy(dst, src, (8) as usize);
}
unsafe extern "C" fn ZSTD_copy16(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: usize = 32;
pub const WILDCOPY_VECLEN: usize = 16;
#[inline(always)]
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: usize,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut u8).offset_from(src as *const u8)
        as std::ffi::c_long;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    if ovtype as std::ffi::c_uint
        == ZSTD_overlap_src_before_dst as std::ffi::c_int as std::ffi::c_uint
        && diff < WILDCOPY_VECLEN as ptrdiff_t
    {
        loop {
            COPY8!(op, ip);
            if !(op < oend) {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        if 16_usize >= length {
            return;
        }
        op = op.offset(16);
        ip = ip.offset(16);
        loop {
            COPY16!(op, ip);
            COPY16!(op, ip);
            if !(op < oend) {
                break;
            }
        }
    };
}
pub const NULL: std::ffi::c_int = 0;
unsafe extern "C" fn ZSTD_copy4(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    libc::memcpy(dst, src, (4) as usize);
}
unsafe extern "C" fn ZSTD_blockSizeMax(mut dctx: *const ZSTD_DCtx) -> usize {
    let blockSizeMax = (if (*dctx).isFrameDecompression != 0 {
        (*dctx).fParams.blockSizeMax
    } else {
        ZSTD_BLOCKSIZE_MAX as std::ffi::c_uint
    }) as usize;
    return blockSizeMax;
}

pub unsafe fn ZSTD_getcBlockSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut bpPtr: *mut blockProperties_t,
) -> usize {
    RETURN_ERROR_IF!(srcSize < ZSTD_blockHeaderSize, ZSTD_error_srcSize_wrong);
    let cBlockHeader = MEM_readLE24(src);
    let cSize = cBlockHeader >> 3;
    (*bpPtr).lastBlock = cBlockHeader & 1;
    (*bpPtr).blockType = (cBlockHeader >> 1 & 3_u32) as blockType_e;
    (*bpPtr).origSize = cSize; /* only useful for RLE */
    if (*bpPtr).blockType == bt_rle {
        return 1;
    }
    RETURN_ERROR_IF!((*bpPtr).blockType == bt_reserved, ZSTD_error_corruption_detected);
    return cSize as usize;
}

unsafe extern "C" fn ZSTD_allocateLiteralsBuffer(
    mut dctx: *mut ZSTD_DCtx,
    dst: *mut std::ffi::c_void,
    dstCapacity: usize,
    litSize: usize,
    streaming: streaming_operation,
    expectedWriteSize: usize,
    splitImmediately: std::ffi::c_uint,
) {
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    if streaming as std::ffi::c_uint
        == not_streaming as std::ffi::c_int as std::ffi::c_uint
        && dstCapacity
            > blockSizeMax
                .wrapping_add(WILDCOPY_OVERLENGTH as usize)
                .wrapping_add(litSize)
                .wrapping_add(WILDCOPY_OVERLENGTH as usize)
    {
        (*dctx)
            .litBuffer = (dst as *mut u8)
            .offset(blockSizeMax as isize)
            .offset(WILDCOPY_OVERLENGTH as isize);
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_in_dst;
    } else if litSize
        <= (if 64 as std::ffi::c_int
            > (if ((1 as std::ffi::c_int) << 16)
                < (128 as std::ffi::c_int) << 10
            {
                (1 as std::ffi::c_int) << 16
            } else {
                (128 as std::ffi::c_int) << 10
            })
        {
            64 as std::ffi::c_int
        } else {
            (if ((1 as std::ffi::c_int) << 16)
                < (128 as std::ffi::c_int) << 10
            {
                (1 as std::ffi::c_int) << 16
            } else {
                (128 as std::ffi::c_int) << 10
            })
        }) as usize
    {
        (*dctx).litBuffer = ((*dctx).litExtraBuffer).as_mut_ptr();
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    } else {
        if splitImmediately != 0 {
            (*dctx)
                .litBuffer = (dst as *mut u8)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize))
                .offset(
                    (if 64 as std::ffi::c_int
                        > (if ((1 as std::ffi::c_int) << 16)
                            < (128 as std::ffi::c_int) << 10
                        {
                            (1 as std::ffi::c_int) << 16
                        } else {
                            (128 as std::ffi::c_int) << 10
                        })
                    {
                        64 as std::ffi::c_int
                    } else {
                        (if ((1 as std::ffi::c_int) << 16)
                            < (128 as std::ffi::c_int) << 10
                        {
                            (1 as std::ffi::c_int) << 16
                        } else {
                            (128 as std::ffi::c_int) << 10
                        })
                    }) as isize,
                )
                .offset(-(WILDCOPY_OVERLENGTH as isize));
            (*dctx)
                .litBufferEnd = ((*dctx).litBuffer)
                .offset(litSize as isize)
                .offset(
                    -((if 64 as std::ffi::c_int
                        > (if ((1 as std::ffi::c_int) << 16)
                            < (128 as std::ffi::c_int) << 10
                        {
                            (1 as std::ffi::c_int) << 16
                        } else {
                            (128 as std::ffi::c_int) << 10
                        })
                    {
                        64 as std::ffi::c_int
                    } else {
                        (if ((1 as std::ffi::c_int) << 16)
                            < (128 as std::ffi::c_int) << 10
                        {
                            (1 as std::ffi::c_int) << 16
                        } else {
                            (128 as std::ffi::c_int) << 10
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
    };
}
unsafe extern "C" fn ZSTD_decodeLiteralsBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    streaming: streaming_operation,
) -> usize {
    RETURN_ERROR_IF!(srcSize < (1 as std::ffi::c_int + 1 as std::ffi::c_int) as usize, ZSTD_error_corruption_detected);
    let istart = src as *const u8;
    let litEncType = (*istart.offset(0) as std::ffi::c_int
        & 3 as std::ffi::c_int) as SymbolEncodingType_e;
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    match litEncType as std::ffi::c_uint {
        3 => {
            RETURN_ERROR_IF!((*dctx).litEntropy == 0, ZSTD_error_dictionary_corrupted);
        }
        2 => {}
        0 => {
            let mut litSize_0: usize = 0;
            let mut lhSize_0: usize = 0;
            let lhlCode_0 = (*istart.offset(0)
                as std::ffi::c_int >> 2 & 3 as std::ffi::c_int)
                as u32;
            let mut expectedWriteSize_0 = std::cmp::min(blockSizeMax, dstCapacity);
            match lhlCode_0 {
                1 => {
                    lhSize_0 = 2;
                    litSize_0 = (MEM_readLE16(istart as *const std::ffi::c_void)
                        as std::ffi::c_int >> 4) as usize;
                }
                3 => {
                    lhSize_0 = 3;
                    if srcSize < 3 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int)
                            as usize;
                    }
                    litSize_0 = (MEM_readLE24(istart as *const std::ffi::c_void)
                        >> 4) as usize;
                }
                0 | 2 | _ => {
                    lhSize_0 = 1;
                    litSize_0 = (*istart.offset(0)
                        as std::ffi::c_int >> 3) as usize;
                }
            }
            RETURN_ERROR_IF!(litSize_0 > 0 && dst.is_null(), ZSTD_error_dstSize_tooSmall);
            RETURN_ERROR_IF!(litSize_0 > blockSizeMax, ZSTD_error_corruption_detected);
            RETURN_ERROR_IF!(expectedWriteSize_0 < litSize_0, ZSTD_error_dstSize_tooSmall);
            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst,
                dstCapacity,
                litSize_0,
                streaming,
                expectedWriteSize_0,
                1,
            );
            if lhSize_0
                .wrapping_add(litSize_0)
                .wrapping_add(WILDCOPY_OVERLENGTH as usize) > srcSize
            {
                if litSize_0.wrapping_add(lhSize_0) > srcSize {
                    return ERROR(ZSTD_error_corruption_detected);
                }
                if (*dctx).litBufferLocation == ZSTD_split {
                    libc::memcpy((*dctx).litBuffer.cast(), istart.add(lhSize_0).cast(), 
                    litSize_0 - ZSTD_LITBUFFEREXTRASIZE);
                    libc::memcpy(addr_of_mut!((*dctx).litExtraBuffer).cast(), 
                        istart.add(lhSize_0).add(litSize_0).sub(ZSTD_LITBUFFEREXTRASIZE).cast(), ZSTD_LITBUFFEREXTRASIZE);
                } else {
                    libc::memcpy((*dctx).litBuffer.cast(), istart.add(lhSize_0).cast(), litSize_0);
                }
                (*dctx).litPtr = (*dctx).litBuffer;
                (*dctx).litSize = litSize_0;
                return lhSize_0.wrapping_add(litSize_0);
            }
            (*dctx).litPtr = istart.add(lhSize_0);
            (*dctx).litSize = litSize_0;
            (*dctx).litBufferEnd = ((*dctx).litPtr).add(litSize_0);
            (*dctx).litBufferLocation = ZSTD_not_in_dst;
            return lhSize_0.wrapping_add(litSize_0);
        }
        1 => {
            let lhlCode_1 = (*istart.offset(0)
                as std::ffi::c_int >> 2 & 3 as std::ffi::c_int)
                as u32;
            let mut litSize_1: usize = 0;
            let mut lhSize_1: usize = 0;
            let mut expectedWriteSize_1 = std::cmp::min(blockSizeMax, dstCapacity);
            match lhlCode_1 {
                1 => {
                    lhSize_1 = 2;
                    if srcSize < 3 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int)
                            as usize;
                    }
                    litSize_1 = (MEM_readLE16(istart as *const std::ffi::c_void)
                        as std::ffi::c_int >> 4) as usize;
                }
                3 => {
                    lhSize_1 = 3;
                    if srcSize < 4 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int)
                            as usize;
                    }
                    litSize_1 = (MEM_readLE24(istart as *const std::ffi::c_void)
                        >> 4) as usize;
                }
                0 | 2 | _ => {
                    lhSize_1 = 1;
                    litSize_1 = (*istart.offset(0)
                        as std::ffi::c_int >> 3) as usize;
                }
            }
            RETURN_ERROR_IF!(litSize_1 > 0 && dst.is_null(), ZSTD_error_dstSize_tooSmall);
            RETURN_ERROR_IF!(litSize_1 > blockSizeMax, ZSTD_error_corruption_detected);
            RETURN_ERROR_IF!(expectedWriteSize_1 < litSize_1, ZSTD_error_dstSize_tooSmall);
            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst,
                dstCapacity,
                litSize_1,
                streaming,
                expectedWriteSize_1,
                1,
            );
            if (*dctx).litBufferLocation == ZSTD_split {
                libc::memset((*dctx).litBuffer.cast(), (*istart.add(lhSize_1)).into(), 
                litSize_1 - ZSTD_LITBUFFEREXTRASIZE);
                libc::memset(addr_of_mut!((*dctx).litExtraBuffer).cast(), (*istart.add(lhSize_1)).into(), ZSTD_LITBUFFEREXTRASIZE);
            } else {
                libc::memset((*dctx).litBuffer.cast(), (*istart.add(lhSize_1)).into(), litSize_1);
            }
            (*dctx).litPtr = (*dctx).litBuffer;
            (*dctx).litSize = litSize_1;
            return lhSize_1.wrapping_add(1);
        }
        _ => return ERROR(ZSTD_error_corruption_detected),
    }
    RETURN_ERROR_IF!(srcSize < 5, ZSTD_error_corruption_detected);
    let mut lhSize: usize = 0;
    let mut litSize: usize = 0;
    let mut litCSize: usize = 0;
    let mut singleStream: u32 = 0;
    let lhlCode = (*istart.offset(0) as std::ffi::c_int
        >> 2 & 3 as std::ffi::c_int) as u32;
    let lhc = MEM_readLE32(istart as *const std::ffi::c_void);
    let mut hufSuccess: usize = 0;
    let mut expectedWriteSize = std::cmp::min(blockSizeMax, dstCapacity);
    let flags = 0 as std::ffi::c_int
        | (if ZSTD_DCtx_get_bmi2(dctx) != 0 {
            HUF_flags_bmi2 as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        })
        | (if (*dctx).disableHufAsm != 0 {
            HUF_flags_disableAsm as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        });
    match lhlCode {
        2 => {
            lhSize = 4;
            litSize = (lhc >> 4 & 0x3fff as std::ffi::c_int as u32)
                as usize;
            litCSize = (lhc >> 18) as usize;
        }
        3 => {
            lhSize = 5;
            litSize = (lhc >> 4 & 0x3ffff as std::ffi::c_int as u32)
                as usize;
            litCSize = ((lhc >> 22) as usize)
                .wrapping_add(
                    (*istart.offset(4) as usize)
                        << 10,
                );
        }
        0 | 1 | _ => {
            singleStream = (lhlCode == 0) as std::ffi::c_int as u32;
            lhSize = 3;
            litSize = (lhc >> 4 & 0x3ff as std::ffi::c_int as u32)
                as usize;
            litCSize = (lhc >> 14 & 0x3ff as std::ffi::c_int as u32)
                as usize;
        }
    }
    RETURN_ERROR_IF!(litSize > 0 && dst.is_null(), ZSTD_error_dstSize_tooSmall);
    RETURN_ERROR_IF!(litSize > blockSizeMax, ZSTD_error_corruption_detected);
    if singleStream == 0 {
        RETURN_ERROR_IF!(litSize < 6, ZSTD_error_literals_headerWrong);
    }
    RETURN_ERROR_IF!(litCSize.wrapping_add(lhSize) > srcSize, ZSTD_error_corruption_detected);
    RETURN_ERROR_IF!(expectedWriteSize < litSize, ZSTD_error_dstSize_tooSmall);
    ZSTD_allocateLiteralsBuffer(
        dctx,
        dst,
        dstCapacity,
        litSize,
        streaming,
        expectedWriteSize,
        0,
    );
    if (*dctx).ddictIsCold != 0 && litSize > 768 {
        let _ptr = (*dctx).HUFptr as *const std::ffi::c_char;
        let _size = ::core::mem::size_of::<[HUF_DTable; 4097]>();
        let mut _pos: usize = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as usize);
        }
    }
    if litEncType as std::ffi::c_uint
        == set_repeat as std::ffi::c_int as std::ffi::c_uint
    {
        if singleStream != 0 {
            hufSuccess = HUF_decompress1X_usingDTable(
                (*dctx).litBuffer as *mut std::ffi::c_void,
                litSize,
                istart.offset(lhSize as isize) as *const std::ffi::c_void,
                litCSize,
                (*dctx).HUFptr,
                flags,
            );
        } else {
            hufSuccess = HUF_decompress4X_usingDTable(
                (*dctx).litBuffer as *mut std::ffi::c_void,
                litSize,
                istart.offset(lhSize as isize) as *const std::ffi::c_void,
                litCSize,
                (*dctx).HUFptr,
                flags,
            );
        }
    } else if singleStream != 0 {
        hufSuccess = HUF_decompress1X1_DCtx_wksp(
            ((*dctx).entropy.hufTable).as_mut_ptr(),
            (*dctx).litBuffer as *mut std::ffi::c_void,
            litSize,
            istart.offset(lhSize as isize) as *const std::ffi::c_void,
            litCSize,
            ((*dctx).workspace).as_mut_ptr() as *mut std::ffi::c_void,
            ::core::mem::size_of::<[u32; 640]>(),
            flags,
        );
    } else {
        hufSuccess = HUF_decompress4X_hufOnly_wksp(
            ((*dctx).entropy.hufTable).as_mut_ptr(),
            (*dctx).litBuffer as *mut std::ffi::c_void,
            litSize,
            istart.offset(lhSize as isize) as *const std::ffi::c_void,
            litCSize,
            ((*dctx).workspace).as_mut_ptr() as *mut std::ffi::c_void,
            ::core::mem::size_of::<[u32; 640]>(),
            flags,
        );
    }
    if (*dctx).litBufferLocation == ZSTD_split {
        libc::memcpy(addr_of_mut!((*dctx).litExtraBuffer).cast(), (*dctx).litBufferEnd.add(ZSTD_LITBUFFEREXTRASIZE).cast(), ZSTD_LITBUFFEREXTRASIZE);
        libc::memmove(
            (*dctx).litBuffer.add(ZSTD_LITBUFFEREXTRASIZE).sub(WILDCOPY_OVERLENGTH).cast(),
            (*dctx).litBuffer.cast(), litSize - ZSTD_LITBUFFEREXTRASIZE);
        (*dctx)
            .litBuffer = ((*dctx).litBuffer)
            .add(ZSTD_LITBUFFEREXTRASIZE - WILDCOPY_OVERLENGTH);
        (*dctx)
            .litBufferEnd = ((*dctx).litBufferEnd)
            .sub(WILDCOPY_OVERLENGTH);
    }
    RETURN_ERROR_IF!(ERR_isError(hufSuccess), ZSTD_error_corruption_detected);
    (*dctx).litPtr = (*dctx).litBuffer;
    (*dctx).litSize = litSize;
    (*dctx).litEntropy = 1;
    if litEncType as std::ffi::c_uint
        == set_compressed as std::ffi::c_int as std::ffi::c_uint
    {
        (*dctx).HUFptr = ((*dctx).entropy.hufTable).as_mut_ptr();
    }
    return litCSize.wrapping_add(lhSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeLiteralsBlock_wrapper(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
) -> usize {
    (*dctx).isFrameDecompression = 0;
    return ZSTD_decodeLiteralsBlock(dctx, src, srcSize, dst, dstCapacity, not_streaming);
}
static mut LL_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1,
            nbAdditionalBits: 1,
            nbBits: 1,
            baseValue: LL_DEFAULTNORMLOG as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 1,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 3,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 4,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 12,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 14,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 16,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 20,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 22,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 28,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 5,
            baseValue: 48,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 6,
            nbBits: 5,
            baseValue: 64,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 7,
            nbBits: 5,
            baseValue: 128,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 8,
            nbBits: 6,
            baseValue: 256,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 10,
            nbBits: 6,
            baseValue: 1024,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 12,
            nbBits: 6,
            baseValue: 4096,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 1,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 2,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 4,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 5,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 13,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 16,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 18,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 22,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 24,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 40,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 64,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 64,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 7,
            nbBits: 5,
            baseValue: 128,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 9,
            nbBits: 6,
            baseValue: 512,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 11,
            nbBits: 6,
            baseValue: 2048,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 48,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 1,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 2,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 3,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 5,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 12,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 15,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 18,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 20,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 24,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 28,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 40,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 4,
            nbBits: 5,
            baseValue: 48,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 16,
            nbBits: 6,
            baseValue: 65536,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 15,
            nbBits: 6,
            baseValue: 32768,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 14,
            nbBits: 6,
            baseValue: 16384,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 13,
            nbBits: 6,
            baseValue: 8192,
        };
        init
    },
];
static mut OF_defaultDTable: [ZSTD_seqSymbol; 33] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1,
            nbAdditionalBits: 1,
            nbBits: 1,
            baseValue: OF_DEFAULTNORMLOG as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 0,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 61,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 9,
            nbBits: 5,
            baseValue: 509,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 15,
            nbBits: 5,
            baseValue: 32765,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 21,
            nbBits: 5,
            baseValue: 2097149,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 5,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 7,
            nbBits: 4,
            baseValue: 125,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 12,
            nbBits: 5,
            baseValue: 4093,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 18,
            nbBits: 5,
            baseValue: 262141,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 23,
            nbBits: 5,
            baseValue: 8388605,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 5,
            nbBits: 5,
            baseValue: 29,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 8,
            nbBits: 4,
            baseValue: 253,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 14,
            nbBits: 5,
            baseValue: 16381,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 20,
            nbBits: 5,
            baseValue: 1048573,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 1,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 7,
            nbBits: 4,
            baseValue: 125,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 11,
            nbBits: 5,
            baseValue: 2045,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 17,
            nbBits: 5,
            baseValue: 131069,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 22,
            nbBits: 5,
            baseValue: 4194301,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 5,
            baseValue: 13,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 8,
            nbBits: 4,
            baseValue: 253,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 13,
            nbBits: 5,
            baseValue: 8189,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 19,
            nbBits: 5,
            baseValue: 524285,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 1,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 61,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 10,
            nbBits: 5,
            baseValue: 1021,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 16,
            nbBits: 5,
            baseValue: 65533,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 28,
            nbBits: 5,
            baseValue: 268435453,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 27,
            nbBits: 5,
            baseValue: 134217725,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 26,
            nbBits: 5,
            baseValue: 67108861,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 25,
            nbBits: 5,
            baseValue: 33554429,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 24,
            nbBits: 5,
            baseValue: 16777213,
        };
        init
    },
];
static mut ML_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 1,
            nbAdditionalBits: 1,
            nbBits: 1,
            baseValue: ML_DEFAULTNORMLOG as u32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 3,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 5,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 13,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 16,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 19,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 22,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 25,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 28,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 31,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 34,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 37,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 41,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 6,
            baseValue: 47,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 6,
            baseValue: 59,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 6,
            baseValue: 83,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 7,
            nbBits: 6,
            baseValue: 131,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 9,
            nbBits: 6,
            baseValue: 515,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 5,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 12,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 15,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 18,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 21,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 24,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 27,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 30,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 33,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 35,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 39,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 6,
            baseValue: 43,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 6,
            baseValue: 51,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 6,
            baseValue: 67,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 5,
            nbBits: 6,
            baseValue: 99,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 8,
            nbBits: 6,
            baseValue: 259,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 48,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 5,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 14,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 17,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 20,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 23,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 26,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 29,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 32,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 16,
            nbBits: 6,
            baseValue: 65539,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 15,
            nbBits: 6,
            baseValue: 32771,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 14,
            nbBits: 6,
            baseValue: 16387,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 13,
            nbBits: 6,
            baseValue: 8195,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 12,
            nbBits: 6,
            baseValue: 4099,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 11,
            nbBits: 6,
            baseValue: 2051,
        };
        init
    },
    {
        let mut init = ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 10,
            nbBits: 6,
            baseValue: 1027,
        };
        init
    },
];
unsafe extern "C" fn ZSTD_buildSeqTable_rle(
    mut dt: *mut ZSTD_seqSymbol,
    mut baseValue: u32,
    mut nbAddBits: u8,
) {
    let mut ptr = dt as *mut std::ffi::c_void;
    let DTableH = ptr as *mut ZSTD_seqSymbol_header;
    let cell = dt.offset(1);
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).nbBits = 0;
    (*cell).nextState = 0;
    (*cell).nbAdditionalBits = nbAddBits;
    (*cell).baseValue = baseValue;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_buildFSETable_body(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: usize,
) {
    let tableDecode = dt.offset(1);
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = 1_u32 << tableLog;
    let mut symbolNext = wksp as *mut u16;
    let mut spread = symbolNext
        .offset(
            (if 35 as std::ffi::c_int > 52 {
                35 as std::ffi::c_int
            } else {
                52 as std::ffi::c_int
            }) as isize,
        )
        .offset(1) as *mut u8;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let mut DTableH = ZSTD_seqSymbol_header {
        fastMode: 0,
        tableLog: 0,
    };
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1;
    let largeLimit = ((1 as std::ffi::c_int)
        << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as std::ffi::c_int
            == -(1 as std::ffi::c_int)
        {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).baseValue = s;
            *symbolNext.offset(s as isize) = 1;
        } else {
            if *normalizedCounter.offset(s as isize) as std::ffi::c_int
                >= largeLimit as std::ffi::c_int
            {
                DTableH.fastMode = 0;
            }
            *symbolNext
                .offset(s as isize) = *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
        s;
    }
    libc::memcpy(
        dt as *mut std::ffi::c_void,
        &mut DTableH as *mut ZSTD_seqSymbol_header as *const std::ffi::c_void,
        ::core::mem::size_of::<ZSTD_seqSymbol_header>(),
    );
    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1);
        let step = FSE_TABLESTEP(tableSize);
        let add = 0x101010101010101_u64;
        let mut pos: usize = 0;
        let mut sv: u64 = 0;
        let mut s_0: u32 = 0;
        while s_0 < maxSV1 {
            let mut i: isize = 0;
            let n = *normalizedCounter.offset(s_0 as isize) as isize;
            MEM_write64(spread.add(pos).cast(), sv);
            i = 8;
            while i < n {
                MEM_write64(
                    spread.add(pos).offset(i as isize).cast(),
                    sv,
                );
                i += 8;
            }
            pos = pos.wrapping_add(n as usize);
            s_0 = s_0.wrapping_add(1);
            sv = sv.wrapping_add(add);
        }
        let mut position: usize = 0;
        let mut s_1: usize = 0;
        let unroll = 2;
        while s_1 < tableSize as usize {
            let mut u: usize = 0;
            while u < unroll {
                let uPosition = position.wrapping_add(u * (step as usize)) & (tableMask as usize);
                (*tableDecode.add(uPosition))
                    .baseValue = *spread.add(s_1.wrapping_add(u)) as u32;
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll * (step as usize)) & (tableMask as usize);
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = FSE_TABLESTEP(tableSize);
        let mut s_2: u32 = 0;
        let mut position_0: u32 = 0;
        while s_2 < maxSV1 {
            let mut i_0 = 0;
            let n_0 = *normalizedCounter.add(s_2 as usize);
            while i_0 < n_0 {
                (*tableDecode.offset(position_0 as isize)).baseValue = s_2;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while UNLIKELY!(position > highThreshold) != 0 {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
            }
            s_2 = s_2.wrapping_add(1);
        }
    }
    let mut u_0: u32 = 0;
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
            << (*tableDecode.offset(u_0 as isize)).nbBits as std::ffi::c_int)
            .wrapping_sub(tableSize) as u16;
        (*tableDecode.offset(u_0 as isize))
            .nbAdditionalBits = *nbAdditionalBits.offset(symbol as isize);
        (*tableDecode.offset(u_0 as isize))
            .baseValue = *baseValue.offset(symbol as isize);
        u_0 = u_0.wrapping_add(1);
        u_0;
    }
}
unsafe extern "C" fn ZSTD_buildFSETable_body_default(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: usize,
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
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: usize,
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
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut bmi2: std::ffi::c_int,
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
    mut type_0: SymbolEncodingType_e,
    mut max: std::ffi::c_uint,
    mut maxLog: u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut defaultTable: *const ZSTD_seqSymbol,
    mut flagRepeatTable: u32,
    mut ddictIsCold: std::ffi::c_int,
    mut nbSeq: std::ffi::c_int,
    mut wksp: *mut u32,
    mut wkspSize: usize,
    mut bmi2: std::ffi::c_int,
) -> usize {
    match type_0 as std::ffi::c_uint {
        1 => {
            RETURN_ERROR_IF!(srcSize == 0, ZSTD_error_srcSize_wrong);
            RETURN_ERROR_IF!(*(src as *const u8) as std::ffi::c_uint > max, ZSTD_error_corruption_detected);
            let symbol = *(src as *const u8) as u32;
            let baseline = *baseValue.offset(symbol as isize);
            let nbBits = *nbAdditionalBits.offset(symbol as isize);
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);
            *DTablePtr = DTableSpace;
            return 1;
        }
        0 => {
            *DTablePtr = defaultTable;
            return 0;
        }
        3 => {
            RETURN_ERROR_IF!(flagRepeatTable == 0, ZSTD_error_corruption_detected);
            if ddictIsCold != 0 && nbSeq > 24 {
                let pStart = *DTablePtr as *const std::ffi::c_void;
                let pSize = (::core::mem::size_of::<ZSTD_seqSymbol>()
                    as std::ffi::c_ulong)
                    .wrapping_mul(
                        (SEQSYMBOL_TABLE_SIZE!(maxLog) + SEQSYMBOL_TABLE_SIZE!(maxLog))
                            as std::ffi::c_ulong,
                    );
                PREFETCH_AREA!(pStart, pSize);
            }
            return 0;
        }
        2 => {
            let mut tableLog: std::ffi::c_uint = 0;
            let mut norm: [i16; 53] = [0; 53];
            let headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut tableLog,
                src,
                srcSize,
            );
            RETURN_ERROR_IF!(ERR_isError(headerSize), ZSTD_error_corruption_detected);
            RETURN_ERROR_IF!(tableLog > maxLog, ZSTD_error_corruption_detected);
            ZSTD_buildFSETable(
                DTableSpace,
                norm.as_mut_ptr(),
                max,
                baseValue,
                nbAdditionalBits,
                tableLog,
                wksp as *mut std::ffi::c_void,
                wkspSize,
                bmi2,
            );
            *DTablePtr = DTableSpace;
            return headerSize;
        }
        _ => return ERROR(ZSTD_error_GENERIC),
    };
}

pub unsafe fn ZSTD_decodeSeqHeaders(
    mut dctx: *mut ZSTD_DCtx,
    mut nbSeqPtr: *mut std::ffi::c_int,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    let mut nbSeq: std::ffi::c_int = 0;

    DEBUGLOG!(5, "ZSTD_decodeSeqHeaders");

    /* check */
    RETURN_ERROR_IF!(srcSize < 1, ZSTD_error_srcSize_wrong);

    /* SeqHead */
    nbSeq = *ip as std::ffi::c_int;
    ip = ip.offset(1);
    if nbSeq > 0x7f {
        if nbSeq == 0xff {
            RETURN_ERROR_IF!(ip.offset(2) > iend, ZSTD_error_srcSize_wrong);
            nbSeq = MEM_readLE16(ip as *const std::ffi::c_void) + LONGNBSEQ as u16;
            ip = ip.offset(2);
        } else {
            RETURN_ERROR_IF!(ip >= iend, ZSTD_error_srcSize_wrong);
            nbSeq = (nbSeq - 0x80) << 8 + (*ip as std::ffi::c_int);
            ip = ip.offset(1);
        }
    }
    *nbSeqPtr = nbSeq;

    if nbSeq == 0 {
        /* No sequence : section ends immediately */
        RETURN_ERROR_IF!(ip != iend, ZSTD_error_corruption_detected,
            "extraneous data present in the Sequences section");
        return ip.offset_from(istart) as usize;
    }

    /* FSE table descriptors */
    RETURN_ERROR_IF!(ip.offset(1) > iend, ZSTD_error_srcSize_wrong); /* minimum possible size: 1 byte for symbol encoding types */
    RETURN_ERROR_IF!(*ip & 3 != 0, ZSTD_error_corruption_detected); /* The last field, Reserved, must be all-zeroes. */
    
    let LLtype = (*ip >> 6) as SymbolEncodingType_e;
    let OFtype = (*ip >> 4 & 3) as SymbolEncodingType_e;
    let MLtype = (*ip >> 2 & 3) as SymbolEncodingType_e;
    ip = ip.offset(1);

    /* Build DTables */
    debug_assert!(ip <= iend);
    let llhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.LLTable).as_mut_ptr(),
        &mut (*dctx).LLTptr,
        LLtype,
        MaxLL as std::ffi::c_uint,
        LLFSELog as u32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as usize,
        LL_base.as_ptr(),
        LL_bits.as_ptr(),
        LL_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; HUF_DECOMPRESS_WORKSPACE_SIZE_U32]>(),
        ZSTD_DCtx_get_bmi2(dctx),
    );
    RETURN_ERROR_IF!(ERR_isError(llhSize), ZSTD_error_corruption_detected);
    ip = ip.add(llhSize);

    debug_assert!(ip <= iend);
    let ofhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.OFTable).as_mut_ptr(),
        &mut (*dctx).OFTptr,
        OFtype,
        MaxOff,
        OffFSELog,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as usize,
        OF_base.as_ptr(),
        OF_bits.as_ptr(),
        OF_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; HUF_DECOMPRESS_WORKSPACE_SIZE_U32]>(),
        ZSTD_DCtx_get_bmi2(dctx),
    );
    RETURN_ERROR_IF!(ERR_isError(ofhSize), ZSTD_error_corruption_detected);
    ip = ip.add(ofhSize);

    debug_assert!(ip <= iend);
    let mlhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.MLTable).as_mut_ptr(),
        &mut (*dctx).MLTptr,
        MLtype,
        MaxML,
        MLFSELog,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as usize,
        ML_base.as_ptr(),
        ML_bits.as_ptr(),
        ML_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; HUF_DECOMPRESS_WORKSPACE_SIZE_U32]>(),
        ZSTD_DCtx_get_bmi2(dctx),
    );
    RETURN_ERROR_IF!(ERR_isError(mlhSize), ZSTD_error_corruption_detected);
    ip = ip.add(mlhSize);

    return ip.offset_from(istart) as usize;
}

#[inline(always)]
unsafe extern "C" fn ZSTD_overlapCopy8(
    mut op: *mut *mut u8,
    mut ip: *mut *const u8,
    mut offset: usize,
) {
    if offset < 8 {
        static mut dec32table: [u32; 8] = [
            0,
            1,
            2,
            1,
            4,
            4,
            4,
            4,
        ];
        static mut dec64table: [std::ffi::c_int; 8] = [
            8,
            8,
            8,
            7,
            8,
            9,
            10,
            11,
        ];
        let sub2 = dec64table[offset as usize];
        *(*op)
            .offset(
                0,
            ) = *(*ip).offset(0);
        *(*op)
            .offset(
                1,
            ) = *(*ip).offset(1);
        *(*op)
            .offset(
                2,
            ) = *(*ip).offset(2);
        *(*op)
            .offset(
                3,
            ) = *(*ip).offset(3);
        *ip = (*ip).offset(dec32table[offset as usize] as isize);
        ZSTD_copy4(
            (*op).offset(4) as *mut std::ffi::c_void,
            *ip as *const std::ffi::c_void,
        );
        *ip = (*ip).offset(-(sub2 as isize));
    } else {
        ZSTD_copy8(*op as *mut std::ffi::c_void, *ip as *const std::ffi::c_void);
    }
    *ip = (*ip).offset(8);
    *op = (*op).offset(8);
}
unsafe extern "C" fn ZSTD_safecopy(
    mut op: *mut u8,
    oend_w: *const u8,
    mut ip: *const u8,
    mut length: usize,
    mut ovtype: ZSTD_overlap_e,
) {
    let diff = op.offset_from(ip) as std::ffi::c_long;
    let oend = op.offset(length as isize);
    if length < 8 {
        while op < oend {
            let fresh7 = ip;
            ip = ip.offset(1);
            let fresh8 = op;
            op = op.offset(1);
            *fresh8 = *fresh7;
        }
        return;
    }
    if ovtype as std::ffi::c_uint
        == ZSTD_overlap_src_before_dst as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_overlapCopy8(&mut op, &mut ip, diff as usize);
        length = length.wrapping_sub(8);
    }
    if oend <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            length,
            ovtype,
        );
        return;
    }
    if op <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            oend_w.offset_from(op) as std::ffi::c_long as usize,
            ovtype,
        );
        ip = ip.offset(oend_w.offset_from(op) as std::ffi::c_long as isize);
        op = op.offset(oend_w.offset_from(op) as std::ffi::c_long as isize);
    }
    while op < oend {
        let fresh9 = ip;
        ip = ip.offset(1);
        let fresh10 = op;
        op = op.offset(1);
        *fresh10 = *fresh9;
    }
}
unsafe extern "C" fn ZSTD_safecopyDstBeforeSrc(
    mut op: *mut u8,
    mut ip: *const u8,
    mut length: usize,
) {
    let diff = op.offset_from(ip);
    let oend = op.add(length);
    if length < 8
        || diff > -8
    {
        while op < oend {
            let fresh11 = ip;
            ip = ip.offset(1);
            let fresh12 = op;
            op = op.offset(1);
            *fresh12 = *fresh11;
        }
        return;
    }
    if op <= oend.offset(-(WILDCOPY_OVERLENGTH as isize))
        && diff < -(WILDCOPY_VECLEN as isize)
    {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op) as usize,
            ZSTD_no_overlap,
        );
        ip = ip
            .offset(
                oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op),
            );
        op = op
            .offset(
                oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op),
            );
    }
    while op < oend {
        let fresh13 = ip;
        ip = ip.offset(1);
        let fresh14 = op;
        op = op.offset(1);
        *fresh14 = *fresh13;
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
) -> usize {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    RETURN_ERROR_IF!(sequenceLength > oend.offset_from(op) as std::ffi::c_long as usize, ZSTD_error_dstSize_tooSmall);
    RETURN_ERROR_IF!(sequence.litLength > litLimit.offset_from(*litPtr) as std::ffi::c_long as usize, ZSTD_error_corruption_detected);
    ZSTD_safecopy(op, oend_w, *litPtr, sequence.litLength, ZSTD_no_overlap);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as usize {
        RETURN_ERROR_IF!(sequence.offset
            > oLitEnd.offset_from(virtualStart) as usize, ZSTD_error_corruption_detected);
        match_0 = dictEnd
            .offset(-prefixStart.offset_from(match_0));
        if match_0.add(sequence.matchLength) <= dictEnd {
            libc::memmove(oLitEnd.cast(), match_0.cast(), sequence.matchLength);
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as usize;
        libc::memmove(oLitEnd.cast(), match_0.cast(), length1);
        op = oLitEnd.add(length1);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    ZSTD_safecopy(
        op,
        oend_w,
        match_0,
        sequence.matchLength,
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
) -> usize {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    RETURN_ERROR_IF!(sequenceLength > oend.offset_from(op) as std::ffi::c_long as usize, ZSTD_error_dstSize_tooSmall);
    RETURN_ERROR_IF!(sequence.litLength > litLimit.offset_from(*litPtr) as std::ffi::c_long as usize, ZSTD_error_corruption_detected);
    RETURN_ERROR_IF!(op > *litPtr as *mut u8
        && op < (*litPtr).offset(sequence.litLength as isize) as *mut u8, ZSTD_error_dstSize_tooSmall);
    ZSTD_safecopyDstBeforeSrc(op, *litPtr, sequence.litLength);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as std::ffi::c_long as usize {
        RETURN_ERROR_IF!(sequence.offset
            > oLitEnd.offset_from(virtualStart) as std::ffi::c_long as usize, ZSTD_error_corruption_detected);
        match_0 = dictEnd
            .offset(-prefixStart.offset_from(match_0));
        if match_0.add(sequence.matchLength) <= dictEnd {
            libc::memmove(oLitEnd.cast(), match_0.cast(), sequence.matchLength);
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as usize;
        libc::memmove(oLitEnd.cast(), match_0.cast(), length1);
        op = oLitEnd.add(length1);
        sequence.matchLength = sequence.matchLength.wrapping_sub(length1);
        match_0 = prefixStart;
    }
    ZSTD_safecopy(
        op,
        oend_w,
        match_0,
        sequence.matchLength,
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
) -> usize {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    if (iLitEnd > litLimit || oMatchEnd > oend_w
        || MEM_32bits
            && (oend.offset_from(op) as std::ffi::c_long as usize)
                < sequenceLength.wrapping_add(32))
        as std::ffi::c_int as std::ffi::c_long != 0
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
    ZSTD_copy16(op as *mut std::ffi::c_void, *litPtr as *const std::ffi::c_void);
    if UNLIKELY!(sequence.litLength > 16) != 0 {
        ZSTD_wildcopy(
            op.offset(16) as *mut std::ffi::c_void,
            (*litPtr).offset(16) as *const std::ffi::c_void,
            (sequence.litLength).wrapping_sub(16),
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as usize {
        RETURN_ERROR_IF!(sequence.offset > oLitEnd.offset_from(virtualStart) as usize,
            ZSTD_error_corruption_detected);
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart));
        if match_0.add(sequence.matchLength) <= dictEnd {
            libc::memmove(oLitEnd.cast(), match_0.cast(), sequence.matchLength);
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as usize;
        libc::memmove(oLitEnd.cast(), match_0.cast(), length1);
        op = oLitEnd.add(length1);
        sequence.matchLength = sequence.matchLength.wrapping_sub(length1);
        match_0 = prefixStart;
    }
    if LIKELY!(sequence.offset >= WILDCOPY_VECLEN) != 0 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            sequence.matchLength,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            (sequence.matchLength).wrapping_sub(8),
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
) -> usize {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    if (iLitEnd > litLimit || oMatchEnd > oend_w as *mut u8
        || MEM_32bits
            && (oend.offset_from(op) as std::ffi::c_long as usize)
                < sequenceLength.wrapping_add(32))
        as std::ffi::c_int as std::ffi::c_long != 0
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
    ZSTD_copy16(op as *mut std::ffi::c_void, *litPtr as *const std::ffi::c_void);
    if UNLIKELY!(sequence.litLength > 16) != 0 {
        ZSTD_wildcopy(
            op.offset(16) as *mut std::ffi::c_void,
            (*litPtr).offset(16) as *const std::ffi::c_void,
            (sequence.litLength).wrapping_sub(16),
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as usize {
        RETURN_ERROR_IF!(sequence.offset > oLitEnd.offset_from(virtualStart) as usize, ZSTD_error_corruption_detected);
        match_0 = dictEnd
            .offset(match_0.offset_from(prefixStart));
        if match_0.add(sequence.matchLength) <= dictEnd {
            libc::memmove(oLitEnd.cast(), match_0.cast(), sequence.matchLength);
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as usize;
        libc::memmove(oLitEnd.cast(), match_0.cast(), length1);
        op = oLitEnd.add(length1);
        sequence.matchLength = sequence.matchLength.wrapping_sub(length1);
        match_0 = prefixStart;
    }
    if LIKELY!(sequence.offset >= WILDCOPY_VECLEN) != 0 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            sequence.matchLength,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            (sequence.matchLength).wrapping_sub(8),
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
    let mut ptr = dt as *const std::ffi::c_void;
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
    (*DStatePtr).state = (nextState as usize).wrapping_add(lowBits);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decodeSequence(
    mut seqState: *mut seqState_t,
    longOffsets: ZSTD_longOffset_e,
    isLastSeq: std::ffi::c_int,
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
    seq.matchLength = (*mlDInfo).baseValue as usize;
    seq.litLength = (*llDInfo).baseValue as usize;
    let ofBase = (*ofDInfo).baseValue;
    let llBits = (*llDInfo).nbAdditionalBits;
    let mlBits = (*mlDInfo).nbAdditionalBits;
    let ofBits = (*ofDInfo).nbAdditionalBits;
    let totalBits = (llBits as std::ffi::c_int + mlBits as std::ffi::c_int
        + ofBits as std::ffi::c_int) as u8;
    let llNext = (*llDInfo).nextState;
    let mlNext = (*mlDInfo).nextState;
    let ofNext = (*ofDInfo).nextState;
    let llnbBits = (*llDInfo).nbBits as u32;
    let mlnbBits = (*mlDInfo).nbBits as u32;
    let ofnbBits = (*ofDInfo).nbBits as u32;
    let mut offset: usize = 0;
    if ofBits as std::ffi::c_int > 1 {
        if MEM_32bits && longOffsets as std::ffi::c_uint != 0
            && ofBits as std::ffi::c_int >= STREAM_ACCUMULATOR_MIN_32
        {
            let extraBits = (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
            } else {
                0 as std::ffi::c_int
            }) as u32;
            offset = (ofBase as usize)
                .wrapping_add(
                    BIT_readBitsFast(
                        &mut (*seqState).DStream,
                        (ofBits as u32).wrapping_sub(extraBits),
                    ) << extraBits,
                );
            BIT_reloadDStream(&mut (*seqState).DStream);
            offset = offset
                .wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream, extraBits));
        } else {
            offset = (ofBase as usize)
                .wrapping_add(
                    BIT_readBitsFast(
                        &mut (*seqState).DStream,
                        ofBits as std::ffi::c_uint,
                    ),
                );
            if MEM_32bits {
                BIT_reloadDStream(&mut (*seqState).DStream);
            }
        }
        (*seqState)
            .prevOffset[2] = (*seqState).prevOffset[1];
        (*seqState)
            .prevOffset[1] = (*seqState).prevOffset[0];
        (*seqState).prevOffset[0] = offset;
    } else {
        let ll0 = ((*llDInfo).baseValue == 0)
            as std::ffi::c_int as u32;
        if (ofBits as std::ffi::c_int == 0) as std::ffi::c_int
            as std::ffi::c_long != 0
        {
            offset = (*seqState).prevOffset[ll0 as usize];
            (*seqState)
                .prevOffset[1] = (*seqState)
                .prevOffset[(ll0 == 0) as std::ffi::c_int as usize];
            (*seqState).prevOffset[0] = offset;
        } else {
            offset = (ofBase.wrapping_add(ll0) as usize)
                .wrapping_add(
                    BIT_readBitsFast(
                        &mut (*seqState).DStream,
                        1,
                    ),
                );
            let mut temp = if offset == 3 {
                ((*seqState).prevOffset[0])
                    .wrapping_sub(1)
            } else {
                (*seqState).prevOffset[offset as usize]
            };
            temp = temp.wrapping_sub((temp == 0) as std::ffi::c_int as usize);
            if offset != 1 {
                (*seqState)
                    .prevOffset[2] = (*seqState).prevOffset[1];
            }
            (*seqState)
                .prevOffset[1] = (*seqState).prevOffset[0];
            offset = temp;
            (*seqState).prevOffset[0] = offset;
        }
    }
    seq.offset = offset;
    if mlBits as std::ffi::c_int > 0 {
        seq
            .matchLength = (seq.matchLength)
            .wrapping_add(
                BIT_readBitsFast(&mut (*seqState).DStream, mlBits as std::ffi::c_uint),
            );
    }
    if MEM_32bits
        && mlBits as std::ffi::c_int + llBits as std::ffi::c_int
            >= STREAM_ACCUMULATOR_MIN_32
                - (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                    ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
                } else {
                    0 as std::ffi::c_int
                })
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if MEM_64bits
        && (totalBits as std::ffi::c_int
            >= 57
                - (9 as std::ffi::c_int + 9 as std::ffi::c_int + 8 as std::ffi::c_int))
            as std::ffi::c_int as std::ffi::c_long != 0
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if llBits as std::ffi::c_int > 0 {
        seq
            .litLength = (seq.litLength)
            .wrapping_add(
                BIT_readBitsFast(&mut (*seqState).DStream, llBits as std::ffi::c_uint),
            );
    }
    if MEM_32bits {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if isLastSeq == 0 {
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
        if MEM_32bits {
            BIT_reloadDStream(&mut (*seqState).DStream);
        }
        ZSTD_updateFseStateWithDInfo(
            &mut (*seqState).stateOffb,
            &mut (*seqState).DStream,
            ofNext,
            ofnbBits,
        );
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    return seq;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_bodySplitLitBuffer(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
    let ostart = dst as *mut u8;
    let oend = ZSTD_maybeNullPtrAdd(
        ostart as *mut std::ffi::c_void,
        maxDstSize as ptrdiff_t,
    ) as *mut u8;
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
                ptr: std::ptr::null(),
                start: std::ptr::null(),
                limitPtr: std::ptr::null(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTD_REP_NUM as u32 {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as usize;
            i = i.wrapping_add(1);
            i;
        }
        RETURN_ERROR_IF!(ERR_isError(BIT_initDStream(&mut seqState.DStream, seqStart, seqSize)), ZSTD_error_corruption_detected);
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        let mut sequence = {
            let mut init = seq_t {
                litLength: 0,
                matchLength: 0,
                offset: 0,
            };
            init
        };
        asm!(".p2align 6", options(preserves_flags, att_syntax));
        while nbSeq != 0 {
            sequence = ZSTD_decodeSequence(
                &mut seqState,
                isLongOffset,
                (nbSeq == 1) as std::ffi::c_int,
            );
            if litPtr.offset(sequence.litLength as isize) > (*dctx).litBufferEnd {
                break;
            }
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
            if ERR_isError(oneSeqSize) as std::ffi::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
            nbSeq;
        }
        if nbSeq > 0 {
            let leftoverLit = ((*dctx).litBufferEnd).offset_from(litPtr)
                as std::ffi::c_long as usize;
            if leftoverLit != 0 {
                RETURN_ERROR_IF!(leftoverLit > oend.offset_from(op) as std::ffi::c_long as usize, ZSTD_error_dstSize_tooSmall);
                ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                sequence.litLength = (sequence.litLength).wrapping_sub(leftoverLit);
                op = op.offset(leftoverLit as isize);
            }
            litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
            litBufferEnd = ((*dctx).litExtraBuffer)
                .as_mut_ptr()
                .offset(
                    (if 64 as std::ffi::c_int
                        > (if ((1 as std::ffi::c_int) << 16)
                            < (128 as std::ffi::c_int) << 10
                        {
                            (1 as std::ffi::c_int) << 16
                        } else {
                            (128 as std::ffi::c_int) << 10
                        })
                    {
                        64 as std::ffi::c_int
                    } else {
                        (if ((1 as std::ffi::c_int) << 16)
                            < (128 as std::ffi::c_int) << 10
                        {
                            (1 as std::ffi::c_int) << 16
                        } else {
                            (128 as std::ffi::c_int) << 10
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
            if ERR_isError(oneSeqSize_0) as std::ffi::c_long != 0 {
                return oneSeqSize_0;
            }
            op = op.offset(oneSeqSize_0 as isize);
            nbSeq -= 1;
            nbSeq;
        }
        if nbSeq > 0 {
            asm!(".p2align 6", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 4", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 3", options(preserves_flags, att_syntax));
            while nbSeq != 0 {
                let sequence_0 = ZSTD_decodeSequence(
                    &mut seqState,
                    isLongOffset,
                    (nbSeq == 1) as std::ffi::c_int,
                );
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
                if ERR_isError(oneSeqSize_1) as std::ffi::c_long != 0 {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
                nbSeq -= 1;
                nbSeq;
            }
        }
        RETURN_ERROR_IF!(nbSeq != 0, ZSTD_error_corruption_detected);
        RETURN_ERROR_IF!(!BIT_endOfDStream(&mut seqState.DStream), ZSTD_error_corruption_detected);
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTD_REP_NUM as u32 {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    if (*dctx).litBufferLocation == ZSTD_split {
        let lastLLSize = litBufferEnd.offset_from(litPtr);
        RETURN_ERROR_IF!(lastLLSize > oend.offset_from(op), ZSTD_error_dstSize_tooSmall);
        if !op.is_null() {
            libc::memmove(op.cast(), litPtr.cast(), lastLLSize as usize);
            op = op.offset(lastLLSize);
        }
        litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
        litBufferEnd = ((*dctx).litExtraBuffer)
            .as_mut_ptr()
            .offset(
                (if 64 as std::ffi::c_int
                    > (if ((1 as std::ffi::c_int) << 16)
                        < (128 as std::ffi::c_int) << 10
                    {
                        (1 as std::ffi::c_int) << 16
                    } else {
                        (128 as std::ffi::c_int) << 10
                    })
                {
                    64 as std::ffi::c_int
                } else {
                    (if ((1 as std::ffi::c_int) << 16)
                        < (128 as std::ffi::c_int) << 10
                    {
                        (1 as std::ffi::c_int) << 16
                    } else {
                        (128 as std::ffi::c_int) << 10
                    })
                }) as isize,
            );
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr);
    RETURN_ERROR_IF!(lastLLSize_0 > oend.offset_from(op), ZSTD_error_dstSize_tooSmall);
    if !op.is_null() {
        libc::memcpy(op.cast(), litPtr.cast(), lastLLSize_0 as usize);
        op = op.offset(lastLLSize_0);
    }
    return op.offset_from(ostart) as usize;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
    let ostart = dst as *mut u8;
    let oend = if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_not_in_dst as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_maybeNullPtrAdd(ostart as *mut std::ffi::c_void, maxDstSize as ptrdiff_t)
            as *mut u8
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
                ptr: std::ptr::null(),
                start: std::ptr::null(),
                limitPtr: std::ptr::null(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTD_REP_NUM as u32 {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as usize;
            i = i.wrapping_add(1);
            i;
        }
        RETURN_ERROR_IF!(ERR_isError(BIT_initDStream(&mut seqState.DStream, seqStart, seqSize)), ZSTD_error_corruption_detected);
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        asm!(".p2align 6", options(preserves_flags, att_syntax));
        asm!("nop", options(preserves_flags, att_syntax));
        asm!(".p2align 4", options(preserves_flags, att_syntax));
        asm!("nop", options(preserves_flags, att_syntax));
        asm!(".p2align 3", options(preserves_flags, att_syntax));
        while nbSeq != 0 {
            let sequence = ZSTD_decodeSequence(
                &mut seqState,
                isLongOffset,
                (nbSeq == 1) as std::ffi::c_int,
            );
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
            if ERR_isError(oneSeqSize) as std::ffi::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
            nbSeq;
        }
        RETURN_ERROR_IF!(!BIT_endOfDStream(&mut seqState.DStream), ZSTD_error_corruption_detected);
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTD_REP_NUM as u32 {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr);
    RETURN_ERROR_IF!(lastLLSize > oend.offset_from(op), ZSTD_error_dstSize_tooSmall);
    if !op.is_null() {
        libc::memcpy(op.cast(), litPtr.cast(), lastLLSize as usize);
        op = op.offset(lastLLSize);
    }
    return op.offset_from(ostart) as usize;
}
unsafe extern "C" fn ZSTD_decompressSequences_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut prefetchPos: usize,
    sequence: seq_t,
    prefixStart: *const u8,
    dictEnd: *const u8,
) -> usize {
    prefetchPos = prefetchPos.wrapping_add(sequence.litLength);
    let matchBase = if sequence.offset > prefetchPos { dictEnd } else { prefixStart };
    let match_0 = ZSTD_wrappedPtrSub(
        ZSTD_wrappedPtrAdd(
            matchBase as *const std::ffi::c_void,
            prefetchPos as ptrdiff_t,
        ),
        sequence.offset as ptrdiff_t,
    ) as *const u8;
    ZSTD_wrappedPtrAdd(
        match_0 as *const std::ffi::c_void,
        64,
    );
    return prefetchPos.wrapping_add(sequence.matchLength);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequencesLong_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
    let ostart = dst as *mut u8;
    let oend = if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_in_dst as std::ffi::c_int as std::ffi::c_uint
    {
        (*dctx).litBuffer
    } else {
        ZSTD_maybeNullPtrAdd(ostart as *mut std::ffi::c_void, maxDstSize as ptrdiff_t)
            as *mut u8
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
        let seqAdvance = std::cmp::min(nbSeq, ADVANCED_SEQS);
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: std::ptr::null(),
                start: std::ptr::null(),
                limitPtr: std::ptr::null(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: std::ptr::null(),
            },
            prevOffset: [0; 3],
        };
        let mut seqNb: std::ffi::c_int = 0;
        let mut prefetchPos = op.offset_from(prefixStart) as std::ffi::c_long as usize;
        (*dctx).fseEntropy = 1;
        let mut i: std::ffi::c_int = 0;
        i = 0;
        while i < ZSTD_REP_NUM {
            seqState.prevOffset[i as usize] = (*dctx).entropy.rep[i as usize] as usize;
            i += 1;
            i;
        }
        RETURN_ERROR_IF!(ERR_isError(BIT_initDStream(&mut seqState.DStream, seqStart, seqSize)), ZSTD_error_corruption_detected);
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        seqNb = 0;
        while seqNb < seqAdvance {
            let sequence = ZSTD_decodeSequence(
                &mut seqState,
                isLongOffset,
                (seqNb == nbSeq - 1 as std::ffi::c_int) as std::ffi::c_int,
            );
            prefetchPos = ZSTD_prefetchMatch(
                prefetchPos,
                sequence,
                prefixStart,
                dictEnd,
            );
            sequences[seqNb as usize] = sequence;
            seqNb += 1;
            seqNb;
        }
        while seqNb < nbSeq {
            let mut sequence_0 = ZSTD_decodeSequence(
                &mut seqState,
                isLongOffset,
                (seqNb == nbSeq - 1 as std::ffi::c_int) as std::ffi::c_int,
            );
            if (*dctx).litBufferLocation as std::ffi::c_uint
                == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                && litPtr
                    .offset(
                        sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize]
                            .litLength as isize,
                    ) > (*dctx).litBufferEnd
            {
                let leftoverLit = ((*dctx).litBufferEnd).offset_from(litPtr)
                    as std::ffi::c_long as usize;
                if leftoverLit != 0 {
                    if leftoverLit > oend.offset_from(op) as std::ffi::c_long as usize {
                        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int)
                            as usize;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                    sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize]
                        .litLength = (sequences[(seqNb - ADVANCED_SEQS
                            & STORED_SEQS_MASK) as usize]
                        .litLength)
                        .wrapping_sub(leftoverLit);
                    op = op.offset(leftoverLit as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer)
                    .as_mut_ptr()
                    .offset(
                        (if 64 as std::ffi::c_int
                            > (if ((1 as std::ffi::c_int) << 16)
                                < (128 as std::ffi::c_int) << 10
                            {
                                (1 as std::ffi::c_int) << 16
                            } else {
                                (128 as std::ffi::c_int) << 10
                            })
                        {
                            64 as std::ffi::c_int
                        } else {
                            (if ((1 as std::ffi::c_int) << 16)
                                < (128 as std::ffi::c_int) << 10
                            {
                                (1 as std::ffi::c_int) << 16
                            } else {
                                (128 as std::ffi::c_int) << 10
                            })
                        }) as isize,
                    );
                (*dctx).litBufferLocation = ZSTD_not_in_dst;
                let oneSeqSize = ZSTD_execSequence(
                    op,
                    oend,
                    sequences[(seqNb - ADVANCED_SEQS & STORED_SEQS_MASK) as usize],
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize) {
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
                let oneSeqSize_0 = if (*dctx).litBufferLocation as std::ffi::c_uint
                    == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
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
                if ERR_isError(oneSeqSize_0) {
                    return oneSeqSize_0;
                }
                prefetchPos = ZSTD_prefetchMatch(
                    prefetchPos,
                    sequence_0,
                    prefixStart,
                    dictEnd,
                );
                sequences[(seqNb & STORED_SEQS_MASK) as usize] = sequence_0;
                op = op.offset(oneSeqSize_0 as isize);
            }
            seqNb += 1;
            seqNb;
        }
        RETURN_ERROR_IF!(!BIT_endOfDStream(&mut seqState.DStream), ZSTD_error_corruption_detected);
        seqNb -= seqAdvance;
        while seqNb < nbSeq {
            let mut sequence_1: *mut seq_t = &mut *sequences
                .as_mut_ptr()
                .offset((seqNb & STORED_SEQS_MASK) as isize) as *mut seq_t;
            if (*dctx).litBufferLocation as std::ffi::c_uint
                == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                && litPtr.offset((*sequence_1).litLength as isize) > (*dctx).litBufferEnd
            {
                let leftoverLit_0 = ((*dctx).litBufferEnd).offset_from(litPtr)
                    as std::ffi::c_long as usize;
                if leftoverLit_0 != 0 {
                    if leftoverLit_0 > oend.offset_from(op) as std::ffi::c_long as usize
                    {
                        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int)
                            as usize;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit_0);
                    (*sequence_1)
                        .litLength = ((*sequence_1).litLength)
                        .wrapping_sub(leftoverLit_0);
                    op = op.offset(leftoverLit_0 as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer)
                    .as_mut_ptr()
                    .offset(
                        (if 64 as std::ffi::c_int
                            > (if ((1 as std::ffi::c_int) << 16)
                                < (128 as std::ffi::c_int) << 10
                            {
                                (1 as std::ffi::c_int) << 16
                            } else {
                                (128 as std::ffi::c_int) << 10
                            })
                        {
                            64 as std::ffi::c_int
                        } else {
                            (if ((1 as std::ffi::c_int) << 16)
                                < (128 as std::ffi::c_int) << 10
                            {
                                (1 as std::ffi::c_int) << 16
                            } else {
                                (128 as std::ffi::c_int) << 10
                            })
                        }) as isize,
                    );
                (*dctx).litBufferLocation = ZSTD_not_in_dst;
                let oneSeqSize_1 = ZSTD_execSequence(
                    op,
                    oend,
                    *sequence_1,
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize_1) {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
            } else {
                let oneSeqSize_2 = if (*dctx).litBufferLocation as std::ffi::c_uint
                    == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
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
                if ERR_isError(oneSeqSize_2) {
                    return oneSeqSize_2;
                }
                op = op.offset(oneSeqSize_2 as isize);
            }
            seqNb += 1;
            seqNb;
        }
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTD_REP_NUM as u32 {
            (*dctx).entropy.rep[i_0 as usize] = seqState.prevOffset[i_0 as usize] as u32;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as std::ffi::c_long as usize;
        RETURN_ERROR_IF!(lastLLSize > oend.offset_from(op) as std::ffi::c_long as usize, ZSTD_error_dstSize_tooSmall);
        if !op.is_null() {
            libc::memmove(op.cast(), litPtr.cast(), lastLLSize);
            op = op.offset(lastLLSize as isize);
        }
        litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
        litBufferEnd = ((*dctx).litExtraBuffer)
            .as_mut_ptr()
            .offset(
                (if 64 as std::ffi::c_int
                    > (if ((1 as std::ffi::c_int) << 16)
                        < (128 as std::ffi::c_int) << 10
                    {
                        (1 as std::ffi::c_int) << 16
                    } else {
                        (128 as std::ffi::c_int) << 10
                    })
                {
                    64 as std::ffi::c_int
                } else {
                    (if ((1 as std::ffi::c_int) << 16)
                        < (128 as std::ffi::c_int) << 10
                    {
                        (1 as std::ffi::c_int) << 16
                    } else {
                        (128 as std::ffi::c_int) << 10
                    })
                }) as isize,
            );
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr);
    RETURN_ERROR_IF!(lastLLSize_0 > oend.offset_from(op), ZSTD_error_dstSize_tooSmall);
    if !op.is_null() {
        libc::memmove(op.cast(), litPtr.cast(), lastLLSize_0 as usize);
        op = op.offset(lastLLSize_0);
    }
    return op.offset_from(ostart) as usize;
}
pub const STORED_SEQS: std::ffi::c_int = 8;
pub const STORED_SEQS_MASK: std::ffi::c_int = STORED_SEQS - 1;
pub const ADVANCED_SEQS: std::ffi::c_int = STORED_SEQS;
unsafe extern "C" fn ZSTD_decompressSequencesLong_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: usize,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> usize {
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
    mut curPtr: *mut std::ffi::c_void,
    mut virtualStart: *const std::ffi::c_void,
) -> usize {
    return (curPtr as *mut std::ffi::c_char)
        .offset_from(virtualStart as *const std::ffi::c_char) as std::ffi::c_long
        as usize;
}
unsafe extern "C" fn ZSTD_getOffsetInfo(
    mut offTable: *const ZSTD_seqSymbol,
    mut nbSeq: std::ffi::c_int,
) -> ZSTD_OffsetInfo {
    let mut info = {
        let mut init = ZSTD_OffsetInfo {
            longOffsetShare: 0,
            maxNbAdditionalBits: 0,
        };
        init
    };
    if nbSeq != 0 {
        let mut ptr = offTable as *const std::ffi::c_void;
        let tableLog = (*(ptr as *const ZSTD_seqSymbol_header)
            .offset(0))
            .tableLog;
        let mut table = offTable.offset(1);
        let max = ((1 as std::ffi::c_int) << tableLog) as u32;
        let mut u: u32 = 0;
        u = 0;
        while u < max {
            info
                .maxNbAdditionalBits = std::cmp::max(
                info.maxNbAdditionalBits, (*table.offset(u as isize)).nbAdditionalBits as std::ffi::c_uint
            );
            if (*table.offset(u as isize)).nbAdditionalBits as std::ffi::c_int
                > 22
            {
                info
                    .longOffsetShare = (info.longOffsetShare)
                    .wrapping_add(1);
            }
            u = u.wrapping_add(1);
            u;
        }
        info.longOffsetShare <<= (OffFSELog as u32).wrapping_sub(tableLog);
    }
    return info;
}
unsafe extern "C" fn ZSTD_maxShortOffset() -> usize {
    if MEM_64bits {
        return -(1 as std::ffi::c_int) as usize
    } else {
        let maxOffbase = (1_usize
            << ((if MEM_32bits {
                STREAM_ACCUMULATOR_MIN_32
            } else {
                STREAM_ACCUMULATOR_MIN_64
            }) as u32)
                .wrapping_add(1))
            .wrapping_sub(1);
        let maxOffset = maxOffbase.wrapping_sub(ZSTD_REP_NUM as usize);
        return maxOffset;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock_internal(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    streaming: streaming_operation,
) -> usize {
    let mut ip = src as *const u8;
    RETURN_ERROR_IF!(srcSize > ZSTD_blockSizeMax(dctx), ZSTD_error_srcSize_wrong);
    let litCSize = ZSTD_decodeLiteralsBlock(
        dctx,
        src,
        srcSize,
        dst,
        dstCapacity,
        streaming,
    );
    if ERR_isError(litCSize) {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = srcSize.wrapping_sub(litCSize);
    let blockSizeMax = if dstCapacity < ZSTD_blockSizeMax(dctx) {
        dstCapacity
    } else {
        ZSTD_blockSizeMax(dctx)
    };
    let totalHistorySize = ZSTD_totalHistorySize(
        ZSTD_maybeNullPtrAdd(dst, blockSizeMax as ptrdiff_t),
        (*dctx).virtualStart as *const u8 as *const std::ffi::c_void,
    );
    let mut isLongOffset = (MEM_32bits
        && totalHistorySize > ZSTD_maxShortOffset()) as std::ffi::c_int
        as ZSTD_longOffset_e;
    let mut usePrefetchDecoder = (*dctx).ddictIsCold;
    let mut nbSeq: std::ffi::c_int = 0;
    let seqHSize = ZSTD_decodeSeqHeaders(
        dctx,
        &mut nbSeq,
        ip as *const std::ffi::c_void,
        srcSize,
    );
    if ERR_isError(seqHSize) {
        return seqHSize;
    }
    ip = ip.offset(seqHSize as isize);
    srcSize = srcSize.wrapping_sub(seqHSize);
    RETURN_ERROR_IF!((dst.is_null() || dstCapacity == 0)
        && nbSeq > 0, ZSTD_error_dstSize_tooSmall);
    RETURN_ERROR_IF!(MEM_64bits
        && ::core::mem::size_of::<usize>()
            == ::core::mem::size_of::<*mut std::ffi::c_void>()
        && (-(1 as std::ffi::c_int) as usize).wrapping_sub(dst as usize)
            < ((1 as std::ffi::c_int) << 20) as usize, ZSTD_error_dstSize_tooSmall);
    if isLongOffset as std::ffi::c_uint != 0
        || usePrefetchDecoder == 0
            && totalHistorySize
                > ((1 as std::ffi::c_uint) << 24) as usize
            && nbSeq > 8
    {
        let info = ZSTD_getOffsetInfo((*dctx).OFTptr, nbSeq);
        if isLongOffset as std::ffi::c_uint != 0
            && info.maxNbAdditionalBits
                <= (if MEM_32bits {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as u32
        {
            isLongOffset = ZSTD_lo_isRegularOffset;
        }
        if usePrefetchDecoder == 0 {
            let minShare = (if MEM_64bits {
                7 as std::ffi::c_int
            } else {
                20 as std::ffi::c_int
            }) as u32;
            usePrefetchDecoder = (info.longOffsetShare >= minShare) as std::ffi::c_int;
        }
    }
    (*dctx).ddictIsCold = 0;
    if usePrefetchDecoder != 0 {
        return ZSTD_decompressSequencesLong(
            dctx,
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        );
    }
    if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
    {
        return ZSTD_decompressSequencesSplitLitBuffer(
            dctx,
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        )
    } else {
        return ZSTD_decompressSequences(
            dctx,
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_checkContinuity(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *const std::ffi::c_void,
    mut dstSize: usize,
) {
    if dst != (*dctx).previousDstEnd && dstSize > 0 {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx)
            .virtualStart = (dst as *const std::ffi::c_char)
            .offset(
                -(((*dctx).previousDstEnd as *const std::ffi::c_char)
                    .offset_from((*dctx).prefixStart as *const std::ffi::c_char)
                    as std::ffi::c_long as isize),
            ) as *const std::ffi::c_void;
        (*dctx).prefixStart = dst;
        (*dctx).previousDstEnd = dst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock_deprecated(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut dSize: usize = 0;
    (*dctx).isFrameDecompression = 0;
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    dSize = ZSTD_decompressBlock_internal(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        not_streaming,
    );
    FORWARD_IF_ERROR!(dSize, "");
    (*dctx)
        .previousDstEnd = (dst as *mut std::ffi::c_char).offset(dSize as isize)
        as *const std::ffi::c_void;
    return dSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    return ZSTD_decompressBlock_deprecated(dctx, dst, dstCapacity, src, srcSize);
}
