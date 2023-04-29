use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type dctx_t = ZSTDv01_Dctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv01_Dctx_s {
    pub LLTable: [U32; 1025],
    pub OffTable: [U32; 513],
    pub MLTable: [U32; 1025],
    pub previousDstEnd: *mut libc::c_void,
    pub base: *mut libc::c_void,
    pub expected: size_t,
    pub bType: blockType_t,
    pub phase: U32,
}
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type blockType_t = libc::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_t,
    pub origSize: U32,
}
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed_2 = 72;
pub const ZSTD_error_maxCode: C2RustUnnamed_2 = 120;
pub const ZSTD_error_GENERIC: C2RustUnnamed_2 = 1;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed_2 = 70;
pub const ZSTD_error_corruption_detected: C2RustUnnamed_2 = 20;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: FSE_DStream_t,
    pub stateLL: FSE_DState_t,
    pub stateOffb: FSE_DState_t,
    pub stateML: FSE_DState_t,
    pub prevOffset: size_t,
    pub dumps: *const BYTE,
    pub dumpsEnd: *const BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub offset: size_t,
    pub matchLength: size_t,
}
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: U32,
    pub c: [BYTE; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const FSE_DStream_unfinished: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i: U32,
    pub c: [BYTE; 4],
}
pub const FSE_DStream_endOfBuffer: C2RustUnnamed_4 = 1;
pub const FSE_DStream_completed: C2RustUnnamed_4 = 2;
pub const FSE_DStream_tooFar: C2RustUnnamed_4 = 3;
pub type FSE_DTable = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const FSE_ERROR_maxCode: C2RustUnnamed_3 = 8;
pub const FSE_ERROR_GENERIC: C2RustUnnamed_3 = 1;
pub const FSE_ERROR_srcSize_wrong: C2RustUnnamed_3 = 6;
pub type S16 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub const FSE_ERROR_tableLog_tooLarge: C2RustUnnamed_3 = 2;
pub const FSE_ERROR_maxSymbolValue_tooLarge: C2RustUnnamed_3 = 3;
pub const FSE_ERROR_maxSymbolValue_tooSmall: C2RustUnnamed_3 = 4;
pub const FSE_ERROR_corruptionDetected: C2RustUnnamed_3 = 7;
pub const FSE_ERROR_dstSize_tooSmall: C2RustUnnamed_3 = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DElt {
    pub byte: BYTE,
    pub nbBits: BYTE,
}
pub type DTable_max_t = [U32; 4097];
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FSE_static_assert: C2RustUnnamed_1 = 1;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed_2 = 10;
pub type ZSTDv01_Dctx = ZSTDv01_Dctx_s;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed_2 = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed_2 = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed_2 = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed_2 = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed_2 = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed_2 = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed_2 = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed_2 = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed_2 = 74;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed_2 = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed_2 = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed_2 = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed_2 = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed_2 = 50;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed_2 = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed_2 = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed_2 = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed_2 = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed_2 = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed_2 = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed_2 = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed_2 = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed_2 = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed_2 = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed_2 = 22;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed_2 = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed_2 = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed_2 = 12;
pub const ZSTD_error_no_error: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const FSE_OK_NoError: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
pub const FSE_MAX_MEMORY_USAGE: libc::c_int = 14 as libc::c_int;
pub const FSE_MAX_SYMBOL_VALUE: libc::c_int = 255 as libc::c_int;
unsafe extern "C" fn FSE_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn FSE_isLittleEndian() -> libc::c_uint {
    let one = C2RustUnnamed_0 {
        i: 1 as libc::c_int as U32,
    };
    return one.c[0 as libc::c_int as usize] as libc::c_uint;
}
unsafe extern "C" fn FSE_read16(mut memPtr: *const libc::c_void) -> U16 {
    let mut val: U16 = 0;
    memcpy(
        &mut val as *mut U16 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<U16>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn FSE_read32(mut memPtr: *const libc::c_void) -> U32 {
    let mut val: U32 = 0;
    memcpy(
        &mut val as *mut U32 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<U32>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn FSE_read64(mut memPtr: *const libc::c_void) -> U64 {
    let mut val: U64 = 0;
    memcpy(
        &mut val as *mut U64 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<U64>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn FSE_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if FSE_isLittleEndian() != 0 {
        return FSE_read16(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as libc::c_int
            + ((*p.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as U16;
    };
}
unsafe extern "C" fn FSE_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if FSE_isLittleEndian() != 0 {
        return FSE_read32(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as U32)
            .wrapping_add(
                (*p.offset(1 as libc::c_int as isize) as U32) << 8 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(2 as libc::c_int as isize) as U32) << 16 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(3 as libc::c_int as isize) as U32) << 24 as libc::c_int,
            );
    };
}
unsafe extern "C" fn FSE_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if FSE_isLittleEndian() != 0 {
        return FSE_read64(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as U64)
            .wrapping_add(
                (*p.offset(1 as libc::c_int as isize) as U64) << 8 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(2 as libc::c_int as isize) as U64) << 16 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(3 as libc::c_int as isize) as U64) << 24 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(4 as libc::c_int as isize) as U64) << 32 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(5 as libc::c_int as isize) as U64) << 40 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(6 as libc::c_int as isize) as U64) << 48 as libc::c_int,
            )
            .wrapping_add(
                (*p.offset(7 as libc::c_int as isize) as U64) << 56 as libc::c_int,
            );
    };
}
unsafe extern "C" fn FSE_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if FSE_32bits() != 0 {
        return FSE_readLE32(memPtr) as size_t
    } else {
        return FSE_readLE64(memPtr)
    };
}
pub const FSE_MAX_TABLELOG: libc::c_int = FSE_MAX_MEMORY_USAGE - 2 as libc::c_int;
pub const FSE_MIN_TABLELOG: libc::c_int = 5 as libc::c_int;
pub const FSE_TABLELOG_ABSOLUTE_MAX: libc::c_int = 15 as libc::c_int;
#[inline(always)]
unsafe extern "C" fn FSE_highbit32(mut val: U32) -> libc::c_uint {
    return (val.leading_zeros() as i32 ^ 31 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn FSE_tableStep(mut tableSize: U32) -> U32 {
    return (tableSize >> 1 as libc::c_int)
        .wrapping_add(tableSize >> 3 as libc::c_int)
        .wrapping_add(3 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn FSE_buildDTable(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSE_DTableHeader;
    let tableDecode = (ptr as *mut FSE_decode_t).offset(1 as libc::c_int as isize);
    let tableSize = ((1 as libc::c_int) << tableLog) as U32;
    let tableMask = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let step = FSE_tableStep(tableSize);
    let mut symbolNext: [U16; 256] = [0; 256];
    let mut position = 0 as libc::c_int as U32;
    let mut highThreshold = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let largeLimit = ((1 as libc::c_int)
        << tableLog.wrapping_sub(1 as libc::c_int as libc::c_uint)) as S16;
    let mut noLarge = 1 as libc::c_int as U32;
    let mut s: U32 = 0;
    if maxSymbolValue > FSE_MAX_SYMBOL_VALUE as libc::c_uint {
        return -(FSE_ERROR_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    if tableLog > FSE_MAX_TABLELOG as libc::c_uint {
        return -(FSE_ERROR_tableLog_tooLarge as libc::c_int) as size_t;
    }
    (*DTableH.offset(0 as libc::c_int as isize)).tableLog = tableLog as U16;
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -(1 as libc::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as BYTE;
            symbolNext[s as usize] = 1 as libc::c_int as U16;
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int
                >= largeLimit as libc::c_int
            {
                noLarge = 0 as libc::c_int as U32;
            }
            symbolNext[s as usize] = *normalizedCounter.offset(s as isize) as U16;
        }
        s = s.wrapping_add(1);
    }
    s = 0 as libc::c_int as U32;
    while s <= maxSymbolValue {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < *normalizedCounter.offset(s as isize) as libc::c_int {
            (*tableDecode.offset(position as isize)).symbol = s as BYTE;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    if position != 0 as libc::c_int as libc::c_uint {
        return -(FSE_ERROR_GENERIC as libc::c_int) as size_t;
    }
    let mut i_0: U32 = 0;
    i_0 = 0 as libc::c_int as U32;
    while i_0 < tableSize {
        let mut symbol = (*tableDecode.offset(i_0 as isize)).symbol;
        let fresh1 = symbolNext[symbol as usize];
        symbolNext[symbol as usize] = (symbolNext[symbol as usize]).wrapping_add(1);
        let mut nextState = fresh1;
        (*tableDecode.offset(i_0 as isize))
            .nbBits = tableLog.wrapping_sub(FSE_highbit32(nextState as U32)) as BYTE;
        (*tableDecode.offset(i_0 as isize))
            .newState = (((nextState as libc::c_int)
            << (*tableDecode.offset(i_0 as isize)).nbBits as libc::c_int)
            as libc::c_uint)
            .wrapping_sub(tableSize) as U16;
        i_0 = i_0.wrapping_add(1);
    }
    (*DTableH).fastMode = noLarge as U16;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FSE_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(FSE_ERROR_maxCode as libc::c_int) as size_t) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn FSE_abs(mut a: libc::c_short) -> libc::c_short {
    return (if (a as libc::c_int) < 0 as libc::c_int {
        -(a as libc::c_int)
    } else {
        a as libc::c_int
    }) as libc::c_short;
}
unsafe extern "C" fn FSE_readNCount(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const BYTE;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: libc::c_int = 0;
    let mut charnum = 0 as libc::c_int as libc::c_uint;
    let mut previous0 = 0 as libc::c_int;
    if hbSize < 4 as libc::c_int as libc::c_ulong {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    bitStream = FSE_readLE32(ip as *const libc::c_void);
    nbBits = (bitStream & 0xf as libc::c_int as libc::c_uint)
        .wrapping_add(FSE_MIN_TABLELOG as libc::c_uint) as libc::c_int;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX {
        return -(FSE_ERROR_tableLog_tooLarge as libc::c_int) as size_t;
    }
    bitStream >>= 4 as libc::c_int;
    bitCount = 4 as libc::c_int;
    *tableLogPtr = nbBits as libc::c_uint;
    remaining = ((1 as libc::c_int) << nbBits) + 1 as libc::c_int;
    threshold = (1 as libc::c_int) << nbBits;
    nbBits += 1;
    while remaining > 1 as libc::c_int && charnum <= *maxSVPtr {
        if previous0 != 0 {
            let mut n0 = charnum;
            while bitStream & 0xffff as libc::c_int as libc::c_uint
                == 0xffff as libc::c_int as libc::c_uint
            {
                n0 = n0.wrapping_add(24 as libc::c_int as libc::c_uint);
                if ip < iend.offset(-(5 as libc::c_int as isize)) {
                    ip = ip.offset(2 as libc::c_int as isize);
                    bitStream = FSE_readLE32(ip as *const libc::c_void) >> bitCount;
                } else {
                    bitStream >>= 16 as libc::c_int;
                    bitCount += 16 as libc::c_int;
                }
            }
            while bitStream & 3 as libc::c_int as libc::c_uint
                == 3 as libc::c_int as libc::c_uint
            {
                n0 = n0.wrapping_add(3 as libc::c_int as libc::c_uint);
                bitStream >>= 2 as libc::c_int;
                bitCount += 2 as libc::c_int;
            }
            n0 = n0.wrapping_add(bitStream & 3 as libc::c_int as libc::c_uint);
            bitCount += 2 as libc::c_int;
            if n0 > *maxSVPtr {
                return -(FSE_ERROR_maxSymbolValue_tooSmall as libc::c_int) as size_t;
            }
            while charnum < n0 {
                let fresh2 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter
                    .offset(fresh2 as isize) = 0 as libc::c_int as libc::c_short;
            }
            if ip <= iend.offset(-(7 as libc::c_int as isize))
                || ip.offset((bitCount >> 3 as libc::c_int) as isize)
                    <= iend.offset(-(4 as libc::c_int as isize))
            {
                ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
                bitCount &= 7 as libc::c_int;
                bitStream = FSE_readLE32(ip as *const libc::c_void) >> bitCount;
            } else {
                bitStream >>= 2 as libc::c_int;
            }
        }
        let max = (2 as libc::c_int * threshold - 1 as libc::c_int - remaining)
            as libc::c_short;
        let mut count: libc::c_short = 0;
        if (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint) < max as U32 {
            count = (bitStream & (threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_short;
            bitCount += nbBits - 1 as libc::c_int;
        } else {
            count = (bitStream
                & (2 as libc::c_int * threshold - 1 as libc::c_int) as libc::c_uint)
                as libc::c_short;
            if count as libc::c_int >= threshold {
                count = (count as libc::c_int - max as libc::c_int) as libc::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        remaining -= FSE_abs(count) as libc::c_int;
        let fresh3 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh3 as isize) = count;
        previous0 = (count == 0) as libc::c_int;
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1 as libc::c_int;
        }
        if ip <= iend.offset(-(7 as libc::c_int as isize))
            || ip.offset((bitCount >> 3 as libc::c_int) as isize)
                <= iend.offset(-(4 as libc::c_int as isize))
        {
            ip = ip.offset((bitCount >> 3 as libc::c_int) as isize);
            bitCount &= 7 as libc::c_int;
        } else {
            bitCount
                -= (8 as libc::c_int as libc::c_long
                    * iend.offset(-(4 as libc::c_int as isize)).offset_from(ip)
                        as libc::c_long) as libc::c_int;
            ip = iend.offset(-(4 as libc::c_int as isize));
        }
        bitStream = FSE_readLE32(ip as *const libc::c_void)
            >> (bitCount & 31 as libc::c_int);
    }
    if remaining != 1 as libc::c_int {
        return -(FSE_ERROR_GENERIC as libc::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1 as libc::c_int as libc::c_uint);
    ip = ip.offset((bitCount + 7 as libc::c_int >> 3 as libc::c_int) as isize);
    if ip.offset_from(istart) as libc::c_long as size_t > hbSize {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    return ip.offset_from(istart) as libc::c_long as size_t;
}
unsafe extern "C" fn FSE_buildDTable_rle(
    mut dt: *mut FSE_DTable,
    mut symbolValue: BYTE,
) -> size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSE_DTableHeader;
    let cell = (ptr as *mut FSE_decode_t).offset(1 as libc::c_int as isize);
    (*DTableH).tableLog = 0 as libc::c_int as U16;
    (*DTableH).fastMode = 0 as libc::c_int as U16;
    (*cell).newState = 0 as libc::c_int as libc::c_ushort;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0 as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FSE_buildDTable_raw(
    mut dt: *mut FSE_DTable,
    mut nbBits: libc::c_uint,
) -> size_t {
    let mut ptr = dt as *mut libc::c_void;
    let DTableH = ptr as *mut FSE_DTableHeader;
    let dinfo = (ptr as *mut FSE_decode_t).offset(1 as libc::c_int as isize);
    let tableSize = ((1 as libc::c_int) << nbBits) as libc::c_uint;
    let tableMask = tableSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let maxSymbolValue = tableMask;
    let mut s: libc::c_uint = 0;
    if nbBits < 1 as libc::c_int as libc::c_uint {
        return -(FSE_ERROR_GENERIC as libc::c_int) as size_t;
    }
    (*DTableH).tableLog = nbBits as U16;
    (*DTableH).fastMode = 1 as libc::c_int as U16;
    s = 0 as libc::c_int as libc::c_uint;
    while s <= maxSymbolValue {
        (*dinfo.offset(s as isize)).newState = 0 as libc::c_int as libc::c_ushort;
        (*dinfo.offset(s as isize)).symbol = s as BYTE;
        (*dinfo.offset(s as isize)).nbBits = nbBits as BYTE;
        s = s.wrapping_add(1);
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn FSE_initDStream(
    mut bitD: *mut FSE_DStream_t,
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 as libc::c_int as libc::c_ulong {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    if srcSize >= ::core::mem::size_of::<size_t>() as libc::c_ulong {
        let mut contain32: U32 = 0;
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD)
            .ptr = (srcBuffer as *const libc::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<size_t>() as libc::c_ulong as isize));
        (*bitD).bitContainer = FSE_readLEST((*bitD).ptr as *const libc::c_void);
        contain32 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32;
        if contain32 == 0 as libc::c_int as libc::c_uint {
            return -(FSE_ERROR_GENERIC as libc::c_int) as size_t;
        }
        (*bitD)
            .bitsConsumed = (8 as libc::c_int as libc::c_uint)
            .wrapping_sub(FSE_highbit32(contain32));
    } else {
        let mut contain32_0: U32 = 0;
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const BYTE) as size_t;
        let mut current_block_19: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(6 as libc::c_int as isize) as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(16 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_19 = 6222053608555494778;
            }
            6 => {
                current_block_19 = 6222053608555494778;
            }
            5 => {
                current_block_19 = 7153118028659730796;
            }
            4 => {
                current_block_19 = 7749605049626543852;
            }
            3 => {
                current_block_19 = 10459093242147315661;
            }
            2 => {
                current_block_19 = 5906948055639026220;
            }
            _ => {
                current_block_19 = 6009453772311597924;
            }
        }
        match current_block_19 {
            6222053608555494778 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(5 as libc::c_int as isize) as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(24 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_19 = 7153118028659730796;
            }
            _ => {}
        }
        match current_block_19 {
            7153118028659730796 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(4 as libc::c_int as isize) as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(32 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                current_block_19 = 7749605049626543852;
            }
            _ => {}
        }
        match current_block_19 {
            7749605049626543852 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(3 as libc::c_int as isize) as size_t)
                            << 24 as libc::c_int,
                    ) as size_t as size_t;
                current_block_19 = 10459093242147315661;
            }
            _ => {}
        }
        match current_block_19 {
            10459093242147315661 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(2 as libc::c_int as isize) as size_t)
                            << 16 as libc::c_int,
                    ) as size_t as size_t;
                current_block_19 = 5906948055639026220;
            }
            _ => {}
        }
        match current_block_19 {
            5906948055639026220 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add(
                        (*((*bitD).start as *const BYTE)
                            .offset(1 as libc::c_int as isize) as size_t)
                            << 8 as libc::c_int,
                    ) as size_t as size_t;
            }
            _ => {}
        }
        contain32_0 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as U32;
        if contain32_0 == 0 as libc::c_int as libc::c_uint {
            return -(FSE_ERROR_GENERIC as libc::c_int) as size_t;
        }
        (*bitD)
            .bitsConsumed = (8 as libc::c_int as libc::c_uint)
            .wrapping_sub(FSE_highbit32(contain32_0));
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                ((::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(srcSize) as U32)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint),
            );
    }
    return srcSize;
}
unsafe extern "C" fn FSE_lookBits(
    mut bitD: *mut FSE_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask) >> 1 as libc::c_int
        >> (bitMask.wrapping_sub(nbBits) & bitMask);
}
unsafe extern "C" fn FSE_lookBitsFast(
    mut bitD: *mut FSE_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1 as libc::c_int as libc::c_uint).wrapping_sub(nbBits)
            & bitMask);
}
unsafe extern "C" fn FSE_skipBits(mut bitD: *mut FSE_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
unsafe extern "C" fn FSE_readBits(
    mut bitD: *mut FSE_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let mut value = FSE_lookBits(bitD, nbBits);
    FSE_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn FSE_readBitsFast(
    mut bitD: *mut FSE_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let mut value = FSE_lookBitsFast(bitD, nbBits);
    FSE_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn FSE_reloadDStream(mut bitD: *mut FSE_DStream_t) -> libc::c_uint {
    if (*bitD).bitsConsumed as libc::c_ulong
        > (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        return FSE_DStream_tooFar as libc::c_int as libc::c_uint;
    }
    if (*bitD).ptr
        >= ((*bitD).start)
            .offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize)
    {
        (*bitD)
            .ptr = ((*bitD).ptr)
            .offset(-(((*bitD).bitsConsumed >> 3 as libc::c_int) as isize));
        (*bitD).bitsConsumed &= 7 as libc::c_int as libc::c_uint;
        (*bitD).bitContainer = FSE_readLEST((*bitD).ptr as *const libc::c_void);
        return FSE_DStream_unfinished as libc::c_int as libc::c_uint;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong)
            < (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            return FSE_DStream_endOfBuffer as libc::c_int as libc::c_uint;
        }
        return FSE_DStream_completed as libc::c_int as libc::c_uint;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as libc::c_int;
    let mut result = FSE_DStream_unfinished as libc::c_int as U32;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as libc::c_long as U32;
        result = FSE_DStream_endOfBuffer as libc::c_int as U32;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD)
        .bitsConsumed = ((*bitD).bitsConsumed)
        .wrapping_sub(nbBytes.wrapping_mul(8 as libc::c_int as libc::c_uint));
    (*bitD).bitContainer = FSE_readLEST((*bitD).ptr as *const libc::c_void);
    return result;
}
unsafe extern "C" fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut FSE_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut ptr = dt as *const libc::c_void;
    let DTableH = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state = FSE_readBits(bitD, (*DTableH).tableLog as U32);
    FSE_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as libc::c_int as isize) as *const libc::c_void;
}
unsafe extern "C" fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut FSE_DStream_t,
) -> BYTE {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let mut symbol = DInfo.symbol;
    let mut lowBits = FSE_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_decodeSymbolFast(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut FSE_DStream_t,
) -> BYTE {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let mut symbol = DInfo.symbol;
    let mut lowBits = FSE_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_endOfDStream(mut bitD: *const FSE_DStream_t) -> libc::c_uint {
    return ((*bitD).ptr == (*bitD).start
        && (*bitD).bitsConsumed as libc::c_ulong
            == (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)) as libc::c_int
        as libc::c_uint;
}
unsafe extern "C" fn FSE_endOfDState(
    mut DStatePtr: *const FSE_DState_t,
) -> libc::c_uint {
    return ((*DStatePtr).state == 0 as libc::c_int as libc::c_ulong) as libc::c_int
        as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn FSE_decompress_usingDTable_generic(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSE_DTable,
    fast: libc::c_uint,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3 as libc::c_int as isize));
    let mut bitD = FSE_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut state1 = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let mut state2 = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    let mut errorCode: size_t = 0;
    errorCode = FSE_initDStream(&mut bitD, cSrc, cSrcSize);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);
    while FSE_reloadDStream(&mut bitD)
        == FSE_DStream_unfinished as libc::c_int as libc::c_uint && op < olimit
    {
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            FSE_reloadDStream(&mut bitD);
        }
        *op
            .offset(
                1 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 4 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            if FSE_reloadDStream(&mut bitD)
                > FSE_DStream_unfinished as libc::c_int as libc::c_uint
            {
                op = op.offset(2 as libc::c_int as isize);
                break;
            }
        }
        *op
            .offset(
                2 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 2 as libc::c_int + 7 as libc::c_int) as libc::c_ulong
            > (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            FSE_reloadDStream(&mut bitD);
        }
        *op
            .offset(
                3 as libc::c_int as isize,
            ) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as BYTE;
        op = op.offset(4 as libc::c_int as isize);
    }
    while !(FSE_reloadDStream(&mut bitD)
        > FSE_DStream_completed as libc::c_int as libc::c_uint || op == omax
        || FSE_endOfDStream(&mut bitD) != 0
            && (fast != 0 || FSE_endOfDState(&mut state1) != 0))
    {
        let fresh4 = op;
        op = op.offset(1);
        *fresh4 = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
        }) as BYTE;
        if FSE_reloadDStream(&mut bitD)
            > FSE_DStream_completed as libc::c_int as libc::c_uint || op == omax
            || FSE_endOfDStream(&mut bitD) != 0
                && (fast != 0 || FSE_endOfDState(&mut state2) != 0)
        {
            break;
        }
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
        }) as BYTE;
    }
    if FSE_endOfDStream(&mut bitD) != 0 && FSE_endOfDState(&mut state1) != 0
        && FSE_endOfDState(&mut state2) != 0
    {
        return op.offset_from(ostart) as libc::c_long as size_t;
    }
    if op == omax {
        return -(FSE_ERROR_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
}
unsafe extern "C" fn FSE_decompress_usingDTable(
    mut dst: *mut libc::c_void,
    mut originalSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSE_DTable,
) -> size_t {
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    memcpy(
        &mut DTableH as *mut FSE_DTableHeader as *mut libc::c_void,
        dt as *const libc::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong,
    );
    if DTableH.fastMode != 0 {
        return FSE_decompress_usingDTable_generic(
            dst,
            originalSize,
            cSrc,
            cSrcSize,
            dt,
            1 as libc::c_int as libc::c_uint,
        );
    }
    return FSE_decompress_usingDTable_generic(
        dst,
        originalSize,
        cSrc,
        cSrcSize,
        dt,
        0 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn FSE_decompress(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart = cSrc as *const BYTE;
    let mut ip = istart;
    let mut counting: [libc::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: libc::c_uint = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as libc::c_uint;
    let mut errorCode: size_t = 0;
    if cSrcSize < 2 as libc::c_int as libc::c_ulong {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    errorCode = FSE_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const libc::c_void,
        cSrcSize,
    );
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(errorCode) as size_t as size_t;
    errorCode = FSE_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    return FSE_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    );
}
pub const HUF_MAX_SYMBOL_VALUE: libc::c_int = 255 as libc::c_int;
pub const HUF_MAX_TABLELOG: libc::c_int = 12 as libc::c_int;
pub const HUF_ABSOLUTEMAX_TABLELOG: libc::c_int = 16 as libc::c_int;
unsafe extern "C" fn HUF_readDTable(
    mut DTable: *mut U16,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut huffWeight: [BYTE; 256] = [0; 256];
    let mut rankVal: [U32; 17] = [0; 17];
    let mut weightTotal: U32 = 0;
    let mut maxBits: U32 = 0;
    let mut ip = src as *const BYTE;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    let mut n: U32 = 0;
    let mut nextRankStart: U32 = 0;
    let mut ptr = DTable.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let dt = ptr as *mut HUF_DElt;
    if srcSize == 0 {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    iSize = *ip.offset(0 as libc::c_int as isize) as size_t;
    if iSize >= 128 as libc::c_int as libc::c_ulong {
        if iSize >= 242 as libc::c_int as libc::c_ulong {
            static mut l: [libc::c_int; 14] = [
                1 as libc::c_int,
                2 as libc::c_int,
                3 as libc::c_int,
                4 as libc::c_int,
                7 as libc::c_int,
                8 as libc::c_int,
                15 as libc::c_int,
                16 as libc::c_int,
                31 as libc::c_int,
                32 as libc::c_int,
                63 as libc::c_int,
                64 as libc::c_int,
                127 as libc::c_int,
                128 as libc::c_int,
            ];
            oSize = l[iSize.wrapping_sub(242 as libc::c_int as libc::c_ulong) as usize]
                as size_t;
            memset(
                huffWeight.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int,
                ::core::mem::size_of::<[BYTE; 256]>() as libc::c_ulong,
            );
            iSize = 0 as libc::c_int as size_t;
        } else {
            oSize = iSize.wrapping_sub(127 as libc::c_int as libc::c_ulong);
            iSize = oSize
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
            if iSize.wrapping_add(1 as libc::c_int as libc::c_ulong) > srcSize {
                return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
            }
            ip = ip.offset(1 as libc::c_int as isize);
            n = 0 as libc::c_int as U32;
            while (n as libc::c_ulong) < oSize {
                huffWeight[n
                    as usize] = (*ip
                    .offset(n.wrapping_div(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int >> 4 as libc::c_int) as BYTE;
                huffWeight[n.wrapping_add(1 as libc::c_int as libc::c_uint)
                    as usize] = (*ip
                    .offset(n.wrapping_div(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int & 15 as libc::c_int) as BYTE;
                n = (n as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as U32 as U32;
            }
        }
    } else {
        if iSize.wrapping_add(1 as libc::c_int as libc::c_ulong) > srcSize {
            return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
        }
        oSize = FSE_decompress(
            huffWeight.as_mut_ptr() as *mut libc::c_void,
            HUF_MAX_SYMBOL_VALUE as size_t,
            ip.offset(1 as libc::c_int as isize) as *const libc::c_void,
            iSize,
        );
        if FSE_isError(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankVal.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[U32; 17]>() as libc::c_ulong,
    );
    weightTotal = 0 as libc::c_int as U32;
    n = 0 as libc::c_int as U32;
    while (n as libc::c_ulong) < oSize {
        if huffWeight[n as usize] as libc::c_int >= HUF_ABSOLUTEMAX_TABLELOG {
            return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
        }
        rankVal[huffWeight[n as usize]
            as usize] = (rankVal[huffWeight[n as usize] as usize]).wrapping_add(1);
        weightTotal = (weightTotal as libc::c_uint)
            .wrapping_add(
                ((1 as libc::c_int) << huffWeight[n as usize] as libc::c_int
                    >> 1 as libc::c_int) as libc::c_uint,
            ) as U32 as U32;
        n = n.wrapping_add(1);
    }
    if weightTotal == 0 as libc::c_int as libc::c_uint {
        return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
    }
    maxBits = (FSE_highbit32(weightTotal))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if maxBits > *DTable.offset(0 as libc::c_int as isize) as libc::c_uint {
        return -(FSE_ERROR_tableLog_tooLarge as libc::c_int) as size_t;
    }
    *DTable.offset(0 as libc::c_int as isize) = maxBits as U16;
    let mut total = ((1 as libc::c_int) << maxBits) as U32;
    let mut rest = total.wrapping_sub(weightTotal);
    let mut verif = ((1 as libc::c_int) << FSE_highbit32(rest)) as U32;
    let mut lastWeight = (FSE_highbit32(rest))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if verif != rest {
        return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
    }
    huffWeight[oSize as usize] = lastWeight as BYTE;
    rankVal[lastWeight as usize] = (rankVal[lastWeight as usize]).wrapping_add(1);
    if rankVal[1 as libc::c_int as usize] < 2 as libc::c_int as libc::c_uint
        || rankVal[1 as libc::c_int as usize] & 1 as libc::c_int as libc::c_uint != 0
    {
        return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
    }
    nextRankStart = 0 as libc::c_int as U32;
    n = 1 as libc::c_int as U32;
    while n <= maxBits {
        let mut current = nextRankStart;
        nextRankStart = (nextRankStart as libc::c_uint)
            .wrapping_add(
                rankVal[n as usize] << n.wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) as U32 as U32;
        rankVal[n as usize] = current;
        n = n.wrapping_add(1);
    }
    n = 0 as libc::c_int as U32;
    while n as libc::c_ulong <= oSize {
        let w = huffWeight[n as usize] as U32;
        let length = ((1 as libc::c_int) << w >> 1 as libc::c_int) as U32;
        let mut i: U32 = 0;
        let mut D = HUF_DElt { byte: 0, nbBits: 0 };
        D.byte = n as BYTE;
        D
            .nbBits = maxBits
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(w) as BYTE;
        i = rankVal[w as usize];
        while i < (rankVal[w as usize]).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
        }
        rankVal[w
            as usize] = (rankVal[w as usize] as libc::c_uint).wrapping_add(length) as U32
            as U32;
        n = n.wrapping_add(1);
    }
    return iSize.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn HUF_decodeSymbol(
    mut Dstream: *mut FSE_DStream_t,
    mut dt: *const HUF_DElt,
    dtLog: U32,
) -> BYTE {
    let val = FSE_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    FSE_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as U32);
    return c;
}
unsafe extern "C" fn HUF_decompress_usingDTable(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U16,
) -> size_t {
    if cSrcSize < 6 as libc::c_int as libc::c_ulong {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = if maxDstSize < 15 as libc::c_int as libc::c_ulong {
        op
    } else {
        omax.offset(-(15 as libc::c_int as isize))
    };
    let mut ptr = DTable as *const libc::c_void;
    let dt = (ptr as *const HUF_DElt).offset(1 as libc::c_int as isize);
    let dtLog = *DTable.offset(0 as libc::c_int as isize) as U32;
    let mut errorCode: size_t = 0;
    let mut reloadStatus: U32 = 0;
    let mut jumpTable = cSrc as *const U16;
    let length1 = FSE_readLE16(jumpTable as *const libc::c_void) as size_t;
    let length2 = FSE_readLE16(
        jumpTable.offset(1 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length3 = FSE_readLE16(
        jumpTable.offset(2 as libc::c_int as isize) as *const libc::c_void,
    ) as size_t;
    let length4 = cSrcSize
        .wrapping_sub(6 as libc::c_int as libc::c_ulong)
        .wrapping_sub(length1)
        .wrapping_sub(length2)
        .wrapping_sub(length3);
    let start1 = (cSrc as *const libc::c_char).offset(6 as libc::c_int as isize);
    let start2 = start1.offset(length1 as isize);
    let start3 = start2.offset(length2 as isize);
    let start4 = start3.offset(length3 as isize);
    let mut bitD1 = FSE_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD2 = FSE_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD3 = FSE_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    let mut bitD4 = FSE_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    if length1
        .wrapping_add(length2)
        .wrapping_add(length3)
        .wrapping_add(6 as libc::c_int as libc::c_ulong) >= cSrcSize
    {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    errorCode = FSE_initDStream(&mut bitD1, start1 as *const libc::c_void, length1);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = FSE_initDStream(&mut bitD2, start2 as *const libc::c_void, length2);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = FSE_initDStream(&mut bitD3, start3 as *const libc::c_void, length3);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = FSE_initDStream(&mut bitD4, start4 as *const libc::c_void, length4);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    reloadStatus = FSE_reloadDStream(&mut bitD2);
    while reloadStatus < FSE_DStream_completed as libc::c_int as libc::c_uint
        && op < olimit
    {
        *op.offset(0 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD1, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD1);
        }
        *op.offset(1 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD2, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD2);
        }
        *op.offset(2 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD3, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD3);
        }
        *op.offset(3 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD4, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD4);
        }
        *op.offset(4 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD1, dt, dtLog);
        if FSE_32bits() != 0 {
            FSE_reloadDStream(&mut bitD1);
        }
        *op.offset(5 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD2, dt, dtLog);
        if FSE_32bits() != 0 {
            FSE_reloadDStream(&mut bitD2);
        }
        *op.offset(6 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD3, dt, dtLog);
        if FSE_32bits() != 0 {
            FSE_reloadDStream(&mut bitD3);
        }
        *op.offset(7 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD4, dt, dtLog);
        if FSE_32bits() != 0 {
            FSE_reloadDStream(&mut bitD4);
        }
        *op.offset(8 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD1, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD1);
        }
        *op.offset(9 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD2, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD2);
        }
        *op.offset(10 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD3, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD3);
        }
        *op.offset(11 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD4, dt, dtLog);
        if FSE_32bits() != 0 && HUF_MAX_TABLELOG > 12 as libc::c_int {
            FSE_reloadDStream(&mut bitD4);
        }
        *op.offset(12 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD1, dt, dtLog);
        *op.offset(13 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD2, dt, dtLog);
        *op.offset(14 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD3, dt, dtLog);
        *op.offset(15 as libc::c_int as isize) = HUF_decodeSymbol(&mut bitD4, dt, dtLog);
        op = op.offset(16 as libc::c_int as isize);
        reloadStatus = FSE_reloadDStream(&mut bitD2) | FSE_reloadDStream(&mut bitD3)
            | FSE_reloadDStream(&mut bitD4);
        FSE_reloadDStream(&mut bitD1);
    }
    if reloadStatus != FSE_DStream_completed as libc::c_int as libc::c_uint {
        return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
    }
    let mut bitTail = FSE_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
    };
    bitTail.ptr = bitD1.ptr;
    bitTail.bitsConsumed = bitD1.bitsConsumed;
    bitTail.bitContainer = bitD1.bitContainer;
    bitTail.start = start1;
    while FSE_reloadDStream(&mut bitTail)
        < FSE_DStream_completed as libc::c_int as libc::c_uint && op < omax
    {
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = HUF_decodeSymbol(&mut bitTail, dt, dtLog);
        op = op.offset(1);
    }
    if FSE_endOfDStream(&mut bitTail) != 0 {
        return op.offset_from(ostart) as libc::c_long as size_t;
    }
    if op == omax {
        return -(FSE_ERROR_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return -(FSE_ERROR_corruptionDetected as libc::c_int) as size_t;
}
unsafe extern "C" fn HUF_decompress(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [libc::c_ushort; 4097] = [
        12 as libc::c_int as libc::c_ushort,
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ip = cSrc as *const BYTE;
    let mut errorCode: size_t = 0;
    errorCode = HUF_readDTable(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if FSE_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(FSE_ERROR_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(errorCode) as size_t as size_t;
    return HUF_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    );
}
static mut ZSTD_magicNumber: U32 = 0xfd2fb51e as libc::c_uint;
pub const BLOCKSIZE: libc::c_int = 128 as libc::c_int
    * ((1 as libc::c_int) << 10 as libc::c_int);
pub const MINMATCH: libc::c_int = 4 as libc::c_int;
pub const MLbits: libc::c_int = 7 as libc::c_int;
pub const LLbits: libc::c_int = 6 as libc::c_int;
pub const Offbits: libc::c_int = 5 as libc::c_int;
pub const MaxML: libc::c_int = ((1 as libc::c_int) << MLbits) - 1 as libc::c_int;
pub const MaxLL: libc::c_int = ((1 as libc::c_int) << LLbits) - 1 as libc::c_int;
pub const MaxOff: libc::c_int = ((1 as libc::c_int) << Offbits) - 1 as libc::c_int;
pub const MLFSELog: libc::c_int = 10 as libc::c_int;
pub const LLFSELog: libc::c_int = 10 as libc::c_int;
pub const OffFSELog: libc::c_int = 9 as libc::c_int;
pub const ZSTD_CONTENTSIZE_ERROR: libc::c_ulonglong = (0 as libc::c_ulonglong)
    .wrapping_sub(2 as libc::c_int as libc::c_ulonglong);
static mut ZSTD_blockHeaderSize: size_t = 3 as libc::c_int as size_t;
static mut ZSTD_frameHeaderSize: size_t = 4 as libc::c_int as size_t;
unsafe extern "C" fn ZSTD_32bits() -> libc::c_uint {
    return (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ZSTD_isLittleEndian() -> libc::c_uint {
    let one = C2RustUnnamed {
        i: 1 as libc::c_int as U32,
    };
    return one.c[0 as libc::c_int as usize] as libc::c_uint;
}
unsafe extern "C" fn ZSTD_read16(mut p: *const libc::c_void) -> U16 {
    let mut r: U16 = 0;
    memcpy(
        &mut r as *mut U16 as *mut libc::c_void,
        p,
        ::core::mem::size_of::<U16>() as libc::c_ulong,
    );
    return r;
}
unsafe extern "C" fn ZSTD_copy4(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    memcpy(dst, src, 4 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_copy8(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    memcpy(dst, src, 8 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut length: ptrdiff_t,
) {
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let oend = op.offset(length as isize);
    while op < oend {
        ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
        op = op.offset(8 as libc::c_int as isize);
        ip = ip.offset(8 as libc::c_int as isize);
    }
}
unsafe extern "C" fn ZSTD_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if ZSTD_isLittleEndian() != 0 {
        return ZSTD_read16(memPtr)
    } else {
        let mut p = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as U16 as libc::c_int
            + ((*p.offset(1 as libc::c_int as isize) as U16 as libc::c_int)
                << 8 as libc::c_int)) as U16;
    };
}
unsafe extern "C" fn ZSTD_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (ZSTD_readLE16(memPtr) as libc::c_int
        + ((*(memPtr as *const BYTE).offset(2 as libc::c_int as isize) as libc::c_int)
            << 16 as libc::c_int)) as U32;
}
unsafe extern "C" fn ZSTD_readBE32(mut memPtr: *const libc::c_void) -> U32 {
    let mut p = memPtr as *const BYTE;
    return ((*p.offset(0 as libc::c_int as isize) as U32) << 24 as libc::c_int)
        .wrapping_add((*p.offset(1 as libc::c_int as isize) as U32) << 16 as libc::c_int)
        .wrapping_add((*p.offset(2 as libc::c_int as isize) as U32) << 8 as libc::c_int)
        .wrapping_add((*p.offset(3 as libc::c_int as isize) as U32) << 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
unsafe extern "C" fn ZSTDv01_getcBlockSize(
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const BYTE;
    let mut headerFlags: BYTE = 0;
    let mut cSize: U32 = 0;
    if srcSize < 3 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    headerFlags = *in_0;
    cSize = (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
        + ((*in_0.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        + ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int)
            << 16 as libc::c_int)) as U32;
    (*bpPtr).blockType = (headerFlags as libc::c_int >> 6 as libc::c_int) as blockType_t;
    (*bpPtr)
        .origSize = if (*bpPtr).blockType as libc::c_uint
        == bt_rle as libc::c_int as libc::c_uint
    {
        cSize
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if (*bpPtr).blockType as libc::c_uint == bt_end as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as size_t;
    }
    if (*bpPtr).blockType as libc::c_uint == bt_rle as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as size_t;
    }
    return cSize as size_t;
}
unsafe extern "C" fn ZSTD_copyUncompressedBlock(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize > maxDstSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if srcSize > 0 as libc::c_int as libc::c_ulong {
        memcpy(dst, src, srcSize);
    }
    return srcSize;
}
unsafe extern "C" fn ZSTD_decompressLiterals(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut op = dst as *mut BYTE;
    let oend = op.offset(maxDstSize as isize);
    let mut ip = src as *const BYTE;
    let mut errorCode: size_t = 0;
    let mut litSize: size_t = 0;
    if srcSize <= 3 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    litSize = (*ip.offset(1 as libc::c_int as isize) as libc::c_int
        + ((*ip.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as size_t;
    litSize = (litSize as libc::c_ulong)
        .wrapping_add(
            ((*ip.offset(-(3 as libc::c_int) as isize) as libc::c_int >> 3 as libc::c_int
                & 7 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong,
        ) as size_t as size_t;
    op = oend.offset(-(litSize as isize));
    if litSize > maxDstSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    errorCode = HUF_decompress(
        op as *mut libc::c_void,
        litSize,
        ip.offset(2 as libc::c_int as isize) as *const libc::c_void,
        srcSize.wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    if FSE_isError(errorCode) != 0 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    }
    return litSize;
}
unsafe extern "C" fn ZSTDv01_decodeLiteralsBlock(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut litStart: *mut *const BYTE,
    mut litSize: *mut size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const BYTE;
    let mut ip = istart;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(maxDstSize as isize);
    let mut litbp = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    let mut litcSize = ZSTDv01_getcBlockSize(src, srcSize, &mut litbp);
    if ZSTDv01_isError(litcSize) != 0 {
        return litcSize;
    }
    if litcSize > srcSize.wrapping_sub(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    ip = ip.offset(ZSTD_blockHeaderSize as isize);
    match litbp.blockType as libc::c_uint {
        1 => {
            *litStart = ip;
            ip = ip.offset(litcSize as isize);
            *litSize = litcSize;
        }
        2 => {
            let mut rleSize = litbp.origSize as size_t;
            if rleSize > maxDstSize {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
            }
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            if rleSize > 0 as libc::c_int as libc::c_ulong {
                memset(
                    oend.offset(-(rleSize as isize)) as *mut libc::c_void,
                    *ip as libc::c_int,
                    rleSize,
                );
            }
            *litStart = oend.offset(-(rleSize as isize));
            *litSize = rleSize;
            ip = ip.offset(1);
        }
        0 => {
            let mut decodedLitSize = ZSTD_decompressLiterals(
                ctx,
                dst,
                maxDstSize,
                ip as *const libc::c_void,
                litcSize,
            );
            if ZSTDv01_isError(decodedLitSize) != 0 {
                return decodedLitSize;
            }
            *litStart = oend.offset(-(decodedLitSize as isize));
            *litSize = decodedLitSize;
            ip = ip.offset(litcSize as isize);
        }
        3 | _ => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
    }
    return ip.offset_from(istart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTDv01_decodeSeqHeaders(
    mut nbSeq: *mut libc::c_int,
    mut dumpsPtr: *mut *const BYTE,
    mut dumpsLengthPtr: *mut size_t,
    mut DTableLL: *mut FSE_DTable,
    mut DTableML: *mut FSE_DTable,
    mut DTableOffb: *mut FSE_DTable,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const BYTE;
    let mut ip = istart;
    let iend = istart.offset(srcSize as isize);
    let mut LLtype: U32 = 0;
    let mut Offtype: U32 = 0;
    let mut MLtype: U32 = 0;
    let mut LLlog: U32 = 0;
    let mut Offlog: U32 = 0;
    let mut MLlog: U32 = 0;
    let mut dumpsLength: size_t = 0;
    if srcSize < 5 as libc::c_int as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    *nbSeq = ZSTD_readLE16(ip as *const libc::c_void) as libc::c_int;
    ip = ip.offset(2 as libc::c_int as isize);
    LLtype = (*ip as libc::c_int >> 6 as libc::c_int) as U32;
    Offtype = (*ip as libc::c_int >> 4 as libc::c_int & 3 as libc::c_int) as U32;
    MLtype = (*ip as libc::c_int >> 2 as libc::c_int & 3 as libc::c_int) as U32;
    if *ip as libc::c_int & 2 as libc::c_int != 0 {
        dumpsLength = *ip.offset(2 as libc::c_int as isize) as size_t;
        dumpsLength = (dumpsLength as libc::c_ulong)
            .wrapping_add(
                ((*ip.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
        ip = ip.offset(3 as libc::c_int as isize);
    } else {
        dumpsLength = *ip.offset(1 as libc::c_int as isize) as size_t;
        dumpsLength = (dumpsLength as libc::c_ulong)
            .wrapping_add(
                ((*ip.offset(0 as libc::c_int as isize) as libc::c_int
                    & 1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
        ip = ip.offset(2 as libc::c_int as isize);
    }
    *dumpsPtr = ip;
    ip = ip.offset(dumpsLength as isize);
    *dumpsLengthPtr = dumpsLength;
    if ip > iend.offset(-(3 as libc::c_int as isize)) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let mut norm: [S16; 128] = [0; 128];
    let mut headerSize: size_t = 0;
    match LLtype {
        2 => {
            LLlog = 0 as libc::c_int as U32;
            let fresh6 = ip;
            ip = ip.offset(1);
            FSE_buildDTable_rle(DTableLL, *fresh6);
        }
        1 => {
            LLlog = LLbits as U32;
            FSE_buildDTable_raw(DTableLL, LLbits as libc::c_uint);
        }
        _ => {
            let mut max = MaxLL as U32;
            headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut LLlog,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            );
            if FSE_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            if LLlog > LLFSELog as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSE_buildDTable(DTableLL, norm.as_mut_ptr(), max, LLlog);
        }
    }
    match Offtype {
        2 => {
            Offlog = 0 as libc::c_int as U32;
            if ip > iend.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            let fresh7 = ip;
            ip = ip.offset(1);
            FSE_buildDTable_rle(DTableOffb, *fresh7);
        }
        1 => {
            Offlog = Offbits as U32;
            FSE_buildDTable_raw(DTableOffb, Offbits as libc::c_uint);
        }
        _ => {
            let mut max_0 = MaxOff as U32;
            headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max_0,
                &mut Offlog,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            );
            if FSE_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            if Offlog > OffFSELog as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSE_buildDTable(DTableOffb, norm.as_mut_ptr(), max_0, Offlog);
        }
    }
    match MLtype {
        2 => {
            MLlog = 0 as libc::c_int as U32;
            if ip > iend.offset(-(2 as libc::c_int as isize)) {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
            }
            let fresh8 = ip;
            ip = ip.offset(1);
            FSE_buildDTable_rle(DTableML, *fresh8);
        }
        1 => {
            MLlog = MLbits as U32;
            FSE_buildDTable_raw(DTableML, MLbits as libc::c_uint);
        }
        _ => {
            let mut max_1 = MaxML as U32;
            headerSize = FSE_readNCount(
                norm.as_mut_ptr(),
                &mut max_1,
                &mut MLlog,
                ip as *const libc::c_void,
                iend.offset_from(ip) as libc::c_long as size_t,
            );
            if FSE_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t;
            }
            if MLlog > MLFSELog as libc::c_uint {
                return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSE_buildDTable(DTableML, norm.as_mut_ptr(), max_1, MLlog);
        }
    }
    return ip.offset_from(istart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decodeSequence(
    mut seq: *mut seq_t,
    mut seqState: *mut seqState_t,
) {
    let mut litLength: size_t = 0;
    let mut prevOffset: size_t = 0;
    let mut offset: size_t = 0;
    let mut matchLength: size_t = 0;
    let mut dumps = (*seqState).dumps;
    let de = (*seqState).dumpsEnd;
    litLength = FSE_decodeSymbol(&mut (*seqState).stateLL, &mut (*seqState).DStream)
        as size_t;
    prevOffset = if litLength != 0 { (*seq).offset } else { (*seqState).prevOffset };
    (*seqState).prevOffset = (*seq).offset;
    if litLength == MaxLL as libc::c_ulong {
        let add = (if dumps < de {
            let fresh9 = dumps;
            dumps = dumps.offset(1);
            *fresh9 as libc::c_int
        } else {
            0 as libc::c_int
        }) as U32;
        if add < 255 as libc::c_int as libc::c_uint {
            litLength = (litLength as libc::c_ulong).wrapping_add(add as libc::c_ulong)
                as size_t as size_t;
        } else if dumps <= de.offset(-(3 as libc::c_int as isize)) {
            litLength = ZSTD_readLE24(dumps as *const libc::c_void) as size_t;
            dumps = dumps.offset(3 as libc::c_int as isize);
        }
    }
    let mut offsetCode: U32 = 0;
    let mut nbBits: U32 = 0;
    offsetCode = FSE_decodeSymbol(&mut (*seqState).stateOffb, &mut (*seqState).DStream)
        as U32;
    if ZSTD_32bits() != 0 {
        FSE_reloadDStream(&mut (*seqState).DStream);
    }
    nbBits = offsetCode.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if offsetCode == 0 as libc::c_int as libc::c_uint {
        nbBits = 0 as libc::c_int as U32;
    }
    offset = ((1 as libc::c_int as size_t)
        << (nbBits as libc::c_ulong
            & (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)))
        .wrapping_add(FSE_readBits(&mut (*seqState).DStream, nbBits));
    if ZSTD_32bits() != 0 {
        FSE_reloadDStream(&mut (*seqState).DStream);
    }
    if offsetCode == 0 as libc::c_int as libc::c_uint {
        offset = prevOffset;
    }
    matchLength = FSE_decodeSymbol(&mut (*seqState).stateML, &mut (*seqState).DStream)
        as size_t;
    if matchLength == MaxML as libc::c_ulong {
        let add_0 = (if dumps < de {
            let fresh10 = dumps;
            dumps = dumps.offset(1);
            *fresh10 as libc::c_int
        } else {
            0 as libc::c_int
        }) as U32;
        if add_0 < 255 as libc::c_int as libc::c_uint {
            matchLength = (matchLength as libc::c_ulong)
                .wrapping_add(add_0 as libc::c_ulong) as size_t as size_t;
        } else if dumps <= de.offset(-(3 as libc::c_int as isize)) {
            matchLength = ZSTD_readLE24(dumps as *const libc::c_void) as size_t;
            dumps = dumps.offset(3 as libc::c_int as isize);
        }
    }
    matchLength = (matchLength as libc::c_ulong).wrapping_add(MINMATCH as libc::c_ulong)
        as size_t as size_t;
    (*seq).litLength = litLength;
    (*seq).offset = offset;
    (*seq).matchLength = matchLength;
    (*seqState).dumps = dumps;
}
unsafe extern "C" fn ZSTD_execSequence(
    mut op: *mut BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    base: *mut BYTE,
    oend: *mut BYTE,
) -> size_t {
    static mut dec32table: [libc::c_int; 8] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
    ];
    static mut dec64table: [libc::c_int; 8] = [
        8 as libc::c_int,
        8 as libc::c_int,
        8 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    let ostart: *const BYTE = op;
    let oLitEnd = op.offset(sequence.litLength as isize);
    let litLength = sequence.litLength;
    let endMatch = op.offset(litLength as isize).offset(sequence.matchLength as isize);
    let litEnd = (*litPtr).offset(litLength as isize);
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as libc::c_long as size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if sequence.offset
        > oLitEnd.offset_from(base) as libc::c_long as U32 as libc::c_ulong
    {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if endMatch > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if litEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if sequence.matchLength > (*litPtr).offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    libc::memmove(
        op as *mut libc::c_void,
        *litPtr as *const libc::c_void,
        sequence.litLength as libc::size_t,
    );
    op = op.offset(litLength as isize);
    *litPtr = litEnd;
    if (oend.offset_from(op) as libc::c_long) < 8 as libc::c_int as libc::c_long {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    let overlapRisk = ((litEnd.offset_from(endMatch) as libc::c_long as size_t)
        < 12 as libc::c_int as libc::c_ulong) as libc::c_int as U32;
    let mut match_0: *const BYTE = op.offset(-(sequence.offset as isize));
    let mut qutt = 12 as libc::c_int as size_t;
    let mut saved: [U64; 2] = [0; 2];
    if match_0 < base as *const BYTE {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if sequence.offset > base as size_t {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if overlapRisk != 0 {
        if endMatch.offset(qutt as isize) > oend {
            qutt = oend.offset_from(endMatch) as libc::c_long as size_t;
        }
        memcpy(
            saved.as_mut_ptr() as *mut libc::c_void,
            endMatch as *const libc::c_void,
            qutt,
        );
    }
    if sequence.offset < 8 as libc::c_int as libc::c_ulong {
        let dec64 = dec64table[sequence.offset as usize];
        *op
            .offset(
                0 as libc::c_int as isize,
            ) = *match_0.offset(0 as libc::c_int as isize);
        *op
            .offset(
                1 as libc::c_int as isize,
            ) = *match_0.offset(1 as libc::c_int as isize);
        *op
            .offset(
                2 as libc::c_int as isize,
            ) = *match_0.offset(2 as libc::c_int as isize);
        *op
            .offset(
                3 as libc::c_int as isize,
            ) = *match_0.offset(3 as libc::c_int as isize);
        match_0 = match_0.offset(dec32table[sequence.offset as usize] as isize);
        ZSTD_copy4(
            op.offset(4 as libc::c_int as isize) as *mut libc::c_void,
            match_0 as *const libc::c_void,
        );
        match_0 = match_0.offset(-(dec64 as isize));
    } else {
        ZSTD_copy8(op as *mut libc::c_void, match_0 as *const libc::c_void);
    }
    op = op.offset(8 as libc::c_int as isize);
    match_0 = match_0.offset(8 as libc::c_int as isize);
    if endMatch > oend.offset(-((16 as libc::c_int - MINMATCH) as isize)) {
        if op < oend.offset(-(8 as libc::c_int as isize)) {
            ZSTD_wildcopy(
                op as *mut libc::c_void,
                match_0 as *const libc::c_void,
                oend.offset(-(8 as libc::c_int as isize)).offset_from(op) as libc::c_long,
            );
            match_0 = match_0
                .offset(
                    oend.offset(-(8 as libc::c_int as isize)).offset_from(op)
                        as libc::c_long as isize,
                );
            op = oend.offset(-(8 as libc::c_int as isize));
        }
        while op < endMatch {
            let fresh11 = match_0;
            match_0 = match_0.offset(1);
            let fresh12 = op;
            op = op.offset(1);
            *fresh12 = *fresh11;
        }
    } else {
        ZSTD_wildcopy(
            op as *mut libc::c_void,
            match_0 as *const libc::c_void,
            sequence.matchLength as ptrdiff_t - 8 as libc::c_int as libc::c_long,
        );
    }
    if overlapRisk != 0 {
        memcpy(
            endMatch as *mut libc::c_void,
            saved.as_mut_ptr() as *const libc::c_void,
            qutt,
        );
    }
    return endMatch.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decompressSequences(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const libc::c_void,
    mut seqSize: size_t,
    mut litStart: *const BYTE,
    mut litSize: size_t,
) -> size_t {
    let mut dctx = ctx as *mut dctx_t;
    let mut ip = seqStart as *const BYTE;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(maxDstSize as isize);
    let mut errorCode: size_t = 0;
    let mut dumpsLength: size_t = 0;
    let mut litPtr = litStart;
    let litEnd = litStart.offset(litSize as isize);
    let mut nbSeq: libc::c_int = 0;
    let mut dumps = 0 as *const BYTE;
    let mut DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let mut DTableML = ((*dctx).MLTable).as_mut_ptr();
    let mut DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *mut BYTE;
    errorCode = ZSTDv01_decodeSeqHeaders(
        &mut nbSeq,
        &mut dumps,
        &mut dumpsLength,
        DTableLL,
        DTableML,
        DTableOffb,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
    );
    if ZSTDv01_isError(errorCode) != 0 {
        return errorCode;
    }
    ip = ip.offset(errorCode as isize);
    let mut sequence = seq_t {
        litLength: 0,
        offset: 0,
        matchLength: 0,
    };
    let mut seqState = seqState_t {
        DStream: FSE_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: 0 as *const libc::c_char,
            start: 0 as *const libc::c_char,
        },
        stateLL: FSE_DState_t {
            state: 0,
            table: 0 as *const libc::c_void,
        },
        stateOffb: FSE_DState_t {
            state: 0,
            table: 0 as *const libc::c_void,
        },
        stateML: FSE_DState_t {
            state: 0,
            table: 0 as *const libc::c_void,
        },
        prevOffset: 0,
        dumps: 0 as *const BYTE,
        dumpsEnd: 0 as *const BYTE,
    };
    memset(
        &mut sequence as *mut seq_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<seq_t>() as libc::c_ulong,
    );
    seqState.dumps = dumps;
    seqState.dumpsEnd = dumps.offset(dumpsLength as isize);
    seqState.prevOffset = 1 as libc::c_int as size_t;
    errorCode = FSE_initDStream(
        &mut seqState.DStream,
        ip as *const libc::c_void,
        iend.offset_from(ip) as libc::c_long as size_t,
    );
    if FSE_isError(errorCode) != 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    FSE_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
    FSE_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
    FSE_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
    while FSE_reloadDStream(&mut seqState.DStream)
        <= FSE_DStream_completed as libc::c_int as libc::c_uint
        && nbSeq > 0 as libc::c_int
    {
        let mut oneSeqSize: size_t = 0;
        nbSeq -= 1;
        ZSTD_decodeSequence(&mut sequence, &mut seqState);
        oneSeqSize = ZSTD_execSequence(op, sequence, &mut litPtr, litEnd, base, oend);
        if ZSTDv01_isError(oneSeqSize) != 0 {
            return oneSeqSize;
        }
        op = op.offset(oneSeqSize as isize);
    }
    if FSE_endOfDStream(&mut seqState.DStream) == 0 {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    if nbSeq < 0 as libc::c_int {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t;
    }
    let mut lastLLSize = litEnd.offset_from(litPtr) as libc::c_long as size_t;
    if op.offset(lastLLSize as isize) > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    if lastLLSize > 0 as libc::c_int as libc::c_ulong {
        if op != litPtr as *mut BYTE {
            memmove(op as *mut libc::c_void, litPtr as *const libc::c_void, lastLLSize);
        }
        op = op.offset(lastLLSize as isize);
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decompressBlock(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    let mut litPtr = NULL as *const BYTE;
    let mut litSize = 0 as libc::c_int as size_t;
    let mut errorCode: size_t = 0;
    errorCode = ZSTDv01_decodeLiteralsBlock(
        ctx,
        dst,
        maxDstSize,
        &mut litPtr,
        &mut litSize,
        src,
        srcSize,
    );
    if ZSTDv01_isError(errorCode) != 0 {
        return errorCode;
    }
    ip = ip.offset(errorCode as isize);
    srcSize = (srcSize as libc::c_ulong).wrapping_sub(errorCode) as size_t as size_t;
    return ZSTD_decompressSequences(
        ctx,
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        srcSize,
        litPtr,
        litSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_decompressDCtx(
    mut ctx: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    let mut iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(maxDstSize as isize);
    let mut remainingSize = srcSize;
    let mut magicNumber: U32 = 0;
    let mut errorCode = 0 as libc::c_int as size_t;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if srcSize < ZSTD_frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    magicNumber = ZSTD_readBE32(src);
    if magicNumber != ZSTD_magicNumber {
        return -(ZSTD_error_prefix_unknown as libc::c_int) as size_t;
    }
    ip = ip.offset(ZSTD_frameHeaderSize as isize);
    remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(ZSTD_frameHeaderSize)
        as size_t as size_t;
    loop {
        let mut blockSize = ZSTDv01_getcBlockSize(
            ip as *const libc::c_void,
            iend.offset_from(ip) as libc::c_long as size_t,
            &mut blockProperties,
        );
        if ZSTDv01_isError(blockSize) != 0 {
            return blockSize;
        }
        ip = ip.offset(ZSTD_blockHeaderSize as isize);
        remainingSize = (remainingSize as libc::c_ulong)
            .wrapping_sub(ZSTD_blockHeaderSize) as size_t as size_t;
        if blockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
        }
        match blockProperties.blockType as libc::c_uint {
            0 => {
                errorCode = ZSTD_decompressBlock(
                    ctx,
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as size_t,
                    ip as *const libc::c_void,
                    blockSize,
                );
            }
            1 => {
                errorCode = ZSTD_copyUncompressedBlock(
                    op as *mut libc::c_void,
                    oend.offset_from(op) as libc::c_long as size_t,
                    ip as *const libc::c_void,
                    blockSize,
                );
            }
            2 => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
                }
            }
            _ => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
        }
        if blockSize == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if ZSTDv01_isError(errorCode) != 0 {
            return errorCode;
        }
        op = op.offset(errorCode as isize);
        ip = ip.offset(blockSize as isize);
        remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(blockSize)
            as size_t as size_t;
    }
    return op.offset_from(ostart) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_decompress(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ctx = dctx_t {
        LLTable: [0; 1025],
        OffTable: [0; 513],
        MLTable: [0; 1025],
        previousDstEnd: 0 as *mut libc::c_void,
        base: 0 as *mut libc::c_void,
        expected: 0,
        bType: bt_compressed,
        phase: 0,
    };
    ctx.base = dst;
    return ZSTDv01_decompressDCtx(
        &mut ctx as *mut dctx_t as *mut libc::c_void,
        dst,
        maxDstSize,
        src,
        srcSize,
    );
}
unsafe extern "C" fn ZSTD_errorFrameSizeInfoLegacy(
    mut cSize: *mut size_t,
    mut dBound: *mut libc::c_ulonglong,
    mut ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_findFrameSizeInfoLegacy(
    mut src: *const libc::c_void,
    mut srcSize: size_t,
    mut cSize: *mut size_t,
    mut dBound: *mut libc::c_ulonglong,
) {
    let mut ip = src as *const BYTE;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as libc::c_int as size_t;
    let mut magicNumber: U32 = 0;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if srcSize < ZSTD_frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t,
        );
        return;
    }
    magicNumber = ZSTD_readBE32(src);
    if magicNumber != ZSTD_magicNumber {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as libc::c_int) as size_t,
        );
        return;
    }
    ip = ip.offset(ZSTD_frameHeaderSize as isize);
    remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(ZSTD_frameHeaderSize)
        as size_t as size_t;
    loop {
        let mut blockSize = ZSTDv01_getcBlockSize(
            ip as *const libc::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ZSTDv01_isError(blockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, blockSize);
            return;
        }
        ip = ip.offset(ZSTD_blockHeaderSize as isize);
        remainingSize = (remainingSize as libc::c_ulong)
            .wrapping_sub(ZSTD_blockHeaderSize) as size_t as size_t;
        if blockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t,
            );
            return;
        }
        if blockSize == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        ip = ip.offset(blockSize as isize);
        remainingSize = (remainingSize as libc::c_ulong).wrapping_sub(blockSize)
            as size_t as size_t;
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const BYTE) as libc::c_long as size_t;
    *dBound = nbBlocks.wrapping_mul(BLOCKSIZE as libc::c_ulong) as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_resetDCtx(mut dctx: *mut ZSTDv01_Dctx) -> size_t {
    (*dctx).expected = ZSTD_frameHeaderSize;
    (*dctx).phase = 0 as libc::c_int as U32;
    (*dctx).previousDstEnd = NULL as *mut libc::c_void;
    (*dctx).base = NULL as *mut libc::c_void;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_createDCtx() -> *mut ZSTDv01_Dctx {
    let mut dctx = malloc(::core::mem::size_of::<ZSTDv01_Dctx>() as libc::c_ulong)
        as *mut ZSTDv01_Dctx;
    if dctx.is_null() {
        return NULL as *mut ZSTDv01_Dctx;
    }
    ZSTDv01_resetDCtx(dctx);
    return dctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_freeDCtx(mut dctx: *mut ZSTDv01_Dctx) -> size_t {
    free(dctx as *mut libc::c_void);
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_nextSrcSizeToDecompress(
    mut dctx: *mut ZSTDv01_Dctx,
) -> size_t {
    return (*(dctx as *mut dctx_t)).expected;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv01_decompressContinue(
    mut dctx: *mut ZSTDv01_Dctx,
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ctx = dctx as *mut dctx_t;
    if srcSize != (*ctx).expected {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t;
    }
    if dst != (*ctx).previousDstEnd {
        (*ctx).base = dst;
    }
    if (*ctx).phase == 0 as libc::c_int as libc::c_uint {
        let mut magicNumber = ZSTD_readBE32(src);
        if magicNumber != ZSTD_magicNumber {
            return -(ZSTD_error_prefix_unknown as libc::c_int) as size_t;
        }
        (*ctx).phase = 1 as libc::c_int as U32;
        (*ctx).expected = ZSTD_blockHeaderSize;
        return 0 as libc::c_int as size_t;
    }
    if (*ctx).phase == 1 as libc::c_int as libc::c_uint {
        let mut bp = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let mut blockSize = ZSTDv01_getcBlockSize(src, ZSTD_blockHeaderSize, &mut bp);
        if ZSTDv01_isError(blockSize) != 0 {
            return blockSize;
        }
        if bp.blockType as libc::c_uint == bt_end as libc::c_int as libc::c_uint {
            (*ctx).expected = 0 as libc::c_int as size_t;
            (*ctx).phase = 0 as libc::c_int as U32;
        } else {
            (*ctx).expected = blockSize;
            (*ctx).bType = bp.blockType;
            (*ctx).phase = 2 as libc::c_int as U32;
        }
        return 0 as libc::c_int as size_t;
    }
    let mut rSize: size_t = 0;
    match (*ctx).bType as libc::c_uint {
        0 => {
            rSize = ZSTD_decompressBlock(
                ctx as *mut libc::c_void,
                dst,
                maxDstSize,
                src,
                srcSize,
            );
        }
        1 => {
            rSize = ZSTD_copyUncompressedBlock(dst, maxDstSize, src, srcSize);
        }
        2 => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
        3 => {
            rSize = 0 as libc::c_int as size_t;
        }
        _ => return -(ZSTD_error_GENERIC as libc::c_int) as size_t,
    }
    (*ctx).phase = 1 as libc::c_int as U32;
    (*ctx).expected = ZSTD_blockHeaderSize;
    (*ctx)
        .previousDstEnd = (dst as *mut libc::c_char).offset(rSize as isize)
        as *mut libc::c_void;
    return rSize;
}
