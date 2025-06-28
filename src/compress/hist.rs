use ::libc;
pub type unalign32 = u32;
use crate::common::error::*;
pub type HIST_checkInput_e = std::ffi::c_uint;
pub const checkMaxSymbolValue: HIST_checkInput_e = 1;
pub const trustInput: HIST_checkInput_e = 0;
use crate::common::mem::*;
pub const HIST_WKSP_SIZE_U32: std::ffi::c_int = 1024;
pub const HIST_WKSP_SIZE: std::ffi::c_ulong = (HIST_WKSP_SIZE_U32 as std::ffi::c_ulong)
    .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>());
#[no_mangle]
pub unsafe extern "C" fn HIST_add(
    mut count: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) {
    let mut ip = src as *const u8;
    let end = ip.offset(srcSize as isize);
    while ip < end {
        let fresh0 = ip;
        ip = ip.offset(1);
        let ref mut fresh1 = *count.offset(*fresh0 as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count_simple(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> std::ffi::c_uint {
    let mut ip = src as *const u8;
    let end = ip.offset(srcSize as isize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount: std::ffi::c_uint = 0;
    libc::memset(
        count as *mut std::ffi::c_void,
        0,
        (maxSymbolValue.wrapping_add(1)
            as std::ffi::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<std::ffi::c_uint>(),
            ) as usize,
    );
    if srcSize == 0 {
        *maxSymbolValuePtr = 0;
        return 0;
    }
    while ip < end {
        let fresh2 = ip;
        ip = ip.offset(1);
        let ref mut fresh3 = *count.offset(*fresh2 as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        *fresh3;
    }
    while *count.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
        maxSymbolValue;
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: u32 = 0;
    s = 0;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > largestCount {
            largestCount = *count.offset(s as isize);
        }
        s = s.wrapping_add(1);
        s;
    }
    return largestCount;
}
unsafe extern "C" fn HIST_count_parallel_wksp(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
    mut check: HIST_checkInput_e,
    workSpace: *mut u32,
) -> usize {
    let mut ip = source as *const u8;
    let iend = ip.offset(sourceSize as isize);
    let countSize = ((*maxSymbolValuePtr)
        .wrapping_add(1) as std::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>());
    let mut max: std::ffi::c_uint = 0;
    let Counting1 = workSpace;
    let Counting2 = Counting1.offset(256);
    let Counting3 = Counting2.offset(256);
    let Counting4 = Counting3.offset(256);
    if sourceSize == 0 {
        libc::memset(count, 0, (countSize) as usize);
        *maxSymbolValuePtr = 0;
        return 0;
    }
    libc::memset(
        workSpace as *mut std::ffi::c_void,
        0,
        ((4 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<std::ffi::c_uint>(),
            ) as usize,
    );
    let mut cached = MEM_read32(ip as *const std::ffi::c_void);
    ip = ip.offset(4);
    while ip < iend.offset(-15_isize) {
        let mut c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4);
        let ref mut fresh4 = *Counting1.offset(c as u8 as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *fresh4;
        let ref mut fresh5 = *Counting2
            .offset((c >> 8) as u8 as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *fresh5;
        let ref mut fresh6 = *Counting3
            .offset((c >> 16) as u8 as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        *fresh6;
        let ref mut fresh7 = *Counting4.offset((c >> 24) as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        *fresh7;
        c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4);
        let ref mut fresh8 = *Counting1.offset(c as u8 as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        *fresh8;
        let ref mut fresh9 = *Counting2
            .offset((c >> 8) as u8 as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
        *fresh9;
        let ref mut fresh10 = *Counting3
            .offset((c >> 16) as u8 as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        *fresh10;
        let ref mut fresh11 = *Counting4.offset((c >> 24) as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
        *fresh11;
        c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4);
        let ref mut fresh12 = *Counting1.offset(c as u8 as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
        *fresh12;
        let ref mut fresh13 = *Counting2
            .offset((c >> 8) as u8 as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
        *fresh13;
        let ref mut fresh14 = *Counting3
            .offset((c >> 16) as u8 as isize);
        *fresh14 = (*fresh14).wrapping_add(1);
        *fresh14;
        let ref mut fresh15 = *Counting4.offset((c >> 24) as isize);
        *fresh15 = (*fresh15).wrapping_add(1);
        *fresh15;
        c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4);
        let ref mut fresh16 = *Counting1.offset(c as u8 as isize);
        *fresh16 = (*fresh16).wrapping_add(1);
        *fresh16;
        let ref mut fresh17 = *Counting2
            .offset((c >> 8) as u8 as isize);
        *fresh17 = (*fresh17).wrapping_add(1);
        *fresh17;
        let ref mut fresh18 = *Counting3
            .offset((c >> 16) as u8 as isize);
        *fresh18 = (*fresh18).wrapping_add(1);
        *fresh18;
        let ref mut fresh19 = *Counting4.offset((c >> 24) as isize);
        *fresh19 = (*fresh19).wrapping_add(1);
        *fresh19;
    }
    ip = ip.offset(-4_isize);
    while ip < iend {
        let fresh20 = ip;
        ip = ip.offset(1);
        let ref mut fresh21 = *Counting1.offset(*fresh20 as isize);
        *fresh21 = (*fresh21).wrapping_add(1);
        *fresh21;
    }
    let mut s: u32 = 0;
    s = 0;
    while s < 256 {
        let ref mut fresh22 = *Counting1.offset(s as isize);
        *fresh22 = (*fresh22)
            .wrapping_add(
                (*Counting2.offset(s as isize))
                    .wrapping_add(*Counting3.offset(s as isize))
                    .wrapping_add(*Counting4.offset(s as isize)),
            );
        if *Counting1.offset(s as isize) > max {
            max = *Counting1.offset(s as isize);
        }
        s = s.wrapping_add(1);
        s;
    }
    let mut maxSymbolValue: std::ffi::c_uint = 255;
    while *Counting1.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
        maxSymbolValue;
    }
    RETURN_ERROR_IF!(check as std::ffi::c_uint != 0 && maxSymbolValue > *maxSymbolValuePtr, ZSTD_error_maxSymbolValue_tooSmall);
    *maxSymbolValuePtr = maxSymbolValue;
    libc::memmove(count, Counting1, (countSize) as usize);
    return max as usize;
}
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast_wksp(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut workSpaceSize: usize,
) -> usize {
    if sourceSize < 1500 {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as usize;
    }
    RETURN_ERROR_IF!(workSpace as usize & 3_usize != 0, ZSTD_error_GENERIC);
    RETURN_ERROR_IF!(workSpaceSize < HIST_WKSP_SIZE, ZSTD_error_workSpace_tooSmall);
    return HIST_count_parallel_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        trustInput,
        workSpace as *mut u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count_wksp(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut workSpaceSize: usize,
) -> usize {
    RETURN_ERROR_IF!(workSpace as usize & 3_usize != 0, ZSTD_error_GENERIC);
    RETURN_ERROR_IF!(workSpaceSize < HIST_WKSP_SIZE, ZSTD_error_workSpace_tooSmall);
    if *maxSymbolValuePtr < 255 {
        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            checkMaxSymbolValue,
            workSpace as *mut u32,
        );
    }
    *maxSymbolValuePtr = 255;
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        workSpace,
        workSpaceSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
) -> usize {
    let mut tmpCounters: [std::ffi::c_uint; 1024] = [0; 1024];
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[std::ffi::c_uint; 1024]>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut tmpCounters: [std::ffi::c_uint; 1024] = [0; 1024];
    return HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[std::ffi::c_uint; 1024]>(),
    );
}
