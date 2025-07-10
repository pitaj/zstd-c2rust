use std::mem::size_of;
use std::ffi::{c_char, c_void};

use crate::common::fse_h::*;
use crate::common::bitstream_h::*;
use crate::common::error::*;
use crate::common::mem::*;
use crate::common::bits::*;

/* **************************************************************
*  Templates
****************************************************************/
/*
  designed to be included
  for type-specific functions (template emulation in C)
  Objective is to write these functions only once, for improved maintenance
*/

unsafe fn FSE_buildDTable_internal(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    let tdPtr = dt.offset(1) as *mut c_void; /* because *dt is unsigned, 32-bits aligned on 32-bits */
    let tableDecode = tdPtr as *mut FSE_decode_t;
    let mut symbolNext = workSpace as *mut u16;
    let mut spread = symbolNext.offset(maxSymbolValue as isize).offset(1) as *mut u8;

    let maxSV1 = maxSymbolValue + 1;
    let tableSize = 1_u32 << tableLog;
    let mut highThreshold = tableSize - 1;

    /* Sanity Checks */
    if FSE_BUILD_DTABLE_WKSP_SIZE(tableLog, maxSymbolValue) > wkspSize { return ERROR(ZSTD_error_maxSymbolValue_tooLarge); }
    if (maxSymbolValue as usize) > FSE_MAX_SYMBOL_VALUE { return ERROR(ZSTD_error_maxSymbolValue_tooLarge); }
    if tableLog > FSE_MAX_TABLELOG { return ERROR(ZSTD_error_tableLog_tooLarge); }

    /* Init, lay down lowprob symbols */
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as u16;
    DTableH.fastMode = 1;
    let largeLimit = 1_i16 << (tableLog - 1);
    for s in 0..(maxSV1 as usize) {
        if *normalizedCounter.add(s) == -1 {
            (*tableDecode.offset(highThreshold as isize)).symbol = s as u8;
            highThreshold -= 1;
            *symbolNext.add(s) = 1;
        } else {
            if *normalizedCounter.add(s) >= largeLimit {
                DTableH.fastMode = 0;
            }
            *symbolNext.add(s) = *normalizedCounter.add(s) as u16;
        }
    }
    libc::memcpy(
        dt as *mut c_void,
        &mut DTableH as *mut FSE_DTableHeader as *const c_void,
        size_of::<FSE_DTableHeader>(),
    );

    /* Spread symbols */
    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1) as usize;
        let step = FSE_TABLESTEP(tableSize) as usize;
        /* First lay down the symbols in order.
         * We use a uint64_t to lay down 8 bytes at a time. This reduces branch
         * misses since small blocks generally have small table logs, so nearly
         * all symbols have counts <= 8. We ensure we have 8 bytes at the end of
         * our buffer to handle the over-write.
         */
        let add = 0x101010101010101_u64;
        let mut pos: usize = 0;
        let mut sv: u64 = 0;
        for s_0 in 0..(maxSV1 as usize) {
            let mut i: isize = 0;
            let n = *normalizedCounter.add(s_0) as isize;
            MEM_write64(spread.add(pos) as *mut c_void, sv);
            i = 8;
            while i < n {
                MEM_write64(
                    spread.add(pos).offset(i) as *mut c_void,
                    sv,
                );
                i += 8;
            }
            pos = pos.wrapping_add(n as usize);
            sv = sv.wrapping_add(add);
        }
        /* Now we spread those positions across the table.
         * The benefit of doing it in two stages is that we avoid the
         * variable size inner loop, which caused lots of branch misses.
         * Now we can run through all the positions without any branch misses.
         * We unroll the loop twice, since that is what empirically worked best.
         */
        let mut position: usize = 0;
        let mut s_1: usize = 0;
        let unroll: usize = 2;
        debug_assert!((tableSize as usize) % unroll == 0); /* FSE_MIN_TABLELOG is 5 */
        while s_1 < tableSize as usize {
            for u in 0..unroll {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                (*tableDecode.add(uPosition)).symbol = *spread.add(s_1.wrapping_add(u));
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
        debug_assert!(position == 0);
    } else {
        let tableMask_0 = tableSize-1;
        let step_0 = FSE_TABLESTEP(tableSize);
        let mut position_0: u32 = 0;
        for s_2 in 0..(maxSV1 as usize) {
            for i_0 in 0..*normalizedCounter.add(s_2) {
                (*tableDecode.offset(position_0 as isize)).symbol = s_2 as u8;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0; /* lowprob area */
                }
            }
        }
        if position_0 != 0 { return ERROR(ZSTD_error_GENERIC); } /* position must reach all cells once, otherwise normalizedCounter is incorrect */
    }

    /* Build Decoding table */
    for u_0 in 0..(tableSize as usize) {
        let symbol = (*tableDecode.add(u_0)).symbol;
        let ref mut fresh1 = *symbolNext.offset(symbol as isize);
        let nextState = *fresh1;
        *fresh1 += 1;
        (*tableDecode.add(u_0)).nbBits = (tableLog - ZSTD_highbit32(u32::from(nextState))) as u8;
        (*tableDecode.add(u_0)).newState = (u32::from(
            nextState << (*tableDecode.add(u_0)).nbBits
        ) - tableSize) as u16;
    }

    return 0;
}

pub unsafe fn FSE_buildDTable_wksp(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    return FSE_buildDTable_internal(
        dt,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    );
}

/*-*******************************************************
*  Decompression (Byte symbols)
*********************************************************/

#[inline(always)]
unsafe fn FSE_decompress_usingDTable_generic(
    mut dst: *mut c_void,
    mut maxDstSize: usize,
    mut cSrc: *const c_void,
    mut cSrcSize: usize,
    mut dt: *const FSE_DTable,
    fast: bool,
) -> usize {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.add(maxDstSize);
    let olimit = omax.offset(-3);

    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null(),
        start: std::ptr::null(),
        limitPtr: std::ptr::null(),
    };
    let mut state1 = FSE_DState_t {
        state: 0,
        table: std::ptr::null(),
    };
    let mut state2 = FSE_DState_t {
        state: 0,
        table: std::ptr::null(),
    };

    /* Init */
    FORWARD_IF_ERROR!(BIT_initDStream(&mut bitD, cSrc, cSrcSize));

    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);

    RETURN_ERROR_IF!(BIT_reloadDStream(&mut bitD) == BIT_DStream_overflow, ZSTD_error_corruption_detected);

    macro_rules! FSE_GETSYMBOL {
        ($statePtr:expr) => {
            if fast {
                FSE_decodeSymbolFast($statePtr, &mut bitD)
            } else {
                FSE_decodeSymbol($statePtr, &mut bitD)
            }
        }
    }

    /* 4 symbols per loop */
    while (BIT_reloadDStream(&mut bitD) == BIT_DStream_unfinished) & (op < olimit) {
        *op.offset(0) = FSE_GETSYMBOL!(&mut state1);

        if ((FSE_MAX_TABLELOG as usize) * 2 + 7) > (size_of::<BitContainerType>() * 8) { /* This test must be static */
            BIT_reloadDStream(&mut bitD);
        }

        *op.offset(1) = FSE_GETSYMBOL!(&mut state2);

        if ((FSE_MAX_TABLELOG as usize) * 4 + 7) > (size_of::<BitContainerType>() * 8) { /* This test must be static */
            if BIT_reloadDStream(&mut bitD) > BIT_DStream_unfinished {
                op = op.offset(2);
                break;
            }
        }

        *op.offset(2) = FSE_GETSYMBOL!(&mut state1);

        if ((FSE_MAX_TABLELOG as usize) * 2 + 7) > (size_of::<BitContainerType>() * 8) { /* This test must be static */
            BIT_reloadDStream(&mut bitD);
        }

        *op.offset(3) = FSE_GETSYMBOL!(&mut state2);

        op = op.offset(4);
    }

    /* tail */
    /* note : BIT_reloadDStream(&bitD) >= FSE_DStream_partiallyFilled; Ends at exactly BIT_DStream_completed */
    loop {
        if op > omax.offset(-2) { return ERROR(ZSTD_error_dstSize_tooSmall); }
        *op = FSE_GETSYMBOL!(&mut state1);
        op = op.offset(1);

        if BIT_reloadDStream(&mut bitD) == BIT_DStream_overflow {
            *op = FSE_GETSYMBOL!(&mut state2);
            op = op.offset(1);
            break;
        }

        if op > omax.offset(-2) { return ERROR(ZSTD_error_dstSize_tooSmall); }
        *op = FSE_GETSYMBOL!(&mut state2);
        op = op.offset(1);
        if BIT_reloadDStream(&mut bitD) == BIT_DStream_overflow {
            *op = FSE_GETSYMBOL!(&mut state1);
            op = op.offset(1);
            break;
        }
    }

    debug_assert!(op >= ostart);
    return op.offset_from(ostart) as usize;
}

#[derive(Clone, Copy, Debug)]
struct FSE_DecompressWksp {
    ncount: [i16; FSE_MAX_SYMBOL_VALUE + 1]
}

#[inline(always)]
unsafe extern "C" fn FSE_decompress_wksp_body(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut cSrc: *const c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
    mut bmi2: bool,
) -> usize {
    let istart = cSrc as *const u8;
    let mut ip = istart;
    let mut tableLog: u32 = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as u32;
    let wksp = workSpace as *mut FSE_DecompressWksp;
    let dtablePos = size_of::<FSE_DecompressWksp>() / size_of::<FSE_DTable>();
    let dtable = (workSpace as *mut FSE_DTable).add(dtablePos);

    const _: () = assert!((FSE_MAX_SYMBOL_VALUE + 1) % 2 == 0);
    if wkspSize < size_of::<FSE_DecompressWksp>() { return ERROR(ZSTD_error_GENERIC); }

    /* correct offset to dtable depends on this property */
    const _: () = assert!(size_of::<FSE_DecompressWksp>() % size_of::<FSE_DTable>() == 0);

    /* normal FSE decoding mode */
    let NCountLength = FSE_readNCount_bmi2(
        ((*wksp).ncount).as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const c_void,
        cSrcSize,
        bmi2,
    );
    if ERR_isError(NCountLength) { return NCountLength; }
    if tableLog > maxLog { return ERROR(ZSTD_error_tableLog_tooLarge); }
    debug_assert!(NCountLength <= cSrcSize);
    ip = ip.add(NCountLength);
    cSrcSize -= NCountLength;

    if FSE_DECOMPRESS_WKSP_SIZE(tableLog, maxSymbolValue) > wkspSize {
        return ERROR(ZSTD_error_tableLog_tooLarge);
    }
    debug_assert!(size_of::<FSE_DecompressWksp>() + FSE_DTABLE_SIZE(tableLog) <= wkspSize);
    workSpace = (workSpace as *mut u8)
        .add(size_of::<FSE_DecompressWksp>())
        .add(FSE_DTABLE_SIZE(tableLog)) as *mut c_void;
    wkspSize -= size_of::<FSE_DecompressWksp>() + FSE_DTABLE_SIZE(tableLog);

    FORWARD_IF_ERROR!(FSE_buildDTable_internal(
        dtable,
        ((*wksp).ncount).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    ));

    let mut ptr = dtable as *const c_void;
    let mut DTableH = ptr as *const FSE_DTableHeader;
    let fastMode = (*DTableH).fastMode != 0;

    /* select fast mode (static) */
    if fastMode {
        return FSE_decompress_usingDTable_generic(
            dst,
            dstCapacity,
            ip as *const c_void,
            cSrcSize,
            dtable,
            true,
        );
    }
    return FSE_decompress_usingDTable_generic(
        dst,
        dstCapacity,
        ip as *const c_void,
        cSrcSize,
        dtable,
        false,
    );
}

/* Avoids the FORCE_INLINE of the _body() function. */
unsafe fn FSE_decompress_wksp_body_default(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut cSrc: *const c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    return FSE_decompress_wksp_body(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
        false,
    );
}

// TODO #if DYNAMIC_BMI2
unsafe fn FSE_decompress_wksp_body_bmi2(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut cSrc: *const c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    return FSE_decompress_wksp_body(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
        true,
    );
}

pub unsafe fn FSE_decompress_wksp_bmi2(
    mut dst: *mut c_void,
    mut dstCapacity: usize,
    mut cSrc: *const c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
    mut bmi2: bool,
) -> usize {
// TODO #if DYNAMIC_BMI2
    if bmi2 {
        return FSE_decompress_wksp_body_bmi2(
            dst,
            dstCapacity,
            cSrc,
            cSrcSize,
            maxLog,
            workSpace,
            wkspSize,
        );
    }
    return FSE_decompress_wksp_body_default(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
    );
}
