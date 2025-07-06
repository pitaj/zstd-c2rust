use ::libc;
use crate::common::fse_h::*;
extern "C" {
    fn FSE_readNCount_bmi2(
        normalizedCounter: *mut i16,
        maxSymbolValuePtr: *mut u32,
        tableLogPtr: *mut u32,
        rBuffer: *const std::ffi::c_void,
        rBuffSize: usize,
        bmi2: i32,
    ) -> usize;
}
pub type unalign32 = u32;
pub type unalign64 = u64;
use crate::common::error::*;
pub type BitContainerType = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: BitContainerType,
    pub bitsConsumed: u32,
    pub ptr: *const std::ffi::c_char,
    pub start: *const std::ffi::c_char,
    pub limitPtr: *const std::ffi::c_char,
}
pub type BIT_DStream_status = u32;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: u16,
    pub symbol: std::ffi::c_uchar,
    pub nbBits: std::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DecompressWksp {
    pub ncount: [i16; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DState_t {
    pub state: usize,
    pub table: *const std::ffi::c_void,
}
use crate::common::mem::*;
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const std::ffi::c_char,
    mut args: ...
) {}
use crate::common::bits::*;
#[inline]
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    if srcSize < 1 {
        libc::memset(
            bitD as *mut std::ffi::c_void,
            0,
            ::core::mem::size_of::<BIT_DStream_t>() as usize,
        );
        return ERROR(ZSTD_error_srcSize_wrong);
    }
    (*bitD).start = srcBuffer as *const std::ffi::c_char;
    (*bitD)
        .limitPtr = ((*bitD).start)
        .offset(
            ::core::mem::size_of::<BitContainerType>() as isize,
        );
    if srcSize >= ::core::mem::size_of::<BitContainerType>() {
        (*bitD)
            .ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(
                -(::core::mem::size_of::<BitContainerType>()
                    as isize),
            );
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        let lastByte = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte as i32 != 0 {
            (8 as u32)
                .wrapping_sub(ZSTD_highbit32(lastByte as u32))
        } else {
            0 as u32
        };
        RETURN_ERROR_IF!(lastByte as i32 == 0, ZSTD_error_GENERIC);
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as BitContainerType;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(6) as BitContainerType)
                            << (::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(16),
                    );
                current_block_32 = 4374783160472978455;
            }
            6 => {
                current_block_32 = 4374783160472978455;
            }
            5 => {
                current_block_32 = 508082133153388528;
            }
            4 => {
                current_block_32 = 11010276459331751112;
            }
            3 => {
                current_block_32 = 6819980346489144569;
            }
            2 => {
                current_block_32 = 8159937075156795181;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        match current_block_32 {
            4374783160472978455 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(5) as BitContainerType)
                            << (::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(24),
                    );
                current_block_32 = 508082133153388528;
            }
            _ => {}
        }
        match current_block_32 {
            508082133153388528 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(4) as BitContainerType)
                            << (::core::mem::size_of::<BitContainerType>()
                                as std::ffi::c_ulong)
                                .wrapping_mul(8)
                                .wrapping_sub(32),
                    );
                current_block_32 = 11010276459331751112;
            }
            _ => {}
        }
        match current_block_32 {
            11010276459331751112 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(3) as BitContainerType)
                            << 24,
                    );
                current_block_32 = 6819980346489144569;
            }
            _ => {}
        }
        match current_block_32 {
            6819980346489144569 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(2) as BitContainerType)
                            << 16,
                    );
                current_block_32 = 8159937075156795181;
            }
            _ => {}
        }
        match current_block_32 {
            8159937075156795181 => {
                (*bitD)
                    .bitContainer = ((*bitD).bitContainer)
                    .wrapping_add(
                        (*(srcBuffer as *const u8)
                            .offset(1) as BitContainerType)
                            << 8,
                    );
            }
            _ => {}
        }
        let lastByte_0 = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1) as isize);
        (*bitD)
            .bitsConsumed = if lastByte_0 as i32 != 0 {
            (8 as u32)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as u32))
        } else {
            0 as u32
        };
        RETURN_ERROR_IF!(lastByte_0 as i32 == 0, ZSTD_error_corruption_detected);
        (*bitD)
            .bitsConsumed = ((*bitD).bitsConsumed)
            .wrapping_add(
                (::core::mem::size_of::<BitContainerType>())
                    .wrapping_sub(srcSize) as u32 * 8,
            );
    }
    return srcSize;
}
#[inline(always)]
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    return bitContainer >> (start & regMask)
        & (1_u64 << nbBits)
            .wrapping_sub(1);
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    return BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<BitContainerType>())
            .wrapping_mul(8)
            .wrapping_sub((*bitD).bitsConsumed as std::ffi::c_ulong)
            .wrapping_sub(nbBits as std::ffi::c_ulong) as u32,
        nbBits,
    );
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask.wrapping_add(1).wrapping_sub(nbBits)
            & regMask);
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline(always)]
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline]
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: u32,
) -> usize {
    let value = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
#[inline]
unsafe extern "C" fn BIT_reloadDStream_internal(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    (*bitD)
        .ptr = ((*bitD).ptr)
        .offset(-(((*bitD).bitsConsumed >> 3) as isize));
    (*bitD).bitsConsumed &= 7;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    return BIT_DStream_unfinished;
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if ((*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<BitContainerType>())
            .wrapping_mul(8)) as i32
        as std::ffi::c_long != 0
    {
        static mut zeroFilled: BitContainerType = 0 as i32
            as BitContainerType;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as u32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD)
        .bitsConsumed = ((*bitD).bitsConsumed)
        .wrapping_sub(nbBytes * 8_u32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    return result;
}
#[inline]
unsafe extern "C" fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut ptr = dt as *const std::ffi::c_void;
    let DTableH = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog as u32);
    BIT_reloadDStream(bitD);
    (*DStatePtr)
        .table = dt.offset(1) as *const std::ffi::c_void;
}
#[inline]
unsafe extern "C" fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
    return symbol;
}
#[inline]
unsafe extern "C" fn FSE_decodeSymbolFast(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t)
        .offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
    return symbol;
}
pub const FSE_MAX_MEMORY_USAGE: i32 = 14;
pub const FSE_MAX_SYMBOL_VALUE: i32 = 255;
pub const FSE_MAX_TABLELOG: i32 = FSE_MAX_MEMORY_USAGE
    - 2;
unsafe extern "C" fn FSE_buildDTable_internal(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let tdPtr = dt.offset(1) as *mut std::ffi::c_void;
    let tableDecode = tdPtr as *mut FSE_decode_t;
    let mut symbolNext = workSpace as *mut u16;
    let mut spread = symbolNext
        .offset(maxSymbolValue as isize)
        .offset(1) as *mut u8;
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1 as i32) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    RETURN_ERROR_IF!(FSE_BUILD_DTABLE_WKSP_SIZE(tableLog, maxSymbolValue)
        > wkspSize as u64, ZSTD_error_maxSymbolValue_tooLarge);
    RETURN_ERROR_IF!(maxSymbolValue > FSE_MAX_SYMBOL_VALUE as u32, ZSTD_error_maxSymbolValue_tooLarge);
    RETURN_ERROR_IF!(tableLog > FSE_MAX_TABLELOG as u32, ZSTD_error_tableLog_tooLarge);
    let mut DTableH = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as u16;
    DTableH.fastMode = 1;
    let largeLimit = ((1 as i32)
        << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as i32
            == -(1 as i32)
        {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as u8;
            *symbolNext.offset(s as isize) = 1;
        } else {
            if *normalizedCounter.offset(s as isize) as i32
                >= largeLimit as i32
            {
                DTableH.fastMode = 0;
            }
            *symbolNext
                .offset(s as isize) = *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
        s;
    }
    libc::memcpy(
        dt as *mut std::ffi::c_void,
        &mut DTableH as *mut FSE_DTableHeader as *const std::ffi::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as usize,
    );
    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1) as usize;
        let step = FSE_TABLESTEP(tableSize);
        let add = 0x101010101010101 as u64 as u64;
        let mut pos: usize = 0;
        let mut sv: u64 = 0;
        let mut s_0: u32 = 0;
        s_0 = 0;
        while s_0 < maxSV1 {
            let mut i: i32 = 0;
            let n = *normalizedCounter.offset(s_0 as isize) as i32;
            MEM_write64(spread.offset(pos as isize) as *mut std::ffi::c_void, sv);
            i = 8;
            while i < n {
                MEM_write64(
                    spread.offset(pos as isize).offset(i as isize)
                        as *mut std::ffi::c_void,
                    sv,
                );
                i += 8;
            }
            pos = pos.wrapping_add(n as usize);
            s_0 = s_0.wrapping_add(1);
            s_0;
            sv = sv.wrapping_add(add);
        }
        let mut position: usize = 0;
        let mut s_1: usize = 0;
        let unroll = 2;
        s_1 = 0;
        while s_1 < tableSize as usize {
            let mut u: usize = 0;
            u = 0;
            while u < unroll {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                (*tableDecode.offset(uPosition as isize))
                    .symbol = *spread.offset(s_1.wrapping_add(u) as isize);
                u = u.wrapping_add(1);
                u;
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = FSE_TABLESTEP(tableSize);
        let mut s_2: u32 = 0;
        let mut position_0: u32 = 0;
        s_2 = 0;
        while s_2 < maxSV1 {
            let mut i_0: i32 = 0;
            i_0 = 0;
            while i_0 < *normalizedCounter.offset(s_2 as isize) as i32 {
                (*tableDecode.offset(position_0 as isize)).symbol = s_2 as u8;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
                i_0;
            }
            s_2 = s_2.wrapping_add(1);
            s_2;
        }
        RETURN_ERROR_IF!(position_0 != 0, ZSTD_error_GENERIC);
    }
    let mut u_0: u32 = 0;
    u_0 = 0;
    while u_0 < tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).symbol;
        let ref mut fresh1 = *symbolNext.offset(symbol as isize);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as u32;
        (*tableDecode.offset(u_0 as isize))
            .nbBits = tableLog.wrapping_sub(ZSTD_highbit32(nextState)) as u8;
        (*tableDecode.offset(u_0 as isize))
            .newState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as i32)
            .wrapping_sub(tableSize) as u16;
        u_0 = u_0.wrapping_add(1);
        u_0;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable_wksp(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const i16,
    mut maxSymbolValue: u32,
    mut tableLog: u32,
    mut workSpace: *mut std::ffi::c_void,
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
#[inline(always)]
unsafe extern "C" fn FSE_decompress_usingDTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut dt: *const FSE_DTable,
    fast: u32,
) -> usize {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-3_isize);
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
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);
    RETURN_ERROR_IF!(BIT_reloadDStream(&mut bitD) as u32
        == BIT_DStream_overflow as i32 as u32, ZSTD_error_corruption_detected);

    macro_rules! FSE_GETSYMBOL {
        ($statePtr:expr) => {
            if fast != 0 {
                FSE_decodeSymbolFast($statePtr, addr_of!(bitD))
            } else {
                FSE_decodeSymbol($statePtr, addr_of!(bitD))
            }
        }
    }

    while (BIT_reloadDStream(&mut bitD) as u32
        == BIT_DStream_unfinished as i32 as u32)
        as i32 & (op < olimit) as i32 != 0
    {
        *op.offset(0) = FSE_GETSYMBOL!(addr_of!(state1));
        if (FSE_MAX_TABLELOG * 2 as i32 + 7 as i32)
            as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(1) = FSE_GETSYMBOL!(addr_of!(state2));
        if (FSE_MAX_TABLELOG * 4 as i32 + 7 as i32)
            as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8)
        {
            if BIT_reloadDStream(&mut bitD) as u32
                > BIT_DStream_unfinished as i32 as u32
            {
                op = op.offset(2);
                break;
            }
        }
        *op.offset(2) = FSE_GETSYMBOL!(addr_of!(state1));
        if (FSE_MAX_TABLELOG * 2 as i32 + 7 as i32)
            as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>())
                .wrapping_mul(8)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(3) = FSE_GETSYMBOL!(addr_of!(state2));
        op = op.offset(4);
    }
    loop {
        RETURN_ERROR_IF!(op > omax.offset(-2_isize), ZSTD_error_dstSize_tooSmall);
        let fresh3 = op;
        op = op.offset(1);
        *fresh3 = FSE_GETSYMBOL!(addr_of!(state1));
        if BIT_reloadDStream(&mut bitD) as u32
            == BIT_DStream_overflow as i32 as u32
        {
            let fresh4 = op;
            op = op.offset(1);
            *fresh4 = FSE_GETSYMBOL!(addr_of!(state2));
            break;
        } else {
            RETURN_ERROR_IF!(op > omax.offset(-2_isize), ZSTD_error_dstSize_tooSmall);
            let fresh5 = op;
            op = op.offset(1);
            *fresh5 = FSE_GETSYMBOL!(addr_of!(state2));
            if !(BIT_reloadDStream(&mut bitD) as u32
                == BIT_DStream_overflow as i32 as u32)
            {
                continue;
            }
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = FSE_GETSYMBOL!(addr_of!(state1));
            break;
        }
    }
    return op.offset_from(ostart) as std::ffi::c_long as usize;
}
#[inline(always)]
unsafe extern "C" fn FSE_decompress_wksp_body(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut bmi2: i32,
) -> usize {
    let istart = cSrc as *const u8;
    let mut ip = istart;
    let mut tableLog: u32 = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as u32;
    let wksp = workSpace as *mut FSE_DecompressWksp;
    let dtablePos = (::core::mem::size_of::<FSE_DecompressWksp>())
        .wrapping_div(::core::mem::size_of::<FSE_DTable>());
    let dtable = (workSpace as *mut FSE_DTable).offset(dtablePos as isize);
    RETURN_ERROR_IF!(wkspSize < ::core::mem::size_of::<FSE_DecompressWksp>(), ZSTD_error_GENERIC);
    let NCountLength = FSE_readNCount_bmi2(
        ((*wksp).ncount).as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const std::ffi::c_void,
        cSrcSize,
        bmi2,
    );
    if ERR_isError(NCountLength) {
        return NCountLength;
    }
    RETURN_ERROR_IF!(tableLog > maxLog, ZSTD_error_tableLog_tooLarge);
    ip = ip.offset(NCountLength as isize);
    cSrcSize = cSrcSize.wrapping_sub(NCountLength);
    RETURN_ERROR_IF!(FSE_DECOMPRESS_WKSP_SIZE(tableLog, maxSymbolValue)
        > wkspSize as u64, ZSTD_error_tableLog_tooLarge);
    workSpace = (workSpace as *mut u8)
        .offset(
            ::core::mem::size_of::<FSE_DecompressWksp>() as isize,
        )
        .offset(FSE_DTABLE_SIZE(tableLog) as isize) as *mut std::ffi::c_void;
    wkspSize = (wkspSize as std::ffi::c_ulong)
        .wrapping_sub(
            (::core::mem::size_of::<FSE_DecompressWksp>())
                .wrapping_add(FSE_DTABLE_SIZE(tableLog)),
        ) as usize as usize;
    let _var_err__ = FSE_buildDTable_internal(
        dtable,
        ((*wksp).ncount).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    );
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    let mut ptr = dtable as *const std::ffi::c_void;
    let mut DTableH = ptr as *const FSE_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSE_decompress_usingDTable_generic(
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            cSrcSize,
            dtable,
            1,
        );
    }
    return FSE_decompress_usingDTable_generic(
        dst,
        dstCapacity,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dtable,
        0,
    );
}
unsafe extern "C" fn FSE_decompress_wksp_body_default(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut std::ffi::c_void,
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
        0,
    );
}
unsafe extern "C" fn FSE_decompress_wksp_body_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut std::ffi::c_void,
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
        1,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FSE_decompress_wksp_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: usize,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: usize,
    mut maxLog: u32,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: usize,
    mut bmi2: i32,
) -> usize {
    if bmi2 != 0 {
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
