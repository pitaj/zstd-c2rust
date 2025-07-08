use core::mem::size_of;

use crate::zstd_h::*;
use crate::common::mem::*;
use crate::common::error::*;
use crate::common::bits::*;

/*
*  This API consists of small unitary functions, which must be inlined for best performance.
*  Since link-time-optimization is not available for all compilers,
*  these functions are defined into a .h to be included.
*/


/*=========================================
*  Target specific
=========================================*/

pub const STREAM_ACCUMULATOR_MIN_32: u32 = 25;
pub const STREAM_ACCUMULATOR_MIN_64: u32 = 57;
pub const STREAM_ACCUMULATOR_MIN: u32 = if MEM_32bits {
    STREAM_ACCUMULATOR_MIN_32
} else {
    STREAM_ACCUMULATOR_MIN_64
};


/*-******************************************
*  bitStream encoding API (write forward)
********************************************/
pub type BitContainerType = usize;
/* bitStream can mix input from multiple sources.
 * A critical property of these streams is that they encode and decode in **reverse** direction.
 * So the first bit sequence you add will be the last to be read, like a LIFO stack.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: BitContainerType,
    pub bitPos: u32,
    pub startPtr: *mut std::ffi::c_char,
    pub ptr: *mut std::ffi::c_char,
    pub endPtr: *mut std::ffi::c_char,
}

// MEM_STATIC size_t BIT_initCStream(BIT_CStream_t* bitC, void* dstBuffer, size_t dstCapacity);
// MEM_STATIC void   BIT_addBits(BIT_CStream_t* bitC, BitContainerType value, unsigned nbBits);
// MEM_STATIC void   BIT_flushBits(BIT_CStream_t* bitC);
// MEM_STATIC size_t BIT_closeCStream(BIT_CStream_t* bitC);

/* Start with initCStream, providing the size of buffer to write into.
*  bitStream will never write outside of this buffer.
*  `dstCapacity` must be >= sizeof(bitD->bitContainer), otherwise @return will be an error code.
*
*  bits are first added to a local register.
*  Local register is BitContainerType, 64-bits on 64-bits systems, or 32-bits on 32-bits systems.
*  Writing data into memory is an explicit operation, performed by the flushBits function.
*  Hence keep track how many bits are potentially stored into local register to avoid register overflow.
*  After a flushBits, a maximum of 7 bits might still be stored into local register.
*
*  Avoid storing elements of more than 24 bits if you want compatibility with 32-bits bitstream readers.
*
*  Last operation is to close the bitStream.
*  The function returns the final size of CStream in bytes.
*  If data couldn't fit into `dstBuffer`, it will return a 0 ( == not storable)
*/


/*-********************************************
*  bitStream decoding API (read backward)
**********************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: BitContainerType,
    pub bitsConsumed: u32,
    pub ptr: *const std::ffi::c_char,
    pub start: *const std::ffi::c_char,
    pub limitPtr: *const std::ffi::c_char,
}

pub type BIT_DStream_status = u32; /* result of BIT_reloadDStream() */
pub const BIT_DStream_unfinished: BIT_DStream_status = 0; /* fully refilled */
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1; /* still some bits left in bitstream */
pub const BIT_DStream_completed: BIT_DStream_status = 2; /* bitstream entirely consumed, bit-exact */
pub const BIT_DStream_overflow: BIT_DStream_status = 3; /* user requested more bits than present in bitstream */


/* Start by invoking BIT_initDStream().
*  A chunk of the bitStream is then stored into a local register.
*  Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (BitContainerType).
*  You can then retrieve bitFields stored into the local register, **in reverse order**.
*  Local register is explicitly reloaded from memory by the BIT_reloadDStream() method.
*  A reload guarantee a minimum of ((8*sizeof(bitD->bitContainer))-7) bits when its result is BIT_DStream_unfinished.
*  Otherwise, it can be less than that, so proceed accordingly.
*  Checking if DStream has reached its end can be performed with BIT_endOfDStream().
*/


/*-****************************************
*  unsafe API
******************************************/
// MEM_STATIC void BIT_addBitsFast(BIT_CStream_t* bitC, BitContainerType value, unsigned nbBits);
// /* faster, but works only if value is "clean", meaning all high bits above nbBits are 0 */

// MEM_STATIC void BIT_flushBitsFast(BIT_CStream_t* bitC);
// /* unsafe version; does not check buffer overflow */

// MEM_STATIC size_t BIT_readBitsFast(BIT_DStream_t* bitD, unsigned nbBits);
// /* faster, but works only if nbBits >= 1 */

/*=====    Local Constants   =====*/
pub const BIT_MASK_SIZE: usize = 32;
pub const BIT_mask: [u32; BIT_MASK_SIZE] = [
    0,          1,         3,         7,         0xF,       0x1F,
    0x3F,       0x7F,      0xFF,      0x1FF,     0x3FF,     0x7FF,
    0xFFF,      0x1FFF,    0x3FFF,    0x7FFF,    0xFFFF,    0x1FFFF,
    0x3FFFF,    0x7FFFF,   0xFFFFF,   0x1FFFFF,  0x3FFFFF,  0x7FFFFF,
    0xFFFFFF,   0x1FFFFFF, 0x3FFFFFF, 0x7FFFFFF, 0xFFFFFFF, 0x1FFFFFFF,
    0x3FFFFFFF, 0x7FFFFFFF ]; /* up to 31 bits */

/*-**************************************************************
*  bitStream encoding
****************************************************************/
/** BIT_initCStream() :
 *  `dstCapacity` must be > sizeof(size_t)
 *  @return : 0 if success,
 *            otherwise an error code (can be tested using ERR_isError()) */
#[inline]
pub unsafe fn BIT_initCStream(
    mut bitC: *mut BIT_CStream_t,
    mut startPtr: *mut std::ffi::c_void,
    mut dstCapacity: usize,
) -> usize {
    (*bitC).bitContainer = 0;
    (*bitC).bitPos = 0;
    (*bitC).startPtr = startPtr as *mut std::ffi::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC)
        .endPtr = ((*bitC).startPtr)
        .add(dstCapacity)
        .sub(size_of::<BitContainerType>());
    RETURN_ERROR_IF!(dstCapacity <= size_of::<BitContainerType>(), ZSTD_error_dstSize_tooSmall);
    return 0;
}

// FORCE_INLINE_TEMPLATE BitContainerType BIT_getLowerBits(BitContainerType bitContainer, U32 const nbBits)
// {
// #if STATIC_BMI2 && !defined(ZSTD_NO_INTRINSICS)
// #  if (defined(__x86_64__) || defined(_M_X64)) && !defined(__ILP32__)
//     return _bzhi_u64(bitContainer, nbBits);
// #  else
//     DEBUG_STATIC_ASSERT(sizeof(bitContainer) == sizeof(U32));
//     return _bzhi_u32(bitContainer, nbBits);
// #  endif
// #else
//     assert(nbBits < BIT_MASK_SIZE);
//     return bitContainer & BIT_mask[nbBits];
// #endif
// }
#[inline(always)]
pub unsafe fn BIT_getLowerBits(
    mut bitContainer: BitContainerType,
    nbBits: u32,
) -> BitContainerType {
    // TODO #if STATIC_BMI2 && !defined(ZSTD_NO_INTRINSICS)
    // #  if (defined(__x86_64__) || defined(_M_X64)) && !defined(__ILP32__)
    //     return _bzhi_u64(bitContainer, nbBits);
    // #  else
    //     DEBUG_STATIC_ASSERT(sizeof(bitContainer) == sizeof(U32));
    //     return _bzhi_u32(bitContainer, nbBits);
    // #  endif
    // #else
    debug_assert!((nbBits as usize) < BIT_MASK_SIZE);
    return bitContainer & BIT_mask[nbBits as usize] as BitContainerType;
}

/** BIT_addBits() :
 *  can add up to 31 bits into `bitC`.
 *  Note : does not check for register overflow ! */
#[inline]
pub unsafe fn BIT_addBits(
    mut bitC: *mut BIT_CStream_t,
    mut value: BitContainerType,
    mut nbBits: u32,
) {
    const _: () = assert!(BIT_MASK_SIZE == 32);
    debug_assert!((nbBits as usize) < BIT_MASK_SIZE);
    debug_assert!(((nbBits + (*bitC).bitPos) as usize) < std::mem::size_of_val(&(*bitC).bitContainer) * 8);
    (*bitC).bitContainer |= BIT_getLowerBits(value, nbBits) << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}

/** BIT_addBitsFast() :
 *  works only if `value` is _clean_,
 *  meaning all high bits above nbBits are 0 */
#[inline]
pub unsafe fn BIT_addBitsFast(
    mut bitC: *mut BIT_CStream_t,
    mut value: BitContainerType,
    mut nbBits: u32,
) {
    debug_assert!((value >> nbBits) == 0);
    debug_assert!(((nbBits + (*bitC).bitPos) as usize) < std::mem::size_of_val(&(*bitC).bitContainer) * 8);
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}

/** BIT_flushBitsFast() :
 *  assumption : bitContainer has not overflowed
 *  unsafe version; does not check buffer overflow */
#[inline]
pub unsafe fn BIT_flushBitsFast(mut bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3) as usize;
    debug_assert!(((*bitC).bitPos as usize) < std::mem::size_of_val(&(*bitC).bitContainer) * 8);
    debug_assert!((*bitC).ptr <= (*bitC).endPtr);
    MEM_writeLEST((*bitC).ptr as *mut std::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    (*bitC).bitPos &= 7;
    (*bitC).bitContainer >>= nbBytes * 8;
}

/** BIT_flushBits() :
 *  assumption : bitContainer has not overflowed
 *  safe version; check for buffer overflow, and prevents it.
 *  note : does not signal buffer overflow.
 *  overflow will be revealed later on using BIT_closeCStream() */
#[inline]
pub unsafe fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3) as usize;
    debug_assert!(((*bitC).bitPos as usize) < std::mem::size_of_val(&(*bitC).bitContainer) * 8);
    debug_assert!((*bitC).ptr <= (*bitC).endPtr);
    MEM_writeLEST((*bitC).ptr as *mut std::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    if (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
    (*bitC).bitPos &= 7;
    (*bitC).bitContainer >>= nbBytes * 8;
}

/** BIT_closeCStream() :
 *  @return : size of CStream, in bytes,
 *            or 0 if it could not fit into dstBuffer */
#[inline]
pub unsafe fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t) -> usize {
    BIT_addBitsFast(bitC, 1, 1); /* endMark */
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0; /* overflow detected */
    }
    return (((*bitC).ptr).offset_from((*bitC).startPtr) as usize)
        .wrapping_add(
            ((*bitC).bitPos > 0) as usize,
        );
}


/*-********************************************************
*  bitStream decoding
**********************************************************/
/** BIT_initDStream() :
 *  Initialize a BIT_DStream_t.
 * `bitD` : a pointer to an already allocated BIT_DStream_t structure.
 * `srcSize` must be the *exact* size of the bitStream, in bytes.
 * @return : size of stream (== srcSize), or an errorCode if a problem is detected
 */
#[inline]
pub unsafe fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: usize,
) -> usize {
    if srcSize < 1 {
        libc::memset(
            bitD as *mut std::ffi::c_void,
            0,
            size_of::<BIT_DStream_t>(),
        );
        return ERROR(ZSTD_error_srcSize_wrong);
    }

    (*bitD).start = srcBuffer as *const std::ffi::c_char;
    (*bitD).limitPtr = ((*bitD).start).add(size_of::<BitContainerType>());

    if srcSize >= size_of::<BitContainerType>() { /* normal case */
        (*bitD).ptr = (srcBuffer as *const std::ffi::c_char)
            .add(srcSize)
            .sub(size_of::<BitContainerType>());
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        let lastByte = *(srcBuffer as *const u8)
            .add(srcSize - 1);
        (*bitD).bitsConsumed = if lastByte != 0 {
            8 - ZSTD_highbit32(lastByte as u32)
        } else {
            0
        }; /* ensures bitsConsumed is always set */
        if lastByte == 0 {
            return ERROR(ZSTD_error_GENERIC); /* endMark not present */
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as BitContainerType;
        
        if srcSize <= 7 {
            if srcSize == 7 {
                (*bitD).bitContainer += (*(srcBuffer as *const u8).offset(6) as BitContainerType)
                        << (size_of::<BitContainerType>()*8 - 16);
            }
            if srcSize >= 6 {
                (*bitD).bitContainer += (*(srcBuffer as *const u8).offset(5) as BitContainerType)
                        << (size_of::<BitContainerType>()*8 - 24);
            }
            if srcSize >= 5 {
                (*bitD).bitContainer += (*(srcBuffer as *const u8).offset(4) as BitContainerType)
                        << (size_of::<BitContainerType>()*8 - 32);
            }
            if srcSize >= 4 {
                (*bitD).bitContainer += (*(srcBuffer as *const u8).offset(3) as BitContainerType)
                        << 24;
            }
            if srcSize >= 3 {
                (*bitD).bitContainer += (*(srcBuffer as *const u8).offset(2) as BitContainerType)
                        << 16;
            }
            if srcSize >= 2 {
                (*bitD).bitContainer += (*(srcBuffer as *const u8).offset(1) as BitContainerType)
                        << 8;
            }
        }
        
        let lastByte_0 = *(srcBuffer as *const u8).add(srcSize - 1);
        (*bitD).bitsConsumed = if lastByte_0 != 0 {
            8 - ZSTD_highbit32(lastByte_0 as u32)
        } else {
            0
        };
        if lastByte_0 == 0 { return ERROR(ZSTD_error_corruption_detected); } /* endMark not present */
        (*bitD).bitsConsumed += ((size_of::<BitContainerType>() - srcSize) * 8) as u32;
    }

    return srcSize;
}

#[inline(always)]
pub const fn BIT_getUpperBits(bitContainer: BitContainerType, start: u32) -> BitContainerType {
    bitContainer >> start
}

#[inline(always)]
pub const fn BIT_getMiddleBits(
    bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    let regMask = size_of::<BitContainerType>().wrapping_mul(8).wrapping_sub(1) as u32;
    /* if start > regMask, bitstream is corrupted, and result is undefined */
    debug_assert!((nbBits as usize) < BIT_MASK_SIZE);
    /* x86 transform & ((1 << nbBits) - 1) to bzhi instruction, it is better
     * than accessing memory. When bmi2 instruction is not present, we consider
     * such cpus old (pre-Haswell, 2013) and their performance is not of that
     * importance.
     */
    #[cfg(target_arch = "x86_64")]
    {
        (bitContainer >> (start & regMask)) & ((1 as BitContainerType) << nbBits).wrapping_sub(1)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        (bitContainer >> (start & regMask)) & BIT_mask[nbBits as usize]
    }
}

/** BIT_lookBits() :
 *  Provides next n bits from local register.
 *  local register is not modified.
 *  On 32-bits, maxNbBits==24.
 *  On 64-bits, maxNbBits==56.
 * @return : value extracted */
#[inline(always)]
pub unsafe fn BIT_lookBits(
    bitD: *const BIT_DStream_t,
    nbBits: u32,
) -> BitContainerType {
    /* if bitD->bitsConsumed + nbBits > sizeof(bitD->bitContainer)*8,
     * bitstream is likely corrupted, and result is undefined */
    BIT_getMiddleBits(
        (*bitD).bitContainer, 
        (size_of::<BitContainerType>() as u32)
            .wrapping_mul(8)
            .wrapping_sub((*bitD).bitsConsumed)
            .wrapping_sub(nbBits),
        nbBits,
    )

    // TODO? #else
    //     /* this code path is slower on my os-x laptop */
    //     U32 const regMask = sizeof(bitD->bitContainer)*8 - 1;
    //     return ((bitD->bitContainer << (bitD->bitsConsumed & regMask)) >> 1) >> ((regMask-nbBits) & regMask);
    // #endif
}

/** BIT_lookBitsFast() :
 *  unsafe version; only works if nbBits >= 1 */
#[inline]
pub unsafe fn BIT_lookBitsFast(
    bitD: *const BIT_DStream_t,
    nbBits: u32,
) -> BitContainerType {
    let regMask = (size_of::<BitContainerType>() as u32)
        .wrapping_mul(8)
        .wrapping_sub(1);
    ((*bitD).bitContainer << ((*bitD).bitsConsumed & regMask))
        >> (regMask.wrapping_add(1).wrapping_sub(nbBits) & regMask)
}

#[inline(always)]
pub unsafe fn BIT_skipBits(bitD: *mut BIT_DStream_t, nbBits: u32) {
    (*bitD).bitsConsumed += nbBits;
}

/** BIT_readBits() :
 *  Read (consume) next n bits from local register and update.
 *  Pay attention to not read more than nbBits contained into local register.
 * @return : extracted value. */
#[inline(always)]
pub unsafe fn BIT_readBits(
    bitD: *mut BIT_DStream_t,
    nbBits: u32,
) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}

/** BIT_readBitsFast() :
 *  unsafe version; only works if nbBits >= 1 */
#[inline]
pub unsafe fn BIT_readBitsFast(
    bitD: *mut BIT_DStream_t,
    nbBits: u32,
) -> usize {
    let value = BIT_lookBitsFast(bitD, nbBits);
    debug_assert!(nbBits >= 1);
    BIT_skipBits(bitD, nbBits);
    return value;
}

/** BIT_reloadDStream_internal() :
 *  Simple variant of BIT_reloadDStream(), with two conditions:
 *  1. bitstream is valid : bitsConsumed <= sizeof(bitD->bitContainer)*8
 *  2. look window is valid after shifted down : bitD->ptr >= bitD->start
 */
#[inline]
pub unsafe fn BIT_reloadDStream_internal(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    debug_assert!(((*bitD).bitsConsumed as usize) <= size_of::<BitContainerType>()*8);
    (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3) as isize));
    debug_assert!((*bitD).ptr >= (*bitD).start);
    (*bitD).bitsConsumed &= 7;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    return BIT_DStream_unfinished;
}

/** BIT_reloadDStreamFast() :
 *  Similar to BIT_reloadDStream(), but with two differences:
 *  1. bitsConsumed <= sizeof(bitD->bitContainer)*8 must hold!
 *  2. Returns BIT_DStream_overflow when bitD->ptr < bitD->limitPtr, at this
 *     point you must use BIT_reloadDStream() to reload.
 */
#[inline]
pub unsafe fn BIT_reloadDStreamFast(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    if UNLIKELY!((*bitD).ptr < (*bitD).limitPtr) {
        return BIT_DStream_overflow;
    }
    return BIT_reloadDStream_internal(bitD);
}

/** BIT_reloadDStream() :
 *  Refill `bitD` from buffer previously set in BIT_initDStream() .
 *  This function is safe, it guarantees it will not never beyond src buffer.
 * @return : status of `BIT_DStream_t` internal register.
 *           when status == BIT_DStream_unfinished, internal register is filled with at least 25 or 57 bits */
#[inline(always)]
pub unsafe fn BIT_reloadDStream(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    /* note : once in overflow mode, a bitstream remains in this mode until it's reset */
    if UNLIKELY!((*bitD).bitsConsumed as usize > (size_of::<BitContainerType>() * 8)) {
        const zeroFilled: BitContainerType = 0;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char; /* aliasing is allowed for char */
        /* overflow detected, erroneous scenario or end of stream: no update */
        return BIT_DStream_overflow;
    }

    debug_assert!((*bitD).ptr >= (*bitD).start);

    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        /* reached end of bitStream => no update */
        if ((*bitD).bitsConsumed as usize) < (size_of::<BitContainerType>() * 8) {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    /* start < ptr < limitPtr => cautious update */
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as u32; /* ptr > start */
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed -= nbBytes * 8;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void); /* reminder : srcSize > sizeof(bitD->bitContainer), otherwise bitD->ptr == bitD->start */
    return result;
}

/** BIT_endOfDStream() :
 * @return : 1 if DStream has _exactly_ reached its end (all bits consumed).
 */
#[inline]
pub unsafe fn BIT_endOfDStream(
    mut DStream: *const BIT_DStream_t,
) -> bool {
    (*DStream).ptr == (*DStream).start
        && ((*DStream).bitsConsumed as usize) == (size_of::<BitContainerType>() * 8)
}
