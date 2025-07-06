use std::mem::{size_of, size_of_val};

use crate::zstd_h::*;
use crate::common::error::*;
use crate::common::zstd_internal_h::*;
use crate::common::mem::*;
use crate::common::fse_h::*;
use crate::common::bitstream_h::*;
use crate::compress::zstd_compress_internal::*;

// ==== from zstd_compress_sequences.h ====

pub type ZSTD_DefaultPolicy_e = std::ffi::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_DefaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_DefaultPolicy_e = 0;

// ==== end  zstd_compress_sequences.h ====

/**
 * -log2(x / 256) lookup table for x in [0, 256).
 * If x == 0: Return 0
 * Else: Return floor(-log2(x / 256) * 256)
 */
#[rustfmt::skip]
const kInverseProbabilityLog256: [std::ffi::c_uint; 256] = [
    0,    2048, 1792, 1642, 1536, 1453, 1386, 1329, 1280, 1236, 1197, 1162,
    1130, 1100, 1073, 1047, 1024, 1001, 980,  960,  941,  923,  906,  889,
    874,  859,  844,  830,  817,  804,  791,  779,  768,  756,  745,  734,
    724,  714,  704,  694,  685,  676,  667,  658,  650,  642,  633,  626,
    618,  610,  603,  595,  588,  581,  574,  567,  561,  554,  548,  542,
    535,  529,  523,  517,  512,  506,  500,  495,  489,  484,  478,  473,
    468,  463,  458,  453,  448,  443,  438,  434,  429,  424,  420,  415,
    411,  407,  402,  398,  394,  390,  386,  382,  377,  373,  370,  366,
    362,  358,  354,  350,  347,  343,  339,  336,  332,  329,  325,  322,
    318,  315,  311,  308,  305,  302,  298,  295,  292,  289,  286,  282,
    279,  276,  273,  270,  267,  264,  261,  258,  256,  253,  250,  247,
    244,  241,  239,  236,  233,  230,  228,  225,  222,  220,  217,  215,
    212,  209,  207,  204,  202,  199,  197,  194,  192,  190,  187,  185,
    182,  180,  178,  175,  173,  171,  168,  166,  164,  162,  159,  157,
    155,  153,  151,  149,  146,  144,  142,  140,  138,  136,  134,  132,
    130,  128,  126,  123,  121,  119,  117,  115,  114,  112,  110,  108,
    106,  104,  102,  100,  98,   96,   94,   93,   91,   89,   87,   85,
    83,   82,   80,   78,   76,   74,   73,   71,   69,   67,   66,   64,
    62,   61,   59,   57,   55,   54,   52,   50,   49,   47,   46,   44,
    42,   41,   39,   37,   36,   34,   33,   31,   30,   28,   26,   25,
    23,   22,   20,   19,   17,   16,   14,   13,   11,   10,   8,    7,
    5,    4,    2,    1,
];

unsafe fn ZSTD_getFSEMaxSymbolValue(
    mut ctable: *const FSE_CTable,
) -> std::ffi::c_uint {
    let mut ptr = ctable as *const std::ffi::c_void;
    let mut u16ptr = ptr as *const u16;
    let maxSymbolValue = MEM_read16(
        u16ptr.offset(1) as *const std::ffi::c_void,
    ) as u32;
    return maxSymbolValue;
}

/**
 * Returns true if we should use ncount=-1 else we should
 * use ncount=1 for low probability symbols instead.
 */
fn ZSTD_useLowProbCount(nbSeq: usize) -> bool {
    /* Heuristic: This should cover most blocks <= 16K and
     * start to fade out after 16K to about 32K depending on
     * compressibility.
     */
    nbSeq >= 2048
}

/**
 * Returns the cost in bytes of encoding the normalized count header.
 * Returns an error if any of the helper functions return an error.
 */
unsafe fn ZSTD_NCountCost(
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
    nbSeq: usize,
    FSELog: std::ffi::c_uint,
) -> usize {
    let mut wksp: [u8; FSE_NCOUNTBOUND] = [0; FSE_NCOUNTBOUND];
    let mut norm: [i16; MaxSeq as usize + 1] = [0; MaxSeq as usize + 1];
    let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
    FORWARD_IF_ERROR!(
        FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count, nbSeq, max,
        ZSTD_useLowProbCount(nbSeq)), ""
    );
    return FSE_writeNCount(
        wksp.as_mut_ptr() as *mut std::ffi::c_void,
        size_of_val(&wksp),
        norm.as_mut_ptr(),
        max,
        tableLog,
    );
}

/**
 * Returns the cost in bits of encoding the distribution described by count
 * using the entropy bound.
 */
unsafe fn ZSTD_entropyCost(
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
    total: usize,
) -> usize {
    let mut cost: std::ffi::c_uint = 0;
    for s in 0..=(max as usize) {
        let mut norm = (
            256_usize.wrapping_mul(*count.add(s) as usize) / total
        ) as std::ffi::c_uint;
        if *count.add(s) != 0 && norm == 0 {
            norm = 1;
        }
        debug_assert!((*count.add(s) as usize) < total);
        cost = cost
            .wrapping_add(
                (*count.add(s))
                    .wrapping_mul(kInverseProbabilityLog256[norm as usize]),
            );
    }
    return (cost >> 8) as usize;
}

/**
 * Returns the cost in bits of encoding the distribution in count using ctable.
 * Returns an error if ctable cannot represent all the symbols in count.
 */
pub unsafe fn ZSTD_fseBitCost(
    mut ctable: *const FSE_CTable,
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
) -> usize {
    let kAccuracyLog = 8;
    let mut cost: usize = 0;
    let mut cstate = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null(),
        symbolTT: std::ptr::null(),
        stateLog: 0,
    };
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max {
        DEBUGLOG!(5, "Repeat FSE_CTable has maxSymbolValue %u < %u",
                    ZSTD_getFSEMaxSymbolValue(ctable), max);
        return ERROR(ZSTD_error_GENERIC);
    }
    for s in 0..=(max as usize) {
        let tableLog = cstate.stateLog;
        let badCost = tableLog.wrapping_add(1) << kAccuracyLog;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s as u32, kAccuracyLog);
        if *count.add(s) == 0 {
            continue;
        }
        if bitCost >= badCost {
            DEBUGLOG!(5, "Repeat FSE_CTable has Prob[%u] == 0", s);
            return ERROR(ZSTD_error_GENERIC);
        }
        cost = cost.wrapping_add(*count.add(s) as usize * bitCost as usize);
    }
    return cost >> kAccuracyLog;
}

/**
 * Returns the cost in bits of encoding the distribution in count using the
 * table described by norm. The max symbol support by norm is assumed >= max.
 * norm must be valid for every symbol with non-zero probability in count.
 */
pub unsafe fn ZSTD_crossEntropyCost(
    mut norm: *const std::ffi::c_short,
    mut accuracyLog: std::ffi::c_uint,
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
) -> usize {
    let shift = (8 as std::ffi::c_uint).wrapping_sub(accuracyLog);
    let mut cost: usize = 0;
    debug_assert!(accuracyLog <= 8);
    for s in 0..=(max as usize) {
        let normAcc = if *norm.add(s) != -1 {
            *norm.add(s) as usize
        } else {
            1
        };
        let norm256 = normAcc << shift;
        cost = cost
            .wrapping_add(
                (*count.add(s))
                    .wrapping_mul(kInverseProbabilityLog256[norm256]) as usize,
            );
    }
    return cost >> 8;
}

pub unsafe fn ZSTD_selectEncodingType(
    mut repeatMode: *mut FSE_repeat,
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
    mostFrequent: usize,
    mut nbSeq: usize,
    FSELog: std::ffi::c_uint,
    mut prevCTable: *const FSE_CTable,
    mut defaultNorm: *const std::ffi::c_short,
    mut defaultNormLog: u32,
    isDefaultAllowed: ZSTD_DefaultPolicy_e,
    strategy: ZSTD_strategy,
) -> SymbolEncodingType_e {
    const _: () = assert!(ZSTD_defaultDisallowed == 0 && ZSTD_defaultAllowed != 0);
    if mostFrequent == nbSeq {
        *repeatMode = FSE_repeat_none;
        if isDefaultAllowed != 0 && nbSeq <= 2 {
            /* Prefer set_basic over set_rle when there are 2 or fewer symbols,
             * since RLE uses 1 byte, but set_basic uses 5-6 bits per symbol.
             * If basic encoding isn't possible, always choose RLE.
             */
            DEBUGLOG!(5, "Selected set_basic");
            return set_basic;
        }
        DEBUGLOG!(5, "Selected set_rle");
        return set_rle;
    }
    if strategy < ZSTD_lazy {
        if isDefaultAllowed == ZSTD_defaultAllowed {
            let staticFse_nbSeq_max = 1000;
            let mult = (10 - strategy) as usize;
            let baseLog = 3;
            let dynamicFse_nbSeq_min = (1_usize << defaultNormLog) * mult >> baseLog; /* 28-36 for offset, 56-72 for lengths */
            debug_assert!(defaultNormLog >= 5 && defaultNormLog <= 6);  /* xx_DEFAULTNORMLOG */
            debug_assert!(mult <= 9 && mult >= 7);
            if *repeatMode == FSE_repeat_valid
                && nbSeq < staticFse_nbSeq_max
            {
                DEBUGLOG!(5, "Selected set_repeat");
                return set_repeat;
            }
            if nbSeq < dynamicFse_nbSeq_min
                || mostFrequent < (nbSeq >> (defaultNormLog - 1))
            {
                DEBUGLOG!(5, "Selected set_basic");
                /* The format allows default tables to be repeated, but it isn't useful.
                 * When using simple heuristics to select encoding type, we don't want
                 * to confuse these tables with dictionaries. When running more careful
                 * analysis, we don't need to waste time checking both repeating tables
                 * and default tables.
                 */
                *repeatMode = FSE_repeat_none;
                return set_basic;
            }
        }
    } else {
        let basicCost = if isDefaultAllowed == ZSTD_defaultAllowed {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count, max)
        } else {
            ERROR(ZSTD_error_GENERIC)
        };
        let repeatCost = if *repeatMode != FSE_repeat_none {
            ZSTD_fseBitCost(prevCTable, count, max)
        } else {
            ERROR(ZSTD_error_GENERIC)
        };
        let NCountCost = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost = (NCountCost << 3) + ZSTD_entropyCost(count, max, nbSeq);

        if isDefaultAllowed == ZSTD_defaultAllowed {
            debug_assert!(!ERR_isError(basicCost));
            debug_assert!(!(*repeatMode == FSE_repeat_valid && ERR_isError(repeatCost)));
        }
        debug_assert!(!ERR_isError(NCountCost));
        debug_assert!(compressedCost < ERROR(ZSTD_error_maxCode));
        DEBUGLOG!(5, "Estimated bit costs: basic=%u\trepeat=%u\tcompressed=%u",
            basicCost, repeatCost, compressedCost);
        if basicCost <= repeatCost && basicCost <= compressedCost {
            DEBUGLOG!(5, "Selected set_basic");
            debug_assert!(isDefaultAllowed == ZSTD_defaultAllowed);
            *repeatMode = FSE_repeat_none;
            return set_basic;
        }
        if repeatCost <= compressedCost {
            DEBUGLOG!(5, "Selected set_repeat");
            debug_assert!(!ERR_isError(repeatCost));
            return set_repeat;
        }
        debug_assert!(compressedCost < basicCost && compressedCost < repeatCost);
    }
    DEBUGLOG!(5, "Selected set_compressed");
    *repeatMode = FSE_repeat_check;
    return set_compressed;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_BuildCTableWksp {
    pub norm: [i16; MaxSeq as usize + 1],
    pub wksp: [u32; FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(MaxSeq, MaxFSELog)],
}

pub unsafe fn ZSTD_buildCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut nextCTable: *mut FSE_CTable,
    mut FSELog: u32,
    mut type_0: SymbolEncodingType_e,
    mut count: *mut std::ffi::c_uint,
    mut max: u32,
    mut codeTable: *const u8,
    mut nbSeq: usize,
    mut defaultNorm: *const i16,
    mut defaultNormLog: u32,
    mut defaultMax: u32,
    mut prevCTable: *const FSE_CTable,
    mut prevCTableSize: usize,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWorkspaceSize: usize,
) -> usize {
    let mut op = dst as *mut u8;
    let oend: *const u8 = op.add(dstCapacity);
    DEBUGLOG!(6, "ZSTD_buildCTable (dstCapacity=%u)", dstCapacity);

    match type_0 as std::ffi::c_uint {
        set_rle => {
            FORWARD_IF_ERROR!(
                FSE_buildCTable_rle(nextCTable, max as u8), ""
            );
            RETURN_ERROR_IF!(dstCapacity == 0, ZSTD_error_dstSize_tooSmall, "not enough space");
            *op = *codeTable.offset(0);
            return 1;
        }
        set_repeat => {
            libc::memcpy(nextCTable.cast(), prevCTable.cast(), prevCTableSize);
            return 0;
        }
        set_basic => {
            FORWARD_IF_ERROR!(
                FSE_buildCTable_wksp(nextCTable, defaultNorm, defaultMax, defaultNormLog,
                entropyWorkspace, entropyWorkspaceSize), ""
            ); /* note : could be pre-calculated */
            return 0;
        }
        set_compressed => {
            let mut wksp = entropyWorkspace as *mut ZSTD_BuildCTableWksp;
            let mut nbSeq_1 = nbSeq;
            let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
            if *count.offset(
                *codeTable.add(nbSeq - 1) as isize,
            ) > 1 {
                let ref mut fresh0 = *count.offset(
                    *codeTable.add(nbSeq - 1) as isize,
                );
                *fresh0 = (*fresh0).wrapping_sub(1);
                nbSeq_1 = nbSeq_1.wrapping_sub(1);
            }
            debug_assert!(nbSeq_1 > 1);
            debug_assert!(entropyWorkspaceSize >= size_of::<ZSTD_BuildCTableWksp>());
            FORWARD_IF_ERROR!(
                FSE_normalizeCount((*wksp).norm.as_mut_ptr(), tableLog, count, nbSeq_1, max,
                ZSTD_useLowProbCount(nbSeq_1)), "FSE_normalizeCount failed"
            );
            debug_assert!(oend >= op);
            let NCountSize = FSE_writeNCount(
                op as *mut std::ffi::c_void,
                oend.offset_from(op) as usize,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
            ); /* overflow protected */
            FORWARD_IF_ERROR!(NCountSize, "FSE_writeNCount failed");
            FORWARD_IF_ERROR!(
                FSE_buildCTable_wksp(nextCTable, (*wksp).norm.as_mut_ptr(), max, tableLog, (*wksp).wksp.as_mut_ptr().cast(), size_of_val(&(*wksp).wksp)), "FSE_buildCTable_wksp failed"
            );
            return NCountSize;
        }
        _ => {
            debug_assert!(false, "impossible to reach");
            return ERROR(ZSTD_error_GENERIC);
        }
    };
}

#[inline(always)]
unsafe fn ZSTD_encodeSequences_body(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: usize,
    mut longOffsets: std::ffi::c_int,
) -> usize {
    let mut blockStream = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: std::ptr::null_mut(),
        ptr: std::ptr::null_mut(),
        endPtr: std::ptr::null_mut(),
    };
    let mut stateMatchLength = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null(),
        symbolTT: std::ptr::null(),
        stateLog: 0,
    };
    let mut stateOffsetBits = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null(),
        symbolTT: std::ptr::null(),
        stateLog: 0,
    };
    let mut stateLitLength = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null(),
        symbolTT: std::ptr::null(),
        stateLog: 0,
    };

    RETURN_ERROR_IF!(ERR_isError(BIT_initCStream(&mut blockStream, dst, dstCapacity)), ZSTD_error_dstSize_tooSmall, "not enough space remaining");
    DEBUGLOG!(6, "available space for bitstream : %i  (dstCapacity=%u)",
                blockStream.endPtr.offset_from(blockStream.startPtr),
                dstCapacity);

    /* first symbols */
    FSE_initCState2(
        &mut stateMatchLength,
        CTable_MatchLength,
        u32::from(*mlCodeTable.add(nbSeq - 1)),
    );
    FSE_initCState2(
        &mut stateOffsetBits,
        CTable_OffsetBits,
        u32::from(*ofCodeTable.add(nbSeq - 1)),
    );
    FSE_initCState2(
        &mut stateLitLength,
        CTable_LitLength,
        u32::from(*llCodeTable.add(nbSeq - 1)),
    );
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq - 1)).litLength as BitContainerType,
        LL_bits[*llCodeTable.add(nbSeq - 1) as usize] as std::ffi::c_uint,
    );
    if MEM_32bits {
        BIT_flushBits(&mut blockStream);
    }
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq - 1)).mlBase as BitContainerType,
        ML_bits[*mlCodeTable.add(nbSeq - 1) as usize] as std::ffi::c_uint,
    );
    if MEM_32bits {
        BIT_flushBits(&mut blockStream);
    }
    if longOffsets != 0 {
        let ofBits = *ofCodeTable.add(nbSeq - 1) as u32;
        let extraBits = ofBits - std::cmp::min(ofBits, STREAM_ACCUMULATOR_MIN - 1);
        if extraBits != 0 {
            BIT_addBits(
                &mut blockStream,
                (*sequences.add(nbSeq - 1)).offBase as BitContainerType,
                extraBits,
            );
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            ((*sequences.add(nbSeq - 1)).offBase >> extraBits) as BitContainerType,
            ofBits.wrapping_sub(extraBits),
        );
    } else {
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(nbSeq - 1)).offBase as BitContainerType,
            *ofCodeTable.add(nbSeq - 1) as std::ffi::c_uint,
        );
    }
    BIT_flushBits(&mut blockStream);

    let mut n: usize = 0;
    n = nbSeq.wrapping_sub(2);
    while n < nbSeq {
        let llCode = *llCodeTable.add(n);
        let ofCode = *ofCodeTable.add(n);
        let mlCode = *mlCodeTable.add(n);
        let llBits = LL_bits[llCode as usize] as u32;
        let ofBits_0 = ofCode as u32;
        let mlBits = ML_bits[mlCode as usize] as u32;
        DEBUGLOG!(6, "encoding: litlen:%2u - matchlen:%2u - offCode:%7u",
                    (*sequences.add(n)).litLength,
                    (*sequences.add(n)).mlBase as u32 + MINMATCH,
                    (*sequences.add(n)).offBase);
                                                /* 32b*/  /* 64b*/
                                                /* (7)*/  /* (7)*/
        FSE_encodeSymbol(                       /* 15 */  /* 15 */
            &mut blockStream,
            &mut stateOffsetBits,
            ofCode as std::ffi::c_uint,
        );
        FSE_encodeSymbol(                       /* 24 */  /* 24 */
            &mut blockStream,
            &mut stateMatchLength,
            mlCode as std::ffi::c_uint,
        );
        if MEM_32bits {
            BIT_flushBits(&mut blockStream);    /* (7)*/
        }
        FSE_encodeSymbol(                       /* 16 */  /* 33 */
            &mut blockStream,
            &mut stateLitLength,
            llCode as std::ffi::c_uint,
        );
        if MEM_32bits
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits)
                >= (64 - 7 - (LLFSELog + MLFSELog + OffFSELog))
        {
            BIT_flushBits(&mut blockStream);    /* (7)*/
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(n)).litLength as BitContainerType,
            llBits,
        );
        if MEM_32bits
            && llBits.wrapping_add(mlBits) > 24
        {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(n)).mlBase as BitContainerType,
            mlBits,
        );
        if MEM_32bits
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) > 56
        {
            BIT_flushBits(&mut blockStream);
        }
        if longOffsets != 0 {
            let extraBits_0 = ofBits_0 - std::cmp::min(ofBits_0, STREAM_ACCUMULATOR_MIN - 1);
            if extraBits_0 != 0 {
                BIT_addBits(
                    &mut blockStream,
                    (*sequences.add(n)).offBase as BitContainerType,
                    extraBits_0,
                );
                BIT_flushBits(&mut blockStream);/* (7)*/
            }
            BIT_addBits(
                &mut blockStream,
                ((*sequences.add(n)).offBase >> extraBits_0)
                    as BitContainerType,
                ofBits_0.wrapping_sub(extraBits_0),
            );                                  /* 31 */
        } else {
            BIT_addBits(
                &mut blockStream,
                (*sequences.add(n)).offBase as BitContainerType,
                ofBits_0,
            );
        }
        BIT_flushBits(&mut blockStream);        /* (7)*/
        DEBUGLOG!(7, "remaining space : %i", blockStream.endPtr.offset_from(blockStream.ptr));

        n = n.wrapping_sub(1); /* intentional underflow */
    }
    
    DEBUGLOG!(6, "ZSTD_encodeSequences: flushing ML state with %u bits", stateMatchLength.stateLog);
    FSE_flushCState(&mut blockStream, &mut stateMatchLength);
    DEBUGLOG!(6, "ZSTD_encodeSequences: flushing Off state with %u bits", stateOffsetBits.stateLog);
    FSE_flushCState(&mut blockStream, &mut stateOffsetBits);
    DEBUGLOG!(6, "ZSTD_encodeSequences: flushing LL state with %u bits", stateLitLength.stateLog);
    FSE_flushCState(&mut blockStream, &mut stateLitLength);
    
    let streamSize = BIT_closeCStream(&mut blockStream);
    RETURN_ERROR_IF!(streamSize == 0, ZSTD_error_dstSize_tooSmall);
    return streamSize;
}

unsafe fn ZSTD_encodeSequences_default(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: usize,
    mut longOffsets: std::ffi::c_int,
) -> usize {
    return ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    );
}

// TODO #if DYNAMIC_BMI2
unsafe fn ZSTD_encodeSequences_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: usize,
    mut longOffsets: std::ffi::c_int,
) -> usize {
    return ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    );
}

pub unsafe fn ZSTD_encodeSequences(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: usize,
    mut longOffsets: std::ffi::c_int,
    mut bmi2: std::ffi::c_int,
) -> usize {
    DEBUGLOG!(5, "ZSTD_encodeSequences: dstCapacity = %u", dstCapacity);
    // TODO #if DYNAMIC_BMI2
    if bmi2 != 0 {
        return ZSTD_encodeSequences_bmi2(
            dst,
            dstCapacity,
            CTable_MatchLength,
            mlCodeTable,
            CTable_OffsetBits,
            ofCodeTable,
            CTable_LitLength,
            llCodeTable,
            sequences,
            nbSeq,
            longOffsets,
        );
    }
    return ZSTD_encodeSequences_default(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    );
}
