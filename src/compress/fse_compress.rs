use crate::common::fse_h::*;
use crate::common::error::*;
use crate::common::bitstream_h::*;
use crate::common::mem::*;
use crate::common::bits::*;

/* Function templates */

/** FSE_buildCTable_wksp() :
 * Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
 * wkspSize should be sized to handle worst case situation, which is `1<<max_tableLog * sizeof(FSE_FUNCTION_TYPE)`
 * workSpace must also be properly aligned with FSE_FUNCTION_TYPE requirements
 */
pub unsafe fn FSE_buildCTable_wksp(
    mut ct: *mut FSE_CTable,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let tableSize = 1_u32 << tableLog;
    let tableMask = tableSize.wrapping_sub(1);
    let ptr = ct as *mut std::ffi::c_void;
    let tableU16 = (ptr as *mut u16).offset(2);
    let FSCT = (ptr as *mut u32)
        .offset(1) /* header */
        .offset(
            if tableLog != 0 {
                tableSize as isize >> 1
            } else {
                1
            },
        ) as *mut std::ffi::c_void;
    let symbolTT = FSCT as *mut FSE_symbolCompressionTransform;
    let step = FSE_TABLESTEP(tableSize);
    let maxSV1 = maxSymbolValue.wrapping_add(1);

    let mut cumul = workSpace as *mut u16; /* size = maxSV1 */
    let tableSymbol = cumul
        .offset(maxSV1.wrapping_add(1) as isize) as *mut u8; /* size = tableSize */
    
    let mut highThreshold = tableSize.wrapping_sub(1);
    
    debug_assert!(workSpace.is_aligned_to(2));  /* Must be 2 bytes-aligned */
    if FSE_BUILD_CTABLE_WORKSPACE_SIZE(maxSymbolValue, tableLog) > wkspSize {
        return ERROR(ZSTD_error_tableLog_tooLarge);
    }
    /* CTable header */
    *tableU16.offset(-2) = tableLog as u16;
    *tableU16.offset(-1) = maxSymbolValue as u16;
    debug_assert!(tableLog < 16);   /* required for threshold strategy to work */

    /* For explanations on how to distribute symbol values over the table :
     * https://fastcompression.blogspot.fr/2014/02/fse-distributing-symbol-values.html */

    // TODO? #ifdef __clang_analyzer__
    //  ZSTD_memset(tableSymbol, 0, sizeof(*tableSymbol) * tableSize);   /* useless initialization, just to keep scan-build happy */
    //  #endif

    /* symbol start positions */
    *cumul.offset(0) = 0;
    for u in 1..=(maxSV1 as usize) {
        if *normalizedCounter.add(u-1) == -1 { /* Low proba symbol */
            *cumul.add(u) = *cumul.add(u-1) + 1;
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            *tableSymbol
                .offset(
                    fresh0 as isize,
                ) = u.wrapping_sub(1) as u8;
        } else {
            debug_assert!(*normalizedCounter.add(u-1) >= 0);
            *cumul.add(u) = *cumul.add(u-1) + 
                (*normalizedCounter.add(u-1) as u16);
        }
    }
    *cumul
        .offset(
            maxSV1 as isize,
        ) = tableSize.wrapping_add(1) as u16;
    
    /* Spread symbols */
    if highThreshold == tableSize.wrapping_sub(1) {
        /* Case for no low prob count symbols. Lay down 8 bytes at a time
         * to reduce branch misses since we are operating on a small block
         */
        let spread = tableSymbol.offset(tableSize as isize); /* size = tableSize + 8 (may write beyond tableSize) */
        let add = 0x101010101010101_u64;
        let mut pos: usize = 0;
        let mut sv: u64 = 0;
        for s in 0..(maxSV1 as usize) {
            let n = *normalizedCounter.add(s) as isize;
            MEM_write64(spread.add(pos).cast(), sv);
            let mut i: isize = 8;
            while i < n {
                MEM_write64(
                    spread.add(pos).offset(i).cast(),
                    sv,
                );
                i += 8;
            }
            debug_assert!(n>=0);
            pos = pos.wrapping_add(n as usize);
            sv = sv.wrapping_add(add);
        }
        /* Spread symbols across the table. Lack of lowprob symbols means that
         * we don't need variable sized inner loop, so we can unroll the loop and
         * reduce branch misses.
         */
        let mut position: usize = 0;
        let unroll: usize = 2; /* TODO? Experimentally determined optimal unroll */
        debug_assert!(tableSize as usize % unroll == 0); /* FSE_MIN_TABLELOG is 5 */
        for s_0 in 0..(tableSize as usize) {
            for u_0 in 0..unroll {
                let uPosition = position.wrapping_add(u_0 * step as usize)
                    & tableMask as usize;
                *tableSymbol
                    .add(
                        uPosition
                    ) = *spread.add(s_0 + u_0);
            }
            position = position.wrapping_add(unroll * step as usize)
                & tableMask as usize;
        }
        debug_assert!(position == 0);   /* Must have initialized all positions */
    } else {
        let mut position_0: u32 = 0;
        for symbol in 0..maxSV1 {
            let freq = *normalizedCounter.offset(symbol as isize);
            for nbOccurrences in 0..freq {
                *tableSymbol.offset(position_0 as isize) = symbol as u8;
                position_0 = position_0.wrapping_add(step) & tableMask;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step) & tableMask; /* Low proba area */
                }
            }
        }
        debug_assert!(position_0 == 0);   /* Must have initialized all positions */
    }

    /* Build table */
    for u_1 in 0..tableSize {
        let mut s_1 = *tableSymbol.offset(u_1 as isize); /* ? note : static analyzer may not understand tableSymbol is properly initialized */
        let ref mut fresh1 = *cumul.offset(s_1 as isize);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        *tableU16.offset(fresh2 as isize) = tableSize.wrapping_add(u_1) as u16; /* TableU16 : sorted by symbol order; gives next state value */
    }

    /* Build Symbol Transformation Table */
    let mut total: u32 = 0;
    for s_2 in 0..=(maxSymbolValue as usize) {
        match *normalizedCounter.add(s_2) {
            0 => {
                /* filling nonetheless, for compatibility with FSE_getMaxNbBits() */
                (*symbolTT.add(s_2))
                    .deltaNbBits = (tableLog.wrapping_add(1) << 16)
                    .wrapping_sub(1_u32 << tableLog);
            }
            -1 | 1 => {
                (*symbolTT.add(s_2))
                    .deltaNbBits = (tableLog << 16).wrapping_sub(1_u32 << tableLog);
                debug_assert!(total <= i32::MAX as _);
                (*symbolTT.add(s_2))
                    .deltaFindState = (total - 1) as i32;
                total = total.wrapping_add(1);
            }
            _ => {
                debug_assert!(*normalizedCounter.add(s_2) > 1);
                let maxBitsOut = tableLog
                    .wrapping_sub(
                        ZSTD_highbit32(
                            (*normalizedCounter.add(s_2) as u32)
                                .wrapping_sub(1),
                        ),
                    );
                let minStatePlus = (*normalizedCounter.add(s_2) as u32)
                    << maxBitsOut;
                (*symbolTT.add(s_2))
                    .deltaNbBits = (maxBitsOut << 16).wrapping_sub(minStatePlus);
                (*symbolTT.add(s_2))
                    .deltaFindState = total
                    .wrapping_sub(
                        *normalizedCounter.add(s_2) as u32,
                    ) as i32;
                total = total
                    .wrapping_add(
                        *normalizedCounter.add(s_2) as u32,
                    );
            }
        }
    }

// TODO? #if 0  /* debug : symbol costs */
//     DEBUGLOG(5, "\n --- table statistics : ");
//     {   U32 symbol;
//         for (symbol=0; symbol<=maxSymbolValue; symbol++) {
//             DEBUGLOG(5, "%3u: w=%3i,   maxBits=%u, fracBits=%.2f",
//                 symbol, normalizedCounter[symbol],
//                 FSE_getMaxNbBits(symbolTT, symbol),
//                 (double)FSE_bitCost(symbolTT, tableLog, symbol, 8) / 256);
//     }   }
// #endif

    return 0;
}


/*-**************************************************************
*  FSE NCount encoding
****************************************************************/
pub unsafe fn FSE_NCountWriteBound(
    mut maxSymbolValue: u32,
    mut tableLog: u32,
) -> usize {
    let maxHeaderSize = (((maxSymbolValue as usize + 1) * (tableLog as usize)
        + 4 /* bitCount initialized at 4 */
        + 2 /* first two symbols may use one additional bit each */) / 8)
        + 1 /* round up to whole nb bytes */
        + 2 /* additional two bytes for bitstream flush */;
    if maxSymbolValue != 0 { maxHeaderSize } else { FSE_NCOUNTBOUND } /* maxSymbolValue==0 ? use default */
}

unsafe fn FSE_writeNCount_generic(
    mut header: *mut std::ffi::c_void,
    mut headerBufferSize: usize,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
    mut writeIsSafe: u32,
) -> usize {
    let ostart = header as *mut u8;
    let mut out = ostart;
    let oend = ostart.add(headerBufferSize);
    let mut nbBits: i32 = 0;
    let tableSize = (1 as i32) << tableLog;
    let mut remaining: i32 = 0;
    let mut threshold: i32 = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: i32 = 0;
    let mut symbol: u32 = 0;
    let alphabetSize = maxSymbolValue.wrapping_add(1);
    let mut previousIs0: bool = false;

    /* Table Size */
    bitStream = bitStream.wrapping_add(
        tableLog.wrapping_sub(FSE_MIN_TABLELOG) << bitCount,
    );
    bitCount += 4;

    /* Init */
    remaining = tableSize + 1; /* +1 for extra accuracy */
    threshold = tableSize;
    nbBits = tableLog as i32 + 1;

    while symbol < alphabetSize && remaining > 1 { /* stops at 1 */
        if previousIs0 {
            let mut start = symbol;
            while symbol < alphabetSize && *normalizedCounter.offset(symbol as isize) == 0 {
                symbol += 1;
            }
            if symbol == alphabetSize {
                break; /* incorrect distribution */
            }
            while symbol >= start+24 {
                start += 24;
                bitStream = bitStream.wrapping_add(0xffff_u32 << bitCount);
                if writeIsSafe == 0 && out > oend.offset(-2) {
                    return ERROR(ZSTD_error_dstSize_tooSmall); /* Buffer overflow */
                }
                *out.offset(0) = bitStream as u8;
                *out.offset(1) = (bitStream >> 8) as u8;
                out = out.offset(2);
                bitStream >>= 16;
            }
            while symbol >= start+3 {
                start += 3;
                bitStream = bitStream.wrapping_add(3_u32 << bitCount);
                bitCount += 2;
            }
            bitStream = bitStream.wrapping_add(symbol.wrapping_sub(start) << bitCount);
            bitCount += 2;
            if bitCount > 16 {
                if writeIsSafe == 0 && out > oend.offset(-2) {
                    return ERROR(ZSTD_error_dstSize_tooSmall); /* Buffer overflow */
                }
                *out.offset(0) = bitStream as u8;
                *out.offset(1) = (bitStream >> 8) as u8;
                out = out.offset(2);
                bitStream >>= 16;
                bitCount -= 16;
            }
        }
        let mut count = *normalizedCounter.offset(symbol as isize) as i32;
        symbol += 1;
        let max = 2 * threshold - 1 - remaining;
        remaining -= if count < 0 { -count } else { count };
        count += 1; /* +1 for extra accuracy */
        if count >= threshold {
            count += max; /* [0..max[ [max..threshold[ (...) [threshold+max 2*threshold[ */
        }
        bitStream = bitStream.wrapping_add((count as u32) << bitCount);
        bitCount += nbBits;
        bitCount -= (count < max) as i32;
        previousIs0 = count == 1;
        if remaining < 1 { return ERROR(ZSTD_error_GENERIC); }
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1;
        }
        if bitCount > 16 {
            if writeIsSafe == 0 && out > oend.offset(-2) {
                return ERROR(ZSTD_error_dstSize_tooSmall); /* Buffer overflow */
            }
            *out.offset(0) = bitStream as u8;
            *out.offset(1) = (bitStream >> 8) as u8;
            out = out.offset(2);
            bitStream >>= 16;
            bitCount -= 16;
        }
    }

    if remaining != 1 { return ERROR(ZSTD_error_GENERIC); }
    debug_assert!(symbol < alphabetSize);

    /* flush remaining bitStream */
    if writeIsSafe == 0 && out > oend.offset(-2) {
        return ERROR(ZSTD_error_dstSize_tooSmall); /* Buffer overflow */
    }
    *out.offset(0) = bitStream as u8;
    *out.offset(1) = (bitStream >> 8) as u8;
    out = out.offset(((bitCount + 7) / 8) as isize);

    debug_assert!(out >= ostart);
    return out.offset_from(ostart) as usize;
}

pub unsafe fn FSE_writeNCount(
    mut buffer: *mut std::ffi::c_void,
    mut bufferSize: usize,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
) -> usize {
    if tableLog > FSE_MAX_TABLELOG { return ERROR(ZSTD_error_tableLog_tooLarge); } /* Unsupported */
    if tableLog < FSE_MIN_TABLELOG { return ERROR(ZSTD_error_GENERIC); } /* Unsupported */
    
    if bufferSize < FSE_NCountWriteBound(maxSymbolValue, tableLog) {
        return FSE_writeNCount_generic(
            buffer,
            bufferSize,
            normalizedCounter,
            maxSymbolValue,
            tableLog,
            0,
        );
    }
    return FSE_writeNCount_generic(
        buffer,
        bufferSize,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        1,
    );
}

/*-**************************************************************
*  FSE Compression Code
****************************************************************/

/* provides the minimum logSize to safely represent a distribution */
unsafe fn FSE_minTableLog(
    mut srcSize: usize,
    mut maxSymbolValue: u32,
) -> u32 {
    let mut minBitsSrc = ZSTD_highbit32(srcSize as u32) + 1;
    let mut minBitsSymbols = ZSTD_highbit32(maxSymbolValue) + 2;
    let mut minBits = if minBitsSrc < minBitsSymbols {
        minBitsSrc
    } else {
        minBitsSymbols
    };
    debug_assert!(srcSize > 1); /* Not supported, RLE should be used instead */
    return minBits;
}

pub unsafe fn FSE_optimalTableLog_internal(
    mut maxTableLog: u32,
    mut srcSize: usize,
    mut maxSymbolValue: u32,
    mut minus: u32,
) -> u32 {
    let mut maxBitsSrc = ZSTD_highbit32(srcSize.wrapping_sub(1) as u32) - minus;
    let mut tableLog = maxTableLog;
    let mut minBits = FSE_minTableLog(srcSize, maxSymbolValue);
    debug_assert!(srcSize > 1); /* Not supported, RLE should be used instead */
    if tableLog == 0 {
        tableLog = FSE_DEFAULT_TABLELOG;
    }
    if maxBitsSrc < tableLog {
        tableLog = maxBitsSrc; /* Accuracy can be reduced */
    }
    if minBits > tableLog {
        tableLog = minBits; /* Need a minimum to safely represent all symbol values */
    }
    if tableLog < FSE_MIN_TABLELOG {
        tableLog = FSE_MIN_TABLELOG;
    }
    if tableLog > FSE_MAX_TABLELOG {
        tableLog = FSE_MAX_TABLELOG;
    }
    return tableLog;
}

pub unsafe fn FSE_optimalTableLog(
    mut maxTableLog: u32,
    mut srcSize: usize,
    mut maxSymbolValue: u32,
) -> u32 {
    return FSE_optimalTableLog_internal(
        maxTableLog,
        srcSize,
        maxSymbolValue,
        2,
    );
}

/* Secondary normalization method.
   To be used when primary method fails. */

unsafe fn FSE_normalizeM2(
    mut norm: *mut i16,
    mut tableLog: u32,
    mut count: *const u32,
    mut total: usize,
    mut maxSymbolValue: u32,
    mut lowProbCount: i16,
) -> usize {
    const NOT_YET_ASSIGNED: i16 = -2;
    let mut distributed: u32 = 0;
    let mut ToDistribute: u32 = 0;

    /* Init */
    let lowThreshold = (total >> tableLog) as u32;
    let mut lowOne = (total * 3 >> (tableLog + 1)) as u32;

    for s in 0..=(maxSymbolValue as usize) {
        if *count.add(s) == 0 {
            *norm.add(s) = 0;
        } else if *count.add(s) <= lowThreshold {
            *norm.add(s) = lowProbCount;
            distributed = distributed.wrapping_add(1);
            total = total.wrapping_sub(*count.add(s) as usize);
        } else if *count.add(s) <= lowOne {
            *norm.add(s) = 1;
            distributed = distributed.wrapping_add(1);
            total = total.wrapping_sub(*count.add(s) as usize);
        } else {
            *norm.add(s) = NOT_YET_ASSIGNED;
        }
    }
    let mut ToDistribute: u32 = (1_u32 << tableLog).wrapping_sub(distributed);

    if ToDistribute == 0 {
        return 0;
    }

    if total / ToDistribute as usize > lowOne as usize {
        /* risk of rounding to zero */
        lowOne = (total * 3 / (ToDistribute * 2) as usize) as u32;
        for s in 0..=(maxSymbolValue as usize) {
            if *norm.add(s) == NOT_YET_ASSIGNED
                && *count.add(s) <= lowOne
            {
                *norm.add(s) = 1;
                distributed = distributed.wrapping_add(1);
                total = total.wrapping_sub(*count.add(s) as usize);
            }
        }
        ToDistribute = (1_u32 << tableLog).wrapping_sub(distributed);
    }
    if distributed == maxSymbolValue+1 {
        /* all values are pretty poor;
           probably incompressible data (should have already been detected);
           find max, then give all remaining points to max */
        let mut maxV: u32 = 0;
        let mut maxC: u32 = 0;
        for s in 0..=(maxSymbolValue as usize) {
            if *count.add(s) > maxC {
                maxV = s as u32;
                maxC = *count.add(s);
            }
        }
        let ref mut fresh4 = *norm.offset(maxV as isize);
        *fresh4 = *fresh4 + ToDistribute as i16;
        return 0;
    }

    if total == 0 {
        /* all of the symbols were low enough for the lowOne or lowThreshold */
        let mut s = 0;
        while ToDistribute > 0 {
            if *norm.add(s) > 0 {
                ToDistribute = ToDistribute.wrapping_sub(1);
                let ref mut fresh5 = *norm.add(s);
                *fresh5 += 1;
            }
            s = s.wrapping_add(1) % (maxSymbolValue as usize).wrapping_add(1);
        }
        return 0;
    }

    let vStepLog = 62_u32.wrapping_sub(tableLog) as u64;
    let mid = (1_u64 << vStepLog.wrapping_sub(1)) - 1;
    let rStep = (((1_u64 << vStepLog) * u64::from(ToDistribute)) + mid) / (total as u64); /* scale on remaining */
    let mut tmpTotal = mid;
    for s in 0..=(maxSymbolValue as usize) {
        if *norm.add(s) == NOT_YET_ASSIGNED {
            let end = tmpTotal.wrapping_add(*count.add(s) as u64 * rStep);
            let sStart = (tmpTotal >> vStepLog) as u32;
            let sEnd = (end >> vStepLog) as u32;
            let weight = sEnd.wrapping_sub(sStart);
            if weight < 1 {
                return ERROR(ZSTD_error_GENERIC);
            }
            *norm.add(s) = weight as i16;
            tmpTotal = end;
        }
    }
    return 0;
}

pub unsafe fn FSE_normalizeCount(
    mut normalizedCounter: *mut i16,
    mut tableLog: u32,
    mut count: *const u32,
    mut total: usize,
    mut maxSymbolValue: u32,
    mut useLowProbCount: bool,
) -> usize {
    /* Sanity checks */
    if tableLog == 0 {
        tableLog = FSE_DEFAULT_TABLELOG;
    }
    if tableLog < FSE_MIN_TABLELOG { return ERROR(ZSTD_error_GENERIC); } /* Unsupported size */
    if tableLog > FSE_MAX_TABLELOG { return ERROR(ZSTD_error_tableLog_tooLarge); } /* Unsupported size */
    if tableLog < FSE_minTableLog(total, maxSymbolValue) { return ERROR(ZSTD_error_GENERIC); } /* Too small tableLog, compression potentially impossible */
    
    #[rustfmt::skip]
    const rtbTable: [u32; 8] = [0, 473195, 504333, 520860, 550000, 700000, 750000, 830000];
    let lowProbCount: i16 = if useLowProbCount { -1 } else { 1 };
    let scale = 62_u64 - u64::from(tableLog);
    let step = (1_u64 << 62) / (total as u64);
    let vStep = 1_u64 << scale - 20;
    let mut stillToDistribute = (1 as i32) << tableLog;
    let mut largest: u32 = 0;
    let mut largestP: i16 = 0;
    let mut lowThreshold = (total >> tableLog) as u32;
    for s in  0..(maxSymbolValue as usize) {
        if *count.add(s) as usize == total {
            return 0; /* rle special case */
        }
        if *count.add(s) == 0 {
            *normalizedCounter.add(s) = 0;
        } else if *count.add(s) <= lowThreshold {
            *normalizedCounter.add(s) = lowProbCount;
            stillToDistribute -= 1;
        } else {
            let mut proba = (*count.add(s) as u64 * step >> scale) as i16;
            if proba < 8 {
                let mut restToBeat = vStep * rtbTable[proba as usize] as u64;
                proba += ((*count.add(s) as u64 * step)
                        .wrapping_sub((proba as u64) << scale) > restToBeat) as i16;
            }
            if proba > largestP {
                largestP = proba;
                largest = s as u32;
            }
            *normalizedCounter.add(s) = proba;
            stillToDistribute -= proba as i32;
        }
    }
    if -stillToDistribute
        >= *normalizedCounter.offset(largest as isize) as i32 >> 1
    {
        /* corner case, need another normalization method */
        let errorCode = FSE_normalizeM2(
            normalizedCounter,
            tableLog,
            count,
            total,
            maxSymbolValue,
            lowProbCount,
        );
        if ERR_isError(errorCode) {
            return errorCode;
        }
    } else {
        let ref mut fresh6 = *normalizedCounter.offset(largest as isize);
        *fresh6 += stillToDistribute as i16;
    }

// TODO? #if 0
//     {   /* Print Table (debug) */
//         U32 s;
//         U32 nTotal = 0;
//         for (s=0; s<=maxSymbolValue; s++)
//             RAWLOG(2, "%3i: %4i \n", s, normalizedCounter[s]);
//         for (s=0; s<=maxSymbolValue; s++)
//             nTotal += abs(normalizedCounter[s]);
//         if (nTotal != (1U<<tableLog))
//             RAWLOG(2, "Warning !!! Total == %u != %u !!!", nTotal, 1U<<tableLog);
//         getchar();
//     }
// #endif

    return tableLog as usize;
}

/* fake FSE_CTable, for rle input (always same symbol) */
pub unsafe fn FSE_buildCTable_rle(
    mut ct: *mut FSE_CTable,
    mut symbolValue: u8,
) -> usize {
    let mut ptr = ct as *mut std::ffi::c_void;
    let mut tableU16 = (ptr as *mut u16).offset(2);
    let mut FSCTptr = (ptr as *mut u32).offset(2) as *mut std::ffi::c_void;
    let mut symbolTT = FSCTptr as *mut FSE_symbolCompressionTransform;

    /* header */
    *tableU16.offset(-2) = 0;
    *tableU16.offset(-1) = symbolValue as u16;

    /* Build table */
    *tableU16.offset(0) = 0;
    *tableU16.offset(1) = 0; /* just in case */

    /* Build Symbol Transformation Table */
    (*symbolTT.offset(symbolValue as isize)).deltaNbBits = 0;
    (*symbolTT.offset(symbolValue as isize)).deltaFindState = 0;

    return 0;
}

unsafe fn FSE_compress_usingCTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut ct: *const FSE_CTable,
    fast: bool,
) -> usize {
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let mut ip = iend;

    let mut bitC = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: std::ptr::null_mut(),
        ptr: std::ptr::null_mut(),
        endPtr: std::ptr::null_mut(),
    };
    let mut CState1 = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null(),
        symbolTT: std::ptr::null(),
        stateLog: 0,
    };
    let mut CState2 = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null(),
        symbolTT: std::ptr::null(),
        stateLog: 0,
    };

    /* init */
    if srcSize <= 2 {
        return 0;
    }
    let initError = BIT_initCStream(&mut bitC, dst, dstSize);
    if ERR_isError(initError) {
        return 0; /* not enough space available to write a bitstream */
    }

    macro_rules! FSE_FLUSHBITS {
        ($s:expr) => {
            if fast {
                BIT_flushBitsFast($s)
            } else {
                BIT_flushBits($s)
            }
        };
    }

    if srcSize & 1 != 0 {
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState1, ct, *ip as u32);
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState2, ct, *ip as u32);
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as u32);
        FSE_FLUSHBITS!(&mut bitC);
    } else {
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState2, ct, *ip as u32);
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState1, ct, *ip as u32);
    }

    /* join to mod 4 */
    srcSize -= 2;
    if ::core::mem::size_of::<BitContainerType>() * 8
        > (FSE_MAX_TABLELOG as usize * 4 + 7) && srcSize & 2 != 0
    { /* test bit 2 */
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as u32);
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as u32);
        FSE_FLUSHBITS!(&mut bitC);
    }

    /* 2 or 4 encoding per loop */
    while ip > istart {
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as u32);

        if ::core::mem::size_of::<BitContainerType>() * 8
            < (FSE_MAX_TABLELOG as usize * 2 + 7)
        { /* this test must be static */
            FSE_FLUSHBITS!(&mut bitC);
        }

        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as u32);

        if ::core::mem::size_of::<BitContainerType>() * 8
            > (FSE_MAX_TABLELOG as usize * 4 + 7)
        { /* this test must be static */
            ip = ip.offset(-1);
            FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as u32);
            ip = ip.offset(-1);
            FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as u32);
        }

        FSE_FLUSHBITS!(&mut bitC);
    }

    FSE_flushCState(&mut bitC, &mut CState2);
    FSE_flushCState(&mut bitC, &mut CState1);
    return BIT_closeCStream(&mut bitC);
}

pub unsafe fn FSE_compress_usingCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut ct: *const FSE_CTable,
) -> usize {
    let fast = dstSize >= FSE_BLOCKBOUND(srcSize);

    if fast {
        return FSE_compress_usingCTable_generic(
            dst,
            dstSize,
            src,
            srcSize,
            ct,
            true,
        )
    } else {
        return FSE_compress_usingCTable_generic(
            dst,
            dstSize,
            src,
            srcSize,
            ct,
            false,
        )
    };
}

pub unsafe fn FSE_compressBound(size: usize) -> usize {
    return FSE_COMPRESSBOUND(size);
}
