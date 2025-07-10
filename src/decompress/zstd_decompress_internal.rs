use std::ffi::{c_char, c_void};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [u32; HUF_DECOMPRESS_WORKSPACE_SIZE_U32], /* space needed when building huffman tables */
    pub previousDstEnd: *const c_void, /* detect continuity */
    pub prefixStart: *const c_void, /* start of current segment */
    pub virtualStart: *const c_void, /* virtual start of previous segment if it was just before current one */
    pub dictEnd: *const c_void, /* end of previous segment */
    pub expected: usize,
    pub fParams: ZSTD_FrameHeader,
    pub processedCSize: u64,
    pub decodedSize: u64,
    pub bType: blockType_e, /* used in ZSTD_decompressContinue(), store blockType between block header decoding and block decompression stages */
    pub stage: ZSTD_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: usize,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e, /* User specified: if == 1, will ignore checksums in compressed frame. Default == 0 */
    pub validateChecksum: u32, /* if == 1, will validate checksum. Is == 1 if (fParams.checksumFlag == 1) and (forceIgnoreChecksum == 0). */
    pub litPtr: *const u8,
    pub customMem: ZSTD_customMem,
    pub litSize: usize,
    pub rleSize: usize,
    pub staticSize: usize,
    pub isFrameDecompression: i32,
    // TODO #if DYNAMIC_BMI2
    pub bmi2: i32, /* == 1 if the CPU supports BMI2 and 0 otherwise. CPU support is determined dynamically once per context lifetime. */
    
    /* dictionary */
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict, /* set by ZSTD_initDStream_usingDDict(), or ZSTD_DCtx_refDDict() */
    pub dictID: u32,
    pub ddictIsCold: i32, /* if == 1 : dictionary is "new" for working context, and presumed "cold" (not in cpu cache) */
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet, /* Hash set for multiple ddicts */
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e, /* User specified: if == 1, will allow references to multiple DDicts. Default == 0 (disabled) */
    pub disableHufAsm: i32,
    pub maxBlockSizeParam: i32,

    /* streaming */
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut c_char,
    pub inBuffSize: usize,
    pub inPos: usize,
    pub maxWindowSize: usize,
    pub outBuff: *mut c_char,
    pub outBuffSize: usize,
    pub outStart: usize,
    pub outEnd: usize,
    pub lhSize: usize,

    // TODO #if defined(ZSTD_LEGACY_SUPPORT) && (ZSTD_LEGACY_SUPPORT>=1)
    pub legacyContext: *mut c_void,
    pub previousLegacyVersion: u32,
    pub legacyVersion: u32,

    pub hostageByte: u32,
    pub noForwardProgress: i32,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,

    /* workspace */
    pub litBuffer: *mut u8,
    pub litBufferEnd: *const u8,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [u8; ZSTD_LITBUFFEREXTRASIZE + WILDCOPY_OVERLENGTH],
    pub headerBuffer: [u8; ZSTD_FRAMEHEADERSIZE_MAX],

    pub oversizedDuration: usize,

    /* Tracing */
    // TODO #if ZSTD_TRACE
    pub traceCtx: ZSTD_TraceCtx,
} /* typedef'd to ZSTD_DCtx within "zstd.h" */
