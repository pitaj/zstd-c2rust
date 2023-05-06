use ::libc;
extern "C" {
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    fn FSE_decompress_wksp_bmi2(
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        cSrc: *const libc::c_void,
        cSrcSize: libc::size_t,
        maxLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
        bmi2: libc::c_int,
    ) -> libc::size_t;
}
pub type unalign32 = u32;
pub type ZSTD_ErrorCode = libc::c_uint;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_externalSequences_invalid: ZSTD_ErrorCode = 107;
pub const ZSTD_error_sequenceProducer_failed: ZSTD_ErrorCode = 106;
pub const ZSTD_error_srcBuffer_wrong: ZSTD_ErrorCode = 105;
pub const ZSTD_error_dstBuffer_wrong: ZSTD_ErrorCode = 104;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ZSTD_ErrorCode = 82;
pub const ZSTD_error_noForwardProgress_destFull: ZSTD_ErrorCode = 80;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ZSTD_ErrorCode = 50;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_combination_unsupported: ZSTD_ErrorCode = 41;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_literals_headerWrong: ZSTD_ErrorCode = 24;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type ERR_enum = ZSTD_ErrorCode;
pub type C2RustUnnamed = libc::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed = 2;
pub const HUF_flags_bmi2: C2RustUnnamed = 1;
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> u32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr);
    } else {
        return MEM_swap32(MEM_read32(memPtr));
    };
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> u32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: libc::size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    return (0).wrapping_sub(code) as ERR_enum;
}
unsafe extern "C" fn ERR_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0);
    return val.trailing_zeros() as i32 as libc::c_uint;
}
pub const FSE_VERSION_NUMBER: libc::c_int =
    FSE_VERSION_MAJOR * 100 * 100 + FSE_VERSION_MINOR * 100 + FSE_VERSION_RELEASE;
pub const FSE_VERSION_MAJOR: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0);
    return (31).wrapping_sub(ZSTD_countLeadingZeros32(val));
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0);
    return val.leading_zeros() as i32 as libc::c_uint;
}
pub const FSE_VERSION_MINOR: libc::c_int = 9 as libc::c_int;
pub const FSE_VERSION_RELEASE: libc::c_int = 0 as libc::c_int;
pub const FSE_MIN_TABLELOG: libc::c_int = 5 as libc::c_int;
pub const FSE_TABLELOG_ABSOLUTE_MAX: libc::c_int = 15 as libc::c_int;
pub const HUF_TABLELOG_MAX: libc::c_int = 12 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn FSE_versionNumber() -> libc::c_uint {
    return FSE_VERSION_NUMBER as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_isError(mut code: libc::size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn FSE_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_isError(mut code: libc::size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_getErrorName(mut code: libc::size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
#[inline(always)]
unsafe extern "C" fn FSE_readNCount_body(
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
    let maxSV1 = (*maxSVPtr).wrapping_add(1);
    let mut previous0 = 0 as libc::c_int;
    if hbSize < 8 {
        let mut buffer: [libc::c_char; 8] = [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0];
        libc::memcpy(
            buffer.as_mut_ptr() as *mut libc::c_void,
            headerBuffer,
            hbSize as libc::size_t,
        );
        let countSize = FSE_readNCount(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            buffer.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 8]>(),
        );
        if FSE_isError(countSize) != 0 {
            return countSize;
        }
        if countSize > hbSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        return countSize;
    }
    debug_assert!(hbSize >= 8);
    libc::memset(
        normalizedCounter as *mut libc::c_void,
        0 as libc::c_int,
        ((*maxSVPtr).wrapping_add(1) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>()) as libc::size_t,
    );
    bitStream = MEM_readLE32(ip as *const libc::c_void);
    nbBits = (bitStream & 0xf as libc::c_int as libc::c_uint)
        .wrapping_add(FSE_MIN_TABLELOG as libc::c_uint) as libc::c_int;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    bitStream >>= 4 as libc::c_int;
    bitCount = 4 as libc::c_int;
    *tableLogPtr = nbBits as libc::c_uint;
    remaining = ((1) << nbBits) + 1;
    threshold = (1) << nbBits;
    nbBits += 1;
    loop {
        if previous0 != 0 {
            let mut repeats = (ZSTD_countTrailingZeros32(!bitStream | 0x80000000 as libc::c_uint)
                >> 1 as libc::c_int) as libc::c_int;
            while repeats >= 12 {
                charnum = charnum.wrapping_add((3 as libc::c_int * 12) as libc::c_uint);
                if (ip <= iend.offset(-(7))) as libc::c_int as libc::c_long != 0 {
                    ip = ip.offset(3);
                } else {
                    bitCount -= (8 as libc::c_int as libc::c_long
                        * iend.offset(-(7)).offset_from(ip) as libc::c_long)
                        as libc::c_int;
                    bitCount &= 31;
                    ip = iend.offset(-(4));
                }
                bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
                repeats = (ZSTD_countTrailingZeros32(!bitStream | 0x80000000 as libc::c_uint)
                    >> 1 as libc::c_int) as libc::c_int;
            }
            charnum = charnum.wrapping_add((3 as libc::c_int * repeats) as libc::c_uint);
            bitStream >>= 2 as libc::c_int * repeats;
            bitCount += 2 as libc::c_int * repeats;
            debug_assert!((bitStream & 3) < 3);
            charnum = charnum.wrapping_add(bitStream & 3);
            bitCount += 2 as libc::c_int;
            if charnum >= maxSV1 {
                break;
            }
            if (ip <= iend.offset(-(7))) as libc::c_int as libc::c_long != 0
                || ip.offset((bitCount >> 3 as libc::c_int) as isize) <= iend.offset(-(4))
            {
                debug_assert!(bitCount >> 3 as libc::c_int <= 3);
                ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
                bitCount &= 7;
            } else {
                bitCount -= (8 as libc::c_int as libc::c_long
                    * iend.offset(-(4)).offset_from(ip) as libc::c_long)
                    as libc::c_int;
                bitCount &= 31;
                ip = iend.offset(-(4));
            }
            bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
        }
        let max = 2 as libc::c_int * threshold - 1 as libc::c_int - remaining;
        let mut count: libc::c_int = 0;
        if (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint) < max as u32 {
            count = (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
            bitCount += nbBits - 1 as libc::c_int;
        } else {
            count = (bitStream & (2 as libc::c_int * threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_int;
            if count >= threshold {
                count -= max;
            }
            bitCount += nbBits;
        }
        count -= 1;
        if count >= 0 {
            remaining -= count;
        } else {
            debug_assert!(count == -(1));
            remaining += count;
        }
        let fresh0 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh0 as isize) = count as libc::c_short;
        previous0 = (count == 0) as libc::c_int;
        debug_assert!(threshold > 1);
        if remaining < threshold {
            if remaining <= 1 {
                break;
            }
            nbBits = (ZSTD_highbit32(remaining as u32)).wrapping_add(1) as libc::c_int;
            threshold = (1) << nbBits - 1 as libc::c_int;
        }
        if charnum >= maxSV1 {
            break;
        }
        if (ip <= iend.offset(-(7))) as libc::c_int as libc::c_long != 0
            || ip.offset((bitCount >> 3 as libc::c_int) as isize) <= iend.offset(-(4))
        {
            ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
            bitCount &= 7;
        } else {
            bitCount -= (8 as libc::c_int as libc::c_long
                * iend.offset(-(4)).offset_from(ip) as libc::c_long)
                as libc::c_int;
            bitCount &= 31;
            ip = iend.offset(-(4));
        }
        bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
    }
    if remaining != 1 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    if charnum > maxSV1 {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as libc::size_t;
    }
    if bitCount > 32 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset((bitCount + 7 >> 3 as libc::c_int) as isize);
    return ip.offset_from(istart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn FSE_readNCount_body_default(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: libc::size_t,
) -> libc::size_t {
    return FSE_readNCount_body(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    );
}
unsafe extern "C" fn FSE_readNCount_body_bmi2(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: libc::size_t,
) -> libc::size_t {
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
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: libc::size_t,
    mut bmi2: libc::c_int,
) -> libc::size_t {
    if bmi2 != 0 {
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
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: libc::size_t,
) -> libc::size_t {
    return FSE_readNCount_bmi2(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readStats(
    mut huffWeight: *mut u8,
    mut hwSize: libc::size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let mut wksp: [u32; 219] = [0; 219];
    return HUF_readStats_wksp(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        wksp.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[u32; 219]>(),
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn HUF_readStats_body(
    mut huffWeight: *mut u8,
    mut hwSize: libc::size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut bmi2: libc::c_int,
) -> libc::size_t {
    let mut weightTotal: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: libc::size_t = 0;
    let mut oSize: libc::size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    iSize = *ip.offset(0) as libc::size_t;
    if iSize >= 128 {
        oSize = iSize.wrapping_sub(127);
        iSize = oSize.wrapping_add(1).wrapping_div(2);
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
        }
        if oSize >= hwSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        ip = ip.offset(1);
        let mut n: u32 = 0;
        n = 0 as libc::c_int as u32;
        while (n as libc::c_ulong) < oSize {
            *huffWeight.offset(n as isize) =
                (*ip.offset(n.wrapping_div(2) as isize) as libc::c_int >> 4 as libc::c_int) as u8;
            *huffWeight.offset(n.wrapping_add(1) as isize) =
                (*ip.offset(n.wrapping_div(2) as isize) as libc::c_int & 15) as u8;
            n = (n as libc::c_uint).wrapping_add(2) as u32 as u32;
        }
    } else {
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
        }
        oSize = FSE_decompress_wksp_bmi2(
            huffWeight as *mut libc::c_void,
            hwSize.wrapping_sub(1),
            ip.offset(1) as *const libc::c_void,
            iSize,
            6 as libc::c_int as libc::c_uint,
            workSpace,
            wkspSize,
            bmi2,
        );
        if FSE_isError(oSize) != 0 {
            return oSize;
        }
    }
    libc::memset(
        rankStats as *mut libc::c_void,
        0 as libc::c_int,
        ((12 as libc::c_int + 1) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<u32>())
            as libc::size_t,
    );
    weightTotal = 0 as libc::c_int as u32;
    let mut n_0: u32 = 0;
    n_0 = 0 as libc::c_int as u32;
    while (n_0 as libc::c_ulong) < oSize {
        if *huffWeight.offset(n_0 as isize) as libc::c_int > HUF_TABLELOG_MAX {
            return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
        }
        let ref mut fresh1 = *rankStats.offset(*huffWeight.offset(n_0 as isize) as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        weightTotal = (weightTotal as libc::c_uint).wrapping_add(
            ((1) << *huffWeight.offset(n_0 as isize) as libc::c_int >> 1 as libc::c_int)
                as libc::c_uint,
        );
        n_0 = n_0.wrapping_add(1);
    }
    if weightTotal == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    let tableLog = (ZSTD_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1) << ZSTD_highbit32(rest)) as u32;
    let lastWeight = (ZSTD_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as u8;
    let ref mut fresh2 = *rankStats.offset(lastWeight as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
    if *rankStats.offset(1) < 2 || *rankStats.offset(1) & 1 != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as libc::size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    return iSize.wrapping_add(1);
}
unsafe extern "C" fn HUF_readStats_body_default(
    mut huffWeight: *mut u8,
    mut hwSize: libc::size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
) -> libc::size_t {
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
        0 as libc::c_int,
    );
}
unsafe extern "C" fn HUF_readStats_body_bmi2(
    mut huffWeight: *mut u8,
    mut hwSize: libc::size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
) -> libc::size_t {
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
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readStats_wksp(
    mut huffWeight: *mut u8,
    mut hwSize: libc::size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut flags: libc::c_int,
) -> libc::size_t {
    if flags & HUF_flags_bmi2 as libc::c_int != 0 {
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
