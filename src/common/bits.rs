use crate::common::mem::*;

#[inline]
pub fn ZSTD_countTrailingZeros32(val: u32) -> u32 {
    val.trailing_zeros()
}
#[inline]
pub fn ZSTD_countLeadingZeros32(val: u32) -> u32 {
    val.leading_zeros()
}

#[inline]
pub fn ZSTD_countTrailingZeros64(val: u64) -> u32 {
    val.trailing_zeros()
}
#[inline]
pub fn ZSTD_countLeadingZeros64(val: u64) -> u32 {
    val.leading_zeros()
}

#[inline]
pub fn ZSTD_NbCommonBytes(val: usize) -> u32 {
    if MEM_isLittleEndian {
        if MEM_64bits {
            ZSTD_countTrailingZeros64(val as u64) >> 3
        } else {
            ZSTD_countTrailingZeros32(val as u32) >> 3
        }
    } else {
        if MEM_64bits {
            ZSTD_countLeadingZeros64(val as u64) >> 3
        } else {
            ZSTD_countLeadingZeros32(val as u32) >> 3
        }
    }
}

#[inline]
pub fn ZSTD_highbit32(val: u32) -> u32 {
    debug_assert!(val != 0);
    31 - ZSTD_countLeadingZeros32(val)
}

#[inline]
pub fn ZSTD_rotateRight_U64(value: u64, count: u32) -> u64 {
    debug_assert!(count < 64);
    value.rotate_right(count)
}
#[inline]
pub fn ZSTD_rotateRight_U32(value: u32, count: u32) -> u32 {
    debug_assert!(count < 32);
    value.rotate_right(count)
}
#[inline]
pub fn ZSTD_rotateRight_U16(value: u16, count: u32) -> u16 {
    debug_assert!(count < 16);
    value.rotate_right(count)
}
