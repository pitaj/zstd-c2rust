use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn HIST_count_wksp(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: size_t,
        workSpace: *mut libc::c_void,
        workSpaceSize: size_t,
    ) -> size_t;
    fn HIST_count_simple(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> libc::c_uint;
    fn FSE_optimalTableLog(
        maxTableLog: libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
    ) -> libc::c_uint;
    fn FSE_normalizeCount(
        normalizedCounter: *mut libc::c_short,
        tableLog: libc::c_uint,
        count: *const libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
        useLowProbCount: libc::c_uint,
    ) -> size_t;
    fn FSE_writeNCount(
        buffer: *mut libc::c_void,
        bufferSize: size_t,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
    ) -> size_t;
    fn FSE_compress_usingCTable(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
        ct: *const FSE_CTable,
    ) -> size_t;
    fn FSE_optimalTableLog_internal(
        maxTableLog: libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
        minus: libc::c_uint,
    ) -> libc::c_uint;
    fn FSE_buildCTable_wksp(
        ct: *mut FSE_CTable,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
        workSpace: *mut libc::c_void,
        wkspSize: size_t,
    ) -> size_t;
    fn HUF_readStats(
        huffWeight: *mut BYTE,
        hwSize: size_t,
        rankStats: *mut U32,
        nbSymbolsPtr: *mut U32,
        tableLogPtr: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type S16 = int16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type unalign64 = U64;
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
pub type HUF_CElt = size_t;
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
    pub bitsToWeight: [BYTE; 13],
    pub huffWeight: [BYTE; 255],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CompressWeightsWksp {
    pub CTable: [FSE_CTable; 59],
    pub scratchBuffer: [U32; 41],
    pub count: [libc::c_uint; 13],
    pub norm: [S16; 13],
}
pub type nodeElt = nodeElt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: U32,
    pub parent: U16,
    pub byte: BYTE,
    pub nbBits: BYTE,
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
    pub base: U16,
    pub curr: U16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [size_t; 2],
    pub bitPos: [size_t; 2],
    pub startPtr: *mut BYTE,
    pub ptr: *mut BYTE,
    pub endPtr: *mut BYTE,
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
    pub hist_wksp: [U32; 1024],
}
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<size_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void, mut value: U16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: U32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void, mut val: U16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut BYTE;
        *p.offset(0 as libc::c_int as isize) = val as BYTE;
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (val as libc::c_int >> 8 as libc::c_int) as BYTE;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: U32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void, mut val64: U64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void, mut val: size_t) {
    if MEM_32bits() != 0 {
        MEM_writeLE32(memPtr, val as U32);
    } else {
        MEM_writeLE64(memPtr, val);
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"unsigned int ZSTD_countLeadingZeros32(U32)\0"))
                .as_ptr(),
        );
    }
    return val.leading_zeros() as i32 as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> libc::c_uint {
    if val != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"val != 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/../common/bits.h\0" as *const u8
                as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"unsigned int ZSTD_highbit32(U32)\0"))
                .as_ptr(),
        );
    }
    return (31 as libc::c_int as libc::c_uint)
        .wrapping_sub(ZSTD_countLeadingZeros32(val));
}
pub const HUF_BLOCKSIZE_MAX: libc::c_int = 128 as libc::c_int * 1024 as libc::c_int;
pub const HUF_TABLELOG_MAX: libc::c_int = 12 as libc::c_int;
pub const HUF_TABLELOG_DEFAULT: libc::c_int = 11 as libc::c_int;
pub const HUF_SYMBOLVALUE_MAX: libc::c_int = 255 as libc::c_int;
pub const HUF_CTABLEBOUND: libc::c_int = 129 as libc::c_int;
pub const HUF_isError: unsafe extern "C" fn(size_t) -> libc::c_uint = ERR_isError;
unsafe extern "C" fn HUF_alignUpWorkspace(
    mut workspace: *mut libc::c_void,
    mut workspaceSizePtr: *mut size_t,
    mut align: size_t,
) -> *mut libc::c_void {
    let mask = align.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let rem = workspace as size_t & mask;
    let add = align.wrapping_sub(rem) & mask;
    let aligned = (workspace as *mut BYTE).offset(add as isize);
    if align & align.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(align & (align - 1)) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *HUF_alignUpWorkspace(void *, size_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if align <= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"align <= HUF_WORKSPACE_MAX_ALIGNMENT\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *HUF_alignUpWorkspace(void *, size_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if *workspaceSizePtr >= add {
        if add < align {} else {
            __assert_fail(
                b"add < align\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                121 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void *HUF_alignUpWorkspace(void *, size_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
        if aligned as size_t & mask == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"((size_t)aligned & mask) == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void *HUF_alignUpWorkspace(void *, size_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
        *workspaceSizePtr = (*workspaceSizePtr as libc::c_ulong).wrapping_sub(add)
            as size_t as size_t;
        return aligned as *mut libc::c_void;
    } else {
        *workspaceSizePtr = 0 as libc::c_int as size_t;
        return NULL as *mut libc::c_void;
    };
}
pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: libc::c_int = 6 as libc::c_int;
unsafe extern "C" fn HUF_compressWeights(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut weightTable: *const libc::c_void,
    mut wtSize: size_t,
    mut workspace: *mut libc::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(dstSize as isize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as libc::c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as U32;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<U32>() as libc::c_ulong,
    ) as *mut HUF_CompressWeightsWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_CompressWeightsWksp>() as libc::c_ulong
    {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if wtSize <= 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    let maxCount = HIST_count_simple(
        ((*wksp).count).as_mut_ptr(),
        &mut maxSymbolValue,
        weightTable,
        wtSize,
    );
    if maxCount as libc::c_ulong == wtSize {
        return 1 as libc::c_int as size_t;
    }
    if maxCount == 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as size_t;
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
        oend.offset_from(op) as libc::c_long as size_t,
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
        ::core::mem::size_of::<[U32; 41]>() as libc::c_ulong,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let cSize = FSE_compress_usingCTable(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    op = op.offset(cSize as isize);
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn HUF_getNbBits(mut elt: HUF_CElt) -> size_t {
    return elt & 0xff as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn HUF_getNbBitsFast(mut elt: HUF_CElt) -> size_t {
    return elt;
}
unsafe extern "C" fn HUF_getValue(mut elt: HUF_CElt) -> size_t {
    return elt & !(0xff as libc::c_int as size_t);
}
unsafe extern "C" fn HUF_getValueFast(mut elt: HUF_CElt) -> size_t {
    return elt;
}
unsafe extern "C" fn HUF_setNbBits(mut elt: *mut HUF_CElt, mut nbBits: size_t) {
    if nbBits <= 12 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nbBits <= HUF_TABLELOG_ABSOLUTEMAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void HUF_setNbBits(HUF_CElt *, size_t)\0"))
                .as_ptr(),
        );
    }
    *elt = nbBits;
}
unsafe extern "C" fn HUF_setValue(mut elt: *mut HUF_CElt, mut value: size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 as libc::c_int as libc::c_ulong {
        if value >> nbBits == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"(value >> nbBits) == 0\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void HUF_setValue(HUF_CElt *, size_t)\0"))
                    .as_ptr(),
            );
        }
        *elt
            |= value
                << (::core::mem::size_of::<HUF_CElt>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(nbBits);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUF_writeCTable_wksp(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut CTable: *const HUF_CElt,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut workspace: *mut libc::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ct = CTable.offset(1 as libc::c_int as isize);
    let mut op = dst as *mut BYTE;
    let mut n: U32 = 0;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<U32>() as libc::c_ulong,
    ) as *mut HUF_WriteCTableWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_WriteCTableWksp>() as libc::c_ulong {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    (*wksp).bitsToWeight[0 as libc::c_int as usize] = 0 as libc::c_int as BYTE;
    n = 1 as libc::c_int as U32;
    while n < huffLog.wrapping_add(1 as libc::c_int as libc::c_uint) {
        (*wksp)
            .bitsToWeight[n
            as usize] = huffLog
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(n) as BYTE;
        n = n.wrapping_add(1);
    }
    n = 0 as libc::c_int as U32;
    while n < maxSymbolValue {
        (*wksp)
            .huffWeight[n
            as usize] = (*wksp)
            .bitsToWeight[HUF_getNbBits(*ct.offset(n as isize)) as usize];
        n = n.wrapping_add(1);
    }
    if maxDstSize < 1 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    let hSize = HUF_compressWeights(
        op.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        maxDstSize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ((*wksp).huffWeight).as_mut_ptr() as *const libc::c_void,
        maxSymbolValue as size_t,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut libc::c_void,
        ::core::mem::size_of::<HUF_CompressWeightsWksp>() as libc::c_ulong,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if (hSize > 1 as libc::c_int as libc::c_ulong) as libc::c_int
        & (hSize
            < maxSymbolValue.wrapping_div(2 as libc::c_int as libc::c_uint)
                as libc::c_ulong) as libc::c_int != 0
    {
        *op.offset(0 as libc::c_int as isize) = hSize as BYTE;
        return hSize.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if maxSymbolValue > (256 as libc::c_int - 128 as libc::c_int) as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    if maxSymbolValue
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong > maxDstSize
    {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    *op
        .offset(
            0 as libc::c_int as isize,
        ) = (128 as libc::c_int as libc::c_uint)
        .wrapping_add(maxSymbolValue.wrapping_sub(1 as libc::c_int as libc::c_uint))
        as BYTE;
    (*wksp).huffWeight[maxSymbolValue as usize] = 0 as libc::c_int as BYTE;
    n = 0 as libc::c_int as U32;
    while n < maxSymbolValue {
        *op
            .offset(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = ((((*wksp).huffWeight[n as usize] as libc::c_int) << 4 as libc::c_int)
            + (*wksp)
                .huffWeight[n.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int) as BYTE;
        n = (n as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint) as U32
            as U32;
    }
    return maxSymbolValue
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTable(
    mut CTable: *mut HUF_CElt,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut hasZeroWeights: *mut libc::c_uint,
) -> size_t {
    let mut huffWeight: [BYTE; 256] = [0; 256];
    let mut rankVal: [U32; 13] = [0; 13];
    let mut tableLog = 0 as libc::c_int as U32;
    let mut nbSymbols = 0 as libc::c_int as U32;
    let ct = CTable.offset(1 as libc::c_int as isize);
    let readSize = HUF_readStats(
        huffWeight.as_mut_ptr(),
        (255 as libc::c_int + 1 as libc::c_int) as size_t,
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
        > 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint;
    if tableLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if nbSymbols > (*maxSymbolValuePtr).wrapping_add(1 as libc::c_int as libc::c_uint) {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
    }
    *CTable.offset(0 as libc::c_int as isize) = tableLog as HUF_CElt;
    let mut n: U32 = 0;
    let mut nextRankStart = 0 as libc::c_int as U32;
    n = 1 as libc::c_int as U32;
    while n <= tableLog {
        let mut curr = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(
                rankVal[n as usize] << n.wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) as U32 as U32;
        rankVal[n as usize] = curr;
        n = n.wrapping_add(1);
    }
    let mut n_0: U32 = 0;
    n_0 = 0 as libc::c_int as U32;
    while n_0 < nbSymbols {
        let w = huffWeight[n_0 as usize] as U32;
        HUF_setNbBits(
            ct.offset(n_0 as isize),
            (tableLog.wrapping_add(1 as libc::c_int as libc::c_uint).wrapping_sub(w)
                as BYTE as libc::c_int
                & -((w != 0 as libc::c_int as libc::c_uint) as libc::c_int)) as size_t,
        );
        n_0 = n_0.wrapping_add(1);
    }
    let mut nbPerRank: [U16; 14] = [
        0 as libc::c_int as U16,
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
    let mut valPerRank: [U16; 14] = [
        0 as libc::c_int as U16,
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
    let mut n_1: U32 = 0;
    n_1 = 0 as libc::c_int as U32;
    while n_1 < nbSymbols {
        nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize))
            as usize] = (nbPerRank[HUF_getNbBits(*ct.offset(n_1 as isize)) as usize])
            .wrapping_add(1);
        n_1 = n_1.wrapping_add(1);
    }
    valPerRank[tableLog.wrapping_add(1 as libc::c_int as libc::c_uint)
        as usize] = 0 as libc::c_int as U16;
    let mut min = 0 as libc::c_int as U16;
    let mut n_2: U32 = 0;
    n_2 = tableLog;
    while n_2 > 0 as libc::c_int as libc::c_uint {
        valPerRank[n_2 as usize] = min;
        min = (min as libc::c_int + nbPerRank[n_2 as usize] as libc::c_int) as U16;
        min = (min as libc::c_int >> 1 as libc::c_int) as U16;
        n_2 = n_2.wrapping_sub(1);
    }
    let mut n_3: U32 = 0;
    n_3 = 0 as libc::c_int as U32;
    while n_3 < nbSymbols {
        let fresh0 = valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize)) as usize];
        valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize))
            as usize] = (valPerRank[HUF_getNbBits(*ct.offset(n_3 as isize)) as usize])
            .wrapping_add(1);
        HUF_setValue(ct.offset(n_3 as isize), fresh0 as size_t);
        n_3 = n_3.wrapping_add(1);
    }
    *maxSymbolValuePtr = nbSymbols.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return readSize;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_getNbBitsFromCTable(
    mut CTable: *const HUF_CElt,
    mut symbolValue: U32,
) -> U32 {
    let ct = CTable.offset(1 as libc::c_int as isize);
    if symbolValue <= 255 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"symbolValue <= HUF_SYMBOLVALUE_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            325 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"U32 HUF_getNbBitsFromCTable(const HUF_CElt *, U32)\0"))
                .as_ptr(),
        );
    }
    return HUF_getNbBits(*ct.offset(symbolValue as isize)) as U32;
}
unsafe extern "C" fn HUF_setMaxHeight(
    mut huffNode: *mut nodeElt,
    mut lastNonNull: U32,
    mut targetNbBits: U32,
) -> U32 {
    let largestBits = (*huffNode.offset(lastNonNull as isize)).nbBits as U32;
    if largestBits <= targetNbBits {
        return largestBits;
    }
    let mut totalCost = 0 as libc::c_int;
    let baseCost = ((1 as libc::c_int) << largestBits.wrapping_sub(targetNbBits)) as U32;
    let mut n = lastNonNull as libc::c_int;
    while (*huffNode.offset(n as isize)).nbBits as libc::c_uint > targetNbBits {
        totalCost = (totalCost as libc::c_uint)
            .wrapping_add(
                baseCost
                    .wrapping_sub(
                        ((1 as libc::c_int)
                            << largestBits
                                .wrapping_sub(
                                    (*huffNode.offset(n as isize)).nbBits as libc::c_uint,
                                )) as libc::c_uint,
                    ),
            ) as libc::c_int as libc::c_int;
        (*huffNode.offset(n as isize)).nbBits = targetNbBits as BYTE;
        n -= 1;
    }
    if (*huffNode.offset(n as isize)).nbBits as libc::c_uint <= targetNbBits {} else {
        __assert_fail(
            b"huffNode[n].nbBits <= targetNbBits\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            374 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"U32 HUF_setMaxHeight(nodeElt *, U32, U32)\0"))
                .as_ptr(),
        );
    }
    while (*huffNode.offset(n as isize)).nbBits as libc::c_uint == targetNbBits {
        n -= 1;
    }
    if totalCost as U32 & baseCost.wrapping_sub(1 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"((U32)totalCost & (baseCost - 1)) == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            380 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"U32 HUF_setMaxHeight(nodeElt *, U32, U32)\0"))
                .as_ptr(),
        );
    }
    totalCost >>= largestBits.wrapping_sub(targetNbBits);
    if totalCost > 0 as libc::c_int {} else {
        __assert_fail(
            b"totalCost > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            382 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"U32 HUF_setMaxHeight(nodeElt *, U32, U32)\0"))
                .as_ptr(),
        );
    }
    let noSymbol = 0xf0f0f0f0 as libc::c_uint;
    let mut rankLast: [U32; 14] = [0; 14];
    libc::memset(
        rankLast.as_mut_ptr() as *mut libc::c_void,
        0xf0 as libc::c_int,
        ::core::mem::size_of::<[U32; 14]>() as libc::c_ulong as libc::size_t,
    );
    let mut currentNbBits = targetNbBits;
    let mut pos: libc::c_int = 0;
    pos = n;
    while pos >= 0 as libc::c_int {
        if !((*huffNode.offset(pos as isize)).nbBits as libc::c_uint >= currentNbBits) {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as U32;
            rankLast[targetNbBits.wrapping_sub(currentNbBits) as usize] = pos as U32;
        }
        pos -= 1;
    }
    while totalCost > 0 as libc::c_int {
        let mut nBitsToDecrease = (ZSTD_highbit32(totalCost as U32))
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        while nBitsToDecrease > 1 as libc::c_int as libc::c_uint {
            let highPos = rankLast[nBitsToDecrease as usize];
            let lowPos = rankLast[nBitsToDecrease
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
            if !(highPos == noSymbol) {
                if lowPos == noSymbol {
                    break;
                }
                let highTotal = (*huffNode.offset(highPos as isize)).count;
                let lowTotal = (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*huffNode.offset(lowPos as isize)).count);
                if highTotal <= lowTotal {
                    break;
                }
            }
            nBitsToDecrease = nBitsToDecrease.wrapping_sub(1);
        }
        if rankLast[nBitsToDecrease as usize] != noSymbol
            || nBitsToDecrease == 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"rankLast[nBitsToDecrease] != noSymbol || nBitsToDecrease == 1\0"
                    as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                416 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"U32 HUF_setMaxHeight(nodeElt *, U32, U32)\0"))
                    .as_ptr(),
            );
        }
        while nBitsToDecrease <= HUF_TABLELOG_MAX as libc::c_uint
            && rankLast[nBitsToDecrease as usize] == noSymbol
        {
            nBitsToDecrease = nBitsToDecrease.wrapping_add(1);
        }
        if rankLast[nBitsToDecrease as usize] != noSymbol {} else {
            __assert_fail(
                b"rankLast[nBitsToDecrease] != noSymbol\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                420 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"U32 HUF_setMaxHeight(nodeElt *, U32, U32)\0"))
                    .as_ptr(),
            );
        }
        totalCost
            -= (1 as libc::c_int)
                << nBitsToDecrease.wrapping_sub(1 as libc::c_int as libc::c_uint);
        let ref mut fresh1 = (*huffNode
            .offset(rankLast[nBitsToDecrease as usize] as isize))
            .nbBits;
        *fresh1 = (*fresh1).wrapping_add(1);
        if rankLast[nBitsToDecrease.wrapping_sub(1 as libc::c_int as libc::c_uint)
            as usize] == noSymbol
        {
            rankLast[nBitsToDecrease.wrapping_sub(1 as libc::c_int as libc::c_uint)
                as usize] = rankLast[nBitsToDecrease as usize];
        }
        if rankLast[nBitsToDecrease as usize] == 0 as libc::c_int as libc::c_uint {
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
    while totalCost < 0 as libc::c_int {
        if rankLast[1 as libc::c_int as usize] == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as libc::c_uint == targetNbBits {
                n -= 1;
            }
            let ref mut fresh2 = (*huffNode.offset((n + 1 as libc::c_int) as isize))
                .nbBits;
            *fresh2 = (*fresh2).wrapping_sub(1);
            if n >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"n >= 0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    460 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"U32 HUF_setMaxHeight(nodeElt *, U32, U32)\0"))
                        .as_ptr(),
                );
            }
            rankLast[1 as libc::c_int as usize] = (n + 1 as libc::c_int) as U32;
            totalCost += 1;
        } else {
            let ref mut fresh3 = (*huffNode
                .offset(
                    (rankLast[1 as libc::c_int as usize])
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
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
    .wrapping_add(ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as U32));
unsafe extern "C" fn HUF_getIndex(count: U32) -> U32 {
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
    maxSymbolValue1: U32,
) -> libc::c_int {
    let mut i: U32 = 0;
    i = 1 as libc::c_int as U32;
    while i < maxSymbolValue1 {
        if (*huffNode.offset(i as isize)).count
            > (*huffNode
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
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
    let size = high - low + 1 as libc::c_int;
    huffNode = huffNode.offset(low as isize);
    i = 1 as libc::c_int;
    while i < size {
        let key = *huffNode.offset(i as isize);
        let mut j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && (*huffNode.offset(j as isize)).count < key.count {
            *huffNode
                .offset((j + 1 as libc::c_int) as isize) = *huffNode.offset(j as isize);
            j -= 1;
        }
        *huffNode.offset((j + 1 as libc::c_int) as isize) = key;
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
        &mut *arr.offset((i + 1 as libc::c_int) as isize),
        &mut *arr.offset(high as isize),
    );
    return i + 1 as libc::c_int;
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
            low = idx + 1 as libc::c_int;
        } else {
            HUF_simpleQuickSort(arr, idx + 1 as libc::c_int, high);
            high = idx - 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn HUF_sort(
    mut huffNode: *mut nodeElt,
    mut count: *const libc::c_uint,
    maxSymbolValue: U32,
    mut rankPosition: *mut rankPos,
) {
    let mut n: U32 = 0;
    let maxSymbolValue1 = maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint);
    libc::memset(
        rankPosition as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<rankPos>() as libc::c_ulong)
            .wrapping_mul(192 as libc::c_int as libc::c_ulong) as libc::size_t,
    );
    n = 0 as libc::c_int as U32;
    while n < maxSymbolValue1 {
        let mut lowerRank = HUF_getIndex(*count.offset(n as isize));
        if lowerRank < (192 as libc::c_int - 1 as libc::c_int) as libc::c_uint {} else {
            __assert_fail(
                b"lowerRank < RANK_POSITION_TABLE_SIZE - 1\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                608 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void HUF_sort(nodeElt *, const unsigned int *, const U32, rankPos *)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh4 = (*rankPosition.offset(lowerRank as isize)).base;
        *fresh4 = (*fresh4).wrapping_add(1);
        n = n.wrapping_add(1);
    }
    if (*rankPosition.offset((192 as libc::c_int - 1 as libc::c_int) as isize)).base
        as libc::c_int == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"rankPosition[RANK_POSITION_TABLE_SIZE - 1].base == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            612 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void HUF_sort(nodeElt *, const unsigned int *, const U32, rankPos *)\0"))
                .as_ptr(),
        );
    }
    n = (RANK_POSITION_TABLE_SIZE - 1 as libc::c_int) as U32;
    while n > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh5 = (*rankPosition
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .base;
        *fresh5 = (*fresh5 as libc::c_int
            + (*rankPosition.offset(n as isize)).base as libc::c_int) as U16;
        (*rankPosition.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .curr = (*rankPosition
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .base;
        n = n.wrapping_sub(1);
    }
    n = 0 as libc::c_int as U32;
    while n < maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)).wrapping_add(1 as libc::c_int as libc::c_uint);
        let ref mut fresh6 = (*rankPosition.offset(r as isize)).curr;
        let fresh7 = *fresh6;
        *fresh6 = (*fresh6).wrapping_add(1);
        let pos = fresh7 as U32;
        if pos < maxSymbolValue1 {} else {
            __assert_fail(
                b"pos < maxSymbolValue1\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                    as *const libc::c_char,
                624 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void HUF_sort(nodeElt *, const unsigned int *, const U32, rankPos *)\0",
                ))
                    .as_ptr(),
            );
        }
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as BYTE;
        n = n.wrapping_add(1);
    }
    n = RANK_POSITION_DISTINCT_COUNT_CUTOFF;
    while n < (RANK_POSITION_TABLE_SIZE - 1 as libc::c_int) as libc::c_uint {
        let bucketSize = (*rankPosition.offset(n as isize)).curr as libc::c_int
            - (*rankPosition.offset(n as isize)).base as libc::c_int;
        let bucketStartIdx = (*rankPosition.offset(n as isize)).base as U32;
        if bucketSize > 1 as libc::c_int {
            if bucketStartIdx < maxSymbolValue1 {} else {
                __assert_fail(
                    b"bucketStartIdx < maxSymbolValue1\0" as *const u8
                        as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0"
                        as *const u8 as *const libc::c_char,
                    634 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"void HUF_sort(nodeElt *, const unsigned int *, const U32, rankPos *)\0",
                    ))
                        .as_ptr(),
                );
            }
            HUF_simpleQuickSort(
                huffNode.offset(bucketStartIdx as isize),
                0 as libc::c_int,
                bucketSize - 1 as libc::c_int,
            );
        }
        n = n.wrapping_add(1);
    }
    if HUF_isSorted(huffNode, maxSymbolValue1) != 0 {} else {
        __assert_fail(
            b"HUF_isSorted(huffNode, maxSymbolValue1)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            639 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void HUF_sort(nodeElt *, const unsigned int *, const U32, rankPos *)\0"))
                .as_ptr(),
        );
    };
}
pub const STARTNODE: libc::c_int = HUF_SYMBOLVALUE_MAX + 1 as libc::c_int;
unsafe extern "C" fn HUF_buildTree(
    mut huffNode: *mut nodeElt,
    mut maxSymbolValue: U32,
) -> libc::c_int {
    let huffNode0 = huffNode.offset(-(1 as libc::c_int as isize));
    let mut nonNullRank: libc::c_int = 0;
    let mut lowS: libc::c_int = 0;
    let mut lowN: libc::c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: libc::c_int = 0;
    let mut nodeRoot: libc::c_int = 0;
    nonNullRank = maxSymbolValue as libc::c_int;
    while (*huffNode.offset(nonNullRank as isize)).count
        == 0 as libc::c_int as libc::c_uint
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
    *fresh8 = nodeNb as U16;
    (*huffNode.offset(lowS as isize)).parent = *fresh8;
    nodeNb += 1;
    lowS -= 2 as libc::c_int;
    n = nodeNb;
    while n <= nodeRoot {
        (*huffNode.offset(n as isize)).count = (1 as libc::c_uint) << 30 as libc::c_int;
        n += 1;
    }
    (*huffNode0.offset(0 as libc::c_int as isize))
        .count = (1 as libc::c_uint) << 31 as libc::c_int;
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
        *fresh13 = nodeNb as U16;
        (*huffNode.offset(n1 as isize)).parent = *fresh13;
        nodeNb += 1;
    }
    (*huffNode.offset(nodeRoot as isize)).nbBits = 0 as libc::c_int as BYTE;
    n = nodeRoot - 1 as libc::c_int;
    while n >= STARTNODE {
        (*huffNode.offset(n as isize))
            .nbBits = ((*huffNode.offset((*huffNode.offset(n as isize)).parent as isize))
            .nbBits as libc::c_int + 1 as libc::c_int) as BYTE;
        n -= 1;
    }
    n = 0 as libc::c_int;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize))
            .nbBits = ((*huffNode.offset((*huffNode.offset(n as isize)).parent as isize))
            .nbBits as libc::c_int + 1 as libc::c_int) as BYTE;
        n += 1;
    }
    return nonNullRank;
}
unsafe extern "C" fn HUF_buildCTableFromTree(
    mut CTable: *mut HUF_CElt,
    mut huffNode: *const nodeElt,
    mut nonNullRank: libc::c_int,
    mut maxSymbolValue: U32,
    mut maxNbBits: U32,
) {
    let ct = CTable.offset(1 as libc::c_int as isize);
    let mut n: libc::c_int = 0;
    let mut nbPerRank: [U16; 13] = [
        0 as libc::c_int as U16,
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
    let mut valPerRank: [U16; 13] = [
        0 as libc::c_int as U16,
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
    let alphabetSize = maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    n = 0 as libc::c_int;
    while n <= nonNullRank {
        nbPerRank[(*huffNode.offset(n as isize)).nbBits
            as usize] = (nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize])
            .wrapping_add(1);
        n += 1;
    }
    let mut min = 0 as libc::c_int as U16;
    n = maxNbBits as libc::c_int;
    while n > 0 as libc::c_int {
        valPerRank[n as usize] = min;
        min = (min as libc::c_int + nbPerRank[n as usize] as libc::c_int) as U16;
        min = (min as libc::c_int >> 1 as libc::c_int) as U16;
        n -= 1;
    }
    n = 0 as libc::c_int;
    while n < alphabetSize {
        HUF_setNbBits(
            ct.offset((*huffNode.offset(n as isize)).byte as libc::c_int as isize),
            (*huffNode.offset(n as isize)).nbBits as size_t,
        );
        n += 1;
    }
    n = 0 as libc::c_int;
    while n < alphabetSize {
        let fresh14 = valPerRank[HUF_getNbBits(*ct.offset(n as isize)) as usize];
        valPerRank[HUF_getNbBits(*ct.offset(n as isize))
            as usize] = (valPerRank[HUF_getNbBits(*ct.offset(n as isize)) as usize])
            .wrapping_add(1);
        HUF_setValue(ct.offset(n as isize), fresh14 as size_t);
        n += 1;
    }
    *CTable.offset(0 as libc::c_int as isize) = maxNbBits as HUF_CElt;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable_wksp(
    mut CTable: *mut HUF_CElt,
    mut count: *const libc::c_uint,
    mut maxSymbolValue: U32,
    mut maxNbBits: U32,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let wksp_tables = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<U32>() as libc::c_ulong,
    ) as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.offset(1 as libc::c_int as isize);
    let mut nonNullRank: libc::c_int = 0;
    if wkspSize < ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as libc::c_ulong
    {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t;
    }
    if maxNbBits == 0 as libc::c_int as libc::c_uint {
        maxNbBits = HUF_TABLELOG_DEFAULT as U32;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    libc::memset(
        huffNode0 as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<huffNodeTable>() as libc::c_ulong as libc::size_t,
    );
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as U32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);
    return maxNbBits as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_estimateCompressedSize(
    mut CTable: *const HUF_CElt,
    mut count: *const libc::c_uint,
    mut maxSymbolValue: libc::c_uint,
) -> size_t {
    let mut ct = CTable.offset(1 as libc::c_int as isize);
    let mut nbBits = 0 as libc::c_int as size_t;
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s <= maxSymbolValue as libc::c_int {
        nbBits = (nbBits as libc::c_ulong)
            .wrapping_add(
                (HUF_getNbBits(*ct.offset(s as isize)))
                    .wrapping_mul(*count.offset(s as isize) as libc::c_ulong),
            ) as size_t as size_t;
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
    let mut ct = CTable.offset(1 as libc::c_int as isize);
    let mut bad = 0 as libc::c_int;
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s <= maxSymbolValue as libc::c_int {
        bad
            |= (*count.offset(s as isize) != 0 as libc::c_int as libc::c_uint)
                as libc::c_int
                & (HUF_getNbBits(*ct.offset(s as isize))
                    == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
        s += 1;
    }
    return (bad == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compressBound(mut size: size_t) -> size_t {
    return (HUF_CTABLEBOUND as libc::c_ulong)
        .wrapping_add(
            size
                .wrapping_add(size >> 8 as libc::c_int)
                .wrapping_add(8 as libc::c_int as libc::c_ulong),
        );
}
pub const HUF_BITS_IN_CONTAINER: libc::c_ulong = (::core::mem::size_of::<size_t>()
    as libc::c_ulong)
    .wrapping_mul(8 as libc::c_int as libc::c_ulong);
unsafe extern "C" fn HUF_initCStream(
    mut bitC: *mut HUF_CStream_t,
    mut startPtr: *mut libc::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    libc::memset(
        bitC as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<HUF_CStream_t>() as libc::c_ulong as libc::size_t,
    );
    (*bitC).startPtr = startPtr as *mut BYTE;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC)
        .endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<size_t>() as libc::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<size_t>() as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return 0 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_addBits(
    mut bitC: *mut HUF_CStream_t,
    mut elt: HUF_CElt,
    mut idx: libc::c_int,
    mut kFast: libc::c_int,
) {
    if idx <= 1 as libc::c_int {} else {
        __assert_fail(
            b"idx <= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            846 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void HUF_addBits(HUF_CStream_t *, HUF_CElt, int, int)\0"))
                .as_ptr(),
        );
    }
    if HUF_getNbBits(elt) <= 12 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"HUF_getNbBits(elt) <= HUF_TABLELOG_ABSOLUTEMAX\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            847 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void HUF_addBits(HUF_CStream_t *, HUF_CElt, int, int)\0"))
                .as_ptr(),
        );
    }
    (*bitC).bitContainer[idx as usize] >>= HUF_getNbBits(elt);
    (*bitC).bitContainer[idx as usize]
        |= if kFast != 0 { HUF_getValueFast(elt) } else { HUF_getValue(elt) };
    (*bitC)
        .bitPos[idx
        as usize] = ((*bitC).bitPos[idx as usize] as libc::c_ulong)
        .wrapping_add(HUF_getNbBitsFast(elt)) as size_t as size_t;
    if (*bitC).bitPos[idx as usize] & 0xff as libc::c_int as libc::c_ulong
        <= (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"(bitC->bitPos[idx] & 0xFF) <= HUF_BITS_IN_CONTAINER\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            859 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void HUF_addBits(HUF_CStream_t *, HUF_CElt, int, int)\0"))
                .as_ptr(),
        );
    }
    let nbBits = HUF_getNbBits(elt);
    let dirtyBits = (if nbBits == 0 as libc::c_int as libc::c_ulong {
        0 as libc::c_int as libc::c_uint
    } else {
        (ZSTD_highbit32(nbBits as U32)).wrapping_add(1 as libc::c_int as libc::c_uint)
    }) as size_t;
    if elt >> dirtyBits << dirtyBits.wrapping_add(nbBits)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"((elt >> dirtyBits) << (dirtyBits + nbBits)) == 0\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            870 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void HUF_addBits(HUF_CStream_t *, HUF_CElt, int, int)\0"))
                .as_ptr(),
        );
    }
    if kFast == 0
        || (*bitC).bitPos[idx as usize] & 0xff as libc::c_int as libc::c_ulong
            <= (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"!kFast || (bitC->bitPos[idx] & 0xFF) <= HUF_BITS_IN_CONTAINER\0"
                as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            872 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void HUF_addBits(HUF_CStream_t *, HUF_CElt, int, int)\0"))
                .as_ptr(),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HUF_zeroIndex1(mut bitC: *mut HUF_CStream_t) {
    (*bitC).bitContainer[1 as libc::c_int as usize] = 0 as libc::c_int as size_t;
    (*bitC).bitPos[1 as libc::c_int as usize] = 0 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_mergeIndex1(mut bitC: *mut HUF_CStream_t) {
    if ((*bitC).bitPos[1 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_ulong)
        < (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"(bitC->bitPos[1] & 0xFF) < HUF_BITS_IN_CONTAINER\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            890 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void HUF_mergeIndex1(HUF_CStream_t *)\0"))
                .as_ptr(),
        );
    }
    (*bitC).bitContainer[0 as libc::c_int as usize]
        >>= (*bitC).bitPos[1 as libc::c_int as usize]
            & 0xff as libc::c_int as libc::c_ulong;
    (*bitC).bitContainer[0 as libc::c_int as usize]
        |= (*bitC).bitContainer[1 as libc::c_int as usize];
    (*bitC)
        .bitPos[0 as libc::c_int
        as usize] = ((*bitC).bitPos[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add((*bitC).bitPos[1 as libc::c_int as usize]) as size_t as size_t;
    if (*bitC).bitPos[0 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_ulong
        <= (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"(bitC->bitPos[0] & 0xFF) <= HUF_BITS_IN_CONTAINER\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            894 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void HUF_mergeIndex1(HUF_CStream_t *)\0"))
                .as_ptr(),
        );
    };
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
    (*bitC).bitPos[0 as libc::c_int as usize] &= 7 as libc::c_int as libc::c_ulong;
    if nbBits > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nbBits > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            913 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void HUF_flushBits(HUF_CStream_t *, int)\0"))
                .as_ptr(),
        );
    }
    if nbBits
        <= (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbBits <= sizeof(bitC->bitContainer[0]) * 8\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            914 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void HUF_flushBits(HUF_CStream_t *, int)\0"))
                .as_ptr(),
        );
    }
    if (*bitC).ptr <= (*bitC).endPtr {} else {
        __assert_fail(
            b"bitC->ptr <= bitC->endPtr\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            915 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void HUF_flushBits(HUF_CStream_t *, int)\0"))
                .as_ptr(),
        );
    }
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    if kFast == 0 || (*bitC).ptr <= (*bitC).endPtr {} else {
        __assert_fail(
            b"!kFast || bitC->ptr <= bitC->endPtr\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            918 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void HUF_flushBits(HUF_CStream_t *, int)\0"))
                .as_ptr(),
        );
    }
    if kFast == 0 && (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
}
unsafe extern "C" fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1 as libc::c_int as size_t);
    HUF_setValue(&mut endMark, 1 as libc::c_int as size_t);
    return endMark;
}
unsafe extern "C" fn HUF_closeCStream(mut bitC: *mut HUF_CStream_t) -> size_t {
    HUF_addBits(bitC, HUF_endMark(), 0 as libc::c_int, 0 as libc::c_int);
    HUF_flushBits(bitC, 0 as libc::c_int);
    let nbBits = (*bitC).bitPos[0 as libc::c_int as usize]
        & 0xff as libc::c_int as libc::c_ulong;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0 as libc::c_int as size_t;
    }
    return (((*bitC).ptr).offset_from((*bitC).startPtr) as libc::c_long as size_t)
        .wrapping_add(
            (nbBits > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        );
}
#[inline(always)]
unsafe extern "C" fn HUF_encodeSymbol(
    mut bitCPtr: *mut HUF_CStream_t,
    mut symbol: U32,
    mut CTable: *const HUF_CElt,
    mut idx: libc::c_int,
    mut fast: libc::c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body_loop(
    mut bitC: *mut HUF_CStream_t,
    mut ip: *const BYTE,
    mut srcSize: size_t,
    mut ct: *const HUF_CElt,
    mut kUnroll: libc::c_int,
    mut kFastFlush: libc::c_int,
    mut kLastFast: libc::c_int,
) {
    let mut n = srcSize as libc::c_int;
    let mut rem = n % kUnroll;
    if rem > 0 as libc::c_int {
        while rem > 0 as libc::c_int {
            n -= 1;
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n as isize) as U32,
                ct,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            rem -= 1;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    if n % kUnroll == 0 as libc::c_int {} else {
        __assert_fail(
            b"n % kUnroll == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            972 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"void HUF_compress1X_usingCTable_internal_body_loop(HUF_CStream_t *, const BYTE *, size_t, const HUF_CElt *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if n % (2 as libc::c_int * kUnroll) != 0 {
        let mut u: libc::c_int = 0;
        u = 1 as libc::c_int;
        while u < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u) as isize) as U32,
                ct,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            u += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as U32,
            ct,
            0 as libc::c_int,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        n -= kUnroll;
    }
    if n % (2 as libc::c_int * kUnroll) == 0 as libc::c_int {} else {
        __assert_fail(
            b"n % (2 * kUnroll) == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            984 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"void HUF_compress1X_usingCTable_internal_body_loop(HUF_CStream_t *, const BYTE *, size_t, const HUF_CElt *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    while n > 0 as libc::c_int {
        let mut u_0: libc::c_int = 0;
        u_0 = 1 as libc::c_int;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u_0) as isize) as U32,
                ct,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            u_0 += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as U32,
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
                *ip.offset((n - kUnroll - u_0) as isize) as U32,
                ct,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            u_0 += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll - kUnroll) as isize) as U32,
            ct,
            1 as libc::c_int,
            kLastFast,
        );
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 as libc::c_int * kUnroll;
    }
    if n == 0 as libc::c_int {} else {
        __assert_fail(
            b"n == 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1007 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"void HUF_compress1X_usingCTable_internal_body_loop(HUF_CStream_t *, const BYTE *, size_t, const HUF_CElt *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn HUF_tightCompressBound(
    mut srcSize: size_t,
    mut tableLog: size_t,
) -> size_t {
    return (srcSize.wrapping_mul(tableLog) >> 3 as libc::c_int)
        .wrapping_add(8 as libc::c_int as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    let tableLog = *CTable.offset(0 as libc::c_int as isize) as U32;
    let mut ct = CTable.offset(1 as libc::c_int as isize);
    let mut ip = src as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    let mut bitC = HUF_CStream_t {
        bitContainer: [0; 2],
        bitPos: [0; 2],
        startPtr: 0 as *mut BYTE,
        ptr: 0 as *mut BYTE,
        endPtr: 0 as *mut BYTE,
    };
    if dstSize < 8 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
    );
    if ERR_isError(initErr) != 0 {
        return 0 as libc::c_int as size_t;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as size_t)
        || tableLog > 11 as libc::c_int as libc::c_uint
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
    if bitC.ptr <= bitC.endPtr {} else {
        __assert_fail(
            b"bitC.ptr <= bitC.endPtr\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1082 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t HUF_compress1X_usingCTable_internal_body(void *, size_t, const void *, size_t, const HUF_CElt *)\0",
            ))
                .as_ptr(),
        );
    }
    return HUF_closeCStream(&mut bitC);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_bmi2(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_default(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    flags: libc::c_int,
) -> size_t {
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
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: libc::c_int,
) -> size_t {
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
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: libc::c_int,
) -> size_t {
    let segmentSize = srcSize
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(4 as libc::c_int as libc::c_ulong);
    let mut ip = src as *const BYTE;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if dstSize
        < (6 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 8 as libc::c_int) as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    if srcSize < 12 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    op = op.offset(6 as libc::c_int as isize);
    if op <= oend {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1150 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t HUF_compress4X_usingCTable_internal(void *, size_t, const void *, size_t, const HUF_CElt *, int)\0",
            ))
                .as_ptr(),
        );
    }
    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        ip as *const libc::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 as libc::c_int as libc::c_ulong
        || cSize > 65535 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    MEM_writeLE16(ostart as *mut libc::c_void, cSize as U16);
    op = op.offset(cSize as isize);
    ip = ip.offset(segmentSize as isize);
    if op <= oend {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1158 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t HUF_compress4X_usingCTable_internal(void *, size_t, const void *, size_t, const HUF_CElt *, int)\0",
            ))
                .as_ptr(),
        );
    }
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        ip as *const libc::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_0) != 0 {
        return cSize_0;
    }
    if cSize_0 == 0 as libc::c_int as libc::c_ulong
        || cSize_0 > 65535 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    MEM_writeLE16(
        ostart.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        cSize_0 as U16,
    );
    op = op.offset(cSize_0 as isize);
    ip = ip.offset(segmentSize as isize);
    if op <= oend {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t HUF_compress4X_usingCTable_internal(void *, size_t, const void *, size_t, const HUF_CElt *, int)\0",
            ))
                .as_ptr(),
        );
    }
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        ip as *const libc::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_1) != 0 {
        return cSize_1;
    }
    if cSize_1 == 0 as libc::c_int as libc::c_ulong
        || cSize_1 > 65535 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    MEM_writeLE16(
        ostart.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        cSize_1 as U16,
    );
    op = op.offset(cSize_1 as isize);
    ip = ip.offset(segmentSize as isize);
    if op <= oend {} else {
        __assert_fail(
            b"op <= oend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1174 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t HUF_compress4X_usingCTable_internal(void *, size_t, const void *, size_t, const HUF_CElt *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if ip <= iend {} else {
        __assert_fail(
            b"ip <= iend\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1175 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"size_t HUF_compress4X_usingCTable_internal(void *, size_t, const void *, size_t, const HUF_CElt *, int)\0",
            ))
                .as_ptr(),
        );
    }
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut libc::c_void,
        oend.offset_from(op) as libc::c_long as size_t,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
        CTable,
        flags,
    );
    if ERR_isError(cSize_2) != 0 {
        return cSize_2;
    }
    if cSize_2 == 0 as libc::c_int as libc::c_ulong
        || cSize_2 > 65535 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    op = op.offset(cSize_2 as isize);
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_usingCTable(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: libc::c_int,
) -> size_t {
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
    ostart: *mut BYTE,
    mut op: *mut BYTE,
    oend: *mut BYTE,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut nbStreams: HUF_nbStreams_e,
    mut CTable: *const HUF_CElt,
    flags: libc::c_int,
) -> size_t {
    let cSize = if nbStreams as libc::c_uint
        == HUF_singleStream as libc::c_int as libc::c_uint
    {
        HUF_compress1X_usingCTable_internal(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut libc::c_void,
            oend.offset_from(op) as libc::c_long as size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    op = op.offset(cSize as isize);
    if op >= ostart {} else {
        __assert_fail(
            b"op >= ostart\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1203 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 137],
                &[libc::c_char; 137],
            >(
                b"size_t HUF_compressCTable_internal(BYTE *const, BYTE *, BYTE *const, const void *, size_t, HUF_nbStreams_e, const HUF_CElt *, const int)\0",
            ))
                .as_ptr(),
        );
    }
    if op.offset_from(ostart) as libc::c_long as size_t
        >= srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return 0 as libc::c_int as size_t;
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
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
    while i < maxSymbolValue.wrapping_add(1 as libc::c_int as libc::c_uint) {
        if *count.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            cardinality = cardinality.wrapping_add(1 as libc::c_int as libc::c_uint);
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
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    return minBitsSymbols;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_optimalTableLog(
    mut maxTableLog: libc::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut table: *mut HUF_CElt,
    mut count: *const libc::c_uint,
    mut flags: libc::c_int,
) -> libc::c_uint {
    if srcSize > 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"srcSize > 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1248 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 124],
                &[libc::c_char; 124],
            >(
                b"unsigned int HUF_optimalTableLog(unsigned int, size_t, unsigned int, void *, size_t, HUF_CElt *, const unsigned int *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if wkspSize >= ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"wkspSize >= sizeof(HUF_buildCTable_wksp_tables)\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1249 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 124],
                &[libc::c_char; 124],
            >(
                b"unsigned int HUF_optimalTableLog(unsigned int, size_t, unsigned int, void *, size_t, HUF_CElt *, const unsigned int *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if flags & HUF_flags_optimalDepth as libc::c_int == 0 {
        return FSE_optimalTableLog_internal(
            maxTableLog,
            srcSize,
            maxSymbolValue,
            1 as libc::c_int as libc::c_uint,
        );
    }
    let mut dst = (workSpace as *mut BYTE)
        .offset(::core::mem::size_of::<HUF_WriteCTableWksp>() as libc::c_ulong as isize);
    let mut dstSize = wkspSize
        .wrapping_sub(::core::mem::size_of::<HUF_WriteCTableWksp>() as libc::c_ulong);
    let mut maxBits: size_t = 0;
    let mut hSize: size_t = 0;
    let mut newSize: size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0 as libc::c_int) as size_t)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
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
                maxBits as U32,
                workSpace,
                wkspSize,
            );
            if !(ERR_isError(hSize) != 0) {
                newSize = (HUF_estimateCompressedSize(table, count, maxSymbolValue))
                    .wrapping_add(hSize);
                if newSize > optSize.wrapping_add(1 as libc::c_int as libc::c_ulong) {
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
    if optLog <= 12 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"optLog <= HUF_TABLELOG_MAX\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/lib/compress/huf_compress.c\0" as *const u8
                as *const libc::c_char,
            1289 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 124],
                &[libc::c_char; 124],
            >(
                b"unsigned int HUF_optimalTableLog(unsigned int, size_t, unsigned int, void *, size_t, HUF_CElt *, const unsigned int *, int)\0",
            ))
                .as_ptr(),
        );
    }
    return optLog;
}
unsafe extern "C" fn HUF_compress_internal(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut nbStreams: HUF_nbStreams_e,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut oldHufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: libc::c_int,
) -> size_t {
    let table = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<size_t>() as libc::c_ulong,
    ) as *mut HUF_compress_tables_t;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if wkspSize < ::core::mem::size_of::<HUF_compress_tables_t>() as libc::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t;
    }
    if srcSize == 0 {
        return 0 as libc::c_int as size_t;
    }
    if dstSize == 0 {
        return 0 as libc::c_int as size_t;
    }
    if srcSize > HUF_BLOCKSIZE_MAX as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if huffLog > HUF_TABLELOG_MAX as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
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
        let mut largestTotal = 0 as libc::c_int as size_t;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const BYTE as *const libc::c_void,
            4096 as libc::c_int as size_t,
        ) as size_t;
        if ERR_isError(largestBegin) != 0 {
            return largestBegin;
        }
        largestTotal = (largestTotal as libc::c_ulong).wrapping_add(largestBegin)
            as size_t as size_t;
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const BYTE)
                .offset(srcSize as isize)
                .offset(-(4096 as libc::c_int as isize)) as *const libc::c_void,
            4096 as libc::c_int as size_t,
        ) as size_t;
        if ERR_isError(largestEnd) != 0 {
            return largestEnd;
        }
        largestTotal = (largestTotal as libc::c_ulong).wrapping_add(largestEnd) as size_t
            as size_t;
        if largestTotal
            <= ((2 as libc::c_int * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE
                >> 7 as libc::c_int) + 4 as libc::c_int) as libc::c_ulong
        {
            return 0 as libc::c_int as size_t;
        }
    }
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const BYTE as *const libc::c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[U32; 1024]>() as libc::c_ulong,
    );
    if ERR_isError(largest) != 0 {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const BYTE).offset(0 as libc::c_int as isize);
        return 1 as libc::c_int as size_t;
    }
    if largest
        <= (srcSize >> 7 as libc::c_int).wrapping_add(4 as libc::c_int as libc::c_ulong)
    {
        return 0 as libc::c_int as size_t;
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
        ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
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
        ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as libc::c_ulong,
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    huffLog = maxBits as U32;
    let ctableSize = maxSymbolValue.wrapping_add(2 as libc::c_int as libc::c_uint)
        as size_t;
    let unusedSize = (::core::mem::size_of::<[HUF_CElt; 257]>() as libc::c_ulong)
        .wrapping_sub(
            ctableSize.wrapping_mul(::core::mem::size_of::<HUF_CElt>() as libc::c_ulong),
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
        ::core::mem::size_of::<HUF_WriteCTableWksp>() as libc::c_ulong,
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
            || hSize.wrapping_add(12 as libc::c_int as libc::c_ulong) >= srcSize
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
    if hSize.wrapping_add(12 as libc::c_ulong) >= srcSize {
        return 0 as libc::c_int as size_t;
    }
    op = op.offset(hSize as isize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut libc::c_void,
            ((*table).CTable).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[HUF_CElt; 257]>() as libc::c_ulong as libc::size_t,
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
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: libc::c_int,
) -> size_t {
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
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut huffLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: libc::c_int,
) -> size_t {
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
