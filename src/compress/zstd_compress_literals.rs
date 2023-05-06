use ::libc;
extern "C" {
    fn ZSTD_cParam_getBounds(cParam: ZSTD_cParameter) -> ZSTD_bounds;
    fn HUF_compress4X_repeat(
        dst: *mut libc::c_void,
        dstSize: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
        hufTable: *mut HUF_CElt,
        repeat: *mut HUF_repeat,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn HUF_compress1X_repeat(
        dst: *mut libc::c_void,
        dstSize: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
        hufTable: *mut HUF_CElt,
        repeat: *mut HUF_repeat,
        flags: libc::c_int,
    ) -> libc::size_t;
}
pub type unalign16 = u16;
pub type unalign32 = u32;
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
pub type symbolEncodingType_e = libc::c_uint;
pub const set_repeat: symbolEncodingType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub const set_rle: symbolEncodingType_e = 1;
pub const set_basic: symbolEncodingType_e = 0;
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = libc::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = libc::size_t;
pub type ZSTD_cParameter = libc::c_uint;
pub const ZSTD_c_experimentalParam19: ZSTD_cParameter = 1016;
pub const ZSTD_c_experimentalParam18: ZSTD_cParameter = 1015;
pub const ZSTD_c_experimentalParam17: ZSTD_cParameter = 1014;
pub const ZSTD_c_experimentalParam16: ZSTD_cParameter = 1013;
pub const ZSTD_c_experimentalParam15: ZSTD_cParameter = 1012;
pub const ZSTD_c_experimentalParam14: ZSTD_cParameter = 1011;
pub const ZSTD_c_experimentalParam13: ZSTD_cParameter = 1010;
pub const ZSTD_c_experimentalParam12: ZSTD_cParameter = 1009;
pub const ZSTD_c_experimentalParam11: ZSTD_cParameter = 1008;
pub const ZSTD_c_experimentalParam10: ZSTD_cParameter = 1007;
pub const ZSTD_c_experimentalParam9: ZSTD_cParameter = 1006;
pub const ZSTD_c_experimentalParam8: ZSTD_cParameter = 1005;
pub const ZSTD_c_experimentalParam7: ZSTD_cParameter = 1004;
pub const ZSTD_c_experimentalParam6: ZSTD_cParameter = 1003;
pub const ZSTD_c_experimentalParam5: ZSTD_cParameter = 1002;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: libc::size_t,
    pub lowerBound: libc::c_int,
    pub upperBound: libc::c_int,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type huf_compress_f = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::size_t,
        *const libc::c_void,
        libc::size_t,
        libc::c_uint,
        libc::c_uint,
        *mut libc::c_void,
        libc::size_t,
        *mut HUF_CElt,
        *mut HUF_repeat,
        libc::c_int,
    ) -> libc::size_t,
>;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: u16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut u8;
        *p.offset(0 as libc::c_int as isize) = val as u8;
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (val as libc::c_int >> 8 as libc::c_int) as u8;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut libc::c_void, mut val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8)
        .offset(2 as libc::c_int as isize) = (val >> 16 as libc::c_int) as u8;
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const libc::c_char,
    mut args: ...
) {}
pub const HUF_SYMBOLVALUE_MAX: libc::c_int = 255 as libc::c_int;
pub const LitHufLog: libc::c_int = 11 as libc::c_int;
pub const HUF_OPTIMAL_DEPTH_THRESHOLD: libc::c_int = ZSTD_btultra as libc::c_int;
pub const ZSTD_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
#[inline]
unsafe extern "C" fn ZSTD_cParam_withinBounds(
    mut cParam: ZSTD_cParameter,
    mut value: libc::c_int,
) -> libc::c_int {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) != 0 {
        return 0 as libc::c_int;
    }
    if value < bounds.lowerBound {
        return 0 as libc::c_int;
    }
    if value > bounds.upperBound {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ZSTD_minGain(
    mut srcSize: libc::size_t,
    mut strat: ZSTD_strategy,
) -> libc::size_t {
    let minlog = if strat as libc::c_uint >= ZSTD_btultra as libc::c_int as libc::c_uint
    {
        (strat as u32).wrapping_sub(1)
    } else {
        6 as libc::c_int as libc::c_uint
    };
    debug_assert!(ZSTD_cParam_withinBounds(ZSTD_c_strategy, strat as libc::c_int) != 0);
    return (srcSize >> minlog).wrapping_add(2);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_noCompressLiterals(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let ostart = dst as *mut u8;
    let flSize = (1 as libc::c_int
        + (srcSize > 31 as libc::c_int as libc::c_ulong) as libc::c_int
        + (srcSize > 4095 as libc::c_int as libc::c_ulong) as libc::c_int) as u32;
    if srcSize.wrapping_add(flSize as libc::c_ulong) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    match flSize {
        1 => {
            *ostart
                .offset(
                    0 as libc::c_int as isize,
                ) = (set_basic as libc::c_int as u32 as libc::c_ulong)
                .wrapping_add(srcSize << 3 as libc::c_int) as u8;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut libc::c_void,
                ((set_basic as libc::c_int as u32)
                    .wrapping_add(
                        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                    ) as libc::c_ulong)
                    .wrapping_add(srcSize << 4 as libc::c_int) as u16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut libc::c_void,
                ((set_basic as libc::c_int as u32)
                    .wrapping_add(
                        ((3 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                    ) as libc::c_ulong)
                    .wrapping_add(srcSize << 4 as libc::c_int) as u32,
            );
        }
        _ => {
            debug_assert!(false);
        }
    }
    libc::memcpy(
        ostart.offset(flSize as isize) as *mut libc::c_void,
        src,
        srcSize as libc::size_t,
    );
    return srcSize.wrapping_add(flSize as libc::c_ulong);
}
unsafe extern "C" fn allBytesIdentical(
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::c_int {
    debug_assert!(srcSize >= 1 as libc::c_int as libc::c_ulong);
    debug_assert!(!src.is_null());
    let b = *(src as *const u8).offset(0 as libc::c_int as isize);
    let mut p: libc::size_t = 0;
    p = 1 as libc::c_int as libc::size_t;
    while p < srcSize {
        if *(src as *const u8).offset(p as isize) as libc::c_int != b as libc::c_int {
            return 0 as libc::c_int;
        }
        p = p.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressRleLiteralsBlock(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
) -> libc::size_t {
    let ostart = dst as *mut u8;
    let flSize = (1 as libc::c_int
        + (srcSize > 31 as libc::c_int as libc::c_ulong) as libc::c_int
        + (srcSize > 4095 as libc::c_int as libc::c_ulong) as libc::c_int) as u32;
    debug_assert!(dstCapacity >= 4 as libc::c_int as libc::c_ulong);
    debug_assert!(allBytesIdentical(src, srcSize) != 0);
    match flSize {
        1 => {
            *ostart
                .offset(
                    0 as libc::c_int as isize,
                ) = (set_rle as libc::c_int as u32 as libc::c_ulong)
                .wrapping_add(srcSize << 3 as libc::c_int) as u8;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut libc::c_void,
                ((set_rle as libc::c_int as u32)
                    .wrapping_add(
                        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                    ) as libc::c_ulong)
                    .wrapping_add(srcSize << 4 as libc::c_int) as u16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut libc::c_void,
                ((set_rle as libc::c_int as u32)
                    .wrapping_add(
                        ((3 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                    ) as libc::c_ulong)
                    .wrapping_add(srcSize << 4 as libc::c_int) as u32,
            );
        }
        _ => {
            debug_assert!(false);
        }
    }
    *ostart.offset(flSize as isize) = *(src as *const u8);
    return flSize.wrapping_add(1) as libc::size_t;
}
unsafe extern "C" fn ZSTD_minLiteralsToCompress(
    mut strategy: ZSTD_strategy,
    mut huf_repeat: HUF_repeat,
) -> libc::size_t {
    debug_assert!(strategy as libc::c_int >= 0 as libc::c_int);
    debug_assert!(strategy as libc::c_int <= 9 as libc::c_int);
    let shift = if (9 as libc::c_int - strategy as libc::c_int) < 3 as libc::c_int {
        9 as libc::c_int - strategy as libc::c_int
    } else {
        3 as libc::c_int
    };
    let mintc = if huf_repeat as libc::c_uint
        == HUF_repeat_valid as libc::c_int as libc::c_uint
    {
        6 as libc::c_int as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::size_t) << shift
    };
    return mintc;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressLiterals(
    mut dst: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut entropyWorkspace: *mut libc::c_void,
    mut entropyWorkspaceSize: libc::size_t,
    mut prevHuf: *const ZSTD_hufCTables_t,
    mut nextHuf: *mut ZSTD_hufCTables_t,
    mut strategy: ZSTD_strategy,
    mut disableLiteralCompression: libc::c_int,
    mut suspectUncompressible: libc::c_int,
    mut bmi2: libc::c_int,
) -> libc::size_t {
    let lhSize = (3 as libc::c_int
        + (srcSize
            >= (1 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong) as libc::c_int
        + (srcSize
            >= (16 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                as libc::c_ulong) as libc::c_int) as libc::size_t;
    let ostart = dst as *mut u8;
    let mut singleStream = (srcSize < 256 as libc::c_int as libc::c_ulong) as libc::c_int
        as u32;
    let mut hType = set_compressed;
    let mut cLitSize: libc::size_t = 0;
    libc::memcpy(
        nextHuf as *mut libc::c_void,
        prevHuf as *const libc::c_void,
        ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong as libc::size_t,
    );
    if disableLiteralCompression != 0 {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if srcSize < ZSTD_minLiteralsToCompress(strategy, (*prevHuf).repeatMode) {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if dstCapacity < lhSize.wrapping_add(1) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    let mut repeat = (*prevHuf).repeatMode;
    let flags = 0 as libc::c_int
        | (if bmi2 != 0 { HUF_flags_bmi2 as libc::c_int } else { 0 as libc::c_int })
        | (if (strategy as libc::c_uint) < ZSTD_lazy as libc::c_int as libc::c_uint
            && srcSize <= 1024 as libc::c_int as libc::c_ulong
        {
            HUF_flags_preferRepeat as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if strategy as libc::c_uint >= HUF_OPTIMAL_DEPTH_THRESHOLD as libc::c_uint {
            HUF_flags_optimalDepth as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if suspectUncompressible != 0 {
            HUF_flags_suspectUncompressible as libc::c_int
        } else {
            0 as libc::c_int
        });
    let mut huf_compress: huf_compress_f = None;
    if repeat as libc::c_uint == HUF_repeat_valid as libc::c_int as libc::c_uint
        && lhSize == 3 as libc::c_int as libc::c_ulong
    {
        singleStream = 1 as libc::c_int as u32;
    }
    huf_compress = if singleStream != 0 {
        Some(
            HUF_compress1X_repeat
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::size_t,
                    *const libc::c_void,
                    libc::size_t,
                    libc::c_uint,
                    libc::c_uint,
                    *mut libc::c_void,
                    libc::size_t,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    libc::c_int,
                ) -> libc::size_t,
        )
    } else {
        Some(
            HUF_compress4X_repeat
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::size_t,
                    *const libc::c_void,
                    libc::size_t,
                    libc::c_uint,
                    libc::c_uint,
                    *mut libc::c_void,
                    libc::size_t,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    libc::c_int,
                ) -> libc::size_t,
        )
    };
    cLitSize = huf_compress
        .expect(
            "non-null function pointer",
        )(
        ostart.offset(lhSize as isize) as *mut libc::c_void,
        dstCapacity.wrapping_sub(lhSize),
        src,
        srcSize,
        HUF_SYMBOLVALUE_MAX as libc::c_uint,
        LitHufLog as libc::c_uint,
        entropyWorkspace,
        entropyWorkspaceSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        &mut repeat,
        flags,
    );
    if repeat as libc::c_uint != HUF_repeat_none as libc::c_int as libc::c_uint {
        hType = set_repeat;
    }
    let minGain = ZSTD_minGain(srcSize, strategy);
    if cLitSize == 0 as libc::c_int as libc::c_ulong
        || cLitSize >= srcSize.wrapping_sub(minGain) || ERR_isError(cLitSize) != 0
    {
        libc::memcpy(
            nextHuf as *mut libc::c_void,
            prevHuf as *const libc::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong as libc::size_t,
        );
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if cLitSize == 1 as libc::c_int as libc::c_ulong {
        if srcSize >= 8 as libc::c_int as libc::c_ulong
            || allBytesIdentical(src, srcSize) != 0
        {
            libc::memcpy(
                nextHuf as *mut libc::c_void,
                prevHuf as *const libc::c_void,
                ::core::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong
                    as libc::size_t,
            );
            return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize);
        }
    }
    if hType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint {
        (*nextHuf).repeatMode = HUF_repeat_check;
    }
    match lhSize {
        3 => {
            if singleStream == 0 {
                debug_assert!(srcSize >= 6 as libc::c_int as libc::c_ulong);
            }
            let lhc = (hType as libc::c_uint)
                .wrapping_add(
                    ((singleStream == 0) as libc::c_int as u32) << 2 as libc::c_int,
                )
                .wrapping_add((srcSize as u32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as u32) << 14 as libc::c_int);
            MEM_writeLE24(ostart as *mut libc::c_void, lhc);
        }
        4 => {
            debug_assert!(srcSize >= 6 as libc::c_int as libc::c_ulong);
            let lhc_0 = (hType as libc::c_uint)
                .wrapping_add(((2 as libc::c_int) << 2 as libc::c_int) as libc::c_uint)
                .wrapping_add((srcSize as u32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as u32) << 18 as libc::c_int);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_0);
        }
        5 => {
            debug_assert!(srcSize >= 6 as libc::c_int as libc::c_ulong);
            let lhc_1 = (hType as libc::c_uint)
                .wrapping_add(((3 as libc::c_int) << 2 as libc::c_int) as libc::c_uint)
                .wrapping_add((srcSize as u32) << 4 as libc::c_int)
                .wrapping_add((cLitSize as u32) << 22 as libc::c_int);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_1);
            *ostart
                .offset(
                    4 as libc::c_int as isize,
                ) = (cLitSize >> 10 as libc::c_int) as u8;
        }
        _ => {
            debug_assert!(false);
        }
    }
    return lhSize.wrapping_add(cLitSize);
}
