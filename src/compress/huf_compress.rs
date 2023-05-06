use ::libc;
extern "C" {
    fn HIST_count_wksp(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        workSpace: *mut libc::c_void,
        workSpaceSize: libc::size_t,
    ) -> libc::size_t;
    fn HIST_count_simple(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::c_uint;
    fn FSE_optimalTableLog(
        maxTableLog: libc::c_uint,
        srcSize: libc::size_t,
        maxSymbolValue: libc::c_uint,
    ) -> libc::c_uint;
    fn FSE_normalizeCount(
        normalizedCounter: *mut libc::c_short,
        tableLog: libc::c_uint,
        count: *const libc::c_uint,
        srcSize: libc::size_t,
        maxSymbolValue: libc::c_uint,
        useLowProbCount: libc::c_uint,
    ) -> libc::size_t;
    fn FSE_writeNCount(
        buffer: *mut libc::c_void,
        bufferSize: libc::size_t,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
    ) -> libc::size_t;
    fn FSE_compress_usingCTable(
        dst: *mut libc::c_void,
        dstCapacity: libc::size_t,
        src: *const libc::c_void,
        srcSize: libc::size_t,
        ct: *const FSE_CTable,
    ) -> libc::size_t;
    fn FSE_optimalTableLog_internal(
        maxTableLog: libc::c_uint,
        srcSize: libc::size_t,
        maxSymbolValue: libc::c_uint,
        minus: libc::c_uint,
    ) -> libc::c_uint;
    fn FSE_buildCTable_wksp(
        ct: *mut FSE_CTable,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: libc::size_t,
    ) -> libc::size_t;
    fn HUF_readStats(
        huffWeight: *mut u8,
        hwSize: libc::size_t,
        rankStats: *mut u32,
        nbSymbolsPtr: *mut u32,
        tableLogPtr: *mut u32,
        src: *const libc::c_void,
        srcSize: libc::size_t,
    ) -> libc::size_t;
}
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
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
pub type FSE_CTable = libc::c_uint;
pub type HUF_CElt = libc::size_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
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
    pub count: [libc::c_uint; 13],
    pub norm: [i16; 13],
}
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
pub struct HUF_CStream_t {
    pub bitContainer: [libc::size_t; 2],
    pub bitPos: [libc::size_t; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}
pub type HUF_repeat = libc::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_nbStreams_e = libc::c_uint;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [libc::c_uint; 256],
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
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<libc::size_t>()
        == 4) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: u16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: u64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut u8;
        *p.offset(0) = val as u8;
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (val as libc::c_int >> 8 as libc::c_int) as u8;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void, mut val64: u64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void, mut val: libc::size_t) {
    if MEM_32bits() != 0 {
        MEM_writeLE32(memPtr, val as u32);
    } else {
        MEM_writeLE64(memPtr, val);
    };
}
unsafe extern "C" fn ERR_isError(mut code: libc::size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as libc::size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.leading_zeros() as i32 as libc::c_uint;
}
const fn ZSTD_countLeadingZeros32_const(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return (31)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
const fn ZSTD_highbit32_const(mut val: u32) -> libc::c_uint {
    debug_assert!(val != 0 as libc::c_int as libc::c_uint);
    return (31)
        .wrapping_sub(ZSTD_countLeadingZeros32_const(val));
}
pub const HUF_BLOCKSIZE_MAX: libc::c_int = 128 as libc::c_int * 1024;
pub const HUF_TABLELOG_MAX: libc::c_int = 12 as libc::c_int;
pub const HUF_TABLELOG_DEFAULT: libc::c_int = 11 as libc::c_int;
pub const HUF_SYMBOLVALUE_MAX: libc::c_int = 255 as libc::c_int;
pub const HUF_CTABLEBOUND: libc::c_int = 129 as libc::c_int;
pub const HUF_isError: unsafe extern "C" fn(libc::size_t) -> libc::c_uint = ERR_isError;
unsafe extern "C" fn HUF_alignUpWorkspace(
    mut workspace: *mut libc::c_void,
    mut workspaceSizePtr: *mut libc::size_t,
    mut align: libc::size_t,
) -> *mut libc::c_void {
    let mask = align.wrapping_sub(1);
    let rem = workspace as libc::size_t & mask;
    let add = align.wrapping_sub(rem) & mask;
    let aligned = (workspace as *mut u8).offset(add as isize);
    debug_assert!(align & align.wrapping_sub(1)
        == 0);
    debug_assert!(align <= 8);
    if *workspaceSizePtr >= add {
        debug_assert!(add < align);
        debug_assert!(aligned as libc::size_t & mask == 0);
        *workspaceSizePtr = (*workspaceSizePtr as libc::c_ulong).wrapping_sub(add)
            ;
        return aligned as *mut libc::c_void;
    } else {
        *workspaceSizePtr = 0 as libc::c_int as libc::size_t;
        return NULL as *mut libc::c_void;
    };
}
pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: libc::c_int = 6 as libc::c_int;
unsafe extern "C" fn HUF_compressWeights(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut weightTable: *const libc::c_void,
    mut wtSize: libc::size_t,
    mut workspace: *mut libc::c_void,
    mut workspaceSize: libc::size_t,
) -> libc::size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.offset(dstSize as isize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as libc::c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as u32;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<u32>() as libc::c_ulong,
    ) as *mut HUF_CompressWeightsWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_CompressWeightsWksp>()
    {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    if wtSize <= 1 {
        return 0 as libc::c_int as libc::size_t;
    }
    let maxCount = HIST_count_simple(
        ((*wksp).count).as_mut_ptr(),
        &mut maxSymbolValue,
        weightTable,
        wtSize,
    );
    if maxCount as libc::c_ulong == wtSize {
        return 1 as libc::c_int as libc::size_t;
    }
    if maxCount == 1 {
        return 0 as libc::c_int as libc::size_t;
    }
    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    let _var_err__ = FSE_normalizeCount(
        ((*wksp).norm).as_mut_ptr(),
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        0 as libc::c_int as libc::c_uint,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let hSize = FSE_writeNCount(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    op = op.offset(hSize as isize);
    let _var_err___0 = FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[u32; 41]>(),
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let cSize = FSE_compress_usingCTable(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(cSize as isize);
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
unsafe extern "C" fn HUF_getNbBits(mut elt: HUF_CElt) -> libc::size_t {
    return elt & 0xff as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn HUF_getNbBitsFast(mut elt: HUF_CElt) -> libc::size_t {
    return elt;
}
unsafe extern "C" fn HUF_getValue(mut elt: HUF_CElt) -> libc::size_t {
    return elt & !(0xff as libc::c_int as libc::size_t);
}
unsafe extern "C" fn HUF_getValueFast(mut elt: HUF_CElt) -> libc::size_t {
    return elt;
}
unsafe extern "C" fn HUF_setNbBits(mut elt: *mut HUF_CElt, mut nbBits: libc::size_t) {
    debug_assert!(nbBits <= 12);
    *elt = nbBits;
}
unsafe extern "C" fn HUF_setValue(mut elt: *mut HUF_CElt, mut value: libc::size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        debug_assert!(value >> nbBits == 0);
        *elt
            |= value
                << (::core::mem::size_of::<HUF_CElt>())
                    .wrapping_mul(8)
                    .wrapping_sub(nbBits);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUF_writeCTable_wksp(
    mut dst: *mut libc::c_void,
    mut maxDstSize: libc::size_t,
    mut CTable: *const HUF_CElt,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut workspace: *mut libc::c_void,
    mut workspaceSize: libc::size_t,
) -> libc::size_t {
    let ct = CTable.offset(1);
    let mut op = dst as *mut u8;
    let mut n: u32 = 0;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<u32>() as libc::c_ulong,
    ) as *mut HUF_WriteCTableWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_WriteCTableWksp>() {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as libc::size_t;
    }
    (*wksp).bitsToWeight[0 as libc::c_int as usize] = 0 as libc::c_int as u8;
    n = 1 as libc::c_int as u32;
    while n < huffLog.wrapping_add(1) {
        (*wksp)
            .bitsToWeight[n
            as usize] = huffLog
            .wrapping_add(1)
            .wrapping_sub(n) as u8;
        n = n.wrapping_add(1);
    }
    n = 0 as libc::c_int as u32;
    while n < maxSymbolValue {
        (*wksp)
            .huffWeight[n
            as usize] = (*wksp)
            .bitsToWeight[HUF_getNbBits(*ct.offset(n as isize)) as usize];
        n = n.wrapping_add(1);
    }
    if maxDstSize < 1 {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    let hSize = HUF_compressWeights(
        op.offset(1) as *mut libc::c_void,
        maxDstSize.wrapping_sub(1),
        ((*wksp).huffWeight).as_mut_ptr() as *const libc::c_void,
        maxSymbolValue as libc::size_t,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut libc::c_void,
        ::core::mem::size_of::<HUF_CompressWeightsWksp>(),
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if (hSize > 1) as libc::c_int
        & (hSize
            < maxSymbolValue.wrapping_div(2)
                as libc::c_ulong) as libc::c_int != 0
    {
        *op.offset(0) = hSize as u8;
        return hSize.wrapping_add(1);
    }
    if maxSymbolValue > (256 as libc::c_int - 128 as libc::c_int) as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    if maxSymbolValue
        .wrapping_add(1)
        .wrapping_div(2)
        .wrapping_add(1) as libc::c_ulong > maxDstSize
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    *op
        .offset(
            0 as libc::c_int as isize,
        ) = (128)
        .wrapping_add(maxSymbolValue.wrapping_sub(1))
        as u8;
    (*wksp).huffWeight[maxSymbolValue as usize] = 0 as libc::c_int as u8;
    n = 0 as libc::c_int as u32;
    while n < maxSymbolValue {
        *op
            .offset(
                n
                    .wrapping_div(2)
                    .wrapping_add(1) as isize,
            ) = ((((*wksp).huffWeight[n as usize] as libc::c_int) << 4 as libc::c_int)
            + (*wksp)
                .huffWeight[n.wrapping_add(1) as usize]
                as libc::c_int) as u8;
        n = (n as libc::c_uint).wrapping_add(2) as u32
            as u32;
    }
    return maxSymbolValue
        .wrapping_add(1)
        .wrapping_div(2)
        .wrapping_add(1) as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTable(
    mut CTable: *mut HUF_CElt,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut hasZeroWeights: *mut libc::c_uint,
) -> libc::size_t {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 13] = [0; 13];
    let mut tableLog = 0 as libc::c_int as u32;
    let mut nbSymbols = 0 as libc::c_int as u32;
    let ct = CTable.offset(1);
    let readSize = HUF_readStats(
        huffWeight.as_mut_ptr(),
        (255 as libc::c_int + 1) as libc::size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if ERR_isError(readSize) != 0 {
        return readSize;
    }
    *hasZeroWeights = (rankVal[0 as libc::c_int as usize]
        > 0) as libc::c_int as libc::c_uint;
    if tableLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    if nbSymbols > (*maxSymbolValuePtr).wrapping_add(1) {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as libc::size_t;
    }
    *CTable.offset(0) = tableLog as HUF_CElt;
    let mut n: u32 = 0;
    let mut nextRankStart = 0 as libc::c_int as u32;
    n = 1 as libc::c_int as u32;
    while n <= tableLog {
        let mut curr = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(
                rankVal[n as usize] << n.wrapping_sub(1),
            ) ;
        rankVal[n as usize] = curr;
        n = n.wrapping_add(1);
    }
    let mut n_0: u32 = 0;
    n_0 = 0 as libc::c_int as u32;
    while n_0 < nbSymbols {
        let w = huffWeight[n_0 as usize] as u32;
        HUF_setNbBits(
            ct.offset(n_0 as isize),
            (tableLog.wrapping_add(1).wrapping_sub(w)
                as u8 as libc::c_int
                & -((w != 0 as libc::c_int as libc::c_uint) as libc::c_int)) as libc::size_t,
        );
        n_0 = n_0.wrapping_add(1);
    }
    let mut nbPerRank: [u16; 14] = [
        0 as libc::c_int as u16,
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
        0 as libc::c_int as u16,
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
    n_1 = 0 as libc::c_int as u32;
    while n_1 < nbSymbols {
        nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize))
            as usize] = (nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize)) as usize])
            .wrapping_add(1);
        n_1 = n_1.wrapping_add(1);
    }
    valPerRank[tableLog.wrapping_add(1)
        as usize] = 0 as libc::c_int as u16;
    let mut min = 0 as libc::c_int as u16;
    let mut n_2: u32 = 0;
    n_2 = tableLog;
    while n_2 > 0 {
        valPerRank[n_2 as usize] = min;
        min = (min as libc::c_int + nbPerRank[n_2 as usize] as libc::c_int) as u16;
        min = (min as libc::c_int >> 1 as libc::c_int) as u16;
        n_2 = n_2.wrapping_sub(1);
    }
    let mut n_3: u32 = 0;
    n_3 = 0 as libc::c_int as u32;
    while n_3 < nbSymbols {
        let fresh0 = valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize)) as usize];
        valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize))
            as usize] = (valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize)) as usize])
            .wrapping_add(1);
        HUF_setValue(ct.offset(n_3 as isize), fresh0 as libc::size_t);
        n_3 = n_3.wrapping_add(1);
    }
    *maxSymbolValuePtr = nbSymbols.wrapping_sub(1);
    return readSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_getNbBitsFromCTable(
    mut CTable: *const HUF_CElt,
    mut symbolValue: u32,
) -> u32 {
    let ct = CTable.offset(1);
    debug_assert!(symbolValue <= 255);
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
    let mut totalCost = 0 as libc::c_int;
    let baseCost = ((1) << largestBits.wrapping_sub(targetNbBits)) as u32;
    let mut n = lastNonNull as libc::c_int;
    while (*huffNode.offset(n as isize)).nbBits as libc::c_uint > targetNbBits {
        totalCost = (totalCost as libc::c_uint)
            .wrapping_add(
                baseCost
                    .wrapping_sub(
                        ((1)
                            << largestBits
                                .wrapping_sub(
                                    (*huffNode.offset(n as isize)).nbBits as libc::c_uint,
                                )) as libc::c_uint,
                    ),
            ) ;
        (*huffNode.offset(n as isize)).nbBits = targetNbBits as u8;
        n -= 1;
    }
    debug_assert!((*huffNode.offset(n as isize)).nbBits as libc::c_uint <= targetNbBits);
    while (*huffNode.offset(n as isize)).nbBits as libc::c_uint == targetNbBits {
        n -= 1;
    }
    debug_assert!(totalCost as u32 & baseCost.wrapping_sub(1)
        == 0);
    totalCost >>= largestBits.wrapping_sub(targetNbBits);
    debug_assert!(totalCost > 0);
    let noSymbol = 0xf0f0f0f0 as libc::c_uint;
    let mut rankLast: [u32; 14] = [0; 14];
    libc::memset(
        rankLast.as_mut_ptr() as *mut libc::c_void,
        0xf0 as libc::c_int,
        ::core::mem::size_of::<[u32; 14]>() as libc::size_t,
    );
    let mut currentNbBits = targetNbBits;
    let mut pos: libc::c_int = 0;
    pos = n;
    while pos >= 0 {
        if !((*huffNode.offset(pos as isize)).nbBits as libc::c_uint >= currentNbBits) {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as u32;
            rankLast[targetNbBits.wrapping_sub(currentNbBits) as usize] = pos as u32;
        }
        pos -= 1;
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
                let lowTotal = (2)
                    .wrapping_mul((*huffNode.offset(lowPos as isize)).count);
                if highTotal <= lowTotal {
                    break;
                }
            }
            nBitsToDecrease = nBitsToDecrease.wrapping_sub(1);
        }
        debug_assert!(rankLast[nBitsToDecrease as usize] != noSymbol
            || nBitsToDecrease == 1);
        while nBitsToDecrease <= HUF_TABLELOG_MAX as libc::c_uint
            && rankLast[nBitsToDecrease as usize] == noSymbol
        {
            nBitsToDecrease = nBitsToDecrease.wrapping_add(1);
        }
        debug_assert!(rankLast[nBitsToDecrease as usize] != noSymbol);
        totalCost
            -= (1)
                << nBitsToDecrease.wrapping_sub(1);
        let ref mut fresh1 = (*huffNode
            .offset(rankLast[nBitsToDecrease as usize] as isize))
            .nbBits;
        *fresh1 = (*fresh1).wrapping_add(1);
        if rankLast[nBitsToDecrease.wrapping_sub(1)
            as usize] == noSymbol
        {
            rankLast[nBitsToDecrease.wrapping_sub(1)
                as usize] = rankLast[nBitsToDecrease as usize];
        }
        if rankLast[nBitsToDecrease as usize] == 0 {
            rankLast[nBitsToDecrease as usize] = noSymbol;
        } else {
            rankLast[nBitsToDecrease
                as usize] = (rankLast[nBitsToDecrease as usize]).wrapping_sub(1);
            if (*huffNode.offset(rankLast[nBitsToDecrease as usize] as isize)).nbBits
                as libc::c_uint != targetNbBits.wrapping_sub(nBitsToDecrease)
            {
                rankLast[nBitsToDecrease as usize] = noSymbol;
            }
        }
    }
    while totalCost < 0 {
        if rankLast[1 as libc::c_int as usize] == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as libc::c_uint == targetNbBits {
                n -= 1;
            }
            let ref mut fresh2 = (*huffNode.offset((n + 1) as isize))
                .nbBits;
            *fresh2 = (*fresh2).wrapping_sub(1);
            debug_assert!(n >= 0);
            rankLast[1 as libc::c_int as usize] = (n + 1) as u32;
            totalCost += 1;
        } else {
            let ref mut fresh3 = (*huffNode
                .offset(
                    (rankLast[1 as libc::c_int as usize])
                        .wrapping_add(1) as isize,
                ))
                .nbBits;
            *fresh3 = (*fresh3).wrapping_sub(1);
            rankLast[1 as libc::c_int
                as usize] = (rankLast[1 as libc::c_int as usize]).wrapping_add(1);
            totalCost += 1;
        }
    }
    return targetNbBits;
}
pub const RANK_POSITION_TABLE_SIZE: libc::c_int = 192 as libc::c_int;
pub const RANK_POSITION_MAX_COUNT_LOG: libc::c_int = 32 as libc::c_int;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: libc::c_int = RANK_POSITION_TABLE_SIZE
    - 1 as libc::c_int - RANK_POSITION_MAX_COUNT_LOG - 1 as libc::c_int;
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: libc::c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN
    as libc::c_uint)
    .wrapping_add(ZSTD_highbit32_const(RANK_POSITION_LOG_BUCKETS_BEGIN as u32));
unsafe extern "C" fn HUF_getIndex(count: u32) -> u32 {
    return if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count))
            .wrapping_add(RANK_POSITION_LOG_BUCKETS_BEGIN as libc::c_uint)
    };
}
unsafe extern "C" fn HUF_swapNodes(mut a: *mut nodeElt, mut b: *mut nodeElt) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}
#[inline]
unsafe extern "C" fn HUF_isSorted(
    mut huffNode: *mut nodeElt,
    maxSymbolValue1: u32,
) -> libc::c_int {
    let mut i: u32 = 0;
    i = 1 as libc::c_int as u32;
    while i < maxSymbolValue1 {
        if (*huffNode.offset(i as isize)).count
            > (*huffNode
                .offset(i.wrapping_sub(1) as isize))
                .count
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn HUF_insertionSort(
    mut huffNode: *mut nodeElt,
    low: libc::c_int,
    high: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let size = high - low + 1;
    huffNode = huffNode.offset(low as isize);
    i = 1 as libc::c_int;
    while i < size {
        let key = *huffNode.offset(i as isize);
        let mut j = i - 1 as libc::c_int;
        while j >= 0 && (*huffNode.offset(j as isize)).count < key.count {
            *huffNode
                .offset((j + 1) as isize) = *huffNode.offset(j as isize);
            j -= 1;
        }
        *huffNode.offset((j + 1) as isize) = key;
        i += 1;
    }
}
unsafe extern "C" fn HUF_quickSortPartition(
    mut arr: *mut nodeElt,
    low: libc::c_int,
    high: libc::c_int,
) -> libc::c_int {
    let pivot = (*arr.offset(high as isize)).count;
    let mut i = low - 1 as libc::c_int;
    let mut j = low;
    while j < high {
        if (*arr.offset(j as isize)).count > pivot {
            i += 1;
            HUF_swapNodes(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1;
    }
    HUF_swapNodes(
        &mut *arr.offset((i + 1) as isize),
        &mut *arr.offset(high as isize),
    );
    return i + 1;
}
unsafe extern "C" fn HUF_simpleQuickSort(
    mut arr: *mut nodeElt,
    mut low: libc::c_int,
    mut high: libc::c_int,
) {
    let kInsertionSortThreshold = 8 as libc::c_int;
    if high - low < kInsertionSortThreshold {
        HUF_insertionSort(arr, low, high);
        return;
    }
    while low < high {
        let idx = HUF_quickSortPartition(arr, low, high);
        if idx - low < high - idx {
            HUF_simpleQuickSort(arr, low, idx - 1 as libc::c_int);
            low = idx + 1;
        } else {
            HUF_simpleQuickSort(arr, idx + 1, high);
            high = idx - 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn HUF_sort(
    mut huffNode: *mut nodeElt,
    mut count: *const libc::c_uint,
    maxSymbolValue: u32,
    mut rankPosition: *mut rankPos,
) {
    let mut n: u32 = 0;
    let maxSymbolValue1 = maxSymbolValue.wrapping_add(1);
    libc::memset(
        rankPosition as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<rankPos>())
            .wrapping_mul(192) as libc::size_t,
    );
    n = 0 as libc::c_int as u32;
    while n < maxSymbolValue1 {
        let mut lowerRank = HUF_getIndex(*count.offset(n as isize));
        debug_assert!(lowerRank < (192 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
        let ref mut fresh4 = (*rankPosition.offset(lowerRank as isize)).base;
        *fresh4 = (*fresh4).wrapping_add(1);
        n = n.wrapping_add(1);
    }
    debug_assert!((*rankPosition.offset((192 as libc::c_int - 1 as libc::c_int) as isize)).base
        as libc::c_int == 0);
    n = (RANK_POSITION_TABLE_SIZE - 1 as libc::c_int) as u32;
    while n > 0 {
        let ref mut fresh5 = (*rankPosition
            .offset(n.wrapping_sub(1) as isize))
            .base;
        *fresh5 = (*fresh5 as libc::c_int
            + (*rankPosition.offset(n as isize)).base as libc::c_int) as u16;
        (*rankPosition.offset(n.wrapping_sub(1) as isize))
            .curr = (*rankPosition
            .offset(n.wrapping_sub(1) as isize))
            .base;
        n = n.wrapping_sub(1);
    }
    n = 0 as libc::c_int as u32;
    while n < maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)).wrapping_add(1);
        let ref mut fresh6 = (*rankPosition.offset(r as isize)).curr;
        let fresh7 = *fresh6;
        *fresh6 = (*fresh6).wrapping_add(1);
        let pos = fresh7 as u32;
        debug_assert!(pos < maxSymbolValue1);
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as u8;
        n = n.wrapping_add(1);
    }
    n = RANK_POSITION_DISTINCT_COUNT_CUTOFF;
    while n < (RANK_POSITION_TABLE_SIZE - 1 as libc::c_int) as libc::c_uint {
        let bucketSize = (*rankPosition.offset(n as isize)).curr as libc::c_int
            - (*rankPosition.offset(n as isize)).base as libc::c_int;
        let bucketStartIdx = (*rankPosition.offset(n as isize)).base as u32;
        if bucketSize > 1 {
            debug_assert!(bucketStartIdx < maxSymbolValue1);
            HUF_simpleQuickSort(
                huffNode.offset(bucketStartIdx as isize),
                0 as libc::c_int,
                bucketSize - 1 as libc::c_int,
            );
        }
        n = n.wrapping_add(1);
    }
    debug_assert!(HUF_isSorted(huffNode, maxSymbolValue1) != 0);;
}
pub const STARTNODE: libc::c_int = HUF_SYMBOLVALUE_MAX + 1;
unsafe extern "C" fn HUF_buildTree(
    mut huffNode: *mut nodeElt,
    mut maxSymbolValue: u32,
) -> libc::c_int {
    let huffNode0 = huffNode.offset(-(1));
    let mut nonNullRank: libc::c_int = 0;
    let mut lowS: libc::c_int = 0;
    let mut lowN: libc::c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: libc::c_int = 0;
    let mut nodeRoot: libc::c_int = 0;
    nonNullRank = maxSymbolValue as libc::c_int;
    while (*huffNode.offset(nonNullRank as isize)).count
        == 0
    {
        nonNullRank -= 1;
    }
    lowS = nonNullRank;
    nodeRoot = nodeNb + lowS - 1 as libc::c_int;
    lowN = nodeNb;
    (*huffNode.offset(nodeNb as isize))
        .count = ((*huffNode.offset(lowS as isize)).count)
        .wrapping_add((*huffNode.offset((lowS - 1 as libc::c_int) as isize)).count);
    let ref mut fresh8 = (*huffNode.offset((lowS - 1 as libc::c_int) as isize)).parent;
    *fresh8 = nodeNb as u16;
    (*huffNode.offset(lowS as isize)).parent = *fresh8;
    nodeNb += 1;
    lowS -= 2 as libc::c_int;
    n = nodeNb;
    while n <= nodeRoot {
        (*huffNode.offset(n as isize)).count = (1) << 30 as libc::c_int;
        n += 1;
    }
    (*huffNode0.offset(0))
        .count = (1) << 31 as libc::c_int;
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
    }
    (*huffNode.offset(nodeRoot as isize)).nbBits = 0 as libc::c_int as u8;
    n = nodeRoot - 1 as libc::c_int;
    while n >= STARTNODE {
        (*huffNode.offset(n as isize))
            .nbBits = ((*huffNode.offset((*huffNode.offset(n as isize)).parent as isize))
            .nbBits as libc::c_int + 1) as u8;
        n -= 1;
    }
    n = 0 as libc::c_int;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize))
            .nbBits = ((*huffNode.offset((*huffNode.offset(n as isize)).parent as isize))
            .nbBits as libc::c_int + 1) as u8;
        n += 1;
    }
    return nonNullRank;
}
unsafe extern "C" fn HUF_buildCTableFromTree(
    mut CTable: *mut HUF_CElt,
    mut huffNode: *const nodeElt,
    mut nonNullRank: libc::c_int,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
) {
    let ct = CTable.offset(1);
    let mut n: libc::c_int = 0;
    let mut nbPerRank: [u16; 13] = [
        0 as libc::c_int as u16,
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
        0 as libc::c_int as u16,
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
        as libc::c_int;
    n = 0 as libc::c_int;
    while n <= nonNullRank {
        nbPerRank[(*huffNode.offset(n as isize)).nbBits
            as usize] = (nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize])
            .wrapping_add(1);
        n += 1;
    }
    let mut min = 0 as libc::c_int as u16;
    n = maxNbBits as libc::c_int;
    while n > 0 {
        valPerRank[n as usize] = min;
        min = (min as libc::c_int + nbPerRank[n as usize] as libc::c_int) as u16;
        min = (min as libc::c_int >> 1 as libc::c_int) as u16;
        n -= 1;
    }
    n = 0 as libc::c_int;
    while n < alphabetSize {
        HUF_setNbBits(
            ct.offset((*huffNode.offset(n as isize)).byte as libc::c_int as isize),
            (*huffNode.offset(n as isize)).nbBits as libc::size_t,
        );
        n += 1;
    }
    n = 0 as libc::c_int;
    while n < alphabetSize {
        let fresh14 = valPerRank[HUF_getNbBits(*ct.offset(n as isize)) as usize];
        valPerRank[HUF_getNbBits(*ct.offset(n as isize))
            as usize] = (valPerRank[HUF_getNbBits(*ct.offset(n as isize)) as usize])
            .wrapping_add(1);
        HUF_setValue(ct.offset(n as isize), fresh14 as libc::size_t);
        n += 1;
    }
    *CTable.offset(0) = maxNbBits as HUF_CElt;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable_wksp(
    mut CTable: *mut HUF_CElt,
    mut count: *const libc::c_uint,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
) -> libc::size_t {
    let wksp_tables = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<u32>() as libc::c_ulong,
    ) as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.offset(1);
    let mut nonNullRank: libc::c_int = 0;
    if wkspSize < ::core::mem::size_of::<HUF_buildCTable_wksp_tables>()
    {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as libc::size_t;
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT as u32;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as libc::size_t;
    }
    libc::memset(
        huffNode0 as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<huffNodeTable>() as libc::size_t,
    );
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as libc::size_t;
    }
    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);
    return maxNbBits as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_estimateCompressedSize(
    mut CTable: *const HUF_CElt,
    mut count: *const libc::c_uint,
    mut maxSymbolValue: libc::c_uint,
) -> libc::size_t {
    let mut ct = CTable.offset(1);
    let mut nbBits = 0 as libc::c_int as libc::size_t;
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s <= maxSymbolValue as libc::c_int {
        nbBits = (nbBits as libc::c_ulong)
            .wrapping_add(
                (HUF_getNbBits(*ct.offset(s as isize)))
                    .wrapping_mul(*count.offset(s as isize) as libc::c_ulong),
            ) ;
        s += 1;
    }
    return nbBits >> 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_validateCTable(
    mut CTable: *const HUF_CElt,
    mut count: *const libc::c_uint,
    mut maxSymbolValue: libc::c_uint,
) -> libc::c_int {
    let mut ct = CTable.offset(1);
    let mut bad = 0 as libc::c_int;
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s <= maxSymbolValue as libc::c_int {
        bad
            |= (*count.offset(s as isize) != 0 as libc::c_int as libc::c_uint)
                as libc::c_int
                & (HUF_getNbBits(*ct.offset(s as isize))
                    == 0) as libc::c_int;
        s += 1;
    }
    return (bad == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compressBound(mut size: libc::size_t) -> libc::size_t {
    return (HUF_CTABLEBOUND as libc::c_ulong)
        .wrapping_add(
            size
                .wrapping_add(size >> 8 as libc::c_int)
                .wrapping_add(8),
        );
}
pub const HUF_BITS_IN_CONTAINER: libc::c_ulong = (::core::mem::size_of::<libc::size_t>()
    as libc::c_ulong)
    .wrapping_mul(8);
unsafe extern "C" fn HUF_initCStream(
    mut bitC: *mut HUF_CStream_t,
    mut startPtr: *mut libc::c_void,
    mut dstCapacity: libc::size_t,
) -> libc::size_t {
    libc::memset(
        bitC as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<HUF_CStream_t>() as libc::size_t,
    );
    (*bitC).startPtr = startPtr as *mut u8;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC)
        .endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<libc::size_t>() as isize));
    if dstCapacity <= ::core::mem::size_of::<libc::size_t>() {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as libc::size_t;
    }
    return 0 as libc::c_int as libc::size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_addBits(
    mut bitC: *mut HUF_CStream_t,
    mut elt: HUF_CElt,
    mut idx: libc::c_int,
    mut kFast: libc::c_int,
) {
    debug_assert!(idx <= 1);
    debug_assert!(HUF_getNbBits(elt) <= 12);
    (*bitC).bitContainer[idx as usize] >>= HUF_getNbBits(elt);
    (*bitC).bitContainer[idx as usize]
        |= if kFast != 0 { HUF_getValueFast(elt) } else { HUF_getValue(elt) };
    (*bitC)
        .bitPos[idx
        as usize] = ((*bitC).bitPos[idx as usize] as libc::c_ulong)
        .wrapping_add(HUF_getNbBitsFast(elt)) ;
    debug_assert!((*bitC).bitPos[idx as usize] & 0xff as libc::c_int as libc::c_ulong
        <= (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8));
    let nbBits = HUF_getNbBits(elt);
    let dirtyBits = (if nbBits == 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        (ZSTD_highbit32(nbBits as u32)).wrapping_add(1)
    }) as libc::size_t;
    debug_assert!(elt >> dirtyBits << dirtyBits.wrapping_add(nbBits)
        == 0);
    debug_assert!(kFast == 0
        || (*bitC).bitPos[idx as usize] & 0xff as libc::c_int as libc::c_ulong
            <= (::core::mem::size_of::<libc::size_t>())
                .wrapping_mul(8));;
}
#[inline(always)]
unsafe extern "C" fn HUF_zeroIndex1(mut bitC: *mut HUF_CStream_t) {
    (*bitC).bitContainer[1 as libc::c_int as usize] = 0 as libc::c_int as libc::size_t;
    (*bitC).bitPos[1 as libc::c_int as usize] = 0 as libc::c_int as libc::size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_mergeIndex1(mut bitC: *mut HUF_CStream_t) {
    debug_assert!(((*bitC).bitPos[1 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_ulong)
        < (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8));
    (*bitC).bitContainer[0 as libc::c_int as usize]
        >>= (*bitC).bitPos[1 as libc::c_int as usize]
            & 0xff as libc::c_int as libc::c_ulong;
    (*bitC).bitContainer[0 as libc::c_int as usize]
        |= (*bitC).bitContainer[1 as libc::c_int as usize];
    (*bitC)
        .bitPos[0 as libc::c_int
        as usize] = ((*bitC).bitPos[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add((*bitC).bitPos[1 as libc::c_int as usize]) ;
    debug_assert!((*bitC).bitPos[0 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_ulong
        <= (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8));;
}
#[inline(always)]
unsafe extern "C" fn HUF_flushBits(
    mut bitC: *mut HUF_CStream_t,
    mut kFast: libc::c_int,
) {
    let nbBits = (*bitC).bitPos[0 as libc::c_int as usize]
        & 0xff as libc::c_int as libc::c_ulong;
    let nbBytes = nbBits >> 3 as libc::c_int;
    let bitContainer = (*bitC).bitContainer[0 as libc::c_int as usize]
        >> HUF_BITS_IN_CONTAINER.wrapping_sub(nbBits);
    (*bitC).bitPos[0 as libc::c_int as usize] &= 7;
    debug_assert!(nbBits > 0);
    debug_assert!(nbBits
        <= (::core::mem::size_of::<libc::size_t>())
            .wrapping_mul(8));
    debug_assert!((*bitC).ptr <= (*bitC).endPtr);
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    debug_assert!(kFast == 0 || (*bitC).ptr <= (*bitC).endPtr);
    if kFast == 0 && (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
}
unsafe extern "C" fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1 as libc::c_int as libc::size_t);
    HUF_setValue(&mut endMark, 1 as libc::c_int as libc::size_t);
    return endMark;
}
unsafe extern "C" fn HUF_closeCStream(mut bitC: *mut HUF_CStream_t) -> libc::size_t {
    HUF_addBits(bitC, HUF_endMark(), 0 as libc::c_int, 0 as libc::c_int);
    HUF_flushBits(bitC, 0 as libc::c_int);
    let nbBits = (*bitC).bitPos[0 as libc::c_int as usize]
        & 0xff as libc::c_int as libc::c_ulong;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0 as libc::c_int as libc::size_t;
    }
    return (((*bitC).ptr).offset_from((*bitC).startPtr) as libc::c_long as libc::size_t)
        .wrapping_add(
            (nbBits > 0) as libc::c_int as libc::c_ulong,
        );
}
#[inline(always)]
unsafe extern "C" fn HUF_encodeSymbol(
    mut bitCPtr: *mut HUF_CStream_t,
    mut symbol: u32,
    mut CTable: *const HUF_CElt,
    mut idx: libc::c_int,
    mut fast: libc::c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body_loop(
    mut bitC: *mut HUF_CStream_t,
    mut ip: *const u8,
    mut srcSize: libc::size_t,
    mut ct: *const HUF_CElt,
    mut kUnroll: libc::c_int,
    mut kFastFlush: libc::c_int,
    mut kLastFast: libc::c_int,
) {
    let mut n = srcSize as libc::c_int;
    let mut rem = n % kUnroll;
    if rem > 0 {
        while rem > 0 {
            n -= 1;
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n as isize) as u32,
                ct,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            rem -= 1;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    debug_assert!(n % kUnroll == 0);
    if n % (2 as libc::c_int * kUnroll) != 0 {
        let mut u: libc::c_int = 0;
        u = 1 as libc::c_int;
        while u < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u) as isize) as u32,
                ct,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            u += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as u32,
            ct,
            0 as libc::c_int,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        n -= kUnroll;
    }
    debug_assert!(n % (2 as libc::c_int * kUnroll) == 0);
    while n > 0 {
        let mut u_0: libc::c_int = 0;
        u_0 = 1 as libc::c_int;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u_0) as isize) as u32,
                ct,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            u_0 += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as u32,
            ct,
            0 as libc::c_int,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        HUF_zeroIndex1(bitC);
        u_0 = 1 as libc::c_int;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - kUnroll - u_0) as isize) as u32,
                ct,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            u_0 += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll - kUnroll) as isize) as u32,
            ct,
            1 as libc::c_int,
            kLastFast,
        );
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 as libc::c_int * kUnroll;
    }
    debug_assert!(n == 0);;
}
unsafe extern "C" fn HUF_tightCompressBound(
    mut srcSize: libc::size_t,
    mut tableLog: libc::size_t,
) -> libc::size_t {
    return (srcSize.wrapping_mul(tableLog) >> 3 as libc::c_int)
        .wrapping_add(8);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
) -> libc::size_t {
    let tableLog = *CTable.offset(0) as u32;
    let mut ct = CTable.offset(1);
    let mut ip = src as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    let mut bitC = HUF_CStream_t {
        bitContainer: [0; 2],
        bitPos: [0; 2],
        startPtr: 0 as *mut u8,
        ptr: 0 as *mut u8,
        endPtr: 0 as *mut u8,
    };
    if dstSize < 8 {
        return 0 as libc::c_int as libc::size_t;
    }
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
    );
    if ERR_isError(initErr) != 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as libc::size_t)
        || tableLog > 11
    {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct,
            if MEM_32bits() != 0 { 2 as libc::c_int } else { 4 as libc::c_int },
            0 as libc::c_int,
            0 as libc::c_int,
        );
    } else if MEM_32bits() != 0 {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            10 | 9 | 8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            7 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    3 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
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
                    5 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    5 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            9 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    6 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    7 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            7 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    8 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            6 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    9 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
            }
        }
    }
    debug_assert!(bitC.ptr <= bitC.endPtr);
    return HUF_closeCStream(&mut bitC);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_bmi2(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
) -> libc::size_t {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_default(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
) -> libc::size_t {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
    flags: libc::c_int,
) -> libc::size_t {
    if flags & HUF_flags_bmi2 as libc::c_int != 0 {
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
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
    mut flags: libc::c_int,
) -> libc::size_t {
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
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
    mut flags: libc::c_int,
) -> libc::size_t {
    let segmentSize = srcSize
        .wrapping_add(3)
        .wrapping_div(4);
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if dstSize
        < (6 as libc::c_int + 1 + 1 + 1
            + 8) as libc::c_ulong
    {
        return 0 as libc::c_int as libc::size_t;
    }
    if srcSize < 12 {
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(6);
    debug_assert!(op <= oend);
    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        ip as *const libc::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0
        || cSize > 65535
    {
        return 0 as libc::c_int as libc::size_t;
    }
    MEM_writeLE16(ostart as *mut libc::c_void, cSize as u16);
    op = op.offset(cSize as isize);
    ip = ip.offset(segmentSize as isize);
    debug_assert!(op <= oend);
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        ip as *const libc::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_0) != 0 {
        return cSize_0;
    }
    if cSize_0 == 0
        || cSize_0 > 65535
    {
        return 0 as libc::c_int as libc::size_t;
    }
    MEM_writeLE16(
        ostart.offset(2) as *mut libc::c_void,
        cSize_0 as u16,
    );
    op = op.offset(cSize_0 as isize);
    ip = ip.offset(segmentSize as isize);
    debug_assert!(op <= oend);
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        ip as *const libc::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_1) != 0 {
        return cSize_1;
    }
    if cSize_1 == 0
        || cSize_1 > 65535
    {
        return 0 as libc::c_int as libc::size_t;
    }
    MEM_writeLE16(
        ostart.offset(4) as *mut libc::c_void,
        cSize_1 as u16,
    );
    op = op.offset(cSize_1 as isize);
    ip = ip.offset(segmentSize as isize);
    debug_assert!(op <= oend);
    debug_assert!(ip <= iend);
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as libc::size_t,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as libc::size_t,
        CTable,
        flags,
    );
    if ERR_isError(cSize_2) != 0 {
        return cSize_2;
    }
    if cSize_2 == 0
        || cSize_2 > 65535
    {
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(cSize_2 as isize);
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_usingCTable(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut CTable: *const HUF_CElt,
    mut flags: libc::c_int,
) -> libc::size_t {
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
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut nbStreams: HUF_nbStreams_e,
    mut CTable: *const HUF_CElt,
    flags: libc::c_int,
) -> libc::size_t {
    let cSize = if nbStreams as libc::c_uint
        == HUF_singleStream as libc::c_int as libc::c_uint
    {
        HUF_compress1X_usingCTable_internal(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as libc::size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as libc::size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(cSize as isize);
    debug_assert!(op >= ostart);
    if op.offset_from(ostart) as libc::c_long as libc::size_t
        >= srcSize.wrapping_sub(1)
    {
        return 0 as libc::c_int as libc::size_t;
    }
    return op.offset_from(ostart) as libc::c_long as libc::size_t;
}
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: libc::c_int = 4096 as libc::c_int;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: libc::c_int = 10 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn HUF_cardinality(
    mut count: *const libc::c_uint,
    mut maxSymbolValue: libc::c_uint,
) -> libc::c_uint {
    let mut cardinality = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < maxSymbolValue.wrapping_add(1) {
        if *count.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            cardinality = cardinality.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return cardinality;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_minTableLog(
    mut symbolCardinality: libc::c_uint,
) -> libc::c_uint {
    let mut minBitsSymbols = (ZSTD_highbit32(symbolCardinality))
        .wrapping_add(1);
    return minBitsSymbols;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_optimalTableLog(
    mut maxTableLog: libc::c_uint,
    mut srcSize: libc::size_t,
    mut maxSymbolValue: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut table: *mut HUF_CElt,
    mut count: *const libc::c_uint,
    mut flags: libc::c_int,
) -> libc::c_uint {
    debug_assert!(srcSize > 1);
    debug_assert!(wkspSize >= ::core::mem::size_of::<HUF_buildCTable_wksp_tables>());
    if flags & HUF_flags_optimalDepth as libc::c_int == 0 {
        return FSE_optimalTableLog_internal(
            maxTableLog,
            srcSize,
            maxSymbolValue,
            1 as libc::c_int as libc::c_uint,
        );
    }
    let mut dst = (workSpace as *mut u8)
        .offset(::core::mem::size_of::<HUF_WriteCTableWksp>() as isize);
    let mut dstSize = wkspSize
        .wrapping_sub(::core::mem::size_of::<HUF_WriteCTableWksp>());
    let mut maxBits: libc::size_t = 0;
    let mut hSize: libc::size_t = 0;
    let mut newSize: libc::size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0) as libc::size_t)
        .wrapping_sub(1);
    let mut optLog = maxTableLog;
    let mut optLogGuess: libc::c_uint = 0;
    optLogGuess = minTableLog;
    while optLogGuess <= maxTableLog {
        maxBits = HUF_buildCTable_wksp(
            table,
            count,
            maxSymbolValue,
            optLogGuess,
            workSpace,
            wkspSize,
        );
        if !(ERR_isError(maxBits) != 0) {
            if maxBits < optLogGuess as libc::c_ulong && optLogGuess > minTableLog {
                break;
            }
            hSize = HUF_writeCTable_wksp(
                dst as *mut libc::c_void,
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
    }
    debug_assert!(optLog <= 12);
    return optLog;
}
unsafe extern "C" fn HUF_compress_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut nbStreams: HUF_nbStreams_e,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut oldHufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: libc::c_int,
) -> libc::size_t {
    let table = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<libc::size_t>() as libc::c_ulong,
    ) as *mut HUF_compress_tables_t;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if wkspSize < ::core::mem::size_of::<HUF_compress_tables_t>() {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as libc::size_t;
    }
    if srcSize == 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    if dstSize == 0 {
        return 0 as libc::c_int as libc::size_t;
    }
    if srcSize > HUF_BLOCKSIZE_MAX as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as libc::size_t;
    }
    if huffLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as libc::size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as libc::size_t;
    }
    if maxSymbolValue == 0 {
        maxSymbolValue = HUF_SYMBOLVALUE_MAX as libc::c_uint;
    }
    if huffLog == 0 {
        huffLog = HUF_TABLELOG_DEFAULT as libc::c_uint;
    }
    if flags & HUF_flags_preferRepeat as libc::c_int != 0 && !repeat.is_null()
        && *repeat as libc::c_uint == HUF_repeat_valid as libc::c_int as libc::c_uint
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
    if flags & HUF_flags_suspectUncompressible as libc::c_int != 0
        && srcSize
            >= (SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO)
                as libc::c_ulong
    {
        let mut largestTotal = 0 as libc::c_int as libc::size_t;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const u8 as *const libc::c_void,
            4096 as libc::c_int as libc::size_t,
        ) as libc::size_t;
        if ERR_isError(largestBegin) != 0 {
            return largestBegin;
        }
        largestTotal = (largestTotal as libc::c_ulong).wrapping_add(largestBegin)
            ;
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const u8)
                .offset(srcSize as isize)
                .offset(-(4096)) as *const libc::c_void,
            4096 as libc::c_int as libc::size_t,
        ) as libc::size_t;
        if ERR_isError(largestEnd) != 0 {
            return largestEnd;
        }
        largestTotal = (largestTotal as libc::c_ulong).wrapping_add(largestEnd) as libc::size_t
            as libc::size_t;
        if largestTotal
            <= ((2 as libc::c_int * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE
                >> 7 as libc::c_int) + 4) as libc::c_ulong
        {
            return 0 as libc::c_int as libc::size_t;
        }
    }
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const libc::c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[u32; 1024]>(),
    );
    if ERR_isError(largest) != 0 {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const u8).offset(0);
        return 1 as libc::c_int as libc::size_t;
    }
    if largest
        <= (srcSize >> 7 as libc::c_int).wrapping_add(4)
    {
        return 0 as libc::c_int as libc::size_t;
    }
    if !repeat.is_null()
        && *repeat as libc::c_uint == HUF_repeat_check as libc::c_int as libc::c_uint
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue)
            == 0
    {
        *repeat = HUF_repeat_none;
    }
    if flags & HUF_flags_preferRepeat as libc::c_int != 0 && !repeat.is_null()
        && *repeat as libc::c_uint != HUF_repeat_none as libc::c_int as libc::c_uint
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
        &mut (*table).wksps as *mut C2RustUnnamed_1 as *mut libc::c_void,
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
            as *mut libc::c_void,
        ::core::mem::size_of::<HUF_buildCTable_wksp_tables>(),
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    huffLog = maxBits as u32;
    let ctableSize = maxSymbolValue.wrapping_add(2)
        as libc::size_t;
    let unusedSize = (::core::mem::size_of::<[HUF_CElt; 257]>())
        .wrapping_sub(
            ctableSize.wrapping_mul(::core::mem::size_of::<HUF_CElt>()),
        );
    libc::memset(
        ((*table).CTable).as_mut_ptr().offset(ctableSize as isize) as *mut libc::c_void,
        0 as libc::c_int,
        unusedSize as libc::size_t,
    );
    let hSize = HUF_writeCTable_wksp(
        op as *mut libc::c_void,
        dstSize,
        ((*table).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp
            as *mut libc::c_void,
        ::core::mem::size_of::<HUF_WriteCTableWksp>(),
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if !repeat.is_null()
        && *repeat as libc::c_uint != HUF_repeat_none as libc::c_int as libc::c_uint
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
        return 0 as libc::c_int as libc::size_t;
    }
    op = op.offset(hSize as isize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut libc::c_void,
            ((*table).CTable).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[HUF_CElt; 257]>() as libc::size_t,
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
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: libc::c_int,
) -> libc::size_t {
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
    mut dst: *mut libc::c_void,
    mut dstSize: libc::size_t,
    mut src: *const libc::c_void,
    mut srcSize: libc::size_t,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: libc::size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: libc::c_int,
) -> libc::size_t {
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
