use crate::zstd_h::*;
use crate::common::mem::*;
use crate::compress::zstd_compress_internal::ZSTD_window_hasExtDict;

/* this module contains definitions which must be identical
 * across compression, decompression and dictBuilder.
 * It also contains a few functions useful to at least 2 of them
 * and which benefit from being inlined */

/*-*************************************
*  shared macros
***************************************/
macro_rules! BOUNDED {
    ($min:expr, $val:expr, $max:expr) => {
        $val.clamp($min, $max)
    }
}

/*-*************************************
*  Common constants
***************************************/
pub const ZSTD_OPT_NUM: u32 = 1_u32 << 12;

pub const ZSTD_REP_NUM: u32 = 3; /* number of repcodes */
pub const repStartValue: [u32; ZSTD_REP_NUM as usize] = [1, 4, 8];

pub const fn KB(n: usize) -> usize { n*(1_usize << 10) }
pub const fn MB(n: usize) -> usize { n*(1_usize << 20) }
pub const fn GB(n: usize) -> usize { n*(1_usize << 30) }

// #define BIT7 128
// #define BIT6  64
// #define BIT5  32
// #define BIT4  16
// #define BIT1   2
// #define BIT0   1

pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: std::ffi::c_int = 10;
pub const ZSTD_fcs_fieldSize: [usize; 4] = [0, 2, 4, 8];
pub const ZSTD_did_fieldSize: [usize; 4] = [0, 1, 2, 4];

pub const ZSTD_FRAMEIDSIZE: usize = 4;   /* magic number size */

pub const ZSTD_BLOCKHEADERSIZE: usize = 3;   /* C standard doesn't allow `static const` variable to be init using another `static const` variable */
#[deprecated]
pub static ZSTD_blockHeaderSize: usize = ZSTD_BLOCKHEADERSIZE;

pub type blockType_e = std::ffi::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;

// #define ZSTD_FRAMECHECKSUMSIZE 4

// #define MIN_SEQUENCES_SIZE 1 /* nbSeq==0 */
pub const MIN_CBLOCK_SIZE: usize = 1 /*litCSize*/ + 1 /* RLE or RAW */;   /* for a non-null block */
// #define MIN_LITERALS_FOR_4_STREAMS 6

pub type SymbolEncodingType_e = std::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;

pub const LONGNBSEQ: usize = 0x7F00;

pub const MINMATCH: u32 = 3;

/// std::cmp::max but const
pub const fn const_max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

pub const Litbits: u32 = 8;
pub const LitHufLog: u32 = 11;
pub const MaxLit: u32 = (1_u32 << Litbits) - 1;
pub const MaxML: u32 = 52;
pub const MaxLL: u32 = 35;
pub const DefaultMaxOff: u32 = 28;
pub const MaxOff: u32 = 31;
pub const MaxSeq: u32 = const_max(MaxLL, MaxML);   /* Assumption : MaxOff < MaxLL,MaxML */
pub const _: () = assert!(MaxOff < MaxSeq);
pub const MLFSELog: u32 = 9;
pub const LLFSELog: u32 = 9;
pub const OffFSELog: u32 = 8;
pub const MaxFSELog: u32 = const_max(const_max(MLFSELog, LLFSELog), OffFSELog);
pub const MaxMLBits: u32 = 16;
pub const MaxLLBits: u32 = 16;

// #define ZSTD_MAX_HUF_HEADER_SIZE 128 /* header + <= 127 byte tree description */
/* Each table cannot take more than #symbols * FSELog bits */
// #define ZSTD_MAX_FSE_HEADERS_SIZE (((MaxML + 1) * MLFSELog + (MaxLL + 1) * LLFSELog + (MaxOff + 1) * OffFSELog + 7) / 8)

#[rustfmt::skip]
pub const LL_bits: [u8; (MaxLL+1) as usize] = [
     0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
     1, 1, 1, 1, 2, 2, 3, 3,
     4, 6, 7, 8, 9,10,11,12,
    13,14,15,16
];
#[rustfmt::skip]
pub const LL_defaultNorm: [i16; (MaxLL+1) as usize] = [
     4, 3, 2, 2, 2, 2, 2, 2,
     2, 2, 2, 2, 2, 1, 1, 1,
     2, 2, 2, 2, 2, 2, 2, 2,
     2, 3, 2, 1, 1, 1, 1, 1,
    -1,-1,-1,-1
];
pub const LL_DEFAULTNORMLOG: u32 = 6;  /* for static allocation */
#[deprecated]
pub static LL_defaultNormLog: u32 = LL_DEFAULTNORMLOG;

#[rustfmt::skip]
pub const ML_bits: [u8; (MaxML+1) as usize] = [
     0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
     1, 1, 1, 1, 2, 2, 3, 3,
     4, 4, 5, 7, 8, 9,10,11,
    12,13,14,15,16
];
#[rustfmt::skip]
pub const ML_defaultNorm: [i16; (MaxML+1) as usize] = [
     1, 4, 3, 2, 2, 2, 2, 2,
     2, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 1, 1, 1,-1,-1,
    -1,-1,-1,-1,-1
];
pub const ML_DEFAULTNORMLOG: u32 = 6;  /* for static allocation */
#[deprecated]
pub static ML_defaultNormLog: u32 = ML_DEFAULTNORMLOG;

#[rustfmt::skip]
pub const OF_defaultNorm: [i16; (DefaultMaxOff+1) as usize] = [
     1, 1, 1, 1, 1, 1, 2, 2,
     2, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 1, 1, 1, 1, 1,
    -1,-1,-1,-1,-1
];
pub const OF_DEFAULTNORMLOG: u32 = 5;  /* for static allocation */
#[deprecated]
pub static OF_defaultNormLog: u32 = OF_DEFAULTNORMLOG;


/*-*******************************************
*  Shared functions to include for inlining
*********************************************/
#[inline]
pub unsafe fn ZSTD_copy8(
    dst: *mut std::ffi::c_void,
    src: *const std::ffi::c_void,
) {
    libc::memcpy(dst, src, 8);
}

#[doc(hidden)]
#[macro_export]
macro_rules! __COPY8 {
    ($d:expr, $s:expr) => {{
        $crate::common::zstd_internal_h::ZSTD_copy8($d.cast(), $s.cast());
        $d = $d.byte_offset(8);
        $s = $s.byte_offset(8);
    }}
}
pub use crate::__COPY8 as COPY8;

/* Need to use memmove here since the literal buffer can now be located within
   the dst buffer. In circumstances where the op "catches up" to where the
   literal buffer is, there can be partial overlaps in this call on the final
   copy if the literal is being shifted by less than 16 bytes. */
#[inline]
pub unsafe fn ZSTD_copy16(
    dst: *mut std::ffi::c_void,
    src: *const std::ffi::c_void,
) {
    libc::memmove(dst, src, 16);

    // TODO
    // #if defined(ZSTD_ARCH_ARM_NEON)
    //     vst1q_u8((uint8_t*)dst, vld1q_u8((const uint8_t*)src));
    // #elif defined(ZSTD_ARCH_X86_SSE2)
    //     _mm_storeu_si128((__m128i*)dst, _mm_loadu_si128((const __m128i*)src));
    // #elif defined(__clang__)
    //     ZSTD_memmove(dst, src, 16);
    // #else
    //     /* ZSTD_memmove is not inlined properly by gcc */
    //     BYTE copy16_buf[16];
    //     ZSTD_memcpy(copy16_buf, src, 16);
    //     ZSTD_memcpy(dst, copy16_buf, 16);
    // #endif
    // _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}

#[doc(hidden)]
#[macro_export]
macro_rules! __COPY16 {
    ($d:expr, $s:expr) => {{
        $crate::common::zstd_internal_h::ZSTD_copy16($d.cast(), $s.cast());
        $d = $d.byte_offset(16);
        $s = $s.byte_offset(16);
    }}
}
pub use crate::__COPY16 as COPY16;

pub const WILDCOPY_OVERLENGTH: usize = 32;
pub const WILDCOPY_VECLEN: isize = 16;

pub type ZSTD_overlap_e = std::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;

/** ZSTD_wildcopy() :
 *  Custom version of ZSTD_memcpy(), can over read/write up to WILDCOPY_OVERLENGTH bytes (if length==0)
 *  @param ovtype controls the overlap detection
 *         - ZSTD_no_overlap: The source and destination are guaranteed to be at least WILDCOPY_VECLEN bytes apart.
 *         - ZSTD_overlap_src_before_dst: The src and dst may overlap, but they MUST be at least 8 bytes apart.
 *           The src buffer must be before the dst buffer.
 */
#[inline(always)]
pub unsafe fn ZSTD_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: usize,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = dst.byte_offset_from(src);
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.add(length);

    if ovtype == ZSTD_overlap_src_before_dst
        && diff < WILDCOPY_VECLEN as isize
    {
        /* Handle short offset copies. */
        loop {
            COPY8!(op, ip);
            if !(op < oend) {
                break;
            }
        }
    } else {
        debug_assert!(diff >= WILDCOPY_VECLEN || diff <= -WILDCOPY_VECLEN);
        /* Separate out the first COPY16() call because the copy length is
         * almost certain to be short, so the branches have different
         * probabilities. Since it is almost certain to be short, only do
         * one COPY16() in the first call. Then, do two calls per loop since
         * at that point it is more likely to have a high trip count.
         */
        ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        if 16 >= length {
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

#[inline]
pub unsafe fn ZSTD_limitCopy(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let length = std::cmp::min(dstCapacity, srcSize);
    if length > 0 {
        libc::memcpy(dst, src, length);
    }
    return length;
}

/* define "workspace is too large" as this number of times larger than needed */
pub const ZSTD_WORKSPACETOOLARGE_FACTOR: usize = 3;

/* when workspace is continuously too large
 * during at least this number of times,
 * context's memory usage is considered wasteful,
 * because it's sized to handle a worst case scenario which rarely happens.
 * In which case, resize it down to free some memory */
pub const ZSTD_WORKSPACETOOLARGE_MAXDURATION: usize = 128;

/* Controls whether the input/output buffer is buffered or stable. */
pub type ZSTD_bufferMode_e = std::ffi::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1; /* ZSTD_inBuffer/ZSTD_outBuffer is stable */
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0; /* Buffer the input/output */


/*-*******************************************
*  Private declarations
*********************************************/

/**
 * Contains the compressed frame size and an upper-bound for the decompressed frame size.
 * Note: before using `compressedSize`, check for errors using ZSTD_isError().
 *       similarly, before using `decompressedBound`, check for errors using:
 *          `decompressedBound != ZSTD_CONTENTSIZE_ERROR`
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameSizeInfo {
    pub nbBlocks: usize,
    pub compressedSize: usize,
    pub decompressedBound: std::ffi::c_ulonglong,
} /* decompress & legacy */

/* ZSTD_invalidateRepCodes() :
 * ensures next compression will not use repcodes from previous block.
 * Note : only works with regular variant;
 *        do not use with extDict variant ! */
pub use crate::compress::zstd_compress::ZSTD_invalidateRepCodes; /* zstdmt, adaptive_compression (shouldn't get this definition from here) */


#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: u32,
    pub origSize: u32,
}  /* declared here for decompress and fullbench */

/** ZSTD_getcBlockSize() :
 *  Provides the size of compressed block from block header `src` */
/*  Used by: decompress, fullbench */
pub use crate::decompress::zstd_decompress_block::ZSTD_getcBlockSize;

/** ZSTD_decodeSeqHeaders() :
 *  decode sequence header from src */
/*  Used by: zstd_decompress_block, fullbench */
pub use crate::decompress::zstd_decompress_block::ZSTD_decodeSeqHeaders;

// TODO?
// /**
//  * @returns true iff the CPU supports dynamic BMI2 dispatch.
//  */
// MEM_STATIC int ZSTD_cpuSupportsBmi2(void)
// {
//     ZSTD_cpuid_t cpuid = ZSTD_cpuid();
//     return ZSTD_cpuid_bmi1(cpuid) && ZSTD_cpuid_bmi2(cpuid);
// }
