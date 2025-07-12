use std::ffi::{c_char, c_void};
use std::mem::{size_of, size_of_val, align_of};

use crate::zstd_h::*;
use crate::common::fse_h::*;
use crate::common::huf_h::*;
use crate::common::error::*;
use crate::common::mem::*;
use crate::common::bits::*;
use crate::compress::hist::*;


/* **************************************************************
*  Required declarations
****************************************************************/
pub type nodeElt = nodeElt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: u32,
    pub parent: u16,
    pub byte: u8,
    pub nbBits: u8,
}

/* **************************************************************
*  Debug Traces
****************************************************************/

// #if DEBUGLEVEL >= 2

// static size_t showU32(const U32* arr, size_t size)
// {
//     size_t u;
//     for (u=0; u<size; u++) {
//         RAWLOG(6, " %u", arr[u]); (void)arr;
//     }
//     RAWLOG(6, " \n");
//     return size;
// }

// static size_t HUF_getNbBits(HUF_CElt elt);

// static size_t showCTableBits(const HUF_CElt* ctable, size_t size)
// {
//     size_t u;
//     for (u=0; u<size; u++) {
//         RAWLOG(6, " %zu", HUF_getNbBits(ctable[u])); (void)ctable;
//     }
//     RAWLOG(6, " \n");
//     return size;

// }

// static size_t showHNodeSymbols(const nodeElt* hnode, size_t size)
// {
//     size_t u;
//     for (u=0; u<size; u++) {
//         RAWLOG(6, " %u", hnode[u].byte); (void)hnode;
//     }
//     RAWLOG(6, " \n");
//     return size;
// }

// static size_t showHNodeBits(const nodeElt* hnode, size_t size)
// {
//     size_t u;
//     for (u=0; u<size; u++) {
//         RAWLOG(6, " %u", hnode[u].nbBits); (void)hnode;
//     }
//     RAWLOG(6, " \n");
//     return size;
// }

// #endif

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CTableHeader {
    pub tableLog: u8,
    pub maxSymbolValue: u8,
    pub unused: [u8; 6],
}

pub type HUF_repeat = u32;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;

/* *******************************************************
*  HUF : Huffman block compression
*********************************************************/
const HUF_WORKSPACE_MAX_ALIGNMENT: usize = 8;

unsafe fn HUF_alignUpWorkspace(
    mut workspace: *mut c_void,
    mut workspaceSizePtr: *mut usize,
    mut align: usize,
) -> *mut c_void {
    let mask = align - 1;
    let rem = workspace as usize & mask;
    let add = align.wrapping_sub(rem) & mask;
    let aligned = workspace.byte_add(add);
    debug_assert!((align & (align - 1)) == 0); /* pow 2 */
    debug_assert!(align <= HUF_WORKSPACE_MAX_ALIGNMENT);
    if *workspaceSizePtr >= add {
        debug_assert!(add < align);
        debug_assert!(((aligned as usize) & mask) == 0);
        *workspaceSizePtr -= add;
        return aligned;
    } else {
        *workspaceSizePtr = 0;
        return std::ptr::null_mut();
    };
}

/* HUF_compressWeights() :
 * Same as FSE_compress(), but dedicated to huff0's weights compression.
 * The use case needs much less stack memory.
 * Note : all elements within weightTable are supposed to be <= HUF_TABLELOG_MAX.
 */
pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: u32 = 6;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CompressWeightsWksp {
    pub CTable: [FSE_CTable; FSE_CTABLE_SIZE_U32(MAX_FSE_TABLELOG_FOR_HUFF_HEADER, HUF_TABLELOG_MAX)],
    pub scratchBuffer: [u32; FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(HUF_TABLELOG_MAX, MAX_FSE_TABLELOG_FOR_HUFF_HEADER)],
    pub count: [u32; (HUF_TABLELOG_MAX as usize)+1],
    pub norm: [i16; (HUF_TABLELOG_MAX as usize)+1],
}

unsafe fn HUF_compressWeights(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut weightTable: *const c_void,
    mut wtSize: usize,
    mut workspace: *mut c_void,
    mut workspaceSize: usize,
) -> usize {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(dstSize);

    let mut maxSymbolValue = HUF_TABLELOG_MAX;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        align_of::<u32>(),
    ) as *mut HUF_CompressWeightsWksp;

    if workspaceSize < size_of::<HUF_CompressWeightsWksp>() { return ERROR(ZSTD_error_GENERIC); }
    
    /* init conditions */
    if wtSize <= 1 {
        return 0; /* Not compressible */
    }

    /* Scan input and build symbol stats */
    let maxCount = HIST_count_simple(
        ((*wksp).count).as_mut_ptr(),
        &mut maxSymbolValue,
        weightTable,
        wtSize,
    );
    if maxCount as usize == wtSize { /* only a single symbol in src : rle */
        return 1;
    }
    if maxCount == 1 { /* each symbol present maximum once => not compressible */
        return 0;
    }
    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    FORWARD_IF_ERROR!(FSE_normalizeCount(
        ((*wksp).norm).as_mut_ptr(),
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        /* useLowProbCount */ false,
    ));

    /* Write table description header */
    let hSize = FSE_writeNCount(
        op as *mut c_void,
        oend.offset_from(op) as usize,
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(hSize) {
        return hSize;
    }
    op = op.add(hSize);

    /* Compress */
    FORWARD_IF_ERROR!(FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut c_void,
        size_of_val(&(*wksp).scratchBuffer),
    ));
    let cSize = FSE_compress_usingCTable(
        op as *mut c_void,
        oend.offset_from(op) as usize,
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 {
        return 0; /* not enough space for compressed data */
    }
    op = op.add(cSize);

    return op.offset_from(ostart) as usize;
}

fn HUF_getNbBits(elt: HUF_CElt) -> usize {
    elt & 0xff
}
fn HUF_getNbBitsFast(elt: HUF_CElt) -> usize {
    elt
}
fn HUF_getValue(elt: HUF_CElt) -> usize {
    elt & !(0xff as HUF_CElt)
}
fn HUF_getValueFast(elt: HUF_CElt) -> usize {
    elt
}
unsafe fn HUF_setNbBits(elt: *mut HUF_CElt, nbBits: usize) {
    debug_assert!(nbBits <= HUF_TABLELOG_ABSOLUTEMAX as usize);
    *elt = nbBits;
}

unsafe fn HUF_setValue(elt: *mut HUF_CElt, value: usize) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        debug_assert!((value >> nbBits) == 0);
        *elt |= value << (size_of::<HUF_CElt>() * 8 - nbBits);
    }
}

pub unsafe fn HUF_readCTableHeader(
    mut ctable: *const HUF_CElt,
) -> HUF_CTableHeader {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    libc::memcpy(
        &mut header as *mut HUF_CTableHeader as *mut c_void,
        ctable as *const c_void,
        size_of::<HUF_CTableHeader>(),
    );
    return header;
}

unsafe fn HUF_writeCTableHeader(
    mut ctable: *mut HUF_CElt,
    mut tableLog: u32,
    mut maxSymbolValue: u32,
) {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    const _: () = assert!(size_of::<HUF_CElt>() == size_of::<HUF_CTableHeader>());
    libc::memset(
        &mut header as *mut HUF_CTableHeader as *mut c_void,
        0,
        size_of::<HUF_CTableHeader>(),
    );
    debug_assert!(tableLog < 256);
    header.tableLog = tableLog as u8;
    debug_assert!(maxSymbolValue < 256);
    header.maxSymbolValue = maxSymbolValue as u8;
    libc::memcpy(
        ctable as *mut c_void,
        &mut header as *mut HUF_CTableHeader as *const c_void,
        size_of::<HUF_CTableHeader>(),
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [u8; HUF_TABLELOG_MAX as usize + 1], /* precomputed conversion table */
    pub huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize],
}

pub unsafe fn HUF_writeCTable_wksp(
    mut dst: *mut c_void,
    mut maxDstSize: usize,
    mut CTable: *const HUF_CElt,
    mut maxSymbolValue: u32,
    mut huffLog: u32,
    mut workspace: *mut c_void,
    mut workspaceSize: usize,
) -> usize {
    let ct = CTable.offset(1);
    let mut op = dst as *mut u8;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        align_of::<u32>(),
    ) as *mut HUF_WriteCTableWksp;

    const _: () = assert!(HUF_CTABLE_WORKSPACE_SIZE >= size_of::<HUF_WriteCTableWksp>());

    debug_assert!(u32::from(HUF_readCTableHeader(CTable).maxSymbolValue) == maxSymbolValue);
    debug_assert!(u32::from(HUF_readCTableHeader(CTable).tableLog) == huffLog);

    /* check conditions */
    if workspaceSize < size_of::<HUF_WriteCTableWksp>() { return ERROR(ZSTD_error_GENERIC); }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX { return ERROR(ZSTD_error_maxSymbolValue_tooLarge); }

    /* convert to weight */
    (*wksp).bitsToWeight[0] = 0;
    for n in 1..huffLog+1 {
        (*wksp).bitsToWeight[n as usize] = (huffLog + 1 - n) as u8;
    }
    for n in 0..(maxSymbolValue as usize) {
        (*wksp).huffWeight[n] = (*wksp).bitsToWeight[HUF_getNbBits(*ct.add(n))];
    }

    /* attempt weights compression by FSE */
    if maxDstSize < 1 { return ERROR(ZSTD_error_dstSize_tooSmall); }
    let hSize = HUF_compressWeights(
        op.offset(1) as *mut c_void,
        maxDstSize.wrapping_sub(1),
        ((*wksp).huffWeight).as_mut_ptr() as *const c_void,
        maxSymbolValue as usize,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut c_void,
        size_of::<HUF_CompressWeightsWksp>(),
    );
    if ERR_isError(hSize) {
        return hSize;
    }
    if (hSize > 1) & (hSize < (maxSymbolValue as usize)/2) { /* FSE compressed */
        *op.offset(0) = hSize as u8;
        return hSize+1;
    }

    /* write raw values as 4-bits (max : 15) */
    if maxSymbolValue > (256 - 128) { return ERROR(ZSTD_error_GENERIC); } /* should not happen : likely means source cannot be compressed */
    if ((maxSymbolValue+1)/2 + 1) as usize > maxDstSize { return ERROR(ZSTD_error_dstSize_tooSmall); } /* not enough space within dst buffer */
    *op.offset(0) = (128 /*special case*/ + (maxSymbolValue-1)) as u8;
    (*wksp).huffWeight[maxSymbolValue as usize] = 0; /* to be sure it doesn't cause msan issue in final combination */
    let mut n = 0;
    while n < (maxSymbolValue as usize) {
        *op.add(n/2+1) = ((u32::from((*wksp).huffWeight[n]) << 4)
            + u32::from((*wksp).huffWeight[n+1])) as u8;
        n += 2;
    }
    return ((maxSymbolValue+1)/2 + 1) as usize;
}

pub unsafe fn HUF_readCTable(
    mut CTable: *mut HUF_CElt,
    mut maxSymbolValuePtr: *mut u32,
    mut src: *const c_void,
    mut srcSize: usize,
    mut hasZeroWeights: *mut u32,
) -> usize {
    let mut huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize + 1] = [0; HUF_SYMBOLVALUE_MAX as usize + 1];
    let mut rankVal: [u32; HUF_TABLELOG_ABSOLUTEMAX as usize + 1] = [0; HUF_TABLELOG_ABSOLUTEMAX as usize + 1];
    let mut tableLog: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let ct = CTable.offset(1);

    /* get symbol weights */
    let readSize = HUF_readStats(
        huffWeight.as_mut_ptr(),
        (255 as i32 + 1 as i32) as usize,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if ERR_isError(readSize) {
        return readSize;
    }
    *hasZeroWeights = (rankVal[0] > 0) as u32;

    /* check result */
    if tableLog > HUF_TABLELOG_MAX { return ERROR(ZSTD_error_tableLog_tooLarge); }
    if nbSymbols > (*maxSymbolValuePtr + 1) { return ERROR(ZSTD_error_maxSymbolValue_tooSmall); }
    
    *maxSymbolValuePtr = nbSymbols - 1;
    
    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);
    
    /* Prepare base value per rank */
    let mut nextRankStart: u32 = 0;
    for n in 1..(tableLog as usize) {
        let mut curr = nextRankStart;
        nextRankStart += rankVal[n] << n.wrapping_sub(1);
        rankVal[n] = curr;
    }

    /* fill nbBits */
    for n_0 in 0..(nbSymbols as usize) {
        let w = huffWeight[n_0] as u32;
        HUF_setNbBits(
            ct.add(n_0),
            ((tableLog + 1 - w) as u8 & 0_u8.wrapping_sub((w != 0) as u8)) as usize,
        );
    }

    /* fill val */
    let mut nbPerRank: [u16; HUF_TABLELOG_MAX as usize + 2] = [0; HUF_TABLELOG_MAX as usize + 2]; /* support w=0=>n=tableLog+1 */
    let mut valPerRank: [u16; HUF_TABLELOG_MAX as usize + 2] = [0; HUF_TABLELOG_MAX as usize + 2];
    for n_1 in 0..(nbSymbols as usize) {
        nbPerRank[HUF_getNbBits(*ct.add(n_1))] += 1;
    }
    /* determine stating value per rank */
    valPerRank[tableLog as usize + 1] = 0; /* for w==0 */
    let mut min: u16 = 0;
    let mut n_2 = tableLog as usize;
    while n_2 > 0 { /* start at n=tablelog <-> w=1 */
        valPerRank[n_2] = min; /* get starting value within each rank */
        min += nbPerRank[n_2];
        min >>= 1;
        n_2 -= 1;
    }
    /* assign value within rank, symbol order */
    for n_3 in 0..(nbSymbols as usize) {
        let fresh0 = &mut valPerRank[HUF_getNbBits(*ct.add(n_3))];
        HUF_setValue(ct.add(n_3), *fresh0 as usize);
        *fresh0 += 1;
    }

    return readSize;
}

pub unsafe fn HUF_getNbBitsFromCTable(
    CTable: *const HUF_CElt,
    symbolValue: u32,
) -> u32 {
    let ct = CTable.offset(1);
    debug_assert!(symbolValue <= HUF_SYMBOLVALUE_MAX);
    if symbolValue > u32::from(HUF_readCTableHeader(CTable).maxSymbolValue) {
        return 0;
    }
    HUF_getNbBits(*ct.offset(symbolValue as isize)) as u32
}

/**
 * HUF_setMaxHeight():
 * Try to enforce @targetNbBits on the Huffman tree described in @huffNode.
 *
 * It attempts to convert all nodes with nbBits > @targetNbBits
 * to employ @targetNbBits instead. Then it adjusts the tree
 * so that it remains a valid canonical Huffman tree.
 *
 * @pre               The sum of the ranks of each symbol == 2^largestBits,
 *                    where largestBits == huffNode[lastNonNull].nbBits.
 * @post              The sum of the ranks of each symbol == 2^largestBits,
 *                    where largestBits is the return value (expected <= targetNbBits).
 *
 * @param huffNode    The Huffman tree modified in place to enforce targetNbBits.
 *                    It's presumed sorted, from most frequent to rarest symbol.
 * @param lastNonNull The symbol with the lowest count in the Huffman tree.
 * @param targetNbBits  The allowed number of bits, which the Huffman tree
 *                    may not respect. After this function the Huffman tree will
 *                    respect targetNbBits.
 * @return            The maximum number of bits of the Huffman tree after adjustment.
 */
unsafe fn HUF_setMaxHeight(
    mut huffNode: *mut nodeElt,
    mut lastNonNull: u32,
    mut targetNbBits: u32,
) -> u32 {
    let largestBits = u32::from((*huffNode.offset(lastNonNull as isize)).nbBits);
    /* early exit : no elt > targetNbBits, so the tree is already valid. */
    if largestBits <= targetNbBits {
        return largestBits;
    }

    DEBUGLOG!(5, "HUF_setMaxHeight (targetNbBits = {})", targetNbBits);

    /* there are several too large elements (at least >= 2) */
    let mut totalCost: i32 = 0;
    let baseCost = 1_u32 << (largestBits.wrapping_sub(targetNbBits));
    let mut n = lastNonNull as isize;

    /* Adjust any ranks > targetNbBits to targetNbBits.
     * Compute totalCost, which is how far the sum of the ranks is
     * we are over 2^largestBits after adjust the offending ranks.
     */
    while u32::from((*huffNode.offset(n)).nbBits) > targetNbBits {
        totalCost += (baseCost as i32)
            .wrapping_sub(1_i32 << (largestBits - u32::from((*huffNode.offset(n)).nbBits)));
        (*huffNode.offset(n)).nbBits = targetNbBits as u8;
        n -= 1;
    }
    /* n stops at huffNode[n].nbBits <= targetNbBits */
    debug_assert!(u32::from((*huffNode.offset(n)).nbBits) <= targetNbBits);
    /* n end at index of smallest symbol using < targetNbBits */
    while u32::from((*huffNode.offset(n)).nbBits) == targetNbBits {
        n -= 1;
    }

    /* renorm totalCost from 2^largestBits to 2^targetNbBits
     * note : totalCost is necessarily a multiple of baseCost */
    debug_assert!(((totalCost as u32) & (baseCost - 1)) == 0);
    totalCost >>= largestBits.wrapping_sub(targetNbBits);
    debug_assert!(totalCost > 0);

    /* repay normalized cost */
    {
        const noSymbol: u32 = 0xF0F0F0F0;
        let mut rankLast: [u32; HUF_TABLELOG_MAX as usize + 2] = [0; HUF_TABLELOG_MAX as usize + 2];

        /* Get pos of last (smallest = lowest cum. count) symbol per rank */
        libc::memset(
            rankLast.as_mut_ptr() as *mut _,
            0xF0,
            std::mem::size_of_val(&rankLast),
        );

        let mut currentNbBits = targetNbBits;
        let mut pos = n;
        while pos >= 0 {
            if u32::from((*huffNode.offset(pos)).nbBits) >= currentNbBits {
                pos -= 1;
                continue;
            }
            currentNbBits = u32::from((*huffNode.offset(pos)).nbBits); /* < targetNbBits */
            rankLast[targetNbBits.wrapping_sub(currentNbBits) as usize] = pos as u32;
            pos -= 1;
        }

        while totalCost > 0 {
            /* Try to reduce the next power of 2 above totalCost because we
             * gain back half the rank.
             */
            let mut nBitsToDecrease = ZSTD_highbit32(totalCost as u32) as usize + 1;
            debug_assert!(nBitsToDecrease <= (HUF_TABLELOG_MAX + 1) as usize);
            while nBitsToDecrease > 1 {
                let highPos = rankLast[nBitsToDecrease];
                let lowPos = rankLast[nBitsToDecrease - 1];
                if highPos == noSymbol {
                    nBitsToDecrease -= 1;
                    continue;
                }
                /* Decrease highPos if no symbols of lowPos or if it is
                 * not cheaper to remove 2 lowPos than highPos.
                 */
                if lowPos == noSymbol {
                    break;
                }
                let highTotal = (*huffNode.offset(highPos as isize)).count;
                let lowTotal = 2 * (*huffNode.offset(lowPos as isize)).count;
                if highTotal <= lowTotal {
                    break;
                }
                nBitsToDecrease -= 1;
            }
            /* only triggered when no more rank 1 symbol left => find closest one (note : there is necessarily at least one !) */
            debug_assert!(rankLast[nBitsToDecrease] != noSymbol || nBitsToDecrease == 1);
            /* HUF_MAX_TABLELOG test just to please gcc 5+; but it should not be necessary */
            while (nBitsToDecrease <= HUF_TABLELOG_MAX as usize) && (rankLast[nBitsToDecrease] == noSymbol) {
                nBitsToDecrease += 1;
            }
            debug_assert!(rankLast[nBitsToDecrease] != noSymbol);
            /* Increase the number of bits to gain back half the rank cost. */
            totalCost -= 1_i32 << (nBitsToDecrease - 1);
            (*huffNode.offset(rankLast[nBitsToDecrease] as isize)).nbBits += 1;

            /* Fix up the new rank.
             * If the new rank was empty, this symbol is now its smallest.
             * Otherwise, this symbol will be the largest in the new rank so no adjustment.
             */
            if rankLast[nBitsToDecrease - 1] == noSymbol {
                rankLast[nBitsToDecrease - 1] = rankLast[nBitsToDecrease];
            }

            /* Fix up the old rank.
             * If the symbol was at position 0, meaning it was the highest weight symbol in the tree,
             * it must be the only symbol in its rank, so the old rank now has no symbols.
             * Otherwise, since the Huffman nodes are sorted by count, the previous position is now
             * the smallest node in the rank. If the previous position belongs to a different rank,
             * then the rank is now empty.
             */
            if rankLast[nBitsToDecrease] == 0 { /* special case, reached largest symbol */
                rankLast[nBitsToDecrease] = noSymbol;
            } else {
                rankLast[nBitsToDecrease] = rankLast[nBitsToDecrease].wrapping_sub(1);
                if u32::from((*huffNode.offset(rankLast[nBitsToDecrease] as isize)).nbBits)
                    != targetNbBits.wrapping_sub(nBitsToDecrease as u32)
                {
                    rankLast[nBitsToDecrease] = noSymbol; /* this rank is now empty */
                }
            }
        } /* while (totalCost > 0) */

        /* If we've removed too much weight, then we have to add it back.
         * To avoid overshooting again, we only adjust the smallest rank.
         * We take the largest nodes from the lowest rank 0 and move them
         * to rank 1. There's guaranteed to be enough rank 0 symbols because
         * TODO.
         */
        while totalCost < 0 { /* Sometimes, cost correction overshoot */
            /* special case : no rank 1 symbol (using targetNbBits-1);
             * let's create one from largest rank 0 (using targetNbBits).
             */
            if rankLast[1] == noSymbol {
                while u32::from((*huffNode.offset(n)).nbBits) == targetNbBits {
                    n -= 1;
                }
                (*huffNode.offset((n + 1) as isize)).nbBits -= 1;
                debug_assert!(n >= 0);
                rankLast[1] = (n + 1) as u32;
                totalCost += 1;
                continue;
            }
            (*huffNode.offset((rankLast[1] + 1) as isize)).nbBits -= 1;
            rankLast[1] += 1;
            totalCost += 1;
        }
    } /* repay normalized cost */

    targetNbBits
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rankPos {
    pub base: u16,
    pub curr: u16,
}

pub type huffNodeTable = [nodeElt; 2 * (HUF_SYMBOLVALUE_MAX as usize + 1)];

/* Number of buckets available for HUF_sort() */
const RANK_POSITION_TABLE_SIZE: usize = 192;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; RANK_POSITION_TABLE_SIZE],
}

/* RANK_POSITION_DISTINCT_COUNT_CUTOFF == Cutoff point in HUF_sort() buckets for which we use log2 bucketing.
 * Strategy is to use as many buckets as possible for representing distinct
 * counts while using the remainder to represent all "large" counts.
 *
 * To satisfy this requirement for 192 buckets, we can do the following:
 * Let buckets 0-166 represent distinct counts of [0, 166]
 * Let buckets 166 to 192 represent all remaining counts up to RANK_POSITION_MAX_COUNT_LOG using log2 bucketing.
 */
const RANK_POSITION_MAX_COUNT_LOG: u32 = 32;
const RANK_POSITION_LOG_BUCKETS_BEGIN: u32 = (RANK_POSITION_TABLE_SIZE as u32 - 1) - RANK_POSITION_MAX_COUNT_LOG - 1 /* == 158 */;
const RANK_POSITION_DISTINCT_COUNT_CUTOFF: u32 = RANK_POSITION_LOG_BUCKETS_BEGIN + ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN) /* == 166 */;

/* Return the appropriate bucket index for a given count. See definition of
 * RANK_POSITION_DISTINCT_COUNT_CUTOFF for explanation of bucketing strategy.
 */
const fn HUF_getIndex(count: u32) -> u32 {
    if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        ZSTD_highbit32(count) + RANK_POSITION_LOG_BUCKETS_BEGIN
    }
}

/* Helper swap function for HUF_quickSortPartition() */
unsafe fn HUF_swapNodes(mut a: *mut nodeElt, mut b: *mut nodeElt) {
    std::ptr::swap(a, b);
}

/* Returns 0 if the huffNode array is not sorted by descending count */
unsafe fn HUF_isSorted(huffNode: *const nodeElt, maxSymbolValue1: u32) -> bool {
    for i in 1..(maxSymbolValue1 as usize) {
        if (*huffNode.add(i)).count > (*huffNode.add(i-1)).count {
            return false;
        }
    }
    true
}

/* Insertion sort by descending order */
#[inline(always)]
unsafe fn HUF_insertionSort(
    mut huffNode: *mut nodeElt,
    low: isize,
    high: isize,
) {
    let size = high - low + 1;
    huffNode = huffNode.offset(low);
    for i in 1..size {
        let key = *huffNode.offset(i);
        let mut j = i - 1;
        while j >= 0 && (*huffNode.offset(j)).count < key.count {
            *huffNode.offset(j + 1) = *huffNode.offset(j);
            j -= 1;
        }
        *huffNode.offset(j + 1) = key;
    }
}

/* Pivot helper function for quicksort. */
unsafe fn HUF_quickSortPartition(
    mut arr: *mut nodeElt,
    low: isize,
    high: isize,
) -> isize {
    /* Simply select rightmost element as pivot. "Better" selectors like
     * median-of-three don't experimentally appear to have any benefit.
     */
    let pivot = (*arr.offset(high)).count;
    let mut i = low - 1;
    let mut j = low;
    for j in low..high {
        if (*arr.offset(j)).count > pivot {
            i += 1;
            HUF_swapNodes(&mut *arr.offset(i), &mut *arr.offset(j));
        }
    }
    HUF_swapNodes(
        &mut *arr.offset(i + 1),
        &mut *arr.offset(high),
    );
    i + 1
}

/* Classic quicksort by descending with partially iterative calls
 * to reduce worst case callstack size.
 */
unsafe fn HUF_simpleQuickSort(
    mut arr: *mut nodeElt,
    mut low: isize,
    mut high: isize,
) {
    const kInsertionSortThreshold: isize = 8;
    if high - low < kInsertionSortThreshold {
        HUF_insertionSort(arr, low, high);
        return;
    }
    while low < high {
        let idx = HUF_quickSortPartition(arr, low, high);
        if idx - low < high - idx {
            HUF_simpleQuickSort(arr, low, idx - 1);
            low = idx + 1;
        } else {
            HUF_simpleQuickSort(arr, idx + 1, high);
            high = idx - 1;
        }
    }
}

/**
 * HUF_sort():
 * Sorts the symbols [0, maxSymbolValue] by count[symbol] in decreasing order.
 * This is a typical bucket sorting strategy that uses either quicksort or insertion sort to sort each bucket.
 *
 * @param[out] huffNode       Sorted symbols by decreasing count. Only members `.count` and `.byte` are filled.
 *                            Must have (maxSymbolValue + 1) entries.
 * @param[in]  count          Histogram of the symbols.
 * @param[in]  maxSymbolValue Maximum symbol value.
 * @param      rankPosition   This is a scratch workspace. Must have RANK_POSITION_TABLE_SIZE entries.
 */
unsafe fn HUF_sort(
    mut huffNode: *mut nodeElt,
    mut count: *const u32,
    maxSymbolValue: u32,
    mut rankPosition: *mut rankPos,
) {
    let maxSymbolValue1 = maxSymbolValue + 1;

    /* Compute base and set curr to base.
     * For symbol s let lowerRank = HUF_getIndex(count[n]) and rank = lowerRank + 1.
     * See HUF_getIndex to see bucketing strategy.
     * We attribute each symbol to lowerRank's base value, because we want to know where
     * each rank begins in the output, so for rank R we want to count ranks R+1 and above.
     */
    libc::memset(rankPosition.cast(), 0, size_of::<rankPos>() * RANK_POSITION_TABLE_SIZE);
    for n in 0..(maxSymbolValue1 as usize) {
        let mut lowerRank = HUF_getIndex(*count.add(n));
        debug_assert!((lowerRank as usize) < RANK_POSITION_TABLE_SIZE - 1);
        (*rankPosition.offset(lowerRank as isize)).base += 1;
    }

    debug_assert!((*rankPosition.add(RANK_POSITION_TABLE_SIZE - 1)).base == 0);
    /* Set up the rankPosition table */
    let mut n = RANK_POSITION_TABLE_SIZE - 1;
    while n > 0 {
        (*rankPosition.add(n-1)).base += (*rankPosition.add(n)).base as u16;
        (*rankPosition.add(n-1)).curr = (*rankPosition.add(n-1)).base;
        n -= 1;
    }

    /* Insert each symbol into their appropriate bucket, setting up rankPosition table. */
    for n in 0..(maxSymbolValue1 as usize) {
        let c = *count.add(n);
        let r = HUF_getIndex(c) + 1;
        let ref mut fresh6 = (*rankPosition.offset(r as isize)).curr;
        let pos = u32::from(*fresh6);
        *fresh6 += 1;
        debug_assert!(pos < maxSymbolValue1);
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as u8;
    }

    /* Sort each bucket. */
    for n in (RANK_POSITION_DISTINCT_COUNT_CUTOFF as usize)..(RANK_POSITION_TABLE_SIZE - 1) {
        let bucketSize = (*rankPosition.add(n)).curr as isize - 
            (*rankPosition.add(n)).base as isize;
        let bucketStartIdx = (*rankPosition.add(n)).base as isize;
        if bucketSize > 1 {
            HUF_simpleQuickSort(
                huffNode.offset(bucketStartIdx),
                0,
                bucketSize - 1,
            );
        }
    }

    debug_assert!(HUF_isSorted(huffNode, maxSymbolValue1));
}


/** HUF_buildCTable_wksp() :
 *  Same as HUF_buildCTable(), but using externally allocated scratch buffer.
 *  `workSpace` must be aligned on 4-bytes boundaries, and be at least as large as sizeof(HUF_buildCTable_wksp_tables).
 */
pub const STARTNODE: u32 = HUF_SYMBOLVALUE_MAX + 1;

/* HUF_buildTree():
 * Takes the huffNode array sorted by HUF_sort() and builds an unlimited-depth Huffman tree.
 *
 * @param huffNode        The array sorted by HUF_sort(). Builds the Huffman tree in this array.
 * @param maxSymbolValue  The maximum symbol value.
 * @return                The smallest node in the Huffman tree (by count).
 */
unsafe fn HUF_buildTree(
    mut huffNode: *mut nodeElt,
    mut maxSymbolValue: u32,
) -> i32 {
    let huffNode0 = huffNode.offset(-1);
    let mut nodeNb: isize = STARTNODE as isize;
    DEBUGLOG!(5, "HUF_buildTree (alphabet size = %u)", maxSymbolValue + 1);
    
    /* init for parents */
    let mut nonNullRank = maxSymbolValue as isize;
    while (*huffNode.offset(nonNullRank)).count == 0 {
        nonNullRank -= 1;
    }
    let mut lowS = nonNullRank;
    let mut nodeRoot = nodeNb + lowS - 1;
    let mut lowN = nodeNb;
    (*huffNode.offset(nodeNb)).count = (*huffNode.offset(lowS)).count + 
        (*huffNode.offset(lowS - 1)).count;
    (*huffNode.offset(lowS)).parent = nodeNb as u16;
    (*huffNode.offset(lowS - 1)).parent = nodeNb as u16;
    nodeNb += 1;
    lowS -= 2;
    for n in nodeNb..=nodeRoot {
        (*huffNode.offset(n)).count = 1_u32 << 30;
    }
    (*huffNode0.offset(0)).count = 1_u32 << 31; /* fake entry, strong barrier */

    /* create parents */
    while nodeNb <= nodeRoot {
        let n1 = if (*huffNode.offset(lowS)).count < (*huffNode.offset(lowN)).count {
            let tmp = lowS;
            lowS -= 1;
            tmp
        } else {
            let tmp = lowN;
            lowN += 1;
            tmp
        };
        let n2 = if (*huffNode.offset(lowS)).count < (*huffNode.offset(lowN)).count {
            let tmp = lowS;
            lowS -= 1;
            tmp
        } else {
            let tmp = lowN;
            lowN += 1;
            tmp
        };
        (*huffNode.offset(nodeNb)).count = (*huffNode.offset(n1)).count +
            (*huffNode.offset(n2)).count;
        (*huffNode.offset(n1)).parent = nodeNb as u16;
        (*huffNode.offset(n2)).parent = nodeNb as u16;
        nodeNb += 1;
    }

    /* distribute weights (unlimited tree height) */
    (*huffNode.offset(nodeRoot)).nbBits = 0;
    let mut n = nodeRoot - 1;
    while n >= STARTNODE as isize {
        let parent = (*huffNode.offset(n)).parent as isize;
        (*huffNode.offset(n)).nbBits = (*huffNode.offset(parent)).nbBits + 1;
        n -= 1;
    }

    for n  in 0..=nonNullRank {
        let parent = (*huffNode.offset(n)).parent as isize;
        (*huffNode.offset(n)).nbBits = (*huffNode.offset(parent)).nbBits + 1;
    }

    // TODO DEBUGLOG!(6, "Initial distribution of bits completed (%zu sorted symbols)", showHNodeBits(huffNode, maxSymbolValue + 1));

    nonNullRank as i32
}

/**
 * HUF_buildCTableFromTree():
 * Build the CTable given the Huffman tree in huffNode.
 *
 * @param[out] CTable         The output Huffman CTable.
 * @param      huffNode       The Huffman tree.
 * @param      nonNullRank    The last and smallest node in the Huffman tree.
 * @param      maxSymbolValue The maximum symbol value.
 * @param      maxNbBits      The exact maximum number of bits used in the Huffman tree.
 */
unsafe fn HUF_buildCTableFromTree(
    mut CTable: *mut HUF_CElt,
    mut huffNode: *const nodeElt,
    mut nonNullRank: i32,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
) {
    let ct = CTable.offset(1);
    /* fill result into ctable (val, nbBits) */
    let mut nbPerRank: [u16; HUF_TABLELOG_MAX as usize + 1] = [0; HUF_TABLELOG_MAX as usize + 1];
    let mut valPerRank: [u16; HUF_TABLELOG_MAX as usize + 1] = [0; HUF_TABLELOG_MAX as usize + 1];
    let alphabetSize = maxSymbolValue + 1;
    for n in 0..=nonNullRank {
        nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize] += 1;
    }
    /* determine starting value per rank */
    let mut min: u16 = 0;
    let mut n = maxNbBits as usize;
    while n > 0 {
        valPerRank[n] = min; /* get starting value within each rank */
        min += nbPerRank[n];
        min >>= 1;
        n -= 1;
    }
    for n in 0..(alphabetSize as usize) {
        HUF_setNbBits(
            ct.offset((*huffNode.add(n)).byte as isize),
            (*huffNode.add(n)).nbBits as usize,
        ); /* push nbBits per symbol, symbol order */
    }
    for n in 0..(alphabetSize as usize) {
        let v = &mut valPerRank[HUF_getNbBits(*ct.add(n))];
        HUF_setValue(ct.add(n), *v as usize);
        *v += 1;
    } /* assign value within rank, symbol order */

    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}

pub unsafe fn HUF_buildCTable_wksp(
    mut CTable: *mut HUF_CElt,
    mut count: *const u32,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    let wksp_tables = HUF_alignUpWorkspace(workSpace, &mut wkspSize, align_of::<u32>())
        as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.offset(1);

    const _: () = assert!(HUF_CTABLE_WORKSPACE_SIZE == size_of::<HUF_buildCTable_wksp_tables>());

    DEBUGLOG!(5, "HUF_buildCTable_wksp (alphabet size = %u)", maxSymbolValue+1);

    /* safety checks */
    if wkspSize < size_of::<HUF_buildCTable_wksp_tables>() {
        return ERROR(ZSTD_error_workSpace_tooSmall);
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX {
        return ERROR(ZSTD_error_maxSymbolValue_tooLarge);
    }
    libc::memset(huffNode0 as *mut c_void, 0, size_of::<huffNodeTable>());

    /* sort, decreasing order */
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    // TODO DEBUGLOG!(6, "sorted symbols completed (%zu symbols)", showHNodeSymbols(huffNode, maxSymbolValue+1));

    /* build tree */
    let nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);

    /* determine and enforce maxTableLog */
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX { /* check fit into table */
        return ERROR(ZSTD_error_GENERIC);
    }

    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);

    maxNbBits as usize
}

pub unsafe fn HUF_estimateCompressedSize(
    mut CTable: *const HUF_CElt,
    mut count: *const u32,
    mut maxSymbolValue: u32,
) -> usize {
    let mut ct = CTable.offset(1);
    let mut nbBits: usize = 0;
    for s in 0..=(maxSymbolValue as usize) {
        nbBits += HUF_getNbBits(*ct.add(s)) * (*count.add(s) as usize);
    }
    return nbBits >> 3;
}

pub unsafe fn HUF_validateCTable(
    mut CTable: *const HUF_CElt,
    mut count: *const u32,
    mut maxSymbolValue: u32,
) -> bool {
    let mut header = HUF_readCTableHeader(CTable);
    let mut ct = CTable.offset(1);

    debug_assert!(u32::from(header.tableLog) <= HUF_TABLELOG_ABSOLUTEMAX);

    if (header.maxSymbolValue as u32) < maxSymbolValue {
        return false;
    }

    let mut bad: bool = false;
    for s in 0..=(maxSymbolValue as usize) {
        bad |= (*count.add(s) != 0) & (HUF_getNbBits(*ct.add(s)) == 0);
    }
    !bad
}

/** HUF_CStream_t:
 * Huffman uses its own BIT_CStream_t implementation.
 * There are three major differences from BIT_CStream_t:
 *   1. HUF_addBits() takes a HUF_CElt (size_t) which is
 *      the pair (nbBits, value) in the format:
 *      format:
 *        - Bits [0, 4)            = nbBits
 *        - Bits [4, 64 - nbBits)  = 0
 *        - Bits [64 - nbBits, 64) = value
 *   2. The bitContainer is built from the upper bits and
 *      right shifted. E.g. to add a new value of N bits
 *      you right shift the bitContainer by N, then or in
 *      the new value into the N upper bits.
 *   3. The bitstream has two bit containers. You can add
 *      bits to the second container and merge them into
 *      the first container.
 */
pub const HUF_BITS_IN_CONTAINER: usize = size_of::<usize>() * 8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [usize; 2],
    pub bitPos: [usize; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}

/** HUF_initCStream():
 * Initializes the bitstream.
 * @returns 0 or an error code.
 */
unsafe fn HUF_initCStream(
    mut bitC: *mut HUF_CStream_t,
    mut startPtr: *mut c_void,
    mut dstCapacity: usize,
) -> usize {
    libc::memset(bitC as *mut c_void, 0, size_of::<HUF_CStream_t>());
    (*bitC).startPtr = startPtr as *mut u8;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr).add(dstCapacity).sub(size_of::<usize>());
    if dstCapacity <= size_of::<usize>() { return ERROR(ZSTD_error_dstSize_tooSmall); }
    return 0;
}

/** HUF_addBits():
 * Adds the symbol stored in HUF_CElt elt to the bitstream.
 *
 * @param elt   The element we're adding. This is a (nbBits, value) pair.
 *              See the HUF_CStream_t docs for the format.
 * @param idx   Insert into the bitstream at this idx.
 * @param kFast This is a template parameter. If the bitstream is guaranteed
 *              to have at least 4 unused bits after this call it may be 1,
 *              otherwise it must be 0. HUF_addBits() is faster when fast is set.
 */
#[inline(always)]
unsafe fn HUF_addBits(
    mut bitC: *mut HUF_CStream_t,
    mut elt: HUF_CElt,
    mut idx: i32,
    mut kFast: bool,
) {
    let idx = idx as usize;

    debug_assert!(idx <= 1);
    debug_assert!(HUF_getNbBits(elt) <= HUF_TABLELOG_ABSOLUTEMAX as usize);
    /* This is efficient on x86-64 with BMI2 because shrx
     * only reads the low 6 bits of the register. The compiler
     * knows this and elides the mask. When fast is set,
     * every operation can use the same value loaded from elt.
     */
    (*bitC).bitContainer[idx] >>= HUF_getNbBits(elt);
    (*bitC).bitContainer[idx] |= if kFast { HUF_getValueFast(elt) } else { HUF_getValue(elt) };
    /* We only read the low 8 bits of bitC->bitPos[idx] so it
     * doesn't matter that the high bits have noise from the value.
     */
    (*bitC).bitPos[idx] += HUF_getNbBitsFast(elt);
    debug_assert!(((*bitC).bitPos[idx] & 0xFF) <= HUF_BITS_IN_CONTAINER);
    /* The last 4-bits of elt are dirty if fast is set,
     * so we must not be overwriting bits that have already been
     * inserted into the bit container.
     */
// TODO #if DEBUGLEVEL >= 1
    // {
    //     size_t const nbBits = HUF_getNbBits(elt);
    //     size_t const dirtyBits = nbBits == 0 ? 0 : ZSTD_highbit32((U32)nbBits) + 1;
    //     (void)dirtyBits;
    //     /* Middle bits are 0. */
    //     assert(((elt >> dirtyBits) << (dirtyBits + nbBits)) == 0);
    //     /* We didn't overwrite any bits in the bit container. */
    //     assert(!kFast || (bitC->bitPos[idx] & 0xFF) <= HUF_BITS_IN_CONTAINER);
    //     (void)dirtyBits;
    // }
}

#[inline(always)]
unsafe fn HUF_zeroIndex1(mut bitC: *mut HUF_CStream_t) {
    (*bitC).bitContainer[1] = 0;
    (*bitC).bitPos[1] = 0;
}

/** HUF_mergeIndex1() :
 * Merges the bit container @ index 1 into the bit container @ index 0
 * and zeros the bit container @ index 1.
 */
#[inline(always)]
unsafe fn HUF_mergeIndex1(mut bitC: *mut HUF_CStream_t) {
    debug_assert!(((*bitC).bitPos[1] & 0xFF) < HUF_BITS_IN_CONTAINER);
    (*bitC).bitContainer[0] >>= (*bitC).bitPos[1] & 0xff;
    (*bitC).bitContainer[0] |= (*bitC).bitContainer[1];
    (*bitC) .bitPos[0] = ((*bitC).bitPos[0]).wrapping_add((*bitC).bitPos[1]);
    debug_assert!(((*bitC).bitPos[0] & 0xFF) < HUF_BITS_IN_CONTAINER);
}

/** HUF_flushBits() :
* Flushes the bits in the bit container @ index 0.
*
* @post bitPos will be < 8.
* @param kFast If kFast is set then we must know a-priori that
*              the bit container will not overflow.
*/
#[inline(always)]
unsafe fn HUF_flushBits(
    mut bitC: *mut HUF_CStream_t,
    mut kFast: bool,
) {
    /* The upper bits of bitPos are noisy, so we must mask by 0xFF. */
    let nbBits = (*bitC).bitPos[0] & 0xff;
    let nbBytes = nbBits >> 3;
    /* The top nbBits bits of bitContainer are the ones we need. */
    let bitContainer = (*bitC).bitContainer[0] >> (HUF_BITS_IN_CONTAINER-nbBits);
    /* Mask bitPos to account for the bytes we consumed. */
    (*bitC).bitPos[0] &= 7;
    debug_assert!(nbBits > 0);
    debug_assert!(nbBits <= size_of::<usize>() * 8);
    debug_assert!((*bitC).ptr <= (*bitC).endPtr);
    MEM_writeLEST((*bitC).ptr as *mut c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    debug_assert!(!kFast || (*bitC).ptr <= (*bitC).endPtr);
    if kFast && (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
    /* bitContainer doesn't need to be modified because the leftover
     * bits are already the top bitPos bits. And we don't care about
     * noise in the lower values.
     */
}

/** HUF_endMark()
 * @returns The Huffman stream end mark: A 1-bit value = 1.
 */
unsafe fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1);
    HUF_setValue(&mut endMark, 1);
    return endMark;
}

/** HUF_closeCStream() :
 *  @return Size of CStream, in bytes,
 *          or 0 if it could not fit into dstBuffer */
unsafe fn HUF_closeCStream(mut bitC: *mut HUF_CStream_t) -> usize {
    HUF_addBits(bitC, HUF_endMark(), /* idx */ 0, /* kFast */ false);
    HUF_flushBits(bitC, /* kFast */ false);
    let nbBits = (*bitC).bitPos[0] & 0xff;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0; /* overflow detected */
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as usize) + (nbBits > 0) as usize
}

#[inline(always)]
unsafe fn HUF_encodeSymbol(
    mut bitCPtr: *mut HUF_CStream_t,
    mut symbol: u32,
    mut CTable: *const HUF_CElt,
    mut idx: i32,
    mut fast: bool,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}

#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body_loop(
    mut bitC: *mut HUF_CStream_t,
    mut ip: *const u8,
    mut srcSize: usize,
    mut ct: *const HUF_CElt,
    mut kUnroll: isize,
    mut kFastFlush: bool,
    mut kLastFast: bool,
) {
    /* Join to kUnroll */
    let mut n = srcSize as isize;
    let mut rem = n % kUnroll;
    if rem > 0 {
        while rem > 0 {
            n -= 1;
            HUF_encodeSymbol(bitC, *ip.offset(n) as u32, ct, 0, false);
            rem -= 1;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    debug_assert!(n % kUnroll == 0);

    /* Join to 2 * kUnroll */
    if n % (2 * kUnroll) != 0 {
        for u in 1..kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n - u) as u32,
                ct,
                0,
                true,
            );
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset(n - kUnroll) as u32,
            ct,
            0,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        n -= kUnroll;
    }
    debug_assert!(n % (2 * kUnroll) == 0);

    while n > 0 {
        /* Encode kUnroll symbols into the bitstream @ index 0. */
        for u_0 in 1..kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n - u_0) as u32,
                ct,
                0,
                true,
            );
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset(n - kUnroll) as u32,
            ct,
            0,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        /* Encode kUnroll symbols into the bitstream @ index 1.
         * This allows us to start filling the bit container
         * without any data dependencies.
         */
        HUF_zeroIndex1(bitC);
        for u_0 in 1..kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n - kUnroll - u_0) as u32,
                ct,
                1,
                true,
            );
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset(n - kUnroll - kUnroll) as u32,
            ct,
            1,
            kLastFast,
        );
        /* Merge bitstream @ index 1 into the bitstream @ index 0 */
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 * kUnroll;
    }
    debug_assert!(n == 0);
}

/**
 * Returns a tight upper bound on the output space needed by Huffman
 * with 8 bytes buffer to handle over-writes. If the output is at least
 * this large we don't need to do bounds checks during Huffman encoding.
 */
const fn HUF_tightCompressBound(
    srcSize: usize,
    tableLog: usize,
) -> usize {
    (srcSize * tableLog >> 3) + 8
}

#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
) -> usize {
    let tableLog = (HUF_readCTableHeader(CTable)).tableLog as u32;
    let mut ct = CTable.offset(1);
    let mut ip = src as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut bitC = HUF_CStream_t {
        bitContainer: [0; 2],
        bitPos: [0; 2],
        startPtr: std::ptr::null_mut(),
        ptr: std::ptr::null_mut(),
        endPtr: std::ptr::null_mut(),
    };

    /* init */
    if dstSize < 8 {
        return 0; /* not enough space to compress */
    }
    let mut op = ostart;
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut c_void,
        oend.offset_from(op) as usize,
    );
    if ERR_isError(initErr) {
        return 0;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as usize) || tableLog > 11 {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct,
            /* kUnroll */ if MEM_32bits { 2 } else { 4 },
            false,
            false,
        );
    } else if MEM_32bits {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2,
                    true,
                    false,
                );
            }
            10 | 9 | 8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2,
                    true,
                    true,
                );
            }
            7 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    3,
                    true,
                    true,
                );
            }
        }
    } else {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    5,
                    true,
                    false,
                );
            }
            10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    5,
                    true,
                    true,
                );
            }
            9 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    6,
                    true,
                    false,
                );
            }
            8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    7,
                    true,
                    false,
                );
            }
            7 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    8,
                    true,
                    false,
                );
            }
            6 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    9,
                    true,
                    true,
                );
            }
        }
    }
    debug_assert!(bitC.ptr <= bitC.endPtr);

    return HUF_closeCStream(&mut bitC);
}

// TODO #if DYNAMIC_BMI2

unsafe fn HUF_compress1X_usingCTable_internal_bmi2(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
) -> usize {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}

unsafe fn HUF_compress1X_usingCTable_internal_default(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
) -> usize {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}

unsafe fn HUF_compress1X_usingCTable_internal(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    flags: i32,
) -> usize {
    if flags & HUF_flags_bmi2 != 0 {
        return HUF_compress1X_usingCTable_internal_bmi2(
            dst,
            dstSize,
            src,
            srcSize,
            CTable,
        );
    }
    return HUF_compress1X_usingCTable_internal_default(
        dst,
        dstSize,
        src,
        srcSize,
        CTable,
    );
}

// #else TODO #if DYNAMIC_BMI2
// static size_t
// HUF_compress1X_usingCTable_internal(void* dst, size_t dstSize,
//                               const void* src, size_t srcSize,
//                               const HUF_CElt* CTable, const int flags)
// {
//     (void)flags;
//     return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
// }

pub unsafe fn HUF_compress1X_usingCTable(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    mut flags: i32,
) -> usize {
    return HUF_compress1X_usingCTable_internal(
        dst,
        dstSize,
        src,
        srcSize,
        CTable,
        flags,
    );
}

unsafe fn HUF_compress4X_usingCTable_internal(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    mut flags: i32,
) -> usize {
    let segmentSize = (srcSize+3) / 4; /* first 3 segments */
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart;

    if dstSize < (6 + 1 + 1 + 1 + 8) { /* minimum space to compress successfully */
        return 0;
    }
    if srcSize < 12 { /* no saving possible : too small input */
        return 0;
    }
    op = op.offset(6); /* jumpTable */

    debug_assert!(op <= oend);
    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from(op) as usize,
        ip as *const c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 || cSize > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart as *mut c_void, cSize as u16);
    op = op.add(cSize);

    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from(op) as usize,
        ip as *const c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_0) {
        return cSize_0;
    }
    if cSize_0 == 0
        || cSize_0 > 65535
    {
        return 0;
    }
    MEM_writeLE16(
        ostart.offset(2) as *mut c_void,
        cSize_0 as u16,
    );
    op = op.add(cSize_0);

    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from(op) as usize,
        ip as *const c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_1) {
        return cSize_1;
    }
    if cSize_1 == 0
        || cSize_1 > 65535
    {
        return 0;
    }
    MEM_writeLE16(
        ostart.offset(4) as *mut c_void,
        cSize_1 as u16,
    );
    op = op.add(cSize_1);

    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    debug_assert!(ip <= iend);
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from(op) as usize,
        ip as *const c_void,
        iend.offset_from(ip) as usize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_2) {
        return cSize_2;
    }
    if cSize_2 == 0
        || cSize_2 > 65535
    {
        return 0;
    }
    op = op.add(cSize_2);

    op.offset_from(ostart) as usize
}

pub unsafe fn HUF_compress4X_usingCTable(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    mut flags: i32,
) -> usize {
    return HUF_compress4X_usingCTable_internal(
        dst,
        dstSize,
        src,
        srcSize,
        CTable,
        flags,
    );
}

pub type HUF_nbStreams_e = u32;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;

unsafe fn HUF_compressCTable_internal(
    ostart: *mut u8,
    mut op: *mut u8,
    oend: *mut u8,
    mut src: *const c_void,
    mut srcSize: usize,
    mut nbStreams: HUF_nbStreams_e,
    mut CTable: *const HUF_CElt,
    flags: i32,
) -> usize {
    let cSize = if nbStreams == HUF_singleStream {
        HUF_compress1X_usingCTable_internal(
            op as *mut c_void,
            oend.offset_from(op) as usize,
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut c_void,
            oend.offset_from(op) as usize,
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 { /* uncompressible */
        return 0;
    }
    op = op.add(cSize);

    /* check compressibility */
    debug_assert!(op >= ostart);
    if op.offset_from(ostart) as usize >= srcSize-1 {
        return 0;
    }
    return op.offset_from(ostart) as usize;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [u32; HUF_SYMBOLVALUE_MAX as usize + 1],
    pub CTable: [HUF_CElt; HUF_CTABLE_SIZE_ST(HUF_SYMBOLVALUE_MAX as usize)],
    pub wksps: HUF_compress_tables_hist_wksp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union HUF_compress_tables_hist_wksp {
    pub buildCTable_wksp: HUF_buildCTable_wksp_tables,
    pub writeCTable_wksp: HUF_WriteCTableWksp,
    pub hist_wksp: [u32; HIST_WKSP_SIZE_U32],
}

pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: usize = 4096;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: usize = 10;

pub unsafe fn HUF_cardinality(
    mut count: *const u32,
    mut maxSymbolValue: u32,
) -> u32 {
    let mut cardinality: u32 = 0;

    for i in 0..(maxSymbolValue as usize + 1) {
        if *count.add(i) != 0 {
            cardinality += 1;
        }
    }

    cardinality
}

pub unsafe fn HUF_minTableLog(
    mut symbolCardinality: u32,
) -> u32 {
    let minBitsSymbols = ZSTD_highbit32(symbolCardinality) + 1;
    minBitsSymbols
}

pub unsafe fn HUF_optimalTableLog(
    mut maxTableLog: u32,
    mut srcSize: usize,
    mut maxSymbolValue: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
    mut table: *mut HUF_CElt,
    mut count: *const u32,
    mut flags: i32,
) -> u32 {
    debug_assert!(srcSize > 1); /* Not supported, RLE should be used instead */
    debug_assert!(wkspSize >= size_of::<HUF_buildCTable_wksp_tables>());

    if flags & HUF_flags_optimalDepth == 0 {
        /* cheap evaluation, based on FSE */
        return FSE_optimalTableLog_internal(
            maxTableLog,
            srcSize,
            maxSymbolValue,
            1,
        );
    }

    let mut dst = (workSpace as *mut u8).add(size_of::<HUF_WriteCTableWksp>());
    let mut dstSize = wkspSize - size_of::<HUF_WriteCTableWksp>();
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = u32::MAX as usize - 1;
    let mut optLog = maxTableLog;

    DEBUGLOG!(6, "HUF_optimalTableLog: probing huf depth (srcSize=%zu)", srcSize);

    /* Search until size increases */
    for optLogGuess in minTableLog..=maxTableLog {
        DEBUGLOG!(7, "checking for huffLog=%u", optLogGuess);

        let hSize = {
            let mut maxBits = HUF_buildCTable_wksp(
                table,
                count,
                maxSymbolValue,
                optLogGuess,
                workSpace,
                wkspSize,
            );
            if ERR_isError(maxBits) {
                continue;
            }

            if maxBits < optLogGuess as usize && optLogGuess > minTableLog {
                break;
            }

            HUF_writeCTable_wksp(
                dst as *mut c_void,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as u32,
                workSpace,
                wkspSize,
            )
        };

        if ERR_isError(hSize) {
            continue;
        }
        
        let newSize = HUF_estimateCompressedSize(table, count, maxSymbolValue) + hSize;
        if newSize > optSize.wrapping_add(1) {
            break;
        }
        if newSize < optSize {
            optSize = newSize;
            optLog = optLogGuess;
        }
    }
    debug_assert!(optLog <= HUF_TABLELOG_MAX);
    optLog
}

/* HUF_compress_internal() :
 * `workSpace_align4` must be aligned on 4-bytes boundaries,
 * and occupies the same space as a table of HUF_WORKSPACE_SIZE_U64 unsigned */
unsafe fn HUF_compress_internal(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut maxSymbolValue: u32,
    mut huffLog: u32,
    mut nbStreams: HUF_nbStreams_e,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
    mut oldHufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: i32,
) -> usize {
    let table = HUF_alignUpWorkspace(workSpace, &mut wkspSize, align_of::<usize>())
        as *mut HUF_compress_tables_t;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;

    DEBUGLOG!(5, "HUF_compress_internal (srcSize=%zu)", srcSize);
    const _: () = assert!(size_of::<HUF_compress_tables_t>() + HUF_WORKSPACE_MAX_ALIGNMENT <= HUF_WORKSPACE_SIZE);

    /* checks & inits */
    if wkspSize < size_of::<HUF_compress_tables_t>() { return ERROR(ZSTD_error_workSpace_tooSmall); }
    if srcSize == 0 { return 0; } /* Uncompressed */
    if dstSize == 0 { return 0; } /* cannot fit anything within dst budget */
    if srcSize > HUF_BLOCKSIZE_MAX as usize { return ERROR(ZSTD_error_srcSize_wrong); } /* current block size limit */
    if huffLog > HUF_TABLELOG_MAX as u32 { return ERROR(ZSTD_error_tableLog_tooLarge); }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX { return ERROR(ZSTD_error_maxSymbolValue_tooLarge); }
    if maxSymbolValue == 0 { maxSymbolValue = HUF_SYMBOLVALUE_MAX; }
    if huffLog == 0 { huffLog = HUF_TABLELOG_DEFAULT; }

    /* Heuristic : If old table is valid, use it for small inputs */
    if flags & HUF_flags_preferRepeat != 0 && !repeat.is_null() && *repeat == HUF_repeat_valid {
        return HUF_compressCTable_internal(
            ostart,
            op,
            oend,
            src,
            srcSize,
            nbStreams,
            oldHufTable,
            flags,
        );
    }

    /* If uncompressible data is suspected, do a smaller sampling first */
    const _: () = assert!(SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO >= 2);
    if flags & HUF_flags_suspectUncompressible != 0 && 
        srcSize >= (SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO)
    {
        let mut largestTotal: usize = 0;
        DEBUGLOG!(5, "input suspected incompressible : sampling to check");
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const u8 as *const c_void,
            SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE,
        ) as usize;
        if ERR_isError(largestBegin) {
            return largestBegin;
        }
        largestTotal += largestBegin;

        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const u8)
                .add(srcSize)
                .sub(SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE) as *const c_void,
            SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE,
        ) as usize;
        if ERR_isError(largestEnd) {
            return largestEnd;
        }
        largestTotal += largestEnd;

        if largestTotal <= ((2 * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE >> 7) + 4) {
            return 0; /* heuristic : probably not compressible enough */
        }
    }

    /* Scan input and build symbol stats */
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut c_void,
        size_of::<[u32; HIST_WKSP_SIZE_U32]>(),
    );
    if ERR_isError(largest) {
        return largest;
    }
    if largest == srcSize { /* single symbol, rle */
        *ostart = *(src as *const u8).offset(0);
        return 1;
    }
    if largest <= (srcSize >> 7) + 4 { /* heuristic : probably not compressible enough */
        return 0;
    }
    // TODO DEBUGLOG!(6, "histogram detail completed (%zu symbols)", showU32(table->count, maxSymbolValue+1));

    /* Check validity of previous table */
    if !repeat.is_null() && *repeat == HUF_repeat_check
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue)
    {
        *repeat = HUF_repeat_none;
    }
    /* Heuristic : use existing table for small inputs */
    if flags & HUF_flags_preferRepeat != 0 && !repeat.is_null() && *repeat != HUF_repeat_none {
        return HUF_compressCTable_internal(
            ostart,
            op,
            oend,
            src,
            srcSize,
            nbStreams,
            oldHufTable,
            flags,
        );
    }

    /* Build Huffman Tree */
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        &mut (*table).wksps as *mut HUF_compress_tables_hist_wksp as *mut c_void,
        size_of::<HUF_compress_tables_hist_wksp>(),
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        flags,
    );
    let maxBits = HUF_buildCTable_wksp(
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.buildCTable_wksp as *mut HUF_buildCTable_wksp_tables
            as *mut c_void,
        size_of::<HUF_buildCTable_wksp_tables>(),
    );
    if ERR_isError(maxBits) {
        return maxBits;
    }
    huffLog = maxBits as u32;
    // TODO DEBUGLOG(6, "bit distribution completed (%zu symbols)", showCTableBits(table->CTable + 1, maxSymbolValue+1));

    /* Write table description header */
    let hSize = HUF_writeCTable_wksp(
        op as *mut c_void,
        dstSize,
        ((*table).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp
            as *mut c_void,
        size_of::<HUF_WriteCTableWksp>(),
    );
    if ERR_isError(hSize) {
        return hSize;
    }
    /* Check if using previous huffman table is beneficial */
    if !repeat.is_null() && *repeat != HUF_repeat_none {
        let oldSize = HUF_estimateCompressedSize(
            oldHufTable,
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );
        let newSize = HUF_estimateCompressedSize(
            ((*table).CTable).as_mut_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );
        if oldSize <= (hSize + newSize) || (hSize + 12) >= srcSize {
            return HUF_compressCTable_internal(
                ostart,
                op,
                oend,
                src,
                srcSize,
                nbStreams,
                oldHufTable,
                flags,
            );
        }
    }

    /* Use the new huffman table */
    if (hSize + 12) >= srcSize {
        return 0;
    }
    op = op.add(hSize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut c_void,
            ((*table).CTable).as_mut_ptr() as *const c_void,
            size_of_val(&(*table).CTable),
        ); /* Save new table */
    }

    return HUF_compressCTable_internal(
        ostart,
        op,
        oend,
        src,
        srcSize,
        nbStreams,
        ((*table).CTable).as_mut_ptr(),
        flags,
    );
}

pub unsafe fn HUF_compress1X_repeat(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut maxSymbolValue: u32,
    mut huffLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: i32,
) -> usize {
    DEBUGLOG!(5, "HUF_compress1X_repeat (srcSize = %zu)", srcSize);
    return HUF_compress_internal(
        dst,
        dstSize,
        src,
        srcSize,
        maxSymbolValue,
        huffLog,
        HUF_singleStream,
        workSpace,
        wkspSize,
        hufTable,
        repeat,
        flags,
    );
}

/* HUF_compress4X_repeat():
 * compress input using 4 streams.
 * consider skipping quickly
 * reuse an existing huffman compression table */
pub unsafe fn HUF_compress4X_repeat(
    mut dst: *mut c_void,
    mut dstSize: usize,
    mut src: *const c_void,
    mut srcSize: usize,
    mut maxSymbolValue: u32,
    mut huffLog: u32,
    mut workSpace: *mut c_void,
    mut wkspSize: usize,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: i32,
) -> usize {
    DEBUGLOG!(5, "HUF_compress4X_repeat (srcSize = %zu)", srcSize);
    return HUF_compress_internal(
        dst,
        dstSize,
        src,
        srcSize,
        maxSymbolValue,
        huffLog,
        HUF_fourStreams,
        workSpace,
        wkspSize,
        hufTable,
        repeat,
        flags,
    );
}
