use ::libc;
extern "C" {
    fn ZSTD_loadDEntropy(
        entropy: *mut ZSTD_entropyDTables_t,
        dict: *const std::ffi::c_void,
        dictSize: usize,
    ) -> usize;
}
use crate::common::error::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [u32; 640],
    pub previousDstEnd: *const std::ffi::c_void,
    pub prefixStart: *const std::ffi::c_void,
    pub virtualStart: *const std::ffi::c_void,
    pub dictEnd: *const std::ffi::c_void,
    pub expected: usize,
    pub fParams: ZSTD_FrameHeader,
    pub processedCSize: u64,
    pub decodedSize: u64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: usize,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTD_customMem,
    pub litSize: usize,
    pub rleSize: usize,
    pub staticSize: usize,
    pub isFrameDecompression: i32,
    pub bmi2: i32,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: u32,
    pub ddictIsCold: i32,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: i32,
    pub maxBlockSizeParam: i32,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: usize,
    pub inPos: usize,
    pub maxWindowSize: usize,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: usize,
    pub outStart: usize,
    pub outEnd: usize,
    pub lhSize: usize,
    pub legacyContext: *mut std::ffi::c_void,
    pub previousLegacyVersion: u32,
    pub legacyVersion: u32,
    pub hostageByte: u32,
    pub noForwardProgress: i32,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut u8,
    pub litBufferEnd: *const u8,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [u8; 65568],
    pub headerBuffer: [u8; 18],
    pub oversizedDuration: usize,
    pub traceCtx: ZSTD_TraceCtx,
}
pub type ZSTD_TraceCtx = u64;
pub type ZSTD_litLocation_e = u32;
pub const ZSTD_split: ZSTD_litLocation_e = 2;
pub const ZSTD_in_dst: ZSTD_litLocation_e = 1;
pub const ZSTD_not_in_dst: ZSTD_litLocation_e = 0;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut std::ffi::c_void,
    pub size: usize,
    pub pos: usize,
}
pub type ZSTD_bufferMode_e = u32;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
pub type ZSTD_dStreamStage = u32;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const zdss_read: ZSTD_dStreamStage = 2;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const zdss_init: ZSTD_dStreamStage = 0;
pub type ZSTD_refMultipleDDicts_e = u32;
pub const ZSTD_rmd_refMultipleDDicts: ZSTD_refMultipleDDicts_e = 1;
pub const ZSTD_rmd_refSingleDDict: ZSTD_refMultipleDDicts_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDictHashSet {
    pub ddictPtrTable: *mut *const ZSTD_DDict,
    pub ddictPtrTableSize: usize,
    pub ddictPtrCount: usize,
}
pub type ZSTD_DDict = ZSTD_DDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDict_s {
    pub dictBuffer: *mut std::ffi::c_void,
    pub dictContent: *const std::ffi::c_void,
    pub dictSize: usize,
    pub entropy: ZSTD_entropyDTables_t,
    pub dictID: u32,
    pub entropyPresent: u32,
    pub cMem: ZSTD_customMem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut std::ffi::c_void,
}
pub type ZSTD_freeFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
>;
pub type ZSTD_allocFunction = Option::<
    unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; 513],
    pub OFTable: [ZSTD_seqSymbol; 257],
    pub MLTable: [ZSTD_seqSymbol; 513],
    pub hufTable: [HUF_DTable; 4097],
    pub rep: [u32; 3],
    pub workspace: [u32; 157],
}
pub type HUF_DTable = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol {
    pub nextState: u16,
    pub nbAdditionalBits: u8,
    pub nbBits: u8,
    pub baseValue: u32,
}
pub type ZSTD_dictUses_e = i32;
pub const ZSTD_use_once: ZSTD_dictUses_e = 1;
pub const ZSTD_dont_use: ZSTD_dictUses_e = 0;
pub const ZSTD_use_indefinitely: ZSTD_dictUses_e = -1;
pub type ZSTD_forceIgnoreChecksum_e = u32;
pub const ZSTD_d_ignoreChecksum: ZSTD_forceIgnoreChecksum_e = 1;
pub const ZSTD_d_validateChecksum: ZSTD_forceIgnoreChecksum_e = 0;
pub type ZSTD_format_e = u32;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
pub type ZSTD_dStage = u32;
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;
pub type blockType_e = u32;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_FrameHeader {
    pub frameContentSize: u64,
    pub windowSize: u64,
    pub blockSizeMax: u32,
    pub frameType: ZSTD_FrameType_e,
    pub headerSize: u32,
    pub dictID: u32,
    pub checksumFlag: u32,
    pub _reserved1: u32,
    pub _reserved2: u32,
}
pub type ZSTD_FrameType_e = u32;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type ZSTD_dictContentType_e = u32;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_dictLoadMethod_e = u32;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub type unalign32 = u32;
pub const ZSTD_MAGIC_DICTIONARY: u32 = 0xec30a437 as u32;
use crate::common::mem::*;
pub const ZSTD_FRAMEIDSIZE: i32 = 4;
#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    mut size: usize,
    mut customMem: ZSTD_customMem,
) -> *mut std::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc)
            .expect("non-null function pointer")(customMem.opaque, size);
    }
    return libc::malloc(size);
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut std::ffi::c_void,
    mut customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree)
                .expect("non-null function pointer")(customMem.opaque, ptr);
        } else {
            ZSTD_free!(ptr)(ZSTD_free!(ptr));
        }
    }
}
#[inline]
unsafe extern "C" fn _force_has_format_string(
    mut format: *const std::ffi::c_char,
    mut args: ...
) {}
pub const NULL: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DDict_dictContent(
    mut ddict: *const ZSTD_DDict,
) -> *const std::ffi::c_void {
    return (*ddict).dictContent;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DDict_dictSize(mut ddict: *const ZSTD_DDict) -> usize {
    return (*ddict).dictSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDDictParameters(
    mut dctx: *mut ZSTD_DCtx,
    mut ddict: *const ZSTD_DDict,
) {
    (*dctx).dictID = (*ddict).dictID;
    (*dctx).prefixStart = (*ddict).dictContent;
    (*dctx).virtualStart = (*ddict).dictContent;
    (*dctx)
        .dictEnd = ((*ddict).dictContent as *const u8)
        .offset((*ddict).dictSize as isize) as *const std::ffi::c_void;
    (*dctx).previousDstEnd = (*dctx).dictEnd;
    if (*ddict).entropyPresent != 0 {
        (*dctx).litEntropy = 1;
        (*dctx).fseEntropy = 1;
        (*dctx).LLTptr = ((*ddict).entropy.LLTable).as_ptr();
        (*dctx).MLTptr = ((*ddict).entropy.MLTable).as_ptr();
        (*dctx).OFTptr = ((*ddict).entropy.OFTable).as_ptr();
        (*dctx).HUFptr = ((*ddict).entropy.hufTable).as_ptr();
        (*dctx)
            .entropy
            .rep[0] = (*ddict).entropy.rep[0];
        (*dctx)
            .entropy
            .rep[1] = (*ddict).entropy.rep[1];
        (*dctx)
            .entropy
            .rep[2] = (*ddict).entropy.rep[2];
    } else {
        (*dctx).litEntropy = 0;
        (*dctx).fseEntropy = 0;
    };
}
unsafe extern "C" fn ZSTD_loadEntropy_intoDDict(
    mut ddict: *mut ZSTD_DDict,
    mut dictContentType: ZSTD_dictContentType_e,
) -> usize {
    (*ddict).dictID = 0;
    (*ddict).entropyPresent = 0;
    if dictContentType as u32
        == ZSTD_dct_rawContent as i32 as u32
    {
        return 0;
    }
    if (*ddict).dictSize < 8 {
        RETURN_ERROR_IF!(dictContentType as u32
            == ZSTD_dct_fullDict as i32 as u32, ZSTD_error_dictionary_corrupted);
        return 0;
    }
    let magic = MEM_readLE32((*ddict).dictContent);
    if magic != ZSTD_MAGIC_DICTIONARY {
        RETURN_ERROR_IF!(dictContentType as u32
            == ZSTD_dct_fullDict as i32 as u32, ZSTD_error_dictionary_corrupted);
        return 0;
    }
    (*ddict)
        .dictID = MEM_readLE32(
        ((*ddict).dictContent as *const std::ffi::c_char)
            .offset(ZSTD_FRAMEIDSIZE as isize) as *const std::ffi::c_void,
    );
    RETURN_ERROR_IF!(ERR_isError(
        ZSTD_loadDEntropy(&mut (*ddict).entropy, (*ddict).dictContent, (*ddict).dictSize),
    ) != 0, ZSTD_error_dictionary_corrupted);
    (*ddict).entropyPresent = 1;
    return 0;
}
unsafe extern "C" fn ZSTD_initDDict_internal(
    mut ddict: *mut ZSTD_DDict,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> usize {
    if dictLoadMethod as u32
        == ZSTD_dlm_byRef as i32 as u32 || dict.is_null()
        || dictSize == 0
    {
        (*ddict).dictBuffer = std::ptr::null_mut();
        (*ddict).dictContent = dict;
        if dict.is_null() {
            dictSize = 0;
        }
    } else {
        let internalBuffer = ZSTD_customMalloc(dictSize, (*ddict).cMem);
        (*ddict).dictBuffer = internalBuffer;
        (*ddict).dictContent = internalBuffer;
        RETURN_ERROR_IF!(internalBuffer.is_null(), ZSTD_error_memory_allocation);
        libc::memcpy(internalBuffer, dict, (dictSize) as usize);
    }
    (*ddict).dictSize = dictSize;
    (*ddict)
        .entropy
        .hufTable[0] = (12 as i32 * 0x1000001 as i32) as HUF_DTable;
    FORWARD_IF_ERROR!(
        ZSTD_loadEntropy_intoDDict(ddict, dictContentType), ""
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_advanced(
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_DDict {
    if (customMem.customAlloc).is_none() as i32
        ^ (customMem.customFree).is_none() as i32 != 0
    {
        return std::ptr::null_mut();
    }
    let ddict = ZSTD_customMalloc(
        ::core::mem::size_of::<ZSTD_DDict>(),
        customMem,
    ) as *mut ZSTD_DDict;
    if ddict.is_null() {
        return std::ptr::null_mut();
    }
    (*ddict).cMem = customMem;
    let initResult = ZSTD_initDDict_internal(
        ddict,
        dict,
        dictSize,
        dictLoadMethod,
        dictContentType,
    );
    if ERR_isError(initResult) {
        ZSTD_freeDDict(ddict);
        return std::ptr::null_mut();
    }
    return ddict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict(
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
) -> *mut ZSTD_DDict {
    let allocator = {
        let mut init = ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTD_allocFunction,
            >(NULL as libc::intptr_t),
            customFree: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTD_freeFunction,
            >(NULL as libc::intptr_t),
            opaque: std::ptr::null_mut(),
        };
        init
    };
    return ZSTD_createDDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
        allocator,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_byReference(
    mut dictBuffer: *const std::ffi::c_void,
    mut dictSize: usize,
) -> *mut ZSTD_DDict {
    let allocator = {
        let mut init = ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTD_allocFunction,
            >(NULL as libc::intptr_t),
            customFree: ::core::mem::transmute::<
                libc::intptr_t,
                ZSTD_freeFunction,
            >(NULL as libc::intptr_t),
            opaque: std::ptr::null_mut(),
        };
        init
    };
    return ZSTD_createDDict_advanced(
        dictBuffer,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        allocator,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDDict(
    mut sBuffer: *mut std::ffi::c_void,
    mut sBufferSize: usize,
    mut dict: *const std::ffi::c_void,
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> *const ZSTD_DDict {
    let neededSpace = (::core::mem::size_of::<ZSTD_DDict>())
        .wrapping_add(
            (if dictLoadMethod as u32
                == ZSTD_dlm_byRef as i32 as u32
            {
                0_usize
            } else {
                dictSize
            }),
        );
    let ddict = sBuffer as *mut ZSTD_DDict;
    if sBuffer as usize & 7_usize != 0 {
        return std::ptr::null();
    }
    if sBufferSize < neededSpace {
        return std::ptr::null();
    }
    if dictLoadMethod as u32
        == ZSTD_dlm_byCopy as i32 as u32
    {
        libc::memcpy(ddict + 1, dict, (dictSize) as usize);
        dict = ddict.offset(1) as *const std::ffi::c_void;
    }
    if ERR_isError(
        ZSTD_initDDict_internal(ddict, dict, dictSize, ZSTD_dlm_byRef, dictContentType),
    ) != 0
    {
        return std::ptr::null();
    }
    return ddict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDDict(mut ddict: *mut ZSTD_DDict) -> usize {
    if ddict.is_null() {
        return 0;
    }
    let cMem = (*ddict).cMem;
    ZSTD_customFree((*ddict).dictBuffer, cMem);
    ZSTD_customFree(ddict as *mut std::ffi::c_void, cMem);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDDictSize(
    mut dictSize: usize,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
) -> usize {
    return (::core::mem::size_of::<ZSTD_DDict>())
        .wrapping_add(
            (if dictLoadMethod as u32
                == ZSTD_dlm_byRef as i32 as u32
            {
                0_usize
            } else {
                dictSize
            }),
        );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DDict(mut ddict: *const ZSTD_DDict) -> usize {
    if ddict.is_null() {
        return 0;
    }
    return (::core::mem::size_of::<ZSTD_DDict>())
        .wrapping_add(
            (if !((*ddict).dictBuffer).is_null() {
                (*ddict).dictSize
            } else {
                0_usize
            }),
        );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDDict(
    mut ddict: *const ZSTD_DDict,
) -> u32 {
    if ddict.is_null() {
        return 0;
    }
    return (*ddict).dictID;
}
