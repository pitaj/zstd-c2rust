use std::mem::size_of;
use std::ptr::{addr_of, addr_of_mut};

use crate::__m128i_u;

#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use core::arch::asm;

use crate::zstd_h::*;
use crate::common::allocations::*;
use crate::common::bits::*;
use crate::common::bitstream_h::*;
use crate::common::zstd_internal_h::*;
use crate::compress::zstd_compress_internal::*;
use crate::compress::zstd_cwksp_h::*;
use crate::common::huf_h::*;
use crate::common::fse_h::*;
use crate::common::error::*;
use crate::common::mem::*;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CDict_s {
    pub dictContent: *const std::ffi::c_void,
    pub dictContentSize: usize,
    pub dictContentType: ZSTD_dictContentType_e,
    pub entropyWorkspace: *mut u32,
    pub workspace: ZSTD_cwksp,
    pub matchState: ZSTD_MatchState_t,
    pub cBlockState: ZSTD_compressedBlockState_t,
    pub customMem: ZSTD_customMem,
    pub dictID: u32,
    pub compressionLevel: i32,
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_symbolEncodingTypeStats_t {
    pub LLtype: u32,
    pub Offtype: u32,
    pub MLtype: u32,
    pub size: usize,
    pub lastCountSize: usize,
    pub longOffsets: i32,
}
pub type ZSTD_DefaultPolicy_e = u32;
pub const ZSTD_defaultAllowed: ZSTD_DefaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_DefaultPolicy_e = 0;
pub type C2RustUnnamed_2 = u32;
pub const ZSTDbss_noCompress: C2RustUnnamed_2 = 1;
pub const ZSTDbss_compress: C2RustUnnamed_2 = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqStoreSplits {
    pub splitLocations: *mut u32,
    pub idx: usize,
}
pub type XXH_errorcode = u32;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type unalign64 = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Trace {
    pub version: u32,
    pub streaming: i32,
    pub dictionaryID: u32,
    pub dictionaryIsCold: i32,
    pub dictionarySize: usize,
    pub uncompressedSize: usize,
    pub compressedSize: usize,
    pub params: *const ZSTD_CCtx_params_s,
    pub cctx: *const ZSTD_CCtx_s,
    pub dctx: *const ZSTD_DCtx_s,
}

pub type ZSTD_compResetPolicy_e = u32;
pub const ZSTDcrp_leaveDirty: ZSTD_compResetPolicy_e = 1;
pub const ZSTDcrp_makeClean: ZSTD_compResetPolicy_e = 0;
pub type ZSTD_resetTarget_e = u32;
pub const ZSTD_resetTarget_CCtx: ZSTD_resetTarget_e = 1;
pub const ZSTD_resetTarget_CDict: ZSTD_resetTarget_e = 0;
pub type ZSTD_indexResetPolicy_e = u32;
pub const ZSTDirp_reset: ZSTD_indexResetPolicy_e = 1;
pub const ZSTDirp_continue: ZSTD_indexResetPolicy_e = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: u32,
    pub f1d: u32,
    pub f7b: u32,
    pub f7c: u32,
}

pub type ZSTD_SequenceCopier_f = Option::<
    unsafe extern "C" fn(
        *mut ZSTD_CCtx,
        *mut ZSTD_SequencePosition,
        *const ZSTD_Sequence,
        usize,
        *const std::ffi::c_void,
        usize,
        ZSTD_ParamSwitch_e,
    ) -> usize,
>;

macro_rules! BOUNDCHECK {
    ($cParam:expr, $val:expr) => {
        RETURN_ERROR_IF!(ZSTD_cParam_withinBounds($cParam, $val), ZSTD_error_parameter_outOfBound)
    }
}

pub const ZSTDMT_JOBSIZE_MIN: i32 = 512 as i32
    * ((1 as i32) << 10);

pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_doubleFast;
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_doubleFast_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_doubleFast_extDict;
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: i32 = 2;
pub const ZSTD_ROW_HASH_TAG_BITS: i32 = 8;
pub const ZSTD_COMPRESSBLOCK_GREEDY: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy;
pub const ZSTD_COMPRESSBLOCK_GREEDY_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_row;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_dictMatchState_row;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_dedicatedDictSearch;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_dedicatedDictSearch_row;
pub const ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_extDict;
pub const ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_greedy_extDict_row;
pub const ZSTD_COMPRESSBLOCK_LAZY: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy;
pub const ZSTD_COMPRESSBLOCK_LAZY_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_row;
pub const ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_dictMatchState_row;
pub const ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_dedicatedDictSearch;
pub const ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_dedicatedDictSearch_row;
pub const ZSTD_COMPRESSBLOCK_LAZY_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_extDict;
pub const ZSTD_COMPRESSBLOCK_LAZY_EXTDICT_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy_extDict_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2;
pub const ZSTD_COMPRESSBLOCK_LAZY2_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_dictMatchState_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_dedicatedDictSearch;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_dedicatedDictSearch_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_extDict;
pub const ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT_ROW: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_lazy2_extDict_row;
pub const ZSTD_COMPRESSBLOCK_BTLAZY2: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btlazy2;
pub const ZSTD_COMPRESSBLOCK_BTLAZY2_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btlazy2_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_BTLAZY2_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btlazy2_extDict;
pub const ZSTD_COMPRESSBLOCK_BTOPT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btopt;
pub const ZSTD_COMPRESSBLOCK_BTOPT_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btopt_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_BTOPT_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btopt_extDict;
pub const ZSTD_COMPRESSBLOCK_BTULTRA: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btultra;
pub const ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btultra_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btultra_extDict;
pub const ZSTD_COMPRESSBLOCK_BTULTRA2: unsafe extern "C" fn(
    *mut ZSTD_MatchState_t,
    *mut SeqStore_t,
    *mut u32,
    *const std::ffi::c_void,
    usize,
) -> usize = ZSTD_compressBlock_btultra2;
pub const ZSTD_LDM_DEFAULT_WINDOW_LOG: i32 = 27;
pub const INT_MAX: i32 = __INT_MAX__;
pub const NULL: i32 = 0;
#[no_mangle]
pub unsafe fn ZSTD_compressBound(mut srcSize: usize) -> usize {
    let r = ZSTD_COMPRESSBOUND(srcSize);
    RETURN_ERROR_IF!(r == 0, ZSTD_error_srcSize_wrong);
    return r;
}
#[no_mangle]
pub unsafe fn ZSTD_createCCtx() -> *mut ZSTD_CCtx {
    return ZSTD_createCCtx_advanced(ZSTD_defaultCMem);
}
unsafe fn ZSTD_initCCtx(
    mut cctx: *mut ZSTD_CCtx,
    mut memManager: ZSTD_customMem,
) {
    libc::memset(
        cctx as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_CCtx>() as usize,
    );
    (*cctx).customMem = memManager;
    (*cctx).bmi2 = false; // TODO ZSTD_cpuSupportsBmi2();
    let err = ZSTD_CCtx_reset(cctx, ZSTD_reset_parameters);
}
#[no_mangle]
pub unsafe fn ZSTD_createCCtx_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CCtx {
    if (customMem.customAlloc).is_none() as i32
        ^ (customMem.customFree).is_none() as i32 != 0
    {
        return std::ptr::null_mut();
    }
    let cctx = ZSTD_customMalloc(
        size_of::<ZSTD_CCtx>(),
        customMem,
    ) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return std::ptr::null_mut();
    }
    ZSTD_initCCtx(cctx, customMem);
    return cctx;
}
#[no_mangle]
pub unsafe fn ZSTD_initStaticCCtx(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: usize,
) -> *mut ZSTD_CCtx {
    let mut ws = ZSTD_cwksp {
        workspace: std::ptr::null_mut(),
        workspaceEnd: std::ptr::null_mut(),
        objectEnd: std::ptr::null_mut(),
        tableEnd: std::ptr::null_mut(),
        tableValidEnd: std::ptr::null_mut(),
        allocStart: std::ptr::null_mut(),
        initOnceStart: std::ptr::null_mut(),
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    if workspaceSize <= size_of::<ZSTD_CCtx>() {
        return std::ptr::null_mut();
    }
    if !workspace.is_aligned_to(8) {
        return std::ptr::null_mut();
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_static_alloc);
    let mut cctx = ZSTD_cwksp_reserve_object(
        &mut ws,
        size_of::<ZSTD_CCtx>(),
    ) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return std::ptr::null_mut();
    }
    libc::memset(
        cctx as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_CCtx>() as usize,
    );
    ZSTD_cwksp_move(&mut (*cctx).workspace, &mut ws);
    (*cctx).staticSize = workspaceSize;
    if ZSTD_cwksp_check_available(
        &mut (*cctx).workspace,
        (if ((((8 as i32) << 10) + 512 as i32)
            as std::ffi::c_ulong)
            .wrapping_add(
                (size_of::<u32>())
                    .wrapping_mul(
                        ((if 35 as i32 > 52 {
                            35 as i32
                        } else {
                            52 as i32
                        }) + 2 as i32) as std::ffi::c_ulong,
                    ),
            ) > 8208
        {
            ((((8 as i32) << 10) + 512 as i32)
                as std::ffi::c_ulong)
                .wrapping_add(
                    (size_of::<u32>())
                        .wrapping_mul(
                            ((if 35 as i32 > 52 {
                                35 as i32
                            } else {
                                52 as i32
                            }) + 2 as i32) as std::ffi::c_ulong,
                        ),
                )
        } else {
            8208 as std::ffi::c_ulong
        })
            .wrapping_add(
                (2 as std::ffi::c_ulong)
                    .wrapping_mul(
                        size_of::<ZSTD_compressedBlockState_t>()
                            as std::ffi::c_ulong,
                    ),
            ),
    ) == 0
    {
        return std::ptr::null_mut();
    }
    (*cctx)
        .blockState
        .prevCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        size_of::<ZSTD_compressedBlockState_t>(),
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx)
        .blockState
        .nextCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        size_of::<ZSTD_compressedBlockState_t>(),
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx)
        .tmpWorkspace = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        if ((((8 as i32) << 10) + 512 as i32)
            as std::ffi::c_ulong)
            .wrapping_add(
                (size_of::<u32>())
                    .wrapping_mul(
                        ((if 35 as i32 > 52 {
                            35 as i32
                        } else {
                            52 as i32
                        }) + 2 as i32) as std::ffi::c_ulong,
                    ),
            ) > 8208
        {
            ((((8 as i32) << 10) + 512 as i32)
                as std::ffi::c_ulong)
                .wrapping_add(
                    (size_of::<u32>())
                        .wrapping_mul(
                            ((if 35 as i32 > 52 {
                                35 as i32
                            } else {
                                52 as i32
                            }) + 2 as i32) as std::ffi::c_ulong,
                        ),
                )
        } else {
            8208 as std::ffi::c_ulong
        },
    );
    (*cctx)
        .tmpWkspSize = if ((((8 as i32) << 10)
        + 512 as i32) as std::ffi::c_ulong)
        .wrapping_add(
            (size_of::<u32>())
                .wrapping_mul(
                    ((if 35 as i32 > 52 {
                        35 as i32
                    } else {
                        52 as i32
                    }) + 2 as i32) as std::ffi::c_ulong,
                ),
        ) > 8208
    {
        ((((8 as i32) << 10) + 512 as i32)
            as std::ffi::c_ulong)
            .wrapping_add(
                (size_of::<u32>())
                    .wrapping_mul(
                        ((if 35 as i32 > 52 {
                            35 as i32
                        } else {
                            52 as i32
                        }) + 2 as i32) as std::ffi::c_ulong,
                    ),
            )
    } else {
        8208 as std::ffi::c_ulong
    };
    (*cctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
    return cctx;
}
unsafe fn ZSTD_clearAllDicts(mut cctx: *mut ZSTD_CCtx) {
    ZSTD_customFree((*cctx).localDict.dictBuffer, (*cctx).customMem);
    ZSTD_freeCDict((*cctx).localDict.cdict);
    libc::memset(
        &mut (*cctx).localDict as *mut ZSTD_localDict as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_localDict>() as usize,
    );
    libc::memset(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_prefixDict>() as usize,
    );
    (*cctx).cdict = std::ptr::null();
}
unsafe fn ZSTD_sizeof_localDict(mut dict: ZSTD_localDict) -> usize {
    let bufferSize = if !(dict.dictBuffer).is_null() {
        dict.dictSize
    } else {
        0_usize
    };
    let cdictSize = ZSTD_sizeof_CDict(dict.cdict);
    return bufferSize.wrapping_add(cdictSize);
}
unsafe fn ZSTD_freeCCtxContent(mut cctx: *mut ZSTD_CCtx) {
    ZSTD_clearAllDicts(cctx);
    ZSTDMT_freeCCtx((*cctx).mtctx);
    (*cctx).mtctx = std::ptr::null_mut();
    ZSTD_cwksp_free(&mut (*cctx).workspace, (*cctx).customMem);
}
#[no_mangle]
pub unsafe fn ZSTD_freeCCtx(mut cctx: *mut ZSTD_CCtx) -> usize {
    if cctx.is_null() {
        return 0;
    }
    RETURN_ERROR_IF!((*cctx).staticSize != 0, ZSTD_error_memory_allocation);
    let mut cctxInWorkspace = ZSTD_cwksp_owns_buffer(
        &mut (*cctx).workspace,
        cctx as *const std::ffi::c_void,
    );
    ZSTD_freeCCtxContent(cctx);
    if cctxInWorkspace == 0 {
        ZSTD_customFree(cctx as *mut std::ffi::c_void, (*cctx).customMem);
    }
    return 0;
}
unsafe fn ZSTD_sizeof_mtctx(mut cctx: *const ZSTD_CCtx) -> usize {
    return ZSTDMT_sizeof_CCtx((*cctx).mtctx);
}
#[no_mangle]
pub unsafe fn ZSTD_sizeof_CCtx(mut cctx: *const ZSTD_CCtx) -> usize {
    if cctx.is_null() {
        return 0;
    }
    return (if (*cctx).workspace.workspace == cctx as *mut std::ffi::c_void {
        0 as std::ffi::c_ulong
    } else {
        size_of::<ZSTD_CCtx>()
    })
        .wrapping_add(ZSTD_cwksp_sizeof(&(*cctx).workspace))
        .wrapping_add(ZSTD_sizeof_localDict((*cctx).localDict))
        .wrapping_add(ZSTD_sizeof_mtctx(cctx));
}
#[no_mangle]
pub unsafe fn ZSTD_sizeof_CStream(mut zcs: *const ZSTD_CStream) -> usize {
    return ZSTD_sizeof_CCtx(zcs);
}
#[no_mangle]
pub unsafe fn ZSTD_getSeqStore(
    mut ctx: *const ZSTD_CCtx,
) -> *const SeqStore_t {
    return &(*ctx).seqStore;
}
unsafe fn ZSTD_rowMatchFinderSupported(
    strategy: ZSTD_strategy,
) -> i32 {
    return (strategy as u32
        >= ZSTD_greedy as i32 as u32
        && strategy as u32
            <= ZSTD_lazy2 as i32 as u32) as i32;
}
unsafe fn ZSTD_rowMatchFinderUsed(
    strategy: ZSTD_strategy,
    mode: ZSTD_ParamSwitch_e,
) -> i32 {
    return (ZSTD_rowMatchFinderSupported(strategy) != 0
        && mode as u32
            == ZSTD_ps_enable as i32 as u32) as i32;
}
unsafe fn ZSTD_resolveRowMatchFinderMode(
    mut mode: ZSTD_ParamSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_ParamSwitch_e {
    let kWindowLogLowerBound = 14;
    if mode as u32 != ZSTD_ps_auto as i32 as u32 {
        return mode;
    }
    mode = ZSTD_ps_disable;
    if ZSTD_rowMatchFinderSupported((*cParams).strategy) == 0 {
        return mode;
    }
    if (*cParams).windowLog > kWindowLogLowerBound {
        mode = ZSTD_ps_enable;
    }
    return mode;
}
unsafe fn ZSTD_resolveBlockSplitterMode(
    mut mode: ZSTD_ParamSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_ParamSwitch_e {
    if mode as u32 != ZSTD_ps_auto as i32 as u32 {
        return mode;
    }
    return (if (*cParams).strategy as u32
        >= ZSTD_btopt as i32 as u32
        && (*cParams).windowLog >= 17
    {
        ZSTD_ps_enable as i32
    } else {
        ZSTD_ps_disable as i32
    }) as ZSTD_ParamSwitch_e;
}
unsafe fn ZSTD_allocateChainTable(
    strategy: ZSTD_strategy,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    forDDSDict: u32,
) -> i32 {
    return (forDDSDict != 0
        || strategy as u32
            != ZSTD_fast as i32 as u32
            && ZSTD_rowMatchFinderUsed(strategy, useRowMatchFinder) == 0)
        as i32;
}
unsafe fn ZSTD_resolveEnableLdm(
    mut mode: ZSTD_ParamSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_ParamSwitch_e {
    if mode as u32 != ZSTD_ps_auto as i32 as u32 {
        return mode;
    }
    return (if (*cParams).strategy as u32
        >= ZSTD_btopt as i32 as u32
        && (*cParams).windowLog >= 27
    {
        ZSTD_ps_enable as i32
    } else {
        ZSTD_ps_disable as i32
    }) as ZSTD_ParamSwitch_e;
}
unsafe fn ZSTD_resolveExternalSequenceValidation(
    mut mode: i32,
) -> i32 {
    return mode;
}
unsafe fn ZSTD_resolveMaxBlockSize(mut maxBlockSize: usize) -> usize {
    if maxBlockSize == 0 {
        return ZSTD_BLOCKSIZE_MAX as usize
    } else {
        return maxBlockSize
    };
}
unsafe fn ZSTD_resolveExternalRepcodeSearch(
    mut value: ZSTD_ParamSwitch_e,
    mut cLevel: i32,
) -> ZSTD_ParamSwitch_e {
    if value as u32 != ZSTD_ps_auto as i32 as u32 {
        return value;
    }
    if cLevel < 10 {
        return ZSTD_ps_disable
    } else {
        return ZSTD_ps_enable
    };
}
unsafe fn ZSTD_CDictIndicesAreTagged(
    cParams: *const ZSTD_compressionParameters,
) -> i32 {
    return ((*cParams).strategy as u32
        == ZSTD_fast as i32 as u32
        || (*cParams).strategy as u32
            == ZSTD_dfast as i32 as u32) as i32;
}
unsafe fn ZSTD_makeCCtxParamsFromCParams(
    mut cParams: ZSTD_compressionParameters,
) -> ZSTD_CCtx_params {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: ZSTD_f_zstd1,
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: std::ptr::null_mut(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    ZSTD_CCtxParams_init(&mut cctxParams, ZSTD_CLEVEL_DEFAULT);
    cctxParams.cParams = cParams;
    cctxParams
        .ldmParams
        .enableLdm = ZSTD_resolveEnableLdm(cctxParams.ldmParams.enableLdm, &mut cParams);
    if cctxParams.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        ZSTD_ldm_adjustParameters(&mut cctxParams.ldmParams, &mut cParams);
    }
    cctxParams
        .postBlockSplitter = ZSTD_resolveBlockSplitterMode(
        cctxParams.postBlockSplitter,
        &mut cParams,
    );
    cctxParams
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        cctxParams.useRowMatchFinder,
        &mut cParams,
    );
    cctxParams
        .validateSequences = ZSTD_resolveExternalSequenceValidation(
        cctxParams.validateSequences,
    );
    cctxParams.maxBlockSize = ZSTD_resolveMaxBlockSize(cctxParams.maxBlockSize);
    cctxParams
        .searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        cctxParams.searchForExternalRepcodes,
        cctxParams.compressionLevel,
    );
    return cctxParams;
}
unsafe fn ZSTD_createCCtxParams_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CCtx_params {
    let mut params = std::ptr::null_mut();
    if (customMem.customAlloc).is_none() as i32
        ^ (customMem.customFree).is_none() as i32 != 0
    {
        return std::ptr::null_mut();
    }
    params = ZSTD_customCalloc(
        size_of::<ZSTD_CCtx_params>(),
        customMem,
    ) as *mut ZSTD_CCtx_params;
    if params.is_null() {
        return std::ptr::null_mut();
    }
    ZSTD_CCtxParams_init(params, ZSTD_CLEVEL_DEFAULT);
    (*params).customMem = customMem;
    return params;
}
#[no_mangle]
pub unsafe fn ZSTD_createCCtxParams() -> *mut ZSTD_CCtx_params {
    return ZSTD_createCCtxParams_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe fn ZSTD_freeCCtxParams(
    mut params: *mut ZSTD_CCtx_params,
) -> usize {
    if params.is_null() {
        return 0;
    }
    ZSTD_customFree(params as *mut std::ffi::c_void, (*params).customMem);
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtxParams_reset(
    mut params: *mut ZSTD_CCtx_params,
) -> usize {
    return ZSTD_CCtxParams_init(params, ZSTD_CLEVEL_DEFAULT);
}
#[no_mangle]
pub unsafe fn ZSTD_CCtxParams_init(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut compressionLevel: i32,
) -> usize {
    RETURN_ERROR_IF!(cctxParams.is_null(), ZSTD_error_GENERIC);
    libc::memset(
        cctxParams as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_CCtx_params>() as usize,
    );
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).fParams.contentSizeFlag = 1;
    return 0;
}
pub const ZSTD_NO_CLEVEL: i32 = 0;
unsafe fn ZSTD_CCtxParams_init_internal(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut params: *const ZSTD_parameters,
    mut compressionLevel: i32,
) {
    libc::memset(
        cctxParams as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_CCtx_params>() as usize,
    );
    (*cctxParams).cParams = (*params).cParams;
    (*cctxParams).fParams = (*params).fParams;
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams)
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        (*cctxParams).useRowMatchFinder,
        &(*params).cParams,
    );
    (*cctxParams)
        .postBlockSplitter = ZSTD_resolveBlockSplitterMode(
        (*cctxParams).postBlockSplitter,
        &(*params).cParams,
    );
    (*cctxParams)
        .ldmParams
        .enableLdm = ZSTD_resolveEnableLdm(
        (*cctxParams).ldmParams.enableLdm,
        &(*params).cParams,
    );
    (*cctxParams)
        .validateSequences = ZSTD_resolveExternalSequenceValidation(
        (*cctxParams).validateSequences,
    );
    (*cctxParams).maxBlockSize = ZSTD_resolveMaxBlockSize((*cctxParams).maxBlockSize);
    (*cctxParams)
        .searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        (*cctxParams).searchForExternalRepcodes,
        compressionLevel,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_CCtxParams_init_advanced(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut params: ZSTD_parameters,
) -> usize {
    RETURN_ERROR_IF!(cctxParams.is_null(), ZSTD_error_GENERIC);
    FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "");
    ZSTD_CCtxParams_init_internal(cctxParams, &mut params, ZSTD_NO_CLEVEL);
    return 0;
}
unsafe fn ZSTD_CCtxParams_setZstdParams(
    mut cctxParams: *mut ZSTD_CCtx_params,
    mut params: *const ZSTD_parameters,
) {
    (*cctxParams).cParams = (*params).cParams;
    (*cctxParams).fParams = (*params).fParams;
    (*cctxParams).compressionLevel = ZSTD_NO_CLEVEL;
}

pub fn ZSTD_cParam_getBounds(param: ZSTD_cParameter) -> ZSTD_bounds {
    match param {
        ZSTD_c_compressionLevel => ZSTD_bounds { lowerBound: ZSTD_minCLevel(), upperBound: ZSTD_maxCLevel(), error: 0 },
        ZSTD_c_windowLog => ZSTD_bounds { lowerBound: ZSTD_WINDOWLOG_MIN, upperBound: ZSTD_WINDOWLOG_MAX, error: 0 },
        ZSTD_c_hashLog => ZSTD_bounds { lowerBound: ZSTD_HASHLOG_MIN, upperBound: ZSTD_HASHLOG_MAX, error: 0 },
        ZSTD_c_chainLog => ZSTD_bounds { lowerBound: ZSTD_CHAINLOG_MIN, upperBound: ZSTD_CHAINLOG_MAX, error: 0 },
        ZSTD_c_searchLog => ZSTD_bounds { lowerBound: ZSTD_SEARCHLOG_MIN, upperBound: ZSTD_SEARCHLOG_MAX, error: 0 },
        ZSTD_c_minMatch => ZSTD_bounds { lowerBound: ZSTD_MINMATCH_MIN, upperBound: ZSTD_MINMATCH_MAX, error: 0 },
        ZSTD_c_targetLength => ZSTD_bounds { lowerBound: ZSTD_TARGETLENGTH_MIN, upperBound: ZSTD_TARGETLENGTH_MAX, error: 0 },
        ZSTD_c_strategy => ZSTD_bounds { lowerBound: ZSTD_STRATEGY_MIN, upperBound: ZSTD_STRATEGY_MAX, error: 0 },
        ZSTD_c_contentSizeFlag => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_checksumFlag => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_dictIDFlag => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },

        ZSTD_c_nbWorkers => ZSTD_bounds {
            lowerBound: 0,
            // upperBound: if ZSTD_MULTITHREAD {
            //     ZSTDMT_NBWORKERS_MAX
            // } else {
            //     0
            // },
            // TODO: multithreading
            upperBound: 0,
            error: 0,
        },
        ZSTD_c_jobSize => ZSTD_bounds {
            lowerBound: 0,
            // upperBound: if ZSTD_MULTITHREAD {
            //     ZSTDMT_JOBSIZE_MAX
            // } else {
            //     0
            // },
            // TODO: multithreading
            upperBound: 0,
            error: 0,
        },
        ZSTD_c_overlapLog => if false /* ZSTD_MULTITHREAD */ { // TODO: multithreading
            ZSTD_bounds {
                lowerBound: ZSTD_OVERLAPLOG_MIN,
                upperBound: ZSTD_OVERLAPLOG_MAX,
                error: 0,
            }
        } else {
            ZSTD_bounds { lowerBound: 0, upperBound: 0, error: 0 }
        },

        ZSTD_c_enableDedicatedDictSearch => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_enableLongDistanceMatching => ZSTD_bounds { lowerBound: ZSTD_ps_auto, upperBound: ZSTD_ps_disable, error: 0 },
        ZSTD_c_ldmHashLog => ZSTD_bounds { lowerBound: ZSTD_LDM_HASHLOG_MIN, upperBound: ZSTD_LDM_HASHLOG_MAX, error: 0 },
        ZSTD_c_ldmMinMatch => ZSTD_bounds { lowerBound: ZSTD_LDM_MINMATCH_MIN, upperBound: ZSTD_LDM_MINMATCH_MAX, error: 0 },
        ZSTD_c_ldmBucketSizeLog => ZSTD_bounds { lowerBound: ZSTD_LDM_BUCKETSIZELOG_MIN, upperBound: ZSTD_LDM_BUCKETSIZELOG_MAX, error: 0 },
        ZSTD_c_ldmHashRateLog => ZSTD_bounds { lowerBound: ZSTD_LDM_HASHRATELOG_MIN, upperBound: ZSTD_LDM_HASHRATELOG_MAX, error: 0 },
        
        /* experimental parameters */
        ZSTD_c_rsyncable => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_forceMaxWindow => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },

        ZSTD_c_format => {
            const _: () = assert!(ZSTD_f_zstd1 < ZSTD_f_zstd1_magicless);
            ZSTD_bounds { 
                lowerBound: ZSTD_f_zstd1,
                upperBound: ZSTD_f_zstd1_magicless,   /* note : how to ensure at compile time that this is the highest value enum ? */
                error: 0,
            }
        }

        ZSTD_c_forceAttachDict => {
            const _: () = assert!(ZSTD_dictDefaultAttach < ZSTD_dictForceLoad);
            /* note : how to ensure at compile time that this is the highest value enum ? */
            ZSTD_bounds { lowerBound: ZSTD_dictDefaultAttach, upperBound: ZSTD_dictForceLoad, error: 0 }    
        }

        ZSTD_c_literalCompressionMode => {
            const _: () = assert!(ZSTD_ps_auto < ZSTD_ps_enable && ZSTD_ps_enable < ZSTD_ps_disable);
            ZSTD_bounds { lowerBound: ZSTD_ps_auto, upperBound: ZSTD_ps_disable, error: 0 }
        }

        ZSTD_c_targetCBlockSize => ZSTD_bounds { lowerBound: ZSTD_TARGETCBLOCKSIZE_MIN, upperBound: ZSTD_TARGETCBLOCKSIZE_MAX, error: 0 },
        ZSTD_c_srcSizeHint => ZSTD_bounds { lowerBound: ZSTD_SRCSIZEHINT_MIN, upperBound: ZSTD_SRCSIZEHINT_MAX, error: 0 },
        ZSTD_c_stableInBuffer => ZSTD_bounds { lowerBound: ZSTD_bm_buffered, upperBound: ZSTD_bm_stable, error: 0 },
        ZSTD_c_blockDelimiters => ZSTD_bounds { lowerBound: ZSTD_sf_noBlockDelimiters, upperBound: ZSTD_sf_explicitBlockDelimiters, error: 0 },
        ZSTD_c_validateSequences => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_splitAfterSequences => ZSTD_bounds { lowerBound: ZSTD_ps_auto, upperBound: ZSTD_ps_disable, error: 0 },
        ZSTD_c_blockSplitterLevel => ZSTD_bounds { lowerBound: 0, upperBound: ZSTD_BLOCKSPLITTER_LEVEL_MAX, error: 0 },
        ZSTD_c_useRowMatchFinder => ZSTD_bounds { lowerBound: ZSTD_ps_auto, upperBound: ZSTD_ps_disable, error: 0 },
        ZSTD_c_deterministicRefPrefix => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_prefetchCDictTables => ZSTD_bounds { lowerBound: ZSTD_ps_auto, upperBound: ZSTD_ps_disable, error: 0 },
        ZSTD_c_enableSeqProducerFallback => ZSTD_bounds { lowerBound: 0, upperBound: 1, error: 0 },
        ZSTD_c_maxBlockSize => ZSTD_bounds { lowerBound: ZSTD_BLOCKSIZE_MAX_MIN, upperBound: ZSTD_BLOCKSIZE_MAX as i32, error: 0 },
        ZSTD_c_repcodeResolution => ZSTD_bounds { lowerBound: ZSTD_ps_auto, upperBound: ZSTD_ps_disable, error: 0 },

        _ => ZSTD_bounds { error: ERROR(ZSTD_error_parameter_unsupported), lowerBound: 0, upperBound: 0 },
    }
}

unsafe fn ZSTD_cParam_clampBounds(
    mut cParam: ZSTD_cParameter,
    mut value: *mut i32,
) -> usize {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) {
        return bounds.error;
    }
    if *value < bounds.lowerBound {
        *value = bounds.lowerBound;
    }
    if *value > bounds.upperBound {
        *value = bounds.upperBound;
    }
    return 0;
}
unsafe fn ZSTD_isUpdateAuthorized(
    mut param: ZSTD_cParameter,
) -> i32 {
    match param as u32 {
        100 | 102 | 103 | 104 | 105 | 106 | 107 | 1017 => return 1,
        10 | 101 | 200 | 201 | 202 | 1000 | 400 | 401 | 402 | 500 | 1005 | 160 | 161
        | 162 | 163 | 164 | 1001 | 1002 | 130 | 1004 | 1006 | 1007 | 1008 | 1009 | 1010
        | 1011 | 1012 | 1013 | 1014 | 1015 | 1016 | _ => return 0,
    };
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_setParameter(
    mut cctx: *mut ZSTD_CCtx,
    mut param: ZSTD_cParameter,
    mut value: i32,
) -> usize {
    if (*cctx).streamStage as u32
        != zcss_init as i32 as u32
    {
        if ZSTD_isUpdateAuthorized(param) != 0 {
            (*cctx).cParamsChanged = 1;
        } else {
            return ERROR(ZSTD_error_stage_wrong)
        }
    }
    match param as u32 {
        400 => {
            RETURN_ERROR_IF!(value != 0 && (*cctx).staticSize != 0, ZSTD_error_parameter_unsupported);
        }
        100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 164 | 10 | 200 | 201 | 202 | 1000
        | 1001 | 1002 | 401 | 402 | 500 | 1005 | 160 | 161 | 162 | 163 | 130 | 1004
        | 1006 | 1007 | 1008 | 1009 | 1010 | 1017 | 1011 | 1012 | 1013 | 1014 | 1015
        | 1016 => {}
        _ => return ERROR(ZSTD_error_parameter_unsupported),
    }
    return ZSTD_CCtxParams_setParameter(&mut (*cctx).requestedParams, param, value);
}
#[no_mangle]
pub unsafe fn ZSTD_CCtxParams_setParameter(
    mut CCtxParams: *mut ZSTD_CCtx_params,
    mut param: ZSTD_cParameter,
    mut value: i32,
) -> usize {
    match param as u32 {
        10 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam2, value);
            (*CCtxParams).format = value as ZSTD_format_e;
            return (*CCtxParams).format as usize;
        }
        100 => {
            FORWARD_IF_ERROR!(
                ZSTD_cParam_clampBounds(param, addr_of!(value)), ""
            );
            if value == 0 {
                (*CCtxParams).compressionLevel = ZSTD_CLEVEL_DEFAULT;
            } else {
                (*CCtxParams).compressionLevel = value;
            }
            if (*CCtxParams).compressionLevel >= 0 {
                return (*CCtxParams).compressionLevel as usize;
            }
            return 0;
        }
        101 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_windowLog, value);
            }
            (*CCtxParams).cParams.windowLog = value as u32;
            return (*CCtxParams).cParams.windowLog as usize;
        }
        102 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_hashLog, value);
            }
            (*CCtxParams).cParams.hashLog = value as u32;
            return (*CCtxParams).cParams.hashLog as usize;
        }
        103 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_chainLog, value);
            }
            (*CCtxParams).cParams.chainLog = value as u32;
            return (*CCtxParams).cParams.chainLog as usize;
        }
        104 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_searchLog, value);
            }
            (*CCtxParams).cParams.searchLog = value as u32;
            return value as usize;
        }
        105 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_minMatch, value);
            }
            (*CCtxParams).cParams.minMatch = value as u32;
            return (*CCtxParams).cParams.minMatch as usize;
        }
        106 => {
            BOUNDCHECK!(ZSTD_c_targetLength, value);
            (*CCtxParams).cParams.targetLength = value as u32;
            return (*CCtxParams).cParams.targetLength as usize;
        }
        107 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_strategy, value);
            }
            (*CCtxParams).cParams.strategy = value as ZSTD_strategy;
            return (*CCtxParams).cParams.strategy as usize;
        }
        200 => {
            (*CCtxParams)
                .fParams
                .contentSizeFlag = (value != 0) as i32;
            return (*CCtxParams).fParams.contentSizeFlag as usize;
        }
        201 => {
            (*CCtxParams)
                .fParams
                .checksumFlag = (value != 0) as i32;
            return (*CCtxParams).fParams.checksumFlag as usize;
        }
        202 => {
            (*CCtxParams).fParams.noDictIDFlag = (value == 0) as i32;
            return ((*CCtxParams).fParams.noDictIDFlag == 0) as i32
                as usize;
        }
        1000 => {
            (*CCtxParams)
                .forceWindow = (value != 0) as i32;
            return (*CCtxParams).forceWindow as usize;
        }
        1001 => {
            let pref = value as ZSTD_dictAttachPref_e;
            BOUNDCHECK!(
                ZSTD_c_experimentalParam4,
                pref as i32,
            );
            (*CCtxParams).attachDictPref = pref;
            return (*CCtxParams).attachDictPref as usize;
        }
        1002 => {
            let lcm = value as ZSTD_ParamSwitch_e;
            BOUNDCHECK!(
                ZSTD_c_experimentalParam5,
                lcm as i32,
            );
            (*CCtxParams).literalCompressionMode = lcm;
            return (*CCtxParams).literalCompressionMode as usize;
        }
        400 => {
            FORWARD_IF_ERROR!(
                ZSTD_cParam_clampBounds(param, addr_of!(value)), ""
            );
            (*CCtxParams).nbWorkers = value;
            return (*CCtxParams).nbWorkers as usize;
        }
        401 => {
            if value != 0 && value < ZSTDMT_JOBSIZE_MIN {
                value = ZSTDMT_JOBSIZE_MIN;
            }
            FORWARD_IF_ERROR!(
                ZSTD_cParam_clampBounds(param, addr_of!(value)), ""
            );
            (*CCtxParams).jobSize = value as usize;
            return (*CCtxParams).jobSize;
        }
        402 => {
            FORWARD_IF_ERROR!(
                ZSTD_cParam_clampBounds(ZSTD_c_overlapLog, addr_of!(value)), ""
            );
            (*CCtxParams).overlapLog = value;
            return (*CCtxParams).overlapLog as usize;
        }
        500 => {
            FORWARD_IF_ERROR!(
                ZSTD_cParam_clampBounds(ZSTD_c_overlapLog, addr_of!(value)), ""
            );
            (*CCtxParams).rsyncable = value;
            return (*CCtxParams).rsyncable as usize;
        }
        1005 => {
            (*CCtxParams)
                .enableDedicatedDictSearch = (value != 0)
                as i32;
            return (*CCtxParams).enableDedicatedDictSearch as usize;
        }
        160 => {
            BOUNDCHECK!(ZSTD_c_enableLongDistanceMatching, value);
            (*CCtxParams).ldmParams.enableLdm = value as ZSTD_ParamSwitch_e;
            return (*CCtxParams).ldmParams.enableLdm as usize;
        }
        161 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_ldmHashLog, value);
            }
            (*CCtxParams).ldmParams.hashLog = value as u32;
            return (*CCtxParams).ldmParams.hashLog as usize;
        }
        162 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_ldmMinMatch, value);
            }
            (*CCtxParams).ldmParams.minMatchLength = value as u32;
            return (*CCtxParams).ldmParams.minMatchLength as usize;
        }
        163 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_ldmBucketSizeLog, value);
            }
            (*CCtxParams).ldmParams.bucketSizeLog = value as u32;
            return (*CCtxParams).ldmParams.bucketSizeLog as usize;
        }
        164 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_ldmHashRateLog, value);
            }
            (*CCtxParams).ldmParams.hashRateLog = value as u32;
            return (*CCtxParams).ldmParams.hashRateLog as usize;
        }
        130 => {
            if value != 0 {
                value = std::cmp::max(value, ZSTD_TARGETCBLOCKSIZE_MIN);
                BOUNDCHECK!(ZSTD_c_targetCBlockSize, value);
            }
            (*CCtxParams).targetCBlockSize = value as u32 as usize;
            return (*CCtxParams).targetCBlockSize;
        }
        1004 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_experimentalParam7, value);
            }
            (*CCtxParams).srcSizeHint = value;
            return (*CCtxParams).srcSizeHint as usize;
        }
        1006 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam9, value);
            (*CCtxParams).inBufferMode = value as ZSTD_bufferMode_e;
            return (*CCtxParams).inBufferMode as usize;
        }
        1007 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam10, value);
            (*CCtxParams).outBufferMode = value as ZSTD_bufferMode_e;
            return (*CCtxParams).outBufferMode as usize;
        }
        1008 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam11, value);
            (*CCtxParams).blockDelimiters = value as ZSTD_SequenceFormat_e;
            return (*CCtxParams).blockDelimiters as usize;
        }
        1009 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam12, value);
            (*CCtxParams).validateSequences = value;
            return (*CCtxParams).validateSequences as usize;
        }
        1010 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam13, value);
            (*CCtxParams).postBlockSplitter = value as ZSTD_ParamSwitch_e;
            return (*CCtxParams).postBlockSplitter as usize;
        }
        1017 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam20, value);
            (*CCtxParams).preBlockSplitter_level = value;
            return (*CCtxParams).preBlockSplitter_level as usize;
        }
        1011 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam14, value);
            (*CCtxParams).useRowMatchFinder = value as ZSTD_ParamSwitch_e;
            return (*CCtxParams).useRowMatchFinder as usize;
        }
        1012 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam15, value);
            (*CCtxParams).deterministicRefPrefix = (value != 0) as i32;
            return (*CCtxParams).deterministicRefPrefix as usize;
        }
        1013 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam16, value);
            (*CCtxParams).prefetchCDictTables = value as ZSTD_ParamSwitch_e;
            return (*CCtxParams).prefetchCDictTables as usize;
        }
        1014 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam17, value);
            (*CCtxParams).enableMatchFinderFallback = value;
            return (*CCtxParams).enableMatchFinderFallback as usize;
        }
        1015 => {
            if value != 0 {
                BOUNDCHECK!(ZSTD_c_experimentalParam18, value);
            }
            (*CCtxParams).maxBlockSize = value as usize;
            return (*CCtxParams).maxBlockSize;
        }
        1016 => {
            BOUNDCHECK!(ZSTD_c_experimentalParam19, value);
            (*CCtxParams).searchForExternalRepcodes = value as ZSTD_ParamSwitch_e;
            return (*CCtxParams).searchForExternalRepcodes as usize;
        }
        _ => return ERROR(ZSTD_error_parameter_unsupported),
    };
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_getParameter(
    mut cctx: *const ZSTD_CCtx,
    mut param: ZSTD_cParameter,
    mut value: *mut i32,
) -> usize {
    return ZSTD_CCtxParams_getParameter(&(*cctx).requestedParams, param, value);
}
#[no_mangle]
pub unsafe fn ZSTD_CCtxParams_getParameter(
    mut CCtxParams: *const ZSTD_CCtx_params,
    mut param: ZSTD_cParameter,
    mut value: *mut i32,
) -> usize {
    match param as u32 {
        10 => {
            *value = (*CCtxParams).format as i32;
        }
        100 => {
            *value = (*CCtxParams).compressionLevel;
        }
        101 => {
            *value = (*CCtxParams).cParams.windowLog as i32;
        }
        102 => {
            *value = (*CCtxParams).cParams.hashLog as i32;
        }
        103 => {
            *value = (*CCtxParams).cParams.chainLog as i32;
        }
        104 => {
            *value = (*CCtxParams).cParams.searchLog as i32;
        }
        105 => {
            *value = (*CCtxParams).cParams.minMatch as i32;
        }
        106 => {
            *value = (*CCtxParams).cParams.targetLength as i32;
        }
        107 => {
            *value = (*CCtxParams).cParams.strategy as i32;
        }
        200 => {
            *value = (*CCtxParams).fParams.contentSizeFlag;
        }
        201 => {
            *value = (*CCtxParams).fParams.checksumFlag;
        }
        202 => {
            *value = ((*CCtxParams).fParams.noDictIDFlag == 0) as i32;
        }
        1000 => {
            *value = (*CCtxParams).forceWindow;
        }
        1001 => {
            *value = (*CCtxParams).attachDictPref as i32;
        }
        1002 => {
            *value = (*CCtxParams).literalCompressionMode as i32;
        }
        400 => {
            *value = (*CCtxParams).nbWorkers;
        }
        401 => {
            *value = (*CCtxParams).jobSize as i32;
        }
        402 => {
            *value = (*CCtxParams).overlapLog;
        }
        500 => {
            *value = (*CCtxParams).rsyncable;
        }
        1005 => {
            *value = (*CCtxParams).enableDedicatedDictSearch;
        }
        160 => {
            *value = (*CCtxParams).ldmParams.enableLdm as i32;
        }
        161 => {
            *value = (*CCtxParams).ldmParams.hashLog as i32;
        }
        162 => {
            *value = (*CCtxParams).ldmParams.minMatchLength as i32;
        }
        163 => {
            *value = (*CCtxParams).ldmParams.bucketSizeLog as i32;
        }
        164 => {
            *value = (*CCtxParams).ldmParams.hashRateLog as i32;
        }
        130 => {
            *value = (*CCtxParams).targetCBlockSize as i32;
        }
        1004 => {
            *value = (*CCtxParams).srcSizeHint;
        }
        1006 => {
            *value = (*CCtxParams).inBufferMode as i32;
        }
        1007 => {
            *value = (*CCtxParams).outBufferMode as i32;
        }
        1008 => {
            *value = (*CCtxParams).blockDelimiters as i32;
        }
        1009 => {
            *value = (*CCtxParams).validateSequences;
        }
        1010 => {
            *value = (*CCtxParams).postBlockSplitter as i32;
        }
        1017 => {
            *value = (*CCtxParams).preBlockSplitter_level;
        }
        1011 => {
            *value = (*CCtxParams).useRowMatchFinder as i32;
        }
        1012 => {
            *value = (*CCtxParams).deterministicRefPrefix;
        }
        1013 => {
            *value = (*CCtxParams).prefetchCDictTables as i32;
        }
        1014 => {
            *value = (*CCtxParams).enableMatchFinderFallback;
        }
        1015 => {
            *value = (*CCtxParams).maxBlockSize as i32;
        }
        1016 => {
            *value = (*CCtxParams).searchForExternalRepcodes as i32;
        }
        _ => return ERROR(ZSTD_error_parameter_unsupported),
    }
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_setParametersUsingCCtxParams(
    mut cctx: *mut ZSTD_CCtx,
    mut params: *const ZSTD_CCtx_params,
) -> usize {
    RETURN_ERROR_IF!((*cctx).streamStage as u32
        != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
    RETURN_ERROR_IF!(!((*cctx).cdict).is_null(), ZSTD_error_stage_wrong);
    (*cctx).requestedParams = *params;
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_setCParams(
    mut cctx: *mut ZSTD_CCtx,
    mut cparams: ZSTD_compressionParameters,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_checkCParams(cparams), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_windowLog, (cparams.windowLog as i32)), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_chainLog, (cparams.chainLog as i32)), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_hashLog, (cparams.hashLog as i32)), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_searchLog, (cparams.searchLog as i32)), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_minMatch, (cparams.minMatch as i32)), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_targetLength, (cparams.targetLength as i32)), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_strategy, (cparams.strategy as i32)), ""
    );
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_setFParams(
    mut cctx: *mut ZSTD_CCtx,
    mut fparams: ZSTD_frameParameters,
) -> usize {
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_contentSizeFlag, fparams.contentSizeFlag !=
        0), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_checksumFlag, fparams.checksumFlag != 0), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(cctx, ZSTD_c_dictIDFlag, fparams.noDictIDFlag == 0), ""
    );
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_setParams(
    mut cctx: *mut ZSTD_CCtx,
    mut params: ZSTD_parameters,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "");
    FORWARD_IF_ERROR!(ZSTD_CCtx_setFParams(cctx, params.fParams), "");
    FORWARD_IF_ERROR!(ZSTD_CCtx_setCParams(cctx, params.cParams), "");
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_setPledgedSrcSize(
    mut cctx: *mut ZSTD_CCtx,
    mut pledgedSrcSize: u64,
) -> usize {
    RETURN_ERROR_IF!((*cctx).streamStage as u32
        != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
    (*cctx)
        .pledgedSrcSizePlusOne = pledgedSrcSize
        .wrapping_add(1);
    return 0;
}
unsafe fn ZSTD_initLocalDict(mut cctx: *mut ZSTD_CCtx) -> usize {
    let dl: *mut ZSTD_localDict = &mut (*cctx).localDict;
    if ((*dl).dict).is_null() {
        return 0;
    }
    if !((*dl).cdict).is_null() {
        return 0;
    }
    (*dl)
        .cdict = ZSTD_createCDict_advanced2(
        (*dl).dict,
        (*dl).dictSize,
        ZSTD_dlm_byRef,
        (*dl).dictContentType,
        &mut (*cctx).requestedParams,
        (*cctx).customMem,
    );
    RETURN_ERROR_IF!(((*dl).cdict).is_null(), ZSTD_error_memory_allocation);
    (*cctx).cdict = (*dl).cdict;
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_loadDictionary_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> usize {
    RETURN_ERROR_IF!((*cctx).streamStage as u32
        != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
    ZSTD_clearAllDicts(cctx);
    if dict.is_null() || dictSize == 0 {
        return 0;
    }
    if dictLoadMethod as u32
        == ZSTD_dlm_byRef as i32 as u32
    {
        (*cctx).localDict.dict = dict;
    } else {
        let mut dictBuffer = std::ptr::null_mut();
        RETURN_ERROR_IF!((*cctx).staticSize != 0, ZSTD_error_memory_allocation);
        dictBuffer = ZSTD_customMalloc(dictSize, (*cctx).customMem);
        RETURN_ERROR_IF!(dictBuffer.is_null(), ZSTD_error_memory_allocation);
        libc::memcpy(dictBuffer, dict, (dictSize) as usize);
        (*cctx).localDict.dictBuffer = dictBuffer;
        (*cctx).localDict.dict = dictBuffer;
    }
    (*cctx).localDict.dictSize = dictSize;
    (*cctx).localDict.dictContentType = dictContentType;
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_loadDictionary_byReference(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
) -> usize {
    return ZSTD_CCtx_loadDictionary_advanced(
        cctx,
        dict,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_loadDictionary(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
) -> usize {
    return ZSTD_CCtx_loadDictionary_advanced(
        cctx,
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_refCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
) -> usize {
    RETURN_ERROR_IF!((*cctx).streamStage as u32
        != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
    ZSTD_clearAllDicts(cctx);
    (*cctx).cdict = cdict;
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_refThreadPool(
    mut cctx: *mut ZSTD_CCtx,
    mut pool: *mut ZSTD_threadPool,
) -> usize {
    RETURN_ERROR_IF!((*cctx).streamStage as u32
        != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
    (*cctx).pool = pool;
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_refPrefix(
    mut cctx: *mut ZSTD_CCtx,
    mut prefix: *const std::ffi::c_void,
    mut prefixSize: usize,
) -> usize {
    return ZSTD_CCtx_refPrefix_advanced(cctx, prefix, prefixSize, ZSTD_dct_rawContent);
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_refPrefix_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut prefix: *const std::ffi::c_void,
    mut prefixSize: usize,
    mut dictContentType: ZSTD_dictContentType_e,
) -> usize {
    RETURN_ERROR_IF!((*cctx).streamStage as u32
        != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
    ZSTD_clearAllDicts(cctx);
    if !prefix.is_null() && prefixSize > 0 {
        (*cctx).prefixDict.dict = prefix;
        (*cctx).prefixDict.dictSize = prefixSize;
        (*cctx).prefixDict.dictContentType = dictContentType;
    }
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_CCtx_reset(
    mut cctx: *mut ZSTD_CCtx,
    mut reset: ZSTD_ResetDirective,
) -> usize {
    if reset as u32
        == ZSTD_reset_session_only as i32 as u32
        || reset as u32
            == ZSTD_reset_session_and_parameters as i32 as u32
    {
        (*cctx).streamStage = zcss_init;
        (*cctx).pledgedSrcSizePlusOne = 0;
    }
    if reset as u32
        == ZSTD_reset_parameters as i32 as u32
        || reset as u32
            == ZSTD_reset_session_and_parameters as i32 as u32
    {
        RETURN_ERROR_IF!((*cctx).streamStage as u32
            != zcss_init as i32 as u32, ZSTD_error_stage_wrong);
        ZSTD_clearAllDicts(cctx);
        return ZSTD_CCtxParams_reset(&mut (*cctx).requestedParams);
    }
    return 0;
}

/** ZSTD_checkCParams() :
    control CParam values remain within authorized range.
    @return : 0, or an error code if one value is beyond authorized range */
pub fn ZSTD_checkCParams(
    cParams: ZSTD_compressionParameters,
) -> usize {
    BOUNDCHECK!(ZSTD_c_windowLog, cParams.windowLog);
    BOUNDCHECK!(ZSTD_c_chainLog, cParams.chainLog);
    BOUNDCHECK!(ZSTD_c_hashLog, cParams.hashLog);
    BOUNDCHECK!(ZSTD_c_searchLog, cParams.searchLog);
    BOUNDCHECK!(ZSTD_c_minMatch, cParams.minMatch);
    BOUNDCHECK!(ZSTD_c_targetLength, cParams.targetLength);
    BOUNDCHECK!(ZSTD_c_strategy, cParams.strategy);

    0
}

/** ZSTD_clampCParams() :
 *  make CParam values within valid range.
 *  @return : valid CParams */
pub fn ZSTD_clampCParams(
    mut cParams: ZSTD_compressionParameters,
) -> ZSTD_compressionParameters {
    macro_rules! CLAMP {
        ($cParam:expr, $val:expr, $typ:ty) => {{
            let bounds: ZSTD_bounds = ZSTD_cParam_getBounds($cParam);
            $val = $val.clamp(bounds.lowerBound as $typ, bounds.upperBound as $typ);
        }};
        ($cParam:expr, $val:expr) => { CLAMP!($cParam, $val, u32) }
    }

    CLAMP!(ZSTD_c_windowLog, cParams.windowLog);
    CLAMP!(ZSTD_c_chainLog, cParams.chainLog);
    CLAMP!(ZSTD_c_hashLog, cParams.hashLog);
    CLAMP!(ZSTD_c_searchLog, cParams.searchLog);
    CLAMP!(ZSTD_c_minMatch, cParams.minMatch);
    CLAMP!(ZSTD_c_targetLength, cParams.targetLength);
    CLAMP!(ZSTD_c_strategy, cParams.strategy, ZSTD_strategy);

    cParams
}

/** ZSTD_cycleLog() :
 *  condition for correct operation : hashLog > 1 */
pub unsafe fn ZSTD_cycleLog(
    hashLog: u32,
    strat: ZSTD_strategy,
) -> u32 {
    if strat >= ZSTD_btlazy2 {
        hashLog - 1
    } else {
        hashLog + 1
    }
}

unsafe fn ZSTD_dictAndWindowLog(
    mut windowLog: u32,
    mut srcSize: u64,
    mut dictSize: u64,
) -> u32 {
    let maxWindowSize = ((1 as u64)
        << (if size_of::<usize>()
            == 4
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        })) as u64;
    if dictSize == 0 {
        return windowLog;
    }
    let windowSize = ((1 as u64) << windowLog) as u64;
    let dictAndWindowSize = dictSize.wrapping_add(windowSize);
    if windowSize >= dictSize.wrapping_add(srcSize) {
        return windowLog
    } else if dictAndWindowSize >= maxWindowSize {
        return (if size_of::<usize>()
            == 4
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as u32
    } else {
        return (ZSTD_highbit32(
            (dictAndWindowSize as u32).wrapping_sub(1),
        ))
            .wrapping_add(1)
    };
}
unsafe fn ZSTD_adjustCParams_internal(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: u64,
    mut dictSize: usize,
    mut mode: ZSTD_CParamMode_e,
    mut useRowMatchFinder: ZSTD_ParamSwitch_e,
) -> ZSTD_compressionParameters {
    let minSrcSize = 513;
    let maxWindowResize = ((1 as u64)
        << (if size_of::<usize>()
            == 4
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) - 1 as i32) as u64;
    match mode as u32 {
        2 => {
            if dictSize != 0 && srcSize == ZSTD_CONTENTSIZE_UNKNOWN {
                srcSize = minSrcSize as u64;
            }
        }
        1 => {
            dictSize = 0;
        }
        3 | 0 | _ => {}
    }
    if srcSize <= maxWindowResize as u64 && dictSize <= maxWindowResize
    {
        let tSize = srcSize.wrapping_add(dictSize as u64) as u32;
        static mut hashSizeMin: u32 = ((1 as i32) << ZSTD_HASHLOG_MIN)
            as u32;
        let srcLog = if tSize < hashSizeMin {
            ZSTD_HASHLOG_MIN as u32
        } else {
            (ZSTD_highbit32(tSize.wrapping_sub(1)))
                .wrapping_add(1)
        };
        if cPar.windowLog > srcLog {
            cPar.windowLog = srcLog;
        }
    }
    if srcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let dictAndWindowLog = ZSTD_dictAndWindowLog(
            cPar.windowLog,
            srcSize as u64,
            dictSize,
        );
        let cycleLog = ZSTD_cycleLog(cPar.chainLog, cPar.strategy);
        if cPar.hashLog > dictAndWindowLog.wrapping_add(1) {
            cPar.hashLog = dictAndWindowLog.wrapping_add(1);
        }
        if cycleLog > dictAndWindowLog {
            cPar
                .chainLog = (cPar.chainLog)
                .wrapping_sub(cycleLog.wrapping_sub(dictAndWindowLog));
        }
    }
    if cPar.windowLog < ZSTD_WINDOWLOG_ABSOLUTEMIN as u32 {
        cPar.windowLog = ZSTD_WINDOWLOG_ABSOLUTEMIN as u32;
    }
    if mode as u32
        == ZSTD_cpm_createCDict as i32 as u32
        && ZSTD_CDictIndicesAreTagged(&mut cPar) != 0
    {
        let maxShortCacheHashLog = (32 as i32 - ZSTD_SHORT_CACHE_TAG_BITS)
            as u32;
        if cPar.hashLog > maxShortCacheHashLog {
            cPar.hashLog = maxShortCacheHashLog;
        }
        if cPar.chainLog > maxShortCacheHashLog {
            cPar.chainLog = maxShortCacheHashLog;
        }
    }
    if useRowMatchFinder as u32
        == ZSTD_ps_auto as i32 as u32
    {
        useRowMatchFinder = ZSTD_ps_enable;
    }
    if ZSTD_rowMatchFinderUsed(cPar.strategy, useRowMatchFinder) != 0 {
        let rowLog = BOUNDED!(4, cPar.searchLog, 6);
        let maxRowHashLog = (32 as i32 - ZSTD_ROW_HASH_TAG_BITS) as u32;
        let maxHashLog = maxRowHashLog.wrapping_add(rowLog);
        if cPar.hashLog > maxHashLog {
            cPar.hashLog = maxHashLog;
        }
    }
    return cPar;
}
#[no_mangle]
pub unsafe fn ZSTD_adjustCParams(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: u64,
    mut dictSize: usize,
) -> ZSTD_compressionParameters {
    cPar = ZSTD_clampCParams(cPar);
    if srcSize == 0 {
        srcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    return ZSTD_adjustCParams_internal(
        cPar,
        srcSize,
        dictSize,
        ZSTD_cpm_unknown,
        ZSTD_ps_auto,
    );
}
unsafe fn ZSTD_overrideCParams(
    mut cParams: *mut ZSTD_compressionParameters,
    mut overrides: *const ZSTD_compressionParameters,
) {
    if (*overrides).windowLog != 0 {
        (*cParams).windowLog = (*overrides).windowLog;
    }
    if (*overrides).hashLog != 0 {
        (*cParams).hashLog = (*overrides).hashLog;
    }
    if (*overrides).chainLog != 0 {
        (*cParams).chainLog = (*overrides).chainLog;
    }
    if (*overrides).searchLog != 0 {
        (*cParams).searchLog = (*overrides).searchLog;
    }
    if (*overrides).minMatch != 0 {
        (*cParams).minMatch = (*overrides).minMatch;
    }
    if (*overrides).targetLength != 0 {
        (*cParams).targetLength = (*overrides).targetLength;
    }
    if (*overrides).strategy as u64 != 0 {
        (*cParams).strategy = (*overrides).strategy;
    }
}

pub unsafe fn ZSTD_getCParamsFromCCtxParams(
    mut CCtxParams: *const ZSTD_CCtx_params,
    mut srcSizeHint: u64,
    mut dictSize: usize,
    mut mode: ZSTD_CParamMode_e,
) -> ZSTD_compressionParameters {
    if srcSizeHint == ZSTD_CONTENTSIZE_UNKNOWN
        && (*CCtxParams).srcSizeHint > 0
    {
        debug_assert!((*CCtxParams).srcSizeHint>=0);
        srcSizeHint = (*CCtxParams).srcSizeHint as u64;
    }
    let mut cParams = ZSTD_getCParams_internal(
        (*CCtxParams).compressionLevel,
        srcSizeHint as u64,
        dictSize,
        mode,
    );
    if (*CCtxParams).ldmParams.enableLdm == ZSTD_ps_enable
    {
        cParams.windowLog = ZSTD_LDM_DEFAULT_WINDOW_LOG as u32;
    }
    ZSTD_overrideCParams(&mut cParams, &(*CCtxParams).cParams);
    debug_assert!(ZSTD_checkCParams(cParams) == 0);
    /* srcSizeHint == 0 means 0 */
    return ZSTD_adjustCParams_internal(
        cParams,
        srcSizeHint as u64,
        dictSize,
        mode,
        (*CCtxParams).useRowMatchFinder,
    );
}

unsafe fn ZSTD_sizeof_matchState(
    cParams: *const ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    enableDedicatedDictSearch: i32,
    forCCtx: u32,
) -> usize {
    let chainSize = if ZSTD_allocateChainTable(
        (*cParams).strategy,
        useRowMatchFinder,
        (enableDedicatedDictSearch != 0 && forCCtx == 0) as i32 as u32,
    ) != 0
    {
        1_usize << (*cParams).chainLog
    } else {
        0_usize
    };
    let hSize = 1_usize << (*cParams).hashLog;
    let hashLog3 = if forCCtx != 0
        && (*cParams).minMatch == 3
    {
        std::cmp::min(ZSTD_HASHLOG3_MAX, (*cParams).windowLog)
    } else {
        0 as u32
    };
    let h3Size = if hashLog3 != 0 {
        1_usize << hashLog3
    } else {
        0_usize
    };
    let tableSpace = chainSize
        .wrapping_mul(size_of::<u32>())
        .wrapping_add(
            hSize.wrapping_mul(size_of::<u32>()),
        )
        .wrapping_add(
            h3Size.wrapping_mul(size_of::<u32>()),
        );
    let optPotentialSpace = (ZSTD_cwksp_aligned64_alloc_size(
        ((MaxML + 1 as i32) as std::ffi::c_ulong)
            .wrapping_mul(size_of::<u32>()),
    ))
        .wrapping_add(
            ZSTD_cwksp_aligned64_alloc_size(
                ((MaxLL + 1 as i32) as std::ffi::c_ulong)
                    .wrapping_mul(size_of::<u32>()),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned64_alloc_size(
                ((MaxOff + 1 as i32) as std::ffi::c_ulong)
                    .wrapping_mul(size_of::<u32>()),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned64_alloc_size(
                (((1 as i32) << Litbits) as std::ffi::c_ulong)
                    .wrapping_mul(size_of::<u32>()),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned64_alloc_size(
                (ZSTD_OPT_SIZE as std::ffi::c_ulong)
                    .wrapping_mul(
                        size_of::<ZSTD_match_t>(),
                    ),
            ),
        )
        .wrapping_add(
            ZSTD_cwksp_aligned64_alloc_size(
                (ZSTD_OPT_SIZE as std::ffi::c_ulong)
                    .wrapping_mul(
                        size_of::<ZSTD_optimal_t>(),
                    ),
            ),
        );
    let lazyAdditionalSpace = if ZSTD_rowMatchFinderUsed(
        (*cParams).strategy,
        useRowMatchFinder,
    ) != 0
    {
        ZSTD_cwksp_aligned64_alloc_size(hSize)
    } else {
        0_usize
    };
    let optSpace = if forCCtx != 0
        && (*cParams).strategy as u32
            >= ZSTD_btopt as i32 as u32
    {
        optPotentialSpace
    } else {
        0_usize
    };
    let slackSpace = ZSTD_cwksp_slack_space_required();
    return tableSpace
        .wrapping_add(optSpace)
        .wrapping_add(slackSpace)
        .wrapping_add(lazyAdditionalSpace);
}
unsafe fn ZSTD_maxNbSeq(
    mut blockSize: usize,
    mut minMatch: u32,
    mut useSequenceProducer: i32,
) -> usize {
    let divider = (if minMatch == 3
        || useSequenceProducer != 0
    {
        3 as i32
    } else {
        4 as i32
    }) as u32;
    return blockSize / divider as usize;
}
unsafe fn ZSTD_estimateCCtxSize_usingCCtxParams_internal(
    mut cParams: *const ZSTD_compressionParameters,
    mut ldmParams: *const ldmParams_t,
    isStatic: i32,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    buffInSize: usize,
    buffOutSize: usize,
    pledgedSrcSize: u64,
    mut useSequenceProducer: i32,
    mut maxBlockSize: usize,
) -> usize {
    let windowSize = BOUNDED!(1ULL, 1ULL << (*cParams).windowLog, pledgedSrcSize)
        as usize;
    let blockSize = std::cmp::min(ZSTD_resolveMaxBlockSize(maxBlockSize), windowSize);
    let maxNbSeq = ZSTD_maxNbSeq(blockSize, (*cParams).minMatch, useSequenceProducer);
    let tokenSpace = (ZSTD_cwksp_alloc_size(
        (WILDCOPY_OVERLENGTH as usize).wrapping_add(blockSize),
    ))
        .wrapping_add(
            ZSTD_cwksp_aligned64_alloc_size(
                maxNbSeq
                    .wrapping_mul(size_of::<SeqDef>()),
            ),
        )
        .wrapping_add(
            3_usize
                * ZSTD_cwksp_alloc_size(
                    maxNbSeq
                        .wrapping_mul(
                            size_of::<u8>(),
                        ),
                ),
        );
    let tmpWorkSpace = ZSTD_cwksp_alloc_size(
        if ((((8 as i32) << 10) + 512 as i32)
            as std::ffi::c_ulong)
            .wrapping_add(
                (size_of::<u32>())
                    .wrapping_mul(
                        ((if 35 as i32 > 52 {
                            35 as i32
                        } else {
                            52 as i32
                        }) + 2 as i32) as std::ffi::c_ulong,
                    ),
            ) > 8208
        {
            ((((8 as i32) << 10) + 512 as i32)
                as std::ffi::c_ulong)
                .wrapping_add(
                    (size_of::<u32>())
                        .wrapping_mul(
                            ((if 35 as i32 > 52 {
                                35 as i32
                            } else {
                                52 as i32
                            }) + 2 as i32) as std::ffi::c_ulong,
                        ),
                )
        } else {
            8208 as std::ffi::c_ulong
        },
    );
    let blockStateSpace = 2_usize
        * ZSTD_cwksp_alloc_size(
            size_of::<ZSTD_compressedBlockState_t>(),
        );
    let matchStateSize = ZSTD_sizeof_matchState(
        cParams,
        useRowMatchFinder,
        0,
        1,
    );
    let ldmSpace = ZSTD_ldm_getTableSize(*ldmParams);
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq(*ldmParams, blockSize);
    let ldmSeqSpace = if (*ldmParams).enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        ZSTD_cwksp_aligned64_alloc_size(
            maxNbLdmSeq
                .wrapping_mul(size_of::<rawSeq>()),
        )
    } else {
        0_usize
    };
    let bufferSpace = (ZSTD_cwksp_alloc_size(buffInSize))
        .wrapping_add(ZSTD_cwksp_alloc_size(buffOutSize));
    let cctxSpace = if isStatic != 0 {
        ZSTD_cwksp_alloc_size(size_of::<ZSTD_CCtx>())
    } else {
        0_usize
    };
    let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
    let externalSeqSpace = if useSequenceProducer != 0 {
        ZSTD_cwksp_aligned64_alloc_size(
            maxNbExternalSeq
                .wrapping_mul(
                    size_of::<ZSTD_Sequence>(),
                ),
        )
    } else {
        0_usize
    };
    let neededSpace = cctxSpace
        .wrapping_add(tmpWorkSpace)
        .wrapping_add(blockStateSpace)
        .wrapping_add(ldmSpace)
        .wrapping_add(ldmSeqSpace)
        .wrapping_add(matchStateSize)
        .wrapping_add(tokenSpace)
        .wrapping_add(bufferSpace)
        .wrapping_add(externalSeqSpace);
    return neededSpace;
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCCtxSize_usingCCtxParams(
    mut params: *const ZSTD_CCtx_params,
) -> usize {
    let cParams = ZSTD_getCParamsFromCCtxParams(
        params,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        0,
        ZSTD_cpm_noAttachDict,
    );
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        (*params).useRowMatchFinder,
        &cParams,
    );
    RETURN_ERROR_IF!((*params).nbWorkers > 0, ZSTD_error_GENERIC);
    return ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &cParams,
        &(*params).ldmParams,
        1,
        useRowMatchFinder,
        0,
        0,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        ZSTD_hasExtSeqProd(params),
        (*params).maxBlockSize,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCCtxSize_usingCParams(
    mut cParams: ZSTD_compressionParameters,
) -> usize {
    let mut initialParams = ZSTD_makeCCtxParamsFromCParams(cParams);
    if ZSTD_rowMatchFinderSupported(cParams.strategy) != 0 {
        let mut noRowCCtxSize: usize = 0;
        let mut rowCCtxSize: usize = 0;
        initialParams.useRowMatchFinder = ZSTD_ps_disable;
        noRowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&mut initialParams);
        initialParams.useRowMatchFinder = ZSTD_ps_enable;
        rowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&mut initialParams);
        return std::cmp::max(noRowCCtxSize, rowCCtxSize);
    } else {
        return ZSTD_estimateCCtxSize_usingCCtxParams(&mut initialParams)
    };
}
static mut srcSizeTiers: [u64; 4] = [0; 4];
unsafe fn ZSTD_estimateCCtxSize_internal(
    mut compressionLevel: i32,
) -> usize {
    let mut tier: i32 = 0;
    let mut largestSize: usize = 0;
    while tier < 4 {
        let cParams = ZSTD_getCParams_internal(
            compressionLevel,
            srcSizeTiers[tier as usize],
            0,
            ZSTD_cpm_noAttachDict,
        );
        largestSize = std::cmp::max(ZSTD_estimateCCtxSize_usingCParams(cParams), largestSize);
        tier += 1;
        tier;
    }
    return largestSize;
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCCtxSize(
    mut compressionLevel: i32,
) -> usize {
    let mut level: i32 = 0;
    let mut memBudget: usize = 0;
    level = std::cmp::min(compressionLevel, 1);
    while level <= compressionLevel {
        let newMB = ZSTD_estimateCCtxSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
        level += 1;
        level;
    }
    return memBudget;
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCStreamSize_usingCCtxParams(
    mut params: *const ZSTD_CCtx_params,
) -> usize {
    RETURN_ERROR_IF!((*params).nbWorkers > 0, ZSTD_error_GENERIC);
    let cParams = ZSTD_getCParamsFromCCtxParams(
        params,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        0,
        ZSTD_cpm_noAttachDict,
    );
    let blockSize = std::cmp::min(
        ZSTD_resolveMaxBlockSize((*params).maxBlockSize), (usize) 1 << cParams.windowLog
    );
    let inBuffSize = if (*params).inBufferMode as u32
        == ZSTD_bm_buffered as i32 as u32
    {
        (1_usize << cParams.windowLog).wrapping_add(blockSize)
    } else {
        0_usize
    };
    let outBuffSize = if (*params).outBufferMode as u32
        == ZSTD_bm_buffered as i32 as u32
    {
        (ZSTD_compressBound(blockSize)).wrapping_add(1)
    } else {
        0_usize
    };
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        (*params).useRowMatchFinder,
        &(*params).cParams,
    );
    return ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &cParams,
        &(*params).ldmParams,
        1,
        useRowMatchFinder,
        inBuffSize,
        outBuffSize,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        ZSTD_hasExtSeqProd(params),
        (*params).maxBlockSize,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCStreamSize_usingCParams(
    mut cParams: ZSTD_compressionParameters,
) -> usize {
    let mut initialParams = ZSTD_makeCCtxParamsFromCParams(cParams);
    if ZSTD_rowMatchFinderSupported(cParams.strategy) != 0 {
        let mut noRowCCtxSize: usize = 0;
        let mut rowCCtxSize: usize = 0;
        initialParams.useRowMatchFinder = ZSTD_ps_disable;
        noRowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&mut initialParams);
        initialParams.useRowMatchFinder = ZSTD_ps_enable;
        rowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&mut initialParams);
        return std::cmp::max(noRowCCtxSize, rowCCtxSize);
    } else {
        return ZSTD_estimateCStreamSize_usingCCtxParams(&mut initialParams)
    };
}
unsafe fn ZSTD_estimateCStreamSize_internal(
    mut compressionLevel: i32,
) -> usize {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0,
        ZSTD_cpm_noAttachDict,
    );
    return ZSTD_estimateCStreamSize_usingCParams(cParams);
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCStreamSize(
    mut compressionLevel: i32,
) -> usize {
    let mut level: i32 = 0;
    let mut memBudget: usize = 0;
    level = std::cmp::min(compressionLevel, 1);
    while level <= compressionLevel {
        let newMB = ZSTD_estimateCStreamSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
        level += 1;
        level;
    }
    return memBudget;
}
#[no_mangle]
pub unsafe fn ZSTD_getFrameProgression(
    mut cctx: *const ZSTD_CCtx,
) -> ZSTD_frameProgression {
    if (*cctx).appliedParams.nbWorkers > 0 {
        return ZSTDMT_getFrameProgression((*cctx).mtctx);
    }
    let mut fp = ZSTD_frameProgression {
        ingested: 0,
        consumed: 0,
        produced: 0,
        flushed: 0,
        currentJobID: 0,
        nbActiveWorkers: 0,
    };
    let buffered = if ((*cctx).inBuff).is_null() {
        0_usize
    } else {
        ((*cctx).inBuffPos).wrapping_sub((*cctx).inToCompress)
    };
    buffered != 0;
    fp
        .ingested = ((*cctx).consumedSrcSize)
        .wrapping_add(buffered as u64);
    fp.consumed = (*cctx).consumedSrcSize;
    fp.produced = (*cctx).producedCSize;
    fp.flushed = (*cctx).producedCSize;
    fp.currentJobID = 0;
    fp.nbActiveWorkers = 0;
    return fp;
}
#[no_mangle]
pub unsafe fn ZSTD_toFlushNow(mut cctx: *mut ZSTD_CCtx) -> usize {
    if (*cctx).appliedParams.nbWorkers > 0 {
        return ZSTDMT_toFlushNow((*cctx).mtctx);
    }
    return 0;
}
unsafe fn ZSTD_assertEqualCParams(
    mut cParams1: ZSTD_compressionParameters,
    mut cParams2: ZSTD_compressionParameters,
) {}

pub unsafe fn ZSTD_reset_compressedBlockState(
    mut bs: *mut ZSTD_compressedBlockState_t,
) {
    for i in 0..(ZSTD_REP_NUM as usize) {
        (*bs).rep[i] = repStartValue[i];
    }
    (*bs).entropy.huf.repeatMode = HUF_repeat_none;
    (*bs).entropy.fse.offcode_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.matchlength_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.litlength_repeatMode = FSE_repeat_none;
}

unsafe fn ZSTD_invalidateMatchState(mut ms: *mut ZSTD_MatchState_t) {
    ZSTD_window_clear(&mut (*ms).window);
    (*ms).nextToUpdate = (*ms).window.dictLimit;
    (*ms).loadedDictEnd = 0;
    (*ms).opt.litLengthSum = 0;
    (*ms).dictMatchState = std::ptr::null();
}
unsafe fn ZSTD_bitmix(mut val: u64, mut len: u64) -> u64 {
    val
        ^= ZSTD_rotateRight_U64(val, 49)
            ^ ZSTD_rotateRight_U64(val, 24);
    val = (val as u64)
        .wrapping_mul(0x9fb21c651e98df25 as u64) as u64 as u64;
    val ^= (val >> 35).wrapping_add(len);
    val = (val as u64)
        .wrapping_mul(0x9fb21c651e98df25 as u64) as u64 as u64;
    return val ^ val >> 28;
}
unsafe fn ZSTD_advanceHashSalt(mut ms: *mut ZSTD_MatchState_t) {
    (*ms)
        .hashSalt = ZSTD_bitmix((*ms).hashSalt, 8)
        ^ ZSTD_bitmix((*ms).hashSaltEntropy as u64, 4);
}
unsafe fn ZSTD_reset_matchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut ws: *mut ZSTD_cwksp,
    mut cParams: *const ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    crp: ZSTD_compResetPolicy_e,
    forceResetIndex: ZSTD_indexResetPolicy_e,
    forWho: ZSTD_resetTarget_e,
) -> usize {
    let chainSize = if ZSTD_allocateChainTable(
        (*cParams).strategy,
        useRowMatchFinder,
        ((*ms).dedicatedDictSearch != 0
            && forWho as u32
                == ZSTD_resetTarget_CDict as i32 as u32)
            as i32 as u32,
    ) != 0
    {
        1_usize << (*cParams).chainLog
    } else {
        0_usize
    };
    let hSize = 1_usize << (*cParams).hashLog;
    let hashLog3 = if forWho as u32
        == ZSTD_resetTarget_CCtx as i32 as u32
        && (*cParams).minMatch == 3
    {
        std::cmp::min(ZSTD_HASHLOG3_MAX, (*cParams).windowLog)
    } else {
        0 as u32
    };
    let h3Size = if hashLog3 != 0 {
        1_usize << hashLog3
    } else {
        0_usize
    };
    if forceResetIndex as u32
        == ZSTDirp_reset as i32 as u32
    {
        ZSTD_window_init(&mut (*ms).window);
        ZSTD_cwksp_mark_tables_dirty(ws);
    }
    (*ms).hashLog3 = hashLog3;
    (*ms).lazySkipping = 0;
    ZSTD_invalidateMatchState(ms);
    ZSTD_cwksp_clear_tables(ws);
    (*ms)
        .hashTable = ZSTD_cwksp_reserve_table(
        ws,
        hSize.wrapping_mul(size_of::<u32>()),
    ) as *mut u32;
    (*ms)
        .chainTable = ZSTD_cwksp_reserve_table(
        ws,
        chainSize.wrapping_mul(size_of::<u32>()),
    ) as *mut u32;
    (*ms)
        .hashTable3 = ZSTD_cwksp_reserve_table(
        ws,
        h3Size.wrapping_mul(size_of::<u32>()),
    ) as *mut u32;
    RETURN_ERROR_IF!(ZSTD_cwksp_reserve_failed(ws) != 0, ZSTD_error_memory_allocation);
    if crp as u32
        != ZSTDcrp_leaveDirty as i32 as u32
    {
        ZSTD_cwksp_clean_tables(ws);
    }
    if ZSTD_rowMatchFinderUsed((*cParams).strategy, useRowMatchFinder) != 0 {
        let tagTableSize = hSize;
        if forWho as u32
            == ZSTD_resetTarget_CCtx as i32 as u32
        {
            (*ms)
                .tagTable = ZSTD_cwksp_reserve_aligned_init_once(ws, tagTableSize)
                as *mut u8;
            ZSTD_advanceHashSalt(ms);
        } else {
            (*ms).tagTable = ZSTD_cwksp_reserve_aligned64(ws, tagTableSize) as *mut u8;
            libc::memset((*ms).tagTable, 0, (tagTableSize) as usize);
            (*ms).hashSalt = 0;
        }
        let rowLog = BOUNDED!(4, (*cParams).searchLog, 6);
        (*ms).rowHashLog = ((*cParams).hashLog).wrapping_sub(rowLog);
    }
    if forWho as u32
        == ZSTD_resetTarget_CCtx as i32 as u32
        && (*cParams).strategy as u32
            >= ZSTD_btopt as i32 as u32
    {
        (*ms)
            .opt
            .litFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            (((1 as i32) << Litbits) as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<u32>(),
                ),
        ) as *mut u32;
        (*ms)
            .opt
            .litLengthFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxLL + 1 as i32) as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<u32>(),
                ),
        ) as *mut u32;
        (*ms)
            .opt
            .matchLengthFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxML + 1 as i32) as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<u32>(),
                ),
        ) as *mut u32;
        (*ms)
            .opt
            .offCodeFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxOff + 1 as i32) as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<u32>(),
                ),
        ) as *mut u32;
        (*ms)
            .opt
            .matchTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            (ZSTD_OPT_SIZE as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<ZSTD_match_t>(),
                ),
        ) as *mut ZSTD_match_t;
        (*ms)
            .opt
            .priceTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            (ZSTD_OPT_SIZE as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<ZSTD_optimal_t>(),
                ),
        ) as *mut ZSTD_optimal_t;
    }
    (*ms).cParams = *cParams;
    RETURN_ERROR_IF!(ZSTD_cwksp_reserve_failed(ws) != 0, ZSTD_error_memory_allocation);
    return 0;
}
pub const ZSTD_INDEXOVERFLOW_MARGIN: i32 = 16 as i32
    * ((1 as i32) << 20);
unsafe fn ZSTD_indexTooCloseToMax(mut w: ZSTD_window_t) -> i32 {
    return ((w.nextSrc).offset_from(w.base) as std::ffi::c_long as usize
        > (if MEM_64bits {
            (3500 as u32)
                .wrapping_mul(
                    ((1 as i32) << 20) as u32,
                )
        } else {
            (2000 as u32)
                .wrapping_mul(
                    ((1 as i32) << 20) as u32,
                )
        })
            .wrapping_sub(ZSTD_INDEXOVERFLOW_MARGIN as u32) as usize)
        as i32;
}
unsafe fn ZSTD_dictTooBig(loadedDictSize: usize) -> i32 {
    return (loadedDictSize
        > (u32::MAX)
            .wrapping_sub(
                (if MEM_64bits {
                    (3500 as u32)
                        .wrapping_mul(
                            ((1 as i32) << 20)
                                as u32,
                        )
                } else {
                    (2000 as u32)
                        .wrapping_mul(
                            ((1 as i32) << 20)
                                as u32,
                        )
                }),
            ) as usize) as i32;
}
unsafe fn ZSTD_resetCCtx_internal(
    mut zc: *mut ZSTD_CCtx,
    mut params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    loadedDictSize: usize,
    crp: ZSTD_compResetPolicy_e,
    zbuff: ZSTD_buffered_policy_e,
) -> usize {
    let ws: *mut ZSTD_cwksp = &mut (*zc).workspace;
    (*zc).isFirstBlock = 1;
    (*zc).appliedParams = *params;
    params = &mut (*zc).appliedParams;
    if (*params).ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        ZSTD_ldm_adjustParameters(
            &mut (*zc).appliedParams.ldmParams,
            &(*params).cParams,
        );
    }
    let windowSize = if 1_usize
        > (if 1_u64 << (*params).cParams.windowLog
            < pledgedSrcSize
        {
            1_u64 << (*params).cParams.windowLog
        } else {
            pledgedSrcSize
        })
    {
        1_usize
    } else if 1_u64 << (*params).cParams.windowLog
        < pledgedSrcSize
    {
        1_u64 << (*params).cParams.windowLog
    } else {
        pledgedSrcSize
    };
    let blockSize = std::cmp::min((*params).maxBlockSize, windowSize);
    let maxNbSeq = ZSTD_maxNbSeq(
        blockSize,
        (*params).cParams.minMatch,
        ZSTD_hasExtSeqProd(params),
    );
    let buffOutSize = if zbuff as u32
        == ZSTDb_buffered as i32 as u32
        && (*params).outBufferMode as u32
            == ZSTD_bm_buffered as i32 as u32
    {
        (ZSTD_compressBound(blockSize)).wrapping_add(1)
    } else {
        0_usize
    };
    let buffInSize = if zbuff as u32
        == ZSTDb_buffered as i32 as u32
        && (*params).inBufferMode as u32
            == ZSTD_bm_buffered as i32 as u32
    {
        windowSize.wrapping_add(blockSize)
    } else {
        0_usize
    };
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq((*params).ldmParams, blockSize);
    let indexTooClose = ZSTD_indexTooCloseToMax((*zc).blockState.matchState.window);
    let dictTooBig = ZSTD_dictTooBig(loadedDictSize);
    let mut needsIndexReset = (if indexTooClose != 0 || dictTooBig != 0
        || (*zc).initialized == 0
    {
        ZSTDirp_reset as i32
    } else {
        ZSTDirp_continue as i32
    }) as ZSTD_indexResetPolicy_e;
    let neededSpace = ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &(*params).cParams,
        &(*params).ldmParams,
        ((*zc).staticSize != 0) as i32,
        (*params).useRowMatchFinder,
        buffInSize,
        buffOutSize,
        pledgedSrcSize,
        ZSTD_hasExtSeqProd(params),
        (*params).maxBlockSize,
    );
    FORWARD_IF_ERROR!(neededSpace, "cctx size estimate failed!");
    if (*zc).staticSize == 0 {
        ZSTD_cwksp_bump_oversized_duration(ws, 0);
    }
    let workspaceTooSmall = (ZSTD_cwksp_sizeof(ws) < neededSpace) as i32;
    let workspaceWasteful = ZSTD_cwksp_check_wasteful(ws, neededSpace);
    let mut resizeWorkspace = (workspaceTooSmall != 0 || workspaceWasteful != 0)
        as i32;
    if resizeWorkspace != 0 {
        RETURN_ERROR_IF!((*zc).staticSize != 0, ZSTD_error_memory_allocation);
        needsIndexReset = ZSTDirp_reset;
        ZSTD_cwksp_free(ws, (*zc).customMem);
        FORWARD_IF_ERROR!(
            ZSTD_cwksp_create(ws, neededSpace, (*zc).customMem), ""
        );
        (*zc)
            .blockState
            .prevCBlock = ZSTD_cwksp_reserve_object(
            ws,
            size_of::<ZSTD_compressedBlockState_t>(),
        ) as *mut ZSTD_compressedBlockState_t;
        RETURN_ERROR_IF!(((*zc).blockState.prevCBlock).is_null(), ZSTD_error_memory_allocation);
        (*zc)
            .blockState
            .nextCBlock = ZSTD_cwksp_reserve_object(
            ws,
            size_of::<ZSTD_compressedBlockState_t>(),
        ) as *mut ZSTD_compressedBlockState_t;
        RETURN_ERROR_IF!(((*zc).blockState.nextCBlock).is_null(), ZSTD_error_memory_allocation);
        (*zc)
            .tmpWorkspace = ZSTD_cwksp_reserve_object(
            ws,
            if ((((8 as i32) << 10)
                + 512 as i32) as std::ffi::c_ulong)
                .wrapping_add(
                    (size_of::<u32>())
                        .wrapping_mul(
                            ((if 35 as i32 > 52 {
                                35 as i32
                            } else {
                                52 as i32
                            }) + 2 as i32) as std::ffi::c_ulong,
                        ),
                ) > 8208
            {
                ((((8 as i32) << 10)
                    + 512 as i32) as std::ffi::c_ulong)
                    .wrapping_add(
                        (size_of::<u32>())
                            .wrapping_mul(
                                ((if 35 as i32 > 52 {
                                    35 as i32
                                } else {
                                    52 as i32
                                }) + 2 as i32) as std::ffi::c_ulong,
                            ),
                    )
            } else {
                8208 as std::ffi::c_ulong
            },
        );
        RETURN_ERROR_IF!(((*zc).tmpWorkspace).is_null(), ZSTD_error_memory_allocation);
        (*zc)
            .tmpWkspSize = if ((((8 as i32) << 10)
            + 512 as i32) as std::ffi::c_ulong)
            .wrapping_add(
                (size_of::<u32>())
                    .wrapping_mul(
                        ((if 35 as i32 > 52 {
                            35 as i32
                        } else {
                            52 as i32
                        }) + 2 as i32) as std::ffi::c_ulong,
                    ),
            ) > 8208
        {
            ((((8 as i32) << 10) + 512 as i32)
                as std::ffi::c_ulong)
                .wrapping_add(
                    (size_of::<u32>())
                        .wrapping_mul(
                            ((if 35 as i32 > 52 {
                                35 as i32
                            } else {
                                52 as i32
                            }) + 2 as i32) as std::ffi::c_ulong,
                        ),
                )
        } else {
            8208 as std::ffi::c_ulong
        };
    }
    ZSTD_cwksp_clear(ws);
    (*zc).blockState.matchState.cParams = (*params).cParams;
    (*zc)
        .blockState
        .matchState
        .prefetchCDictTables = ((*params).prefetchCDictTables as u32
        == ZSTD_ps_enable as i32 as u32) as i32;
    (*zc)
        .pledgedSrcSizePlusOne = pledgedSrcSize.wrapping_add(1)
        as u64;
    (*zc).consumedSrcSize = 0;
    (*zc).producedCSize = 0;
    if pledgedSrcSize as u64 == ZSTD_CONTENTSIZE_UNKNOWN {
        (*zc).appliedParams.fParams.contentSizeFlag = 0;
    }
    (*zc).blockSizeMax = blockSize;
    ZSTD_XXH64_reset(&mut (*zc).xxhState, 0);
    (*zc).stage = ZSTDcs_init;
    (*zc).dictID = 0;
    (*zc).dictContentSize = 0;
    ZSTD_reset_compressedBlockState((*zc).blockState.prevCBlock);
    FORWARD_IF_ERROR!(
        ZSTD_reset_matchState(addr_of!((*zc).blockState.matchState), ws, addr_of!((*params).cParams),
 (*       params).useRowMatchFinder, crp, needsIndexReset, ZSTD_resetTarget_CCtx), ""
    );
    (*zc)
        .seqStore
        .sequencesStart = ZSTD_cwksp_reserve_aligned64(
        ws,
        maxNbSeq.wrapping_mul(size_of::<SeqDef>()),
    ) as *mut SeqDef;
    if (*params).ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        let ldmHSize = 1_usize << (*params).ldmParams.hashLog;
        (*zc)
            .ldmState
            .hashTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            ldmHSize
                .wrapping_mul(size_of::<ldmEntry_t>()),
        ) as *mut ldmEntry_t;
        libc::memset(
            (*zc).ldmState.hashTable as *mut std::ffi::c_void,
            0,
            ldmHSize
                .wrapping_mul(size_of::<ldmEntry_t>())
                as usize,
        );
        (*zc)
            .ldmSequences = ZSTD_cwksp_reserve_aligned64(
            ws,
            maxNbLdmSeq
                .wrapping_mul(size_of::<rawSeq>()),
        ) as *mut rawSeq;
        (*zc).maxNbLdmSequences = maxNbLdmSeq;
        ZSTD_window_init(&mut (*zc).ldmState.window);
        (*zc).ldmState.loadedDictEnd = 0;
    }
    if ZSTD_hasExtSeqProd(params) != 0 {
        let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
        (*zc).extSeqBufCapacity = maxNbExternalSeq;
        (*zc)
            .extSeqBuf = ZSTD_cwksp_reserve_aligned64(
            ws,
            maxNbExternalSeq
                .wrapping_mul(
                    size_of::<ZSTD_Sequence>(),
                ),
        ) as *mut ZSTD_Sequence;
    }
    (*zc)
        .seqStore
        .litStart = ZSTD_cwksp_reserve_buffer(
        ws,
        blockSize.wrapping_add(WILDCOPY_OVERLENGTH as usize),
    );
    (*zc).seqStore.maxNbLit = blockSize;
    (*zc).bufferedPolicy = zbuff;
    (*zc).inBuffSize = buffInSize;
    (*zc).inBuff = ZSTD_cwksp_reserve_buffer(ws, buffInSize) as *mut std::ffi::c_char;
    (*zc).outBuffSize = buffOutSize;
    (*zc).outBuff = ZSTD_cwksp_reserve_buffer(ws, buffOutSize) as *mut std::ffi::c_char;
    if (*params).ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        let numBuckets = 1_usize
            << ((*params).ldmParams.hashLog)
                .wrapping_sub((*params).ldmParams.bucketSizeLog);
        (*zc).ldmState.bucketOffsets = ZSTD_cwksp_reserve_buffer(ws, numBuckets);
        libc::memset((*zc).ldmState.bucketOffsets, 0, (numBuckets) as usize);
    }
    ZSTD_referenceExternalSequences(
        zc,
        std::ptr::null_mut(),
        0,
    );
    (*zc).seqStore.maxNbSeq = maxNbSeq;
    (*zc)
        .seqStore
        .llCode = ZSTD_cwksp_reserve_buffer(
        ws,
        maxNbSeq.wrapping_mul(size_of::<u8>()),
    );
    (*zc)
        .seqStore
        .mlCode = ZSTD_cwksp_reserve_buffer(
        ws,
        maxNbSeq.wrapping_mul(size_of::<u8>()),
    );
    (*zc)
        .seqStore
        .ofCode = ZSTD_cwksp_reserve_buffer(
        ws,
        maxNbSeq.wrapping_mul(size_of::<u8>()),
    );
    (*zc).initialized = 1;
    return 0;
}

pub unsafe fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx) {
    let mut i: usize = 0;
    while i < ZSTD_REP_NUM {
        (*(*cctx).blockState.prevCBlock).rep[i] = 0;
        i += 1;
    }
    debug_assert!(!ZSTD_window_hasExtDict((*cctx).blockState.matchState.window));
}

static mut attachDictSizeCutoffs: [usize; 10] = [
    (8 as i32 * ((1 as i32) << 10)) as usize,
    (8 as i32 * ((1 as i32) << 10)) as usize,
    (16 as i32 * ((1 as i32) << 10))
        as usize,
    (32 as i32 * ((1 as i32) << 10))
        as usize,
    (32 as i32 * ((1 as i32) << 10))
        as usize,
    (32 as i32 * ((1 as i32) << 10))
        as usize,
    (32 as i32 * ((1 as i32) << 10))
        as usize,
    (32 as i32 * ((1 as i32) << 10))
        as usize,
    (8 as i32 * ((1 as i32) << 10)) as usize,
    (8 as i32 * ((1 as i32) << 10)) as usize,
];
unsafe fn ZSTD_shouldAttachDict(
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
) -> i32 {
    let mut cutoff = attachDictSizeCutoffs[(*cdict).matchState.cParams.strategy
        as usize];
    let dedicatedDictSearch = (*cdict).matchState.dedicatedDictSearch;
    return (dedicatedDictSearch != 0
        || (pledgedSrcSize <= cutoff
            || pledgedSrcSize as u64 == ZSTD_CONTENTSIZE_UNKNOWN
            || (*params).attachDictPref as u32
                == ZSTD_dictForceAttach as i32 as u32)
            && (*params).attachDictPref as u32
                != ZSTD_dictForceCopy as i32 as u32
            && (*params).forceWindow == 0) as i32;
}
unsafe fn ZSTD_resetCCtx_byAttachingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> usize {
    let mut adjusted_cdict_cParams = (*cdict).matchState.cParams;
    let windowLog = params.cParams.windowLog;
    if (*cdict).matchState.dedicatedDictSearch != 0 {
        ZSTD_dedicatedDictSearch_revertCParams(&mut adjusted_cdict_cParams);
    }
    params
        .cParams = ZSTD_adjustCParams_internal(
        adjusted_cdict_cParams,
        pledgedSrcSize as u64,
        (*cdict).dictContentSize,
        ZSTD_cpm_attachDict,
        params.useRowMatchFinder,
    );
    params.cParams.windowLog = windowLog;
    params.useRowMatchFinder = (*cdict).useRowMatchFinder;
    FORWARD_IF_ERROR!(
        ZSTD_resetCCtx_internal(cctx, addr_of!(params), pledgedSrcSize, 0, ZSTDcrp_makeClean,
        zbuff), ""
    );
    let cdictEnd = ((*cdict).matchState.window.nextSrc)
        .offset_from((*cdict).matchState.window.base) as std::ffi::c_long as u32;
    let cdictLen = cdictEnd.wrapping_sub((*cdict).matchState.window.dictLimit);
    if !(cdictLen == 0) {
        (*cctx).blockState.matchState.dictMatchState = &(*cdict).matchState;
        if (*cctx).blockState.matchState.window.dictLimit < cdictEnd {
            (*cctx)
                .blockState
                .matchState
                .window
                .nextSrc = ((*cctx).blockState.matchState.window.base)
                .offset(cdictEnd as isize);
            ZSTD_window_clear(&mut (*cctx).blockState.matchState.window);
        }
        (*cctx)
            .blockState
            .matchState
            .loadedDictEnd = (*cctx).blockState.matchState.window.dictLimit;
    }
    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;
    libc::memcpy(
        (*cctx).blockState.prevCBlock as *mut std::ffi::c_void,
        &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t
            as *const std::ffi::c_void,
        size_of::<ZSTD_compressedBlockState_t>()
            as usize,
    );
    return 0;
}
unsafe fn ZSTD_copyCDictTableIntoCCtx(
    mut dst: *mut u32,
    mut src: *const u32,
    mut tableSize: usize,
    mut cParams: *const ZSTD_compressionParameters,
) {
    if ZSTD_CDictIndicesAreTagged(cParams) != 0 {
        let mut i: usize = 0;
        i = 0;
        while i < tableSize {
            let taggedIndex = *src.offset(i as isize);
            let index = taggedIndex >> ZSTD_SHORT_CACHE_TAG_BITS;
            *dst.offset(i as isize) = index;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        libc::memcpy(
            dst as *mut std::ffi::c_void,
            src as *const std::ffi::c_void,
            tableSize.wrapping_mul(size_of::<u32>())
                as usize,
        );
    };
}
unsafe fn ZSTD_resetCCtx_byCopyingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> usize {
    let mut cdict_cParams: *const ZSTD_compressionParameters = &(*cdict)
        .matchState
        .cParams;
    let windowLog = params.cParams.windowLog;
    params.cParams = *cdict_cParams;
    params.cParams.windowLog = windowLog;
    params.useRowMatchFinder = (*cdict).useRowMatchFinder;
    FORWARD_IF_ERROR!(
        ZSTD_resetCCtx_internal(cctx, addr_of!(params), pledgedSrcSize, 0, ZSTDcrp_leaveDirty,
        zbuff), ""
    );
    ZSTD_cwksp_mark_tables_dirty(&mut (*cctx).workspace);
    let chainSize = if ZSTD_allocateChainTable(
        (*cdict_cParams).strategy,
        (*cdict).useRowMatchFinder,
        0,
    ) != 0
    {
        1_usize << (*cdict_cParams).chainLog
    } else {
        0_usize
    };
    let hSize = 1_usize << (*cdict_cParams).hashLog;
    ZSTD_copyCDictTableIntoCCtx(
        (*cctx).blockState.matchState.hashTable,
        (*cdict).matchState.hashTable,
        hSize,
        cdict_cParams,
    );
    if ZSTD_allocateChainTable(
        (*cctx).appliedParams.cParams.strategy,
        (*cctx).appliedParams.useRowMatchFinder,
        0,
    ) != 0
    {
        ZSTD_copyCDictTableIntoCCtx(
            (*cctx).blockState.matchState.chainTable,
            (*cdict).matchState.chainTable,
            chainSize,
            cdict_cParams,
        );
    }
    if ZSTD_rowMatchFinderUsed((*cdict_cParams).strategy, (*cdict).useRowMatchFinder)
        != 0
    {
        let tagTableSize = hSize;
        libc::memcpy((*cctx).blockState.matchState.tagTable, (*cdict).matchState.tagTable, (tagTableSize) as usize);
        (*cctx).blockState.matchState.hashSalt = (*cdict).matchState.hashSalt;
    }
    let h3log = (*cctx).blockState.matchState.hashLog3;
    let h3Size = if h3log != 0 {
        1_usize << h3log
    } else {
        0_usize
    };
    libc::memset(
        (*cctx).blockState.matchState.hashTable3 as *mut std::ffi::c_void,
        0,
        h3Size.wrapping_mul(size_of::<u32>())
            as usize,
    );
    ZSTD_cwksp_mark_tables_clean(&mut (*cctx).workspace);
    let mut srcMatchState: *const ZSTD_MatchState_t = &(*cdict).matchState;
    let mut dstMatchState: *mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
    (*dstMatchState).window = (*srcMatchState).window;
    (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
    (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;
    libc::memcpy(
        (*cctx).blockState.prevCBlock as *mut std::ffi::c_void,
        &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t
            as *const std::ffi::c_void,
        size_of::<ZSTD_compressedBlockState_t>()
            as usize,
    );
    return 0;
}
unsafe fn ZSTD_resetCCtx_usingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> usize {
    if ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) != 0 {
        return ZSTD_resetCCtx_byAttachingCDict(
            cctx,
            cdict,
            *params,
            pledgedSrcSize,
            zbuff,
        )
    } else {
        return ZSTD_resetCCtx_byCopyingCDict(cctx, cdict, *params, pledgedSrcSize, zbuff)
    };
}
unsafe fn ZSTD_copyCCtx_internal(
    mut dstCCtx: *mut ZSTD_CCtx,
    mut srcCCtx: *const ZSTD_CCtx,
    mut fParams: ZSTD_frameParameters,
    mut pledgedSrcSize: u64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> usize {
    RETURN_ERROR_IF!((*srcCCtx).stage as u32
        != ZSTDcs_init as i32 as u32, ZSTD_error_stage_wrong);
    libc::memcpy(
        &mut (*dstCCtx).customMem as *mut ZSTD_customMem as *mut std::ffi::c_void,
        &(*srcCCtx).customMem as *const ZSTD_customMem as *const std::ffi::c_void,
        size_of::<ZSTD_customMem>() as usize,
    );
    let mut params = (*dstCCtx).requestedParams;
    params.cParams = (*srcCCtx).appliedParams.cParams;
    params.useRowMatchFinder = (*srcCCtx).appliedParams.useRowMatchFinder;
    params.postBlockSplitter = (*srcCCtx).appliedParams.postBlockSplitter;
    params.ldmParams = (*srcCCtx).appliedParams.ldmParams;
    params.fParams = fParams;
    params.maxBlockSize = (*srcCCtx).appliedParams.maxBlockSize;
    ZSTD_resetCCtx_internal(
        dstCCtx,
        &mut params,
        pledgedSrcSize,
        0,
        ZSTDcrp_leaveDirty,
        zbuff,
    );
    ZSTD_cwksp_mark_tables_dirty(&mut (*dstCCtx).workspace);
    let chainSize = if ZSTD_allocateChainTable(
        (*srcCCtx).appliedParams.cParams.strategy,
        (*srcCCtx).appliedParams.useRowMatchFinder,
        0,
    ) != 0
    {
        1_usize << (*srcCCtx).appliedParams.cParams.chainLog
    } else {
        0_usize
    };
    let hSize = 1_usize
        << (*srcCCtx).appliedParams.cParams.hashLog;
    let h3log = (*srcCCtx).blockState.matchState.hashLog3;
    let h3Size = if h3log != 0 {
        1_usize << h3log
    } else {
        0_usize
    };
    libc::memcpy(
        (*dstCCtx).blockState.matchState.hashTable as *mut std::ffi::c_void,
        (*srcCCtx).blockState.matchState.hashTable as *const std::ffi::c_void,
        hSize.wrapping_mul(size_of::<u32>())
            as usize,
    );
    libc::memcpy(
        (*dstCCtx).blockState.matchState.chainTable as *mut std::ffi::c_void,
        (*srcCCtx).blockState.matchState.chainTable as *const std::ffi::c_void,
        chainSize.wrapping_mul(size_of::<u32>())
            as usize,
    );
    libc::memcpy(
        (*dstCCtx).blockState.matchState.hashTable3 as *mut std::ffi::c_void,
        (*srcCCtx).blockState.matchState.hashTable3 as *const std::ffi::c_void,
        h3Size.wrapping_mul(size_of::<u32>())
            as usize,
    );
    ZSTD_cwksp_mark_tables_clean(&mut (*dstCCtx).workspace);
    let mut srcMatchState: *const ZSTD_MatchState_t = &(*srcCCtx).blockState.matchState;
    let mut dstMatchState: *mut ZSTD_MatchState_t = &mut (*dstCCtx)
        .blockState
        .matchState;
    (*dstMatchState).window = (*srcMatchState).window;
    (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
    (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*dstCCtx).dictID = (*srcCCtx).dictID;
    (*dstCCtx).dictContentSize = (*srcCCtx).dictContentSize;
    libc::memcpy(
        (*dstCCtx).blockState.prevCBlock as *mut std::ffi::c_void,
        (*srcCCtx).blockState.prevCBlock as *const std::ffi::c_void,
        size_of::<ZSTD_compressedBlockState_t>()
            as usize,
    );
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_copyCCtx(
    mut dstCCtx: *mut ZSTD_CCtx,
    mut srcCCtx: *const ZSTD_CCtx,
    mut pledgedSrcSize: u64,
) -> usize {
    let mut fParams = {
        let mut init = ZSTD_frameParameters {
            contentSizeFlag: 1,
            checksumFlag: 0,
            noDictIDFlag: 0,
        };
        init
    };
    let zbuff = (*srcCCtx).bufferedPolicy;
    if pledgedSrcSize == 0 {
        pledgedSrcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    fParams
        .contentSizeFlag = (pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN)
        as i32;
    return ZSTD_copyCCtx_internal(
        dstCCtx,
        srcCCtx,
        fParams,
        pledgedSrcSize as u64,
        zbuff,
    );
}
pub const ZSTD_ROWSIZE: i32 = 16;
#[inline(always)]
unsafe fn ZSTD_reduceTable_internal(
    table: *mut u32,
    size: u32,
    reducerValue: u32,
    preserveMark: i32,
) {
    let nbRows = size as i32 / ZSTD_ROWSIZE;
    let mut cellNb: i32 = 0;
    let mut rowNb: i32 = 0;
    let reducerThreshold = reducerValue.wrapping_add(ZSTD_WINDOW_START_INDEX as u32);
    rowNb = 0;
    while rowNb < nbRows {
        let mut column: i32 = 0;
        column = 0;
        while column < ZSTD_ROWSIZE {
            let mut newVal: u32 = 0;
            if preserveMark != 0
                && *table.offset(cellNb as isize) == ZSTD_DUBT_UNSORTED_MARK as u32
            {
                newVal = ZSTD_DUBT_UNSORTED_MARK as u32;
            } else if *table.offset(cellNb as isize) < reducerThreshold {
                newVal = 0;
            } else {
                newVal = (*table.offset(cellNb as isize)).wrapping_sub(reducerValue);
            }
            *table.offset(cellNb as isize) = newVal;
            cellNb += 1;
            cellNb;
            column += 1;
            column;
        }
        rowNb += 1;
        rowNb;
    }
}
unsafe fn ZSTD_reduceTable(table: *mut u32, size: u32, reducerValue: u32) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 0);
}
unsafe fn ZSTD_reduceTable_btlazy2(
    table: *mut u32,
    size: u32,
    reducerValue: u32,
) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 1);
}
unsafe fn ZSTD_reduceIndex(
    mut ms: *mut ZSTD_MatchState_t,
    mut params: *const ZSTD_CCtx_params,
    reducerValue: u32,
) {
    let hSize = 1_u32 << (*params).cParams.hashLog;
    ZSTD_reduceTable((*ms).hashTable, hSize, reducerValue);
    if ZSTD_allocateChainTable(
        (*params).cParams.strategy,
        (*params).useRowMatchFinder,
        (*ms).dedicatedDictSearch as u32,
    ) != 0
    {
        let chainSize = 1_u32 << (*params).cParams.chainLog;
        if (*params).cParams.strategy as u32
            == ZSTD_btlazy2 as i32 as u32
        {
            ZSTD_reduceTable_btlazy2((*ms).chainTable, chainSize, reducerValue);
        } else {
            ZSTD_reduceTable((*ms).chainTable, chainSize, reducerValue);
        }
    }
    if (*ms).hashLog3 != 0 {
        let h3Size = 1_u32 << (*ms).hashLog3;
        ZSTD_reduceTable((*ms).hashTable3, h3Size, reducerValue);
    }
}
#[no_mangle]
pub unsafe fn ZSTD_seqToCodes(
    mut seqStorePtr: *const SeqStore_t,
) -> i32 {
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable = (*seqStorePtr).llCode;
    let ofCodeTable = (*seqStorePtr).ofCode;
    let mlCodeTable = (*seqStorePtr).mlCode;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as std::ffi::c_long as u32;
    let mut u: u32 = 0;
    let mut longOffsets: i32 = 0;
    u = 0;
    while u < nbSeq {
        let llv = (*sequences.offset(u as isize)).litLength as u32;
        let ofCode = ZSTD_highbit32((*sequences.offset(u as isize)).offBase);
        let mlv = (*sequences.offset(u as isize)).mlBase as u32;
        *llCodeTable.offset(u as isize) = ZSTD_LLcode(llv) as u8;
        *ofCodeTable.offset(u as isize) = ofCode as u8;
        *mlCodeTable.offset(u as isize) = ZSTD_MLcode(mlv) as u8;
        if MEM_32bits
            && ofCode
                >= (if MEM_32bits {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as u32
        {
            longOffsets = 1;
        }
        u = u.wrapping_add(1);
        u;
    }
    if (*seqStorePtr).longLengthType as u32
        == ZSTD_llt_literalLength as i32 as u32
    {
        *llCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxLL as u8;
    }
    if (*seqStorePtr).longLengthType as u32
        == ZSTD_llt_matchLength as i32 as u32
    {
        *mlCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxML as u8;
    }
    return longOffsets;
}
unsafe fn ZSTD_useTargetCBlockSize(
    mut cctxParams: *const ZSTD_CCtx_params,
) -> i32 {
    return ((*cctxParams).targetCBlockSize != 0)
        as i32;
}
unsafe fn ZSTD_blockSplitterEnabled(
    mut cctxParams: *mut ZSTD_CCtx_params,
) -> i32 {
    return ((*cctxParams).postBlockSplitter as u32
        == ZSTD_ps_enable as i32 as u32) as i32;
}
unsafe fn ZSTD_buildSequencesStatistics(
    mut seqStorePtr: *const SeqStore_t,
    mut nbSeq: usize,
    mut prevEntropy: *const ZSTD_fseCTables_t,
    mut nextEntropy: *mut ZSTD_fseCTables_t,
    mut dst: *mut u8,
    dstEnd: *const u8,
    mut strategy: ZSTD_strategy,
    mut countWorkspace: *mut u32,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWkspSize: usize,
) -> ZSTD_symbolEncodingTypeStats_t {
    let ostart = dst;
    let oend = dstEnd;
    let mut op = ostart;
    let mut CTable_LitLength = ((*nextEntropy).litlengthCTable).as_mut_ptr();
    let mut CTable_OffsetBits = ((*nextEntropy).offcodeCTable).as_mut_ptr();
    let mut CTable_MatchLength = ((*nextEntropy).matchlengthCTable).as_mut_ptr();
    let ofCodeTable: *const u8 = (*seqStorePtr).ofCode;
    let llCodeTable: *const u8 = (*seqStorePtr).llCode;
    let mlCodeTable: *const u8 = (*seqStorePtr).mlCode;
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        LLtype: 0,
        Offtype: 0,
        MLtype: 0,
        size: 0,
        lastCountSize: 0,
        longOffsets: 0,
    };
    stats.lastCountSize = 0;
    stats.longOffsets = ZSTD_seqToCodes(seqStorePtr);
    let mut max = MaxLL as u32;
    let mostFrequent = HIST_countFast_wksp(
        countWorkspace,
        &mut max,
        llCodeTable as *const std::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    (*nextEntropy).litlength_repeatMode = (*prevEntropy).litlength_repeatMode;
    stats
        .LLtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).litlength_repeatMode,
        countWorkspace,
        max,
        mostFrequent,
        nbSeq,
        LLFSELog as u32,
        ((*prevEntropy).litlengthCTable).as_ptr(),
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        ZSTD_defaultAllowed,
        strategy,
    ) as u32;
    let countSize = ZSTD_buildCTable(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        CTable_LitLength,
        LLFSELog as u32,
        stats.LLtype as SymbolEncodingType_e,
        countWorkspace,
        max,
        llCodeTable,
        nbSeq,
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        MaxLL as u32,
        ((*prevEntropy).litlengthCTable).as_ptr(),
        size_of::<[FSE_CTable; 329]>(),
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize) {
        stats.size = countSize;
        return stats;
    }
    if stats.LLtype == set_compressed as i32 as u32 {
        stats.lastCountSize = countSize;
    }
    op = op.offset(countSize as isize);
    let mut max_0 = MaxOff as u32;
    let mostFrequent_0 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_0,
        ofCodeTable as *const std::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    let defaultPolicy = (if max_0 <= DefaultMaxOff as u32 {
        ZSTD_defaultAllowed as i32
    } else {
        ZSTD_defaultDisallowed as i32
    }) as ZSTD_DefaultPolicy_e;
    (*nextEntropy).offcode_repeatMode = (*prevEntropy).offcode_repeatMode;
    stats
        .Offtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).offcode_repeatMode,
        countWorkspace,
        max_0,
        mostFrequent_0,
        nbSeq,
        OffFSELog as u32,
        ((*prevEntropy).offcodeCTable).as_ptr(),
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        defaultPolicy,
        strategy,
    ) as u32;
    let countSize_0 = ZSTD_buildCTable(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        CTable_OffsetBits,
        OffFSELog as u32,
        stats.Offtype as SymbolEncodingType_e,
        countWorkspace,
        max_0,
        ofCodeTable,
        nbSeq,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        DefaultMaxOff as u32,
        ((*prevEntropy).offcodeCTable).as_ptr(),
        size_of::<[FSE_CTable; 193]>(),
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_0) {
        stats.size = countSize_0;
        return stats;
    }
    if stats.Offtype == set_compressed as i32 as u32 {
        stats.lastCountSize = countSize_0;
    }
    op = op.offset(countSize_0 as isize);
    let mut max_1 = MaxML as u32;
    let mostFrequent_1 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_1,
        mlCodeTable as *const std::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    (*nextEntropy).matchlength_repeatMode = (*prevEntropy).matchlength_repeatMode;
    stats
        .MLtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).matchlength_repeatMode,
        countWorkspace,
        max_1,
        mostFrequent_1,
        nbSeq,
        MLFSELog as u32,
        ((*prevEntropy).matchlengthCTable).as_ptr(),
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        ZSTD_defaultAllowed,
        strategy,
    ) as u32;
    let countSize_1 = ZSTD_buildCTable(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        CTable_MatchLength,
        MLFSELog as u32,
        stats.MLtype as SymbolEncodingType_e,
        countWorkspace,
        max_1,
        mlCodeTable,
        nbSeq,
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        MaxML as u32,
        ((*prevEntropy).matchlengthCTable).as_ptr(),
        size_of::<[FSE_CTable; 363]>(),
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_1) {
        stats.size = countSize_1;
        return stats;
    }
    if stats.MLtype == set_compressed as i32 as u32 {
        stats.lastCountSize = countSize_1;
    }
    op = op.offset(countSize_1 as isize);
    stats.size = op.offset_from(ostart) as std::ffi::c_long as usize;
    return stats;
}
pub const SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO: i32 = 20;
#[inline]
unsafe fn ZSTD_entropyCompressSeqStore_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut literals: *const std::ffi::c_void,
    mut litSize: usize,
    mut seqStorePtr: *const SeqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWkspSize: usize,
    bmi2: i32,
) -> usize {
    let strategy = (*cctxParams).cParams.strategy;
    let mut count = entropyWorkspace as *mut u32;
    let mut CTable_LitLength = ((*nextEntropy).fse.litlengthCTable).as_mut_ptr();
    let mut CTable_OffsetBits = ((*nextEntropy).fse.offcodeCTable).as_mut_ptr();
    let mut CTable_MatchLength = ((*nextEntropy).fse.matchlengthCTable).as_mut_ptr();
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as std::ffi::c_long as usize;
    let ofCodeTable: *const u8 = (*seqStorePtr).ofCode;
    let llCodeTable: *const u8 = (*seqStorePtr).llCode;
    let mlCodeTable: *const u8 = (*seqStorePtr).mlCode;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstCapacity as isize);
    let mut op = ostart;
    let mut lastCountSize: usize = 0;
    let mut longOffsets: i32 = 0;
    entropyWorkspace = count
        .offset(
            ((if 35 as i32 > 52 {
                35 as i32
            } else {
                52 as i32
            }) + 1 as i32) as isize,
        ) as *mut std::ffi::c_void;
    entropyWkspSize = (entropyWkspSize as std::ffi::c_ulong)
        .wrapping_sub(
            (((if 35 as i32 > 52 {
                35 as i32
            } else {
                52 as i32
            }) + 1 as i32) as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<u32>(),
                ),
        ) as usize as usize;
    let numSequences = ((*seqStorePtr).sequences)
        .offset_from((*seqStorePtr).sequencesStart) as std::ffi::c_long as usize;
    let suspectUncompressible = (numSequences == 0
        || litSize / numSequences >= SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO as usize)
        as i32;
    let cSize = ZSTD_compressLiterals(
        op as *mut std::ffi::c_void,
        dstCapacity,
        literals,
        litSize,
        entropyWorkspace,
        entropyWkspSize,
        &(*prevEntropy).huf,
        &mut (*nextEntropy).huf,
        (*cctxParams).cParams.strategy,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        suspectUncompressible,
        bmi2,
    );
    FORWARD_IF_ERROR!(cSize, "ZSTD_compressLiterals failed");
    op = op.offset(cSize as isize);
    RETURN_ERROR_IF!((oend.offset_from(op) as std::ffi::c_long)
        < (3 as i32 + 1 as i32) as std::ffi::c_long, ZSTD_error_dstSize_tooSmall);
    if nbSeq < 128 {
        let fresh2 = op;
        op = op.offset(1);
        *fresh2 = nbSeq as u8;
    } else if nbSeq < LONGNBSEQ as usize {
        *op
            .offset(
                0,
            ) = (nbSeq >> 8)
            .wrapping_add(0x80 as i32 as usize) as u8;
        *op.offset(1) = nbSeq as u8;
        op = op.offset(2);
    } else {
        *op.offset(0) = 0xff as i32 as u8;
        MEM_writeLE16(
            op.offset(1) as *mut std::ffi::c_void,
            nbSeq.wrapping_sub(LONGNBSEQ as usize) as u16,
        );
        op = op.offset(3);
    }
    if nbSeq == 0 {
        libc::memcpy(
            &mut (*nextEntropy).fse as *mut ZSTD_fseCTables_t as *mut std::ffi::c_void,
            &(*prevEntropy).fse as *const ZSTD_fseCTables_t as *const std::ffi::c_void,
            size_of::<ZSTD_fseCTables_t>()
                as usize,
        );
        return op.offset_from(ostart) as std::ffi::c_long as usize;
    }
    let fresh3 = op;
    op = op.offset(1);
    let seqHead = fresh3;
    let stats = ZSTD_buildSequencesStatistics(
        seqStorePtr,
        nbSeq,
        &(*prevEntropy).fse,
        &mut (*nextEntropy).fse,
        op,
        oend,
        strategy,
        count,
        entropyWorkspace,
        entropyWkspSize,
    );
    FORWARD_IF_ERROR!(
        stats.size, "ZSTD_buildSequencesStatistics failed!"
    );
    *seqHead = (stats.LLtype << 6)
        .wrapping_add(stats.Offtype << 4)
        .wrapping_add(stats.MLtype << 2) as u8;
    lastCountSize = stats.lastCountSize;
    op = op.offset(stats.size as isize);
    longOffsets = stats.longOffsets;
    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
        bmi2,
    );
    FORWARD_IF_ERROR!(bitstreamSize, "ZSTD_encodeSequences failed");
    op = op.offset(bitstreamSize as isize);
    if lastCountSize != 0
        && lastCountSize.wrapping_add(bitstreamSize) < 4
    {
        return 0;
    }
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
unsafe fn ZSTD_entropyCompressSeqStore_wExtLitBuffer(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut literals: *const std::ffi::c_void,
    mut litSize: usize,
    mut blockSize: usize,
    mut seqStorePtr: *const SeqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWkspSize: usize,
    mut bmi2: i32,
) -> usize {
    let cSize = ZSTD_entropyCompressSeqStore_internal(
        dst,
        dstCapacity,
        literals,
        litSize,
        seqStorePtr,
        prevEntropy,
        nextEntropy,
        cctxParams,
        entropyWorkspace,
        entropyWkspSize,
        bmi2,
    );
    if cSize == 0 {
        return 0;
    }
    if (cSize == ERROR(ZSTD_error_dstSize_tooSmall)) as i32
        & (blockSize <= dstCapacity) as i32 != 0
    {
        return 0;
    }
    FORWARD_IF_ERROR!(
        cSize, "ZSTD_entropyCompressSeqStore_internal failed"
    );
    let maxCSize = blockSize
        .wrapping_sub(ZSTD_minGain(blockSize, (*cctxParams).cParams.strategy));
    if cSize >= maxCSize {
        return 0;
    }
    return cSize;
}
unsafe fn ZSTD_entropyCompressSeqStore(
    mut seqStorePtr: *const SeqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut srcSize: usize,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWkspSize: usize,
    mut bmi2: i32,
) -> usize {
    return ZSTD_entropyCompressSeqStore_wExtLitBuffer(
        dst,
        dstCapacity,
        (*seqStorePtr).litStart as *const std::ffi::c_void,
        ((*seqStorePtr).lit).offset_from((*seqStorePtr).litStart) as std::ffi::c_long
            as usize,
        srcSize,
        seqStorePtr,
        prevEntropy,
        nextEntropy,
        cctxParams,
        entropyWorkspace,
        entropyWkspSize,
        bmi2,
    );
}

/* ZSTD_selectBlockCompressor() :
 * Not static, but internal use only (used by long distance matcher)
 * assumption : strat is a valid strategy */
pub unsafe fn ZSTD_selectBlockCompressor(
    mut strat: ZSTD_strategy,
    mut useRowMatchFinder: ZSTD_ParamSwitch_e,
    mut dictMode: ZSTD_dictMode_e,
) -> ZSTD_BlockCompressor_f {
    static blockCompressor: [[ZSTD_BlockCompressor_f; ZSTD_STRATEGY_MAX as usize + 1]; 4] = [
        [
            Some(ZSTD_compressBlock_fast), /* default for 0 */
            Some(ZSTD_compressBlock_fast),
            Some(ZSTD_COMPRESSBLOCK_DOUBLEFAST),
            Some(ZSTD_COMPRESSBLOCK_GREEDY),
            Some(ZSTD_COMPRESSBLOCK_LAZY),
            Some(ZSTD_COMPRESSBLOCK_LAZY2),
            Some(ZSTD_COMPRESSBLOCK_BTLAZY2),
            Some(ZSTD_COMPRESSBLOCK_BTOPT),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA2),
        ],
        [
            Some(ZSTD_compressBlock_fast_extDict), /* default for 0 */
            Some(ZSTD_compressBlock_fast_extDict),
            Some(ZSTD_COMPRESSBLOCK_DOUBLEFAST_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_LAZY_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTLAZY2_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTOPT_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT),
        ],
        [
            Some(ZSTD_compressBlock_fast_dictMatchState), /* default for 0 */
            Some(ZSTD_compressBlock_fast_dictMatchState),
            Some(ZSTD_COMPRESSBLOCK_DOUBLEFAST_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTLAZY2_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTOPT_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE),
        ],
        [
            None,
            None,
            None,
            Some(ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH),
            Some(ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH),
            Some(ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH),
            None,
            None,
            None,
            None,
        ],
    ];
    let mut selectedCompressor: ZSTD_BlockCompressor_f = None;
    const _: () = assert!(ZSTD_fast == 1);

    debug_assert!(ZSTD_cParam_withinBounds(ZSTD_c_strategy, strat as i32));
    if ZSTD_rowMatchFinderUsed(strat, useRowMatchFinder) != 0 {
        static mut rowBasedBlockCompressors: [[ZSTD_BlockCompressor_f; 3]; 4] = [
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_ROW),
            ],
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_EXTDICT_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT_ROW),
            ],
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE_ROW),
            ],
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH_ROW),
            ],
        ];
        DEBUGLOG!(5, "Selecting a row-based matchfinder");
        debug_assert!(useRowMatchFinder != ZSTD_ps_auto);
        selectedCompressor = rowBasedBlockCompressors[dictMode as usize][strat as usize - ZSTD_greedy as usize];
    } else {
        selectedCompressor = blockCompressor[dictMode as usize][strat as usize];
    }
    debug_assert!(selectedCompressor.is_some());
    return selectedCompressor;
}

unsafe fn ZSTD_storeLastLiterals(
    mut seqStorePtr: *mut SeqStore_t,
    mut anchor: *const u8,
    mut lastLLSize: usize,
) {
    libc::memcpy((*seqStorePtr).lit, anchor, (lastLLSize) as usize);
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(lastLLSize as isize);
}

pub unsafe fn ZSTD_resetSeqStore(mut ssPtr: *mut SeqStore_t) {
    (*ssPtr).lit = (*ssPtr).litStart;
    (*ssPtr).sequences = (*ssPtr).sequencesStart;
    (*ssPtr).longLengthType = ZSTD_llt_none;
}

unsafe fn ZSTD_postProcessSequenceProducerResult(
    mut outSeqs: *mut ZSTD_Sequence,
    mut nbExternalSeqs: usize,
    mut outSeqsCapacity: usize,
    mut srcSize: usize,
) -> usize {
    RETURN_ERROR_IF!(nbExternalSeqs > outSeqsCapacity, ZSTD_error_sequenceProducer_failed);
    RETURN_ERROR_IF!(nbExternalSeqs == 0
        && srcSize > 0, ZSTD_error_sequenceProducer_failed);
    if srcSize == 0 {
        libc::memset(
            &mut *outSeqs.offset(0) as *mut ZSTD_Sequence
                as *mut std::ffi::c_void,
            0,
            size_of::<ZSTD_Sequence>() as usize,
        );
        return 1;
    }
    let lastSeq = *outSeqs
        .offset(nbExternalSeqs.wrapping_sub(1) as isize);
    if lastSeq.offset == 0
        && lastSeq.matchLength == 0
    {
        return nbExternalSeqs;
    }
    RETURN_ERROR_IF!(nbExternalSeqs == outSeqsCapacity, ZSTD_error_sequenceProducer_failed);
    libc::memset(
        &mut *outSeqs.offset(nbExternalSeqs as isize) as *mut ZSTD_Sequence
            as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_Sequence>() as usize,
    );
    return nbExternalSeqs.wrapping_add(1);
}
unsafe fn ZSTD_fastSequenceLengthSum(
    mut seqBuf: *const ZSTD_Sequence,
    mut seqBufSize: usize,
) -> usize {
    let mut matchLenSum: usize = 0;
    let mut litLenSum: usize = 0;
    let mut i: usize = 0;
    matchLenSum = 0;
    litLenSum = 0;
    i = 0;
    while i < seqBufSize {
        litLenSum = litLenSum
            .wrapping_add((*seqBuf.offset(i as isize)).litLength as usize);
        matchLenSum = matchLenSum
            .wrapping_add((*seqBuf.offset(i as isize)).matchLength as usize);
        i = i.wrapping_add(1);
        i;
    }
    return litLenSum.wrapping_add(matchLenSum);
}
unsafe fn ZSTD_validateSeqStore(
    mut seqStore: *const SeqStore_t,
    mut cParams: *const ZSTD_compressionParameters,
) {}
unsafe fn ZSTD_buildSeqStore(
    mut zc: *mut ZSTD_CCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let ms: *mut ZSTD_MatchState_t = &mut (*zc).blockState.matchState;
    ZSTD_assertEqualCParams((*zc).appliedParams.cParams, (*ms).cParams);
    if srcSize
        < (MIN_CBLOCK_SIZE as usize)
            .wrapping_add(ZSTD_blockHeaderSize)
            .wrapping_add(1)
            .wrapping_add(1)
    {
        if (*zc).appliedParams.cParams.strategy as u32
            >= ZSTD_btopt as i32 as u32
        {
            ZSTD_ldm_skipRawSeqStoreBytes(&mut (*zc).externSeqStore, srcSize);
        } else {
            ZSTD_ldm_skipSequences(
                &mut (*zc).externSeqStore,
                srcSize,
                (*zc).appliedParams.cParams.minMatch,
            );
        }
        return ZSTDbss_noCompress as i32 as usize;
    }
    ZSTD_resetSeqStore(&mut (*zc).seqStore);
    (*ms).opt.symbolCosts = &mut (*(*zc).blockState.prevCBlock).entropy;
    (*ms).opt.literalCompressionMode = (*zc).appliedParams.literalCompressionMode;
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let curr = istart.offset_from(base) as std::ffi::c_long as u32;
    size_of::<ptrdiff_t>()
        == 8;
    if curr > ((*ms).nextToUpdate).wrapping_add(384) {
        (*ms)
            .nextToUpdate = curr
            .wrapping_sub(
                (if 192_u32
                    < curr
                        .wrapping_sub((*ms).nextToUpdate)
                        .wrapping_sub(384)
                {
                    192_u32
                } else {
                    curr.wrapping_sub((*ms).nextToUpdate)
                        .wrapping_sub(384)
                }),
            );
    }
    let dictMode = ZSTD_matchState_dictMode(ms);
    let mut lastLLSize: usize = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < ZSTD_REP_NUM {
        (*(*zc).blockState.nextCBlock)
            .rep[i as usize] = (*(*zc).blockState.prevCBlock).rep[i as usize];
        i += 1;
        i;
    }
    if (*zc).externSeqStore.pos < (*zc).externSeqStore.size {
        if ZSTD_hasExtSeqProd(&mut (*zc).appliedParams) != 0 {
            return -(ZSTD_error_parameter_combination_unsupported as i32)
                as usize;
        }
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut (*zc).externSeqStore,
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
    } else if (*zc).appliedParams.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        let mut ldmSeqStore = kNullRawSeqStore;
        if ZSTD_hasExtSeqProd(&mut (*zc).appliedParams) != 0 {
            return -(ZSTD_error_parameter_combination_unsupported as i32)
                as usize;
        }
        ldmSeqStore.seq = (*zc).ldmSequences;
        ldmSeqStore.capacity = (*zc).maxNbLdmSequences;
        FORWARD_IF_ERROR!(
            ZSTD_ldm_generateSequences(addr_of!((*zc).ldmState), addr_of!(ldmSeqStore), addr_of!((*zc).appliedParams.ldmParams), src, srcSize), ""
        );
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut ldmSeqStore,
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
    } else if ZSTD_hasExtSeqProd(&mut (*zc).appliedParams) != 0 {
        let windowSize = 1_u32
            << (*zc).appliedParams.cParams.windowLog;
        let nbExternalSeqs = ((*zc).appliedParams.extSeqProdFunc)
            .expect(
                "non-null function pointer",
            )(
            (*zc).appliedParams.extSeqProdState,
            (*zc).extSeqBuf,
            (*zc).extSeqBufCapacity,
            src,
            srcSize,
            std::ptr::null(),
            0,
            (*zc).appliedParams.compressionLevel,
            windowSize as usize,
        );
        let nbPostProcessedSeqs = ZSTD_postProcessSequenceProducerResult(
            (*zc).extSeqBuf,
            nbExternalSeqs,
            (*zc).extSeqBufCapacity,
            srcSize,
        );
        if ERR_isError(nbPostProcessedSeqs) == 0 {
            let mut seqPos = {
                let mut init = ZSTD_SequencePosition {
                    idx: 0,
                    posInSequence: 0,
                    posInSrc: 0,
                };
                init
            };
            let seqLenSum = ZSTD_fastSequenceLengthSum(
                (*zc).extSeqBuf,
                nbPostProcessedSeqs,
            );
            if seqLenSum > srcSize {
                return -(ZSTD_error_externalSequences_invalid as i32)
                    as usize;
            }
            FORWARD_IF_ERROR!(
                ZSTD_transferSequences_wBlockDelim(zc, addr_of!(seqPos), (*zc).extSeqBuf,
                nbPostProcessedSeqs, src, srcSize, (*zc).appliedParams
                .searchForExternalRepcodes),
                "Failed to copy external sequences to seqStore!"
            );
            (*ms).ldmSeqStore = std::ptr::null();
            return ZSTDbss_compress as i32 as usize;
        }
        if (*zc).appliedParams.enableMatchFinderFallback == 0 {
            return nbPostProcessedSeqs;
        }
        let blockCompressor = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        (*ms).ldmSeqStore = std::ptr::null();
        lastLLSize = blockCompressor
            .expect(
                "non-null function pointer",
            )(
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            src,
            srcSize,
        );
    } else {
        let blockCompressor_0 = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        (*ms).ldmSeqStore = std::ptr::null();
        lastLLSize = blockCompressor_0
            .expect(
                "non-null function pointer",
            )(
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            src,
            srcSize,
        );
    }
    let lastLiterals = (src as *const u8)
        .offset(srcSize as isize)
        .offset(-(lastLLSize as isize));
    ZSTD_storeLastLiterals(&mut (*zc).seqStore, lastLiterals, lastLLSize);
    ZSTD_validateSeqStore(&mut (*zc).seqStore, &mut (*zc).appliedParams.cParams);
    return ZSTDbss_compress as i32 as usize;
}
unsafe fn ZSTD_copyBlockSequences(
    mut seqCollector: *mut SeqCollector,
    mut seqStore: *const SeqStore_t,
    mut prevRepcodes: *const u32,
) -> usize {
    let mut inSeqs: *const SeqDef = (*seqStore).sequencesStart;
    let nbInSequences = ((*seqStore).sequences).offset_from(inSeqs) as std::ffi::c_long
        as usize;
    let nbInLiterals = ((*seqStore).lit).offset_from((*seqStore).litStart)
        as std::ffi::c_long as usize;
    let mut outSeqs = if (*seqCollector).seqIndex == 0 {
        (*seqCollector).seqStart
    } else {
        ((*seqCollector).seqStart).offset((*seqCollector).seqIndex as isize)
    };
    let nbOutSequences = nbInSequences.wrapping_add(1);
    let mut nbOutLiterals: usize = 0;
    let mut repcodes = repcodes_s { rep: [0; 3] };
    let mut i: usize = 0;
    RETURN_ERROR_IF!(nbOutSequences
        > ((*seqCollector).maxSequences).wrapping_sub((*seqCollector).seqIndex), ZSTD_error_dstSize_tooSmall);
    libc::memcpy(
        &mut repcodes as *mut Repcodes_t as *mut std::ffi::c_void,
        prevRepcodes as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    i = 0;
    while i < nbInSequences {
        let mut rawOffset: u32 = 0;
        (*outSeqs.offset(i as isize))
            .litLength = (*inSeqs.offset(i as isize)).litLength as u32;
        (*outSeqs.offset(i as isize))
            .matchLength = ((*inSeqs.offset(i as isize)).mlBase as i32
            + MINMATCH) as u32;
        (*outSeqs.offset(i as isize)).rep = 0;
        if i == (*seqStore).longLengthPos as usize {
            if (*seqStore).longLengthType as u32
                == ZSTD_llt_literalLength as i32 as u32
            {
                let ref mut fresh4 = (*outSeqs.offset(i as isize)).litLength;
                *fresh4 = (*fresh4)
                    .wrapping_add(0x10000 as i32 as u32);
            } else if (*seqStore).longLengthType as u32
                == ZSTD_llt_matchLength as i32 as u32
            {
                let ref mut fresh5 = (*outSeqs.offset(i as isize)).matchLength;
                *fresh5 = (*fresh5)
                    .wrapping_add(0x10000 as i32 as u32);
            }
        }
        if OFFBASE_IS_REPCODE!(inSeqs[i].offBase) != 0 {
            let repcode = OFFBASE_TO_REPCODE!(inSeqs[i].offBase);
            (*outSeqs.offset(i as isize)).rep = repcode;
            if (*outSeqs.offset(i as isize)).litLength
                != 0
            {
                rawOffset = repcodes
                    .rep[repcode.wrapping_sub(1) as usize];
            } else if repcode == 3 {
                rawOffset = (repcodes.rep[0])
                    .wrapping_sub(1);
            } else {
                rawOffset = repcodes.rep[repcode as usize];
            }
        } else {
            rawOffset = OFFBASE_TO_OFFSET!(inSeqs[i].offBase);
        }
        (*outSeqs.offset(i as isize)).offset = rawOffset;
        ZSTD_updateRep(
            (repcodes.rep).as_mut_ptr(),
            (*inSeqs.offset(i as isize)).offBase,
            ((*inSeqs.offset(i as isize)).litLength as i32
                == 0) as i32 as u32,
        );
        nbOutLiterals = nbOutLiterals
            .wrapping_add((*outSeqs.offset(i as isize)).litLength as usize);
        i = i.wrapping_add(1);
        i;
    }
    let lastLLSize = nbInLiterals.wrapping_sub(nbOutLiterals);
    (*outSeqs.offset(nbInSequences as isize)).litLength = lastLLSize as u32;
    (*outSeqs.offset(nbInSequences as isize))
        .matchLength = 0;
    (*outSeqs.offset(nbInSequences as isize))
        .offset = 0;
    (*seqCollector).seqIndex = ((*seqCollector).seqIndex).wrapping_add(nbOutSequences);
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_sequenceBound(mut srcSize: usize) -> usize {
    let maxNbSeq = (srcSize / ZSTD_MINMATCH_MIN as usize)
        .wrapping_add(1);
    let maxNbDelims = (srcSize / ZSTD_BLOCKSIZE_MAX_MIN as usize)
        .wrapping_add(1);
    return maxNbSeq.wrapping_add(maxNbDelims);
}
#[no_mangle]
pub unsafe fn ZSTD_generateSequences(
    mut zc: *mut ZSTD_CCtx,
    mut outSeqs: *mut ZSTD_Sequence,
    mut outSeqsSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let dstCapacity = ZSTD_compressBound(srcSize);
    let mut dst = std::ptr::null_mut();
    let mut seqCollector = SeqCollector {
        collectSequences: 0,
        seqStart: std::ptr::null_mut(),
        seqIndex: 0,
        maxSequences: 0,
    };
    let mut targetCBlockSize: i32 = 0;
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_getParameter(zc, ZSTD_c_targetCBlockSize, addr_of!(targetCBlockSize)), ""
    );
    RETURN_ERROR_IF!(targetCBlockSize != 0, ZSTD_error_parameter_unsupported);
    let mut nbWorkers: i32 = 0;
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_getParameter(zc, ZSTD_c_nbWorkers, addr_of!(nbWorkers)), ""
    );
    RETURN_ERROR_IF!(nbWorkers != 0, ZSTD_error_parameter_unsupported);
    dst = ZSTD_customMalloc(dstCapacity, ZSTD_defaultCMem);
    RETURN_ERROR_IF!(dst.is_null(), ZSTD_error_memory_allocation);
    seqCollector.collectSequences = 1;
    seqCollector.seqStart = outSeqs;
    seqCollector.seqIndex = 0;
    seqCollector.maxSequences = outSeqsSize;
    (*zc).seqCollector = seqCollector;
    let ret = ZSTD_compress2(zc, dst, dstCapacity, src, srcSize);
    ZSTD_customFree(dst, ZSTD_defaultCMem);
    FORWARD_IF_ERROR!(ret, "ZSTD_compress2 failed");
    return (*zc).seqCollector.seqIndex;
}
#[no_mangle]
pub unsafe fn ZSTD_mergeBlockDelimiters(
    mut sequences: *mut ZSTD_Sequence,
    mut seqsSize: usize,
) -> usize {
    let mut in_0: usize = 0;
    let mut out: usize = 0;
    while in_0 < seqsSize {
        if (*sequences.offset(in_0 as isize)).offset
            == 0
            && (*sequences.offset(in_0 as isize)).matchLength
                == 0
        {
            if in_0 != seqsSize.wrapping_sub(1) {
                let ref mut fresh6 = (*sequences
                    .offset(in_0.wrapping_add(1) as isize))
                    .litLength;
                *fresh6 = (*fresh6)
                    .wrapping_add((*sequences.offset(in_0 as isize)).litLength);
            }
        } else {
            *sequences.offset(out as isize) = *sequences.offset(in_0 as isize);
            out = out.wrapping_add(1);
            out;
        }
        in_0 = in_0.wrapping_add(1);
        in_0;
    }
    return out;
}
unsafe fn ZSTD_isRLE(
    mut src: *const u8,
    mut length: usize,
) -> i32 {
    let mut ip = src;
    let value = *ip.offset(0);
    let valueST = (value as u64 as u64)
        .wrapping_mul(0x101010101010101 as u64) as usize;
    let unrollSize = (size_of::<usize>())
        .wrapping_mul(4);
    let unrollMask = unrollSize.wrapping_sub(1);
    let prefixLength = length & unrollMask;
    let mut i: usize = 0;
    if length == 1 {
        return 1;
    }
    if prefixLength != 0
        && ZSTD_count(
            ip.offset(1),
            ip,
            ip.offset(prefixLength as isize),
        ) != prefixLength.wrapping_sub(1)
    {
        return 0;
    }
    i = prefixLength;
    while i != length {
        let mut u: usize = 0;
        u = 0;
        while u < unrollSize {
            if MEM_readST(
                ip.offset(i as isize).offset(u as isize) as *const std::ffi::c_void,
            ) != valueST
            {
                return 0;
            }
            u = (u as std::ffi::c_ulong)
                .wrapping_add(size_of::<usize>())
                as usize as usize;
        }
        i = i.wrapping_add(unrollSize);
    }
    return 1;
}
unsafe fn ZSTD_maybeRLE(mut seqStore: *const SeqStore_t) -> i32 {
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
        as std::ffi::c_long as usize;
    let nbLits = ((*seqStore).lit).offset_from((*seqStore).litStart) as std::ffi::c_long
        as usize;
    return (nbSeqs < 4
        && nbLits < 10) as i32;
}
unsafe fn ZSTD_blockState_confirmRepcodesAndEntropyTables(
    bs: *mut ZSTD_blockState_t,
) {
    let tmp = (*bs).prevCBlock;
    (*bs).prevCBlock = (*bs).nextCBlock;
    (*bs).nextCBlock = tmp;
}
unsafe fn writeBlockHeader(
    mut op: *mut std::ffi::c_void,
    mut cSize: usize,
    mut blockSize: usize,
    mut lastBlock: u32,
) {
    let cBlockHeader = if cSize == 1 {
        lastBlock
            .wrapping_add((bt_rle as i32 as u32) << 1)
            .wrapping_add((blockSize << 3) as u32)
    } else {
        lastBlock
            .wrapping_add(
                (bt_compressed as i32 as u32) << 1,
            )
            .wrapping_add((cSize << 3) as u32)
    };
    MEM_writeLE24(op, cBlockHeader);
}

/** ZSTD_buildBlockEntropyStats_literals() :
 *  Builds entropy for the literals.
 *  Stores literals block type (raw, rle, compressed, repeat) and
 *  huffman description table to hufMetadata.
 *  Requires ENTROPY_WORKSPACE_SIZE workspace
 * @return : size of huffman description table, or an error code
 */
pub unsafe fn ZSTD_buildBlockEntropyStats_literals(
    src: *mut std::ffi::c_void,
    mut srcSize: usize,
    mut prevHuf: *const ZSTD_hufCTables_t,
    mut nextHuf: *mut ZSTD_hufCTables_t,
    mut hufMetadata: *mut ZSTD_hufCTablesMetadata_t,
    literalsCompressionIsDisabled: i32,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut hufFlags: i32,
) -> usize {
    let wkspStart = workspace as *mut u8;
    let wkspEnd = wkspStart.add(wkspSize);
    let countWkspStart = wkspStart;
    let countWksp = workspace as *mut u32;
    let countWkspSize = (HUF_SYMBOLVALUE_MAX as usize + 1)
        .wrapping_mul(size_of::<u32>());
    let nodeWksp = countWkspStart.add(countWkspSize);
    let nodeWkspSize = wkspEnd.offset_from(nodeWksp) as std::ffi::c_long as usize;
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX as u32;
    let mut huffLog = LitHufLog as u32;
    let mut repeat = (*prevHuf).repeatMode;
    DEBUGLOG!(5, "ZSTD_buildBlockEntropyStats_literals (srcSize=%zu)", srcSize);

    /* Prepare nextEntropy assuming reusing the existing table */
    libc::memcpy(
        nextHuf as *mut std::ffi::c_void,
        prevHuf as *const std::ffi::c_void,
        size_of::<ZSTD_hufCTables_t>(),
    );
    if literalsCompressionIsDisabled != 0 {
        DEBUGLOG!(5, "set_basic - disabled");
        (*hufMetadata).hType = set_basic;
        return 0;
    }

    /* small ? don't even attempt compression (speed opt) */
    pub const COMPRESS_LITERALS_SIZE_MIN: usize = 63; /* heuristic */
    let minLitSize = if (*prevHuf).repeatMode
        == HUF_repeat_valid
    {
        6
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    };
    if srcSize <= minLitSize {
        DEBUGLOG!(5, "set_basic - too small");
        (*hufMetadata).hType = set_basic;
        return 0;
    }

    /* Scan input and build symbol stats */
    let largest = HIST_count_wksp(
        countWksp,
        &mut maxSymbolValue,
        src as *const u8 as *const std::ffi::c_void,
        srcSize,
        workspace,
        wkspSize,
    );
    FORWARD_IF_ERROR!(largest, "HIST_count_wksp failed");
    if largest == srcSize {
        /* only one literal symbol */
        DEBUGLOG!(5, "set_rle");
        (*hufMetadata).hType = set_rle;
        return 0;
    }
    if largest <= (srcSize >> 7).wrapping_add(4) {
        /* heuristic: likely not compressible */
        DEBUGLOG!(5, "set_basic - no gain");
        (*hufMetadata).hType = set_basic;
        return 0;
    }

    /* Validate the previous Huffman table */
    if repeat == HUF_repeat_check
        && HUF_validateCTable(((*prevHuf).CTable).as_ptr(), countWksp, maxSymbolValue)
            == 0
    {
        repeat = HUF_repeat_none;
    }

    /* Build Huffman Tree */
    libc::memset(
        ((*nextHuf).CTable).as_mut_ptr() as *mut std::ffi::c_void,
        0,
        size_of::<[HUF_CElt; 257]>(), // TODO
    );
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        nodeWksp as *mut std::ffi::c_void,
        nodeWkspSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        hufFlags,
    );
    debug_assert!(huffLog <= LitHufLog);
    let maxBits = HUF_buildCTable_wksp(
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut std::ffi::c_void,
        nodeWkspSize,
    );
    FORWARD_IF_ERROR!(maxBits, "HUF_buildCTable_wksp");
    huffLog = maxBits as u32;

    /* Build and write the CTable */
    let newCSize = HUF_estimateCompressedSize(
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        maxSymbolValue,
    );
    let hSize = HUF_writeCTable_wksp(
        ((*hufMetadata).hufDesBuffer).as_mut_ptr() as *mut std::ffi::c_void,
        size_of::<[u8; 128]>(),
        ((*nextHuf).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut std::ffi::c_void,
        nodeWkspSize,
    );
    /* Check against repeating the previous CTable */
    if repeat
        != HUF_repeat_none
    {
        let oldCSize = HUF_estimateCompressedSize(
            ((*prevHuf).CTable).as_ptr(),
            countWksp,
            maxSymbolValue,
        );
        if oldCSize < srcSize
            && (oldCSize <= hSize.wrapping_add(newCSize)
                || hSize.wrapping_add(12) >= srcSize)
        {
            DEBUGLOG!(5, "set_repeat - smaller");
            libc::memcpy(
                nextHuf as *mut std::ffi::c_void,
                prevHuf as *const std::ffi::c_void,
                size_of::<ZSTD_hufCTables_t>(),
            );
            (*hufMetadata).hType = set_repeat;
            return 0;
        }
    }
    if newCSize.wrapping_add(hSize) >= srcSize {
        DEBUGLOG!(5, "set_basic - no gains");
        libc::memcpy(
            nextHuf as *mut std::ffi::c_void,
            prevHuf as *const std::ffi::c_void,
            size_of::<ZSTD_hufCTables_t>()
                as usize,
        );
        (*hufMetadata).hType = set_basic;
        return 0;
    }
    DEBUGLOG!(5, "set_compressed (hSize=%u)", hSize);
    (*hufMetadata).hType = set_compressed;
    (*nextHuf).repeatMode = HUF_repeat_check;
    return hSize;
}

unsafe fn ZSTD_buildDummySequencesStatistics(
    mut nextEntropy: *mut ZSTD_fseCTables_t,
) -> ZSTD_symbolEncodingTypeStats_t {
    let mut stats = {
        let mut init = ZSTD_symbolEncodingTypeStats_t {
            LLtype: set_basic as i32 as u32,
            Offtype: set_basic as i32 as u32,
            MLtype: set_basic as i32 as u32,
            size: 0,
            lastCountSize: 0,
            longOffsets: 0,
        };
        init
    };
    (*nextEntropy).litlength_repeatMode = FSE_repeat_none;
    (*nextEntropy).offcode_repeatMode = FSE_repeat_none;
    (*nextEntropy).matchlength_repeatMode = FSE_repeat_none;
    return stats;
}
unsafe fn ZSTD_buildBlockEntropyStats_sequences(
    mut seqStorePtr: *const SeqStore_t,
    mut prevEntropy: *const ZSTD_fseCTables_t,
    mut nextEntropy: *mut ZSTD_fseCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut fseMetadata: *mut ZSTD_fseCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let strategy = (*cctxParams).cParams.strategy;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as std::ffi::c_long as usize;
    let ostart = ((*fseMetadata).fseTablesBuffer).as_mut_ptr();
    let oend = ostart
        .offset(size_of::<[u8; 133]>() as isize);
    let mut op = ostart;
    let mut countWorkspace = workspace as *mut u32;
    let mut entropyWorkspace = countWorkspace
        .offset(
            ((if 35 as i32 > 52 {
                35 as i32
            } else {
                52 as i32
            }) + 1 as i32) as isize,
        );
    let mut entropyWorkspaceSize = wkspSize
        .wrapping_sub(
            (((if 35 as i32 > 52 {
                35 as i32
            } else {
                52 as i32
            }) + 1 as i32) as std::ffi::c_ulong)
                .wrapping_mul(
                    size_of::<u32>(),
                ),
        );
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        LLtype: 0,
        Offtype: 0,
        MLtype: 0,
        size: 0,
        lastCountSize: 0,
        longOffsets: 0,
    };
    stats = if nbSeq != 0 {
        ZSTD_buildSequencesStatistics(
            seqStorePtr,
            nbSeq,
            prevEntropy,
            nextEntropy,
            op,
            oend,
            strategy,
            countWorkspace,
            entropyWorkspace as *mut std::ffi::c_void,
            entropyWorkspaceSize,
        )
    } else {
        ZSTD_buildDummySequencesStatistics(nextEntropy)
    };
    FORWARD_IF_ERROR!(
        stats.size, "ZSTD_buildSequencesStatistics failed!"
    );
    (*fseMetadata).llType = stats.LLtype as SymbolEncodingType_e;
    (*fseMetadata).ofType = stats.Offtype as SymbolEncodingType_e;
    (*fseMetadata).mlType = stats.MLtype as SymbolEncodingType_e;
    (*fseMetadata).lastCountSize = stats.lastCountSize;
    return stats.size;
}

#[no_mangle]
pub unsafe fn ZSTD_buildBlockEntropyStats(
    mut seqStorePtr: *const SeqStore_t,
    mut prevEntropy: *const ZSTD_entropyCTables_t,
    mut nextEntropy: *mut ZSTD_entropyCTables_t,
    mut cctxParams: *const ZSTD_CCtx_params,
    mut entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let litSize = ((*seqStorePtr).lit).offset_from((*seqStorePtr).litStart)
        as std::ffi::c_long as usize;
    let huf_useOptDepth = ((*cctxParams).cParams.strategy as u32
        >= HUF_OPTIMAL_DEPTH_THRESHOLD as u32) as i32;
    let hufFlags = if huf_useOptDepth != 0 {
        HUF_flags_optimalDepth as i32
    } else {
        0 as i32
    };
    (*entropyMetadata)
        .hufMetadata
        .hufDesSize = ZSTD_buildBlockEntropyStats_literals(
        (*seqStorePtr).litStart as *mut std::ffi::c_void,
        litSize,
        &(*prevEntropy).huf,
        &mut (*nextEntropy).huf,
        &mut (*entropyMetadata).hufMetadata,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        workspace,
        wkspSize,
        hufFlags,
    );
    FORWARD_IF_ERROR!(
 (*       entropyMetadata).hufMetadata.hufDesSize,
        "ZSTD_buildBlockEntropyStats_literals failed"
    );
    (*entropyMetadata)
        .fseMetadata
        .fseTablesSize = ZSTD_buildBlockEntropyStats_sequences(
        seqStorePtr,
        &(*prevEntropy).fse,
        &mut (*nextEntropy).fse,
        cctxParams,
        &mut (*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
    );
    FORWARD_IF_ERROR!(
 (*       entropyMetadata).fseMetadata.fseTablesSize,
        "ZSTD_buildBlockEntropyStats_sequences failed"
    );
    return 0;
}
unsafe fn ZSTD_estimateBlockSize_literal(
    mut literals: *const u8,
    mut litSize: usize,
    mut huf: *const ZSTD_hufCTables_t,
    mut hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut writeEntropy: i32,
) -> usize {
    let countWksp = workspace as *mut u32;
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX as u32;
    let mut literalSectionHeaderSize = (3 as i32
        + (litSize
            >= (1 as i32 * ((1 as i32) << 10))
                as usize) as i32
        + (litSize
            >= (16 as i32
                * ((1 as i32) << 10)) as usize)
            as i32) as usize;
    let mut singleStream = (litSize < 256)
        as i32 as u32;
    if (*hufMetadata).hType as u32
        == set_basic as i32 as u32
    {
        return litSize
    } else if (*hufMetadata).hType as u32
        == set_rle as i32 as u32
    {
        return 1_usize
    } else if (*hufMetadata).hType as u32
        == set_compressed as i32 as u32
        || (*hufMetadata).hType as u32
            == set_repeat as i32 as u32
    {
        let largest = HIST_count_wksp(
            countWksp,
            &mut maxSymbolValue,
            literals as *const std::ffi::c_void,
            litSize,
            workspace,
            wkspSize,
        );
        if ERR_isError(largest) {
            return litSize;
        }
        let mut cLitSizeEstimate = HUF_estimateCompressedSize(
            ((*huf).CTable).as_ptr(),
            countWksp,
            maxSymbolValue,
        );
        if writeEntropy != 0 {
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add((*hufMetadata).hufDesSize);
        }
        if singleStream == 0 {
            cLitSizeEstimate = cLitSizeEstimate
                .wrapping_add(6);
        }
        return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
    }
    return 0;
}
unsafe fn ZSTD_estimateBlockSize_symbolType(
    mut type_0: SymbolEncodingType_e,
    mut codeTable: *const u8,
    mut nbSeq: usize,
    mut maxCode: u32,
    mut fseCTable: *const FSE_CTable,
    mut additionalBits: *const u8,
    mut defaultNorm: *const i16,
    mut defaultNormLog: u32,
    mut defaultMax: u32,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let countWksp = workspace as *mut u32;
    let mut ctp = codeTable;
    let ctStart = ctp;
    let ctEnd = ctStart.offset(nbSeq as isize);
    let mut cSymbolTypeSizeEstimateInBits: usize = 0;
    let mut max = maxCode;
    HIST_countFast_wksp(
        countWksp,
        &mut max,
        codeTable as *const std::ffi::c_void,
        nbSeq,
        workspace,
        wkspSize,
    );
    if type_0 as u32 == set_basic as i32 as u32 {
        cSymbolTypeSizeEstimateInBits = ZSTD_crossEntropyCost(
            defaultNorm,
            defaultNormLog,
            countWksp,
            max,
        );
    } else if type_0 as u32
        == set_rle as i32 as u32
    {
        cSymbolTypeSizeEstimateInBits = 0;
    } else if type_0 as u32
        == set_compressed as i32 as u32
        || type_0 as u32
            == set_repeat as i32 as u32
    {
        cSymbolTypeSizeEstimateInBits = ZSTD_fseBitCost(fseCTable, countWksp, max);
    }
    if ERR_isError(cSymbolTypeSizeEstimateInBits) {
        return nbSeq * 10;
    }
    while ctp < ctEnd {
        if !additionalBits.is_null() {
            cSymbolTypeSizeEstimateInBits = cSymbolTypeSizeEstimateInBits
                .wrapping_add(*additionalBits.offset(*ctp as isize) as usize);
        } else {
            cSymbolTypeSizeEstimateInBits = cSymbolTypeSizeEstimateInBits
                .wrapping_add(*ctp as usize);
        }
        ctp = ctp.offset(1);
        ctp;
    }
    return cSymbolTypeSizeEstimateInBits >> 3;
}
unsafe fn ZSTD_estimateBlockSize_sequences(
    mut ofCodeTable: *const u8,
    mut llCodeTable: *const u8,
    mut mlCodeTable: *const u8,
    mut nbSeq: usize,
    mut fseTables: *const ZSTD_fseCTables_t,
    mut fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut writeEntropy: i32,
) -> usize {
    let mut sequencesSectionHeaderSize = (1 as i32 + 1 as i32
        + (nbSeq >= 128) as i32
        + (nbSeq >= LONGNBSEQ as usize) as i32) as usize;
    let mut cSeqSizeEstimate: usize = 0;
    cSeqSizeEstimate = cSeqSizeEstimate
        .wrapping_add(
            ZSTD_estimateBlockSize_symbolType(
                (*fseMetadata).ofType,
                ofCodeTable,
                nbSeq,
                MaxOff as u32,
                ((*fseTables).offcodeCTable).as_ptr(),
                std::ptr::null(),
                OF_defaultNorm.as_ptr(),
                OF_defaultNormLog,
                DefaultMaxOff as u32,
                workspace,
                wkspSize,
            ),
        );
    cSeqSizeEstimate = cSeqSizeEstimate
        .wrapping_add(
            ZSTD_estimateBlockSize_symbolType(
                (*fseMetadata).llType,
                llCodeTable,
                nbSeq,
                MaxLL as u32,
                ((*fseTables).litlengthCTable).as_ptr(),
                LL_bits.as_ptr(),
                LL_defaultNorm.as_ptr(),
                LL_defaultNormLog,
                MaxLL as u32,
                workspace,
                wkspSize,
            ),
        );
    cSeqSizeEstimate = cSeqSizeEstimate
        .wrapping_add(
            ZSTD_estimateBlockSize_symbolType(
                (*fseMetadata).mlType,
                mlCodeTable,
                nbSeq,
                MaxML as u32,
                ((*fseTables).matchlengthCTable).as_ptr(),
                ML_bits.as_ptr(),
                ML_defaultNorm.as_ptr(),
                ML_defaultNormLog,
                MaxML as u32,
                workspace,
                wkspSize,
            ),
        );
    if writeEntropy != 0 {
        cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add((*fseMetadata).fseTablesSize);
    }
    return cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize);
}
unsafe fn ZSTD_estimateBlockSize(
    mut literals: *const u8,
    mut litSize: usize,
    mut ofCodeTable: *const u8,
    mut llCodeTable: *const u8,
    mut mlCodeTable: *const u8,
    mut nbSeq: usize,
    mut entropy: *const ZSTD_entropyCTables_t,
    mut entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut writeLitEntropy: i32,
    mut writeSeqEntropy: i32,
) -> usize {
    let literalsSize = ZSTD_estimateBlockSize_literal(
        literals,
        litSize,
        &(*entropy).huf,
        &(*entropyMetadata).hufMetadata,
        workspace,
        wkspSize,
        writeLitEntropy,
    );
    let seqSize = ZSTD_estimateBlockSize_sequences(
        ofCodeTable,
        llCodeTable,
        mlCodeTable,
        nbSeq,
        &(*entropy).fse,
        &(*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
        writeSeqEntropy,
    );
    return seqSize.wrapping_add(literalsSize).wrapping_add(ZSTD_blockHeaderSize);
}
unsafe fn ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
    mut seqStore: *mut SeqStore_t,
    mut zc: *mut ZSTD_CCtx,
) -> usize {
    let entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t = &mut (*zc)
        .blockSplitCtx
        .entropyMetadata;
    FORWARD_IF_ERROR!(
        ZSTD_buildBlockEntropyStats(seqStore, addr_of!((*(*zc).blockState.prevCBlock).entropy), addr_of!((*(*       zc).blockState.nextCBlock).entropy), addr_of!((*zc).appliedParams), entropyMetadata,
 (*       zc).tmpWorkspace, (*zc).tmpWkspSize), ""
    );
    return ZSTD_estimateBlockSize(
        (*seqStore).litStart,
        ((*seqStore).lit).offset_from((*seqStore).litStart) as std::ffi::c_long
            as usize,
        (*seqStore).ofCode,
        (*seqStore).llCode,
        (*seqStore).mlCode,
        ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
            as std::ffi::c_long as usize,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        entropyMetadata,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
        ((*entropyMetadata).hufMetadata.hType as u32
            == set_compressed as i32 as u32) as i32,
        1,
    );
}
unsafe fn ZSTD_countSeqStoreLiteralsBytes(
    seqStore: *const SeqStore_t,
) -> usize {
    let mut literalsBytes: usize = 0;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as usize;
    for i in 0..nbSeqs {
        let seq = *((*seqStore).sequencesStart).add(i);
        literalsBytes = literalsBytes.wrapping_add(seq.litLength as usize);
        if i == (*seqStore).longLengthPos as usize
            && (*seqStore).longLengthType
                == ZSTD_llt_literalLength
        {
            literalsBytes = literalsBytes
                .wrapping_add(0x10000);
        }
    }
    return literalsBytes;
}
unsafe fn ZSTD_countSeqStoreMatchBytes(
    seqStore: *const SeqStore_t,
) -> usize {
    let mut matchBytes: usize = 0;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as usize;
    for i in 0..nbSeqs {
        let mut seq = *((*seqStore).sequencesStart).add(i);
        matchBytes = matchBytes
            .wrapping_add((seq.mlBase as u32 + MINMATCH) as usize);
        if i == (*seqStore).longLengthPos as usize
            && (*seqStore).longLengthType
                == ZSTD_llt_matchLength
        {
            matchBytes = matchBytes.wrapping_add(0x10000 as i32 as usize);
        }
    }
    return matchBytes;
}
unsafe fn ZSTD_deriveSeqStoreChunk(
    mut resultSeqStore: *mut SeqStore_t,
    mut originalSeqStore: *const SeqStore_t,
    mut startIdx: usize,
    mut endIdx: usize,
) {
    *resultSeqStore = *originalSeqStore;
    if startIdx > 0 {
        (*resultSeqStore)
            .sequences = ((*originalSeqStore).sequencesStart).offset(startIdx as isize);
        (*resultSeqStore)
            .litStart = ((*resultSeqStore).litStart)
            .offset(ZSTD_countSeqStoreLiteralsBytes(resultSeqStore) as isize);
    }
    if (*originalSeqStore).longLengthType as u32
        != ZSTD_llt_none as i32 as u32
    {
        if ((*originalSeqStore).longLengthPos as usize) < startIdx
            || (*originalSeqStore).longLengthPos as usize > endIdx
        {
            (*resultSeqStore).longLengthType = ZSTD_llt_none;
        } else {
            (*resultSeqStore)
                .longLengthPos = ((*resultSeqStore).longLengthPos)
                .wrapping_sub(startIdx as u32);
        }
    }
    (*resultSeqStore)
        .sequencesStart = ((*originalSeqStore).sequencesStart).offset(startIdx as isize);
    (*resultSeqStore)
        .sequences = ((*originalSeqStore).sequencesStart).offset(endIdx as isize);
    if !(endIdx
        == ((*originalSeqStore).sequences)
            .offset_from((*originalSeqStore).sequencesStart) as std::ffi::c_long
            as usize)
    {
        let literalsBytes = ZSTD_countSeqStoreLiteralsBytes(resultSeqStore);
        (*resultSeqStore)
            .lit = ((*resultSeqStore).litStart).offset(literalsBytes as isize);
    }
    (*resultSeqStore).llCode = ((*resultSeqStore).llCode).offset(startIdx as isize);
    (*resultSeqStore).mlCode = ((*resultSeqStore).mlCode).offset(startIdx as isize);
    (*resultSeqStore).ofCode = ((*resultSeqStore).ofCode).offset(startIdx as isize);
}
unsafe fn ZSTD_resolveRepcodeToRawOffset(
    mut rep: *const u32,
    offBase: u32,
    ll0: u32,
) -> u32 {
    let adjustedRepCode = OFFBASE_TO_REPCODE(offBase)
        .wrapping_sub(1)
        .wrapping_add(ll0);
    if adjustedRepCode == ZSTD_REP_NUM as u32 {
        return (*rep.offset(0))
            .wrapping_sub(1);
    }
    return *rep.offset(adjustedRepCode as isize);
}
unsafe fn ZSTD_seqStore_resolveOffCodes(
    dRepcodes: *mut Repcodes_t,
    cRepcodes: *mut Repcodes_t,
    seqStore: *const SeqStore_t,
    nbSeq: u32,
) {
    let mut idx: u32 = 0;
    let longLitLenIdx = if (*seqStore).longLengthType as u32
        == ZSTD_llt_literalLength as i32 as u32
    {
        (*seqStore).longLengthPos
    } else {
        nbSeq
    };
    while idx < nbSeq {
        let seq = ((*seqStore).sequencesStart).offset(idx as isize);
        let ll0 = ((*seq).litLength as i32 == 0
            && idx != longLitLenIdx) as i32 as u32;
        let offBase = (*seq).offBase;
        if OFFBASE_IS_REPCODE(offBase) != 0 {
            let dRawOffset = ZSTD_resolveRepcodeToRawOffset(
                ((*dRepcodes).rep).as_mut_ptr() as *const u32,
                offBase,
                ll0,
            );
            let cRawOffset = ZSTD_resolveRepcodeToRawOffset(
                ((*cRepcodes).rep).as_mut_ptr() as *const u32,
                offBase,
                ll0,
            );
            if dRawOffset != cRawOffset {
                (*seq).offBase = OFFSET_TO_OFFBASE(cRawOffset);
            }
        }
        ZSTD_updateRep(((*dRepcodes).rep).as_mut_ptr(), (*seq).offBase, ll0);
        ZSTD_updateRep(((*cRepcodes).rep).as_mut_ptr(), offBase, ll0);
        idx = idx.wrapping_add(1);
        idx;
    }
}
unsafe fn ZSTD_compressSeqStore_singleBlock(
    mut zc: *mut ZSTD_CCtx,
    seqStore: *const SeqStore_t,
    dRep: *mut Repcodes_t,
    cRep: *mut Repcodes_t,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastBlock: u32,
    mut isPartition: u32,
) -> usize {
    let rleMaxLength = 25;
    let mut op = dst as *mut u8;
    let mut ip = src as *const u8;
    let mut cSize: usize = 0;
    let mut cSeqsSize: usize = 0;
    let dRepOriginal = *dRep;
    if isPartition != 0 {
        ZSTD_seqStore_resolveOffCodes(
            dRep,
            cRep,
            seqStore,
            ((*seqStore).sequences).offset_from((*seqStore).sequencesStart)
                as std::ffi::c_long as u32,
        );
    }
    RETURN_ERROR_IF!(dstCapacity < ZSTD_blockHeaderSize, ZSTD_error_dstSize_tooSmall);
    cSeqsSize = ZSTD_entropyCompressSeqStore(
        seqStore,
        &mut (*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &mut (*zc).appliedParams,
        op.offset(ZSTD_blockHeaderSize as isize) as *mut std::ffi::c_void,
        dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
        srcSize,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
        (*zc).bmi2,
    );
    FORWARD_IF_ERROR!(cSeqsSize, "ZSTD_entropyCompressSeqStore failed!");
    if (*zc).isFirstBlock == 0 && cSeqsSize < rleMaxLength as usize
        && ZSTD_isRLE(src as *const u8, srcSize) != 0
    {
        cSeqsSize = 1;
    }
    if (*zc).seqCollector.collectSequences != 0 {
        FORWARD_IF_ERROR!(
            ZSTD_copyBlockSequences(&mut (*zc).seqCollector, seqStore, dRepOriginal.rep.as_ptr()),
            "copyBlockSequences failed"
        );
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        return 0;
    }
    if cSeqsSize == 0 {
        cSize = ZSTD_noCompressBlock(
            op as *mut std::ffi::c_void,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            lastBlock,
        );
        FORWARD_IF_ERROR!(cSize, "Nocompress block failed");
        *dRep = dRepOriginal;
    } else if cSeqsSize == 1 {
        cSize = ZSTD_rleCompressBlock(
            op as *mut std::ffi::c_void,
            dstCapacity,
            *ip,
            srcSize,
            lastBlock,
        );
        FORWARD_IF_ERROR!(cSize, "RLE compress block failed");
        *dRep = dRepOriginal;
    } else {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        writeBlockHeader(op as *mut std::ffi::c_void, cSeqsSize, srcSize, lastBlock);
        cSize = ZSTD_blockHeaderSize.wrapping_add(cSeqsSize);
    }
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as u32
        == FSE_repeat_valid as i32 as u32
    {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode = FSE_repeat_check;
    }
    return cSize;
}
pub const MIN_SEQUENCES_BLOCK_SPLITTING: i32 = 300;
unsafe fn ZSTD_deriveBlockSplitsHelper(
    mut splits: *mut seqStoreSplits,
    mut startIdx: usize,
    mut endIdx: usize,
    mut zc: *mut ZSTD_CCtx,
    mut origSeqStore: *const SeqStore_t,
) {
    let fullSeqStoreChunk: *mut SeqStore_t = &mut (*zc).blockSplitCtx.fullSeqStoreChunk;
    let firstHalfSeqStore: *mut SeqStore_t = &mut (*zc).blockSplitCtx.firstHalfSeqStore;
    let secondHalfSeqStore: *mut SeqStore_t = &mut (*zc)
        .blockSplitCtx
        .secondHalfSeqStore;
    let mut estimatedOriginalSize: usize = 0;
    let mut estimatedFirstHalfSize: usize = 0;
    let mut estimatedSecondHalfSize: usize = 0;
    let mut midIdx = startIdx.wrapping_add(endIdx) / 2;
    if endIdx.wrapping_sub(startIdx) < MIN_SEQUENCES_BLOCK_SPLITTING as usize
        || (*splits).idx >= ZSTD_MAX_NB_BLOCK_SPLITS as usize
    {
        return;
    }
    ZSTD_deriveSeqStoreChunk(fullSeqStoreChunk, origSeqStore, startIdx, endIdx);
    ZSTD_deriveSeqStoreChunk(firstHalfSeqStore, origSeqStore, startIdx, midIdx);
    ZSTD_deriveSeqStoreChunk(secondHalfSeqStore, origSeqStore, midIdx, endIdx);
    estimatedOriginalSize = ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
        fullSeqStoreChunk,
        zc,
    );
    estimatedFirstHalfSize = ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
        firstHalfSeqStore,
        zc,
    );
    estimatedSecondHalfSize = ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
        secondHalfSeqStore,
        zc,
    );
    if ERR_isError(estimatedOriginalSize)
        || ERR_isError(estimatedFirstHalfSize) != 0
        || ERR_isError(estimatedSecondHalfSize) != 0
    {
        return;
    }
    if estimatedFirstHalfSize.wrapping_add(estimatedSecondHalfSize)
        < estimatedOriginalSize
    {
        ZSTD_deriveBlockSplitsHelper(splits, startIdx, midIdx, zc, origSeqStore);
        *((*splits).splitLocations).offset((*splits).idx as isize) = midIdx as u32;
        (*splits).idx = ((*splits).idx).wrapping_add(1);
        (*splits).idx;
        ZSTD_deriveBlockSplitsHelper(splits, midIdx, endIdx, zc, origSeqStore);
    }
}
unsafe fn ZSTD_deriveBlockSplits(
    mut zc: *mut ZSTD_CCtx,
    mut partitions: *mut u32,
    mut nbSeq: u32,
) -> usize {
    let mut splits = seqStoreSplits {
        splitLocations: std::ptr::null_mut(),
        idx: 0,
    };
    splits.splitLocations = partitions;
    splits.idx = 0;
    if nbSeq <= 4 {
        return 0;
    }
    ZSTD_deriveBlockSplitsHelper(
        &mut splits,
        0,
        nbSeq as usize,
        zc,
        &mut (*zc).seqStore,
    );
    *(splits.splitLocations).offset(splits.idx as isize) = nbSeq;
    return splits.idx;
}
unsafe fn ZSTD_compressBlock_splitBlock_internal(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut blockSize: usize,
    mut lastBlock: u32,
    mut nbSeq: u32,
) -> usize {
    let mut cSize: usize = 0;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let mut i: usize = 0;
    let mut srcBytesTotal: usize = 0;
    let partitions = ((*zc).blockSplitCtx.partitions).as_mut_ptr();
    let nextSeqStore: *mut SeqStore_t = &mut (*zc).blockSplitCtx.nextSeqStore;
    let currSeqStore: *mut SeqStore_t = &mut (*zc).blockSplitCtx.currSeqStore;
    let numSplits = ZSTD_deriveBlockSplits(zc, partitions, nbSeq);
    let mut dRep = repcodes_s { rep: [0; 3] };
    let mut cRep = repcodes_s { rep: [0; 3] };
    libc::memcpy(
        (dRep.rep).as_mut_ptr() as *mut std::ffi::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    libc::memcpy(
        (cRep.rep).as_mut_ptr() as *mut std::ffi::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    libc::memset(
        nextSeqStore as *mut std::ffi::c_void,
        0,
        size_of::<SeqStore_t>() as usize,
    );
    if numSplits == 0 {
        let mut cSizeSingleBlock = ZSTD_compressSeqStore_singleBlock(
            zc,
            &mut (*zc).seqStore,
            &mut dRep,
            &mut cRep,
            op as *mut std::ffi::c_void,
            dstCapacity,
            ip as *const std::ffi::c_void,
            blockSize,
            lastBlock,
            0,
        );
        FORWARD_IF_ERROR!(
            cSizeSingleBlock,
            "Compressing single block from splitBlock_internal() failed!"
        );
        return cSizeSingleBlock;
    }
    ZSTD_deriveSeqStoreChunk(
        currSeqStore,
        &mut (*zc).seqStore,
        0,
        *partitions.offset(0) as usize,
    );
    i = 0;
    while i <= numSplits {
        let mut cSizeChunk: usize = 0;
        let lastPartition = (i == numSplits) as i32 as u32;
        let mut lastBlockEntireSrc: u32 = 0;
        let mut srcBytes = (ZSTD_countSeqStoreLiteralsBytes(currSeqStore))
            .wrapping_add(ZSTD_countSeqStoreMatchBytes(currSeqStore));
        srcBytesTotal = srcBytesTotal.wrapping_add(srcBytes);
        if lastPartition != 0 {
            srcBytes = srcBytes.wrapping_add(blockSize.wrapping_sub(srcBytesTotal));
            lastBlockEntireSrc = lastBlock;
        } else {
            ZSTD_deriveSeqStoreChunk(
                nextSeqStore,
                &mut (*zc).seqStore,
                *partitions.offset(i as isize) as usize,
                *partitions
                    .offset(i.wrapping_add(1) as isize)
                    as usize,
            );
        }
        cSizeChunk = ZSTD_compressSeqStore_singleBlock(
            zc,
            currSeqStore,
            &mut dRep,
            &mut cRep,
            op as *mut std::ffi::c_void,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcBytes,
            lastBlockEntireSrc,
            1,
        );
        FORWARD_IF_ERROR!(cSizeChunk, "Compressing chunk failed!");
        ip = ip.offset(srcBytes as isize);
        op = op.offset(cSizeChunk as isize);
        dstCapacity = dstCapacity.wrapping_sub(cSizeChunk);
        cSize = cSize.wrapping_add(cSizeChunk);
        *currSeqStore = *nextSeqStore;
        i = i.wrapping_add(1);
        i;
    }
    libc::memcpy(
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *mut std::ffi::c_void,
        (dRep.rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    return cSize;
}
unsafe fn ZSTD_compressBlock_splitBlock(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastBlock: u32,
) -> usize {
    let mut nbSeq: u32 = 0;
    let mut cSize: usize = 0;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    FORWARD_IF_ERROR!(bss, "ZSTD_buildSeqStore failed");
    if bss == ZSTDbss_noCompress as i32 as usize {
        if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode
            as u32
            == FSE_repeat_valid as i32 as u32
        {
            (*(*zc).blockState.prevCBlock)
                .entropy
                .fse
                .offcode_repeatMode = FSE_repeat_check;
        }
        RETURN_ERROR_IF!((*zc).seqCollector.collectSequences != 0, ZSTD_error_sequenceProducer_failed);
        cSize = ZSTD_noCompressBlock(dst, dstCapacity, src, srcSize, lastBlock);
        FORWARD_IF_ERROR!(cSize, "ZSTD_noCompressBlock failed");
        return cSize;
    }
    nbSeq = ((*zc).seqStore.sequences).offset_from((*zc).seqStore.sequencesStart)
        as std::ffi::c_long as u32;
    cSize = ZSTD_compressBlock_splitBlock_internal(
        zc,
        dst,
        dstCapacity,
        src,
        srcSize,
        lastBlock,
        nbSeq,
    );
    FORWARD_IF_ERROR!(cSize, "Splitting blocks failed!");
    return cSize;
}
unsafe fn ZSTD_compressBlock_internal(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut frame: u32,
) -> usize {
    let rleMaxLength = 25;
    let mut cSize: usize = 0;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    FORWARD_IF_ERROR!(bss, "ZSTD_buildSeqStore failed");
    if bss == ZSTDbss_noCompress as i32 as usize {
        RETURN_ERROR_IF!((*zc).seqCollector.collectSequences != 0, ZSTD_error_sequenceProducer_failed);
        cSize = 0;
    } else {
        if (*zc).seqCollector.collectSequences != 0 {
            FORWARD_IF_ERROR!(
                ZSTD_copyBlockSequences(&mut (*zc).seqCollector, ZSTD_getSeqStore(zc), (*(*zc).blockState.prevCBlock).rep.as_ptr()), "copyBlockSequences failed"
            );
            ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
            return 0;
        }
        cSize = ZSTD_entropyCompressSeqStore(
            &mut (*zc).seqStore,
            &mut (*(*zc).blockState.prevCBlock).entropy,
            &mut (*(*zc).blockState.nextCBlock).entropy,
            &mut (*zc).appliedParams,
            dst,
            dstCapacity,
            srcSize,
            (*zc).tmpWorkspace,
            (*zc).tmpWkspSize,
            (*zc).bmi2,
        );
        if frame != 0 && (*zc).isFirstBlock == 0 && cSize < rleMaxLength as usize
            && ZSTD_isRLE(ip, srcSize) != 0
        {
            cSize = 1;
            *op
                .offset(
                    0,
                ) = *ip.offset(0);
        }
    }
    if ERR_isError(cSize) && cSize > 1 {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
    }
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as u32
        == FSE_repeat_valid as i32 as u32
    {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode = FSE_repeat_check;
    }
    return cSize;
}
unsafe fn ZSTD_compressBlock_targetCBlockSize_body(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    bss: usize,
    mut lastBlock: u32,
) -> usize {
    if bss == ZSTDbss_compress as i32 as usize {
        if (*zc).isFirstBlock == 0 && ZSTD_maybeRLE(&mut (*zc).seqStore) != 0
            && ZSTD_isRLE(src as *const u8, srcSize) != 0
        {
            return ZSTD_rleCompressBlock(
                dst,
                dstCapacity,
                *(src as *const u8),
                srcSize,
                lastBlock,
            );
        }
        let cSize = ZSTD_compressSuperBlock(
            zc,
            dst,
            dstCapacity,
            src,
            srcSize,
            lastBlock,
        );
        if cSize != ERROR(ZSTD_error_dstSize_tooSmall) {
            let maxCSize = srcSize
                .wrapping_sub(
                    ZSTD_minGain(srcSize, (*zc).appliedParams.cParams.strategy),
                );
            FORWARD_IF_ERROR!(cSize, "ZSTD_compressSuperBlock failed");
            if cSize != 0
                && cSize < maxCSize.wrapping_add(ZSTD_blockHeaderSize)
            {
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
                return cSize;
            }
        }
    }
    return ZSTD_noCompressBlock(dst, dstCapacity, src, srcSize, lastBlock);
}
unsafe fn ZSTD_compressBlock_targetCBlockSize(
    mut zc: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastBlock: u32,
) -> usize {
    let mut cSize: usize = 0;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    FORWARD_IF_ERROR!(bss, "ZSTD_buildSeqStore failed");
    cSize = ZSTD_compressBlock_targetCBlockSize_body(
        zc,
        dst,
        dstCapacity,
        src,
        srcSize,
        bss,
        lastBlock,
    );
    FORWARD_IF_ERROR!(
        cSize, "ZSTD_compressBlock_targetCBlockSize_body failed"
    );
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as u32
        == FSE_repeat_valid as i32 as u32
    {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode = FSE_repeat_check;
    }
    return cSize;
}
unsafe fn ZSTD_overflowCorrectIfNeeded(
    mut ms: *mut ZSTD_MatchState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut ip: *const std::ffi::c_void,
    mut iend: *const std::ffi::c_void,
) {
    let cycleLog = ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy);
    let maxDist = 1_u32 << (*params).cParams.windowLog;
    if ZSTD_window_needOverflowCorrection(
        (*ms).window,
        cycleLog,
        maxDist,
        (*ms).loadedDictEnd,
        ip,
        iend,
    ) != 0
    {
        let correction = ZSTD_window_correctOverflow(
            &mut (*ms).window,
            cycleLog,
            maxDist,
            ip,
        );
        ZSTD_cwksp_mark_tables_dirty(ws);
        ZSTD_reduceIndex(ms, params, correction);
        ZSTD_cwksp_mark_tables_clean(ws);
        if (*ms).nextToUpdate < correction {
            (*ms).nextToUpdate = 0;
        } else {
            (*ms).nextToUpdate = ((*ms).nextToUpdate).wrapping_sub(correction);
        }
        (*ms).loadedDictEnd = 0;
        (*ms).dictMatchState = std::ptr::null();
    }
}
unsafe fn ZSTD_optimalBlockSize(
    mut cctx: *mut ZSTD_CCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut blockSizeMax: usize,
    mut splitLevel: i32,
    mut strat: ZSTD_strategy,
    mut savings: i64,
) -> usize {
    static mut splitLevels: [i32; 10] = [
        0,
        0,
        1,
        2,
        2,
        3,
        3,
        4,
        4,
        4,
    ];
    if srcSize
        < (128 as i32 * ((1 as i32) << 10))
            as usize
        || blockSizeMax
            < (128 as i32
                * ((1 as i32) << 10)) as usize
    {
        return std::cmp::min(srcSize, blockSizeMax);
    }
    if savings < 3 {
        return (128 as i32
            * ((1 as i32) << 10)) as usize;
    }
    if splitLevel == 1 {
        return (128 as i32
            * ((1 as i32) << 10)) as usize;
    }
    if splitLevel == 0 {
        splitLevel = splitLevels[strat as usize];
    } else {
        splitLevel -= 2;
    }
    return ZSTD_splitBlock(
        src,
        blockSizeMax,
        splitLevel,
        (*cctx).tmpWorkspace,
        (*cctx).tmpWkspSize,
    );
}
unsafe fn ZSTD_compress_frameChunk(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut lastFrameChunk: u32,
) -> usize {
    let mut blockSizeMax = (*cctx).blockSizeMax;
    let mut remaining = srcSize;
    let mut ip = src as *const u8;
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let maxDist = 1_u32
        << (*cctx).appliedParams.cParams.windowLog;
    let mut savings = (*cctx).consumedSrcSize as i64 - (*cctx).producedCSize as i64;
    if (*cctx).appliedParams.fParams.checksumFlag != 0 && srcSize != 0 {
        ZSTD_XXH64_update(&mut (*cctx).xxhState, src, srcSize);
    }
    while remaining != 0 {
        let ms: *mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
        let blockSize = ZSTD_optimalBlockSize(
            cctx,
            ip as *const std::ffi::c_void,
            remaining,
            blockSizeMax,
            (*cctx).appliedParams.preBlockSplitter_level,
            (*cctx).appliedParams.cParams.strategy,
            savings,
        );
        let lastBlock = lastFrameChunk
            & (blockSize == remaining) as i32 as u32;
        RETURN_ERROR_IF!(dstCapacity
            < ZSTD_blockHeaderSize
                .wrapping_add((1 as i32 + 1 as i32) as usize)
                .wrapping_add(1), ZSTD_error_dstSize_tooSmall);
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            ip as *const std::ffi::c_void,
            ip.offset(blockSize as isize) as *const std::ffi::c_void,
        );
        ZSTD_checkDictValidity(
            &mut (*ms).window,
            ip.offset(blockSize as isize) as *const std::ffi::c_void,
            maxDist,
            &mut (*ms).loadedDictEnd,
            &mut (*ms).dictMatchState,
        );
        ZSTD_window_enforceMaxDist(
            &mut (*ms).window,
            ip as *const std::ffi::c_void,
            maxDist,
            &mut (*ms).loadedDictEnd,
            &mut (*ms).dictMatchState,
        );
        if (*ms).nextToUpdate < (*ms).window.lowLimit {
            (*ms).nextToUpdate = (*ms).window.lowLimit;
        }
        let mut cSize: usize = 0;
        if ZSTD_useTargetCBlockSize(&mut (*cctx).appliedParams) != 0 {
            cSize = ZSTD_compressBlock_targetCBlockSize(
                cctx,
                op as *mut std::ffi::c_void,
                dstCapacity,
                ip as *const std::ffi::c_void,
                blockSize,
                lastBlock,
            );
            FORWARD_IF_ERROR!(
                cSize, "ZSTD_compressBlock_targetCBlockSize failed"
            );
        } else if ZSTD_blockSplitterEnabled(&mut (*cctx).appliedParams) != 0 {
            cSize = ZSTD_compressBlock_splitBlock(
                cctx,
                op as *mut std::ffi::c_void,
                dstCapacity,
                ip as *const std::ffi::c_void,
                blockSize,
                lastBlock,
            );
            FORWARD_IF_ERROR!(
                cSize, "ZSTD_compressBlock_splitBlock failed"
            );
        } else {
            cSize = ZSTD_compressBlock_internal(
                cctx,
                op.offset(ZSTD_blockHeaderSize as isize) as *mut std::ffi::c_void,
                dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                ip as *const std::ffi::c_void,
                blockSize,
                1,
            );
            FORWARD_IF_ERROR!(
                cSize, "ZSTD_compressBlock_internal failed"
            );
            if cSize == 0 {
                cSize = ZSTD_noCompressBlock(
                    op as *mut std::ffi::c_void,
                    dstCapacity,
                    ip as *const std::ffi::c_void,
                    blockSize,
                    lastBlock,
                );
                FORWARD_IF_ERROR!(cSize, "ZSTD_noCompressBlock failed");
            } else {
                let cBlockHeader = if cSize == 1 {
                    lastBlock
                        .wrapping_add(
                            (bt_rle as i32 as u32) << 1,
                        )
                        .wrapping_add((blockSize << 3) as u32)
                } else {
                    lastBlock
                        .wrapping_add(
                            (bt_compressed as i32 as u32)
                                << 1,
                        )
                        .wrapping_add((cSize << 3) as u32)
                };
                MEM_writeLE24(op as *mut std::ffi::c_void, cBlockHeader);
                cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
            }
        }
        savings += blockSize as i64 - cSize as i64;
        ip = ip.offset(blockSize as isize);
        remaining = remaining.wrapping_sub(blockSize);
        op = op.offset(cSize as isize);
        dstCapacity = dstCapacity.wrapping_sub(cSize);
        (*cctx).isFirstBlock = 0;
    }
    if lastFrameChunk != 0 && op > ostart {
        (*cctx).stage = ZSTDcs_ending;
    }
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
unsafe fn ZSTD_writeFrameHeader(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
    mut dictID: u32,
) -> usize {
    let op = dst as *mut u8;
    let dictIDSizeCodeLength = ((dictID > 0) as i32
        + (dictID >= 256) as i32
        + (dictID >= 65536) as i32) as u32;
    let dictIDSizeCode = if (*params).fParams.noDictIDFlag != 0 {
        0_u32
    } else {
        dictIDSizeCodeLength
    };
    let checksumFlag = ((*params).fParams.checksumFlag > 0)
        as i32 as u32;
    let windowSize = 1_u32 << (*params).cParams.windowLog;
    let singleSegment = ((*params).fParams.contentSizeFlag != 0
        && windowSize as u64 >= pledgedSrcSize) as i32 as u32;
    let windowLogByte = (((*params).cParams.windowLog)
        .wrapping_sub(ZSTD_WINDOWLOG_ABSOLUTEMIN as u32)
        << 3) as u8;
    let fcsCode = (if (*params).fParams.contentSizeFlag != 0 {
        (pledgedSrcSize >= 256) as i32
            + (pledgedSrcSize
                >= (65536 as i32 + 256 as i32) as u64)
                as i32
            + (pledgedSrcSize >= 0xffffffff as u32 as u64)
                as i32
    } else {
        0 as i32
    }) as u32;
    let frameHeaderDescriptionByte = dictIDSizeCode
        .wrapping_add(checksumFlag << 2)
        .wrapping_add(singleSegment << 5)
        .wrapping_add(fcsCode << 6) as u8;
    let mut pos: usize = 0;
    RETURN_ERROR_IF!(dstCapacity < 18, ZSTD_error_dstSize_tooSmall);
    if (*params).format as u32
        == ZSTD_f_zstd1 as i32 as u32
    {
        MEM_writeLE32(dst, ZSTD_MAGICNUMBER);
        pos = 4;
    }
    let fresh7 = pos;
    pos = pos.wrapping_add(1);
    *op.offset(fresh7 as isize) = frameHeaderDescriptionByte;
    if singleSegment == 0 {
        let fresh8 = pos;
        pos = pos.wrapping_add(1);
        *op.offset(fresh8 as isize) = windowLogByte;
    }
    match dictIDSizeCode {
        1 => {
            *op.offset(pos as isize) = dictID as u8;
            pos = pos.wrapping_add(1);
            pos;
        }
        2 => {
            MEM_writeLE16(
                op.offset(pos as isize) as *mut std::ffi::c_void,
                dictID as u16,
            );
            pos = pos.wrapping_add(2);
        }
        3 => {
            MEM_writeLE32(op.offset(pos as isize) as *mut std::ffi::c_void, dictID);
            pos = pos.wrapping_add(4);
        }
        0 | _ => {}
    }
    match fcsCode {
        1 => {
            MEM_writeLE16(
                op.offset(pos as isize) as *mut std::ffi::c_void,
                pledgedSrcSize.wrapping_sub(256) as u16,
            );
            pos = pos.wrapping_add(2);
        }
        2 => {
            MEM_writeLE32(
                op.offset(pos as isize) as *mut std::ffi::c_void,
                pledgedSrcSize as u32,
            );
            pos = pos.wrapping_add(4);
        }
        3 => {
            MEM_writeLE64(
                op.offset(pos as isize) as *mut std::ffi::c_void,
                pledgedSrcSize,
            );
            pos = pos.wrapping_add(8);
        }
        0 | _ => {
            if singleSegment != 0 {
                let fresh9 = pos;
                pos = pos.wrapping_add(1);
                *op.offset(fresh9 as isize) = pledgedSrcSize as u8;
            }
        }
    }
    return pos;
}
#[no_mangle]
pub unsafe fn ZSTD_writeSkippableFrame(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut magicVariant: u32,
) -> usize {
    let mut op = dst as *mut u8;
    RETURN_ERROR_IF!(dstCapacity < srcSize.wrapping_add(8), ZSTD_error_dstSize_tooSmall);
    RETURN_ERROR_IF!(srcSize > 0xffffffff as u32 as usize, ZSTD_error_srcSize_wrong);
    RETURN_ERROR_IF!(magicVariant > 15, ZSTD_error_parameter_outOfBound);
    MEM_writeLE32(
        op as *mut std::ffi::c_void,
        (ZSTD_MAGIC_SKIPPABLE_START as u32).wrapping_add(magicVariant),
    );
    MEM_writeLE32(
        op.offset(4) as *mut std::ffi::c_void,
        srcSize as u32,
    );
    libc::memcpy(op + 8, src, (srcSize) as usize);
    return srcSize.wrapping_add(ZSTD_SKIPPABLEHEADERSIZE as usize);
}

/* ZSTD_writeLastEmptyBlock() :
 * output an empty Block with end-of-frame mark to complete a frame
 * @return : size of data written into `dst` (== ZSTD_blockHeaderSize (defined in zstd_internal.h))
 *           or an error code if `dstCapacity` is too small (<ZSTD_blockHeaderSize)
 */
pub unsafe fn ZSTD_writeLastEmptyBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
) -> usize {
    RETURN_ERROR_IF!(dstCapacity < ZSTD_blockHeaderSize, ZSTD_error_dstSize_tooSmall,
        "dst buf is too small to write frame trailer empty block.");
    let cBlockHeader24 = 1_u32 /*lastBlock*/ + ((bt_raw as u32) << 1); /* 0 size */
    MEM_writeLE24(dst, cBlockHeader24);
    return ZSTD_blockHeaderSize;
}

pub unsafe fn ZSTD_referenceExternalSequences(
    mut cctx: *mut ZSTD_CCtx,
    mut seq: *mut rawSeq,
    mut nbSeq: usize,
) {
    debug_assert!((*cctx).stage == ZSTDcs_init);
    debug_assert!(nbSeq == 0 || (*cctx).appliedParams.ldmParams.enableLdm != ZSTD_ps_enable);
    (*cctx).externSeqStore.seq = seq;
    (*cctx).externSeqStore.size = nbSeq;
    (*cctx).externSeqStore.capacity = nbSeq;
    (*cctx).externSeqStore.pos = 0;
    (*cctx).externSeqStore.posInSequence = 0;
}


unsafe fn ZSTD_compressContinue_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut frame: u32,
    mut lastFrameChunk: u32,
) -> usize {
    let ms: *mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
    let mut fhSize: usize = 0;
    RETURN_ERROR_IF!((*cctx).stage as u32
        == ZSTDcs_created as i32 as u32, ZSTD_error_stage_wrong);
    if frame != 0
        && (*cctx).stage as u32
            == ZSTDcs_init as i32 as u32
    {
        fhSize = ZSTD_writeFrameHeader(
            dst,
            dstCapacity,
            &mut (*cctx).appliedParams,
            ((*cctx).pledgedSrcSizePlusOne)
                .wrapping_sub(1) as u64,
            (*cctx).dictID,
        );
        FORWARD_IF_ERROR!(fhSize, "ZSTD_writeFrameHeader failed");
        dstCapacity = dstCapacity.wrapping_sub(fhSize);
        dst = (dst as *mut std::ffi::c_char).offset(fhSize as isize)
            as *mut std::ffi::c_void;
        (*cctx).stage = ZSTDcs_ongoing;
    }
    if srcSize == 0 {
        return fhSize;
    }
    if ZSTD_window_update(&mut (*ms).window, src, srcSize, (*ms).forceNonContiguous) == 0
    {
        (*ms).forceNonContiguous = 0;
        (*ms).nextToUpdate = (*ms).window.dictLimit;
    }
    if (*cctx).appliedParams.ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32
    {
        ZSTD_window_update(
            &mut (*cctx).ldmState.window,
            src,
            srcSize,
            0,
        );
    }
    if frame == 0 {
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            src,
            (src as *const u8).offset(srcSize as isize) as *const std::ffi::c_void,
        );
    }
    let cSize = if frame != 0 {
        ZSTD_compress_frameChunk(cctx, dst, dstCapacity, src, srcSize, lastFrameChunk)
    } else {
        ZSTD_compressBlock_internal(
            cctx,
            dst,
            dstCapacity,
            src,
            srcSize,
            0,
        )
    };
    FORWARD_IF_ERROR!(
        cSize, "%s", frame ? "ZSTD_compress_frameChunk failed" :
        "ZSTD_compressBlock_internal failed"
    );
    (*cctx)
        .consumedSrcSize = ((*cctx).consumedSrcSize)
        .wrapping_add(srcSize as u64);
    (*cctx)
        .producedCSize = ((*cctx).producedCSize)
        .wrapping_add(cSize.wrapping_add(fhSize) as u64);
    if (*cctx).pledgedSrcSizePlusOne != 0 {
        RETURN_ERROR_IF!(((*cctx).consumedSrcSize)
            .wrapping_add(1)
            > (*cctx).pledgedSrcSizePlusOne, ZSTD_error_srcSize_wrong);
    }
    return cSize.wrapping_add(fhSize);
}

pub unsafe fn ZSTD_compressContinue_public(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    DEBUGLOG!(5, "ZSTD_compressContinue (srcSize=%u)", srcSize);
    ZSTD_compressContinue_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        1 /* frame mode */,
        0 /* last chunk */,
    )
}

#[no_mangle]
pub unsafe fn ZSTD_compressContinue(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    return ZSTD_compressContinue_public(cctx, dst, dstCapacity, src, srcSize);
}
unsafe fn ZSTD_getBlockSize_deprecated(mut cctx: *const ZSTD_CCtx) -> usize {
    let cParams = (*cctx).appliedParams.cParams;
    return std::cmp::min((*cctx).appliedParams.maxBlockSize, 1_usize << cParams.windowLog);
}
#[no_mangle]
pub unsafe fn ZSTD_getBlockSize(mut cctx: *const ZSTD_CCtx) -> usize {
    return ZSTD_getBlockSize_deprecated(cctx);
}

/* NOTE: Must just wrap ZSTD_compressBlock_deprecated() */
pub unsafe fn ZSTD_compressBlock_deprecated(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    DEBUGLOG!(5, "ZSTD_compressBlock: srcSize = %u", srcSize);
    let blockSizeMax = ZSTD_getBlockSize_deprecated(cctx);
    RETURN_ERROR_IF!(srcSize > blockSizeMax, ZSTD_error_srcSize_wrong, "input is larger than a block");
    ZSTD_compressContinue_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        0 /* frame mode */,
        0 /* last chunk */,
    )
}

#[no_mangle]
pub unsafe fn ZSTD_compressBlock(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    return ZSTD_compressBlock_deprecated(cctx, dst, dstCapacity, src, srcSize);
}
unsafe fn ZSTD_loadDictionaryContent(
    mut ms: *mut ZSTD_MatchState_t,
    mut ls: *mut ldmState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
) -> usize {
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let loadLdmDict = ((*params).ldmParams.enableLdm as u32
        == ZSTD_ps_enable as i32 as u32 && !ls.is_null())
        as i32;
    ZSTD_assertEqualCParams((*params).cParams, (*ms).cParams);
    let mut maxDictSize = (if MEM_64bits {
        (3500 as u32)
            .wrapping_mul(
                ((1 as i32) << 20) as u32,
            )
    } else {
        (2000 as u32)
            .wrapping_mul(
                ((1 as i32) << 20) as u32,
            )
    })
        .wrapping_sub(ZSTD_WINDOW_START_INDEX as u32);
    let CDictTaggedIndices = ZSTD_CDictIndicesAreTagged(&(*params).cParams);
    if CDictTaggedIndices != 0
        && tfp as u32
            == ZSTD_tfp_forCDict as i32 as u32
    {
        let shortCacheMaxDictSize = ((1 as u32)
            << 32 - ZSTD_SHORT_CACHE_TAG_BITS)
            .wrapping_sub(ZSTD_WINDOW_START_INDEX as u32);
        maxDictSize = std::cmp::min(maxDictSize, shortCacheMaxDictSize);
    }
    if srcSize > maxDictSize as usize {
        ip = iend.offset(-(maxDictSize as isize));
        src = ip as *const std::ffi::c_void;
        srcSize = maxDictSize as usize;
    }
    if srcSize
        > (u32::MAX)
            .wrapping_sub(
                (if MEM_64bits {
                    (3500 as u32)
                        .wrapping_mul(
                            ((1 as i32) << 20)
                                as u32,
                        )
                } else {
                    (2000 as u32)
                        .wrapping_mul(
                            ((1 as i32) << 20)
                                as u32,
                        )
                }),
            ) as usize
    {
        loadLdmDict != 0;
    }
    ZSTD_window_update(&mut (*ms).window, src, srcSize, 0);
    if loadLdmDict != 0 {
        ZSTD_window_update(&mut (*ls).window, src, srcSize, 0);
        (*ls)
            .loadedDictEnd = if (*params).forceWindow != 0 {
            0_u32
        } else {
            iend.offset_from((*ls).window.base) as std::ffi::c_long as u32
        };
        ZSTD_ldm_fillHashTable(ls, ip, iend, &(*params).ldmParams);
    }
    let mut maxDictSize_0 = (1 as u32)
        << std::cmp::min(MAX((*params).cParams.hashLog + 3, (*params).cParams.chainLog + 1), 31);
    if srcSize > maxDictSize_0 as usize {
        ip = iend.offset(-(maxDictSize_0 as isize));
        src = ip as *const std::ffi::c_void;
        srcSize = maxDictSize_0 as usize;
    }
    (*ms).nextToUpdate = ip.offset_from((*ms).window.base) as std::ffi::c_long as u32;
    (*ms)
        .loadedDictEnd = if (*params).forceWindow != 0 {
        0_u32
    } else {
        iend.offset_from((*ms).window.base) as std::ffi::c_long as u32
    };
    (*ms).forceNonContiguous = (*params).deterministicRefPrefix;
    if srcSize <= HASH_READ_SIZE as usize {
        return 0;
    }
    ZSTD_overflowCorrectIfNeeded(
        ms,
        ws,
        params,
        ip as *const std::ffi::c_void,
        iend as *const std::ffi::c_void,
    );
    match (*params).cParams.strategy as u32 {
        1 => {
            ZSTD_fillHashTable(ms, iend as *const std::ffi::c_void, dtlm, tfp);
        }
        2 => {
            ZSTD_fillDoubleHashTable(ms, iend as *const std::ffi::c_void, dtlm, tfp);
        }
        3 | 4 | 5 => {
            if (*ms).dedicatedDictSearch != 0 {
                ZSTD_dedicatedDictSearch_lazy_loadDictionary(
                    ms,
                    iend.offset(-(HASH_READ_SIZE as isize)),
                );
            } else if (*params).useRowMatchFinder as u32
                == ZSTD_ps_enable as i32 as u32
            {
                let tagTableSize = 1_usize
                    << (*params).cParams.hashLog;
                libc::memset((*ms).tagTable, 0, (tagTableSize) as usize);
                ZSTD_row_update(ms, iend.offset(-(HASH_READ_SIZE as isize)));
            } else {
                ZSTD_insertAndFindFirstIndex(
                    ms,
                    iend.offset(-(HASH_READ_SIZE as isize)),
                );
            }
        }
        6 | 7 | 8 | 9 => {
            ZSTD_updateTree(ms, iend.offset(-(HASH_READ_SIZE as isize)), iend);
        }
        _ => {}
    }
    (*ms).nextToUpdate = iend.offset_from((*ms).window.base) as std::ffi::c_long as u32;
    return 0;
}
unsafe fn ZSTD_dictNCountRepeat(
    mut normalizedCounter: *mut i16,
    mut dictMaxSymbolValue: u32,
    mut maxSymbolValue: u32,
) -> FSE_repeat {
    let mut s: u32 = 0;
    if dictMaxSymbolValue < maxSymbolValue {
        return FSE_repeat_check;
    }
    s = 0;
    while s <= maxSymbolValue {
        if *normalizedCounter.offset(s as isize) as i32
            == 0
        {
            return FSE_repeat_check;
        }
        s = s.wrapping_add(1);
        s;
    }
    return FSE_repeat_valid;
}

pub unsafe fn ZSTD_loadCEntropy(
    mut bs: *mut ZSTD_compressedBlockState_t,
    mut workspace: *mut std::ffi::c_void,
    dict: *const std::ffi::c_void,
    mut dictSize: usize,
) -> usize {
    let mut offcodeNCount: [i16; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as u32;
    let mut dictPtr = dict as *const u8; /* skip magic num and dict ID */
    let dictEnd = dictPtr.add(dictSize);
    dictPtr = dictPtr.offset(8);
    (*bs).entropy.huf.repeatMode = HUF_repeat_check;

    let mut maxSymbolValue: u32 = 255;
    let mut hasZeroWeights: u32 = 1;
    let hufHeaderSize = HUF_readCTable(
        ((*bs).entropy.huf.CTable).as_mut_ptr(),
        &mut maxSymbolValue,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as usize,
        &mut hasZeroWeights,
    );

    /* We only set the loaded table as valid if it contains all non-zero
     * weights. Otherwise, we set it to check */
    if hasZeroWeights == 0
        && maxSymbolValue == 255
    {
        (*bs).entropy.huf.repeatMode = HUF_repeat_valid;
    }

    RETURN_ERROR_IF!(ERR_isError(hufHeaderSize), ZSTD_error_dictionary_corrupted);
    dictPtr = dictPtr.add(hufHeaderSize);


    let mut offcodeLog: u32 = 0;
    let offcodeHeaderSize = FSE_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as usize,
    );
    RETURN_ERROR_IF!(ERR_isError(offcodeHeaderSize), ZSTD_error_dictionary_corrupted);
    RETURN_ERROR_IF!(offcodeLog > 8, ZSTD_error_dictionary_corrupted);
    /* fill all offset symbols to avoid garbage at end of table */
    RETURN_ERROR_IF!(ERR_isError(
        FSE_buildCTable_wksp(
            ((*bs).entropy.fse.offcodeCTable).as_mut_ptr(),
            offcodeNCount.as_mut_ptr(),
            31,
            offcodeLog,
            workspace,
            HUF_WORKSPACE_SIZE
        ),
    ), ZSTD_error_dictionary_corrupted);
    /* Defer checking offcodeMaxValue because we need to know the size of the dictionary content */
    dictPtr = dictPtr.add(offcodeHeaderSize);


    let mut matchlengthNCount: [i16; MaxML as usize + 1] = [0; MaxML as usize + 1];
    let mut matchlengthMaxValue = MaxML as u32;
    let mut matchlengthLog: u32 = 0;
    let matchlengthHeaderSize = FSE_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as usize,
    );
    RETURN_ERROR_IF!(ERR_isError(matchlengthHeaderSize), ZSTD_error_dictionary_corrupted);
    RETURN_ERROR_IF!(matchlengthLog > 9, ZSTD_error_dictionary_corrupted);
    RETURN_ERROR_IF!(ERR_isError(
        FSE_buildCTable_wksp(
            ((*bs).entropy.fse.matchlengthCTable).as_mut_ptr(),
            matchlengthNCount.as_mut_ptr(),
            matchlengthMaxValue,
            matchlengthLog,
            workspace,
            HUF_WORKSPACE_SIZE,
        ),
    ), ZSTD_error_dictionary_corrupted);
    (*bs)
        .entropy
        .fse
        .matchlength_repeatMode = ZSTD_dictNCountRepeat(
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        MaxML as u32,
    );
    dictPtr = dictPtr.add(matchlengthHeaderSize);


    let mut litlengthNCount: [i16; MaxLL as usize + 1] = [0; MaxLL as usize + 1];
    let mut litlengthMaxValue = MaxLL as u32;
    let mut litlengthLog: u32 = 0;
    let litlengthHeaderSize = FSE_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as usize,
    );
    RETURN_ERROR_IF!(ERR_isError(litlengthHeaderSize), ZSTD_error_dictionary_corrupted);
    RETURN_ERROR_IF!(litlengthLog > 9, ZSTD_error_dictionary_corrupted);
    RETURN_ERROR_IF!(ERR_isError(
        FSE_buildCTable_wksp(
            ((*bs).entropy.fse.litlengthCTable).as_mut_ptr(),
            litlengthNCount.as_mut_ptr(),
            litlengthMaxValue,
            litlengthLog,
            workspace,
            HUF_WORKSPACE_SIZE,
        ),
    ), ZSTD_error_dictionary_corrupted);
    (*bs)
        .entropy
        .fse
        .litlength_repeatMode = ZSTD_dictNCountRepeat(
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        MaxLL as u32,
    );
    dictPtr = dictPtr.offset(litlengthHeaderSize as isize);


    RETURN_ERROR_IF!(dictPtr.offset(12) > dictEnd, ZSTD_error_dictionary_corrupted);
    (*bs)
        .rep[0] = MEM_readLE32(
        dictPtr.offset(0) as *const std::ffi::c_void,
    );
    (*bs)
        .rep[1] = MEM_readLE32(
        dictPtr.offset(4) as *const std::ffi::c_void,
    );
    (*bs)
        .rep[2] = MEM_readLE32(
        dictPtr.offset(8) as *const std::ffi::c_void,
    );
    dictPtr = dictPtr.offset(12);

    let dictContentSize = dictEnd.offset_from(dictPtr) as usize;
    let mut offcodeMax = MaxOff;
    if dictContentSize
        <= (u32::MAX - KB(128) as u32)
    {
        let maxOffset = (dictContentSize + KB(128)) as u32; /* The maximum offset that must be supported */
        offcodeMax = ZSTD_highbit32(maxOffset); /* Calculate minimum offset code required to represent maxOffset */
    }
    /* All offset values <= dictContentSize + 128 KB must be representable for a valid table */
    (*bs)
        .entropy
        .fse
        .offcode_repeatMode = ZSTD_dictNCountRepeat(
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        std::cmp::min(offcodeMax, MaxOff),
    );
    
    /* All repCodes must be <= dictContentSize and != 0 */
    for u in 0..3 {
        RETURN_ERROR_IF!((*bs).rep[u] == 0, ZSTD_error_dictionary_corrupted);
        RETURN_ERROR_IF!((*bs).rep[u] as usize > dictContentSize, ZSTD_error_dictionary_corrupted);
    }
    return dictPtr.offset_from(dict as *const u8) as usize;
}

unsafe fn ZSTD_loadZstdDictionary(
    mut bs: *mut ZSTD_compressedBlockState_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
    mut workspace: *mut std::ffi::c_void,
) -> usize {
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.offset(dictSize as isize);
    let mut dictID: usize = 0;
    let mut eSize: usize = 0;
    dictID = (if (*params).fParams.noDictIDFlag != 0 {
        0_u32
    } else {
        MEM_readLE32(
            dictPtr.offset(4) as *const std::ffi::c_void,
        )
    }) as usize;
    eSize = ZSTD_loadCEntropy(bs, workspace, dict, dictSize);
    FORWARD_IF_ERROR!(eSize, "ZSTD_loadCEntropy failed");
    dictPtr = dictPtr.offset(eSize as isize);
    let dictContentSize = dictEnd.offset_from(dictPtr) as std::ffi::c_long as usize;
    FORWARD_IF_ERROR!(
        ZSTD_loadDictionaryContent(ms, NULL, ws, params, dictPtr, dictContentSize, dtlm,
        tfp), ""
    );
    return dictID;
}
unsafe fn ZSTD_compress_insertDictionary(
    mut bs: *mut ZSTD_compressedBlockState_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut ls: *mut ldmState_t,
    mut ws: *mut ZSTD_cwksp,
    mut params: *const ZSTD_CCtx_params,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictContentType: ZSTD_dictContentType_e,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
    mut workspace: *mut std::ffi::c_void,
) -> usize {
    if dict.is_null() || dictSize < 8 {
        RETURN_ERROR_IF!(dictContentType as u32
            == ZSTD_dct_fullDict as i32 as u32, ZSTD_error_dictionary_wrong);
        return 0;
    }
    ZSTD_reset_compressedBlockState(bs);
    if dictContentType as u32
        == ZSTD_dct_rawContent as i32 as u32
    {
        return ZSTD_loadDictionaryContent(ms, ls, ws, params, dict, dictSize, dtlm, tfp);
    }
    if MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY {
        if dictContentType as u32
            == ZSTD_dct_auto as i32 as u32
        {
            return ZSTD_loadDictionaryContent(
                ms,
                ls,
                ws,
                params,
                dict,
                dictSize,
                dtlm,
                tfp,
            );
        }
        RETURN_ERROR_IF!(dictContentType as u32
            == ZSTD_dct_fullDict as i32 as u32, ZSTD_error_dictionary_wrong);
    }
    return ZSTD_loadZstdDictionary(
        bs,
        ms,
        ws,
        params,
        dict,
        dictSize,
        dtlm,
        tfp,
        workspace,
    );
}
pub const ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF: i32 = 128 as i32
    * ((1 as i32) << 10);
pub const ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER: u64 = 6
    as u64;
unsafe fn ZSTD_compressBegin_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictContentType: ZSTD_dictContentType_e,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
    mut zbuff: ZSTD_buffered_policy_e,
) -> usize {
    let dictContentSize = if !cdict.is_null() {
        (*cdict).dictContentSize
    } else {
        dictSize
    };
    (*cctx)
        .traceCtx = if (Some(
        ZSTD_trace_compress_begin
            as unsafe extern "C" fn(*const ZSTD_CCtx_s) -> ZSTD_TraceCtx,
    ))
        .is_some()
    {
        ZSTD_trace_compress_begin(cctx)
    } else {
        0 as ZSTD_TraceCtx
    };
    if !cdict.is_null() && (*cdict).dictContentSize > 0
        && (pledgedSrcSize < ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF as u64
            || (pledgedSrcSize as u64)
                < ((*cdict).dictContentSize as u64)
                    .wrapping_mul(ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER)
            || pledgedSrcSize as u64 == ZSTD_CONTENTSIZE_UNKNOWN
            || (*cdict).compressionLevel == 0)
        && (*params).attachDictPref as u32
            != ZSTD_dictForceLoad as i32 as u32
    {
        return ZSTD_resetCCtx_usingCDict(cctx, cdict, params, pledgedSrcSize, zbuff);
    }
    FORWARD_IF_ERROR!(
        ZSTD_resetCCtx_internal(cctx, params, pledgedSrcSize, dictContentSize,
        ZSTDcrp_makeClean, zbuff), ""
    );
    let dictID = if !cdict.is_null() {
        ZSTD_compress_insertDictionary(
            (*cctx).blockState.prevCBlock,
            &mut (*cctx).blockState.matchState,
            &mut (*cctx).ldmState,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            (*cdict).dictContent,
            (*cdict).dictContentSize,
            (*cdict).dictContentType,
            dtlm,
            ZSTD_tfp_forCCtx,
            (*cctx).tmpWorkspace,
        )
    } else {
        ZSTD_compress_insertDictionary(
            (*cctx).blockState.prevCBlock,
            &mut (*cctx).blockState.matchState,
            &mut (*cctx).ldmState,
            &mut (*cctx).workspace,
            &mut (*cctx).appliedParams,
            dict,
            dictSize,
            dictContentType,
            dtlm,
            ZSTD_tfp_forCCtx,
            (*cctx).tmpWorkspace,
        )
    };
    FORWARD_IF_ERROR!(dictID, "ZSTD_compress_insertDictionary failed");
    (*cctx).dictID = dictID as u32;
    (*cctx).dictContentSize = dictContentSize;
    return 0;
}

pub unsafe fn ZSTD_compressBegin_advanced_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictContentType: ZSTD_dictContentType_e,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
) -> usize {
    DEBUGLOG!(4, "ZSTD_compressBegin_advanced_internal: wlog=%u", (*params).cParams.windowLog);
    /* compression parameters verification and optimization */
    FORWARD_IF_ERROR!(ZSTD_checkCParams((*params).cParams), "");
    return ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        dictContentType,
        dtlm,
        cdict,
        params,
        pledgedSrcSize as u64,
        ZSTDb_not_buffered,
    );
}

#[no_mangle]
pub unsafe fn ZSTD_compressBegin_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut params: ZSTD_parameters,
    mut pledgedSrcSize: u64,
) -> usize {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: ZSTD_f_zstd1,
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: std::ptr::null_mut(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    ZSTD_CCtxParams_init_internal(&mut cctxParams, &mut params, ZSTD_NO_CLEVEL);
    return ZSTD_compressBegin_advanced_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        std::ptr::null(),
        &mut cctxParams,
        pledgedSrcSize,
    );
}
unsafe fn ZSTD_compressBegin_usingDict_deprecated(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> usize {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: ZSTD_f_zstd1,
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: std::ptr::null_mut(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    let params = ZSTD_getParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_noAttachDict,
    );
    ZSTD_CCtxParams_init_internal(
        &mut cctxParams,
        &params,
        if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        },
    );
    return ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        std::ptr::null(),
        &mut cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        ZSTDb_not_buffered,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_compressBegin_usingDict(
    mut cctx: *mut ZSTD_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> usize {
    return ZSTD_compressBegin_usingDict_deprecated(
        cctx,
        dict,
        dictSize,
        compressionLevel,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_compressBegin(
    mut cctx: *mut ZSTD_CCtx,
    mut compressionLevel: i32,
) -> usize {
    return ZSTD_compressBegin_usingDict_deprecated(
        cctx,
        std::ptr::null(),
        0,
        compressionLevel,
    );
}
unsafe fn ZSTD_writeEpilogue(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
) -> usize {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    RETURN_ERROR_IF!((*cctx).stage as u32
        == ZSTDcs_created as i32 as u32, ZSTD_error_stage_wrong);
    if (*cctx).stage as u32
        == ZSTDcs_init as i32 as u32
    {
        let mut fhSize = ZSTD_writeFrameHeader(
            dst,
            dstCapacity,
            &mut (*cctx).appliedParams,
            0,
            0,
        );
        FORWARD_IF_ERROR!(fhSize, "ZSTD_writeFrameHeader failed");
        dstCapacity = dstCapacity.wrapping_sub(fhSize);
        op = op.offset(fhSize as isize);
        (*cctx).stage = ZSTDcs_ongoing;
    }
    if (*cctx).stage as u32
        != ZSTDcs_ending as i32 as u32
    {
        let cBlockHeader24 = 1_u32
            .wrapping_add((bt_raw as i32 as u32) << 1)
            .wrapping_add(0);
        RETURN_ERROR_IF!(dstCapacity < 3, ZSTD_error_dstSize_tooSmall);
        MEM_writeLE24(op as *mut std::ffi::c_void, cBlockHeader24);
        op = op.offset(ZSTD_blockHeaderSize as isize);
        dstCapacity = dstCapacity.wrapping_sub(ZSTD_blockHeaderSize);
    }
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        let checksum = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as u32;
        RETURN_ERROR_IF!(dstCapacity < 4, ZSTD_error_dstSize_tooSmall);
        MEM_writeLE32(op as *mut std::ffi::c_void, checksum);
        op = op.offset(4);
    }
    (*cctx).stage = ZSTDcs_created;
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}

pub unsafe fn ZSTD_CCtx_trace(
    mut cctx: *mut ZSTD_CCtx,
    mut extraCSize: usize,
) {
    // TODO #if ZSTD_TRACE
    // if (*cctx).traceCtx != 0
    //     && (Some(
    //         ZSTD_trace_compress_end
    //             as unsafe extern "C" fn(ZSTD_TraceCtx, *const ZSTD_Trace) -> (),
    //     ))
    //         .is_some()
    // {
    //     let streaming = ((*cctx).inBuffSize > 0
    //         || (*cctx).outBuffSize > 0
    //         || (*cctx).appliedParams.nbWorkers > 0)
    //         as i32;
    //     let mut trace = ZSTD_Trace {
    //         version: 0,
    //         streaming: 0,
    //         dictionaryID: 0,
    //         dictionaryIsCold: 0,
    //         dictionarySize: 0,
    //         uncompressedSize: 0,
    //         compressedSize: 0,
    //         params: std::ptr::null(),
    //         cctx: std::ptr::null(),
    //         dctx: std::ptr::null(),
    //     };
    //     libc::memset(
    //         &mut trace as *mut ZSTD_Trace as *mut std::ffi::c_void,
    //         0,
    //         size_of::<ZSTD_Trace>() as usize,
    //     );
    //     trace.version = ZSTD_VERSION_NUMBER as u32;
    //     trace.streaming = streaming;
    //     trace.dictionaryID = (*cctx).dictID;
    //     trace.dictionarySize = (*cctx).dictContentSize;
    //     trace.uncompressedSize = (*cctx).consumedSrcSize as usize;
    //     trace
    //         .compressedSize = ((*cctx).producedCSize)
    //         .wrapping_add(extraCSize as u64) as usize;
    //     trace.params = &mut (*cctx).appliedParams;
    //     trace.cctx = cctx;
    //     ZSTD_trace_compress_end((*cctx).traceCtx, &mut trace);
    // }
    // (*cctx).traceCtx = 0;
}

pub unsafe fn ZSTD_compressEnd_public(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let cSize = ZSTD_compressContinue_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        1 /* frame mode */,
        1 /* last chunk */,
    );
    FORWARD_IF_ERROR!(cSize, "ZSTD_compressContinue_internal failed");
    let endResult = ZSTD_writeEpilogue(
        cctx,
        dst.byte_add(cSize),
        dstCapacity - cSize,
    );
    FORWARD_IF_ERROR!(endResult, "ZSTD_writeEpilogue failed");
    debug_assert!(!((*cctx).appliedParams.fParams.contentSizeFlag != 0 && (*cctx).pledgedSrcSizePlusOne == 0));
    if (*cctx).pledgedSrcSizePlusOne != 0 { /* control src size */
        const _: () = assert!(ZSTD_CONTENTSIZE_UNKNOWN == u64::MAX);
        DEBUGLOG!(4, "end of frame : controlling src size");
        RETURN_ERROR_IF!((*cctx).pledgedSrcSizePlusOne
            != ((*cctx).consumedSrcSize)
                .wrapping_add(1), ZSTD_error_srcSize_wrong,
             "error : pledgedSrcSize = %u, while realSrcSize = %u",
            (*cctx).pledgedSrcSizePlusOne-1,
            (*cctx).consumedSrcSize);
    }
    ZSTD_CCtx_trace(cctx, endResult);
    return cSize.wrapping_add(endResult);
}

#[no_mangle]
pub unsafe fn ZSTD_compressEnd(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    return ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe fn ZSTD_compress_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut params: ZSTD_parameters,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "");
    ZSTD_CCtxParams_init_internal(
        &mut (*cctx).simpleApiParams,
        &mut params,
        ZSTD_NO_CLEVEL,
    );
    return ZSTD_compress_advanced_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        &mut (*cctx).simpleApiParams,
    );
}

/* Internal */
pub unsafe fn ZSTD_compress_advanced_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut params: *const ZSTD_CCtx_params,
) -> usize {
    DEBUGLOG!(4, "ZSTD_compress_advanced_internal (srcSize:%u)", srcSize);
    FORWARD_IF_ERROR!(
        ZSTD_compressBegin_internal(cctx, dict, dictSize, ZSTD_dct_auto, ZSTD_dtlm_fast,
        std::ptr::null(), params, srcSize, ZSTDb_not_buffered), ""
    );
    return ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize);
}

#[no_mangle]
pub unsafe fn ZSTD_compress_usingDict(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> usize {
    let params = ZSTD_getParams_internal(
        compressionLevel,
        srcSize as u64,
        if !dict.is_null() { dictSize } else { 0_usize },
        ZSTD_cpm_noAttachDict,
    );
    ZSTD_CCtxParams_init_internal(
        &mut (*cctx).simpleApiParams,
        &params,
        if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        },
    );
    return ZSTD_compress_advanced_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        &mut (*cctx).simpleApiParams,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_compressCCtx(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut compressionLevel: i32,
) -> usize {
    return ZSTD_compress_usingDict(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        std::ptr::null(),
        0,
        compressionLevel,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_compress(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut compressionLevel: i32,
) -> usize {
    let mut result: usize = 0;
    let mut ctxBody = ZSTD_CCtx_s {
        stage: ZSTDcs_created,
        cParamsChanged: 0,
        bmi2: 0,
        requestedParams: ZSTD_CCtx_params_s {
            format: ZSTD_f_zstd1,
            cParams: ZSTD_compressionParameters {
                windowLog: 0,
                chainLog: 0,
                hashLog: 0,
                searchLog: 0,
                minMatch: 0,
                targetLength: 0,
                strategy: 0,
            },
            fParams: ZSTD_frameParameters {
                contentSizeFlag: 0,
                checksumFlag: 0,
                noDictIDFlag: 0,
            },
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictDefaultAttach,
            literalCompressionMode: ZSTD_ps_auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            postBlockSplitter: ZSTD_ps_auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem {
                customAlloc: None,
                customFree: None,
                opaque: std::ptr::null_mut(),
            },
            prefetchCDictTables: ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: std::ptr::null_mut(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ZSTD_ps_auto,
        },
        appliedParams: ZSTD_CCtx_params_s {
            format: ZSTD_f_zstd1,
            cParams: ZSTD_compressionParameters {
                windowLog: 0,
                chainLog: 0,
                hashLog: 0,
                searchLog: 0,
                minMatch: 0,
                targetLength: 0,
                strategy: 0,
            },
            fParams: ZSTD_frameParameters {
                contentSizeFlag: 0,
                checksumFlag: 0,
                noDictIDFlag: 0,
            },
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictDefaultAttach,
            literalCompressionMode: ZSTD_ps_auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            postBlockSplitter: ZSTD_ps_auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem {
                customAlloc: None,
                customFree: None,
                opaque: std::ptr::null_mut(),
            },
            prefetchCDictTables: ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: std::ptr::null_mut(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ZSTD_ps_auto,
        },
        simpleApiParams: ZSTD_CCtx_params_s {
            format: ZSTD_f_zstd1,
            cParams: ZSTD_compressionParameters {
                windowLog: 0,
                chainLog: 0,
                hashLog: 0,
                searchLog: 0,
                minMatch: 0,
                targetLength: 0,
                strategy: 0,
            },
            fParams: ZSTD_frameParameters {
                contentSizeFlag: 0,
                checksumFlag: 0,
                noDictIDFlag: 0,
            },
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictDefaultAttach,
            literalCompressionMode: ZSTD_ps_auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            postBlockSplitter: ZSTD_ps_auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem {
                customAlloc: None,
                customFree: None,
                opaque: std::ptr::null_mut(),
            },
            prefetchCDictTables: ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: std::ptr::null_mut(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ZSTD_ps_auto,
        },
        dictID: 0,
        dictContentSize: 0,
        workspace: ZSTD_cwksp {
            workspace: std::ptr::null_mut(),
            workspaceEnd: std::ptr::null_mut(),
            objectEnd: std::ptr::null_mut(),
            tableEnd: std::ptr::null_mut(),
            tableValidEnd: std::ptr::null_mut(),
            allocStart: std::ptr::null_mut(),
            initOnceStart: std::ptr::null_mut(),
            allocFailed: 0,
            workspaceOversizedDuration: 0,
            phase: ZSTD_cwksp_alloc_objects,
            isStatic: ZSTD_cwksp_dynamic_alloc,
        },
        blockSizeMax: 0,
        pledgedSrcSizePlusOne: 0,
        consumedSrcSize: 0,
        producedCSize: 0,
        xxhState: XXH64_state_s {
            total_len: 0,
            v: [0; 4],
            mem64: [0; 4],
            memsize: 0,
            reserved32: 0,
            reserved64: 0,
        },
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        pool: std::ptr::null_mut(),
        staticSize: 0,
        seqCollector: SeqCollector {
            collectSequences: 0,
            seqStart: std::ptr::null_mut(),
            seqIndex: 0,
            maxSequences: 0,
        },
        isFirstBlock: 0,
        initialized: 0,
        seqStore: SeqStore_t {
            sequencesStart: std::ptr::null_mut(),
            sequences: std::ptr::null_mut(),
            litStart: std::ptr::null_mut(),
            lit: std::ptr::null_mut(),
            llCode: std::ptr::null_mut(),
            mlCode: std::ptr::null_mut(),
            ofCode: std::ptr::null_mut(),
            maxNbSeq: 0,
            maxNbLit: 0,
            longLengthType: ZSTD_llt_none,
            longLengthPos: 0,
        },
        ldmState: ldmState_t {
            window: ZSTD_window_t {
                nextSrc: std::ptr::null(),
                base: std::ptr::null(),
                dictBase: std::ptr::null(),
                dictLimit: 0,
                lowLimit: 0,
                nbOverflowCorrections: 0,
            },
            hashTable: std::ptr::null_mut(),
            loadedDictEnd: 0,
            bucketOffsets: std::ptr::null_mut(),
            splitIndices: [0; 64],
            matchCandidates: [ldmMatchCandidate_t {
                split: std::ptr::null(),
                hash: 0,
                checksum: 0,
                bucket: std::ptr::null_mut(),
            }; 64],
        },
        ldmSequences: std::ptr::null_mut(),
        maxNbLdmSequences: 0,
        externSeqStore: RawSeqStore_t {
            seq: std::ptr::null_mut(),
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        },
        blockState: ZSTD_blockState_t {
            prevCBlock: std::ptr::null_mut(),
            nextCBlock: std::ptr::null_mut(),
            matchState: ZSTD_MatchState_t {
                window: ZSTD_window_t {
                    nextSrc: std::ptr::null(),
                    base: std::ptr::null(),
                    dictBase: std::ptr::null(),
                    dictLimit: 0,
                    lowLimit: 0,
                    nbOverflowCorrections: 0,
                },
                loadedDictEnd: 0,
                nextToUpdate: 0,
                hashLog3: 0,
                rowHashLog: 0,
                tagTable: std::ptr::null_mut(),
                hashCache: [0; 8],
                hashSalt: 0,
                hashSaltEntropy: 0,
                hashTable: std::ptr::null_mut(),
                hashTable3: std::ptr::null_mut(),
                chainTable: std::ptr::null_mut(),
                forceNonContiguous: 0,
                dedicatedDictSearch: 0,
                opt: optState_t {
                    litFreq: std::ptr::null_mut(),
                    litLengthFreq: std::ptr::null_mut(),
                    matchLengthFreq: std::ptr::null_mut(),
                    offCodeFreq: std::ptr::null_mut(),
                    matchTable: std::ptr::null_mut(),
                    priceTable: std::ptr::null_mut(),
                    litSum: 0,
                    litLengthSum: 0,
                    matchLengthSum: 0,
                    offCodeSum: 0,
                    litSumBasePrice: 0,
                    litLengthSumBasePrice: 0,
                    matchLengthSumBasePrice: 0,
                    offCodeSumBasePrice: 0,
                    priceType: zop_dynamic,
                    symbolCosts: std::ptr::null(),
                    literalCompressionMode: ZSTD_ps_auto,
                },
                dictMatchState: std::ptr::null(),
                cParams: ZSTD_compressionParameters {
                    windowLog: 0,
                    chainLog: 0,
                    hashLog: 0,
                    searchLog: 0,
                    minMatch: 0,
                    targetLength: 0,
                    strategy: 0,
                },
                ldmSeqStore: std::ptr::null(),
                prefetchCDictTables: 0,
                lazySkipping: 0,
            },
        },
        tmpWorkspace: std::ptr::null_mut(),
        tmpWkspSize: 0,
        bufferedPolicy: ZSTDb_not_buffered,
        inBuff: std::ptr::null_mut(),
        inBuffSize: 0,
        inToCompress: 0,
        inBuffPos: 0,
        inBuffTarget: 0,
        outBuff: std::ptr::null_mut(),
        outBuffSize: 0,
        outBuffContentSize: 0,
        outBuffFlushedSize: 0,
        streamStage: zcss_init,
        frameEnded: 0,
        expectedInBuffer: ZSTD_inBuffer_s {
            src: std::ptr::null(),
            size: 0,
            pos: 0,
        },
        stableIn_notConsumed: 0,
        expectedOutBufferSize: 0,
        localDict: ZSTD_localDict {
            dictBuffer: std::ptr::null_mut(),
            dict: std::ptr::null(),
            dictSize: 0,
            dictContentType: ZSTD_dct_auto,
            cdict: std::ptr::null_mut(),
        },
        cdict: std::ptr::null(),
        prefixDict: ZSTD_prefixDict_s {
            dict: std::ptr::null(),
            dictSize: 0,
            dictContentType: ZSTD_dct_auto,
        },
        mtctx: std::ptr::null_mut(),
        traceCtx: 0,
        blockSplitCtx: ZSTD_blockSplitCtx {
            fullSeqStoreChunk: SeqStore_t {
                sequencesStart: std::ptr::null_mut(),
                sequences: std::ptr::null_mut(),
                litStart: std::ptr::null_mut(),
                lit: std::ptr::null_mut(),
                llCode: std::ptr::null_mut(),
                mlCode: std::ptr::null_mut(),
                ofCode: std::ptr::null_mut(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            firstHalfSeqStore: SeqStore_t {
                sequencesStart: std::ptr::null_mut(),
                sequences: std::ptr::null_mut(),
                litStart: std::ptr::null_mut(),
                lit: std::ptr::null_mut(),
                llCode: std::ptr::null_mut(),
                mlCode: std::ptr::null_mut(),
                ofCode: std::ptr::null_mut(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            secondHalfSeqStore: SeqStore_t {
                sequencesStart: std::ptr::null_mut(),
                sequences: std::ptr::null_mut(),
                litStart: std::ptr::null_mut(),
                lit: std::ptr::null_mut(),
                llCode: std::ptr::null_mut(),
                mlCode: std::ptr::null_mut(),
                ofCode: std::ptr::null_mut(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            currSeqStore: SeqStore_t {
                sequencesStart: std::ptr::null_mut(),
                sequences: std::ptr::null_mut(),
                litStart: std::ptr::null_mut(),
                lit: std::ptr::null_mut(),
                llCode: std::ptr::null_mut(),
                mlCode: std::ptr::null_mut(),
                ofCode: std::ptr::null_mut(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            nextSeqStore: SeqStore_t {
                sequencesStart: std::ptr::null_mut(),
                sequences: std::ptr::null_mut(),
                litStart: std::ptr::null_mut(),
                lit: std::ptr::null_mut(),
                llCode: std::ptr::null_mut(),
                mlCode: std::ptr::null_mut(),
                ofCode: std::ptr::null_mut(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            partitions: [0; 196],
            entropyMetadata: ZSTD_entropyCTablesMetadata_t {
                hufMetadata: ZSTD_hufCTablesMetadata_t {
                    hType: set_basic,
                    hufDesBuffer: [0; 128],
                    hufDesSize: 0,
                },
                fseMetadata: ZSTD_fseCTablesMetadata_t {
                    llType: set_basic,
                    ofType: set_basic,
                    mlType: set_basic,
                    fseTablesBuffer: [0; 133],
                    fseTablesSize: 0,
                    lastCountSize: 0,
                },
            },
        },
        extSeqBuf: std::ptr::null_mut(),
        extSeqBufCapacity: 0,
    };
    ZSTD_initCCtx(&mut ctxBody, ZSTD_defaultCMem);
    result = ZSTD_compressCCtx(
        &mut ctxBody,
        dst,
        dstCapacity,
        src,
        srcSize,
        compressionLevel,
    );
    ZSTD_freeCCtxContent(&mut ctxBody);
    return result;
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCDictSize_advanced(
    mut dictSize: usize,
    mut cParams: ZSTD_compressionParameters,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
) -> usize {
    return (ZSTD_cwksp_alloc_size(
        size_of::<ZSTD_CDict>(),
    ))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE as usize))
        .wrapping_add(
            ZSTD_sizeof_matchState(
                &mut cParams,
                ZSTD_resolveRowMatchFinderMode(ZSTD_ps_auto, &mut cParams),
                1,
                0,
            ),
        )
        .wrapping_add(
            (if dictLoadMethod as u32
                == ZSTD_dlm_byRef as i32 as u32
            {
                0_usize
            } else {
                ZSTD_cwksp_alloc_size(
                    ZSTD_cwksp_align(
                        dictSize,
                        size_of::<*mut std::ffi::c_void>()
                            as std::ffi::c_ulong,
                    ),
                )
            }),
        );
}
#[no_mangle]
pub unsafe fn ZSTD_estimateCDictSize(
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> usize {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    return ZSTD_estimateCDictSize_advanced(dictSize, cParams, ZSTD_dlm_byCopy);
}
#[no_mangle]
pub unsafe fn ZSTD_sizeof_CDict(mut cdict: *const ZSTD_CDict) -> usize {
    if cdict.is_null() {
        return 0;
    }
    return (if (*cdict).workspace.workspace == cdict as *mut std::ffi::c_void {
        0 as std::ffi::c_ulong
    } else {
        size_of::<ZSTD_CDict>()
    })
        .wrapping_add(ZSTD_cwksp_sizeof(&(*cdict).workspace));
}
unsafe fn ZSTD_initCDict_internal(
    mut cdict: *mut ZSTD_CDict,
    mut dictBuffer: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut params: ZSTD_CCtx_params,
) -> usize {
    (*cdict).matchState.cParams = params.cParams;
    (*cdict).matchState.dedicatedDictSearch = params.enableDedicatedDictSearch;
    if dictLoadMethod as u32
        == ZSTD_dlm_byRef as i32 as u32 || dictBuffer.is_null()
        || dictSize == 0
    {
        (*cdict).dictContent = dictBuffer;
    } else {
        let mut internalBuffer = ZSTD_cwksp_reserve_object(
            &mut (*cdict).workspace,
            ZSTD_cwksp_align(
                dictSize,
                size_of::<*mut std::ffi::c_void>(),
            ),
        );
        RETURN_ERROR_IF!(internalBuffer.is_null(), ZSTD_error_memory_allocation);
        (*cdict).dictContent = internalBuffer;
        libc::memcpy(internalBuffer, dictBuffer, (dictSize) as usize);
    }
    (*cdict).dictContentSize = dictSize;
    (*cdict).dictContentType = dictContentType;
    (*cdict)
        .entropyWorkspace = ZSTD_cwksp_reserve_object(
        &mut (*cdict).workspace,
        HUF_WORKSPACE_SIZE as usize,
    ) as *mut u32;
    ZSTD_reset_compressedBlockState(&mut (*cdict).cBlockState);
    FORWARD_IF_ERROR!(
        ZSTD_reset_matchState(addr_of!((*cdict).matchState), addr_of!((*cdict).workspace), addr_of!(params
        .cParams), params.useRowMatchFinder, ZSTDcrp_makeClean, ZSTDirp_reset,
        ZSTD_resetTarget_CDict), ""
    );
    params.compressionLevel = ZSTD_CLEVEL_DEFAULT;
    params.fParams.contentSizeFlag = 1;
    let dictID = ZSTD_compress_insertDictionary(
        &mut (*cdict).cBlockState,
        &mut (*cdict).matchState,
        std::ptr::null_mut(),
        &mut (*cdict).workspace,
        &mut params,
        (*cdict).dictContent,
        (*cdict).dictContentSize,
        dictContentType,
        ZSTD_dtlm_full,
        ZSTD_tfp_forCDict,
        (*cdict).entropyWorkspace as *mut std::ffi::c_void,
    );
    FORWARD_IF_ERROR!(dictID, "ZSTD_compress_insertDictionary failed");
    (*cdict).dictID = dictID as u32;
    return 0;
}
unsafe fn ZSTD_createCDict_advanced_internal(
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut cParams: ZSTD_compressionParameters,
    mut useRowMatchFinder: ZSTD_ParamSwitch_e,
    mut enableDedicatedDictSearch: i32,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    if (customMem.customAlloc).is_none() as i32
        ^ (customMem.customFree).is_none() as i32 != 0
    {
        return std::ptr::null_mut();
    }
    let workspaceSize = (ZSTD_cwksp_alloc_size(
        size_of::<ZSTD_CDict>(),
    ))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE as usize))
        .wrapping_add(
            ZSTD_sizeof_matchState(
                &mut cParams,
                useRowMatchFinder,
                enableDedicatedDictSearch,
                0,
            ),
        )
        .wrapping_add(
            (if dictLoadMethod as u32
                == ZSTD_dlm_byRef as i32 as u32
            {
                0_usize
            } else {
                ZSTD_cwksp_alloc_size(
                    ZSTD_cwksp_align(
                        dictSize,
                        size_of::<*mut std::ffi::c_void>()
                            as std::ffi::c_ulong,
                    ),
                )
            }),
        );
    let workspace = ZSTD_customMalloc(workspaceSize, customMem);
    let mut ws = ZSTD_cwksp {
        workspace: std::ptr::null_mut(),
        workspaceEnd: std::ptr::null_mut(),
        objectEnd: std::ptr::null_mut(),
        tableEnd: std::ptr::null_mut(),
        tableValidEnd: std::ptr::null_mut(),
        allocStart: std::ptr::null_mut(),
        initOnceStart: std::ptr::null_mut(),
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    let mut cdict = std::ptr::null_mut();
    if workspace.is_null() {
        ZSTD_customFree(workspace, customMem);
        return std::ptr::null_mut();
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_dynamic_alloc);
    cdict = ZSTD_cwksp_reserve_object(
        &mut ws,
        size_of::<ZSTD_CDict>(),
    ) as *mut ZSTD_CDict;
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    (*cdict).customMem = customMem;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL;
    (*cdict).useRowMatchFinder = useRowMatchFinder;
    return cdict;
}
#[no_mangle]
pub unsafe fn ZSTD_createCDict_advanced(
    mut dictBuffer: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cParams: ZSTD_compressionParameters,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: ZSTD_f_zstd1,
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: std::ptr::null_mut(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    libc::memset(
        &mut cctxParams as *mut ZSTD_CCtx_params as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_CCtx_params>() as usize,
    );
    ZSTD_CCtxParams_init(&mut cctxParams, 0);
    cctxParams.cParams = cParams;
    cctxParams.customMem = customMem;
    return ZSTD_createCDict_advanced2(
        dictBuffer,
        dictSize,
        dictLoadMethod,
        dictContentType,
        &mut cctxParams,
        customMem,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_createCDict_advanced2(
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut originalCctxParams: *const ZSTD_CCtx_params,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let mut cctxParams = *originalCctxParams;
    let mut cParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0,
    };
    let mut cdict = std::ptr::null_mut();
    if (customMem.customAlloc).is_none() as i32
        ^ (customMem.customFree).is_none() as i32 != 0
    {
        return std::ptr::null_mut();
    }
    if cctxParams.enableDedicatedDictSearch != 0 {
        cParams = ZSTD_dedicatedDictSearch_getCParams(
            cctxParams.compressionLevel,
            dictSize,
        );
        ZSTD_overrideCParams(&mut cParams, &mut cctxParams.cParams);
    } else {
        cParams = ZSTD_getCParamsFromCCtxParams(
            &mut cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN as u64,
            dictSize,
            ZSTD_cpm_createCDict,
        );
    }
    if ZSTD_dedicatedDictSearch_isSupported(&mut cParams) == 0 {
        cctxParams.enableDedicatedDictSearch = 0;
        cParams = ZSTD_getCParamsFromCCtxParams(
            &mut cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN as u64,
            dictSize,
            ZSTD_cpm_createCDict,
        );
    }
    cctxParams.cParams = cParams;
    cctxParams
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        cctxParams.useRowMatchFinder,
        &mut cParams,
    );
    cdict = ZSTD_createCDict_advanced_internal(
        dictSize,
        dictLoadMethod,
        cctxParams.cParams,
        cctxParams.useRowMatchFinder,
        cctxParams.enableDedicatedDictSearch,
        customMem,
    );
    if cdict.is_null()
        || ERR_isError(
            ZSTD_initCDict_internal(
                cdict,
                dict,
                dictSize,
                dictLoadMethod,
                dictContentType,
                cctxParams,
            ),
        ) != 0
    {
        ZSTD_freeCDict(cdict);
        return std::ptr::null_mut();
    }
    return cdict;
}
#[no_mangle]
pub unsafe fn ZSTD_createCDict(
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> *mut ZSTD_CDict {
    let mut cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    let cdict = ZSTD_createCDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
        cParams,
        ZSTD_defaultCMem,
    );
    if !cdict.is_null() {
        (*cdict)
            .compressionLevel = if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        };
    }
    return cdict;
}
#[no_mangle]
pub unsafe fn ZSTD_createCDict_byReference(
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> *mut ZSTD_CDict {
    let mut cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    let cdict = ZSTD_createCDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        cParams,
        ZSTD_defaultCMem,
    );
    if !cdict.is_null() {
        (*cdict)
            .compressionLevel = if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        };
    }
    return cdict;
}
#[no_mangle]
pub unsafe fn ZSTD_freeCDict(mut cdict: *mut ZSTD_CDict) -> usize {
    if cdict.is_null() {
        return 0;
    }
    let cMem = (*cdict).customMem;
    let mut cdictInWorkspace = ZSTD_cwksp_owns_buffer(
        &mut (*cdict).workspace,
        cdict as *const std::ffi::c_void,
    );
    ZSTD_cwksp_free(&mut (*cdict).workspace, cMem);
    if cdictInWorkspace == 0 {
        ZSTD_customFree(cdict as *mut std::ffi::c_void, cMem);
    }
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_initStaticCDict(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: usize,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cParams: ZSTD_compressionParameters,
) -> *const ZSTD_CDict {
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(ZSTD_ps_auto, &mut cParams);
    let matchStateSize = ZSTD_sizeof_matchState(
        &mut cParams,
        useRowMatchFinder,
        1,
        0,
    );
    let neededSize = (ZSTD_cwksp_alloc_size(
        size_of::<ZSTD_CDict>(),
    ))
        .wrapping_add(
            (if dictLoadMethod as u32
                == ZSTD_dlm_byRef as i32 as u32
            {
                0_usize
            } else {
                ZSTD_cwksp_alloc_size(
                    ZSTD_cwksp_align(
                        dictSize,
                        size_of::<*mut std::ffi::c_void>()
                            as std::ffi::c_ulong,
                    ),
                )
            }),
        )
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE as usize))
        .wrapping_add(matchStateSize);
    let mut cdict = std::ptr::null_mut();
    let mut params = ZSTD_CCtx_params_s {
        format: ZSTD_f_zstd1,
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: std::ptr::null_mut(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    if workspace as usize & 7_usize != 0 {
        return std::ptr::null();
    }
    let mut ws = ZSTD_cwksp {
        workspace: std::ptr::null_mut(),
        workspaceEnd: std::ptr::null_mut(),
        objectEnd: std::ptr::null_mut(),
        tableEnd: std::ptr::null_mut(),
        tableValidEnd: std::ptr::null_mut(),
        allocStart: std::ptr::null_mut(),
        initOnceStart: std::ptr::null_mut(),
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_static_alloc);
    cdict = ZSTD_cwksp_reserve_object(
        &mut ws,
        size_of::<ZSTD_CDict>(),
    ) as *mut ZSTD_CDict;
    if cdict.is_null() {
        return std::ptr::null();
    }
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    if workspaceSize < neededSize {
        return std::ptr::null();
    }
    ZSTD_CCtxParams_init(&mut params, 0);
    params.cParams = cParams;
    params.useRowMatchFinder = useRowMatchFinder;
    (*cdict).useRowMatchFinder = useRowMatchFinder;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL;
    if ERR_isError(
        ZSTD_initCDict_internal(
            cdict,
            dict,
            dictSize,
            dictLoadMethod,
            dictContentType,
            params,
        ),
    ) != 0
    {
        return std::ptr::null();
    }
    return cdict;
}

pub unsafe fn ZSTD_getCParamsFromCDict(
    mut cdict: *const ZSTD_CDict,
) -> ZSTD_compressionParameters {
    debug_assert!(!cdict.is_null());
    return (*cdict).matchState.cParams;
}

#[no_mangle]
pub unsafe fn ZSTD_getDictID_fromCDict(
    mut cdict: *const ZSTD_CDict,
) -> u32 {
    if cdict.is_null() {
        return 0;
    }
    return (*cdict).dictID;
}
unsafe fn ZSTD_compressBegin_usingCDict_internal(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: u64,
) -> usize {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: ZSTD_f_zstd1,
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictDefaultAttach,
        literalCompressionMode: ZSTD_ps_auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: std::ptr::null_mut(),
        },
        prefetchCDictTables: ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: std::ptr::null_mut(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ps_auto,
    };
    RETURN_ERROR_IF!(cdict.is_null(), ZSTD_error_dictionary_wrong);
    let mut params = ZSTD_parameters {
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
    };
    params.fParams = fParams;
    params
        .cParams = if pledgedSrcSize
        < ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF as u64
        || pledgedSrcSize
            < ((*cdict).dictContentSize as u64)
                .wrapping_mul(ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER)
        || pledgedSrcSize == ZSTD_CONTENTSIZE_UNKNOWN
        || (*cdict).compressionLevel == 0
    {
        ZSTD_getCParamsFromCDict(cdict)
    } else {
        ZSTD_getCParams(
            (*cdict).compressionLevel,
            pledgedSrcSize,
            (*cdict).dictContentSize,
        )
    };
    ZSTD_CCtxParams_init_internal(
        &mut cctxParams,
        &mut params,
        (*cdict).compressionLevel,
    );
    if pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let limitedSrcSize = std::cmp::min(pledgedSrcSize, 1U << 19) as u32;
        let limitedSrcLog = if limitedSrcSize > 1 {
            (ZSTD_highbit32(limitedSrcSize.wrapping_sub(1)))
                .wrapping_add(1)
        } else {
            1 as u32
        };
        cctxParams.cParams.windowLog = std::cmp::max(cctxParams.cParams.windowLog, limitedSrcLog);
    }
    return ZSTD_compressBegin_internal(
        cctx,
        std::ptr::null(),
        0,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        cdict,
        &mut cctxParams,
        pledgedSrcSize as u64,
        ZSTDb_not_buffered,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_compressBegin_usingCDict_advanced(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: u64,
) -> usize {
    return ZSTD_compressBegin_usingCDict_internal(cctx, cdict, fParams, pledgedSrcSize);
}

/* ZSTD_compressBegin_usingCDict() :
 * cdict must be != NULL */
#[deprecated]
pub unsafe fn ZSTD_compressBegin_usingCDict_deprecated(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
) -> usize {
    debug_assert!(!cdict.is_null());
    let fParams = ZSTD_frameParameters {
        contentSizeFlag: 0 /*content*/,
        checksumFlag: 0 /*checksum*/,
        noDictIDFlag: 0 /*noDictID*/,
    };
    return ZSTD_compressBegin_usingCDict_internal(
        cctx,
        cdict,
        fParams,
        ZSTD_CONTENTSIZE_UNKNOWN,
    );
}

#[no_mangle]
pub unsafe fn ZSTD_compressBegin_usingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut cdict: *const ZSTD_CDict,
) -> usize {
    return ZSTD_compressBegin_usingCDict_deprecated(cctx, cdict);
}
unsafe fn ZSTD_compress_usingCDict_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut cdict: *const ZSTD_CDict,
    mut fParams: ZSTD_frameParameters,
) -> usize {
    FORWARD_IF_ERROR!(
        ZSTD_compressBegin_usingCDict_internal(cctx, cdict, fParams, srcSize), ""
    );
    return ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe fn ZSTD_compress_usingCDict_advanced(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut cdict: *const ZSTD_CDict,
    mut fParams: ZSTD_frameParameters,
) -> usize {
    return ZSTD_compress_usingCDict_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        cdict,
        fParams,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_compress_usingCDict(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut cdict: *const ZSTD_CDict,
) -> usize {
    let fParams = {
        let mut init = ZSTD_frameParameters {
            contentSizeFlag: 1,
            checksumFlag: 0,
            noDictIDFlag: 0,
        };
        init
    };
    return ZSTD_compress_usingCDict_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        cdict,
        fParams,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_createCStream() -> *mut ZSTD_CStream {
    return ZSTD_createCStream_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe fn ZSTD_initStaticCStream(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: usize,
) -> *mut ZSTD_CStream {
    return ZSTD_initStaticCCtx(workspace, workspaceSize);
}
#[no_mangle]
pub unsafe fn ZSTD_createCStream_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_CStream {
    return ZSTD_createCCtx_advanced(customMem);
}
#[no_mangle]
pub unsafe fn ZSTD_freeCStream(mut zcs: *mut ZSTD_CStream) -> usize {
    return ZSTD_freeCCtx(zcs);
}
#[no_mangle]
pub unsafe fn ZSTD_CStreamInSize() -> usize {
    return ZSTD_BLOCKSIZE_MAX as usize;
}
#[no_mangle]
pub unsafe fn ZSTD_CStreamOutSize() -> usize {
    return (ZSTD_compressBound(ZSTD_BLOCKSIZE_MAX as usize))
        .wrapping_add(ZSTD_blockHeaderSize)
        .wrapping_add(4);
}
unsafe fn ZSTD_getCParamMode(
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
) -> ZSTD_CParamMode_e {
    if !cdict.is_null() && ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) != 0 {
        return ZSTD_cpm_attachDict
    } else {
        return ZSTD_cpm_noAttachDict
    };
}
#[no_mangle]
pub unsafe fn ZSTD_resetCStream(
    mut zcs: *mut ZSTD_CStream,
    mut pss: u64,
) -> usize {
    let pledgedSrcSize = (if pss == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    }) as u64;
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize), ""
    );
    return 0;
}

/** ZSTD_initCStream_internal() :
 *  Note : for lib/compress only. Used by zstdmt_compress.c.
 *  Assumption 1 : params are valid
 *  Assumption 2 : either dict, or cdict, is defined, not both */
pub unsafe fn ZSTD_initCStream_internal(
    mut zcs: *mut ZSTD_CStream,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut cdict: *const ZSTD_CDict,
    mut params: *const ZSTD_CCtx_params,
    mut pledgedSrcSize: u64,
) -> usize {
    DEBUGLOG!(4, "ZSTD_initCStream_internal");
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize), ""
    );
    (*zcs).requestedParams = *params;
    debug_assert!(!(dict.is_null() ^ cdict.is_null()));  /* either dict or cdict, not both */
    if !dict.is_null() {
        FORWARD_IF_ERROR!(
            ZSTD_CCtx_loadDictionary(zcs, dict, dictSize), ""
        );
    } else {
        /* Dictionary is cleared if !cdict */
        FORWARD_IF_ERROR!(ZSTD_CCtx_refCDict(zcs, cdict), "");
    }
    return 0;
}

#[no_mangle]
pub unsafe fn ZSTD_initCStream_usingCDict_advanced(
    mut zcs: *mut ZSTD_CStream,
    mut cdict: *const ZSTD_CDict,
    mut fParams: ZSTD_frameParameters,
    mut pledgedSrcSize: u64,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize), ""
    );
    (*zcs).requestedParams.fParams = fParams;
    FORWARD_IF_ERROR!(ZSTD_CCtx_refCDict(zcs, cdict), "");
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_initCStream_usingCDict(
    mut zcs: *mut ZSTD_CStream,
    mut cdict: *const ZSTD_CDict,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(ZSTD_CCtx_refCDict(zcs, cdict), "");
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_initCStream_advanced(
    mut zcs: *mut ZSTD_CStream,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut params: ZSTD_parameters,
    mut pss: u64,
) -> usize {
    let pledgedSrcSize = (if pss == 0
        && params.fParams.contentSizeFlag == 0
    {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    }) as u64;
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize), ""
    );
    FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "");
    ZSTD_CCtxParams_setZstdParams(&mut (*zcs).requestedParams, &mut params);
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_loadDictionary(zcs, dict, dictSize), ""
    );
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_initCStream_usingDict(
    mut zcs: *mut ZSTD_CStream,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut compressionLevel: i32,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(zcs, ZSTD_c_compressionLevel, compressionLevel), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_loadDictionary(zcs, dict, dictSize), ""
    );
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_initCStream_srcSize(
    mut zcs: *mut ZSTD_CStream,
    mut compressionLevel: i32,
    mut pss: u64,
) -> usize {
    let pledgedSrcSize = (if pss == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    }) as u64;
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(ZSTD_CCtx_refCDict(zcs, NULL), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(zcs, ZSTD_c_compressionLevel, compressionLevel), ""
    );
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize), ""
    );
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_initCStream(
    mut zcs: *mut ZSTD_CStream,
    mut compressionLevel: i32,
) -> usize {
    FORWARD_IF_ERROR!(ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR!(ZSTD_CCtx_refCDict(zcs, NULL), "");
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_setParameter(zcs, ZSTD_c_compressionLevel, compressionLevel), ""
    );
    return 0;
}
unsafe fn ZSTD_nextInputSizeHint(mut cctx: *const ZSTD_CCtx) -> usize {
    if (*cctx).appliedParams.inBufferMode as u32
        == ZSTD_bm_stable as i32 as u32
    {
        return ((*cctx).blockSizeMax).wrapping_sub((*cctx).stableIn_notConsumed);
    }
    let mut hintInSize = ((*cctx).inBuffTarget).wrapping_sub((*cctx).inBuffPos);
    if hintInSize == 0 {
        hintInSize = (*cctx).blockSizeMax;
    }
    return hintInSize;
}
unsafe fn ZSTD_compressStream_generic(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
    flushMode: ZSTD_EndDirective,
) -> usize {
    let istart = (*input).src as *const std::ffi::c_char;
    let iend = if !istart.is_null() {
        istart.offset((*input).size as isize)
    } else {
        istart
    };
    let mut ip = if !istart.is_null() {
        istart.offset((*input).pos as isize)
    } else {
        istart
    };
    let ostart = (*output).dst as *mut std::ffi::c_char;
    let oend = if !ostart.is_null() {
        ostart.offset((*output).size as isize)
    } else {
        ostart
    };
    let mut op = if !ostart.is_null() {
        ostart.offset((*output).pos as isize)
    } else {
        ostart
    };
    let mut someMoreWork: u32 = 1;
    if (*zcs).appliedParams.inBufferMode as u32
        == ZSTD_bm_stable as i32 as u32
    {
        (*input).pos = ((*input).pos).wrapping_sub((*zcs).stableIn_notConsumed);
        if !ip.is_null() {
            ip = ip.offset(-((*zcs).stableIn_notConsumed as isize));
        }
        (*zcs).stableIn_notConsumed = 0;
    }
    (*zcs).appliedParams.inBufferMode as u32
        == ZSTD_bm_buffered as i32 as u32;
    (*zcs).appliedParams.outBufferMode as u32
        == ZSTD_bm_buffered as i32 as u32;
    ((*input).src).is_null();
    ((*output).dst).is_null();
    while someMoreWork != 0 {
        let mut current_block_156: u64;
        match (*zcs).streamStage as u32 {
            0 => return ERROR(ZSTD_error_init_missing),
            1 => {
                if flushMode as u32
                    == ZSTD_e_end as i32 as u32
                    && (oend.offset_from(op) as std::ffi::c_long as usize
                        >= ZSTD_compressBound(
                            iend.offset_from(ip) as std::ffi::c_long as usize,
                        )
                        || (*zcs).appliedParams.outBufferMode as u32
                            == ZSTD_bm_stable as i32 as u32)
                    && (*zcs).inBuffPos == 0
                {
                    let cSize = ZSTD_compressEnd_public(
                        zcs,
                        op as *mut std::ffi::c_void,
                        oend.offset_from(op) as std::ffi::c_long as usize,
                        ip as *const std::ffi::c_void,
                        iend.offset_from(ip) as std::ffi::c_long as usize,
                    );
                    FORWARD_IF_ERROR!(cSize, "ZSTD_compressEnd failed");
                    ip = iend;
                    op = op.offset(cSize as isize);
                    (*zcs).frameEnded = 1;
                    ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                    someMoreWork = 0;
                    current_block_156 = 16754622181974910496;
                } else {
                    if (*zcs).appliedParams.inBufferMode as u32
                        == ZSTD_bm_buffered as i32 as u32
                    {
                        let toLoad = ((*zcs).inBuffTarget)
                            .wrapping_sub((*zcs).inBuffPos);
                        let loaded = ZSTD_limitCopy(
                            ((*zcs).inBuff).offset((*zcs).inBuffPos as isize)
                                as *mut std::ffi::c_void,
                            toLoad,
                            ip as *const std::ffi::c_void,
                            iend.offset_from(ip) as std::ffi::c_long as usize,
                        );
                        (*zcs).inBuffPos = ((*zcs).inBuffPos).wrapping_add(loaded);
                        if !ip.is_null() {
                            ip = ip.offset(loaded as isize);
                        }
                        if flushMode as u32
                            == ZSTD_e_continue as i32 as u32
                            && (*zcs).inBuffPos < (*zcs).inBuffTarget
                        {
                            someMoreWork = 0;
                            current_block_156 = 16754622181974910496;
                        } else if flushMode as u32
                            == ZSTD_e_flush as i32 as u32
                            && (*zcs).inBuffPos == (*zcs).inToCompress
                        {
                            someMoreWork = 0;
                            current_block_156 = 16754622181974910496;
                        } else {
                            current_block_156 = 13910774313357589740;
                        }
                    } else if flushMode as u32
                        == ZSTD_e_continue as i32 as u32
                        && (iend.offset_from(ip) as std::ffi::c_long as usize)
                            < (*zcs).blockSizeMax
                    {
                        (*zcs)
                            .stableIn_notConsumed = iend.offset_from(ip)
                            as std::ffi::c_long as usize;
                        ip = iend;
                        someMoreWork = 0;
                        current_block_156 = 16754622181974910496;
                    } else if flushMode as u32
                        == ZSTD_e_flush as i32 as u32
                        && ip == iend
                    {
                        someMoreWork = 0;
                        current_block_156 = 16754622181974910496;
                    } else {
                        current_block_156 = 13910774313357589740;
                    }
                    match current_block_156 {
                        16754622181974910496 => {}
                        _ => {
                            let inputBuffered = ((*zcs).appliedParams.inBufferMode
                                as u32
                                == ZSTD_bm_buffered as i32 as u32)
                                as i32;
                            let mut cDst = std::ptr::null_mut();
                            let mut cSize_0: usize = 0;
                            let mut oSize = oend.offset_from(op) as std::ffi::c_long
                                as usize;
                            let iSize = if inputBuffered != 0 {
                                ((*zcs).inBuffPos).wrapping_sub((*zcs).inToCompress)
                            } else {
                                std::cmp::min((usize) (iend - ip), (*zcs).blockSizeMax)
                            };
                            if oSize >= ZSTD_compressBound(iSize)
                                || (*zcs).appliedParams.outBufferMode as u32
                                    == ZSTD_bm_stable as i32 as u32
                            {
                                cDst = op as *mut std::ffi::c_void;
                            } else {
                                cDst = (*zcs).outBuff as *mut std::ffi::c_void;
                                oSize = (*zcs).outBuffSize;
                            }
                            if inputBuffered != 0 {
                                let lastBlock = (flushMode as u32
                                    == ZSTD_e_end as i32 as u32
                                    && ip == iend) as i32 as u32;
                                cSize_0 = if lastBlock != 0 {
                                    ZSTD_compressEnd_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ((*zcs).inBuff).offset((*zcs).inToCompress as isize)
                                            as *const std::ffi::c_void,
                                        iSize,
                                    )
                                } else {
                                    ZSTD_compressContinue_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ((*zcs).inBuff).offset((*zcs).inToCompress as isize)
                                            as *const std::ffi::c_void,
                                        iSize,
                                    )
                                };
                                FORWARD_IF_ERROR!(
                                    cSize, "%s", lastBlock ? "ZSTD_compressEnd failed" :
                                    "ZSTD_compressContinue failed"
                                );
                                (*zcs).frameEnded = lastBlock;
                                (*zcs)
                                    .inBuffTarget = ((*zcs).inBuffPos)
                                    .wrapping_add((*zcs).blockSizeMax);
                                if (*zcs).inBuffTarget > (*zcs).inBuffSize {
                                    (*zcs).inBuffPos = 0;
                                    (*zcs).inBuffTarget = (*zcs).blockSizeMax;
                                }
                                lastBlock == 0;
                                (*zcs).inToCompress = (*zcs).inBuffPos;
                            } else {
                                let lastBlock_0 = (flushMode as u32
                                    == ZSTD_e_end as i32 as u32
                                    && ip.offset(iSize as isize) == iend) as i32
                                    as u32;
                                cSize_0 = if lastBlock_0 != 0 {
                                    ZSTD_compressEnd_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ip as *const std::ffi::c_void,
                                        iSize,
                                    )
                                } else {
                                    ZSTD_compressContinue_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ip as *const std::ffi::c_void,
                                        iSize,
                                    )
                                };
                                if !ip.is_null() {
                                    ip = ip.offset(iSize as isize);
                                }
                                FORWARD_IF_ERROR!(
                                    cSize, "%s", lastBlock ? "ZSTD_compressEnd failed" :
                                    "ZSTD_compressContinue failed"
                                );
                                (*zcs).frameEnded = lastBlock_0;
                                lastBlock_0 != 0;
                            }
                            if cDst == op as *mut std::ffi::c_void {
                                op = op.offset(cSize_0 as isize);
                                if (*zcs).frameEnded != 0 {
                                    someMoreWork = 0;
                                    ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                                }
                                current_block_156 = 16754622181974910496;
                            } else {
                                (*zcs).outBuffContentSize = cSize_0;
                                (*zcs).outBuffFlushedSize = 0;
                                (*zcs).streamStage = zcss_flush;
                                current_block_156 = 5431927413890720344;
                            }
                        }
                    }
                }
            }
            2 => {
                current_block_156 = 5431927413890720344;
            }
            _ => {
                current_block_156 = 16754622181974910496;
            }
        }
        match current_block_156 {
            5431927413890720344 => {
                let toFlush = ((*zcs).outBuffContentSize)
                    .wrapping_sub((*zcs).outBuffFlushedSize);
                let flushed = ZSTD_limitCopy(
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as usize,
                    ((*zcs).outBuff).offset((*zcs).outBuffFlushedSize as isize)
                        as *const std::ffi::c_void,
                    toFlush,
                );
                if flushed != 0 {
                    op = op.offset(flushed as isize);
                }
                (*zcs)
                    .outBuffFlushedSize = ((*zcs).outBuffFlushedSize)
                    .wrapping_add(flushed);
                if toFlush != flushed {
                    someMoreWork = 0;
                } else {
                    (*zcs).outBuffFlushedSize = 0;
                    (*zcs).outBuffContentSize = (*zcs).outBuffFlushedSize;
                    if (*zcs).frameEnded != 0 {
                        someMoreWork = 0;
                        ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                    } else {
                        (*zcs).streamStage = zcss_load;
                    }
                }
            }
            _ => {}
        }
    }
    (*input).pos = ip.offset_from(istart) as std::ffi::c_long as usize;
    (*output).pos = op.offset_from(ostart) as std::ffi::c_long as usize;
    if (*zcs).frameEnded != 0 {
        return 0;
    }
    return ZSTD_nextInputSizeHint(zcs);
}
unsafe fn ZSTD_nextInputSizeHint_MTorST(
    mut cctx: *const ZSTD_CCtx,
) -> usize {
    if (*cctx).appliedParams.nbWorkers >= 1 {
        return ZSTDMT_nextInputSizeHint((*cctx).mtctx);
    }
    return ZSTD_nextInputSizeHint(cctx);
}
#[no_mangle]
pub unsafe fn ZSTD_compressStream(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
) -> usize {
    FORWARD_IF_ERROR!(
        ZSTD_compressStream2(zcs, output, input, ZSTD_e_continue), ""
    );
    return ZSTD_nextInputSizeHint_MTorST(zcs);
}
unsafe fn ZSTD_setBufferExpectations(
    mut cctx: *mut ZSTD_CCtx,
    mut output: *const ZSTD_outBuffer,
    mut input: *const ZSTD_inBuffer,
) {
    if (*cctx).appliedParams.inBufferMode as u32
        == ZSTD_bm_stable as i32 as u32
    {
        (*cctx).expectedInBuffer = *input;
    }
    if (*cctx).appliedParams.outBufferMode as u32
        == ZSTD_bm_stable as i32 as u32
    {
        (*cctx).expectedOutBufferSize = ((*output).size).wrapping_sub((*output).pos);
    }
}
unsafe fn ZSTD_checkBufferStability(
    mut cctx: *const ZSTD_CCtx,
    mut output: *const ZSTD_outBuffer,
    mut input: *const ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> usize {
    if (*cctx).appliedParams.inBufferMode as u32
        == ZSTD_bm_stable as i32 as u32
    {
        let expect = (*cctx).expectedInBuffer;
        if expect.src != (*input).src || expect.pos != (*input).pos {
            return -(ZSTD_error_stabilityCondition_notRespected as i32)
                as usize;
        }
    }
    if (*cctx).appliedParams.outBufferMode as u32
        == ZSTD_bm_stable as i32 as u32
    {
        let outBufferSize = ((*output).size).wrapping_sub((*output).pos);
        if (*cctx).expectedOutBufferSize != outBufferSize {
            return -(ZSTD_error_stabilityCondition_notRespected as i32)
                as usize;
        }
    }
    return 0;
}
unsafe fn ZSTD_CCtx_init_compressStream2(
    mut cctx: *mut ZSTD_CCtx,
    mut endOp: ZSTD_EndDirective,
    mut inSize: usize,
) -> usize {
    let mut params = (*cctx).requestedParams;
    let prefixDict = (*cctx).prefixDict;
    FORWARD_IF_ERROR!(ZSTD_initLocalDict(cctx), "");
    libc::memset(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_prefixDict>() as usize,
    );
    if !((*cctx).cdict).is_null() && ((*cctx).localDict.cdict).is_null() {
        params.compressionLevel = (*(*cctx).cdict).compressionLevel;
    }
    if endOp as u32 == ZSTD_e_end as i32 as u32 {
        (*cctx)
            .pledgedSrcSizePlusOne = inSize.wrapping_add(1)
            as u64;
    }
    let dictSize = if !(prefixDict.dict).is_null() {
        prefixDict.dictSize
    } else if !((*cctx).cdict).is_null() {
        (*(*cctx).cdict).dictContentSize
    } else {
        0_usize
    };
    let mode = ZSTD_getCParamMode(
        (*cctx).cdict,
        &mut params,
        ((*cctx).pledgedSrcSizePlusOne)
            .wrapping_sub(1) as u64,
    );
    params
        .cParams = ZSTD_getCParamsFromCCtxParams(
        &mut params,
        ((*cctx).pledgedSrcSizePlusOne)
            .wrapping_sub(1) as u64,
        dictSize,
        mode,
    );
    params
        .postBlockSplitter = ZSTD_resolveBlockSplitterMode(
        params.postBlockSplitter,
        &mut params.cParams,
    );
    params
        .ldmParams
        .enableLdm = ZSTD_resolveEnableLdm(
        params.ldmParams.enableLdm,
        &mut params.cParams,
    );
    params
        .useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(
        params.useRowMatchFinder,
        &mut params.cParams,
    );
    params
        .validateSequences = ZSTD_resolveExternalSequenceValidation(
        params.validateSequences,
    );
    params.maxBlockSize = ZSTD_resolveMaxBlockSize(params.maxBlockSize);
    params
        .searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        params.searchForExternalRepcodes,
        params.compressionLevel,
    );
    if ZSTD_hasExtSeqProd(&mut params) != 0 && params.nbWorkers >= 1 {
        return -(ZSTD_error_parameter_combination_unsupported as i32)
            as usize;
    }
    if ((*cctx).pledgedSrcSizePlusOne)
        .wrapping_sub(1)
        <= ZSTDMT_JOBSIZE_MIN as u64
    {
        params.nbWorkers = 0;
    }
    if params.nbWorkers > 0 {
        (*cctx)
            .traceCtx = if (Some(
            ZSTD_trace_compress_begin
                as unsafe extern "C" fn(*const ZSTD_CCtx_s) -> ZSTD_TraceCtx,
        ))
            .is_some()
        {
            ZSTD_trace_compress_begin(cctx)
        } else {
            0 as ZSTD_TraceCtx
        };
        if ((*cctx).mtctx).is_null() {
            (*cctx)
                .mtctx = ZSTDMT_createCCtx_advanced(
                params.nbWorkers as u32,
                (*cctx).customMem,
                (*cctx).pool,
            );
            RETURN_ERROR_IF!(((*cctx).mtctx).is_null(), ZSTD_error_memory_allocation);
        }
        FORWARD_IF_ERROR!(
            ZSTDMT_initCStream_internal((*cctx).mtctx, prefixDict.dict, prefixDict
            .dictSize, prefixDict.dictContentType, (*cctx).cdict, params, (*cctx).pledgedSrcSizePlusOne - 1), ""
        );
        (*cctx)
            .dictID = if !((*cctx).cdict).is_null() {
            (*(*cctx).cdict).dictID
        } else {
            0_u32
        };
        (*cctx)
            .dictContentSize = if !((*cctx).cdict).is_null() {
            (*(*cctx).cdict).dictContentSize
        } else {
            prefixDict.dictSize
        };
        (*cctx).consumedSrcSize = 0;
        (*cctx).producedCSize = 0;
        (*cctx).streamStage = zcss_load;
        (*cctx).appliedParams = params;
    } else {
        let pledgedSrcSize = ((*cctx).pledgedSrcSizePlusOne)
            .wrapping_sub(1) as u64;
        FORWARD_IF_ERROR!(
            ZSTD_compressBegin_internal(cctx, prefixDict.dict, prefixDict.dictSize,
            prefixDict.dictContentType, ZSTD_dtlm_fast, (*cctx).cdict, addr_of!(params),
            pledgedSrcSize, ZSTDb_buffered), ""
        );
        (*cctx).inToCompress = 0;
        (*cctx).inBuffPos = 0;
        if (*cctx).appliedParams.inBufferMode as u32
            == ZSTD_bm_buffered as i32 as u32
        {
            (*cctx)
                .inBuffTarget = ((*cctx).blockSizeMax)
                .wrapping_add(
                    ((*cctx).blockSizeMax == pledgedSrcSize) as i32 as usize,
                );
        } else {
            (*cctx).inBuffTarget = 0;
        }
        (*cctx).outBuffFlushedSize = 0;
        (*cctx).outBuffContentSize = (*cctx).outBuffFlushedSize;
        (*cctx).streamStage = zcss_load;
        (*cctx).frameEnded = 0;
    }
    return 0;
}
#[no_mangle]
pub unsafe fn ZSTD_compressStream2(
    mut cctx: *mut ZSTD_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> usize {
    RETURN_ERROR_IF!((*output).pos > (*output).size, ZSTD_error_dstSize_tooSmall);
    RETURN_ERROR_IF!((*input).pos > (*input).size, ZSTD_error_srcSize_wrong);
    RETURN_ERROR_IF!(endOp as u32 > ZSTD_e_end as i32 as u32, ZSTD_error_parameter_outOfBound);
    if (*cctx).streamStage as u32
        == zcss_init as i32 as u32
    {
        let inputSize = ((*input).size).wrapping_sub((*input).pos);
        let totalInputSize = inputSize.wrapping_add((*cctx).stableIn_notConsumed);
        if (*cctx).requestedParams.inBufferMode as u32
            == ZSTD_bm_stable as i32 as u32
            && endOp as u32
                == ZSTD_e_continue as i32 as u32
            && totalInputSize < ZSTD_BLOCKSIZE_MAX as usize
        {
            if (*cctx).stableIn_notConsumed != 0 {
                if (*input).src != (*cctx).expectedInBuffer.src {
                    return -(ZSTD_error_stabilityCondition_notRespected
                        as i32) as usize;
                }
                if (*input).pos != (*cctx).expectedInBuffer.size {
                    return -(ZSTD_error_stabilityCondition_notRespected
                        as i32) as usize;
                }
            }
            (*input).pos = (*input).size;
            (*cctx).expectedInBuffer = *input;
            (*cctx)
                .stableIn_notConsumed = ((*cctx).stableIn_notConsumed)
                .wrapping_add(inputSize);
            return ZSTD_FRAMEHEADERSIZE_MIN((*cctx).requestedParams.format);
        }
        FORWARD_IF_ERROR!(
            ZSTD_CCtx_init_compressStream2(cctx, endOp, totalInputSize),
            "compressStream2 initialization failed"
        );
        ZSTD_setBufferExpectations(cctx, output, input);
    }
    FORWARD_IF_ERROR!(
        ZSTD_checkBufferStability(cctx, output, input, endOp), "invalid buffers"
    );
    if (*cctx).appliedParams.nbWorkers > 0 {
        let mut flushMin: usize = 0;
        if (*cctx).cParamsChanged != 0 {
            ZSTDMT_updateCParams_whileCompressing(
                (*cctx).mtctx,
                &mut (*cctx).requestedParams,
            );
            (*cctx).cParamsChanged = 0;
        }
        if (*cctx).stableIn_notConsumed != 0 {
            (*input).pos = ((*input).pos).wrapping_sub((*cctx).stableIn_notConsumed);
            (*cctx).stableIn_notConsumed = 0;
        }
        loop {
            let ipos = (*input).pos;
            let opos = (*output).pos;
            flushMin = ZSTDMT_compressStream_generic(
                (*cctx).mtctx,
                output,
                input,
                endOp,
            );
            (*cctx)
                .consumedSrcSize = ((*cctx).consumedSrcSize)
                .wrapping_add(
                    ((*input).pos).wrapping_sub(ipos) as u64,
                );
            (*cctx)
                .producedCSize = ((*cctx).producedCSize)
                .wrapping_add(
                    ((*output).pos).wrapping_sub(opos) as u64,
                );
            if ERR_isError(flushMin)
                || endOp as u32
                    == ZSTD_e_end as i32 as u32
                    && flushMin == 0
            {
                if flushMin == 0 {
                    ZSTD_CCtx_trace(cctx, 0);
                }
                ZSTD_CCtx_reset(cctx, ZSTD_reset_session_only);
            }
            FORWARD_IF_ERROR!(
                flushMin, "ZSTDMT_compressStream_generic failed"
            );
            if endOp as u32
                == ZSTD_e_continue as i32 as u32
            {
                if (*input).pos != ipos || (*output).pos != opos
                    || (*input).pos == (*input).size || (*output).pos == (*output).size
                {
                    break;
                }
            } else if flushMin == 0
                || (*output).pos == (*output).size
            {
                break;
            }
        }
        ZSTD_setBufferExpectations(cctx, output, input);
        return flushMin;
    }
    FORWARD_IF_ERROR!(
        ZSTD_compressStream_generic(cctx, output, input, endOp), ""
    );
    ZSTD_setBufferExpectations(cctx, output, input);
    return ((*cctx).outBuffContentSize).wrapping_sub((*cctx).outBuffFlushedSize);
}
#[no_mangle]
pub unsafe fn ZSTD_compressStream2_simpleArgs(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut dstPos: *mut usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut srcPos: *mut usize,
    mut endOp: ZSTD_EndDirective,
) -> usize {
    let mut output = ZSTD_outBuffer_s {
        dst: std::ptr::null_mut(),
        size: 0,
        pos: 0,
    };
    let mut input = ZSTD_inBuffer_s {
        src: std::ptr::null(),
        size: 0,
        pos: 0,
    };
    output.dst = dst;
    output.size = dstCapacity;
    output.pos = *dstPos;
    input.src = src;
    input.size = srcSize;
    input.pos = *srcPos;
    let cErr = ZSTD_compressStream2(cctx, &mut output, &mut input, endOp);
    *dstPos = output.pos;
    *srcPos = input.pos;
    return cErr;
}
#[no_mangle]
pub unsafe fn ZSTD_compress2(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let originalInBufferMode = (*cctx).requestedParams.inBufferMode;
    let originalOutBufferMode = (*cctx).requestedParams.outBufferMode;
    ZSTD_CCtx_reset(cctx, ZSTD_reset_session_only);
    (*cctx).requestedParams.inBufferMode = ZSTD_bm_stable;
    (*cctx).requestedParams.outBufferMode = ZSTD_bm_stable;
    let mut oPos: usize = 0;
    let mut iPos: usize = 0;
    let result = ZSTD_compressStream2_simpleArgs(
        cctx,
        dst,
        dstCapacity,
        &mut oPos,
        src,
        srcSize,
        &mut iPos,
        ZSTD_e_end,
    );
    (*cctx).requestedParams.inBufferMode = originalInBufferMode;
    (*cctx).requestedParams.outBufferMode = originalOutBufferMode;
    FORWARD_IF_ERROR!(result, "ZSTD_compressStream2_simpleArgs failed");
    RETURN_ERROR_IF!(result != 0, ZSTD_error_dstSize_tooSmall);
    return oPos;
}
unsafe fn ZSTD_validateSequence(
    mut offBase: u32,
    mut matchLength: u32,
    mut minMatch: u32,
    mut posInSrc: usize,
    mut windowLog: u32,
    mut dictSize: usize,
    mut useSequenceProducer: i32,
) -> usize {
    let windowSize = (1 as u32) << windowLog;
    let offsetBound = if posInSrc > windowSize as usize {
        windowSize as usize
    } else {
        posInSrc.wrapping_add(dictSize)
    };
    let matchLenLowerBound = (if minMatch == 3
        || useSequenceProducer != 0
    {
        3 as i32
    } else {
        4 as i32
    }) as usize;
    RETURN_ERROR_IF!(offBase as usize > offsetBound.wrapping_add(3), ZSTD_error_externalSequences_invalid);
    RETURN_ERROR_IF!((matchLength as usize) < matchLenLowerBound, ZSTD_error_externalSequences_invalid);
    return 0;
}
unsafe fn ZSTD_finalizeOffBase(
    mut rawOffset: u32,
    mut rep: *const u32,
    mut ll0: u32,
) -> u32 {
    let mut offBase = OFFSET_TO_OFFBASE!(rawOffset);
    if ll0 == 0 && rawOffset == *rep.offset(0) {
        offBase = REPCODE1_TO_OFFBASE as u32;
    } else if rawOffset == *rep.offset(1) {
        offBase = REPCODE_TO_OFFBASE!(2 - ll0);
    } else if rawOffset == *rep.offset(2) {
        offBase = REPCODE_TO_OFFBASE!(3 - ll0);
    } else if ll0 != 0
        && rawOffset
            == (*rep.offset(0))
                .wrapping_sub(1)
    {
        offBase = REPCODE3_TO_OFFBASE as u32;
    }
    return offBase;
}
unsafe fn ZSTD_transferSequences_wBlockDelim(
    mut cctx: *mut ZSTD_CCtx,
    mut seqPos: *mut ZSTD_SequencePosition,
    inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut src: *const std::ffi::c_void,
    mut blockSize: usize,
    mut externalRepSearch: ZSTD_ParamSwitch_e,
) -> usize {
    let mut idx = (*seqPos).idx;
    let startIdx = idx;
    let mut ip = src as *const u8;
    let iend = ip.offset(blockSize as isize);
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };
    let mut dictSize: u32 = 0;
    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize as u32;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize as u32;
    } else {
        dictSize = 0;
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut std::ffi::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    while (idx as usize) < inSeqsSize
        && ((*inSeqs.offset(idx as isize)).matchLength
            != 0
            || (*inSeqs.offset(idx as isize)).offset
                != 0)
    {
        let litLength = (*inSeqs.offset(idx as isize)).litLength;
        let matchLength = (*inSeqs.offset(idx as isize)).matchLength;
        let mut offBase: u32 = 0;
        if externalRepSearch as u32
            == ZSTD_ps_disable as i32 as u32
        {
            offBase = OFFSET_TO_OFFBASE!(inSeqs[idx].offset);
        } else {
            let ll0 = (litLength == 0) as i32
                as u32;
            offBase = ZSTD_finalizeOffBase(
                (*inSeqs.offset(idx as isize)).offset,
                (updatedRepcodes.rep).as_mut_ptr() as *const u32,
                ll0,
            );
            ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        }
        if (*cctx).appliedParams.validateSequences != 0 {
            (*seqPos)
                .posInSrc = ((*seqPos).posInSrc)
                .wrapping_add(litLength.wrapping_add(matchLength) as usize);
            FORWARD_IF_ERROR!(
                ZSTD_validateSequence(offBase, matchLength, (*cctx).appliedParams.cParams
                .minMatch, (*seqPos).posInSrc, (*cctx).appliedParams.cParams.windowLog,
                dictSize, ZSTD_hasExtSeqProd(addr_of!((*cctx).appliedParams))),
                "Sequence validation failed"
            );
        }
        RETURN_ERROR_IF!(idx.wrapping_sub((*seqPos).idx) as usize >= (*cctx).seqStore.maxNbSeq, ZSTD_error_externalSequences_invalid);
        ZSTD_storeSeq(
            &mut (*cctx).seqStore,
            litLength as usize,
            ip,
            iend,
            offBase,
            matchLength as usize,
        );
        ip = ip.offset(matchLength.wrapping_add(litLength) as isize);
        idx = idx.wrapping_add(1);
        idx;
    }
    RETURN_ERROR_IF!(idx as usize == inSeqsSize, ZSTD_error_externalSequences_invalid);
    if externalRepSearch as u32
        == ZSTD_ps_disable as i32 as u32 && idx != startIdx
    {
        let rep = (updatedRepcodes.rep).as_mut_ptr();
        let mut lastSeqIdx = idx.wrapping_sub(1);
        if lastSeqIdx >= startIdx.wrapping_add(2) {
            *rep
                .offset(
                    2,
                ) = (*inSeqs
                .offset(lastSeqIdx.wrapping_sub(2) as isize))
                .offset;
            *rep
                .offset(
                    1,
                ) = (*inSeqs
                .offset(lastSeqIdx.wrapping_sub(1) as isize))
                .offset;
            *rep
                .offset(
                    0,
                ) = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else if lastSeqIdx == startIdx.wrapping_add(1) {
            *rep
                .offset(
                    2,
                ) = *rep.offset(0);
            *rep
                .offset(
                    1,
                ) = (*inSeqs
                .offset(lastSeqIdx.wrapping_sub(1) as isize))
                .offset;
            *rep
                .offset(
                    0,
                ) = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else {
            *rep
                .offset(
                    2,
                ) = *rep.offset(1);
            *rep
                .offset(
                    1,
                ) = *rep.offset(0);
            *rep
                .offset(
                    0,
                ) = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        }
    }
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut std::ffi::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    if (*inSeqs.offset(idx as isize)).litLength != 0 {
        ZSTD_storeLastLiterals(
            &mut (*cctx).seqStore,
            ip,
            (*inSeqs.offset(idx as isize)).litLength as usize,
        );
        ip = ip.offset((*inSeqs.offset(idx as isize)).litLength as isize);
        (*seqPos)
            .posInSrc = ((*seqPos).posInSrc)
            .wrapping_add((*inSeqs.offset(idx as isize)).litLength as usize);
    }
    RETURN_ERROR_IF!(ip != iend, ZSTD_error_externalSequences_invalid);
    (*seqPos).idx = idx.wrapping_add(1);
    return blockSize;
}
unsafe fn ZSTD_transferSequences_noDelim(
    mut cctx: *mut ZSTD_CCtx,
    mut seqPos: *mut ZSTD_SequencePosition,
    inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut src: *const std::ffi::c_void,
    mut blockSize: usize,
    mut externalRepSearch: ZSTD_ParamSwitch_e,
) -> usize {
    let mut idx = (*seqPos).idx;
    let mut startPosInSequence = (*seqPos).posInSequence;
    let mut endPosInSequence = ((*seqPos).posInSequence).wrapping_add(blockSize as u32);
    let mut dictSize: usize = 0;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut iend = istart.offset(blockSize as isize);
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };
    let mut bytesAdjustment: u32 = 0;
    let mut finalMatchSplit: u32 = 0;
    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize;
    } else {
        dictSize = 0;
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut std::ffi::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    while endPosInSequence != 0 && (idx as usize) < inSeqsSize && finalMatchSplit == 0 {
        let currSeq = *inSeqs.offset(idx as isize);
        let mut litLength = currSeq.litLength;
        let mut matchLength = currSeq.matchLength;
        let rawOffset = currSeq.offset;
        let mut offBase: u32 = 0;
        if endPosInSequence >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            if startPosInSequence >= litLength {
                startPosInSequence = startPosInSequence.wrapping_sub(litLength);
                litLength = 0;
                matchLength = matchLength.wrapping_sub(startPosInSequence);
            } else {
                litLength = litLength.wrapping_sub(startPosInSequence);
            }
            endPosInSequence = (endPosInSequence as u32)
                .wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength))
                as u32 as u32;
            startPosInSequence = 0;
        } else {
            if !(endPosInSequence > litLength) {
                break;
            }
            let mut firstHalfMatchLength: u32 = 0;
            litLength = if startPosInSequence >= litLength {
                0_u32
            } else {
                litLength.wrapping_sub(startPosInSequence)
            };
            firstHalfMatchLength = endPosInSequence
                .wrapping_sub(startPosInSequence)
                .wrapping_sub(litLength);
            if matchLength as usize > blockSize
                && firstHalfMatchLength >= (*cctx).appliedParams.cParams.minMatch
            {
                let mut secondHalfMatchLength = (currSeq.matchLength)
                    .wrapping_add(currSeq.litLength)
                    .wrapping_sub(endPosInSequence);
                if secondHalfMatchLength < (*cctx).appliedParams.cParams.minMatch {
                    endPosInSequence = (endPosInSequence as u32)
                        .wrapping_sub(
                            ((*cctx).appliedParams.cParams.minMatch)
                                .wrapping_sub(secondHalfMatchLength),
                        ) as u32 as u32;
                    bytesAdjustment = ((*cctx).appliedParams.cParams.minMatch)
                        .wrapping_sub(secondHalfMatchLength);
                    firstHalfMatchLength = firstHalfMatchLength
                        .wrapping_sub(bytesAdjustment);
                }
                matchLength = firstHalfMatchLength;
                finalMatchSplit = 1;
            } else {
                bytesAdjustment = endPosInSequence.wrapping_sub(currSeq.litLength);
                endPosInSequence = currSeq.litLength;
                break;
            }
        }
        let ll0 = (litLength == 0) as i32 as u32;
        offBase = ZSTD_finalizeOffBase(
            rawOffset,
            (updatedRepcodes.rep).as_mut_ptr() as *const u32,
            ll0,
        );
        ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        if (*cctx).appliedParams.validateSequences != 0 {
            (*seqPos)
                .posInSrc = ((*seqPos).posInSrc)
                .wrapping_add(litLength.wrapping_add(matchLength) as usize);
            FORWARD_IF_ERROR!(
                ZSTD_validateSequence(offBase, matchLength, (*cctx).appliedParams.cParams
                .minMatch, (*seqPos).posInSrc, (*cctx).appliedParams.cParams.windowLog,
                dictSize, ZSTD_hasExtSeqProd(addr_of!((*cctx).appliedParams))),
                "Sequence validation failed"
            );
        }
        RETURN_ERROR_IF!(idx.wrapping_sub((*seqPos).idx) as usize >= (*cctx).seqStore.maxNbSeq, ZSTD_error_externalSequences_invalid);
        ZSTD_storeSeq(
            &mut (*cctx).seqStore,
            litLength as usize,
            ip,
            iend,
            offBase,
            matchLength as usize,
        );
        ip = ip.offset(matchLength.wrapping_add(litLength) as isize);
        if finalMatchSplit == 0 {
            idx = idx.wrapping_add(1);
            idx;
        }
    }
    (*seqPos).idx = idx;
    (*seqPos).posInSequence = endPosInSequence;
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut std::ffi::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>() as usize,
    );
    iend = iend.offset(-(bytesAdjustment as isize));
    if ip != iend {
        let lastLLSize = iend.offset_from(ip) as std::ffi::c_long as u32;
        ZSTD_storeLastLiterals(&mut (*cctx).seqStore, ip, lastLLSize as usize);
        (*seqPos).posInSrc = ((*seqPos).posInSrc).wrapping_add(lastLLSize as usize);
    }
    return iend.offset_from(istart) as std::ffi::c_long as usize;
}
unsafe fn ZSTD_selectSequenceCopier(
    mut mode: ZSTD_SequenceFormat_e,
) -> ZSTD_SequenceCopier_f {
    if mode as u32
        == ZSTD_sf_explicitBlockDelimiters as i32 as u32
    {
        return Some(
            ZSTD_transferSequences_wBlockDelim
                as unsafe extern "C" fn(
                    *mut ZSTD_CCtx,
                    *mut ZSTD_SequencePosition,
                    *const ZSTD_Sequence,
                    usize,
                    *const std::ffi::c_void,
                    usize,
                    ZSTD_ParamSwitch_e,
                ) -> usize,
        );
    }
    return Some(
        ZSTD_transferSequences_noDelim
            as unsafe extern "C" fn(
                *mut ZSTD_CCtx,
                *mut ZSTD_SequencePosition,
                *const ZSTD_Sequence,
                usize,
                *const std::ffi::c_void,
                usize,
                ZSTD_ParamSwitch_e,
            ) -> usize,
    );
}
unsafe fn blockSize_explicitDelimiter(
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut seqPos: ZSTD_SequencePosition,
) -> usize {
    let mut end: i32 = 0;
    let mut blockSize: usize = 0;
    let mut spos = seqPos.idx as usize;
    while spos < inSeqsSize {
        end = ((*inSeqs.offset(spos as isize)).offset
            == 0) as i32;
        blockSize = blockSize
            .wrapping_add(
                ((*inSeqs.offset(spos as isize)).litLength)
                    .wrapping_add((*inSeqs.offset(spos as isize)).matchLength) as usize,
            );
        if end != 0 {
            if (*inSeqs.offset(spos as isize)).matchLength
                != 0
            {
                return -(ZSTD_error_externalSequences_invalid as i32)
                    as usize;
            }
            break;
        } else {
            spos = spos.wrapping_add(1);
            spos;
        }
    }
    RETURN_ERROR_IF!(end == 0, ZSTD_error_externalSequences_invalid);
    return blockSize;
}
unsafe fn determine_blockSize(
    mut mode: ZSTD_SequenceFormat_e,
    mut blockSize: usize,
    mut remaining: usize,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut seqPos: ZSTD_SequencePosition,
) -> usize {
    if mode as u32
        == ZSTD_sf_noBlockDelimiters as i32 as u32
    {
        return std::cmp::min(remaining, blockSize);
    }
    let explicitBlockSize = blockSize_explicitDelimiter(inSeqs, inSeqsSize, seqPos);
    FORWARD_IF_ERROR!(
        explicitBlockSize, "Error while determining block size with explicit delimiters"
    );
    RETURN_ERROR_IF!(explicitBlockSize > blockSize, ZSTD_error_externalSequences_invalid);
    RETURN_ERROR_IF!(explicitBlockSize > remaining, ZSTD_error_externalSequences_invalid);
    return explicitBlockSize;
}
unsafe fn ZSTD_compressSequences_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut cSize: usize = 0;
    let mut remaining = srcSize;
    let mut seqPos = {
        let mut init = ZSTD_SequencePosition {
            idx: 0,
            posInSequence: 0,
            posInSrc: 0,
        };
        init
    };
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let sequenceCopier = ZSTD_selectSequenceCopier(
        (*cctx).appliedParams.blockDelimiters,
    );
    if remaining == 0 {
        let cBlockHeader24 = 1_u32
            .wrapping_add((bt_raw as i32 as u32) << 1);
        RETURN_ERROR_IF!(dstCapacity < 4, ZSTD_error_dstSize_tooSmall);
        MEM_writeLE32(op as *mut std::ffi::c_void, cBlockHeader24);
        op = op.offset(ZSTD_blockHeaderSize as isize);
        dstCapacity = dstCapacity.wrapping_sub(ZSTD_blockHeaderSize);
        cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
    }
    while remaining != 0 {
        let mut compressedSeqsSize: usize = 0;
        let mut cBlockSize: usize = 0;
        let mut blockSize = determine_blockSize(
            (*cctx).appliedParams.blockDelimiters,
            (*cctx).blockSizeMax,
            remaining,
            inSeqs,
            inSeqsSize,
            seqPos,
        );
        let lastBlock = (blockSize == remaining) as i32 as u32;
        FORWARD_IF_ERROR!(
            blockSize, "Error while trying to determine block size"
        );
        ZSTD_resetSeqStore(&mut (*cctx).seqStore);
        blockSize = sequenceCopier
            .expect(
                "non-null function pointer",
            )(
            cctx,
            &mut seqPos,
            inSeqs,
            inSeqsSize,
            ip as *const std::ffi::c_void,
            blockSize,
            (*cctx).appliedParams.searchForExternalRepcodes,
        );
        FORWARD_IF_ERROR!(blockSize, "Bad sequence copy");
        if blockSize
            < (MIN_CBLOCK_SIZE as usize)
                .wrapping_add(ZSTD_blockHeaderSize)
                .wrapping_add(1)
                .wrapping_add(1)
        {
            cBlockSize = ZSTD_noCompressBlock(
                op as *mut std::ffi::c_void,
                dstCapacity,
                ip as *const std::ffi::c_void,
                blockSize,
                lastBlock,
            );
            FORWARD_IF_ERROR!(cBlockSize, "Nocompress block failed");
            cSize = cSize.wrapping_add(cBlockSize);
            ip = ip.offset(blockSize as isize);
            op = op.offset(cBlockSize as isize);
            remaining = remaining.wrapping_sub(blockSize);
            dstCapacity = dstCapacity.wrapping_sub(cBlockSize);
        } else {
            RETURN_ERROR_IF!(dstCapacity < ZSTD_blockHeaderSize, ZSTD_error_dstSize_tooSmall);
            compressedSeqsSize = ZSTD_entropyCompressSeqStore(
                &mut (*cctx).seqStore,
                &mut (*(*cctx).blockState.prevCBlock).entropy,
                &mut (*(*cctx).blockState.nextCBlock).entropy,
                &mut (*cctx).appliedParams,
                op.offset(ZSTD_blockHeaderSize as isize) as *mut std::ffi::c_void,
                dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                blockSize,
                (*cctx).tmpWorkspace,
                (*cctx).tmpWkspSize,
                (*cctx).bmi2,
            );
            FORWARD_IF_ERROR!(
                compressedSeqsSize, "Compressing sequences of block failed"
            );
            if (*cctx).isFirstBlock == 0 && ZSTD_maybeRLE(&mut (*cctx).seqStore) != 0
                && ZSTD_isRLE(ip, blockSize) != 0
            {
                compressedSeqsSize = 1;
            }
            if compressedSeqsSize == 0 {
                cBlockSize = ZSTD_noCompressBlock(
                    op as *mut std::ffi::c_void,
                    dstCapacity,
                    ip as *const std::ffi::c_void,
                    blockSize,
                    lastBlock,
                );
                FORWARD_IF_ERROR!(
                    cBlockSize, "ZSTD_noCompressBlock failed"
                );
            } else if compressedSeqsSize == 1 {
                cBlockSize = ZSTD_rleCompressBlock(
                    op as *mut std::ffi::c_void,
                    dstCapacity,
                    *ip,
                    blockSize,
                    lastBlock,
                );
                FORWARD_IF_ERROR!(
                    cBlockSize, "ZSTD_rleCompressBlock failed"
                );
            } else {
                let mut cBlockHeader: u32 = 0;
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
                if (*(*cctx).blockState.prevCBlock).entropy.fse.offcode_repeatMode
                    as u32
                    == FSE_repeat_valid as i32 as u32
                {
                    (*(*cctx).blockState.prevCBlock)
                        .entropy
                        .fse
                        .offcode_repeatMode = FSE_repeat_check;
                }
                cBlockHeader = lastBlock
                    .wrapping_add(
                        (bt_compressed as i32 as u32) << 1,
                    )
                    .wrapping_add((compressedSeqsSize << 3) as u32);
                MEM_writeLE24(op as *mut std::ffi::c_void, cBlockHeader);
                cBlockSize = ZSTD_blockHeaderSize.wrapping_add(compressedSeqsSize);
            }
            cSize = cSize.wrapping_add(cBlockSize);
            if lastBlock != 0 {
                break;
            }
            ip = ip.offset(blockSize as isize);
            op = op.offset(cBlockSize as isize);
            remaining = remaining.wrapping_sub(blockSize);
            dstCapacity = dstCapacity.wrapping_sub(cBlockSize);
            (*cctx).isFirstBlock = 0;
        }
    }
    return cSize;
}
#[no_mangle]
pub unsafe fn ZSTD_compressSequences(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut op = dst as *mut u8;
    let mut cSize: usize = 0;
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, srcSize),
        "CCtx initialization failed"
    );
    let frameHeaderSize = ZSTD_writeFrameHeader(
        op as *mut std::ffi::c_void,
        dstCapacity,
        &mut (*cctx).appliedParams,
        srcSize,
        (*cctx).dictID,
    );
    op = op.offset(frameHeaderSize as isize);
    dstCapacity = dstCapacity.wrapping_sub(frameHeaderSize);
    cSize = cSize.wrapping_add(frameHeaderSize);
    if (*cctx).appliedParams.fParams.checksumFlag != 0 && srcSize != 0 {
        ZSTD_XXH64_update(&mut (*cctx).xxhState, src, srcSize);
    }
    let cBlocksSize = ZSTD_compressSequences_internal(
        cctx,
        op as *mut std::ffi::c_void,
        dstCapacity,
        inSeqs,
        inSeqsSize,
        src,
        srcSize,
    );
    FORWARD_IF_ERROR!(cBlocksSize, "Compressing blocks failed!");
    cSize = cSize.wrapping_add(cBlocksSize);
    dstCapacity = dstCapacity.wrapping_sub(cBlocksSize);
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        let checksum = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as u32;
        RETURN_ERROR_IF!(dstCapacity < 4, ZSTD_error_dstSize_tooSmall);
        MEM_writeLE32(
            (dst as *mut std::ffi::c_char).offset(cSize as isize)
                as *mut std::ffi::c_void,
            checksum,
        );
        cSize = cSize.wrapping_add(4);
    }
    return cSize;
}
unsafe fn convertSequences_noRepcodes(
    mut dstSeqs: *mut SeqDef,
    mut inSeqs: *const ZSTD_Sequence,
    mut nbSequences: usize,
) -> usize {
    let mut longLen: usize = 0;
    let mut n: usize = 0;
    n = 0;
    while n < nbSequences {
        (*dstSeqs.offset(n as isize)).offBase = OFFSET_TO_OFFBASE!(inSeqs[n].offset);
        (*dstSeqs.offset(n as isize))
            .litLength = (*inSeqs.offset(n as isize)).litLength as u16;
        (*dstSeqs.offset(n as isize))
            .mlBase = ((*inSeqs.offset(n as isize)).matchLength)
            .wrapping_sub(MINMATCH as u32) as u16;
        if UNLIKELY!((*inSeqs.offset(n as isize)).matchLength > 65535 + MINMATCH) != 0 {
            longLen = n.wrapping_add(1);
        }
        if UNLIKELY!((*inSeqs.offset(n as isize)).litLength > 65535) != 0 {
            longLen = n
                .wrapping_add(nbSequences)
                .wrapping_add(1);
        }
        n = n.wrapping_add(1);
        n;
    }
    return longLen;
}

/*
 * Precondition: Sequences must end on an explicit Block Delimiter
 * @return: 0 on success, or an error code.
 * Note: Sequence validation functionality has been disabled (removed).
 * This is helpful to generate a lean main pipeline, improving performance.
 * It may be re-inserted later.
 */
pub unsafe fn ZSTD_convertBlockSequences(
    mut cctx: *mut ZSTD_CCtx,
    inSeqs: *const ZSTD_Sequence,
    mut nbSequences: usize,
    mut repcodeResolution: i32,
) -> usize {
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };

    DEBUGLOG!(5, "ZSTD_convertBlockSequences (nbSequences = %zu)", nbSequences);

    RETURN_ERROR_IF!(nbSequences >= (*cctx).seqStore.maxNbSeq, ZSTD_error_externalSequences_invalid,
        "Not enough memory allocated. Try adjusting ZSTD_c_minMatch.");

    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut std::ffi::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>(),
    );

    /* check end condition */
    debug_assert!(nbSequences >= 1);
    debug_assert!((*inSeqs.add(nbSequences).sub(1)).matchLength == 0);
    debug_assert!((*inSeqs.add(nbSequences).sub(1)).offset == 0);

    /* Convert Sequences from public format to internal format */
    if repcodeResolution == 0 {
        let longl = convertSequences_noRepcodes(
            (*cctx).seqStore.sequencesStart,
            inSeqs,
            nbSequences.wrapping_sub(1),
        );
        (*cctx)
            .seqStore
            .sequences = ((*cctx).seqStore.sequencesStart)
            .add(nbSequences)
            .sub(1);
        if longl != 0 {
            DEBUGLOG!(5, "long length");
            debug_assert!((*cctx).seqStore.longLengthType == ZSTD_llt_none);
            if longl <= nbSequences.wrapping_sub(1) {
                DEBUGLOG!(5, "long match length detected at pos %zu", longl-1);
                (*cctx).seqStore.longLengthType = ZSTD_llt_matchLength;
                (*cctx)
                    .seqStore
                    .longLengthPos = longl.wrapping_sub(1) as u32;
            } else {
                DEBUGLOG!(5, "long literals length detected at pos %zu", longl-nbSequences);
                (*cctx).seqStore.longLengthType = ZSTD_llt_literalLength;
                (*cctx)
                    .seqStore
                    .longLengthPos = longl
                    .wrapping_sub(
                        nbSequences.wrapping_sub(1),
                    )
                    .wrapping_sub(1) as u32;
            }
        }
    } else {
        for seqNb in 0..(nbSequences - 1) {
            let litLength = (*inSeqs.add(seqNb)).litLength;
            let matchLength = (*inSeqs.add(seqNb)).matchLength;
            let ll0 = (litLength == 0) as u32;
            let offBase = ZSTD_finalizeOffBase(
                (*inSeqs.add(seqNb)).offset,
                (updatedRepcodes.rep).as_mut_ptr() as *const u32,
                ll0,
            );
            
            DEBUGLOG!(6, "Storing sequence: (of: %u, ml: %u, ll: %u)", offBase, matchLength, litLength);
            ZSTD_storeSeqOnly(
                &mut (*cctx).seqStore,
                litLength as usize,
                offBase,
                matchLength as usize,
            );
            ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        }
    }

    /* If we skipped repcode search while parsing, we need to update repcodes now */
    if repcodeResolution == 0 && nbSequences > 1 {
        let rep = (updatedRepcodes.rep).as_mut_ptr();
        if nbSequences >= 4 {
            let lastSeqIdx = nbSequences - 2; /* index of last full sequence */
            *rep
                .offset(
                    2,
                ) = (*inSeqs
                .add(lastSeqIdx - 2))
                .offset;
            *rep
                .offset(
                    1,
                ) = (*inSeqs
                .add(lastSeqIdx - 1))
                .offset;
            *rep
                .offset(
                    0,
                ) = (*inSeqs.add(lastSeqIdx)).offset;
        } else if nbSequences == 3 {
            *rep
                .offset(
                    2,
                ) = *rep.offset(0);
            *rep
                .offset(
                    1,
                ) = (*inSeqs.offset(0)).offset;
            *rep
                .offset(
                    0,
                ) = (*inSeqs.offset(1)).offset;
        } else {
            debug_assert!(nbSequences == 2);
            *rep
                .offset(
                    2,
                ) = *rep.offset(1);
            *rep
                .offset(
                    1,
                ) = *rep.offset(0);
            *rep
                .offset(
                    0,
                ) = (*inSeqs.offset(0)).offset;
        }
    }

    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut std::ffi::c_void,
        (updatedRepcodes.rep).as_ptr() as *const std::ffi::c_void,
        size_of::<Repcodes_t>(),
    );

    return 0;
}

pub unsafe fn ZSTD_get1BlockSummary(
    mut seqs: *const ZSTD_Sequence,
    mut nbSeqs: usize,
) -> BlockSummary {
    // FIXME: C implementation has SIMD impls for this function 
    let mut totalMatchSize: usize = 0;
    let mut litSize: usize = 0;
    let mut n: usize = 0;
    debug_assert!(!seqs.is_null());
    while n < nbSeqs {
        totalMatchSize += (*seqs.add(n)).matchLength;
        litSize += (*seqs.add(n)).litLength;
        if (*seqs.add(n)).matchLength == 0 {
            debug_assert!((*seqs.add(n)).offset == 0);
            break;
        }
        n += 1;
    }
    if n == nbSeqs {
        return BlockSummary {
            nbSequences: ERROR(ZSTD_error_externalSequences_invalid),
            blockSize: 0,
            litSize: 0,
        };
    }
    BlockSummary {
        nbSequences: n + 1,
        blockSize: litSize + totalMatchSize,
        litSize,
    }
}

unsafe fn ZSTD_compressSequencesAndLiterals_internal(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut inSeqs: *const ZSTD_Sequence,
    mut nbSequences: usize,
    mut literals: *const std::ffi::c_void,
    mut litSize: usize,
    mut srcSize: usize,
) -> usize {
    let mut remaining = srcSize;
    let mut cSize: usize = 0;
    let mut op = dst as *mut u8;
    let repcodeResolution = ((*cctx).appliedParams.searchForExternalRepcodes
        as u32 == ZSTD_ps_enable as i32 as u32)
        as i32;
    RETURN_ERROR_IF!(nbSequences == 0, ZSTD_error_externalSequences_invalid);
    if nbSequences == 1
        && (*inSeqs.offset(0)).litLength
            == 0
    {
        let cBlockHeader24 = 1_u32
            .wrapping_add((bt_raw as i32 as u32) << 1);
        RETURN_ERROR_IF!(dstCapacity < 3, ZSTD_error_dstSize_tooSmall);
        MEM_writeLE24(op as *mut std::ffi::c_void, cBlockHeader24);
        op = op.offset(ZSTD_blockHeaderSize as isize);
        dstCapacity = dstCapacity.wrapping_sub(ZSTD_blockHeaderSize);
        cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
    }
    while nbSequences != 0 {
        let mut compressedSeqsSize: usize = 0;
        let mut cBlockSize: usize = 0;
        let mut conversionStatus: usize = 0;
        let block = ZSTD_get1BlockSummary(inSeqs, nbSequences);
        let lastBlock = (block.nbSequences == nbSequences) as i32 as u32;
        FORWARD_IF_ERROR!(
            block.nbSequences,
            "Error while trying to determine nb of sequences for a block"
        );
        RETURN_ERROR_IF!(block.litSize > litSize, ZSTD_error_externalSequences_invalid);
        ZSTD_resetSeqStore(&mut (*cctx).seqStore);
        conversionStatus = ZSTD_convertBlockSequences(
            cctx,
            inSeqs,
            block.nbSequences,
            repcodeResolution,
        );
        FORWARD_IF_ERROR!(conversionStatus, "Bad sequence conversion");
        inSeqs = inSeqs.offset(block.nbSequences as isize);
        nbSequences = nbSequences.wrapping_sub(block.nbSequences);
        remaining = remaining.wrapping_sub(block.blockSize);
        RETURN_ERROR_IF!(dstCapacity < ZSTD_blockHeaderSize, ZSTD_error_dstSize_tooSmall);
        compressedSeqsSize = ZSTD_entropyCompressSeqStore_internal(
            op.offset(ZSTD_blockHeaderSize as isize) as *mut std::ffi::c_void,
            dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
            literals,
            block.litSize,
            &mut (*cctx).seqStore,
            &mut (*(*cctx).blockState.prevCBlock).entropy,
            &mut (*(*cctx).blockState.nextCBlock).entropy,
            &mut (*cctx).appliedParams,
            (*cctx).tmpWorkspace,
            (*cctx).tmpWkspSize,
            (*cctx).bmi2,
        );
        FORWARD_IF_ERROR!(
            compressedSeqsSize, "Compressing sequences of block failed"
        );
        if compressedSeqsSize > (*cctx).blockSizeMax {
            compressedSeqsSize = 0;
        }
        litSize = litSize.wrapping_sub(block.litSize);
        literals = (literals as *const std::ffi::c_char).offset(block.litSize as isize)
            as *const std::ffi::c_void;
        if compressedSeqsSize == 0 {
            return -(ZSTD_error_cannotProduce_uncompressedBlock as i32)
                as usize
        } else {
            let mut cBlockHeader: u32 = 0;
            ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
            if (*(*cctx).blockState.prevCBlock).entropy.fse.offcode_repeatMode
                as u32
                == FSE_repeat_valid as i32 as u32
            {
                (*(*cctx).blockState.prevCBlock)
                    .entropy
                    .fse
                    .offcode_repeatMode = FSE_repeat_check;
            }
            cBlockHeader = lastBlock
                .wrapping_add(
                    (bt_compressed as i32 as u32) << 1,
                )
                .wrapping_add((compressedSeqsSize << 3) as u32);
            MEM_writeLE24(op as *mut std::ffi::c_void, cBlockHeader);
            cBlockSize = ZSTD_blockHeaderSize.wrapping_add(compressedSeqsSize);
        }
        cSize = cSize.wrapping_add(cBlockSize);
        op = op.offset(cBlockSize as isize);
        dstCapacity = dstCapacity.wrapping_sub(cBlockSize);
        (*cctx).isFirstBlock = 0;
        if lastBlock != 0 {
            break;
        }
    }
    RETURN_ERROR_IF!(litSize != 0, ZSTD_error_externalSequences_invalid);
    RETURN_ERROR_IF!(remaining != 0, ZSTD_error_externalSequences_invalid);
    return cSize;
}
#[no_mangle]
pub unsafe fn ZSTD_compressSequencesAndLiterals(
    mut cctx: *mut ZSTD_CCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut inSeqs: *const ZSTD_Sequence,
    mut inSeqsSize: usize,
    mut literals: *const std::ffi::c_void,
    mut litSize: usize,
    mut litCapacity: usize,
    mut decompressedSize: usize,
) -> usize {
    let mut op = dst as *mut u8;
    let mut cSize: usize = 0;
    RETURN_ERROR_IF!(litCapacity < litSize, ZSTD_error_workSpace_tooSmall);
    FORWARD_IF_ERROR!(
        ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, decompressedSize),
        "CCtx initialization failed"
    );
    RETURN_ERROR_IF!((*cctx).appliedParams.blockDelimiters as u32
        == ZSTD_sf_noBlockDelimiters as i32 as u32, ZSTD_error_frameParameter_unsupported);
    RETURN_ERROR_IF!((*cctx).appliedParams.validateSequences != 0, ZSTD_error_parameter_unsupported);
    RETURN_ERROR_IF!((*cctx).appliedParams.fParams.checksumFlag != 0, ZSTD_error_frameParameter_unsupported);
    let frameHeaderSize = ZSTD_writeFrameHeader(
        op as *mut std::ffi::c_void,
        dstCapacity,
        &mut (*cctx).appliedParams,
        decompressedSize,
        (*cctx).dictID,
    );
    op = op.offset(frameHeaderSize as isize);
    dstCapacity = dstCapacity.wrapping_sub(frameHeaderSize);
    cSize = cSize.wrapping_add(frameHeaderSize);
    let cBlocksSize = ZSTD_compressSequencesAndLiterals_internal(
        cctx,
        op as *mut std::ffi::c_void,
        dstCapacity,
        inSeqs,
        inSeqsSize,
        literals,
        litSize,
        decompressedSize,
    );
    FORWARD_IF_ERROR!(cBlocksSize, "Compressing blocks failed!");
    cSize = cSize.wrapping_add(cBlocksSize);
    dstCapacity = dstCapacity.wrapping_sub(cBlocksSize);
    return cSize;
}
unsafe fn inBuffer_forEndFlush(
    mut zcs: *const ZSTD_CStream,
) -> ZSTD_inBuffer {
    let nullInput = {
        let mut init = ZSTD_inBuffer_s {
            src: std::ptr::null(),
            size: 0,
            pos: 0,
        };
        init
    };
    let stableInput = ((*zcs).appliedParams.inBufferMode as u32
        == ZSTD_bm_stable as i32 as u32) as i32;
    return if stableInput != 0 { (*zcs).expectedInBuffer } else { nullInput };
}
#[no_mangle]
pub unsafe fn ZSTD_flushStream(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
) -> usize {
    let mut input = inBuffer_forEndFlush(zcs);
    input.size = input.pos;
    return ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_flush);
}
#[no_mangle]
pub unsafe fn ZSTD_endStream(
    mut zcs: *mut ZSTD_CStream,
    mut output: *mut ZSTD_outBuffer,
) -> usize {
    let mut input = inBuffer_forEndFlush(zcs);
    let remainingToFlush = ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_end);
    FORWARD_IF_ERROR!(
        remainingToFlush, "ZSTD_compressStream2(,,ZSTD_e_end) failed"
    );
    if (*zcs).appliedParams.nbWorkers > 0 {
        return remainingToFlush;
    }
    let lastBlockSize = (if (*zcs).frameEnded != 0 {
        0 as i32
    } else {
        ZSTD_BLOCKHEADERSIZE
    }) as usize;
    let checksumSize = (if (*zcs).frameEnded != 0 {
        0 as i32
    } else {
        (*zcs).appliedParams.fParams.checksumFlag * 4 as i32
    }) as usize;
    let toFlush = remainingToFlush
        .wrapping_add(lastBlockSize)
        .wrapping_add(checksumSize);
    return toFlush;
}
pub const ZSTD_MAX_CLEVEL: i32 = 22;
static mut ZSTD_defaultCParameters: [[ZSTD_compressionParameters; 23]; 4] = [
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 19,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 6,
                targetLength: 1,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 19,
                chainLog: 13,
                hashLog: 14,
                searchLog: 1,
                minMatch: 7,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 20,
                chainLog: 15,
                hashLog: 16,
                searchLog: 1,
                minMatch: 6,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 16,
                hashLog: 17,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 18,
                hashLog: 18,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 18,
                hashLog: 19,
                searchLog: 3,
                minMatch: 5,
                targetLength: 2,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 18,
                hashLog: 19,
                searchLog: 3,
                minMatch: 5,
                targetLength: 4,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 19,
                hashLog: 20,
                searchLog: 4,
                minMatch: 5,
                targetLength: 8,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 19,
                hashLog: 20,
                searchLog: 4,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 20,
                hashLog: 21,
                searchLog: 4,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 21,
                hashLog: 22,
                searchLog: 5,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 21,
                hashLog: 22,
                searchLog: 6,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 23,
                searchLog: 6,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 22,
                searchLog: 4,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 23,
                searchLog: 5,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 23,
                hashLog: 23,
                searchLog: 6,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 22,
                searchLog: 5,
                minMatch: 5,
                targetLength: 48,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 23,
                chainLog: 23,
                hashLog: 22,
                searchLog: 5,
                minMatch: 4,
                targetLength: 64,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 23,
                chainLog: 23,
                hashLog: 22,
                searchLog: 6,
                minMatch: 3,
                targetLength: 64,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 23,
                chainLog: 24,
                hashLog: 22,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 25,
                chainLog: 25,
                hashLog: 23,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 26,
                chainLog: 26,
                hashLog: 24,
                searchLog: 7,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 27,
                chainLog: 27,
                hashLog: 25,
                searchLog: 9,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 5,
                targetLength: 1,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 13,
                hashLog: 14,
                searchLog: 1,
                minMatch: 6,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 14,
                hashLog: 14,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 16,
                hashLog: 16,
                searchLog: 1,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 5,
                targetLength: 2,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 17,
                hashLog: 18,
                searchLog: 5,
                minMatch: 5,
                targetLength: 2,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 3,
                minMatch: 5,
                targetLength: 4,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 4,
                targetLength: 4,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 6,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 5,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 7,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 4,
                targetLength: 16,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 3,
                targetLength: 32,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 10,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 12,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 13,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 12,
                hashLog: 12,
                searchLog: 1,
                minMatch: 5,
                targetLength: 1,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 6,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 13,
                hashLog: 15,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 15,
                hashLog: 16,
                searchLog: 2,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 17,
                hashLog: 17,
                searchLog: 2,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 2,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 4,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 4,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 6,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 17,
                hashLog: 17,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 7,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 4,
                minMatch: 3,
                targetLength: 32,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 6,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 10,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 5,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 7,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 9,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 11,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
    [
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 5,
                targetLength: 1,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 15,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 15,
                searchLog: 1,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_fast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 15,
                searchLog: 2,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_dfast,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 4,
                minMatch: 4,
                targetLength: 2,
                strategy: ZSTD_greedy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 3,
                minMatch: 4,
                targetLength: 4,
                strategy: ZSTD_lazy,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 4,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 6,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 8,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 9,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_btlazy2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 3,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 4,
                minMatch: 3,
                targetLength: 24,
                strategy: ZSTD_btopt,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 5,
                minMatch: 3,
                targetLength: 32,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 6,
                minMatch: 3,
                targetLength: 64,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 5,
                minMatch: 3,
                targetLength: 48,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 8,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 9,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            };
            init
        },
        {
            let mut init = ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 10,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            };
            init
        },
    ],
];

unsafe fn ZSTD_dedicatedDictSearch_getCParams(
    compressionLevel: i32,
    dictSize: usize,
) -> ZSTD_compressionParameters {
    let mut cParams = ZSTD_getCParams_internal(
        compressionLevel,
        0,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    match cParams.strategy as u32 {
        3 | 4 | 5 => {
            cParams
                .hashLog = (cParams.hashLog)
                .wrapping_add(ZSTD_LAZY_DDSS_BUCKET_LOG as u32);
        }
        1 | 2 | 6 | 7 | 8 | 9 | _ => {}
    }
    return cParams;
}
unsafe fn ZSTD_dedicatedDictSearch_isSupported(
    mut cParams: *const ZSTD_compressionParameters,
) -> i32 {
    return ((*cParams).strategy as u32
        >= ZSTD_greedy as i32 as u32
        && (*cParams).strategy as u32
            <= ZSTD_lazy2 as i32 as u32
        && (*cParams).hashLog > (*cParams).chainLog
        && (*cParams).chainLog <= 24)
        as i32;
}
unsafe fn ZSTD_dedicatedDictSearch_revertCParams(
    mut cParams: *mut ZSTD_compressionParameters,
) {
    match (*cParams).strategy as u32 {
        3 | 4 | 5 => {
            (*cParams)
                .hashLog = ((*cParams).hashLog)
                .wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as u32);
            if (*cParams).hashLog < ZSTD_HASHLOG_MIN as u32 {
                (*cParams).hashLog = ZSTD_HASHLOG_MIN as u32;
            }
        }
        1 | 2 | 6 | 7 | 8 | 9 | _ => {}
    };
}
unsafe fn ZSTD_getCParamRowSize(
    mut srcSizeHint: u64,
    mut dictSize: usize,
    mut mode: ZSTD_CParamMode_e,
) -> u64 {
    match mode as u32 {
        1 => {
            dictSize = 0;
        }
        3 | 0 | 2 | _ => {}
    }
    let unknown = (srcSizeHint as u64 == ZSTD_CONTENTSIZE_UNKNOWN)
        as i32;
    let addedSize = (if unknown != 0 && dictSize > 0 {
        500 as i32
    } else {
        0 as i32
    }) as usize;
    return (if unknown != 0 && dictSize == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        srcSizeHint.wrapping_add(dictSize).wrapping_add(addedSize)
            as u64
    }) as u64;
}
unsafe fn ZSTD_getCParams_internal(
    mut compressionLevel: i32,
    mut srcSizeHint: u64,
    mut dictSize: usize,
    mut mode: ZSTD_CParamMode_e,
) -> ZSTD_compressionParameters {
    let rSize = ZSTD_getCParamRowSize(srcSizeHint as u64, dictSize, mode);
    let tableID = ((rSize
        <= (256 as i32 * ((1 as i32) << 10))
            as u64) as i32
        + (rSize
            <= (128 as i32
                * ((1 as i32) << 10)) as u64)
            as i32
        + (rSize
            <= (16 as i32
                * ((1 as i32) << 10)) as u64)
            as i32) as u32;
    let mut row: i32 = 0;
    if compressionLevel == 0 {
        row = ZSTD_CLEVEL_DEFAULT;
    } else if compressionLevel < 0 {
        row = 0;
    } else if compressionLevel > ZSTD_MAX_CLEVEL {
        row = ZSTD_MAX_CLEVEL;
    } else {
        row = compressionLevel;
    }
    let mut cp = ZSTD_defaultCParameters[tableID as usize][row as usize];
    if compressionLevel < 0 {
        let clampedCompressionLevel = std::cmp::max(ZSTD_minCLevel(), compressionLevel);
        cp.targetLength = -clampedCompressionLevel as u32;
    }
    return ZSTD_adjustCParams_internal(cp, srcSizeHint, dictSize, mode, ZSTD_ps_auto);
}
#[no_mangle]
pub unsafe fn ZSTD_getCParams(
    mut compressionLevel: i32,
    mut srcSizeHint: u64,
    mut dictSize: usize,
) -> ZSTD_compressionParameters {
    if srcSizeHint == 0 {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    return ZSTD_getCParams_internal(
        compressionLevel,
        srcSizeHint,
        dictSize,
        ZSTD_cpm_unknown,
    );
}
unsafe fn ZSTD_getParams_internal(
    mut compressionLevel: i32,
    mut srcSizeHint: u64,
    mut dictSize: usize,
    mut mode: ZSTD_CParamMode_e,
) -> ZSTD_parameters {
    let mut params = ZSTD_parameters {
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
    };
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        srcSizeHint,
        dictSize,
        mode,
    );
    libc::memset(
        &mut params as *mut ZSTD_parameters as *mut std::ffi::c_void,
        0,
        size_of::<ZSTD_parameters>() as usize,
    );
    params.cParams = cParams;
    params.fParams.contentSizeFlag = 1;
    return params;
}
#[no_mangle]
pub unsafe fn ZSTD_getParams(
    mut compressionLevel: i32,
    mut srcSizeHint: u64,
    mut dictSize: usize,
) -> ZSTD_parameters {
    if srcSizeHint == 0 {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    return ZSTD_getParams_internal(
        compressionLevel,
        srcSizeHint,
        dictSize,
        ZSTD_cpm_unknown,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_registerSequenceProducer(
    mut zc: *mut ZSTD_CCtx,
    mut extSeqProdState: *mut std::ffi::c_void,
    mut extSeqProdFunc: ZSTD_sequenceProducer_F,
) {
    ZSTD_CCtxParams_registerSequenceProducer(
        &mut (*zc).requestedParams,
        extSeqProdState,
        extSeqProdFunc,
    );
}
#[no_mangle]
pub unsafe fn ZSTD_CCtxParams_registerSequenceProducer(
    mut params: *mut ZSTD_CCtx_params,
    mut extSeqProdState: *mut std::ffi::c_void,
    mut extSeqProdFunc: ZSTD_sequenceProducer_F,
) {
    if extSeqProdFunc.is_some() {
        (*params).extSeqProdFunc = extSeqProdFunc;
        (*params).extSeqProdState = extSeqProdState;
    } else {
        (*params)
            .extSeqProdFunc = ::core::mem::transmute::<
            libc::intptr_t,
            ZSTD_sequenceProducer_F,
        >(NULL as libc::intptr_t);
        (*params).extSeqProdState = std::ptr::null_mut();
    };
}
pub const __INT_MAX__: i32 = 2147483647;
unsafe fn run_static_initializers() {
    srcSizeTiers = [
        (16 as i32 * ((1 as i32) << 10))
            as u64,
        (128 as i32 * ((1 as i32) << 10))
            as u64,
        (256 as i32 * ((1 as i32) << 10))
            as u64,
        ZSTD_CONTENTSIZE_UNKNOWN,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
