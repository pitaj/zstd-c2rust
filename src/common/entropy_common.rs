use ::libc;
extern "C" {
    fn FSE_decompress_wksp_bmi2(
        dst: *mut std::ffi::c_void,
        dstCapacity: usize,
        cSrc: *const std::ffi::c_void,
        cSrcSize: usize,
        maxLog: u32,
        workSpace: *mut std::ffi::c_void,
        wkspSize: usize,
        bmi2: i32,
    ) -> usize;
}
pub type unalign32 = u32;
use crate::common::error::*;
pub type C2RustUnnamed = u32;
pub const HUF_flags_disableFast: C2RustUnnamed = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed = 2;
pub const HUF_flags_bmi2: C2RustUnnamed = 1;
use crate::common::mem::*;
pub const FSE_VERSION_MAJOR: i32 = 0;
pub const FSE_VERSION_MINOR: i32 = 9;
pub const FSE_VERSION_RELEASE: i32 = 0;
pub const FSE_VERSION_NUMBER: i32 = FSE_VERSION_MAJOR
    * 100 as i32 * 100 as i32
    + FSE_VERSION_MINOR * 100 as i32 + FSE_VERSION_RELEASE;
pub const FSE_MIN_TABLELOG: i32 = 5;
pub const FSE_TABLELOG_ABSOLUTE_MAX: i32 = 15;
use crate::common::bits::*;
pub const HUF_TABLELOG_MAX: i32 = 12;
#[no_mangle]
pub unsafe extern "C" fn FSE_versionNumber() -> u32 {
    return FSE_VERSION_NUMBER as u32;
}
#[inline(always)]
unsafe extern "C" fn FSE_readNCount_body(
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
    let mut previous0: i32 = 0;
    if hbSize < 8 {
        let mut buffer: [std::ffi::c_char; 8] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        libc::memcpy(buffer, headerBuffer, (hbSize) as usize);
        let countSize = FSE_readNCount(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            buffer.as_mut_ptr() as *const std::ffi::c_void,
            ::core::mem::size_of::<[std::ffi::c_char; 8]>(),
        );
        if ERR_isError(countSize) {
            return countSize;
        }
        RETURN_ERROR_IF!(countSize > hbSize, ZSTD_error_corruption_detected);
        return countSize;
    }
    libc::memset(
        normalizedCounter as *mut std::ffi::c_void,
        0,
        ((*maxSVPtr).wrapping_add(1)
            as std::ffi::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<i16>(),
            ) as usize,
    );
    bitStream = MEM_readLE32(ip as *const std::ffi::c_void);
    nbBits = (bitStream & 0xf as i32 as u32)
        .wrapping_add(FSE_MIN_TABLELOG as u32) as i32;
    RETURN_ERROR_IF!(nbBits > FSE_TABLELOG_ABSOLUTE_MAX, ZSTD_error_tableLog_tooLarge);
    bitStream >>= 4;
    bitCount = 4;
    *tableLogPtr = nbBits as u32;
    remaining = ((1 as i32) << nbBits) + 1;
    threshold = (1 as i32) << nbBits;
    nbBits += 1;
    nbBits;
    loop {
        if previous0 != 0 {
            let mut repeats = (ZSTD_countTrailingZeros32(
                !bitStream | 0x80000000 as u32,
            ) >> 1) as i32;
            while repeats >= 12 {
                charnum = charnum
                    .wrapping_add(
                        (3 as i32 * 12 as i32)
                            as u32,
                    );
                if LIKELY!(ip <= iend - 7) != 0 {
                    ip = ip.offset(3);
                } else {
                    bitCount
                        -= (8 as std::ffi::c_long
                            * iend
                                .offset(-7_isize)
                                .offset_from(ip) as std::ffi::c_long) as i32;
                    bitCount &= 31;
                    ip = iend.offset(-4_isize);
                }
                bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
                repeats = (ZSTD_countTrailingZeros32(
                    !bitStream | 0x80000000 as u32,
                ) >> 1) as i32;
            }
            charnum = charnum
                .wrapping_add((3 as i32 * repeats) as u32);
            bitStream >>= 2 * repeats;
            bitCount += 2 as i32 * repeats;
            charnum = charnum.wrapping_add(bitStream & 3_u32);
            bitCount += 2;
            if charnum >= maxSV1 {
                break;
            }
            if LIKELY!(ip <= iend - 7) != 0
                || ip.offset((bitCount >> 3) as isize)
                    <= iend.offset(-4_isize)
            {
                ip = ip.offset((bitCount >> 3) as isize);
                bitCount &= 7;
            } else {
                bitCount
                    -= (8 as std::ffi::c_long
                        * iend.offset(-4_isize).offset_from(ip)
                            as std::ffi::c_long) as i32;
                bitCount &= 31;
                ip = iend.offset(-4_isize);
            }
            bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
        }
        let max = 2 as i32 * threshold - 1 as i32 - remaining;
        let mut count: i32 = 0;
        if (bitStream & (threshold - 1 as i32) as u32) < max as u32 {
            count = (bitStream & (threshold - 1 as i32) as u32)
                as i32;
            bitCount += nbBits - 1;
        } else {
            count = (bitStream
                & (2 as i32 * threshold - 1 as i32) as u32)
                as i32;
            if count >= threshold {
                count -= max;
            }
            bitCount += nbBits;
        }
        count -= 1;
        count;
        if count >= 0 {
            remaining -= count;
        } else {
            remaining += count;
        }
        let fresh0 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh0 as isize) = count as i16;
        previous0 = (count == 0) as i32;
        if remaining < threshold {
            if remaining <= 1 {
                break;
            }
            nbBits = (ZSTD_highbit32(remaining as u32))
                .wrapping_add(1)
                as i32;
            threshold = (1 as i32) << nbBits - 1;
        }
        if charnum >= maxSV1 {
            break;
        }
        if LIKELY!(ip <= iend - 7) != 0
            || ip.offset((bitCount >> 3) as isize)
                <= iend.offset(-4_isize)
        {
            ip = ip.offset((bitCount >> 3) as isize);
            bitCount &= 7;
        } else {
            bitCount
                -= (8 as std::ffi::c_long
                    * iend.offset(-4_isize).offset_from(ip)
                        as std::ffi::c_long) as i32;
            bitCount &= 31;
            ip = iend.offset(-4_isize);
        }
        bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
    }
    RETURN_ERROR_IF!(remaining != 1, ZSTD_error_corruption_detected);
    RETURN_ERROR_IF!(charnum > maxSV1, ZSTD_error_maxSymbolValue_tooSmall);
    RETURN_ERROR_IF!(bitCount > 32, ZSTD_error_corruption_detected);
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset((bitCount + 7 as i32 >> 3) as isize);
    return ip.offset_from(istart) as std::ffi::c_long as usize;
}
unsafe extern "C" fn FSE_readNCount_body_default(
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
unsafe extern "C" fn FSE_readNCount_body_bmi2(
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
#[no_mangle]
pub unsafe extern "C" fn FSE_readNCount_bmi2(
    mut normalizedCounter: *mut i16,
    mut maxSVPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: usize,
    mut bmi2: bool,
) -> usize {
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
#[no_mangle]
pub unsafe extern "C" fn FSE_readNCount(
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
        0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readStats(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut wksp: [u32; 219] = [0; 219];
    return HUF_readStats_wksp(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        wksp.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 219]>(),
        0,
    );
}
#[inline(always)]
unsafe extern "C" fn HUF_readStats_body(
    mut huffWeight: *mut u8,
    mut hwSize: usize,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut bmi2: i32,
) -> usize {
    let mut weightTotal: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: usize = 0;
    let mut oSize: usize = 0;
    RETURN_ERROR_IF!(srcSize == 0, ZSTD_error_srcSize_wrong);
    iSize = *ip.offset(0) as usize;
    if iSize >= 128 {
        oSize = iSize.wrapping_sub(127);
        iSize = oSize.wrapping_add(1)
            / 2;
        RETURN_ERROR_IF!(iSize.wrapping_add(1) > srcSize, ZSTD_error_srcSize_wrong);
        RETURN_ERROR_IF!(oSize >= hwSize, ZSTD_error_corruption_detected);
        ip = ip.offset(1);
        let mut n: u32 = 0;
        n = 0;
        while (n as usize) < oSize {
            *huffWeight
                .offset(
                    n as isize,
                ) = (*ip.offset((n / 2_u32) as isize)
                as i32 >> 4) as u8;
            *huffWeight
                .offset(
                    n.wrapping_add(1) as isize,
                ) = (*ip.offset((n / 2_u32) as isize)
                as i32 & 15 as i32) as u8;
            n = n.wrapping_add(2);
        }
    } else {
        RETURN_ERROR_IF!(iSize.wrapping_add(1) > srcSize, ZSTD_error_srcSize_wrong);
        oSize = FSE_decompress_wksp_bmi2(
            huffWeight as *mut std::ffi::c_void,
            hwSize.wrapping_sub(1),
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
    libc::memset(
        rankStats as *mut std::ffi::c_void,
        0,
        ((12 as i32 + 1 as i32) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>())
            as usize,
    );
    weightTotal = 0;
    let mut n_0: u32 = 0;
    n_0 = 0;
    while (n_0 as usize) < oSize {
        RETURN_ERROR_IF!(*huffWeight.offset(n_0 as isize) as i32 > HUF_TABLELOG_MAX, ZSTD_error_corruption_detected);
        let ref mut fresh1 = *rankStats
            .offset(*huffWeight.offset(n_0 as isize) as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
        weightTotal = weightTotal
            .wrapping_add(
                ((1 as i32)
                    << *huffWeight.offset(n_0 as isize) as i32
                    >> 1) as u32,
            );
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    RETURN_ERROR_IF!(weightTotal == 0, ZSTD_error_corruption_detected);
    let tableLog = (ZSTD_highbit32(weightTotal))
        .wrapping_add(1);
    RETURN_ERROR_IF!(tableLog > HUF_TABLELOG_MAX as u32, ZSTD_error_corruption_detected);
    *tableLogPtr = tableLog;
    let total = ((1 as i32) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1 as i32) << ZSTD_highbit32(rest)) as u32;
    let lastWeight = (ZSTD_highbit32(rest))
        .wrapping_add(1);
    RETURN_ERROR_IF!(verif != rest, ZSTD_error_corruption_detected);
    *huffWeight.offset(oSize as isize) = lastWeight as u8;
    let ref mut fresh2 = *rankStats.offset(lastWeight as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
    *fresh2;
    RETURN_ERROR_IF!(*rankStats.offset(1) < 2
        || *rankStats.offset(1) & 1_u32
            != 0, ZSTD_error_corruption_detected);
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    return iSize.wrapping_add(1);
}
unsafe extern "C" fn HUF_readStats_body_default(
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
        0,
    );
}
unsafe extern "C" fn HUF_readStats_body_bmi2(
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
        1,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readStats_wksp(
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
    if flags & HUF_flags_bmi2 as i32 != 0 {
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
