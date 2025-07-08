use std::mem::size_of;

use crate::common::mem::*;
use crate::common::bitstream_h::*;

/*------   Version   ------*/
macro_rules! version {
    ($major:literal , $minor:literal , $release:literal) => {
        pub const FSE_VERSION_MAJOR: u32 = $major;
        pub const FSE_VERSION_MINOR: u32 = $minor;
        pub const FSE_VERSION_RELEASE: u32 = $release;

        pub const FSE_VERSION_STRING: &'static str = concat!($major, ".", $minor, ".", $release);
    }
}

version!(0, 9, 0);

pub const FSE_VERSION_NUMBER: u32 = FSE_VERSION_MAJOR *100*100 + FSE_VERSION_MINOR *100 + FSE_VERSION_RELEASE;

/** library version number; to be used when checking dll version */
pub fn FSE_versionNumber() -> u32 {
    FSE_VERSION_NUMBER
}


/*-*****************************************
*  Tool functions
******************************************/
pub use crate::compress::fse_compress::FSE_compressBound; /* maximum compressed size */

/* Error Management */
// FSE_PUBLIC_API unsigned    FSE_isError(size_t code);        /* tells if a return value is an error code */
// FSE_PUBLIC_API const char* FSE_getErrorName(size_t code);   /* provides error code string (useful for debugging) */


/*-*****************************************
*  FSE detailed API
******************************************/
/*/!
FSE_compress() does the following:
1. count symbol occurrence from source[] into table count[] (see hist.h)
2. normalize counters so that sum(count[]) == Power_of_2 (2^tableLog)
3. save normalized counters to memory buffer using writeNCount()
4. build encoding table 'CTable' from normalized counters
5. encode the data stream using encoding table 'CTable'

FSE_decompress() does the following:
1. read normalized counters with readNCount()
2. build decoding table 'DTable' from normalized counters
3. decode the data stream using decoding table 'DTable'

The following API allows targeting specific sub-functions for advanced tasks.
For example, it's possible to compress several blocks using the same 'CTable',
or to save and provide normalized distribution using external method.
*/

/* *** COMPRESSION *** */

/** FSE_optimalTableLog():
    dynamically downsize 'tableLog' when conditions are met.
    It saves CPU time, by using smaller tables, while preserving or even improving compression ratio.
    @return : recommended tableLog (necessarily <= 'maxTableLog') */
pub use crate::compress::fse_compress::FSE_optimalTableLog;

/** FSE_normalizeCount():
    normalize counts so that sum(count[]) == Power_of_2 (2^tableLog)
    'normalizedCounter' is a table of short, of minimum size (maxSymbolValue+1).
    useLowProbCount is a boolean parameter which trades off compressed size for
    faster header decoding. When it is set to 1, the compressed data will be slightly
    smaller. And when it is set to 0, FSE_readNCount() and FSE_buildDTable() will be
    faster. If you are compressing a small amount of data (< 2 KB) then useLowProbCount=0
    is a good default, since header deserialization makes a big speed difference.
    Otherwise, useLowProbCount=1 is a good default, since the speed difference is small.
    @return : tableLog,
              or an errorCode, which can be tested using FSE_isError() */
pub use crate::compress::fse_compress::FSE_normalizeCount;

/** FSE_NCountWriteBound():
    Provides the maximum possible size of an FSE normalized table, given 'maxSymbolValue' and 'tableLog'.
    Typically useful for allocation purpose. */
pub use crate::compress::fse_compress::FSE_NCountWriteBound;

/** FSE_writeNCount():
    Compactly save 'normalizedCounter' into 'buffer'.
    @return : size of the compressed table,
              or an errorCode, which can be tested using FSE_isError(). */
pub use crate::compress::fse_compress::FSE_writeNCount;

/* Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
pub type FSE_CTable = u32; /* don't allocate that. It's only meant to be more restrictive than void* */

// /*! FSE_buildCTable():
//     Builds `ct`, which must be already allocated, using FSE_createCTable().
//     @return : 0, or an errorCode, which can be tested using FSE_isError() */
// FSE_PUBLIC_API size_t FSE_buildCTable(FSE_CTable* ct, const short* normalizedCounter, unsigned maxSymbolValue, unsigned tableLog);

/** FSE_compress_usingCTable():
    Compress `src` using `ct` into `dst` which must be already allocated.
    @return : size of compressed data (<= `dstCapacity`),
              or 0 if compressed data could not fit into `dst`,
              or an errorCode, which can be tested using FSE_isError() */
pub use crate::compress::fse_compress::FSE_compress_usingCTable;

/*/!
Tutorial :
----------
The first step is to count all symbols. FSE_count() does this job very fast.
Result will be saved into 'count', a table of unsigned int, which must be already allocated, and have 'maxSymbolValuePtr[0]+1' cells.
'src' is a table of bytes of size 'srcSize'. All values within 'src' MUST be <= maxSymbolValuePtr[0]
maxSymbolValuePtr[0] will be updated, with its real value (necessarily <= original value)
FSE_count() will return the number of occurrence of the most frequent symbol.
This can be used to know if there is a single symbol within 'src', and to quickly evaluate its compressibility.
If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError()).

The next step is to normalize the frequencies.
FSE_normalizeCount() will ensure that sum of frequencies is == 2 ^'tableLog'.
It also guarantees a minimum of 1 to any Symbol with frequency >= 1.
You can use 'tableLog'==0 to mean "use default tableLog value".
If you are unsure of which tableLog value to use, you can ask FSE_optimalTableLog(),
which will provide the optimal valid tableLog given sourceSize, maxSymbolValue, and a user-defined maximum (0 means "default").

The result of FSE_normalizeCount() will be saved into a table,
called 'normalizedCounter', which is a table of signed short.
'normalizedCounter' must be already allocated, and have at least 'maxSymbolValue+1' cells.
The return value is tableLog if everything proceeded as expected.
It is 0 if there is a single symbol within distribution.
If there is an error (ex: invalid tableLog value), the function will return an ErrorCode (which can be tested using FSE_isError()).

'normalizedCounter' can be saved in a compact manner to a memory area using FSE_writeNCount().
'buffer' must be already allocated.
For guaranteed success, buffer size must be at least FSE_headerBound().
The result of the function is the number of bytes written into 'buffer'.
If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError(); ex : buffer size too small).

'normalizedCounter' can then be used to create the compression table 'CTable'.
The space required by 'CTable' must be already allocated, using FSE_createCTable().
You can then use FSE_buildCTable() to fill 'CTable'.
If there is an error, both functions will return an ErrorCode (which can be tested using FSE_isError()).

'CTable' can then be used to compress 'src', with FSE_compress_usingCTable().
Similar to FSE_count(), the convention is that 'src' is assumed to be a table of char of size 'srcSize'
The function returns the size of compressed data (without header), necessarily <= `dstCapacity`.
If it returns '0', compressed data could not fit into 'dst'.
If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError()).
*/


/* *** DECOMPRESSION *** */

/** FSE_readNCount():
    Read compactly saved 'normalizedCounter' from 'rBuffer'.
    @return : size read from 'rBuffer',
              or an errorCode, which can be tested using FSE_isError().
              maxSymbolValuePtr[0] and tableLogPtr[0] will also be updated with their respective values */
pub use crate::common::entropy_common::FSE_readNCount;

/** FSE_readNCount_bmi2():
 * Same as FSE_readNCount() but pass bmi2=1 when your CPU supports BMI2 and 0 otherwise.
 */
pub use crate::common::entropy_common::FSE_readNCount_bmi2;

pub type FSE_DTable = u32; /* don't allocate that. It's just a way to be more restrictive than void* */

/*/!
Tutorial :
----------
(Note : these functions only decompress FSE-compressed blocks.
 If block is uncompressed, use memcpy() instead
 If block is a single repeated byte, use memset() instead )

The first step is to obtain the normalized frequencies of symbols.
This can be performed by FSE_readNCount() if it was saved using FSE_writeNCount().
'normalizedCounter' must be already allocated, and have at least 'maxSymbolValuePtr[0]+1' cells of signed short.
In practice, that means it's necessary to know 'maxSymbolValue' beforehand,
or size the table to handle worst case situations (typically 256).
FSE_readNCount() will provide 'tableLog' and 'maxSymbolValue'.
The result of FSE_readNCount() is the number of bytes read from 'rBuffer'.
Note that 'rBufferSize' must be at least 4 bytes, even if useful information is less than that.
If there is an error, the function will return an error code, which can be tested using FSE_isError().

The next step is to build the decompression tables 'FSE_DTable' from 'normalizedCounter'.
This is performed by the function FSE_buildDTable().
The space required by 'FSE_DTable' must be already allocated using FSE_createDTable().
If there is an error, the function will return an error code, which can be tested using FSE_isError().

`FSE_DTable` can then be used to decompress `cSrc`, with FSE_decompress_usingDTable().
`cSrcSize` must be strictly correct, otherwise decompression will fail.
FSE_decompress_usingDTable() result will tell how many bytes were regenerated (<=`dstCapacity`).
If there is an error, the function will return an error code, which can be tested using FSE_isError(). (ex: dst buffer too small)
*/


/* *****************************************
*  Static allocation
*******************************************/
/* FSE buffer bounds */
pub const FSE_NCOUNTBOUND: usize = 512;
pub const fn FSE_BLOCKBOUND(size: usize) -> usize {
    size + (size >> 7) + 4 /* fse states */ + size_of::<usize>() /* bitContainer */
}
pub const fn FSE_COMPRESSBOUND(size: usize) -> usize {
    FSE_NCOUNTBOUND + FSE_BLOCKBOUND(size) /* Macro version, useful for static allocation */
}

/* It is possible to statically allocate FSE CTable/DTable as a table of FSE_CTable/FSE_DTable using below macros */
pub const fn FSE_CTABLE_SIZE_U32(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    1 + (1_usize << (maxTableLog - 1)) + (maxSymbolValue as usize + 1)*2
}
pub const fn FSE_DTABLE_SIZE_U32(maxTableLog: u32) -> usize {
    1 + (1_usize << maxTableLog)
}

/* or use the size to malloc() space directly. Pay attention to alignment restrictions though */
pub const fn FSE_DTABLE_SIZE(maxTableLog: u32) -> usize {
    FSE_DTABLE_SIZE_U32(maxTableLog) * size_of::<FSE_DTable>()
}
pub const fn FSE_CTABLE_SIZE(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    FSE_CTABLE_SIZE_U32(maxTableLog, maxSymbolValue) * size_of::<FSE_DTable>()
}

/* *****************************************
 *  FSE advanced API
 ***************************************** */

/// same as FSE_optimalTableLog(), which used `minus==2`
pub use crate::compress::fse_compress::FSE_optimalTableLog_internal;

/// build a fake FSE_CTable, designed to compress always the same symbolValue
pub use crate::compress::fse_compress::FSE_buildCTable_rle;


pub const fn FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue: u32, tableLog: u32) -> usize {
    ((maxSymbolValue as usize + 2) + (1_usize << tableLog))/2 +
    size_of::<u64>()/size_of::<u32>() // additional 8 bytes for potential table overwrite
}
pub const fn FSE_BUILD_CTABLE_WORKSPACE_SIZE(maxSymbolValue: u32, tableLog: u32) -> usize {
    size_of::<u32>() * FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue, tableLog)
}

/* FSE_buildCTable_wksp() :
 * Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
 * `wkspSize` must be >= `FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue, tableLog)` of `unsigned`.
 * See FSE_buildCTable_wksp() for breakdown of workspace usage.
 */
pub use crate::compress::fse_compress::FSE_buildCTable_wksp;

pub const fn FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    size_of::<i16>() * ((maxSymbolValue as usize) + 1) + (1_usize << maxTableLog) + 8
}
pub const fn FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    (FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog, maxSymbolValue) + size_of::<u32>() - 1) / size_of::<u32>()
}
/// Same as FSE_buildDTable(), using an externally allocated `workspace` produced with `FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxSymbolValue)`
pub use crate::common::fse_decompress::FSE_buildDTable_wksp;

pub const fn FSE_DECOMPRESS_WKSP_SIZE_U32(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    FSE_DTABLE_SIZE_U32(maxTableLog) + 1 + FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxTableLog, maxSymbolValue) + (FSE_MAX_SYMBOL_VALUE + 1) / 2 + 1
}
pub const fn FSE_DECOMPRESS_WKSP_SIZE(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    FSE_DECOMPRESS_WKSP_SIZE_U32(maxTableLog, maxSymbolValue) * size_of::<u32>()
}

/** same as FSE_decompress(), using an externally allocated `workSpace` produced with `FSE_DECOMPRESS_WKSP_SIZE_U32(maxLog, maxSymbolValue)`.
 * Set bmi2 to 1 if your CPU supports BMI2 or 0 if it doesn't */
 pub use crate::common::fse_decompress::FSE_decompress_wksp_bmi2;


pub type FSE_repeat = u32;
/// Cannot use the previous table
pub const FSE_repeat_none: FSE_repeat = 0;
/// Can use the previous table but it must be checked
pub const FSE_repeat_check: FSE_repeat = 1;
/// Can use the previous table and it is assumed to be valid
pub const FSE_repeat_valid: FSE_repeat = 2;

/* *****************************************
*  FSE symbol compression API
*******************************************/
/**
   This API consists of small unitary functions, which highly benefit from being inlined.
   Hence their body are included in next section.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: isize,
    pub stateTable: *const std::ffi::c_void,
    pub symbolTT: *const std::ffi::c_void,
    pub stateLog: u32,
}

// static void FSE_initCState(FSE_CState_t* CStatePtr, const FSE_CTable* ct);

// static void FSE_encodeSymbol(BIT_CStream_t* bitC, FSE_CState_t* CStatePtr, unsigned symbol);

// static void FSE_flushCState(BIT_CStream_t* bitC, const FSE_CState_t* CStatePtr);

/* *<
These functions are inner components of FSE_compress_usingCTable().
They allow the creation of custom streams, mixing multiple tables and bit sources.

A key property to keep in mind is that encoding and decoding are done **in reverse direction**.
So the first symbol you will encode is the last you will decode, like a LIFO stack.

You will need a few variables to track your CStream. They are :

FSE_CTable    ct;         // Provided by FSE_buildCTable()
BIT_CStream_t bitStream;  // bitStream tracking structure
FSE_CState_t  state;      // State tracking structure (can have several)


The first thing to do is to init bitStream and state.
    size_t errorCode = BIT_initCStream(&bitStream, dstBuffer, maxDstSize);
    FSE_initCState(&state, ct);

Note that BIT_initCStream() can produce an error code, so its result should be tested, using FSE_isError();
You can then encode your input data, byte after byte.
FSE_encodeSymbol() outputs a maximum of 'tableLog' bits at a time.
Remember decoding will be done in reverse direction.
    FSE_encodeByte(&bitStream, &state, symbol);

At any time, you can also add any bit sequence.
Note : maximum allowed nbBits is 25, for compatibility with 32-bits decoders
    BIT_addBits(&bitStream, bitField, nbBits);

The above methods don't commit data to memory, they just store it into local register, for speed.
Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (size_t).
Writing data to memory is a manual operation, performed by the flushBits function.
    BIT_flushBits(&bitStream);

Your last FSE encoding operation shall be to flush your last state value(s).
    FSE_flushState(&bitStream, &state);

Finally, you must close the bitStream.
The function returns the size of CStream in bytes.
If data couldn't fit into dstBuffer, it will return a 0 ( == not compressible)
If there is an error, it returns an errorCode (which can be tested using FSE_isError()).
    size_t size = BIT_closeCStream(&bitStream);
*/


/* *****************************************
*  FSE symbol decompression API
*******************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DState_t {
    pub state: usize,
    pub table: *const std::ffi::c_void, /* precise table may vary, depending on U16 */
}


// static void     FSE_initDState(FSE_DState_t* DStatePtr, BIT_DStream_t* bitD, const FSE_DTable* dt);

// static unsigned char FSE_decodeSymbol(FSE_DState_t* DStatePtr, BIT_DStream_t* bitD);

// static unsigned FSE_endOfDState(const FSE_DState_t* DStatePtr);

/* *<
Let's now decompose FSE_decompress_usingDTable() into its unitary components.
You will decode FSE-encoded symbols from the bitStream,
and also any other bitFields you put in, **in reverse order**.

You will need a few variables to track your bitStream. They are :

BIT_DStream_t DStream;    // Stream context
FSE_DState_t  DState;     // State context. Multiple ones are possible
FSE_DTable*   DTablePtr;  // Decoding table, provided by FSE_buildDTable()

The first thing to do is to init the bitStream.
    errorCode = BIT_initDStream(&DStream, srcBuffer, srcSize);

You should then retrieve your initial state(s)
(in reverse flushing order if you have several ones) :
    errorCode = FSE_initDState(&DState, &DStream, DTablePtr);

You can then decode your data, symbol after symbol.
For information the maximum number of bits read by FSE_decodeSymbol() is 'tableLog'.
Keep in mind that symbols are decoded in reverse order, like a LIFO stack (last in, first out).
    unsigned char symbol = FSE_decodeSymbol(&DState, &DStream);

You can retrieve any bitfield you eventually stored into the bitStream (in reverse order)
Note : maximum allowed nbBits is 25, for 32-bits compatibility
    size_t bitField = BIT_readBits(&DStream, nbBits);

All above operations only read from local register (which size depends on size_t).
Refueling the register from memory is manually performed by the reload method.
    endSignal = FSE_reloadDStream(&DStream);

BIT_reloadDStream() result tells if there is still some more data to read from DStream.
BIT_DStream_unfinished : there is still some data left into the DStream.
BIT_DStream_endOfBuffer : Dstream reached end of buffer. Its container may no longer be completely filled.
BIT_DStream_completed : Dstream reached its exact end, corresponding in general to decompression completed.
BIT_DStream_tooFar : Dstream went too far. Decompression result is corrupted.

When reaching end of buffer (BIT_DStream_endOfBuffer), progress slowly, notably if you decode multiple symbols per loop,
to properly detect the exact end of stream.
After each decoded symbol, check if DStream is fully consumed using this simple test :
    BIT_reloadDStream(&DStream) >= BIT_DStream_completed

When it's done, verify decompression is fully completed, by checking both DStream and the relevant states.
Checking if DStream has reached its end is performed by :
    BIT_endOfDStream(&DStream);
Check also the states. There might be some symbols left there, if some high probability ones (>50%) are possible.
    FSE_endOfDState(&DState);
*/


/* *****************************************
*  FSE unsafe API
*******************************************/
// static unsigned char FSE_decodeSymbolFast(FSE_DState_t* DStatePtr, BIT_DStream_t* bitD);
/* faster, but works only if nbBits is always >= 1 (otherwise, result will be corrupted) */


/* *****************************************
*  Implementation of inlined functions
*******************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: i32,
    pub deltaNbBits: u32,
} /* total 8 bytes */

#[inline]
pub unsafe fn FSE_initCState(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
) {
    let mut ptr = ct as *const std::ffi::c_void;
    let mut u16ptr = ptr as *const u16;
    let tableLog = MEM_read16(ptr) as u32;
    (*statePtr).value = 1_isize << tableLog;
    (*statePtr)
        .stateTable = u16ptr.offset(2)
        as *const std::ffi::c_void;
    (*statePtr)
        .symbolTT = ct
        .offset(1)
        .offset(
            if tableLog != 0 {
                1_isize << tableLog.wrapping_sub(1)
            } else {
                1
            },
        ) as *const std::ffi::c_void;
    (*statePtr).stateLog = tableLog;
}

/** FSE_initCState2() :
*   Same as FSE_initCState(), but the first symbol to include (which will be the last to be read)
*   uses the smallest state value possible, saving the cost of this symbol */
#[inline]
pub unsafe fn FSE_initCState2(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
    mut symbol: u32,
) {
    FSE_initCState(statePtr, ct);
    let symbolTT = *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform)
        .offset(symbol as isize);
    let mut stateTable = (*statePtr).stateTable as *const u16;
    let mut nbBitsOut = (symbolTT.deltaNbBits)
        .wrapping_add(1_u32 << 15)
        >> 16;
    (*statePtr)
        .value = (nbBitsOut << 16).wrapping_sub(symbolTT.deltaNbBits)
        as isize;
    (*statePtr)
        .value = *stateTable
        .offset(
            (((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as isize),
        ) as isize;
}


#[inline]
pub unsafe fn FSE_encodeSymbol(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *mut FSE_CState_t,
    mut symbol: u32,
) {
    let symbolTT = *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform)
        .offset(symbol as isize);
    let stateTable = (*statePtr).stateTable as *const u16;
    let nbBitsOut = ((*statePtr).value + symbolTT.deltaNbBits as isize >> 16) as u32;
    BIT_addBits(bitC, (*statePtr).value as BitContainerType, nbBitsOut);
    (*statePtr)
        .value = *stateTable
        .offset(
            ((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as isize
        ) as isize;
}

#[inline]
pub unsafe fn FSE_flushCState(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *const FSE_CState_t,
) {
    BIT_addBits(bitC, (*statePtr).value as BitContainerType, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}


// /* FSE_getMaxNbBits() :
//  * Approximate maximum cost of a symbol, in bits.
//  * Fractional get rounded up (i.e. a symbol with a normalized frequency of 3 gives the same result as a frequency of 2)
//  * note 1 : assume symbolValue is valid (<= maxSymbolValue)
//  * note 2 : if freq[symbolValue]==0, @return a fake cost of tableLog+1 bits */
// MEM_STATIC U32 FSE_getMaxNbBits(const void* symbolTTPtr, U32 symbolValue)
// {
//     const FSE_symbolCompressionTransform* symbolTT = (const FSE_symbolCompressionTransform*) symbolTTPtr;
//     return (symbolTT[symbolValue].deltaNbBits + ((1<<16)-1)) >> 16;
// }

/** FSE_bitCost() :
 * Approximate symbol cost, as fractional value, using fixed-point format (accuracyLog fractional bits)
 * note 1 : assume symbolValue is valid (<= maxSymbolValue)
 * note 2 : if freq[symbolValue]==0, @return a fake cost of tableLog+1 bits */
#[inline]
pub unsafe extern "C" fn FSE_bitCost(
    mut symbolTTPtr: *const std::ffi::c_void,
    mut tableLog: u32,
    mut symbolValue: u32,
    mut accuracyLog: u32,
) -> u32 {
    let mut symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits = (*symbolTT.offset(symbolValue as isize)).deltaNbBits
        >> 16;
    let threshold = minNbBits.wrapping_add(1)
        << 16;
    debug_assert!(tableLog < 16);
    debug_assert!(accuracyLog < 31-tableLog);  /* ensure enough room for renormalization double shift */
    let tableSize = 1_u32 << tableLog;
    let deltaFromThreshold = threshold
        .wrapping_sub(
            ((*symbolTT.offset(symbolValue as isize)).deltaNbBits)
                .wrapping_add(tableSize),
        );
    let normalizedDeltaFromThreshold = (deltaFromThreshold << accuracyLog) >> tableLog;
    let bitMultiplier = 1_u32 << accuracyLog;
    debug_assert!((*symbolTT.offset(symbolValue as isize)).deltaNbBits + tableSize <= threshold);
    debug_assert!(normalizedDeltaFromThreshold <= bitMultiplier);
    return (minNbBits.wrapping_add(1) * bitMultiplier)
        .wrapping_sub(normalizedDeltaFromThreshold);
}


/* ======    Decompression    ====== */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
} /* sizeof U32 */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: u16,
    pub symbol: std::ffi::c_uchar,
    pub nbBits: std::ffi::c_uchar,
}  /* size == U32 */

#[inline]
pub unsafe fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut ptr = dt as *const std::ffi::c_void;
    let DTableH = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog as u32);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1) as *const std::ffi::c_void;
}

// MEM_STATIC BYTE FSE_peekSymbol(const FSE_DState_t* DStatePtr)
// {
//     FSE_decode_t const DInfo = ((const FSE_decode_t*)(DStatePtr->table))[DStatePtr->state];
//     return DInfo.symbol;
// }

// MEM_STATIC void FSE_updateState(FSE_DState_t* DStatePtr, BIT_DStream_t* bitD)
// {
//     FSE_decode_t const DInfo = ((const FSE_decode_t*)(DStatePtr->table))[DStatePtr->state];
//     U32 const nbBits = DInfo.nbBits;
//     size_t const lowBits = BIT_readBits(bitD, nbBits);
//     DStatePtr->state = DInfo.newState + lowBits;
// }

#[inline]
pub unsafe fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBits(bitD, nbBits);

    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
    return symbol;
}

/** FSE_decodeSymbolFast() :
    unsafe, only works if no symbol has a probability > 50% */
#[inline]
pub unsafe fn FSE_decodeSymbolFast(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBitsFast(bitD, nbBits);

    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
    return symbol;
}

// MEM_STATIC unsigned FSE_endOfDState(const FSE_DState_t* DStatePtr)
// {
//     return DStatePtr->state == 0;
// }



// TODO #ifndef FSE_COMMONDEFS_ONLY ?

/* **************************************************************
*  Tuning parameters
****************************************************************/
/* !MEMORY_USAGE :
*  Memory usage formula : N->2^N Bytes (examples : 10 -> 1KB; 12 -> 4KB ; 16 -> 64KB; 20 -> 1MB; etc.)
*  Increasing memory usage improves compression ratio
*  Reduced memory usage can improve speed, due to cache effect
*  Recommended max value is 14, for 16KB, which nicely fits into Intel x86 L1 cache */
// TODO #ifndef FSE_MAX_MEMORY_USAGE
pub const FSE_MAX_MEMORY_USAGE: u32 = 14;
// TODO #ifndef FSE_DEFAULT_MEMORY_USAGE
pub const FSE_DEFAULT_MEMORY_USAGE: u32 = 13;
const _: () = assert!(FSE_DEFAULT_MEMORY_USAGE <= FSE_MAX_MEMORY_USAGE,
    "FSE_DEFAULT_MEMORY_USAGE must be <= FSE_MAX_MEMORY_USAGE");

/**FSE_MAX_SYMBOL_VALUE :
*  Maximum symbol value authorized.
*  Required for proper stack allocation */
pub const FSE_MAX_SYMBOL_VALUE: usize = 255; // TODO configurable?

/* **************************************************************
*  template functions type & suffix
****************************************************************/
// #define FSE_FUNCTION_TYPE BYTE
// #define FSE_FUNCTION_EXTENSION
// #define FSE_DECODE_TYPE FSE_decode_t


/* ***************************************************************
*  Constants
*****************************************************************/
pub const FSE_MAX_TABLELOG: u32 = FSE_MAX_MEMORY_USAGE - 2;
pub const FSE_MAX_TABLESIZE: usize = 1_usize << FSE_MAX_TABLELOG;
pub const FSE_MAXTABLESIZE_MASK: usize = FSE_MAX_TABLESIZE - 1;
pub const FSE_DEFAULT_TABLELOG: u32 = FSE_DEFAULT_MEMORY_USAGE-2;
pub const FSE_MIN_TABLELOG: u32 = 5;

pub const FSE_TABLELOG_ABSOLUTE_MAX: u32 = 15;
const _: () = assert!(FSE_MAX_TABLELOG <= FSE_TABLELOG_ABSOLUTE_MAX,
    "FSE_MAX_TABLELOG > FSE_TABLELOG_ABSOLUTE_MAX is not supported");

pub const fn FSE_TABLESTEP(tableSize: u32) -> u32 {
    (tableSize >> 1) + ((tableSize >> 3) + 3)
}
// pub const fn FSE_TABLESTEP32(tableSize: u32) -> u32 {
//     (tableSize >> 1) + ((tableSize >> 3) + 3)
// }
