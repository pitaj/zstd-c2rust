use std::mem::{size_of, size_of_val};

use crate::common::mem::*;
use crate::common::error::*;
use crate::common::fse_h::*;
use crate::common::huf_h::*;
use crate::common::bits::*;
use crate::zstd_h::*;

/*-**************************************************************
*  FSE NCount encoding-decoding
****************************************************************/
#[inline(always)]
unsafe fn FSE_readNCount_body(
    mut normalizedCounter: *mut i16,
    mut maxSVPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: usize,
) -> usize {
    let istart = headerBuffer as *const u8;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: i32 = 0;
    let mut remaining: i32 = 0;
    let mut threshold: i32 = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: i32 = 0;
    let mut charnum: u32 = 0;
    let maxSV1 = (*maxSVPtr).wrapping_add(1);
    let mut previous0: bool = false;
    if hbSize < 8 {
        /* This function only works when hbSize >= 8 */
        let mut buffer: [std::ffi::c_char; 8] = [0; 8];
        libc::memcpy(buffer.as_mut_ptr().cast(), headerBuffer, hbSize);
        let countSize = FSE_readNCount(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            buffer.as_mut_ptr().cast(),
            size_of_val(&buffer),
        );
        if ERR_isError(countSize) {
            return countSize;
        }
        if countSize > hbSize { return ERROR(ZSTD_error_corruption_detected); }
        return countSize;
    }
    debug_assert!(hbSize >= 8);

    /* init */
    libc::memset(normalizedCounter.cast(), 0,
        ((*maxSVPtr + 1) as usize) * size_of::<i16>()
    ); /* all symbols not present in NCount have a frequency of 0 */
    bitStream = MEM_readLE32(ip as *const std::ffi::c_void);
    nbBits = (bitStream & 0xf).wrapping_add(FSE_MIN_TABLELOG) as i32; /* extract tableLog */
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX as i32 { return ERROR(ZSTD_error_tableLog_tooLarge); }
    bitStream >>= 4;
    bitCount = 4;
    *tableLogPtr = nbBits as u32;
    remaining = (1_i32 << nbBits) + 1;
    threshold = 1_i32 << nbBits;
    nbBits += 1;

    loop {
        if previous0 {
            /* Count the number of repeats. Each time the
             * 2-bit repeat code is 0b11 there is another
             * repeat.
             * Avoid UB by setting the high bit to 1.
             */
            let mut repeats = ZSTD_countTrailingZeros32(!bitStream | 0x80000000) >> 1;
            while repeats >= 12 {
                charnum += 3 * 12;
                if LIKELY!(ip <= iend.offset(-7)) {
                    ip = ip.offset(3);
                } else {
                    bitCount -= (8 * iend.offset(-7).offset_from(ip)) as i32;
                    bitCount &= 31;
                    ip = iend.offset(-4);
                }
                bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
                repeats = ZSTD_countTrailingZeros32(!bitStream | 0x80000000) >> 1;
            }
            charnum += 3 * repeats;
            bitStream >>= 2 * repeats;
            bitCount += 2 * (repeats as i32);

            /* Add the final repeat which isn't 0b11. */
            debug_assert!((bitStream & 3) < 3);
            charnum += (bitStream & 3);
            bitCount += 2;

            /* This is an error, but break and return an error
             * at the end, because returning out of a loop makes
             * it harder for the compiler to optimize.
             */
            if charnum >= maxSV1 {
                break;
            }

            /* We don't need to set the normalized count to 0
             * because we already memset the whole buffer to 0.
             */

            if LIKELY!(ip <= iend.offset(-7)) || 
                ip.offset((bitCount >> 3) as isize) <= iend.offset(-4)
            {
                debug_assert!((bitCount >> 3) <= 3); /* For first condition to work */
                ip = ip.offset((bitCount >> 3) as isize);
                bitCount &= 7;
            } else {
                bitCount -= (8 * iend.offset(-4_isize).offset_from(ip)) as i32;
                bitCount &= 31;
                ip = iend.offset(-4);
            }
            bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
        }

        let max = (2 * threshold - 1) - remaining;
        let mut count: i32 = 0;

        if (bitStream & (threshold - 1) as u32) < max as u32 {
            count = (bitStream as i32) & (threshold - 1);
            bitCount += nbBits - 1;
        } else {
            count = (bitStream & (2*threshold - 1) as u32) as i32;
            if count >= threshold {
                count -= max;
            }
            bitCount += nbBits;
        }

        count -= 1; /* extra accuracy */
        /* When it matters (small blocks), this is a
         * predictable branch, because we don't use -1.
         */
        if count >= 0 {
            remaining -= count;
        } else {
            debug_assert!(count == -1);
            remaining += count;
        }
        *normalizedCounter.offset(charnum as isize) = count as i16;
        charnum += 1;
        previous0 = count == 0;

        debug_assert!(threshold > 1);
        if remaining < threshold {
            /* This branch can be folded into the
             * threshold update condition because we
             * know that threshold > 1.
             */
            if remaining <= 1 {
                break;
            }
            nbBits = (ZSTD_highbit32(remaining as u32) + 1) as i32;
            threshold = 1_i32 << (nbBits - 1);
        }
        if charnum >= maxSV1 {
            break;
        }

        if LIKELY!(ip <= iend.offset(-7))
            || ip.offset((bitCount >> 3) as isize) <= iend.offset(-4)
        {
            ip = ip.offset((bitCount >> 3) as isize);
            bitCount &= 7;
        } else {
            bitCount -= (8 * iend.offset(-4).offset_from(ip)) as i32;
            bitCount &= 31;
            ip = iend.offset(-4);
        }
        bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
    }
    if remaining != 1 { return ERROR(ZSTD_error_corruption_detected); }
    /* Only possible when there are too many zeros. */
    if charnum > maxSV1 { return ERROR(ZSTD_error_maxSymbolValue_tooSmall); }
    if bitCount > 32 { return ERROR(ZSTD_error_corruption_detected); }
    *maxSVPtr = charnum.wrapping_sub(1);

    ip = ip.offset(((bitCount + 7) >> 3) as isize);
    return ip.offset_from(istart) as usize;
}

/* Avoids the FORCE_INLINE of the _body() function. */
unsafe fn FSE_readNCount_body_default(
    mut normalizedCounter: *mut i16,
    mut maxSVPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: usize,
) -> usize {
    return FSE_readNCount_body(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    );
}

// TODO #if DYNAMIC_BMI2
unsafe fn FSE_readNCount_body_bmi2(
    mut normalizedCounter: *mut i16,
    mut maxSVPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: usize,
) -> usize {
    return FSE_readNCount_body(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    );
}

pub unsafe fn FSE_readNCount_bmi2(
    mut normalizedCounter: *mut i16,
    mut maxSVPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: usize,
    mut bmi2: bool,
) -> usize {
    // TODO #if DYNAMIC_BMI2
    if bmi2 {
        return FSE_readNCount_body_bmi2(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            headerBuffer,
            hbSize,
        );
    }
    return FSE_readNCount_body_default(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    );
}

pub unsafe fn FSE_readNCount(
    mut normalizedCounter: *mut i16,
    mut maxSVPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: usize,
) -> usize {
    return FSE_readNCount_bmi2(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
        false,
    );
}

/** HUF_readStats() :
    Read compact Huffman tree, saved by HUF_writeCTable().
    `huffWeight` is destination buffer.
    `rankStats` is assumed to be a table of at least HUF_TABLELOG_MAX U32.
    @return : size read from `src` , or an error Code .
    Note : Needed by HUF_readCTable() and HUF_readDTableX?() .
*/
pub unsafe fn HUF_readStats(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut wksp: [u32; HUF_READ_STATS_WORKSPACE_SIZE_U32] = [0; HUF_READ_STATS_WORKSPACE_SIZE_U32];
    return HUF_readStats_wksp(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        wksp.as_mut_ptr() as *mut std::ffi::c_void,
        size_of::<[u32; HUF_READ_STATS_WORKSPACE_SIZE_U32]>(),
        /* flags */ 0,
    );
}

#[inline(always)]
unsafe fn HUF_readStats_body(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut bmi2: bool,
) -> usize {
    let mut weightTotal: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: usize = 0;
    let mut oSize: usize = 0;

    if srcSize == 0 { return ERROR(ZSTD_error_srcSize_wrong); }
    iSize = *ip.offset(0) as usize;
    /* ZSTD_memset(huffWeight, 0, hwSize);   *//* is not necessary, even though some analyzer complain ... */

    if iSize >= 128 { /* special header */
        oSize = iSize.wrapping_sub(127);
        iSize = oSize.wrapping_add(1) / 2;
        if iSize.wrapping_add(1) > srcSize { return ERROR(ZSTD_error_srcSize_wrong); }
        if oSize >= hwSize { return ERROR(ZSTD_error_corruption_detected); }
        ip = ip.offset(1);
        let mut n: usize = 0;
        while n < oSize {
            *huffWeight.add(n) = *ip.add(n / 2) >> 4;
            *huffWeight.add(n+1) = *ip.add(n / 2) & 15;
            n = n.wrapping_add(2);
        }
    } else { /* header compressed with FSE (normal case) */
        if iSize+1 > srcSize { return ERROR(ZSTD_error_srcSize_wrong); }
        /* max (hwSize-1) values decoded, as last one is implied */
        oSize = FSE_decompress_wksp_bmi2(
            huffWeight as *mut std::ffi::c_void,
            hwSize - 1,
            ip.offset(1) as *const std::ffi::c_void,
            iSize,
            6,
            workSpace,
            wkspSize,
            bmi2,
        );
        if ERR_isError(oSize) {
            return oSize;
        }
    }

    /* collect weight stats */
    libc::memset(
        rankStats as *mut std::ffi::c_void,
        0,
        (HUF_TABLELOG_MAX + 1) as usize * size_of::<u32>()
    );
    weightTotal = 0;
    for n_0 in 0..oSize {
        if *huffWeight.add(n_0) as u32 > HUF_TABLELOG_MAX { return ERROR(ZSTD_error_corruption_detected); }
        *rankStats.offset(*huffWeight.add(n_0) as isize) += 1;
        weightTotal += (1_u32 << *huffWeight.add(n_0)) >> 1;
    }
    if weightTotal == 0 { return ERROR(ZSTD_error_corruption_detected); }

    /* get last non-null symbol weight (implied, total must be 2^n) */
    let tableLog = ZSTD_highbit32(weightTotal) + 1;
    if tableLog > HUF_TABLELOG_MAX { return ERROR(ZSTD_error_corruption_detected); }
    *tableLogPtr = tableLog;
    /* determine last weight */
    let total = 1_u32 << tableLog;
    let rest = total - weightTotal;
    let verif = 1_u32 << ZSTD_highbit32(rest);
    let lastWeight = ZSTD_highbit32(rest) + 1;
    if verif != rest { return ERROR(ZSTD_error_corruption_detected); } /* last value must be a clean power of 2 */
    *huffWeight.add(oSize) = lastWeight as u8;
    *rankStats.offset(lastWeight as isize) += 1;

    /* check tree construction validity */
    if *rankStats.offset(1) < 2 || *rankStats.offset(1) & 1_u32 != 0 {
        return ERROR(ZSTD_error_corruption_detected); /* by construction : at least 2 elts of rank 1, must be even */
    }

    /* results */
    *nbSymbolsPtr = (oSize+1) as u32;
    return iSize+1;
}

/* Avoids the FORCE_INLINE of the _body() function. */
unsafe fn HUF_readStats_body_default(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    return HUF_readStats_body(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        workSpace,
        wkspSize,
        false,
    );
}

// TODO #if DYNAMIC_BMI2
unsafe fn HUF_readStats_body_bmi2(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    return HUF_readStats_body(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        workSpace,
        wkspSize,
        true,
    );
}

pub unsafe fn HUF_readStats_wksp(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut flags: i32,
) -> usize {
    // TODO #if DYNAMIC_BMI2
    if flags & HUF_flags_bmi2 != 0 {
        return HUF_readStats_body_bmi2(
            huffWeight,
            hwSize,
            rankStats,
            nbSymbolsPtr,
            tableLogPtr,
            src,
            srcSize,
            workSpace,
            wkspSize,
        );
    }
    return HUF_readStats_body_default(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        workSpace,
        wkspSize,
    );
}
