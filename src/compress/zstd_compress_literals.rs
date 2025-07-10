use std::ffi::{c_char, c_void};
extern "C" {
    fn HUF_compress4X_repeat(
        dst: *mut c_void,
        dstSize: usize,
        src: *const c_void,
        srcSize: usize,
        maxSymbolValue: u32,
        tableLog: u32,
        workSpace: *mut c_void,
        wkspSize: usize,
        hufTable: *mut HUF_CElt,
        repeat: *mut HUF_repeat,
        flags: i32,
    ) -> usize;
    fn HUF_compress1X_repeat(
        dst: *mut c_void,
        dstSize: usize,
        src: *const c_void,
        srcSize: usize,
        maxSymbolValue: u32,
        tableLog: u32,
        workSpace: *mut c_void,
        wkspSize: usize,
        hufTable: *mut HUF_CElt,
        repeat: *mut HUF_repeat,
        flags: i32,
    ) -> usize;
}
pub type unalign16 = u16;
pub type unalign32 = u32;
use crate::common::error::*;
pub type SymbolEncodingType_e = u32;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type ZSTD_strategy = u32;
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
pub type HUF_repeat = u32;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = usize;
pub type C2RustUnnamed_0 = u32;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type huf_compress_f = Option::<
    unsafe extern "C" fn(
        *mut c_void,
        usize,
        *const c_void,
        usize,
        u32,
        u32,
        *mut c_void,
        usize,
        *mut HUF_CElt,
        *mut HUF_repeat,
        i32,
    ) -> usize,
>;
use crate::common::mem::*;
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const c_char,
    mut args: ...
) {}
#[inline]
unsafe extern "C" fn ZSTD_minGain(
    mut srcSize: usize,
    mut strat: ZSTD_strategy,
) -> usize {
    let minlog = if strat as u32
        >= ZSTD_btultra as i32 as u32
    {
        (strat as u32).wrapping_sub(1)
    } else {
        6_u32
    };
    return (srcSize >> minlog).wrapping_add(2);
}
pub const LitHufLog: i32 = 11;
pub const HUF_SYMBOLVALUE_MAX: i32 = 255;
pub const HUF_OPTIMAL_DEPTH_THRESHOLD: i32 = ZSTD_btultra as i32;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_noCompressLiterals(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut src: *const c_void,
    mut srcSize: usize,
) -> usize {
    let ostart = dst as *mut u8;
    let flSize = (1 as i32
        + (srcSize > 31) as i32
        + (srcSize > 4095) as i32) as u32;
    RETURN_ERROR_IF!(srcSize.wrapping_add(flSize as usize) > dstCapacity, ZSTD_error_dstSize_tooSmall);
    match flSize {
        1 => {
            *ostart
                .offset(
                    0,
                ) = (set_basic as i32 as u32 as usize)
                .wrapping_add(srcSize << 3) as u8;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut c_void,
                ((set_basic as i32 as u32)
                    .wrapping_add(
                        ((1 as i32) << 2) as u32,
                    ) as usize)
                    .wrapping_add(srcSize << 4) as u16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut c_void,
                ((set_basic as i32 as u32)
                    .wrapping_add(
                        ((3 as i32) << 2) as u32,
                    ) as usize)
                    .wrapping_add(srcSize << 4) as u32,
            );
        }
        _ => {}
    }
    libc::memcpy(ostart + flSize, src, (srcSize) as usize);
    return srcSize.wrapping_add(flSize as usize);
}
unsafe extern "C" fn allBytesIdentical(
    mut src: *const c_void,
    mut srcSize: usize,
) -> i32 {
    let b = *(src as *const u8).offset(0);
    let mut p: usize = 0;
    p = 1;
    while p < srcSize {
        if *(src as *const u8).offset(p as isize) as i32
            != b as i32
        {
            return 0;
        }
        p = p.wrapping_add(1);
        p;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressRleLiteralsBlock(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut src: *const c_void,
    mut srcSize: usize,
) -> usize {
    let ostart = dst as *mut u8;
    let flSize = (1 as i32
        + (srcSize > 31) as i32
        + (srcSize > 4095) as i32) as u32;
    match flSize {
        1 => {
            *ostart
                .offset(
                    0,
                ) = (set_rle as i32 as u32 as usize)
                .wrapping_add(srcSize << 3) as u8;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut c_void,
                ((set_rle as i32 as u32)
                    .wrapping_add(
                        ((1 as i32) << 2) as u32,
                    ) as usize)
                    .wrapping_add(srcSize << 4) as u16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut c_void,
                ((set_rle as i32 as u32)
                    .wrapping_add(
                        ((3 as i32) << 2) as u32,
                    ) as usize)
                    .wrapping_add(srcSize << 4) as u32,
            );
        }
        _ => {}
    }
    *ostart.offset(flSize as isize) = *(src as *const u8);
    return flSize.wrapping_add(1) as usize;
}
unsafe extern "C" fn ZSTD_minLiteralsToCompress(
    mut strategy: ZSTD_strategy,
    mut huf_repeat: HUF_repeat,
) -> usize {
    let shift = std::cmp::min(9 - (int) strategy, 3);
    let mintc = if huf_repeat as u32
        == HUF_repeat_valid as i32 as u32
    {
        6_usize
    } else {
        8_usize << shift
    };
    return mintc;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressLiterals(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut entropyWorkspace: *mut c_void,
    mut entropyWorkspaceSize: usize,
    mut prevHuf: *const ZSTD_hufCTables_t,
    mut nextHuf: *mut ZSTD_hufCTables_t,
    mut strategy: ZSTD_strategy,
    mut disableLiteralCompression: i32,
    mut suspectUncompressible: i32,
    mut bmi2: i32,
) -> usize {
    let lhSize = (3 as i32
        + (srcSize
            >= (1 as i32 * ((1 as i32) << 10))
                as usize) as i32
        + (srcSize
            >= (16 as i32
                * ((1 as i32) << 10)) as usize)
            as i32) as usize;
    let ostart = dst as *mut u8;
    let mut singleStream = (srcSize < 256)
        as i32 as u32;
    let mut hType = set_compressed;
    let mut cLitSize: usize = 0;
    libc::memcpy(
        nextHuf as *mut c_void,
        prevHuf as *const c_void,
        ::core::mem::size_of::<ZSTD_hufCTables_t>() as usize,
    );
    if disableLiteralCompression != 0 {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if srcSize < ZSTD_minLiteralsToCompress(strategy, (*prevHuf).repeatMode) {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    RETURN_ERROR_IF!(dstCapacity < lhSize.wrapping_add(1), ZSTD_error_dstSize_tooSmall);
    let mut repeat = (*prevHuf).repeatMode;
    let flags = 0 as i32
        | (if bmi2 != 0 {
            HUF_flags_bmi2 as i32
        } else {
            0 as i32
        })
        | (if (strategy as u32)
            < ZSTD_lazy as i32 as u32
            && srcSize <= 1024
        {
            HUF_flags_preferRepeat as i32
        } else {
            0 as i32
        })
        | (if strategy as u32
            >= HUF_OPTIMAL_DEPTH_THRESHOLD as u32
        {
            HUF_flags_optimalDepth as i32
        } else {
            0 as i32
        })
        | (if suspectUncompressible != 0 {
            HUF_flags_suspectUncompressible as i32
        } else {
            0 as i32
        });
    let mut huf_compress: huf_compress_f = None;
    if repeat as u32
        == HUF_repeat_valid as i32 as u32
        && lhSize == 3
    {
        singleStream = 1;
    }
    huf_compress = if singleStream != 0 {
        Some(
            HUF_compress1X_repeat
                as unsafe extern "C" fn(
                    *mut c_void,
                    usize,
                    *const c_void,
                    usize,
                    u32,
                    u32,
                    *mut c_void,
                    usize,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    i32,
                ) -> usize,
        )
    } else {
        Some(
            HUF_compress4X_repeat
                as unsafe extern "C" fn(
                    *mut c_void,
                    usize,
                    *const c_void,
                    usize,
                    u32,
                    u32,
                    *mut c_void,
                    usize,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    i32,
                ) -> usize,
        )
    };
    cLitSize = huf_compress
        .expect(
            "non-null function pointer",
        )(
        ostart.offset(lhSize as isize) as *mut c_void,
        dstCapacity.wrapping_sub(lhSize),
        src,
        srcSize,
        HUF_SYMBOLVALUE_MAX as u32,
        LitHufLog as u32,
        entropyWorkspace,
        entropyWorkspaceSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        &mut repeat,
        flags,
    );
    if repeat as u32
        != HUF_repeat_none as i32 as u32
    {
        hType = set_repeat;
    }
    let minGain = ZSTD_minGain(srcSize, strategy);
    if cLitSize == 0
        || cLitSize >= srcSize.wrapping_sub(minGain) || ERR_isError(cLitSize) != 0
    {
        libc::memcpy(
            nextHuf as *mut c_void,
            prevHuf as *const c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>()
                as usize,
        );
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if cLitSize == 1 {
        if srcSize >= 8
            || allBytesIdentical(src, srcSize) != 0
        {
            libc::memcpy(
                nextHuf as *mut c_void,
                prevHuf as *const c_void,
                ::core::mem::size_of::<ZSTD_hufCTables_t>()
                    as usize,
            );
            return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize);
        }
    }
    if hType as u32 == set_compressed as i32 as u32
    {
        (*nextHuf).repeatMode = HUF_repeat_check;
    }
    match lhSize {
        3 => {
            singleStream == 0;
            let lhc = (hType as u32)
                .wrapping_add(
                    ((singleStream == 0) as i32 as u32)
                        << 2,
                )
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 14);
            MEM_writeLE24(ostart as *mut c_void, lhc);
        }
        4 => {
            let lhc_0 = (hType as u32)
                .wrapping_add(
                    ((2 as i32) << 2) as u32,
                )
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 18);
            MEM_writeLE32(ostart as *mut c_void, lhc_0);
        }
        5 => {
            let lhc_1 = (hType as u32)
                .wrapping_add(
                    ((3 as i32) << 2) as u32,
                )
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 22);
            MEM_writeLE32(ostart as *mut c_void, lhc_1);
            *ostart
                .offset(
                    4,
                ) = (cLitSize >> 10) as u8;
        }
        _ => {}
    }
    return lhSize.wrapping_add(cLitSize);
}
