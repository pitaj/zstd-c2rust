use ::libc;
extern "C" {
    fn HIST_count_wksp(
        count: *mut std::ffi::c_uint,
        maxSymbolValuePtr: *mut std::ffi::c_uint,
        src: *const std::ffi::c_void,
        srcSize: usize,
        workSpace: *mut std::ffi::c_void,
        workSpaceSize: usize,
    ) -> usize;
    fn HIST_count_simple(
        count: *mut std::ffi::c_uint,
        maxSymbolValuePtr: *mut std::ffi::c_uint,
        src: *const std::ffi::c_void,
        srcSize: usize,
    ) -> std::ffi::c_uint;
    fn FSE_optimalTableLog(
        maxTableLog: std::ffi::c_uint,
        srcSize: usize,
        maxSymbolValue: std::ffi::c_uint,
    ) -> std::ffi::c_uint;
    fn FSE_normalizeCount(
        normalizedCounter: *mut std::ffi::c_short,
        tableLog: std::ffi::c_uint,
        count: *const std::ffi::c_uint,
        srcSize: usize,
        maxSymbolValue: std::ffi::c_uint,
        useLowProbCount: std::ffi::c_uint,
    ) -> usize;
    fn FSE_writeNCount(
        buffer: *mut std::ffi::c_void,
        bufferSize: usize,
        normalizedCounter: *const std::ffi::c_short,
        maxSymbolValue: std::ffi::c_uint,
        tableLog: std::ffi::c_uint,
    ) -> usize;
    fn FSE_compress_usingCTable(
        dst: *mut std::ffi::c_void,
        dstCapacity: usize,
        src: *const std::ffi::c_void,
        srcSize: usize,
        ct: *const FSE_CTable,
    ) -> usize;
    fn FSE_optimalTableLog_internal(
        maxTableLog: std::ffi::c_uint,
        srcSize: usize,
        maxSymbolValue: std::ffi::c_uint,
        minus: std::ffi::c_uint,
    ) -> std::ffi::c_uint;
    fn FSE_buildCTable_wksp(
        ct: *mut FSE_CTable,
        normalizedCounter: *const std::ffi::c_short,
        maxSymbolValue: std::ffi::c_uint,
        tableLog: std::ffi::c_uint,
        workSpace: *mut std::ffi::c_void,
        wkspSize: usize,
    ) -> usize;
    fn HUF_readStats(
        huffWeight: *mut u8,
        hwSize: usize,
        rankStats: *mut u32,
        nbSymbolsPtr: *mut u32,
        tableLogPtr: *mut u32,
        src: *const std::ffi::c_void,
        srcSize: usize,
    ) -> usize;
}
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
use crate::common::error::*;
pub type FSE_CTable = std::ffi::c_uint;
pub type HUF_CElt = usize;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type nodeElt = nodeElt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: u32,
    pub parent: u16,
    pub byte: u8,
    pub nbBits: u8,
}
pub type huffNodeTable = [nodeElt; 512];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; 192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rankPos {
    pub base: u16,
    pub curr: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CTableHeader {
    pub tableLog: u8,
    pub maxSymbolValue: u8,
    pub unused: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [u8; 13],
    pub huffWeight: [u8; 255],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CompressWeightsWksp {
    pub CTable: [FSE_CTable; 59],
    pub scratchBuffer: [u32; 41],
    pub count: [std::ffi::c_uint; 13],
    pub norm: [i16; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [usize; 2],
    pub bitPos: [usize; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}
pub type HUF_repeat = std::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_nbStreams_e = std::ffi::c_uint;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [std::ffi::c_uint; 256],
    pub CTable: [HUF_CElt; 257],
    pub wksps: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub buildCTable_wksp: HUF_buildCTable_wksp_tables,
    pub writeCTable_wksp: HUF_WriteCTableWksp,
    pub hist_wksp: [u32; 1024],
}
use crate::common::mem::*;
use crate::common::bits::*;
pub const HUF_BLOCKSIZE_MAX: std::ffi::c_int = 128 as std::ffi::c_int
    * 1024;
pub const HUF_TABLELOG_MAX: std::ffi::c_int = 12;
pub const HUF_TABLELOG_DEFAULT: std::ffi::c_int = 11;
pub const HUF_SYMBOLVALUE_MAX: std::ffi::c_int = 255;
pub const HUF_CTABLEBOUND: std::ffi::c_int = 129;
pub const NULL: std::ffi::c_int = 0;
unsafe extern "C" fn HUF_alignUpWorkspace(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSizePtr: *mut usize,
    mut align: usize,
) -> *mut std::ffi::c_void {
    let mask = align.wrapping_sub(1);
    let rem = workspace as usize & mask;
    let add = align.wrapping_sub(rem) & mask;
    let aligned = (workspace as *mut u8).offset(add as isize);
    if *workspaceSizePtr >= add {
        *workspaceSizePtr = (*workspaceSizePtr).wrapping_sub(add);
        return aligned as *mut std::ffi::c_void;
    } else {
        *workspaceSizePtr = 0;
        return NULL as *mut std::ffi::c_void;
    };
}
pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: std::ffi::c_int = 6;
unsafe extern "C" fn HUF_compressWeights(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut weightTable: *const std::ffi::c_void,
    mut wtSize: usize,
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: usize,
) -> usize {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.offset(dstSize as isize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as std::ffi::c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as u32;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        std::mem::align_of::<u32>(),
    ) as *mut HUF_CompressWeightsWksp;
    if workspaceSize
        < ::core::mem::size_of::<HUF_CompressWeightsWksp>()
    {
        return ERROR(ZSTD_error_GENERIC);
    }
    if wtSize <= 1 {
        return 0;
    }
    let maxCount = HIST_count_simple(
        ((*wksp).count).as_mut_ptr(),
        &mut maxSymbolValue,
        weightTable,
        wtSize,
    );
    if maxCount as usize == wtSize {
        return 1;
    }
    if maxCount == 1 {
        return 0;
    }
    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    let _var_err__ = FSE_normalizeCount(
        ((*wksp).norm).as_mut_ptr(),
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        0,
    );
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    let hSize = FSE_writeNCount(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if CHECK_V_F!(
        hSize, FSE_writeNCount(op, (usize) (oend - op), wksp -> norm, maxSymbolValue,
        tableLog)
    ) != 0
    {
        return hSize;
    }
    op = op.offset(hSize as isize);
    let _var_err___0 = FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 41]>(),
    );
    if ERR_isError(_var_err___0) {
        return _var_err___0;
    }
    let cSize = FSE_compress_usingCTable(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if CHECK_V_F!(
        cSize, FSE_compress_usingCTable(op, (usize) (oend - op), weightTable, wtSize,
        wksp -> CTable)
    ) != 0
    {
        return cSize;
    }
    if cSize == 0 {
        return 0;
    }
    op = op.offset(cSize as isize);
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
unsafe extern "C" fn HUF_getNbBits(mut elt: HUF_CElt) -> usize {
    return elt & 0xff as std::ffi::c_int as HUF_CElt;
}
unsafe extern "C" fn HUF_getNbBitsFast(mut elt: HUF_CElt) -> usize {
    return elt;
}
unsafe extern "C" fn HUF_getValue(mut elt: HUF_CElt) -> usize {
    return elt & !(0xff as std::ffi::c_int as usize);
}
unsafe extern "C" fn HUF_getValueFast(mut elt: HUF_CElt) -> usize {
    return elt;
}
unsafe extern "C" fn HUF_setNbBits(mut elt: *mut HUF_CElt, mut nbBits: usize) {
    *elt = nbBits;
}
unsafe extern "C" fn HUF_setValue(mut elt: *mut HUF_CElt, mut value: usize) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        *elt
            |= value
                << (::core::mem::size_of::<HUF_CElt>())
                    .wrapping_mul(8)
                    .wrapping_sub(nbBits);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTableHeader(
    mut ctable: *const HUF_CElt,
) -> HUF_CTableHeader {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    libc::memcpy(
        &mut header as *mut HUF_CTableHeader as *mut std::ffi::c_void,
        ctable as *const std::ffi::c_void,
        ::core::mem::size_of::<HUF_CTableHeader>() as usize,
    );
    return header;
}
unsafe extern "C" fn HUF_writeCTableHeader(
    mut ctable: *mut HUF_CElt,
    mut tableLog: u32,
    mut maxSymbolValue: u32,
) {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    libc::memset(
        &mut header as *mut HUF_CTableHeader as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<HUF_CTableHeader>() as usize,
    );
    header.tableLog = tableLog as u8;
    header.maxSymbolValue = maxSymbolValue as u8;
    libc::memcpy(
        ctable as *mut std::ffi::c_void,
        &mut header as *mut HUF_CTableHeader as *const std::ffi::c_void,
        ::core::mem::size_of::<HUF_CTableHeader>() as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_writeCTable_wksp(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut CTable: *const HUF_CElt,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: usize,
) -> usize {
    let ct = CTable.offset(1);
    let mut op = dst as *mut u8;
    let mut n: u32 = 0;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        std::mem::align_of::<u32>(),
    ) as *mut HUF_WriteCTableWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_WriteCTableWksp>()
    {
        return ERROR(ZSTD_error_GENERIC);
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as std::ffi::c_uint {
        return ERROR(ZSTD_error_maxSymbolValue_tooLarge);
    }
    (*wksp).bitsToWeight[0] = 0;
    n = 1;
    while n < huffLog.wrapping_add(1) {
        (*wksp)
            .bitsToWeight[n
            as usize] = huffLog
            .wrapping_add(1)
            .wrapping_sub(n) as u8;
        n = n.wrapping_add(1);
        n;
    }
    n = 0;
    while n < maxSymbolValue {
        (*wksp)
            .huffWeight[n
            as usize] = (*wksp)
            .bitsToWeight[HUF_getNbBits(*ct.offset(n as isize)) as usize];
        n = n.wrapping_add(1);
        n;
    }
    if maxDstSize < 1 {
        return ERROR(ZSTD_error_dstSize_tooSmall);
    }
    let hSize = HUF_compressWeights(
        op.offset(1) as *mut std::ffi::c_void,
        maxDstSize.wrapping_sub(1),
        ((*wksp).huffWeight).as_mut_ptr() as *const std::ffi::c_void,
        maxSymbolValue as usize,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut std::ffi::c_void,
        ::core::mem::size_of::<HUF_CompressWeightsWksp>(),
    );
    if CHECK_V_F!(
        hSize, HUF_compressWeights(op + 1, maxDstSize - 1, wksp -> huffWeight,
        maxSymbolValue, & wksp -> wksp, sizeof(wksp -> wksp))
    ) != 0
    {
        return hSize;
    }
    if (hSize > 1) as std::ffi::c_int
        & (hSize
            < maxSymbolValue.wrapping_div(2)
                as usize) as std::ffi::c_int != 0
    {
        *op.offset(0) = hSize as u8;
        return hSize.wrapping_add(1);
    }
    if maxSymbolValue
        > (256 as std::ffi::c_int - 128 as std::ffi::c_int) as std::ffi::c_uint
    {
        return ERROR(ZSTD_error_GENERIC);
    }
    if maxSymbolValue
        .wrapping_add(1)
        .wrapping_div(2)
        .wrapping_add(1) as usize > maxDstSize
    {
        return ERROR(ZSTD_error_dstSize_tooSmall);
    }
    *op
        .offset(
            0,
        ) = (128 as std::ffi::c_uint)
        .wrapping_add(
            maxSymbolValue.wrapping_sub(1),
        ) as u8;
    (*wksp).huffWeight[maxSymbolValue as usize] = 0;
    n = 0;
    while n < maxSymbolValue {
        *op
            .offset(
                (n / 2_u32)
                    .wrapping_add(1) as isize,
            ) = ((((*wksp).huffWeight[n as usize] as std::ffi::c_int)
            << 4)
            + (*wksp).huffWeight[n.wrapping_add(1) as usize]
                as std::ffi::c_int) as u8;
        n = n.wrapping_add(2);
    }
    return maxSymbolValue
        .wrapping_add(1)
        .wrapping_div(2)
        .wrapping_add(1) as usize;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTable(
    mut CTable: *mut HUF_CElt,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut hasZeroWeights: *mut std::ffi::c_uint,
) -> usize {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 13] = [0; 13];
    let mut tableLog: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let ct = CTable.offset(1);
    let readSize = HUF_readStats(
        huffWeight.as_mut_ptr(),
        (255 as std::ffi::c_int + 1 as std::ffi::c_int) as usize,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if ERR_isError(readSize) {
        return readSize;
    }
    *hasZeroWeights = (rankVal[0]
        > 0) as std::ffi::c_int as std::ffi::c_uint;
    if tableLog > HUF_TABLELOG_MAX as u32 {
        return ERROR(ZSTD_error_tableLog_tooLarge);
    }
    if nbSymbols
        > (*maxSymbolValuePtr).wrapping_add(1)
    {
        return ERROR(ZSTD_error_maxSymbolValue_tooSmall);
    }
    *maxSymbolValuePtr = nbSymbols.wrapping_sub(1);
    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);
    let mut n: u32 = 0;
    let mut nextRankStart: u32 = 0;
    n = 1;
    while n <= tableLog {
        let mut curr = nextRankStart;
        nextRankStart = nextRankStart
            .wrapping_add(
                rankVal[n as usize] << n.wrapping_sub(1),
            );
        rankVal[n as usize] = curr;
        n = n.wrapping_add(1);
        n;
    }
    let mut n_0: u32 = 0;
    n_0 = 0;
    while n_0 < nbSymbols {
        let w = huffWeight[n_0 as usize] as u32;
        HUF_setNbBits(
            ct.offset(n_0 as isize),
            (tableLog.wrapping_add(1).wrapping_sub(w) as u8
                as std::ffi::c_int
                & -((w != 0) as std::ffi::c_int)) as usize,
        );
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    let mut nbPerRank: [u16; 14] = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut valPerRank: [u16; 14] = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut n_1: u32 = 0;
    n_1 = 0;
    while n_1 < nbSymbols {
        nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize))
            as usize] = (nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize)) as usize])
            .wrapping_add(1);
        nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize)) as usize];
        n_1 = n_1.wrapping_add(1);
        n_1;
    }
    valPerRank[tableLog.wrapping_add(1)
        as usize] = 0;
    let mut min: u16 = 0;
    let mut n_2: u32 = 0;
    n_2 = tableLog;
    while n_2 > 0 {
        valPerRank[n_2 as usize] = min;
        min = (min as std::ffi::c_int + nbPerRank[n_2 as usize] as std::ffi::c_int)
            as u16;
        min = (min as std::ffi::c_int >> 1) as u16;
        n_2 = n_2.wrapping_sub(1);
        n_2;
    }
    let mut n_3: u32 = 0;
    n_3 = 0;
    while n_3 < nbSymbols {
        let fresh0 = valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize)) as usize];
        valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize))
            as usize] = (valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize)) as usize])
            .wrapping_add(1);
        HUF_setValue(ct.offset(n_3 as isize), fresh0 as usize);
        n_3 = n_3.wrapping_add(1);
        n_3;
    }
    return readSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_getNbBitsFromCTable(
    mut CTable: *const HUF_CElt,
    mut symbolValue: u32,
) -> u32 {
    let ct = CTable.offset(1);
    if symbolValue > (HUF_readCTableHeader(CTable)).maxSymbolValue as u32 {
        return 0;
    }
    return HUF_getNbBits(*ct.offset(symbolValue as isize)) as u32;
}
unsafe extern "C" fn HUF_setMaxHeight(
    mut huffNode: *mut nodeElt,
    mut lastNonNull: u32,
    mut targetNbBits: u32,
) -> u32 {
    let largestBits = (*huffNode.offset(lastNonNull as isize)).nbBits as u32;
    if largestBits <= targetNbBits {
        return largestBits;
    }
    let mut totalCost: std::ffi::c_int = 0;
    let baseCost = ((1 as std::ffi::c_int) << largestBits.wrapping_sub(targetNbBits))
        as u32;
    let mut n = lastNonNull as std::ffi::c_int;
    while (*huffNode.offset(n as isize)).nbBits as u32 > targetNbBits {
        totalCost = (totalCost as u32)
            .wrapping_add(
                baseCost
                    .wrapping_sub(
                        ((1 as std::ffi::c_int)
                            << largestBits
                                .wrapping_sub((*huffNode.offset(n as isize)).nbBits as u32))
                            as u32,
                    ),
            ) as std::ffi::c_int as std::ffi::c_int;
        (*huffNode.offset(n as isize)).nbBits = targetNbBits as u8;
        n -= 1;
        n;
    }
    while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
        n -= 1;
        n;
    }
    totalCost >>= largestBits.wrapping_sub(targetNbBits);
    let noSymbol = 0xf0f0f0f0 as std::ffi::c_uint;
    let mut rankLast: [u32; 14] = [0; 14];
    libc::memset(
        rankLast.as_mut_ptr() as *mut std::ffi::c_void,
        0xf0 as std::ffi::c_int,
        ::core::mem::size_of::<[u32; 14]>() as usize,
    );
    let mut currentNbBits = targetNbBits;
    let mut pos: std::ffi::c_int = 0;
    pos = n;
    while pos >= 0 {
        if !((*huffNode.offset(pos as isize)).nbBits as u32 >= currentNbBits) {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as u32;
            rankLast[targetNbBits.wrapping_sub(currentNbBits) as usize] = pos as u32;
        }
        pos -= 1;
        pos;
    }
    while totalCost > 0 {
        let mut nBitsToDecrease = (ZSTD_highbit32(totalCost as u32))
            .wrapping_add(1);
        while nBitsToDecrease > 1 {
            let highPos = rankLast[nBitsToDecrease as usize];
            let lowPos = rankLast[nBitsToDecrease
                .wrapping_sub(1) as usize];
            if !(highPos == noSymbol) {
                if lowPos == noSymbol {
                    break;
                }
                let highTotal = (*huffNode.offset(highPos as isize)).count;
                let lowTotal = 2_u32
                    * (*huffNode.offset(lowPos as isize)).count;
                if highTotal <= lowTotal {
                    break;
                }
            }
            nBitsToDecrease = nBitsToDecrease.wrapping_sub(1);
            nBitsToDecrease;
        }
        while nBitsToDecrease <= HUF_TABLELOG_MAX as u32
            && rankLast[nBitsToDecrease as usize] == noSymbol
        {
            nBitsToDecrease = nBitsToDecrease.wrapping_add(1);
            nBitsToDecrease;
        }
        totalCost
            -= (1 as std::ffi::c_int)
                << nBitsToDecrease.wrapping_sub(1);
        let ref mut fresh1 = (*huffNode
            .offset(rankLast[nBitsToDecrease as usize] as isize))
            .nbBits;
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
        if rankLast[nBitsToDecrease.wrapping_sub(1) as usize]
            == noSymbol
        {
            rankLast[nBitsToDecrease.wrapping_sub(1)
                as usize] = rankLast[nBitsToDecrease as usize];
        }
        if rankLast[nBitsToDecrease as usize] == 0 {
            rankLast[nBitsToDecrease as usize] = noSymbol;
        } else {
            rankLast[nBitsToDecrease
                as usize] = (rankLast[nBitsToDecrease as usize]).wrapping_sub(1);
            rankLast[nBitsToDecrease as usize];
            if (*huffNode.offset(rankLast[nBitsToDecrease as usize] as isize)).nbBits
                as u32 != targetNbBits.wrapping_sub(nBitsToDecrease)
            {
                rankLast[nBitsToDecrease as usize] = noSymbol;
            }
        }
    }
    while totalCost < 0 {
        if rankLast[1] == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
                n -= 1;
                n;
            }
            let ref mut fresh2 = (*huffNode.offset((n + 1 as std::ffi::c_int) as isize))
                .nbBits;
            *fresh2 = (*fresh2).wrapping_sub(1);
            *fresh2;
            rankLast[1] = (n + 1 as std::ffi::c_int) as u32;
            totalCost += 1;
            totalCost;
        } else {
            let ref mut fresh3 = (*huffNode
                .offset(
                    (rankLast[1])
                        .wrapping_add(1) as isize,
                ))
                .nbBits;
            *fresh3 = (*fresh3).wrapping_sub(1);
            *fresh3;
            rankLast[1] = (rankLast[1]).wrapping_add(1);
            rankLast[1];
            totalCost += 1;
            totalCost;
        }
    }
    return targetNbBits;
}
pub const RANK_POSITION_TABLE_SIZE: std::ffi::c_int = 192;
pub const RANK_POSITION_MAX_COUNT_LOG: std::ffi::c_int = 32;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: std::ffi::c_int = RANK_POSITION_TABLE_SIZE
    - 1 as std::ffi::c_int - RANK_POSITION_MAX_COUNT_LOG - 1;
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: std::ffi::c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN
    as std::ffi::c_uint)
    .wrapping_add(ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as u32));
unsafe extern "C" fn HUF_getIndex(count: u32) -> u32 {
    return if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count))
            .wrapping_add(RANK_POSITION_LOG_BUCKETS_BEGIN as std::ffi::c_uint)
    };
}
unsafe extern "C" fn HUF_swapNodes(mut a: *mut nodeElt, mut b: *mut nodeElt) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}
#[inline(always)]
unsafe extern "C" fn HUF_insertionSort(
    mut huffNode: *mut nodeElt,
    low: std::ffi::c_int,
    high: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let size = high - low + 1;
    huffNode = huffNode.offset(low as isize);
    i = 1;
    while i < size {
        let key = *huffNode.offset(i as isize);
        let mut j = i - 1;
        while j >= 0
            && (*huffNode.offset(j as isize)).count < key.count
        {
            *huffNode
                .offset(
                    (j + 1 as std::ffi::c_int) as isize,
                ) = *huffNode.offset(j as isize);
            j -= 1;
            j;
        }
        *huffNode.offset((j + 1 as std::ffi::c_int) as isize) = key;
        i += 1;
        i;
    }
}
unsafe extern "C" fn HUF_quickSortPartition(
    mut arr: *mut nodeElt,
    low: std::ffi::c_int,
    high: std::ffi::c_int,
) -> std::ffi::c_int {
    let pivot = (*arr.offset(high as isize)).count;
    let mut i = low - 1;
    let mut j = low;
    while j < high {
        if (*arr.offset(j as isize)).count > pivot {
            i += 1;
            i;
            HUF_swapNodes(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1;
        j;
    }
    HUF_swapNodes(
        &mut *arr.offset((i + 1 as std::ffi::c_int) as isize),
        &mut *arr.offset(high as isize),
    );
    return i + 1;
}
unsafe extern "C" fn HUF_simpleQuickSort(
    mut arr: *mut nodeElt,
    mut low: std::ffi::c_int,
    mut high: std::ffi::c_int,
) {
    let kInsertionSortThreshold = 8;
    if high - low < kInsertionSortThreshold {
        HUF_insertionSort(arr, low, high);
        return;
    }
    while low < high {
        let idx = HUF_quickSortPartition(arr, low, high);
        if idx - low < high - idx {
            HUF_simpleQuickSort(arr, low, idx - 1 as std::ffi::c_int);
            low = idx + 1;
        } else {
            HUF_simpleQuickSort(arr, idx + 1, high);
            high = idx - 1;
        }
    }
}
unsafe extern "C" fn HUF_sort(
    mut huffNode: *mut nodeElt,
    mut count: *const std::ffi::c_uint,
    maxSymbolValue: u32,
    mut rankPosition: *mut rankPos,
) {
    let mut n: u32 = 0;
    let maxSymbolValue1 = maxSymbolValue.wrapping_add(1);
    libc::memset(rankPosition, 0, (sizeof(* rankPosition) * RANK_POSITION_TABLE_SIZE) as usize);
    n = 0;
    while n < maxSymbolValue1 {
        let mut lowerRank = HUF_getIndex(*count.offset(n as isize));
        let ref mut fresh4 = (*rankPosition.offset(lowerRank as isize)).base;
        *fresh4 = (*fresh4).wrapping_add(1);
        *fresh4;
        n = n.wrapping_add(1);
        n;
    }
    n = (RANK_POSITION_TABLE_SIZE - 1 as std::ffi::c_int) as u32;
    while n > 0 {
        let ref mut fresh5 = (*rankPosition
            .offset(n.wrapping_sub(1) as isize))
            .base;
        *fresh5 = (*fresh5 as std::ffi::c_int
            + (*rankPosition.offset(n as isize)).base as std::ffi::c_int) as u16;
        (*rankPosition.offset(n.wrapping_sub(1) as isize))
            .curr = (*rankPosition
            .offset(n.wrapping_sub(1) as isize))
            .base;
        n = n.wrapping_sub(1);
        n;
    }
    n = 0;
    while n < maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)).wrapping_add(1);
        let ref mut fresh6 = (*rankPosition.offset(r as isize)).curr;
        let fresh7 = *fresh6;
        *fresh6 = (*fresh6).wrapping_add(1);
        let pos = fresh7 as u32;
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as u8;
        n = n.wrapping_add(1);
        n;
    }
    n = RANK_POSITION_DISTINCT_COUNT_CUTOFF;
    while n < (RANK_POSITION_TABLE_SIZE - 1 as std::ffi::c_int) as u32 {
        let bucketSize = (*rankPosition.offset(n as isize)).curr as std::ffi::c_int
            - (*rankPosition.offset(n as isize)).base as std::ffi::c_int;
        let bucketStartIdx = (*rankPosition.offset(n as isize)).base as u32;
        if bucketSize > 1 {
            HUF_simpleQuickSort(
                huffNode.offset(bucketStartIdx as isize),
                0,
                bucketSize - 1,
            );
        }
        n = n.wrapping_add(1);
        n;
    }
}
pub const STARTNODE: std::ffi::c_int = HUF_SYMBOLVALUE_MAX + 1;
unsafe extern "C" fn HUF_buildTree(
    mut huffNode: *mut nodeElt,
    mut maxSymbolValue: u32,
) -> std::ffi::c_int {
    let huffNode0 = huffNode.offset(-1_isize);
    let mut nonNullRank: std::ffi::c_int = 0;
    let mut lowS: std::ffi::c_int = 0;
    let mut lowN: std::ffi::c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: std::ffi::c_int = 0;
    let mut nodeRoot: std::ffi::c_int = 0;
    nonNullRank = maxSymbolValue as std::ffi::c_int;
    while (*huffNode.offset(nonNullRank as isize)).count == 0 {
        nonNullRank -= 1;
        nonNullRank;
    }
    lowS = nonNullRank;
    nodeRoot = nodeNb + lowS - 1;
    lowN = nodeNb;
    (*huffNode.offset(nodeNb as isize))
        .count = ((*huffNode.offset(lowS as isize)).count)
        .wrapping_add((*huffNode.offset((lowS - 1 as std::ffi::c_int) as isize)).count);
    let ref mut fresh8 = (*huffNode.offset((lowS - 1 as std::ffi::c_int) as isize))
        .parent;
    *fresh8 = nodeNb as u16;
    (*huffNode.offset(lowS as isize)).parent = *fresh8;
    nodeNb += 1;
    nodeNb;
    lowS -= 2;
    n = nodeNb;
    while n <= nodeRoot {
        (*huffNode.offset(n as isize))
            .count = (1 as std::ffi::c_uint) << 30;
        n += 1;
        n;
    }
    (*huffNode0.offset(0))
        .count = (1 as std::ffi::c_uint) << 31;
    while nodeNb <= nodeRoot {
        let n1 = if (*huffNode.offset(lowS as isize)).count
            < (*huffNode.offset(lowN as isize)).count
        {
            let fresh9 = lowS;
            lowS = lowS - 1;
            fresh9
        } else {
            let fresh10 = lowN;
            lowN = lowN + 1;
            fresh10
        };
        let n2 = if (*huffNode.offset(lowS as isize)).count
            < (*huffNode.offset(lowN as isize)).count
        {
            let fresh11 = lowS;
            lowS = lowS - 1;
            fresh11
        } else {
            let fresh12 = lowN;
            lowN = lowN + 1;
            fresh12
        };
        (*huffNode.offset(nodeNb as isize))
            .count = ((*huffNode.offset(n1 as isize)).count)
            .wrapping_add((*huffNode.offset(n2 as isize)).count);
        let ref mut fresh13 = (*huffNode.offset(n2 as isize)).parent;
        *fresh13 = nodeNb as u16;
        (*huffNode.offset(n1 as isize)).parent = *fresh13;
        nodeNb += 1;
        nodeNb;
    }
    (*huffNode.offset(nodeRoot as isize)).nbBits = 0;
    n = nodeRoot - 1;
    while n >= STARTNODE {
        (*huffNode.offset(n as isize))
            .nbBits = ((*huffNode.offset((*huffNode.offset(n as isize)).parent as isize))
            .nbBits as std::ffi::c_int + 1 as std::ffi::c_int) as u8;
        n -= 1;
        n;
    }
    n = 0;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize))
            .nbBits = ((*huffNode.offset((*huffNode.offset(n as isize)).parent as isize))
            .nbBits as std::ffi::c_int + 1 as std::ffi::c_int) as u8;
        n += 1;
        n;
    }
    return nonNullRank;
}
unsafe extern "C" fn HUF_buildCTableFromTree(
    mut CTable: *mut HUF_CElt,
    mut huffNode: *const nodeElt,
    mut nonNullRank: std::ffi::c_int,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
) {
    let ct = CTable.offset(1);
    let mut n: std::ffi::c_int = 0;
    let mut nbPerRank: [u16; 13] = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut valPerRank: [u16; 13] = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let alphabetSize = maxSymbolValue.wrapping_add(1)
        as std::ffi::c_int;
    n = 0;
    while n <= nonNullRank {
        nbPerRank[(*huffNode.offset(n as isize)).nbBits
            as usize] = (nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize])
            .wrapping_add(1);
        nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize];
        n += 1;
        n;
    }
    let mut min: u16 = 0;
    n = maxNbBits as std::ffi::c_int;
    while n > 0 {
        valPerRank[n as usize] = min;
        min = (min as std::ffi::c_int + nbPerRank[n as usize] as std::ffi::c_int) as u16;
        min = (min as std::ffi::c_int >> 1) as u16;
        n -= 1;
        n;
    }
    n = 0;
    while n < alphabetSize {
        HUF_setNbBits(
            ct.offset((*huffNode.offset(n as isize)).byte as std::ffi::c_int as isize),
            (*huffNode.offset(n as isize)).nbBits as usize,
        );
        n += 1;
        n;
    }
    n = 0;
    while n < alphabetSize {
        let fresh14 = valPerRank[HUF_getNbBits(*ct.offset(n as isize)) as usize];
        valPerRank[HUF_getNbBits(*ct.offset(n as isize))
            as usize] = (valPerRank[HUF_getNbBits(*ct.offset(n as isize)) as usize])
            .wrapping_add(1);
        HUF_setValue(ct.offset(n as isize), fresh14 as usize);
        n += 1;
        n;
    }
    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable_wksp(
    mut CTable: *mut HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let wksp_tables = HUF_alignUpWorkspace(workSpace, &mut wkspSize, std::mem::align_of::<u32>())
        as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.offset(1);
    let mut nonNullRank: std::ffi::c_int = 0;
    if wkspSize
        < ::core::mem::size_of::<HUF_buildCTable_wksp_tables>()
    {
        return ERROR(ZSTD_error_workSpace_tooSmall);
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT as u32;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as u32 {
        return ERROR(ZSTD_error_maxSymbolValue_tooLarge);
    }
    libc::memset(
        huffNode0 as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<huffNodeTable>() as usize,
    );
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as u32 {
        return ERROR(ZSTD_error_GENERIC);
    }
    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);
    return maxNbBits as usize;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_estimateCompressedSize(
    mut CTable: *const HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: std::ffi::c_uint,
) -> usize {
    let mut ct = CTable.offset(1);
    let mut nbBits: usize = 0;
    let mut s: std::ffi::c_int = 0;
    s = 0;
    while s <= maxSymbolValue as std::ffi::c_int {
        nbBits = nbBits
            .wrapping_add(
                HUF_getNbBits(*ct.offset(s as isize))
                    * *count.offset(s as isize) as usize,
            );
        s += 1;
        s;
    }
    return nbBits >> 3;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_validateCTable(
    mut CTable: *const HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: std::ffi::c_uint,
) -> std::ffi::c_int {
    let mut header = HUF_readCTableHeader(CTable);
    let mut ct = CTable.offset(1);
    let mut bad: std::ffi::c_int = 0;
    let mut s: std::ffi::c_int = 0;
    if (header.maxSymbolValue as std::ffi::c_uint) < maxSymbolValue {
        return 0;
    }
    s = 0;
    while s <= maxSymbolValue as std::ffi::c_int {
        bad
            |= (*count.offset(s as isize) != 0)
                as std::ffi::c_int
                & (HUF_getNbBits(*ct.offset(s as isize))
                    == 0) as std::ffi::c_int;
        s += 1;
        s;
    }
    return (bad == 0) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compressBound(mut size: usize) -> usize {
    return HUF_COMPRESSBOUND!(size);
}
pub const HUF_BITS_IN_CONTAINER: std::ffi::c_ulong = (::core::mem::size_of::<usize>()
    as std::ffi::c_ulong)
    .wrapping_mul(8);
unsafe extern "C" fn HUF_initCStream(
    mut bitC: *mut HUF_CStream_t,
    mut startPtr: *mut std::ffi::c_void,
    mut dstCapacity: usize,
) -> usize {
    libc::memset(
        bitC as *mut std::ffi::c_void,
        0,
        ::core::mem::size_of::<HUF_CStream_t>() as usize,
    );
    (*bitC).startPtr = startPtr as *mut u8;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC)
        .endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<usize>() as isize));
    if dstCapacity <= ::core::mem::size_of::<usize>() {
        return ERROR(ZSTD_error_dstSize_tooSmall);
    }
    return 0;
}
#[inline(always)]
unsafe extern "C" fn HUF_addBits(
    mut bitC: *mut HUF_CStream_t,
    mut elt: HUF_CElt,
    mut idx: std::ffi::c_int,
    mut kFast: std::ffi::c_int,
) {
    (*bitC).bitContainer[idx as usize] >>= HUF_getNbBits(elt);
    (*bitC).bitContainer[idx as usize]
        |= if kFast != 0 { HUF_getValueFast(elt) } else { HUF_getValue(elt) };
    (*bitC)
        .bitPos[idx
        as usize] = ((*bitC).bitPos[idx as usize]).wrapping_add(HUF_getNbBitsFast(elt));
}
#[inline(always)]
unsafe extern "C" fn HUF_zeroIndex1(mut bitC: *mut HUF_CStream_t) {
    (*bitC).bitContainer[1] = 0;
    (*bitC).bitPos[1] = 0;
}
#[inline(always)]
unsafe extern "C" fn HUF_mergeIndex1(mut bitC: *mut HUF_CStream_t) {
    (*bitC).bitContainer[0]
        >>= (*bitC).bitPos[1]
            & 0xff as std::ffi::c_int as usize;
    (*bitC).bitContainer[0]
        |= (*bitC).bitContainer[1];
    (*bitC)
        .bitPos[0] = ((*bitC).bitPos[0])
        .wrapping_add((*bitC).bitPos[1]);
}
#[inline(always)]
unsafe extern "C" fn HUF_flushBits(
    mut bitC: *mut HUF_CStream_t,
    mut kFast: std::ffi::c_int,
) {
    let nbBits = (*bitC).bitPos[0]
        & 0xff as std::ffi::c_int as usize;
    let nbBytes = nbBits >> 3;
    let bitContainer = (*bitC).bitContainer[0]
        >> HUF_BITS_IN_CONTAINER.wrapping_sub(nbBits);
    (*bitC).bitPos[0] &= 7;
    MEM_writeLEST((*bitC).ptr as *mut std::ffi::c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    if kFast == 0 && (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
}
unsafe extern "C" fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1);
    HUF_setValue(&mut endMark, 1);
    return endMark;
}
unsafe extern "C" fn HUF_closeCStream(mut bitC: *mut HUF_CStream_t) -> usize {
    HUF_addBits(bitC, HUF_endMark(), 0, 0);
    HUF_flushBits(bitC, 0);
    let nbBits = (*bitC).bitPos[0]
        & 0xff as std::ffi::c_int as usize;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0;
    }
    return (((*bitC).ptr).offset_from((*bitC).startPtr) as std::ffi::c_long as usize)
        .wrapping_add(
            (nbBits > 0) as std::ffi::c_int as usize,
        );
}
#[inline(always)]
unsafe extern "C" fn HUF_encodeSymbol(
    mut bitCPtr: *mut HUF_CStream_t,
    mut symbol: u32,
    mut CTable: *const HUF_CElt,
    mut idx: std::ffi::c_int,
    mut fast: std::ffi::c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body_loop(
    mut bitC: *mut HUF_CStream_t,
    mut ip: *const u8,
    mut srcSize: usize,
    mut ct: *const HUF_CElt,
    mut kUnroll: std::ffi::c_int,
    mut kFastFlush: std::ffi::c_int,
    mut kLastFast: std::ffi::c_int,
) {
    let mut n = srcSize as std::ffi::c_int;
    let mut rem = n % kUnroll;
    if rem > 0 {
        while rem > 0 {
            n -= 1;
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n as isize) as u32,
                ct,
                0,
                0,
            );
            rem -= 1;
            rem;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    if n % (2 as std::ffi::c_int * kUnroll) != 0 {
        let mut u: std::ffi::c_int = 0;
        u = 1;
        while u < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u) as isize) as u32,
                ct,
                0,
                1,
            );
            u += 1;
            u;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as u32,
            ct,
            0,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        n -= kUnroll;
    }
    while n > 0 {
        let mut u_0: std::ffi::c_int = 0;
        u_0 = 1;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u_0) as isize) as u32,
                ct,
                0,
                1,
            );
            u_0 += 1;
            u_0;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as u32,
            ct,
            0,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        HUF_zeroIndex1(bitC);
        u_0 = 1;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - kUnroll - u_0) as isize) as u32,
                ct,
                1,
                1,
            );
            u_0 += 1;
            u_0;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll - kUnroll) as isize) as u32,
            ct,
            1,
            kLastFast,
        );
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 as std::ffi::c_int * kUnroll;
    }
}
unsafe extern "C" fn HUF_tightCompressBound(
    mut srcSize: usize,
    mut tableLog: usize,
) -> usize {
    return (srcSize * tableLog >> 3)
        .wrapping_add(8);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
) -> usize {
    let tableLog = (HUF_readCTableHeader(CTable)).tableLog as u32;
    let mut ct = CTable.offset(1);
    let mut ip = src as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut bitC = HUF_CStream_t {
        bitContainer: [0; 2],
        bitPos: [0; 2],
        startPtr: 0 as *mut u8,
        ptr: 0 as *mut u8,
        endPtr: 0 as *mut u8,
    };
    if dstSize < 8 {
        return 0;
    }
    let mut op = ostart;
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
    );
    if ERR_isError(initErr) {
        return 0;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as usize)
        || tableLog > 11
    {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct,
            if MEM_32bits { 2 as std::ffi::c_int } else { 4 as std::ffi::c_int },
            0,
            0,
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
                    1,
                    0,
                );
            }
            10 | 9 | 8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2,
                    1,
                    1,
                );
            }
            7 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    3,
                    1,
                    1,
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
                    1,
                    0,
                );
            }
            10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    5,
                    1,
                    1,
                );
            }
            9 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    6,
                    1,
                    0,
                );
            }
            8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    7,
                    1,
                    0,
                );
            }
            7 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    8,
                    1,
                    0,
                );
            }
            6 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    9,
                    1,
                    1,
                );
            }
        }
    }
    return HUF_closeCStream(&mut bitC);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
) -> usize {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
) -> usize {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    flags: std::ffi::c_int,
) -> usize {
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
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
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_usingCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    mut flags: std::ffi::c_int,
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
unsafe extern "C" fn HUF_compress4X_usingCTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    mut flags: std::ffi::c_int,
) -> usize {
    let segmentSize = srcSize.wrapping_add(3)
        / 4;
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if dstSize
        < (6 as std::ffi::c_int + 1 as std::ffi::c_int + 1 as std::ffi::c_int
            + 1 as std::ffi::c_int + 8 as std::ffi::c_int) as usize
    {
        return 0;
    }
    if srcSize < 12 {
        return 0;
    }
    op = op.offset(6);
    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        ip as *const std::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if CHECK_V_F!(
        cSize, HUF_compress1X_usingCTable_internal(op, (usize) (oend - op), ip,
        segmentSize, CTable, flags)
    ) != 0
    {
        return cSize;
    }
    if cSize == 0
        || cSize > 65535
    {
        return 0;
    }
    MEM_writeLE16(ostart as *mut std::ffi::c_void, cSize as u16);
    op = op.offset(cSize as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        ip as *const std::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if CHECK_V_F!(
        cSize, HUF_compress1X_usingCTable_internal(op, (usize) (oend - op), ip,
        segmentSize, CTable, flags)
    ) != 0
    {
        return cSize_0;
    }
    if cSize_0 == 0
        || cSize_0 > 65535
    {
        return 0;
    }
    MEM_writeLE16(
        ostart.offset(2) as *mut std::ffi::c_void,
        cSize_0 as u16,
    );
    op = op.offset(cSize_0 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        ip as *const std::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if CHECK_V_F!(
        cSize, HUF_compress1X_usingCTable_internal(op, (usize) (oend - op), ip,
        segmentSize, CTable, flags)
    ) != 0
    {
        return cSize_1;
    }
    if cSize_1 == 0
        || cSize_1 > 65535
    {
        return 0;
    }
    MEM_writeLE16(
        ostart.offset(4) as *mut std::ffi::c_void,
        cSize_1 as u16,
    );
    op = op.offset(cSize_1 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as usize,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as usize,
        CTable,
        flags,
    );
    if CHECK_V_F!(
        cSize, HUF_compress1X_usingCTable_internal(op, (usize) (oend - op), ip, (usize)
        (iend - ip), CTable, flags)
    ) != 0
    {
        return cSize_2;
    }
    if cSize_2 == 0
        || cSize_2 > 65535
    {
        return 0;
    }
    op = op.offset(cSize_2 as isize);
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_usingCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut CTable: *const HUF_CElt,
    mut flags: std::ffi::c_int,
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
unsafe extern "C" fn HUF_compressCTable_internal(
    ostart: *mut u8,
    mut op: *mut u8,
    oend: *mut u8,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut nbStreams: HUF_nbStreams_e,
    mut CTable: *const HUF_CElt,
    flags: std::ffi::c_int,
) -> usize {
    let cSize = if nbStreams as std::ffi::c_uint
        == HUF_singleStream as std::ffi::c_int as std::ffi::c_uint
    {
        HUF_compress1X_usingCTable_internal(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as usize,
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as usize,
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 {
        return 0;
    }
    op = op.offset(cSize as isize);
    if op.offset_from(ostart) as std::ffi::c_long as usize
        >= srcSize.wrapping_sub(1)
    {
        return 0;
    }
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: std::ffi::c_int = 4096;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: std::ffi::c_int = 10;
#[no_mangle]
pub unsafe extern "C" fn HUF_cardinality(
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: std::ffi::c_uint,
) -> std::ffi::c_uint {
    let mut cardinality: std::ffi::c_uint = 0;
    let mut i: std::ffi::c_uint = 0;
    i = 0;
    while i < maxSymbolValue.wrapping_add(1) {
        if *count.offset(i as isize) != 0 {
            cardinality = cardinality
                .wrapping_add(1);
        }
        i = i.wrapping_add(1);
        i;
    }
    return cardinality;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_minTableLog(
    mut symbolCardinality: std::ffi::c_uint,
) -> std::ffi::c_uint {
    let mut minBitsSymbols = (ZSTD_highbit32(symbolCardinality))
        .wrapping_add(1);
    return minBitsSymbols;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_optimalTableLog(
    mut maxTableLog: std::ffi::c_uint,
    mut srcSize: usize,
    mut maxSymbolValue: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut table: *mut HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_uint {
    if flags & HUF_flags_optimalDepth as std::ffi::c_int == 0 {
        return FSE_optimalTableLog_internal(
            maxTableLog,
            srcSize,
            maxSymbolValue,
            1,
        );
    }
    let mut dst = (workSpace as *mut u8)
        .offset(
            ::core::mem::size_of::<HUF_WriteCTableWksp>() as isize,
        );
    let mut dstSize = wkspSize
        .wrapping_sub(
            ::core::mem::size_of::<HUF_WriteCTableWksp>(),
        );
    let mut hSize: usize = 0;
    let mut newSize: usize = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0 as std::ffi::c_int) as usize)
        .wrapping_sub(1);
    let mut optLog = maxTableLog;
    let mut optLogGuess: std::ffi::c_uint = 0;
    optLogGuess = minTableLog;
    while optLogGuess <= maxTableLog {
        let mut maxBits = HUF_buildCTable_wksp(
            table,
            count,
            maxSymbolValue,
            optLogGuess,
            workSpace,
            wkspSize,
        );
        if !(ERR_isError(maxBits) != 0) {
            if maxBits < optLogGuess as usize && optLogGuess > minTableLog {
                break;
            }
            hSize = HUF_writeCTable_wksp(
                dst as *mut std::ffi::c_void,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as u32,
                workSpace,
                wkspSize,
            );
            if !(ERR_isError(hSize) != 0) {
                newSize = (HUF_estimateCompressedSize(table, count, maxSymbolValue))
                    .wrapping_add(hSize);
                if newSize > optSize.wrapping_add(1) {
                    break;
                }
                if newSize < optSize {
                    optSize = newSize;
                    optLog = optLogGuess;
                }
            }
        }
        optLogGuess = optLogGuess.wrapping_add(1);
        optLogGuess;
    }
    return optLog;
}
unsafe extern "C" fn HUF_compress_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut nbStreams: HUF_nbStreams_e,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut oldHufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: std::ffi::c_int,
) -> usize {
    let table = HUF_alignUpWorkspace(workSpace, &mut wkspSize, std::mem::align_of::<usize>())
        as *mut HUF_compress_tables_t;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if wkspSize < ::core::mem::size_of::<HUF_compress_tables_t>() {
        return ERROR(ZSTD_error_workSpace_tooSmall);
    }
    if srcSize == 0 {
        return 0;
    }
    if dstSize == 0 {
        return 0;
    }
    if srcSize > HUF_BLOCKSIZE_MAX as usize {
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    if huffLog > HUF_TABLELOG_MAX as std::ffi::c_uint {
        return ERROR(ZSTD_error_tableLog_tooLarge);
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as std::ffi::c_uint {
        return ERROR(ZSTD_error_maxSymbolValue_tooLarge);
    }
    if maxSymbolValue == 0 {
        maxSymbolValue = HUF_SYMBOLVALUE_MAX as std::ffi::c_uint;
    }
    if huffLog == 0 {
        huffLog = HUF_TABLELOG_DEFAULT as std::ffi::c_uint;
    }
    if flags & HUF_flags_preferRepeat as std::ffi::c_int != 0 && !repeat.is_null()
        && *repeat as std::ffi::c_uint
            == HUF_repeat_valid as std::ffi::c_int as std::ffi::c_uint
    {
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
    if flags & HUF_flags_suspectUncompressible as std::ffi::c_int != 0
        && srcSize
            >= (SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO)
                as usize
    {
        let mut largestTotal: usize = 0;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const u8 as *const std::ffi::c_void,
            4096,
        ) as usize;
        if CHECK_V_F!(
            largestBegin, HIST_count_simple(table -> count, & maxSymbolValueBegin, (const
            u8 *) src, SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE)
        ) != 0
        {
            return largestBegin;
        }
        largestTotal = largestTotal.wrapping_add(largestBegin);
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const u8)
                .offset(srcSize as isize)
                .offset(-4096_isize) as *const std::ffi::c_void,
            4096,
        ) as usize;
        if CHECK_V_F!(
            largestEnd, HIST_count_simple(table -> count, & maxSymbolValueEnd, (const
            u8 *) src + srcSize - SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE,
            SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE)
        ) != 0
        {
            return largestEnd;
        }
        largestTotal = largestTotal.wrapping_add(largestEnd);
        if largestTotal
            <= ((2 as std::ffi::c_int * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE
                >> 7) + 4 as std::ffi::c_int) as usize
        {
            return 0;
        }
    }
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const std::ffi::c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 1024]>(),
    );
    if CHECK_V_F!(
        largest, HIST_count_wksp(table -> count, & maxSymbolValue, (const u8 *) src,
        srcSize, table -> wksps.hist_wksp, sizeof(table -> wksps.hist_wksp))
    ) != 0
    {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const u8).offset(0);
        return 1;
    }
    if largest
        <= (srcSize >> 7).wrapping_add(4)
    {
        return 0;
    }
    if !repeat.is_null()
        && *repeat as std::ffi::c_uint
            == HUF_repeat_check as std::ffi::c_int as std::ffi::c_uint
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue)
            == 0
    {
        *repeat = HUF_repeat_none;
    }
    if flags & HUF_flags_preferRepeat as std::ffi::c_int != 0 && !repeat.is_null()
        && *repeat as std::ffi::c_uint
            != HUF_repeat_none as std::ffi::c_int as std::ffi::c_uint
    {
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
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        &mut (*table).wksps as *mut C2RustUnnamed_1 as *mut std::ffi::c_void,
        ::core::mem::size_of::<C2RustUnnamed_1>(),
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
            as *mut std::ffi::c_void,
        ::core::mem::size_of::<HUF_buildCTable_wksp_tables>(),
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    huffLog = maxBits as u32;
    let hSize = HUF_writeCTable_wksp(
        op as *mut std::ffi::c_void,
        dstSize,
        ((*table).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp
            as *mut std::ffi::c_void,
        ::core::mem::size_of::<HUF_WriteCTableWksp>(),
    );
    if CHECK_V_F!(
        hSize, HUF_writeCTable_wksp(op, dstSize, table -> CTable, maxSymbolValue,
        huffLog, & table -> wksps.writeCTable_wksp, sizeof(table -> wksps
        .writeCTable_wksp))
    ) != 0
    {
        return hSize;
    }
    if !repeat.is_null()
        && *repeat as std::ffi::c_uint
            != HUF_repeat_none as std::ffi::c_int as std::ffi::c_uint
    {
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
        if oldSize <= hSize.wrapping_add(newSize)
            || hSize.wrapping_add(12) >= srcSize
        {
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
    if hSize.wrapping_add(12) >= srcSize {
        return 0;
    }
    op = op.offset(hSize as isize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut std::ffi::c_void,
            ((*table).CTable).as_mut_ptr() as *const std::ffi::c_void,
            ::core::mem::size_of::<[HUF_CElt; 257]>()
                as usize,
        );
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
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_repeat(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: std::ffi::c_int,
) -> usize {
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
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_repeat(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: usize,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: std::ffi::c_int,
) -> usize {
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
