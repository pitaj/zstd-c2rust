pub const fn ZSTD_FRAMEHEADERSIZE_MIN(format: ZSTD_format_e) {
    if format == ZSTD_f_zstd1 {
        6
    } else {
        2
    }
}

pub const fn ZSTD_FRAMEHEADERSIZE_PREFIX(format: ZSTD_format_e) -> usize {
    if format == ZSTD_f_zstd1 {
        5
    } else {
        1
    }
}

pub const MEM_32bits: bool = usize::BITS == 32;

/* *************************************
 *  Default constant
 ***************************************/
// #ifndef ZSTD_CLEVEL_DEFAULT
pub const ZSTD_CLEVEL_DEFAULT: std::ffi::c_int = 3;

/* *************************************
 *  Constants
 ***************************************/

/* All magic numbers are supposed read/written to/from files/memory using little-endian convention */
pub const ZSTD_MAGICNUMBER: u32 = 0xFD2FB528;    /* valid since v0.8.0 */
pub const ZSTD_MAGIC_DICTIONARY: u32 = 0xEC30A437;    /* valid since v0.7.0 */
pub const ZSTD_MAGIC_SKIPPABLE_START: u32 = 0x184D2A50;    /* all 16 values, from 0x184D2A50 to 0x184D2A5F, signal the beginning of a skippable frame */
pub const ZSTD_MAGIC_SKIPPABLE_MASK: u32 = 0xFFFFFFF0;

pub const ZSTD_BLOCKSIZELOG_MAX: u32 = 17;
pub const ZSTD_BLOCKSIZE_MAX: usize = 1_usize << ZSTD_BLOCKSIZELOG_MAX;

