use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    fn FSE_decompress_wksp_bmi2(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
        maxLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
        bmi2: libc::c_int,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type unalign32 = U32;
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
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
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
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    return (0 as libc::c_int as libc::c_ulong).wrapping_sub(code) as ERR_enum;
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t) -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bits.h\0"
                as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"unsigned int ZSTD_countTrailingZeros32(U32)\0"))
                .as_ptr(),
        );
    }
    return val.trailing_zeros() as i32 as libc::c_uint;
}
pub const FSE_VERSION_NUMBER: libc::c_int = FSE_VERSION_MAJOR * 100 as libc::c_int
    * 100 as libc::c_int + FSE_VERSION_MINOR * 100 as libc::c_int + FSE_VERSION_RELEASE;
pub const FSE_VERSION_MAJOR: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bits.h\0"
                as *const u8 as *const libc::c_char,
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
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/build/cmake/../../lib/common/bits.h\0"
                as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn FSE_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn FSE_getErrorName(mut code: size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_getErrorName(mut code: size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
#[inline(always)]
unsafe extern "C" fn FSE_readNCount_body(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const BYTE;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: libc::c_int = 0;
    let mut charnum = 0 as libc::c_int as libc::c_uint;
    let maxSV1 = (*maxSVPtr).wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut previous0 = 0 as libc::c_int;
    if hbSize < 8 as libc::c_int as libc::c_ulong {
        let mut buffer: [libc::c_char; 8] = [
            0 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
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
            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        if FSE_isError(countSize) != 0 {
            return countSize;
        }
        if countSize > hbSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        return countSize;
    }
    if hbSize >= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hbSize >= 8\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/common/entropy_common.c\0" as *const u8
                as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"size_t FSE_readNCount_body(short *, unsigned int *, unsigned int *, const void *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    libc::memset(
        normalizedCounter as *mut libc::c_void,
        0 as libc::c_int,
        ((*maxSVPtr).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            as libc::size_t,
    );
    bitStream = MEM_readLE32(ip as *const libc::c_void);
    nbBits = (bitStream & 0xf as libc::c_int as libc::c_uint)
        .wrapping_add(FSE_MIN_TABLELOG as libc::c_uint) as libc::c_int;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    bitStream >>= 4 as libc::c_int;
    bitCount = 4 as libc::c_int;
    *tableLogPtr = nbBits as libc::c_uint;
    remaining = ((1 as libc::c_int) << nbBits) + 1 as libc::c_int;
    threshold = (1 as libc::c_int) << nbBits;
    nbBits += 1;
    loop {
        if previous0 != 0 {
            let mut repeats = (ZSTD_countTrailingZeros32(
                !bitStream | 0x80000000 as libc::c_uint,
            ) >> 1 as libc::c_int) as libc::c_int;
            while repeats >= 12 as libc::c_int {
                charnum = charnum
                    .wrapping_add(
                        (3 as libc::c_int * 12 as libc::c_int) as libc::c_uint,
                    );
                if (ip <= iend.offset(-(7 as libc::c_int as isize))) as libc::c_int
                    as libc::c_long != 0
                {
                    ip = ip.offset(3 as libc::c_int as isize);
                } else {
                    bitCount
                        -= (8 as libc::c_int as libc::c_long
                            * iend.offset(-(7 as libc::c_int as isize)).offset_from(ip)
                                as libc::c_long) as libc::c_int;
                    bitCount &= 31 as libc::c_int;
                    ip = iend.offset(-(4 as libc::c_int as isize));
                }
                bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
                repeats = (ZSTD_countTrailingZeros32(
                    !bitStream | 0x80000000 as libc::c_uint,
                ) >> 1 as libc::c_int) as libc::c_int;
            }
            charnum = charnum.wrapping_add((3 as libc::c_int * repeats) as libc::c_uint);
            bitStream >>= 2 as libc::c_int * repeats;
            bitCount += 2 as libc::c_int * repeats;
            if (bitStream & 3 as libc::c_int as libc::c_uint)
                < 3 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"(bitStream & 3) < 3\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/common/entropy_common.c\0"
                        as *const u8 as *const libc::c_char,
                    106 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 90],
                        &[libc::c_char; 90],
                    >(
                        b"size_t FSE_readNCount_body(short *, unsigned int *, unsigned int *, const void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            charnum = charnum.wrapping_add(bitStream & 3 as libc::c_int as libc::c_uint);
            bitCount += 2 as libc::c_int;
            if charnum >= maxSV1 {
                break;
            }
            if (ip <= iend.offset(-(7 as libc::c_int as isize))) as libc::c_int
                as libc::c_long != 0
                || ip.offset((bitCount >> 3 as libc::c_int) as isize)
                    <= iend.offset(-(4 as libc::c_int as isize))
            {
                if bitCount >> 3 as libc::c_int <= 3 as libc::c_int {} else {
                    __assert_fail(
                        b"(bitCount >> 3) <= 3\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/common/entropy_common.c\0"
                            as *const u8 as *const libc::c_char,
                        121 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 90],
                            &[libc::c_char; 90],
                        >(
                            b"size_t FSE_readNCount_body(short *, unsigned int *, unsigned int *, const void *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
                bitCount &= 7 as libc::c_int;
            } else {
                bitCount
                    -= (8 as libc::c_int as libc::c_long
                        * iend.offset(-(4 as libc::c_int as isize)).offset_from(ip)
                            as libc::c_long) as libc::c_int;
                bitCount &= 31 as libc::c_int;
                ip = iend.offset(-(4 as libc::c_int as isize));
            }
            bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
        }
        let max = 2 as libc::c_int * threshold - 1 as libc::c_int - remaining;
        let mut count: libc::c_int = 0;
        if (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint) < max as U32 {
            count = (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_int;
            bitCount += nbBits - 1 as libc::c_int;
        } else {
            count = (bitStream
                & (2 as libc::c_int * threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_int;
            if count >= threshold {
                count -= max;
            }
            bitCount += nbBits;
        }
        count -= 1;
        if count >= 0 as libc::c_int {
            remaining -= count;
        } else {
            if count == -(1 as libc::c_int) {} else {
                __assert_fail(
                    b"count == -1\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/common/entropy_common.c\0"
                        as *const u8 as *const libc::c_char,
                    151 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 90],
                        &[libc::c_char; 90],
                    >(
                        b"size_t FSE_readNCount_body(short *, unsigned int *, unsigned int *, const void *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            remaining += count;
        }
        let fresh0 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh0 as isize) = count as libc::c_short;
        previous0 = (count == 0) as libc::c_int;
        if threshold > 1 as libc::c_int {} else {
            __assert_fail(
                b"threshold > 1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/common/entropy_common.c\0" as *const u8
                    as *const libc::c_char,
                157 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"size_t FSE_readNCount_body(short *, unsigned int *, unsigned int *, const void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if remaining < threshold {
            if remaining <= 1 as libc::c_int {
                break;
            }
            nbBits = (ZSTD_highbit32(remaining as U32))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            threshold = (1 as libc::c_int) << nbBits - 1 as libc::c_int;
        }
        if charnum >= maxSV1 {
            break;
        }
        if (ip <= iend.offset(-(7 as libc::c_int as isize))) as libc::c_int
            as libc::c_long != 0
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
            bitCount &= 31 as libc::c_int;
            ip = iend.offset(-(4 as libc::c_int as isize));
        }
        bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount;
    }
    if remaining != 1 as libc::c_int {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if charnum > maxSV1 {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
    }
    if bitCount > 32 as libc::c_int {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1 as libc::c_int as libc::c_uint);
    ip = ip.offset((bitCount + 7 as libc::c_int >> 3 as libc::c_int) as isize);
    return ip.offset_from(istart) as libc::c_long as size_t;
}
unsafe extern "C" fn FSE_readNCount_body_default(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: size_t,
) -> size_t {
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
    mut hbSize: size_t,
) -> size_t {
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
    mut hbSize: size_t,
    mut bmi2: libc::c_int,
) -> size_t {
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
    mut hbSize: size_t,
) -> size_t {
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
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut wksp: [U32; 219] = [0; 219];
    return HUF_readStats_wksp(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        wksp.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[U32; 219]>() as libc::c_ulong,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn HUF_readStats_body(
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut bmi2: libc::c_int,
) -> size_t {
    let mut weightTotal: U32 = 0;
    let mut ip = src as *const BYTE;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    iSize = *ip.offset(0 as libc::c_int as isize) as size_t;
    if iSize >= 128 as libc::c_int as libc::c_ulong {
        oSize = iSize.wrapping_sub(127 as libc::c_int as libc::c_ulong);
        iSize = oSize
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if iSize.wrapping_add(1 as libc::c_int as libc::c_ulong) > srcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
        if oSize >= hwSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        ip = ip.offset(1 as libc::c_int as isize);
        let mut n: U32 = 0;
        n = 0 as libc::c_int as U32;
        while (n as libc::c_ulong) < oSize {
            *huffWeight
                .offset(
                    n as isize,
                ) = (*ip
                .offset(n.wrapping_div(2 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int >> 4 as libc::c_int) as BYTE;
            *huffWeight
                .offset(
                    n.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = (*ip
                .offset(n.wrapping_div(2 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int & 15 as libc::c_int) as BYTE;
            n = (n as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint) as U32
                as U32;
        }
    } else {
        if iSize.wrapping_add(1 as libc::c_int as libc::c_ulong) > srcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
        oSize = FSE_decompress_wksp_bmi2(
            huffWeight as *mut libc::c_void,
            hwSize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
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
        ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<U32>() as libc::c_ulong) as libc::size_t,
    );
    weightTotal = 0 as libc::c_int as U32;
    let mut n_0: U32 = 0;
    n_0 = 0 as libc::c_int as U32;
    while (n_0 as libc::c_ulong) < oSize {
        if *huffWeight.offset(n_0 as isize) as libc::c_int > HUF_TABLELOG_MAX {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
        }
        let ref mut fresh1 = *rankStats
            .offset(*huffWeight.offset(n_0 as isize) as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        weightTotal = (weightTotal as libc::c_uint)
            .wrapping_add(
                ((1 as libc::c_int) << *huffWeight.offset(n_0 as isize) as libc::c_int
                    >> 1 as libc::c_int) as libc::c_uint,
            ) as U32 as U32;
        n_0 = n_0.wrapping_add(1);
    }
    if weightTotal == 0 as libc::c_int as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let tableLog = (ZSTD_highbit32(weightTotal))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if tableLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1 as libc::c_int) << tableLog) as U32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1 as libc::c_int) << ZSTD_highbit32(rest)) as U32;
    let lastWeight = (ZSTD_highbit32(rest))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as BYTE;
    let ref mut fresh2 = *rankStats.offset(lastWeight as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
    if *rankStats.offset(1 as libc::c_int as isize) < 2 as libc::c_int as libc::c_uint
        || *rankStats.offset(1 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint != 0
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1 as libc::c_int as libc::c_ulong) as U32;
    return iSize.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn HUF_readStats_body_default(
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
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
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
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
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut flags: libc::c_int,
) -> size_t {
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
