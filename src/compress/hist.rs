use std::mem::size_of;

use crate::zstd_h::*;
use crate::common::mem::*;
use crate::common::error::*;

// ==== from hist.h ====
pub const HIST_WKSP_SIZE_U32: usize = 1024;
pub const HIST_WKSP_SIZE: usize = HIST_WKSP_SIZE_U32 * size_of::<u32>();
// ==== end  hist.h ====

/*-**************************************************************
 *  Histogram functions
 ****************************************************************/

 /** HIST_add() :
 *  Lowest level: just add nb of occurrences of characters from @src into @count.
 *  @count is not reset. @count array is presumed large enough (i.e. 1 KB).
 @  This function does not need any additional stack memory.
 */
pub unsafe fn HIST_add(
    count: *mut u32,
    src: *const std::ffi::c_void,
    srcSize: usize,
) {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);
    while ip < end {
        let ref mut fresh1 = *count.offset(*ip as isize);
        *fresh1 = (*fresh1).wrapping_add(1);

        ip = ip.offset(1);
    }
}

/** HIST_count_simple() :
 *  Same as HIST_countFast(), this function is unsafe,
 *  and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
 *  It is also a bit slower for large inputs.
 *  However, it does not need any additional memory (not even on stack).
 * @return : count of the most frequent symbol.
 *  Note this function doesn't produce any error (i.e. it must succeed).
 */
pub unsafe fn HIST_count_simple(
    mut count: *mut u32,
    mut maxSymbolValuePtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> u32 {
    let mut ip = src as *const u8;
    let end = ip.offset(srcSize as isize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount: u32 = 0;
    libc::memset(
        count as *mut std::ffi::c_void,
        0,
        (maxSymbolValue as usize + 1) * size_of::<u32>(),
    );
    if srcSize == 0 {
        *maxSymbolValuePtr = 0;
        return 0;
    }
    while ip < end {
        let ref mut fresh3 = *count.offset(*ip as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        ip = ip.offset(1);
    }
    while *count.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: u32 = 0;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > largestCount {
            largestCount = *count.offset(s as isize);
        }
        s = s.wrapping_add(1);
    }
    return largestCount;
}

pub type HIST_checkInput_e = u32;
pub const checkMaxSymbolValue: HIST_checkInput_e = 1;
pub const trustInput: HIST_checkInput_e = 0;

/* HIST_count_parallel_wksp() :
 * store histogram into 4 intermediate tables, recombined at the end.
 * this design makes better use of OoO cpus,
 * and is noticeably faster when some values are heavily repeated.
 * But it needs some additional workspace for intermediate tables.
 * `workSpace` must be a U32 table of size >= HIST_WKSP_SIZE_U32.
 * @return : largest histogram frequency,
 *           or an error code (notably when histogram's alphabet is larger than *maxSymbolValuePtr) */
unsafe fn HIST_count_parallel_wksp(
    mut count: *mut u32,
    mut maxSymbolValuePtr: *mut u32,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
    mut check: HIST_checkInput_e,
    workSpace: *mut u32,
) -> usize {
    let mut ip = source as *const u8;
    let iend = ip.offset(sourceSize as isize);
    let countSize = ((*maxSymbolValuePtr) as usize + 1)
        .wrapping_mul(size_of::<u32>());
    let mut max: u32 = 0;
    let Counting1 = workSpace;
    let Counting2 = Counting1.offset(256);
    let Counting3 = Counting2.offset(256);
    let Counting4 = Counting3.offset(256);

    /* safety checks */
    debug_assert!(*maxSymbolValuePtr <= 255);
    if sourceSize == 0 {
        libc::memset(count.cast(), 0, countSize);
        *maxSymbolValuePtr = 0;
        return 0;
    }
    libc::memset(
        workSpace as *mut std::ffi::c_void,
        0,
        (4_usize * 256) * size_of::<u32>(),
    );

    /* by stripes of 16 bytes */
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

    /* finish last symbols */
    while ip < iend {
        let ref mut fresh21 = *Counting1.offset(*ip as isize);
        *fresh21 = (*fresh21).wrapping_add(1);
        ip = ip.offset(1);
    }

    let mut s: u32 = 0;
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

    let mut maxSymbolValue: u32 = 255;
    while *Counting1.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    RETURN_ERROR_IF!(check != 0 && maxSymbolValue > *maxSymbolValuePtr, ZSTD_error_maxSymbolValue_tooSmall);
    *maxSymbolValuePtr = maxSymbolValue;
    libc::memmove(count.cast(), Counting1.cast_const().cast(), countSize); /* in case count & Counting1 are overlapping */
    return max as usize;
}

/** HIST_countFast_wksp() :
 *  Same as HIST_countFast(), but using an externally provided scratch buffer.
 * `workSpace` is a writable buffer which must be 4-bytes aligned,
 * `workSpaceSize` must be >= HIST_WKSP_SIZE
 */
pub unsafe fn HIST_countFast_wksp(
    mut count: *mut u32,
    mut maxSymbolValuePtr: *mut u32,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut workSpaceSize: usize,
) -> usize {
    if sourceSize < 1500 { /* heuristic threshold */
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as usize;
    }
    RETURN_ERROR_IF!(!workSpace.is_aligned_to(4), ZSTD_error_GENERIC); /* must be aligned on 4-bytes boundaries */
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

/** HIST_count_wksp() :
 *  Same as HIST_count(), but using an externally provided scratch buffer.
 *  Benefit is this function will use very little stack space.
 * `workSpace` is a writable buffer which must be 4-bytes aligned,
 * `workSpaceSize` must be >= HIST_WKSP_SIZE
 */
pub unsafe fn HIST_count_wksp(
    mut count: *mut u32,
    mut maxSymbolValuePtr: *mut u32,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
    mut workSpace: *mut std::ffi::c_void,
    mut workSpaceSize: usize,
) -> usize {
    RETURN_ERROR_IF!(!workSpace.is_aligned_to(4), ZSTD_error_GENERIC); /* must be aligned on 4-bytes boundaries */
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

/** HIST_countFast() :
 *  same as HIST_count(), but blindly trusts that all byte values within src are <= *maxSymbolValuePtr.
 *  This function is unsafe, and will segfault if any value within `src` is `> *maxSymbolValuePtr`
 */
pub unsafe fn HIST_countFast(
    mut count: *mut u32,
    mut maxSymbolValuePtr: *mut u32,
    mut source: *const std::ffi::c_void,
    mut sourceSize: usize,
) -> usize {
    let mut tmpCounters: [u32; 1024] = [0; 1024];
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut std::ffi::c_void,
        size_of::<[u32; 1024]>(),
    );
}
/** HIST_count():
 *  Provides the precise count of each byte within a table 'count'.
 * 'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
 *  Updates *maxSymbolValuePtr with actual largest symbol value detected.
 * @return : count of the most frequent symbol (which isn't identified).
 *           or an error code, which can be tested using HIST_isError().
 *           note : if return == srcSize, there is only one symbol.
 */
pub unsafe fn HIST_count(
    mut count: *mut u32,
    mut maxSymbolValuePtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    let mut tmpCounters: [u32; 1024] = [0; 1024];
    return HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut std::ffi::c_void,
        size_of::<[u32; 1024]>(),
    );
}
