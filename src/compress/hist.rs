use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type unalign32 = U32;
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
pub type HIST_checkInput_e = libc::c_uint;
pub const checkMaxSymbolValue: HIST_checkInput_e = 1;
pub const trustInput: HIST_checkInput_e = 0;
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return *(ptr as *const unalign32);
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
pub const HIST_WKSP_SIZE_U32: libc::c_int = 1024 as libc::c_int;
pub const HIST_WKSP_SIZE: libc::c_ulong = (HIST_WKSP_SIZE_U32 as libc::c_ulong)
    .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong);
#[no_mangle]
pub unsafe extern "C" fn HIST_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count_simple(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> libc::c_uint {
    let mut ip = src as *const BYTE;
    let end = ip.offset(srcSize as isize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount = 0 as libc::c_int as libc::c_uint;
    libc::memset(
        count as *mut libc::c_void,
        0 as libc::c_int,
        (maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as libc::size_t,
    );
    if srcSize == 0 as libc::c_int as libc::c_ulong {
        *maxSymbolValuePtr = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int as libc::c_uint;
    }
    while ip < end {
        if *ip as libc::c_uint <= maxSymbolValue {} else {
            __assert_fail(
                b"*ip <= maxSymbolValue\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/hist.c\0" as *const u8
                    as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"unsigned int HIST_count_simple(unsigned int *, unsigned int *, const void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        let fresh0 = ip;
        ip = ip.offset(1);
        let ref mut fresh1 = *count.offset(*fresh0 as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
    }
    while *count.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: U32 = 0;
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > largestCount {
            largestCount = *count.offset(s as isize);
        }
        s = s.wrapping_add(1);
    }
    return largestCount;
}
unsafe extern "C" fn HIST_count_parallel_wksp(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
    mut check: HIST_checkInput_e,
    workSpace: *mut U32,
) -> size_t {
    let mut ip = source as *const BYTE;
    let iend = ip.offset(sourceSize as isize);
    let countSize = ((*maxSymbolValuePtr).wrapping_add(1 as libc::c_int as libc::c_uint)
        as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong);
    let mut max = 0 as libc::c_int as libc::c_uint;
    let Counting1 = workSpace;
    let Counting2 = Counting1.offset(256 as libc::c_int as isize);
    let Counting3 = Counting2.offset(256 as libc::c_int as isize);
    let Counting4 = Counting3.offset(256 as libc::c_int as isize);
    if *maxSymbolValuePtr <= 255 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"*maxSymbolValuePtr <= 255\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/hist.c\0" as *const u8
                as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"size_t HIST_count_parallel_wksp(unsigned int *, unsigned int *, const void *, size_t, HIST_checkInput_e, U32 *const)\0",
            ))
                .as_ptr(),
        );
    }
    if sourceSize == 0 {
        libc::memset(
            count as *mut libc::c_void,
            0 as libc::c_int,
            countSize as libc::size_t,
        );
        *maxSymbolValuePtr = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int as size_t;
    }
    libc::memset(
        workSpace as *mut libc::c_void,
        0 as libc::c_int,
        ((4 as libc::c_int * 256 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as libc::size_t,
    );
    let mut cached = MEM_read32(ip as *const libc::c_void);
    ip = ip.offset(4 as libc::c_int as isize);
    while ip < iend.offset(-(15 as libc::c_int as isize)) {
        let mut c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4 as libc::c_int as isize);
        let ref mut fresh2 = *Counting1.offset(c as BYTE as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        let ref mut fresh3 = *Counting2.offset((c >> 8 as libc::c_int) as BYTE as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        let ref mut fresh4 = *Counting3
            .offset((c >> 16 as libc::c_int) as BYTE as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        let ref mut fresh5 = *Counting4.offset((c >> 24 as libc::c_int) as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4 as libc::c_int as isize);
        let ref mut fresh6 = *Counting1.offset(c as BYTE as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        let ref mut fresh7 = *Counting2.offset((c >> 8 as libc::c_int) as BYTE as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        let ref mut fresh8 = *Counting3
            .offset((c >> 16 as libc::c_int) as BYTE as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        let ref mut fresh9 = *Counting4.offset((c >> 24 as libc::c_int) as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4 as libc::c_int as isize);
        let ref mut fresh10 = *Counting1.offset(c as BYTE as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        let ref mut fresh11 = *Counting2
            .offset((c >> 8 as libc::c_int) as BYTE as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
        let ref mut fresh12 = *Counting3
            .offset((c >> 16 as libc::c_int) as BYTE as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
        let ref mut fresh13 = *Counting4.offset((c >> 24 as libc::c_int) as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4 as libc::c_int as isize);
        let ref mut fresh14 = *Counting1.offset(c as BYTE as isize);
        *fresh14 = (*fresh14).wrapping_add(1);
        let ref mut fresh15 = *Counting2
            .offset((c >> 8 as libc::c_int) as BYTE as isize);
        *fresh15 = (*fresh15).wrapping_add(1);
        let ref mut fresh16 = *Counting3
            .offset((c >> 16 as libc::c_int) as BYTE as isize);
        *fresh16 = (*fresh16).wrapping_add(1);
        let ref mut fresh17 = *Counting4.offset((c >> 24 as libc::c_int) as isize);
        *fresh17 = (*fresh17).wrapping_add(1);
    }
    ip = ip.offset(-(4 as libc::c_int as isize));
    while ip < iend {
        let fresh18 = ip;
        ip = ip.offset(1);
        let ref mut fresh19 = *Counting1.offset(*fresh18 as isize);
        *fresh19 = (*fresh19).wrapping_add(1);
    }
    let mut s: U32 = 0;
    s = 0 as libc::c_int as U32;
    while s < 256 as libc::c_int as libc::c_uint {
        let ref mut fresh20 = *Counting1.offset(s as isize);
        *fresh20 = (*fresh20 as libc::c_uint)
            .wrapping_add(
                (*Counting2.offset(s as isize))
                    .wrapping_add(*Counting3.offset(s as isize))
                    .wrapping_add(*Counting4.offset(s as isize)),
            ) as U32 as U32;
        if *Counting1.offset(s as isize) > max {
            max = *Counting1.offset(s as isize);
        }
        s = s.wrapping_add(1);
    }
    let mut maxSymbolValue = 255 as libc::c_int as libc::c_uint;
    while *Counting1.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    if check as libc::c_uint != 0 && maxSymbolValue > *maxSymbolValuePtr {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
    }
    *maxSymbolValuePtr = maxSymbolValue;
    libc::memmove(
        count as *mut libc::c_void,
        Counting1 as *const libc::c_void,
        countSize as libc::size_t,
    );
    return max as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast_wksp(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut workSpaceSize: size_t,
) -> size_t {
    if sourceSize < 1500 as libc::c_int as libc::c_ulong {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as size_t;
    }
    if workSpace as size_t & 3 as libc::c_int as libc::c_ulong != 0 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t;
    }
    return HIST_count_parallel_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        trustInput,
        workSpace as *mut U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count_wksp(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut libc::c_void,
    mut workSpaceSize: size_t,
) -> size_t {
    if workSpace as size_t & 3 as libc::c_int as libc::c_ulong != 0 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t;
    }
    if *maxSymbolValuePtr < 255 as libc::c_int as libc::c_uint {
        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            checkMaxSymbolValue,
            workSpace as *mut U32,
        );
    }
    *maxSymbolValuePtr = 255 as libc::c_int as libc::c_uint;
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
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [libc::c_uint; 1024] = [0; 1024];
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_uint; 1024]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [libc::c_uint; 1024] = [0; 1024];
    return HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_uint; 1024]>() as libc::c_ulong,
    );
}
